# MySender

An open-source, cross-platform app to share files and folders between devices on the same local network (Wi-Fi / hotspot). No internet required, no cloud servers, no file size limits.

Everything is transferred peer-to-peer with TLS encryption, straight from device to device at full local network speed.

---

## Downloads

Download pre-built binaries from the [Releases](https://github.com/saferill/MySender/releases/latest) page. Windows binaries code signing is provided by the [SignPath Foundation](https://signpath.org).

| Platform | Format | Description |
| :--- | :--- | :--- |
| **Windows** | `.exe` | Standard setup installer |
| | `.zip` | Portable version (extract and run) |
| **Android** | `arm64-v8a.apk` | For most modern Android phones (64-bit) |
| | `armeabi-v7a.apk` | For older 32-bit Android phones |
| | `x86_64.apk` | For emulators / ChromeOS |
| | `universal.apk` | All-in-one APK (works on any device) |
| | `.aab` | Google Play Store bundle |
| **Linux** | `.tar.gz` | Standalone binary bundle (x86_64) |
| **macOS** | `.dmg` | Universal installer (Apple Silicon & Intel) |
| | `.zip` | Portable standalone app |
| **iOS** | `.ipa` | Sideload package (AltStore / TrollStore) |

---

## Features

- **Direct P2P transfer**: Fast local transfer over Wi-Fi without going through third-party servers.
- **End-to-end security**: All connections are encrypted with local HTTPS/TLS certificates.
- **Web Send**: Share files directly to a browser on any device via local IP or QR code without installing the app.
- **Cross-platform**: Runs on Android, Windows, Linux, macOS, and iOS.
- **Privacy first**: No analytics, no telemetry, no accounts, and no ads.

---

## How It Works

1. **Discovery**: Devices on the same Wi-Fi find each other automatically using UDP multicast / local network scan.
2. **Handshake & Transfer**: When you send a file, the receiver gets a prompt to accept or decline.
3. **Rust Core**: Heavy network IO and cryptographic operations are handled by a lightweight Rust core backend for maximum throughput and low memory usage.

---

## Building from Source

### Prerequisites
- [Flutter SDK](https://flutter.dev/) (pinned with FVM: `3.38.3`)
- [Rust Toolchain](https://rustup.rs/) (stable)

### 1. Setup

```bash
# Clone the repository
git clone https://github.com/saferill/MySender.git
cd MySender/app

# Install Dart dependencies
fvm flutter pub get
```

### 2. Run / Build

```bash
# Run locally in debug mode
fvm flutter run

# Build for Windows
fvm flutter build windows

# Build split APKs for Android
fvm flutter build apk --split-per-abi

# Build for Linux
fvm flutter build linux
```

---

## Privacy & Security

MySender is strictly local-only:
- No telemetry or crash analytics are sent to any remote server.
- Files never touch external servers or the cloud.
- Check [PRIVACY.md](PRIVACY.md) for full privacy details.

---

## License & Credits

- Licensed under the [Apache License 2.0](LICENSE).
- Based on the open-source project [LocalSend](https://github.com/localsend/localsend) by Tien Do Nam & contributors.
