#![windows_subsystem = "windows"]

#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sysinfo::{ProcessesToUpdate, System};

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// ID par défaut pour Rainbow Six Siege sur Ubisoft Connect (635).
const DEFAULT_LAUNCH_URI: &str = "uplay://launch/635/0";

/// Timeout maximum pour détecter le démarrage initial (5 minutes = 300 s)
const MAX_STARTUP_WAIT_SECS: u64 = 300;

/// Intervalle de vérification durant la phase de lancement (en secondes)
const STARTUP_CHECK_INTERVAL_SECS: u64 = 1;

/// Intervalle de vérification durant la session de jeu (en secondes)
const RUNNING_CHECK_INTERVAL_SECS: u64 = 3;

/// Nombre de vérifications consécutives sans processus nécessaires pour valider la fermeture (6 * 3s = 18s).
/// Absorbe les micro-coupures de transition (BattlEye splash -> Ubisoft sync -> RainbowSix.exe).
const MAX_CONSECUTIVE_MISSES: u32 = 6;

fn log(msg: &str) {
    if let Ok(mut temp_path) = std::env::var("TEMP") {
        temp_path.push_str("\\r6_tracker.log");
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&temp_path) {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let _ = writeln!(file, "[{}] {}", timestamp, msg);
        }
    }
}

fn is_target_process(name: &str) -> bool {
    let lower = name.to_lowercase();
    // Exclure notre propre binaire
    if lower.contains("r6_tracker") {
        return false;
    }

    lower == "rainbowsix.exe"
        || lower == "rainbowsix"
        || lower == "rainbowsix_vulkan.exe"
        || lower == "rainbowsix_vulkan"
        || lower == "rainbowsix_dx11.exe"
        || lower == "rainbowsix_dx11"
        || lower == "rainbowsix_be.exe"
        || lower == "rainbowsix_be"
        || lower == "rainbowsixhelper.exe"
        || lower == "rainbowsixhelper"
}

fn count_game_processes(system: &mut System) -> usize {
    system.refresh_processes(ProcessesToUpdate::All, true);
    system.processes().values().filter(|proc| {
        let name = proc.name().to_string_lossy();
        is_target_process(&name)
    }).count()
}

fn launch_game() {
    let launch_target = std::env::args().nth(1).map(|arg| {
        if arg.chars().all(|c| c.is_ascii_digit()) {
            format!("uplay://launch/{}/0", arg)
        } else {
            arg
        }
    }).unwrap_or_else(|| DEFAULT_LAUNCH_URI.to_string());

    log(&format!("Launching target: {}", launch_target));

    let mut cmd = Command::new("cmd");
    cmd.args(["/C", "start", "", &launch_target]);
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let _ = cmd.spawn();
}

fn main() {
    log("=== R6 Tracker Started ===");
    launch_game();

    let mut system = System::new();

    // Phase 1 : Attente du lancement initial du jeu ou du splash BattlEye
    log("Phase 1: Waiting for game process to appear...");
    let mut initial_detected = false;
    for _ in 0..(MAX_STARTUP_WAIT_SECS / STARTUP_CHECK_INTERVAL_SECS) {
        thread::sleep(Duration::from_secs(STARTUP_CHECK_INTERVAL_SECS));
        let count = count_game_processes(&mut system);
        if count > 0 {
            log(&format!("Game process detected (count: {}). Transitioning to Phase 2.", count));
            initial_detected = true;
            break;
        }
    }

    if !initial_detected {
        log("Timeout: Game process was not detected within 5 minutes. Exiting.");
        return;
    }

    // Phase 2 : Surveillance active avec Debounce
    // Empêche Hydra de stopper le suivi pendant le court intervalle entre le splash BattlEye et le jeu principal
    let mut consecutive_misses = 0;
    loop {
        thread::sleep(Duration::from_secs(RUNNING_CHECK_INTERVAL_SECS));
        let count = count_game_processes(&mut system);
        if count > 0 {
            if consecutive_misses > 0 {
                log(&format!("Game process active again (count: {}). Resetting misses to 0.", count));
            }
            consecutive_misses = 0;
        } else {
            consecutive_misses += 1;
            log(&format!(
                "Game process not found. Consecutive misses: {}/{}",
                consecutive_misses, MAX_CONSECUTIVE_MISSES
            ));
            if consecutive_misses >= MAX_CONSECUTIVE_MISSES {
                log("Game process confirmed closed. Terminating tracker.");
                break;
            }
        }
    }
    log("=== R6 Tracker Exited ===");
}
