# OPEN QUESTIONS & HUMAN DECISIONS

Questions for the human, tracking answered decisions and proposed defaults.

## Answered Questions
| Date | Question | Decision |
|---|---|---|
| 2026-10-15 | Software License | **Dual MIT / Apache-2.0** |
| 2026-10-15 | Distribution Package | **NSIS executable installer + Portable ZIP** (`xhair.portable` marker) |
| 2026-10-15 | Memory Graph Backend | **In-repo JSONL only** (`docs/agent/graph/`) |

## Open / Pending Questions
| ID | Topic | Question | Proposed Default | Status |
|---|---|---|---|---|
| **Q1** | **Product Branding** | Final product name and icon direction | Name: `xhair`, gaming minimalist reticle icon | Pending confirmation |
| **Q2** | **Accent Theme** | Preferred primary accent color for dark theme | Neon Cyan `#00F5D4` / `#00D2FF` | Pending confirmation |
| **Q3** | **Code Signing** | Is a Windows Authenticode signing certificate available now or later? | Unsigned for initial dev; document SmartScreen warning | Accepted default |
| **Q4** | **Pre-release** | Is a beta/nightly pre-release channel desired? | Off by default (standard SemVer releases on main) | Accepted default |
| **Q5** | **GitHub Actions** | Is "Allow GitHub Actions to create and approve pull requests" enabled on the repository settings? | Must be enabled in repo Settings -> Actions -> General for release-please | Action item for user |
