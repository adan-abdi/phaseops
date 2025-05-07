# PhaseOps: The Digital Career Exoskeleton

**PhaseOps** is a modular, AI-augmented, local-first job search operating system.  
It integrates a Rust CLI, a local web UI, and an in-browser HUD to track, augment, and accelerate your job hunt.

![PhaseOps Progress](https://img.shields.io/badge/Progress-14%25-brightgreen)
![Rust](https://img.shields.io/badge/Language-Rust-orange)
![Platform](https://img.shields.io/badge/Platform-CLI%20%7C%20Web%20%7C%20BrowserExtension-blue)
![Stars](https://img.shields.io/github/stars/adan-abdi/phaseops?style=social)
![Status](https://img.shields.io/badge/Status-in--Development-blue)

---

## 🧬 Operational Channels

| Channel           | Role                                          | Target UX                |
|-------------------|-----------------------------------------------|--------------------------|
| **CLI**           | Power-user control & automation               | Terminal-based operators |
| **Web Portal**    | Local UI for resume gen, logging, dashboard   | Visual-first users       |
| **Browser HUD**   | Overlay inside LinkedIn Jobs (Tauri/Extension)| Real-time field ops      |

All channels operate on shared local data, version-controlled resume templates, and LLM-assisted job targeting.

---

## ⚙️ PhaseOps Pipeline Breakdown

---

### 🔍 Phase 0: Intake
**"What job are we targeting?"**

- Input:
  - CLI: Paste JD
  - Web: JD input field
  - Browser HUD: Detect from view + clipboard
- Extract:
  - Title, company, location, stack
  - ATS keywords via LLM
- Classify:
  - Role type, effort level
- Stored in: `logs/applications.csv`

---

### 🧠 Phase 1: Resume Generation
**"Fire the right bullet for this target."**

- Resume base selected based on stack/goals
- LLM generates:
  - Skills prose block
  - ATS keyword sentence
- Injected into:
  - `resumes/templates/ats_optimized.md`
- Exported:
  - PDF saved to `/resumes/generated/`
  - Copied to desktop
  - Logged with version tag

**Run From:**
- CLI: `phaseops generate-resume`
- Web: Generate button
- HUD: “Build & Load Resume” button

---

### 📤 Phase 2: Application Logging
**"We don't send blind bullets—we track them all."**

- Trigger: Confirm applied via CLI, web, or HUD
- Prompt: Resume version used, effort, method
- Auto-log:
  - Company, title, resume, date, source
- Save to: `logs/applications.csv`
- Auto-commit: `git snapshot` of log + resume

---

### 📥 Phase 3: Feedback Ingestion
**"Did we get hit back?"**

- Input sources:
  - Email (pasted content)
  - LinkedIn (“Viewed,” “Downloaded,” “Rejected”)
- Ingestion point:
  - CLI: `phaseops update-status`
  - Web: status dropdown
  - HUD: LinkedIn status buttons
- Status updated in log
- Optional: LLM summarize rejection

---

### 🧊 Phase 4: Cold Outreach
**"When Easy Apply is dead, we go direct."**

- Input: cold DM/email form
- Track:
  - Contact, company, channel, message, follow-up
- Suggest:
  - Follow-up time
  - Message templates (LLM)
- Stored in: `logs/outreach.json`
- Feedback loop if response arrives

---

### 📊 Phase 5: Review & Analytics Dashboard
**"Adapt. Iterate. Hit harder."**

- Dashboard hosted on `localhost:PORT`
- Features:
  - Searchable, filterable job app table
  - Application heatmaps & response graphs
  - Cold outreach pipeline view
- View from:
  - Browser
  - Embedded HUD panel
  - CLI export (e.g., CSV to Markdown or PDF)

---

### 🤖 Phase 6: Feedback Loop + RAG (Future)
**"The exosuit learns."**

- Store:
  - JD → Resume → Outcome chains
  - Outreach → Response chains
- Embed all into local vector DB
- LLM + RAG system:
  - Recommend resume versions
  - Suggest effort level
  - Reject similar jobs based on history
  - Auto-fill resume with context-aware skill blocks

---

## 🛠️ Tech Stack

| Layer              | Tech                               |
|--------------------|------------------------------------|
| Core Lang          | Rust (`clap`, `tokio`, `serde`)    |
| Web Server         | `axum`, static HTML + Tailwind     |
| Resume Generator   | Markdown → PDF (`printpdf`, `tectonic`) |
| AI Layer           | OpenAI / Gemini via `reqwest`      |
| Data Storage       | CSV + Git / SQLite (TBD)           |
| Vector DB (Future) | `Qdrant`, `Weaviate`, or local Emb |
| Browser HUD        | Tauri App w/ WebView or Chrome Ext |
| Git Ops            | `git2`, auto-commits of logs + PDFs|

---

## 🛡️ Browser HUD (Safe Mode)

- Loads `https://linkedin.com/jobs` inside Tauri or via Chrome Extension
- Displays PhaseOps overlay:
  - Resume selector
  - JD parser
  - Effort class tagger
  - Logging confirm button
- **No DOM automation**
- **No click simulation**
- **All human-initiated**
- **Fully compliant, invisible to LinkedIn's automation detectors**

---

## 🧠 Summary

> PhaseOps is a modular career control suite.  
> You operate. It tracks. It adapts. You evolve.  

