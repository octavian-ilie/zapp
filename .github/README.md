# Zapp CDMA2000 / EV-DO

**An open-source restoration of Romania's historical Zapp CDMA mobile network.**

> **Status: early development.** Initial upstream audit completed. The
> complete 1xBTS source is imported without modification. Full validation
> pending.
>
> The network has not been run as a whole. Nothing has been verified over the
> air or with handsets, and the Zapp distribution itself (installer, managed
> services, Zapp Network Studio) does not exist yet. Functionality and handset
> compatibility will be documented only after they have been implemented and
> tested.

This is Project Zapp's README. The `README.md` at the root of the repository
is the original 1xBTS README, kept unmodified.

---

## About

Project Zapp is an independent open-source project that aims to recreate the
technologies and user experience of Zapp, the CDMA mobile network that
Telemobil S.A. operated in Romania. The project uses modern software and
software-defined radio (SDR) hardware.

The goal is more than a generic CDMA base station. Project Zapp is building a
maintained, reproducible software distribution that brings together:

- a CDMA2000 radio-access and core-network stack,
- a web management application, **Zapp Network Studio**,
- tools for installing, configuring, operating and diagnosing the network,
- historical research that documents how the original network worked.

Complete historical fidelity may need technical information that no longer
exists. The project separates verified history, published standards, software
defaults and its own reconstruction choices, and labels each one clearly.

Project Zapp is not affiliated with Telemobil S.A., OTE, Cosmote Romania or
Telekom Romania. It is not an official continuation of the Zapp commercial
network.

## Historical background

Telemobil began as an analogue NMT-450 operator in the early 1990s. It then
moved its 450 MHz spectrum to digital CDMA technology under the Zapp brand.
Zapp became an example of an alternative technical path in European mobile
telecommunications, where GSM and its successors eventually dominated.

| Date | Milestone |
| --- | --- |
| 7 December 2001 | Commercial launch of CDMA2000 voice and data services in the 450 MHz band (CDMA450) |
| 2004 | Introduction of EV-DO Rev. 0 mobile broadband |
| 24 March 2013 | Expiry of the CDMA 450 MHz licence and closure of the CDMA450 network |

Zapp's EV-DO service offered packet-data mobile broadband in Romania several
years before 4G and 5G networks became common. When the CDMA450 network
closed, the devices built for it lost the only network they could use.

Restoring that environment means understanding radio air interfaces, protocol
signalling, core-network behaviour, historical operating practice and the
limits of surviving handsets.
And that’s what makes Project Zapp such a fascinating combination of software engineering, radio science and telecommunications history.

## Technologies

Project Zapp concerns the **3GPP2 CDMA2000** family of standards. It does not
use the 3GPP family (GSM, UMTS, LTE, NR) that most European networks used.

- **CDMA2000 1x (1xRTT).** The core CDMA2000 air interface for circuit-switched
  voice, SMS and packet data, with an architecture based on BTS, BSC, MSC and
  HLR nodes.
- **1xEV-DO (HRPD).** "Evolution-Data Optimized", a separate data-only air
  interface that runs on its own carrier and is designed for high-rate packet
  data. Rev. 0 was the first commercial release; Rev. A added faster uplink
  and lower latency.
- **CDMA450.** The use of CDMA2000 in the 450 MHz band, usually by operators
  moving on from NMT-450. Zapp was a CDMA450 operator.

## Objectives

These are the project's goals. The imported 1xBTS stack is the starting point
for the network infrastructure. **None of these goals is complete or
verified as part of the Zapp distribution yet.**

- **CDMA2000 1xRTT network infrastructure:** radio access and core network
  for voice, SMS and packet data.
- **EV-DO packet data:** to be scoped against what the upstream stack actually
  supports and what is verified in testing.
- **Zapp Network Studio:** a web interface for administration, configuration,
  provisioning and monitoring.
- **Diagnostics and simulation:** tools and environments for testing protocol
  behaviour without radio transmission.
- **Historical network profiles:** research-based configuration references,
  labelled clearly until their accuracy is established.
- **Reproducible deployment:** a distribution that installs and runs on
  compatible Linux systems, not only on one development machine.
- **Operations:** service management, upgrades, backup and restore, and
  troubleshooting.
- **Quality:** automated tests and continuous integration.

### Zapp Network Studio

Zapp Network Studio is the planned web management application for the
distribution. It is intended to cover:

- network status, administration and configuration,
- CDMA2000 network elements and services,
- subscriber and handset management,
- historical network profiles,
- logs, events, diagnostics and monitoring,
- simulation and test environments,
- deployment, maintenance and software updates,
- backup and restoration.

The interface is planned to be modern, clean and technically focused, while
keeping a visible link to the historical Zapp identity.

### A deployable distribution

A central requirement is that Project Zapp ships as a complete distribution
rather than a set of scripts tied to one machine. The planned distribution
covers:

- a reproducible installation process with dependency detection,
- configuration setup and validation,
- database setup and migrations where needed,
- managed system services,
- health checks and diagnostics,
- upgrade, backup, restore and recovery procedures,
- documentation of supported hardware and operating systems.

**Radio transmission is never enabled automatically.** Any RF operation will
need explicit operator configuration and is the operator's responsibility to
license.

## Technical foundation

