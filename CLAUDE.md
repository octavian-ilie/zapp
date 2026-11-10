# CLAUDE.md: Project Zapp engineering instructions

This file contains standing instructions for anyone working in this repository,
including AI coding assistants. It applies to every change.

## Project

Zapp CDMA2000 / EV-DO is an independent open-source software distribution that
restores the historical Romanian Zapp (Telemobil) CDMA450 network experience.
It is based on the Apache-2.0 licensed
[1xBTS](https://github.com/chrismoos/1xbts) stack.

See `README.md` for scope and status.

## Ground rules

1. **Never claim unverified functionality.** Do not write in code, docs,
   commit messages or UI text that something works until it has been built and
   tested. Use "planned", "proposed", "experimental" or "upstream reports"
   wherever they apply.
2. **No automatic radio transmission.** No installer, service unit, default
   configuration, script or test may start RF transmission without explicit
   operator action. Defaults must be non-transmitting.
3. **The public repository holds public material only.** See *Privacy* below.
   When unsure, leave it out and ask.
4. **Stay in scope.** Make only the change that was requested. Do not start
   the next milestone, refactor unrelated code or add speculative features.

## Architecture principles

- Keep a clear boundary between **upstream-derived code** and **Project Zapp
  code**. Zapp-specific behaviour goes in Zapp-owned modules, configuration and
  the management layer. Change upstream-derived code only when that is
  necessary, and record why.
- The distribution must be **self-contained and reproducible**. Do not use Git
  submodules, subtrees, runtime clones of upstream repositories or automatic
  upstream synchronisation.
- Do not tie anything to the development host. Avoid hard-coded paths,
  hostnames, users, network addresses or assumptions about specific hardware.
  Supported platforms must be documented, not implied.
- Configuration is declarative and validated before use. Every setting has a
  documented safe default.
- Keep historical profiles as data, separate from code. Each profile records
  its sources and how confident the project is in it.

## Upstream source and licensing

- Before importing upstream source, record the upstream repository URL, exact
  commit hash, import date and imported paths in `NOTICE` and in a record next
  to the imported code.
- Keep every upstream `LICENSE`, copyright header and attribution unchanged.
  Do not remove or rewrite third-party copyright notices.
- Mark modified upstream files with a prominent notice of the change, as
  Apache-2.0 Section 4(b) requires.
- Check the licence of every dependency and bundled component before adding
  it. Do not add code under licences incompatible with Apache-2.0 without
  explicit approval.
- Original Project Zapp material is Apache-2.0. Never imply that Project Zapp
  owns or relicenses upstream contributions.
- Before modifying an upstream import, build it unmodified, run its existing
  tests and document the results as the baseline.

## Code quality

- Follow the established language conventions. For Rust, use `rustfmt`,
  `clippy` with no new warnings, and the toolchain pinned by the project.
- Prefer small, reviewable changes with one clear purpose.
- Handle errors explicitly. Do not silently ignore failures in installers,
  service management or configuration loading.
- Comments explain *why*, not *what*. Avoid placeholder code, dead code and
  empty scaffolding directories.
- Do not commit generated files, build outputs or vendored binaries unless
  they are deliberately part of the distribution and documented.

## Verification

- Run the relevant build, tests and linters before declaring work complete.
  Report the real results, including failures and skipped steps.
- New behaviour needs automated tests where practical. Protocol and
  configuration logic should be testable without RF hardware.
- Installer and service changes must be checked on a clean system or
  container, not only on the development machine.
- Distinguish clearly between tests that ran and passed, tests that were not
  run, and behaviour inferred from reading code.

## Documentation

- Update the relevant documentation in the same change as the behaviour it
  describes. Keep the README status table accurate.
- Label historical statements by evidence level: **verified history** (with a
  source), **published standard**, **software default**, **proposed
  implementation** or **unresolved**.
- Never present assumptions as authentic Zapp network settings.
- Write documentation for the project's readers. Do not include conversation
  history or notes addressed to a specific person.

## Privacy

This repository is public. Never commit:

- personal telephone numbers, subscriber identities (IMSI/MIN/MDN), handset
  ESN/MEID or any other unique device identifier,
- private handset inventories, original handset photographs or screenshots,
- credentials of any kind: SSH keys, passwords, tokens, API keys, service
  access codes, `.env` files,
- extracted vendor firmware, including Apple firmware,
- private IP addresses, hostnames, Wi-Fi details or other machine-specific
  configuration,
- historical frequency or channel mappings, specific RF reconstruction
  settings, operational radio configurations or physical test plans,
- private engineering notes or research.

Private research stays **outside** this Git repository. Earlier research
documents are not approved for publication automatically, even if they were
previously marked as sanitised. Each one needs explicit review.

Use obviously fictitious placeholder values in examples and tests, and mark
them as such.

Before every commit, review the staged diff for the items above.

## Commits and publishing

- Use concise conventional commit messages (`docs:`, `feat:`, `fix:`,
  `build:`, `test:`, `chore:`), written in the imperative mood.
- Stage specific files. Do not use blanket `git add -A` without reviewing what
  it picks up.
- Never force-push, rewrite published history or delete remote branches
  without explicit approval.
- Do not add AI tool attribution trailers, "generated with" footers or session
  links to commits or pull requests.

## Responsible development

Project Zapp exists for preservation, research, interoperability testing and
education. Do not add features that are designed to interfere with,
impersonate or intercept live networks or third-party devices. RF operation
must follow local law and licensing.
