// Copyright 2026 Octavian Ilie (Project Zapp). Licensed under the Apache
// License, Version 2.0; see LICENSES/Project-Zapp-LICENSE.txt.

//! A subscriber provisioning request that the HLR rejects must not leave a
//! partial change behind: no new subscriber, no overwritten profile and no
//! half-assigned identity.

use std::sync::Arc;

use cdma_hlr::proto;
use cdma_hlr::proto::hlr_service_server::HlrService;
use cdma_hlr::repository::{HlrRepository, PostgresHlrRepository};
use cdma_hlr::service::HlrServiceImpl;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tonic::{Code, Request};
use uuid::Uuid;

// Fictitious test identities: IMSIs on the 001-01 test network, IMEIs that only
// satisfy the Luhn check digit.
const IMSI: &str = "001010000000001";
const OTHER_IMSI: &str = "001010000000002";
const ESN: u32 = 0x8000_0001;
const OTHER_ESN: u32 = 0x8000_0002;
const IMEI: &str = "000000000000018";
const OTHER_IMEI: &str = "000000000000026";

fn request(
    phone_number: &str,
    display_name: &str,
    identity: Option<(Option<&str>, u32)>,
    imei: Option<&str>,
) -> Request<proto::UpsertSubscriberRequest> {
    Request::new(proto::UpsertSubscriberRequest {
        phone_number: phone_number.to_string(),
        display_name: display_name.to_string(),
        status: proto::SubscriberStatus::Active as i32,
        imsi: identity.and_then(|(imsi, _)| imsi.map(str::to_string)),
        esn: identity.map(|(_, esn)| esn),
        imei: imei.map(str::to_string),
        ..Default::default()
    })
}

async fn verify(pool: PgPool) {
    let repo = Arc::new(PostgresHlrRepository::new(pool.clone()));
    repo.run_migrations().await.unwrap();
    let hlr = HlrServiceImpl::new(repo.clone());

    // An existing subscriber owns ESN and IMEI.
    hlr.upsert_subscriber(request(
        "555100",
        "Owner",
        Some((Some(IMSI), ESN)),
        Some(IMEI),
    ))
    .await
    .unwrap();

    // A new subscriber with a taken ESN is rejected and not created.
    let err = hlr
        .upsert_subscriber(request(
            "555101",
            "New",
            Some((Some(OTHER_IMSI), ESN)),
            None,
        ))
        .await
        .unwrap_err();
    assert_eq!(err.code(), Code::AlreadyExists);
    assert!(
        repo.get_subscriber_by_phone_number("555101")
            .await
            .unwrap()
            .is_none(),
        "a rejected request must not create the subscriber"
    );

    // For an existing subscriber, the rejected request leaves the profile and
    // identities unchanged.
    hlr.upsert_subscriber(request("555102", "Before", None, None))
        .await
        .unwrap();
    let err = hlr
        .upsert_subscriber(request(
            "555102",
            "After",
            Some((Some(OTHER_IMSI), ESN)),
            None,
        ))
        .await
        .unwrap_err();
    assert_eq!(err.code(), Code::AlreadyExists);
    let kept = repo
        .get_subscriber_by_phone_number("555102")
        .await
        .unwrap()
        .expect("the existing subscriber stays");
    assert_eq!(kept.subscriber.display_name, "Before");
    assert!(kept.identities.is_empty());

    // A taken IMEI is rejected after the identity itself was accepted: the
    // subscriber and its identity are not stored either.
    let err = hlr
        .upsert_subscriber(request(
            "555103",
            "New",
            Some((Some(OTHER_IMSI), OTHER_ESN)),
            Some(IMEI),
        ))
        .await
        .unwrap_err();
    assert_eq!(err.code(), Code::AlreadyExists);
    assert!(
        repo.get_subscriber_by_phone_number("555103")
            .await
            .unwrap()
            .is_none(),
        "a rejected IMEI must not leave the subscriber behind"
    );
    assert!(
        repo.resolve_by_hardware_identity(Some(OTHER_ESN), None)
            .await
            .map_err(|_| "hardware lookup failed")
            .unwrap()
            .is_none(),
        "a rejected IMEI must not leave the identity behind"
    );

    // An identity without an IMSI is rejected by validation in the identity
    // step: the subscriber is not created either.
    assert!(
        hlr.upsert_subscriber(request("555105", "New", Some((None, OTHER_ESN)), None))
            .await
            .is_err()
    );
    assert!(
        repo.get_subscriber_by_phone_number("555105")
            .await
            .unwrap()
            .is_none(),
        "a rejected identity must not leave the subscriber behind"
    );

    // A valid request still stores the subscriber, its identity and its IMEI.
    let created = hlr
        .upsert_subscriber(request(
            "555104",
            "Valid",
            Some((Some(OTHER_IMSI), OTHER_ESN)),
            Some(OTHER_IMEI),
        ))
        .await
        .unwrap()
        .into_inner();
    let identity = created.identity.expect("identity in the response");
    assert_eq!(identity.esn, Some(OTHER_ESN));
    assert_eq!(identity.imei.as_deref(), Some(OTHER_IMEI));
    let stored = repo
        .get_subscriber_by_phone_number("555104")
        .await
        .unwrap()
        .expect("the subscriber is stored");
    assert_eq!(stored.subscriber.display_name, "Valid");
    let stored_identity = stored.primary_identity.expect("the identity is stored");
    assert_eq!(stored_identity.esn, Some(OTHER_ESN));
    assert_eq!(stored_identity.imei.as_deref(), Some(OTHER_IMEI));
}

#[tokio::test]
#[ignore = "requires HLR_TEST_DSN with permission to create a temporary database"]
async fn rejected_provisioning_leaves_no_partial_change() {
    let admin_dsn = std::env::var("HLR_TEST_DSN").expect("set HLR_TEST_DSN");
    let admin = PgPoolOptions::new().connect(&admin_dsn).await.unwrap();
    let database = format!("hlr_atomic_test_{}", Uuid::new_v4().simple());
    sqlx::query(&format!("CREATE DATABASE {database}"))
        .execute(&admin)
        .await
        .unwrap();
    let (base, _) = admin_dsn.rsplit_once('/').unwrap();
    let pool = PgPoolOptions::new()
        .after_connect(|connection, _| {
            Box::pin(async move {
                sqlx::query("SET search_path TO hlr, public")
                    .execute(connection)
                    .await?;
                Ok(())
            })
        })
        .connect(&format!("{base}/{database}"))
        .await
        .unwrap();
    let result = tokio::spawn(verify(pool.clone())).await;
    pool.close().await;
    sqlx::query(&format!("DROP DATABASE {database} WITH (FORCE)"))
        .execute(&admin)
        .await
        .unwrap();
    admin.close().await;
    result.unwrap();
}
