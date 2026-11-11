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
| Modifications | None |

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

All 1,649 paths matched. No upstream file is modified, moved, removed or left
out.

## Project Zapp files

Project Zapp's own files sit next to the upstream tree. None of them replaces
an upstream file.

| File | Purpose |
| --- | --- |
| [`.github/README.md`](.github/README.md) | Project Zapp's README. GitHub shows it on the repository page. The root `README.md` is the original 1xBTS README. |
| [`LICENSES/Project-Zapp-LICENSE.txt`](LICENSES/Project-Zapp-LICENSE.txt) | Licence for original Project Zapp material (Apache License 2.0). The root `LICENSE` is the original 1xBTS licence file. |
| [`NOTICE`](NOTICE) | Attribution and third-party notices. |
| `SOURCE_ORIGIN.md` | This record. |

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
