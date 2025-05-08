# PhaseOps: The Digital Career Exoskeleton

**PhaseOps** is a modular, AI-augmented job operations toolkit designed to streamline and supercharge the job hunt.  
It offers a local-first, telemetry-enabled control surface across CLI, web, and browser-embedded interfaces.

![Progress](https://img.shields.io/badge/Progress-14%25-brightgreen)
![Language](https://img.shields.io/badge/Language-Rust-orange)
![Platform](https://img.shields.io/badge/Platform-CLI%20%7C%20Web%20%7C%20Extension-blue)
![Status](https://img.shields.io/badge/Status-In--Development-blue)
![Stars](https://img.shields.io/github/stars/adan-abdi/phaseops?style=social)

---

## 🧭 Project Vision

PhaseOps isn't a resume builder.  
It's a **battle-tested system** for orchestrating, tracking, and evolving career operations—designed for developers who prefer **command over chaos**.

- CLI-first, built in Rust for speed and precision.
- Modular architecture with local persistence and optional AI augmentation.
- Secure, human-driven interactions across terminals, dashboards, and embedded HUDs.

---

## 🔌 Architecture

- **Language**: Rust (`clap`, `tokio`, `serde`, `git2`)
- **UI**: Static HTML + Tailwind (web), Tauri (HUD)
- **Data**: CSV, Git, optional SQLite
- **AI Layer**: Pluggable OpenAI/Gemini LLM hooks
- **PDF Generation**: Markdown → PDF pipeline
- **Planned**: Local vector indexing and adaptive scoring

---

## 🔒 Operational Channels

| Interface        | Purpose                                |
|------------------|----------------------------------------|
| **CLI**          | Core job automation & interaction flow |
| **Web UI**       | Visual interface for logs and history  |
| **Browser HUD**  | In-context overlay on job platforms    |

All interfaces interoperate via shared local data and Git-backed logs.

---

## 🧠 What It's *Not*

- Not a SaaS
- Not click-to-apply spamware
- Not resume fluff or template generator

PhaseOps is built for people who **track their own telemetry**, manage their job ops like Git branches, and want complete control over how they adapt in a volatile hiring market.

---

## 📦 Status

This repo contains the core CLI scaffold and early HUD experiments.  
More modules will be released incrementally.

---

## 🛡️ Legal

All code is licensed under the [Business Source License 1.1](LICENSE.md).  

---

## 🧠 Summary

> PhaseOps is a modular career control suite.  
> You operate. It tracks. It adapts. You evolve.  
