use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub serial: String,
    pub status: String,
    pub device_type: String, // "usb" or "wireless"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdbConfig {
    pub path: String,
}

// Global state to store the ADB path
pub struct AdbState {
    pub adb_path: Mutex<String>,
}

impl AdbState {
    pub fn new() -> Self {
        Self {
            adb_path: Mutex::new(String::new()),
        }
    }
}

/// Find ADB in common locations
fn find_adb() -> Option<String> {
    // First, try the system PATH
    if let Ok(output) = Command::new("adb").arg("version").output() {
        if output.status.success() {
            return Some("adb".to_string());
        }
    }

    // Common ADB locations on macOS
    let common_paths = [
        "/usr/local/bin/adb",
        "/opt/homebrew/bin/adb",
        "/usr/bin/adb",
        "/bin/adb",
        "~/Library/Android/sdk/platform-tools/adb",
        "~/android-sdk/platform-tools/adb",
        "/Applications/Android Studio.app/Contents/sdk/platform-tools/adb",
        "~/Library/Android/sdk/platform-tools/adb.exe",
    ];

    for path in &common_paths {
        let expanded = shellexpand::tilde(path);
        let path_str = expanded.as_ref();
        if std::path::Path::new(path_str).exists() {
            // Verify it's actually working
            if let Ok(output) = Command::new(path_str).arg("version").output() {
                if output.status.success() {
                    return Some(path_str.to_string());
                }
            }
        }
    }

    None
}

/// Get the current ADB path from state or find it
fn get_adb_path(state: &tauri::State<AdbState>) -> String {
    // Try to get from state first
    if let Ok(path) = state.adb_path.lock() {
        if !path.is_empty() {
            return path.clone();
        }
    }

    // Find and cache the path
    if let Some(path) = find_adb() {
        if let Ok(mut state_path) = state.adb_path.lock() {
            *state_path = path.clone();
        }
        return path;
    }

    // Fallback to "adb" and let it fail with a clear error
    "adb".to_string()
}

/// Check if ADB is reachable on the system
#[tauri::command]
fn get_adb_status(state: tauri::State<AdbState>) -> CommandResult {
    let adb_path = get_adb_path(&state);

    match Command::new(&adb_path).arg("version").output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            if output.status.success() {
                let version_line = stdout.lines().next().unwrap_or("ADB found").to_string();
                CommandResult {
                    success: true,
                    message: format!("{} (path: {})", version_line, adb_path),
                }
            } else {
                CommandResult {
                    success: false,
                    message: "ADB found but returned an error".to_string(),
                }
            }
        }
        Err(e) => CommandResult {
            success: false,
            message: format!("ADB not found: {}. Please install ADB or set the correct path in settings.", e),
        },
    }
}

/// Get the current ADB path
#[tauri::command]
fn get_adb_config(state: tauri::State<AdbState>) -> AdbConfig {
    let path = get_adb_path(&state);
    AdbConfig { path }
}

/// Set a custom ADB path
#[tauri::command]
fn set_adb_path(config: AdbConfig, state: tauri::State<AdbState>) -> CommandResult {
    // Verify the path works
    match Command::new(&config.path).arg("version").output() {
        Ok(output) => {
            if output.status.success() {
                if let Ok(mut path) = state.adb_path.lock() {
                    *path = config.path;
                }
                CommandResult {
                    success: true,
                    message: "ADB path updated successfully".to_string(),
                }
            } else {
                CommandResult {
                    success: false,
                    message: "Invalid ADB path: command returned an error".to_string(),
                }
            }
        }
        Err(e) => CommandResult {
            success: false,
            message: format!("Failed to verify ADB path: {}", e),
        },
    }
}

/// List all connected ADB devices
#[tauri::command]
fn list_devices(state: tauri::State<AdbState>) -> Result<Vec<Device>, String> {
    let adb_path = get_adb_path(&state);
    let output = Command::new(&adb_path)
        .arg("devices")
        .arg("-l")
        .output()
        .map_err(|e| format!("Failed to run adb devices: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let mut devices = Vec::new();

    for line in stdout.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let serial = parts[0].to_string();
            let status = parts[1].to_string();

            let device_type = if serial.contains(':') {
                "wireless".to_string()
            } else {
                "usb".to_string()
            };

            devices.push(Device {
                serial,
                status,
                device_type,
            });
        }
    }

    Ok(devices)
}

