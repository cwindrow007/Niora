#  Niora
___

## **Niora** is a lightweight, open-source application powered by Rust and React that helps you stay consistent with your medications and supports the daily needs of transgender individuals.
Whether you’re taking daily pills, weekly injections, or just need a gentle reminder, Niora runs quietly in the background so you can focus on living your life. It also aims to provide a safe way to share trans-friendly spaces, understand HRT, and track your wellness.
---

##  Features

-  **Custom Reminder Scheduling**
   - Supports daily and weekly medication reminders
   - Define times easily in a simple `reminders.json` file

-  **Customizable Native Notifications**
   - Styled popups using a clean GUI (built with `egui`)
   - Non-intrusive, auto-dismissable messages

-  **Runs in the Background**
   - Minimal resource usage
   - Optional setup to launch on Windows startup

-  **Open Source and Personalizable**
   - Easily customize notification messages
   - Future plans for logs, themes, and sound alerts

---

## Installation

###  Requirements
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- A Windows machine (for now; cross-platform support coming soon)

###  Run Locally

```bash
git clone https://github.com/yourusername/niora.git
cd niora
cargo run --release
```
----
## Roadmap
-  System tray support with pause/exit options
-  Custom notification sounds
-  Clickable notifications or snooze button
-  information in regards to medical symptoms
-  Cross-platform support (macOS/Linux)
-  Editable GUI for setting reminders
 - Encrypted logs and dose tracking
 - Mobile Support
---
## Why Niora?
Niora began as a personal tool I built for myself to manage my HRT medications: Spironolactone twice daily and Injectable Estrogen once weekly.

I started this journey on June 25th, 2025, after learning how important timing and consistency were for my treatment and emotional well-being. I realized how easy it was to forget a dose or feel off due to small disruptions, and I needed something reliable but unobtrusive.

Niora is to be a customizable desktop app that anyone can use to stay on track not just for HRT, but for any kind of medication, supplement, or recurring reminder.

