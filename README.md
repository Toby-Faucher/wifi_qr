# WiFi QR Code Generator

A Rust-based command-line tool that generates QR codes for WiFi network connections, allowing users to easily share WiFi credentials by scanning a QR code.

## Features

- Generate QR codes for WiFi networks with SSID and password
- Support for multiple security types (WPA/WPA2, WEP, Open networks)
- Multiple output formats (PNG, SVG, ASCII terminal display)
- Customizable QR code appearance and error correction levels
- Batch processing for multiple networks
- Secure password input and validation

## Development Roadmap

  Critical Missing Features:
  - No unit or integration tests
  - Missing error correction level options
  - No batch processing capabilities
  - Limited output format support

  Performance Opportunities:
  - QR code object caching for batch operations
  - Memory optimization for large image generation
  - Parallel processing implementation
  - Streaming for large files

  Security & Validation Gaps:
  - Password strength validation missing
  - Secure password input not implemented
  - Limited input validation beyond basic escaping

  User Experience Enhancements:
  - Configuration file support
  - Template system for common networks
  - Progress indicators for operations
  - Clipboard integration
  - Better terminal rendering

  Protocol & Standards:
  - WPA3-SAE proper implementation
  - WiFi profile export formats
  - QR code verification functionality

## Getting Started

### Prerequisites
- Rust 1.70+ (2024 edition support)
- Cargo package manager

### Installation
```bash
# Clone the repository
git clone <repository-url>
cd wifi_qr

# Build the project
cargo build --release

# Run the application
cargo run
```

## Usage

*Usage examples will be added as features are implemented*

## WiFi QR Code Format

The tool generates QR codes using the standard WiFi QR code format:
```
WIFI:T:<security>;S:<SSID>;P:<password>;H:<hidden>;;
```

Where:
- `T`: Security type (WPA, WEP, or blank for open)
- `S`: SSID (network name)
- `P`: Password (blank for open networks)
- `H`: Hidden network flag (true/false, optional)

## Contributing

1. Check the development roadmap above
2. Pick an unchecked item from the appropriate phase
3. Create a feature branch
4. Implement the feature with tests
5. Submit a pull request

## License

*License information to be added*
