# Architecture Overview

This document provides an overview of the ADB Wireless Debug application architecture.

## Technology Stack

### Frontend
- **HTML5/CSS3**: Modern, responsive UI
- **Vanilla JavaScript**: No framework dependencies for minimal bundle size
- **Custom CSS**: Dark theme with smooth animations

### Backend
- **Rust**: High-performance, memory-safe backend
- **Tauri**: Cross-platform desktop framework
- **ADB CLI**: Android Debug Bridge command-line interface

## Application Flow

```
┌─────────────────────────────────────────────────────────────┐
│                         Frontend (JS)                        │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │  Device  │  │   ADB    │  │ Wireless │  │   Log    │   │
│  │   List   │  │  Status  │  │  Enable  │  │  Panel   │   │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘   │
│       │             │              │             │          │
└───────┼─────────────┼──────────────┼─────────────┼──────────┘
        │             │              │             │
        │      Tauri IPC (invoke)    │             │
        │             │              │             │
┌───────▼─────────────▼──────────────▼─────────────▼──────────┐
│                      Backend (Rust)                          │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              Tauri Command Handlers                  │   │
│  │  • get_adb_status()                                  │   │
│  │  • list_devices()                                    │   │
│  │  • enable_wireless(serial, port)                     │   │
│  │  • disconnect_device(address)                        │   │
│  └────────────────────┬─────────────────────────────────┘   │
│                       │                                      │
│  ┌────────────────────▼─────────────────────────────────┐   │
│  │              ADB Command Executor                    │   │
│  │  • std::process::Command                             │   │
│  │  • Executes ADB CLI commands                         │   │
│  │  • Parses output                                     │   │
│  └────────────────────┬─────────────────────────────────┘   │
└───────────────────────┼───────────────────────────────────────┘
                        │
                        │ System Calls
                        │
┌───────────────────────▼───────────────────────────────────────┐
│                    ADB (Android Debug Bridge)                 │
│  • adb devices -l                                             │
│  • adb -s <serial> shell ip addr show wlan0                   │
│  • adb -s <serial> tcpip 5555                                 │
│  • adb connect <ip>:5555                                      │
│  • adb disconnect <address>                                   │
└───────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Frontend (src/)

#### index.html
- Main application structure
- Semantic HTML5 markup
- Responsive layout

#### main.js
- Event handling and UI updates
- Tauri IPC communication via `window.__TAURI__.core.invoke()`
- DOM manipulation
- Logging system

#### styles.css
- Modern dark theme
- CSS custom properties for theming
- Responsive design
- Smooth animations and transitions

### 2. Backend (src-tauri/src/)

#### lib.rs
Core application logic with four main command handlers:

**Data Structures:**
```rust
struct Device {
    serial: String,      // Device identifier
    status: String,      // "device", "offline", etc.
    device_type: String, // "usb" or "wireless"
}

struct CommandResult {
    success: bool,
    message: String,
}
```

**Commands:**

1. `get_adb_status()` → CommandResult
   - Checks if ADB is available
   - Runs `adb version`
   - Returns version info or error

2. `list_devices()` → Result<Vec<Device>, String>
   - Executes `adb devices -l`
   - Parses output to extract device info
   - Determines device type (USB vs wireless)

3. `enable_wireless(serial, port)` → CommandResult
   - Gets device IP via `get_device_ip()`
   - Enables TCP/IP mode: `adb tcpip 5555`
   - Connects wirelessly: `adb connect <ip>:5555`

4. `disconnect_device(address)` → CommandResult
   - Disconnects wireless device
   - Runs `adb disconnect <address>`

**Helper Functions:**

- `get_device_ip(serial)` → Result<String, String>
  - Queries device Wi-Fi IP
  - Tries `ip addr show wlan0`
  - Falls back to `ip route` parsing

#### main.rs
- Application entry point
- Minimal wrapper that calls `lib.rs::run()`
- Windows subsystem configuration

## Communication Flow

### Device List Refresh
```
User clicks "Refresh" 
  → JS: refreshDevices()
  → Tauri IPC: invoke("list_devices")
  → Rust: list_devices()
  → System: adb devices -l
  → Rust: Parse output → Vec<Device>
  → JS: renderDevices(devices)
  → DOM: Update UI
```

### Enable Wireless Debugging
```
User clicks "Enable Wireless"
  → JS: enableWireless(serial)
  → Tauri IPC: invoke("enable_wireless", {serial})
  → Rust: enable_wireless(serial, port)
  → System: adb -s <serial> shell ip addr show wlan0
  → Rust: get_device_ip() → IP address
  → System: adb -s <serial> tcpip 5555
  → System: adb connect <ip>:5555
  → Rust: CommandResult {success, message}
  → JS: Update UI + refresh device list
```

## Security Considerations

1. **No Network Exposure**: App doesn't open any network ports
2. **Local ADB Only**: All commands go through local ADB installation
3. **No Data Storage**: No persistent storage of device info
4. **Command Injection**: Input sanitization via Rust's type system
5. **User Authorization**: Relies on Android's USB debugging authorization

## Error Handling

### Frontend
- Try-catch blocks around all Tauri invocations
- User-friendly error messages in logs
- Graceful degradation (empty states)

### Backend
- Result types for fallible operations
- Detailed error messages
- Command output parsing with fallbacks

## Performance Considerations

1. **Minimal Dependencies**: Vanilla JS frontend, lean Rust backend
2. **Async Operations**: Non-blocking UI during ADB commands
3. **Small Bundle Size**: Tauri produces ~10MB installers
4. **Low Memory Usage**: Rust's zero-cost abstractions
5. **Fast Startup**: No heavy frameworks to initialize

## Future Enhancements

Potential areas for improvement:

- [ ] Custom port configuration
- [ ] Device pairing support (Android 11+)
- [ ] Multiple device selection
- [ ] Connection profiles/favorites
- [ ] Auto-reconnect on disconnect
- [ ] System tray integration
- [ ] Notification support
- [ ] Dark/light theme toggle
- [ ] Internationalization (i18n)
- [ ] Unit and integration tests
