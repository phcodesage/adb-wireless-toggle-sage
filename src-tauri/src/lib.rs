use serde::{Deserialize, Serialize};
use std::process::Command;

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

/// Check if ADB is reachable on the system
#[tauri::command]
fn get_adb_status() -> CommandResult {
    match Command::new("adb").arg("version").output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            if output.status.success() {
                let version_line = stdout.lines().next().unwrap_or("ADB found").to_string();
                CommandResult {
                    success: true,
                    message: version_line,
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
            message: format!("ADB not found: {}", e),
        },
    }
}

/// List all connected ADB devices
#[tauri::command]
fn list_devices() -> Result<Vec<Device>, String> {
    let output = Command::new("adb")
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
fn get_device_ip(serial: &str) -> Result<String, String> {
    // Try via ip addr show wlan0
    let output = Command::new("adb")
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
    let output2 = Command::new("adb")
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
fn enable_wireless(serial: String, port: Option<u16>) -> CommandResult {
    let port = port.unwrap_or(5555);
    let port_str = port.to_string();

    // Step 1: Enable TCP/IP mode
    let tcpip_output = Command::new("adb")
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
    let ip = match get_device_ip(&serial) {
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
    let connect_output = Command::new("adb")
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
fn disconnect_device(address: String) -> CommandResult {
    let output = Command::new("adb")
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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_adb_status,
            list_devices,
            enable_wireless,
            disconnect_device,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
