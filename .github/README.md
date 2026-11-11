# Zapp CDMA2000 / EV-DO

**An open-source restoration of Romania's historical Zapp CDMA mobile network.**

> **Status: early development.** This repository currently contains project
> documentation only. No part of the Zapp distribution is operational yet.
> Functionality and handset compatibility will be documented only after they
> have been implemented and tested.

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

These are the project's goals. **None of them is implemented in this
repository yet.**

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
Project Zapp has **not yet** independently built, tested or verified these
capabilities.

The plan is to bring the relevant upstream source directly into this
repository, so that the distribution is self-contained and does not rely on
separately cloned upstream repositories. Each import will:

- record the exact upstream commit it was taken from,
- keep all upstream licences, copyright notices and attribution,
- be built and tested as an unmodified baseline before any Zapp-specific
  changes are made.

Third-party components remain under their own licences. See
[`NOTICE`](NOTICE).

## Development status

| Area | Status |
| --- | --- |
| Repository, licensing and documentation | In progress |
| Upstream 1xBTS baseline (import, build, test) | Not started |
| CDMA2000 1x network services | Not started |
| EV-DO packet data | Not started |
| Zapp Network Studio | Not started |
| Installation and service management | Not started |
| Diagnostics and simulation | Not started |
| Historical network profiles | Not started |
| Automated testing and CI | Not started |

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

## Documentation

- [NOTICE](NOTICE): attribution and third-party licensing.

Technical documentation will be added as components are implemented and
verified.

## Responsible use

CDMA2000 equipment transmits on licensed radio spectrum. Operate RF hardware
only where you are legally authorised to transmit, for example under an
appropriate licence or inside a shielded test environment. Project Zapp does
not provide or endorse operational frequency plans.

## License

Original Project Zapp material is licensed under the
[Apache License, Version 2.0](LICENSE). Third-party components, including any
source code derived from 1xBTS, remain under their respective licences and
copyright notices.

## References

- RCR Wireless, "Telemobil introduces CDMA services in Romania":
  <https://www.rcrwireless.com/?p=113925>
- ITU-D, presentation by Telemobil (2003):
  <https://www.itu.int/ITU-D/tech/events/2003/slovenia2003/Presentations/Day%203/3.4.3_Stan.pdf>
- Economica.net, "Zapp Mobile nu va mai furniza servicii în banda de 450 MHz":
  <https://www.economica.net/zapp-mobilenu-va-mai-furniza-servicii-in-banda-de-450-mhz_46261.html>
- Wikipedia, "Zapp Mobile": <https://en.wikipedia.org/wiki/Zapp_Mobile>
- 1xBTS: <https://github.com/chrismoos/1xbts>