Project Zapp is based on [1xBTS](https://github.com/chrismoos/1xbts), an
open-source CDMA2000 1x and HRPD (1xEV-DO) radio-access and core-network stack
written mainly in Rust and licensed under the Apache License 2.0. 1xBTS is
developed by Chris Moos and its contributors.

Upstream describes 1xBTS as experimental and intended for research,
interoperability testing and lab use. It reports working 1x SMS, voice and
packet data, plus initial experimental EV-DO Rev. 0 and Rev. A packet data.

The complete 1xBTS source at commit `9897714` (8 November 2026) is imported
into this repository without modification, at its original paths. That
includes every crate and network component, the reference voice codecs, the
SIP voice gateway, the SIM applet, the test data and the CI workflow. Nothing
is cloned from 1xBTS at build or install time. From here on, Project Zapp
develops it independently, and keeps its own changes separate from this
baseline. [`SOURCE_ORIGIN.md`](../SOURCE_ORIGIN.md) records the import and how
it was checked.

Validation of the import is in progress. In a local Linux build without the
SDR backends and the SIP voice gateway, which need further system libraries,
the imported code compiles, passes the formatting and Clippy checks, and its
automated tests give the same results as the same upstream revision built and
tested the same way: 4,199 passed and 44 are ignored by upstream. One
timing-sensitive upstream test, which also fails intermittently on the
unmodified upstream source, failed in both runs. These are software tests
(protocol state machines, synthetic and recorded signals), so they do not show
that any capability works over the air. Full validation, including the SDR
builds, the SIP voice gateway, the database-backed tests and upstream's CI on
this repository, is pending.

Some files in the 1xBTS tree, including the reference voice codecs, are under
their own terms rather than the Apache License. See [`NOTICE`](../NOTICE).

## Development status

| Area | Status |
| --- | --- |
| Repository, licensing and documentation | In progress |
| Upstream technical audit | Done: initial audit of 1xBTS `9897714` |
| Upstream source import | Done: complete 1xBTS `9897714`, unmodified; full validation pending |
| CDMA2000 1x network services | Imported from 1xBTS; software tests only, not verified with handsets |
| Voice | Imported from 1xBTS, including the reference codecs; codec tests pass locally; SIP voice gateway not yet built or tested by Project Zapp |
| EV-DO packet data | Imported from 1xBTS (experimental upstream); not verified |
| Zapp Network Studio | Not started (the upstream dashboard is imported as `1xbts-web`) |
| Installation and service management | Not started |
| Diagnostics and simulation | Upstream simulation tools imported; not yet integrated |
| Historical network profiles | Not started |
| Automated testing and CI | Upstream CI workflow imported unmodified; not yet run for this repository |

## Roadmap

The milestones are listed in their intended order. Their scope may change as
the upstream baseline is assessed.

1. **Repository foundation.** Licensing, documentation, architecture and
   contribution standards.
2. **Upstream baseline.** Import 1xBTS at a recorded revision, build it, run
   its existing tests and document its verified capabilities.
3. **Distribution foundation.** Installation, configuration, service
   management and health checks.
4. **Zapp Network Studio.** First management interface on top of the
   distribution.
5. **Simulation and diagnostics.** Test environments that do not need RF
   transmission.
6. **Historical profiles.** Research-based network profiles with documented
   sources and confidence levels.
7. **Operations and quality.** Upgrade, backup and restore, CI, and
   documentation of supported hardware.

## Building from source

There is no installer yet. Install [Git LFS](https://git-lfs.com/) before
cloning: test recordings, PRL test fixtures and the prebuilt SIM applet are
stored with it.

The prerequisites and node descriptions are in the original
[1xBTS README](../README.md). A development build of the Rust workspace needs:
- Rust 1.96.0, selected automatically by `rust-toolchain.toml`;
- `protoc` (`protobuf-compiler`);
- a C/C++ toolchain and `pkg-config`.

The base station's SDR backends and the SIP voice gateway need further native
libraries, listed in the 1xBTS README.

To build without any radio hardware support or the SIP voice gateway:

```sh
cargo build --release --workspace \
  --exclude cdma-bts --exclude cdma-nib \
  --exclude cdma-voice-gw --exclude libre --exclude libre-sys
cargo build --release -p cdma-bts -p cdma-nib --no-default-features
```

## Documentation

- [SOURCE_ORIGIN.md](../SOURCE_ORIGIN.md): imported revision and how the
  import was checked.
- [NOTICE](../NOTICE): attribution and third-party licensing.
- [README.md](../README.md): the original 1xBTS documentation, including
  prerequisites and node descriptions.
- [docs/PORTS.md](../docs/PORTS.md): network ports used by the nodes (upstream
  document).

## Responsible use

CDMA2000 equipment transmits on licensed radio spectrum. Operate RF hardware
only where you are legally authorised to transmit, for example under an
appropriate licence or inside a shielded test environment. Project Zapp does
not provide or endorse operational frequency plans.

The imported stack transmits as soon as a base station is started with a
real SDR radio configured. The default configuration uses no radio.

## License

Original Project Zapp material is licensed under the
[Apache License, Version 2.0](../LICENSES/Project-Zapp-LICENSE.txt). The
[`LICENSE`](../LICENSE) file at the root of the repository is the original
1xBTS licence. 1xBTS and the other third-party components remain under their
respective licences and copyright notices; see [`NOTICE`](../NOTICE).

## References

- RCR Wireless, "Telemobil introduces CDMA services in Romania":
  <https://www.rcrwireless.com/?p=113925>
- ITU-D, presentation by Telemobil (2003):
  <https://www.itu.int/ITU-D/tech/events/2003/slovenia2003/Presentations/Day%203/3.4.3_Stan.pdf>
- Economica.net, "Zapp Mobile nu va mai furniza servicii în banda de 450 MHz":
  <https://www.economica.net/zapp-mobilenu-va-mai-furniza-servicii-in-banda-de-450-mhz_46261.html>
- Wikipedia, "Zapp Mobile": <https://en.wikipedia.org/wiki/Zapp_Mobile>
- 1xBTS: <https://github.com/chrismoos/1xbts>
