# Source origin

This repository contains a complete, unmodified snapshot of
[1xBTS](https://github.com/chrismoos/1xbts). This file records where the
snapshot came from, how it was imported and checked, and which files Project
Zapp adds next to it.

## Imported revision

| | |
| --- | --- |
| Upstream project | 1xBTS, <https://github.com/chrismoos/1xbts> |
| Upstream commit | `9897714153261b86913ef6c6b18e7f9a344ff4fe` (2026-11-08) |
| Import date | 2026-11-11 |
| Imported paths | All 1,649 files tracked at that commit, at their original paths |
| Git LFS | 37 of those files are stored with Git LFS, as upstream stores them (34 distinct objects, 229 MB in total) |
| Modifications | Six upstream files, listed under [Changes made by Project Zapp](#changes-made-by-project-zapp) |

The import is a one-time snapshot. The repository has no Git submodules, no
subtrees, no upstream remote and no automatic synchronisation, and nothing is
downloaded from 1xBTS at build or install time. Upstream's commit history is
not included.

## How the import was made and checked

Every file was copied from the upstream commit as a Git object, so each one
keeps its exact content and file mode. Git LFS files keep upstream's LFS
pointers and objects, and upstream's `.gitattributes` is included unchanged.

A file-by-file comparison against the upstream commit checked, for each of the
1,649 paths:

- that this repository's tree has the same file mode and the same Git object
  ID;
- that the checked-out file has the same content, or for a Git LFS file, the
  same SHA-256 and size as the LFS object;
- that the executable bit matches the file mode.

All 1,649 paths matched. At import, no upstream file was modified, moved,
removed or left out.

## Changes made by Project Zapp

Each changed upstream file carries a notice at the top stating that Project
Zapp modified it, as Section 4(b) of the Apache License requires.

| File | Change | Reason |
| --- | --- | --- |
| `crates/cdma-ms/tests/fake_sdr_registration.rs` | The five fake-SDR tests take a shared lock, so they run one at a time instead of concurrently. Assertions and timeouts are unchanged. | `fake_sdr_delivers_mo_sms_over_hardware_time` failed intermittently, also on unmodified upstream and in upstream's own CI. Run alone, its SMS arrived after 12.8 s every time. Run concurrently with the other four tests, the SMS never arrived, even with the wait raised from 20 s to 120 s. Run one at a time, all five tests passed every time. |
| `1xbts-web/server.mjs` | The WebSocket hub fetches the dashboard's own API routes at the address the server actually listens on (a wildcard listener at loopback), instead of always at `127.0.0.1`. | With the server listening on `::1`, which is what `localhost` resolves to on many systems, or on a specific LAN address, every self-fetch failed and the dashboard's live data and events stayed empty. Reproduced in a network sandbox; after the change the hub reached the server on `::1`, `127.0.0.1`, `0.0.0.0` and `::`. |
| `1xbts-web/src/app/api/subscribers/route.ts` | `GET` fetches every page of subscribers (1,000 per request) instead of only the newest 100. | The subscribers page shows only this list, so subscribers beyond the newest 100 could not be reached. Against a test HLR holding 2,345 subscribers, the route returned 100 before the change and all 2,345 after it. |
| `crates/cdma-hlr/src/lib/repository.rs` | New method `HlrRepository::upsert_subscriber_with_identity`. `PostgresHlrRepository` runs it in one database transaction. The existing subscriber, identity and IMEI queries moved unchanged into helper functions shared by both paths. Other implementations keep the previous step-by-step behaviour by default. | When the HLR rejected the identity in a provisioning request (an ESN, MEID or IMEI already in use, or an identity without an IMSI), the subscriber had already been created or its profile overwritten. The new test `crates/cdma-hlr/tests/provisioning_atomicity.rs` failed before the change and passes after it. |
| `crates/cdma-hlr/src/lib/service.rs` | `UpsertSubscriber` stores the subscriber, identity and IMEI through the new method. | As above. |
| `CHANGELOG.md` | Three entries at the top of `# HEAD`, marked "(Project Zapp)", for the three bug fixes above. | Upstream's `AGENTS.md` asks for a changelog entry for every user-visible change. |

## Project Zapp files

Project Zapp's own files sit next to the upstream tree. None of them replaces
an upstream file.

| File | Purpose |
| --- | --- |
| [`.github/README.md`](.github/README.md) | Project Zapp's README. GitHub shows it on the repository page. The root `README.md` is the original 1xBTS README. |
| [`LICENSES/Project-Zapp-LICENSE.txt`](LICENSES/Project-Zapp-LICENSE.txt) | Licence for original Project Zapp material (Apache License 2.0). The root `LICENSE` is the original 1xBTS licence file. |
| [`NOTICE`](NOTICE) | Attribution and third-party notices. |
| `SOURCE_ORIGIN.md` | This record. |
| `crates/cdma-hlr/tests/provisioning_atomicity.rs` | Test that a rejected provisioning request leaves no partial change. Like upstream's other HLR database tests, it runs only with `HLR_TEST_DSN` set. |

All other files, including `.gitignore`, `.gitattributes`, `CLAUDE.md`,
`AGENTS.md`, `CHANGELOG.md`, `CONTRIBUTING.md` and the CI workflow in
`.github/workflows/`, are upstream 1xBTS files.

## Material with its own terms

Some files in the 1xBTS tree are under terms other than the Apache License:
the reference voice codec sources in `crates/cdma-voice/csrc/` (3GPP2, TIA,
Qualcomm and Lucent notices, and a GPL-licensed `getopt`), and two Unwired
Planet specifications in `crates/hdtp-gw/docs/`. [`NOTICE`](NOTICE) lists
them with their terms.

## Cloning

The Git LFS files are test recordings, PRL test fixtures, a sample sound and
the prebuilt SIM applet. Install [Git LFS](https://git-lfs.com/) before
cloning to get their content. Without it, those files are checked out as
small pointer files instead of their real content.
