# ADB Wireless Debug

A modern desktop application for enabling wireless Android debugging with one click. Built with Tauri, Rust, and vanilla JavaScript.

![ADB Wireless Debug](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## Screenshot

![ADB Wireless Debug App](https://github.com/yourusername/adb-wireless/assets/screenshot.png)

## Features

- 🔌 **One-Click Wireless Setup** - Enable wireless debugging instantly
- 📱 **Device Detection** - Automatically detects USB and wireless devices
- 🔄 **Real-time Status** - Live ADB status monitoring
- 📊 **Activity Logs** - Track all operations with detailed logging
- 🎨 **Modern UI** - Clean, dark-themed interface
- ⚡ **Fast & Lightweight** - Built with Tauri for minimal resource usage

## Prerequisites

Before using this application, ensure you have:

- **ADB (Android Debug Bridge)** installed and available in your system PATH
  - Download from [Android SDK Platform Tools](https://developer.android.com/tools/releases/platform-tools)
- **Android device** with USB Debugging enabled
  - Go to Settings → About Phone → Tap "Build Number" 7 times
  - Go to Settings → Developer Options → Enable "USB Debugging"
- **Wi-Fi connection** - Device must be on the same network as your computer

## Installation

### Download Pre-built Binaries

Download the latest release for your platform from the [Releases](https://github.com/yourusername/adb-wireless/releases) page.

### Build from Source

1. Install prerequisites:
   - [Rust](https://rustup.rs/) (latest stable)
   - [Node.js](https://nodejs.org/) (v16 or higher)
   - [ADB](https://developer.android.com/tools/releases/platform-tools)

2. Clone the repository:
```bash
git clone https://github.com/yourusername/adb-wireless.git
cd adb-wireless
```

3. Install dependencies:
```bash
npm install
```

4. Run in development mode:
```bash
npm run tauri dev
```

5. Build for production:
```bash
npm run tauri build
```

## Usage

1. **Connect your Android device via USB**
   - Ensure USB Debugging is enabled
   - Accept the debugging authorization prompt on your device

2. **Launch the application**
   - The app will automatically check for ADB availability
   - Connected devices will appear in the device list

3. **Enable Wireless Debugging**
   - Click "Enable Wireless" on your USB-connected device
   - The app will automatically configure wireless debugging
   - Once connected, you can unplug the USB cable

4. **Disconnect Wireless Devices**
   - Click "Disconnect" on any wireless device to remove it

## How It Works

1. **Detection**: The app runs `adb devices -l` to list all connected devices
2. **IP Discovery**: For USB devices, it queries the device's Wi-Fi IP address using `adb shell ip addr show wlan0`
3. **TCP/IP Mode**: Enables TCP/IP mode on port 5555 using `adb tcpip 5555`
4. **Connection**: Connects to the device wirelessly using `adb connect <ip>:5555`

## Troubleshooting

### ADB Not Found
- Ensure ADB is installed and added to your system PATH
- Restart the application after installing ADB

### Device Not Detected
- Check that USB Debugging is enabled on your device
- Try a different USB cable or port
- Revoke and re-authorize USB debugging on your device

### Cannot Get Device IP
- Ensure your device is connected to Wi-Fi
- Check that your device and computer are on the same network
- Some devices may require additional permissions

### Connection Failed
- Verify your firewall isn't blocking port 5555
- Ensure your device and computer are on the same network
- Try restarting ADB: `adb kill-server && adb start-server`

## Development

### Project Structure

```
adb-wireless/
├── src/                  # Frontend (HTML, CSS, JS)
│   ├── index.html
│   ├── main.js
│   └── styles.css
├── src-tauri/           # Backend (Rust)
│   ├── src/
│   │   ├── lib.rs       # Core ADB logic
│   │   └── main.rs      # Entry point
│   └── Cargo.toml
└── package.json
```

### Tech Stack

- **Frontend**: Vanilla JavaScript, HTML5, CSS3
- **Backend**: Rust with Tauri
- **Build Tool**: Tauri CLI
- **ADB**: Android Debug Bridge

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/)
- Uses [Android Debug Bridge](https://developer.android.com/tools/adb)
- Inspired by the need for simpler wireless debugging workflows

## Support

If you encounter any issues or have questions:
- Open an [issue](https://github.com/yourusername/adb-wireless/issues)
- Check existing issues for solutions
- Read the troubleshooting section above

---

Made with ❤️ for Android developers
