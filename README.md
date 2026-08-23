# R6 Siege - Hydra Launcher Tracker Wrapper 🎮

[![Language](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows-blue.svg)](https://www.microsoft.com/windows)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

An ultra-lightweight (~240 KB), fully invisible, native **Rust** wrapper designed to enable [Hydra Launcher](https://hydralauncher.gg/) to accurately track gameplay time on **Tom Clancy's Rainbow Six Siege**.

---

## 🎯 Why This Wrapper?

When launching Rainbow Six Siege through third-party launchers:
1. The initial bootstrap executable opens and immediately exits to hand over execution to **Ubisoft Connect** and **BattlEye**.
2. **Hydra Launcher** interprets the initial process exit as the end of the gaming session and prematurely stops tracking playtime.
3. Traditional Batch (`.bat`) scripts and standard shortcuts fail due to fluctuating wait times (Ubisoft cloud synchronization, game updates, anti-cheat handshakes).

This native wrapper solves this issue by remaining active in the background as a lightweight, headless bridge for the entire duration of the actual game session.

---

## ✨ Key Features

- **100% Headless & Invisible**: Compiled with `#![windows_subsystem = "windows"]` and `CREATE_NO_WINDOW` flags so no CMD terminal or prompt ever flashes on screen.
- **Official Launcher Integration**: Starts the game using the official Ubisoft protocol (`uplay://launch/635/0`), ensuring proper authentication, cloud saves, patches, and BattlEye anti-cheat initialization.
- **Custom App ID & Steam Edition Support**: Accepts custom launch arguments (e.g., passing `1843` for linked Steam versions or custom URIs).
- **Startup Grace Period (5-Minute Timeout)** : Waits patiently for the game process to boot up.
- **Anti-Bounce (Debounce) Logic**: Absorbs the transitional gap between the BattlEye splash screen closing and the main game window opening (requires 6 consecutive missed polls / 18 seconds before registering game exit).
- **Near-Zero Resource Usage (~0% CPU)**: Periodic polling utilizing native OS thread sleeps.
- **Clean Exit**: Exits automatically as soon as the game closes, signaling Hydra Launcher to end tracking accurately.
- **Diagnostic Logging**: Writes timestamped event logs to `%TEMP%\r6_tracker.log`.

---

## 🚀 Setup with Hydra Launcher

### 1. Build or Download the Executable
Once compiled (see instructions below), the release binary is located at:
```text
target/release/r6_tracker.exe
```

### 2. Configure Hydra Launcher
1. Open **Hydra Launcher**.
2. Navigate to **Tom Clancy's Rainbow Six Siege**.
3. In game settings, set the executable path to point to:
   `C:\path\to\r6_tracker.exe`
4. Launch the game from Hydra Launcher!

> [!TIP]
> **Steam-Linked Edition or Custom App ID:**
> - By default, the wrapper launches Ubisoft Connect App ID **`635`** (`uplay://launch/635/0`).
> - If your account uses a linked Steam license, specify `1843` in Hydra's launch arguments.

---

## 🛠️ Build from Source

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (1.70+ recommended)
- Operating System: Windows 10 / 11

### Compilation
```powershell
# Clone the repository
git clone https://github.com/LenoreSlate/HydraR6_wrapper.git
cd HydraR6_wrapper

# Build the release binary
cargo build --release
```

The optimized binary will be created in `target/release/r6_tracker.exe`.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