/// Get the Wi-Fi IP address of a USB-connected device
fn get_device_ip(serial: &str, adb_path: &str) -> Result<String, String> {
    // Try via ip addr show wlan0
    let output = Command::new(adb_path)
        .args(["-s", serial, "shell", "ip", "addr", "show", "wlan0"])
        .output()
        .map_err(|e| format!("Failed to get IP: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    // Parse "inet X.X.X.X/..." from output
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("inet ") && !trimmed.starts_with("inet6") {
            if let Some(addr_part) = trimmed.split_whitespace().nth(1) {
                let ip = addr_part.split('/').next().unwrap_or("").to_string();
                if !ip.is_empty() && ip != "127.0.0.1" {
                    return Ok(ip);
                }
            }
        }
    }

    // Fallback: try ip route
    let output2 = Command::new(adb_path)
        .args(["-s", serial, "shell", "ip", "route"])
        .output()
        .map_err(|e| format!("Failed to get IP via route: {}", e))?;

    let stdout2 = String::from_utf8_lossy(&output2.stdout).to_string();

    for line in stdout2.lines() {
        // Look for "src X.X.X.X"
        if let Some(src_pos) = line.find("src ") {
            let ip = line[src_pos + 4..]
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string();
            if !ip.is_empty() && ip != "127.0.0.1" {
                return Ok(ip);
            }
        }
    }

    Err("Could not detect device Wi-Fi IP. Make sure the device is connected to Wi-Fi.".to_string())
}

/// Enable wireless debugging: sets TCP/IP mode, detects IP, and connects
#[tauri::command]
fn enable_wireless(serial: String, port: Option<u16>, state: tauri::State<AdbState>) -> CommandResult {
    let adb_path = get_adb_path(&state);
    let port = port.unwrap_or(5555);
    let port_str = port.to_string();

    // Step 1: Enable TCP/IP mode
    let tcpip_output = Command::new(&adb_path)
        .args(["-s", &serial, "tcpip", &port_str])
        .output();

    match tcpip_output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if !output.status.success() {
                return CommandResult {
                    success: false,
                    message: format!(
                        "Failed to enable TCP/IP mode: {}",
                        if stderr.is_empty() { &stdout } else { &stderr }
                    ),
                };
            }
        }
        Err(e) => {
            return CommandResult {
                success: false,
                message: format!("Failed to run adb tcpip: {}", e),
            };
        }
    }

    // Brief pause to let the device restart in TCP/IP mode
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Step 2: Get device IP
    let ip = match get_device_ip(&serial, &adb_path) {
        Ok(ip) => ip,
        Err(e) => {
            return CommandResult {
                success: false,
                message: e,
            };
        }
    };

    // Step 3: Connect wirelessly
    let connect_addr = format!("{}:{}", ip, port);
    let connect_output = Command::new(&adb_path)
        .args(["connect", &connect_addr])
        .output();

    match connect_output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let msg = stdout.trim().to_string();

            if msg.contains("connected") {
                CommandResult {
                    success: true,
                    message: format!("✅ Connected wirelessly to {} — you can unplug USB!", connect_addr),
                }
            } else {
                CommandResult {
                    success: false,
                    message: format!(
                        "Connection attempt returned: {} {}",
                        msg,
                        stderr.trim()
                    ),
                }
            }
        }
        Err(e) => CommandResult {
            success: false,
            message: format!("Failed to run adb connect: {}", e),
        },
    }
}

/// Disconnect a wireless device
#[tauri::command]
fn disconnect_device(address: String, state: tauri::State<AdbState>) -> CommandResult {
    let adb_path = get_adb_path(&state);
    let output = Command::new(&adb_path)
        .args(["disconnect", &address])
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            CommandResult {
                success: true,
                message: stdout.trim().to_string(),
            }
        }
        Err(e) => CommandResult {
            success: false,
            message: format!("Failed to disconnect: {}", e),
        },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AdbState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_adb_status,
            get_adb_config,
            set_adb_path,
            list_devices,
            enable_wireless,
            disconnect_device,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
