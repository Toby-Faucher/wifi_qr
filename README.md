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

### Phase 1: Core Setup and Dependencies
- [ ] Set up Rust project structure with Cargo.toml and dependencies
- [ ] Add QR code generation library dependency (qrcode crate)
- [ ] Add image handling dependencies (image crate for PNG/JPEG output)
- [ ] Add command line argument parsing (clap crate)

### Phase 2: Core WiFi QR Code Functionality
- [ ] Create WiFi configuration struct to hold SSID, password, security type
- [ ] Implement WiFi QR code format string generation (WIFI:T:WPA;S:ssid;P:password;;)
- [ ] Create QR code generation function that takes WiFi config and returns QR code
- [ ] Add support for hidden network flag in WiFi QR format

### Phase 3: User Interface
- [ ] Implement command line interface for SSID input
- [ ] Implement command line interface for password input (with secure input option)
- [ ] Implement command line interface for security type selection (WPA/WPA2/WEP/Open)
- [ ] Add proper error handling and user-friendly error messages

### Phase 4: Output Formats and Customization
- [ ] Implement PNG image output functionality
- [ ] Implement SVG output functionality for scalable graphics
- [ ] Add terminal/console ASCII art QR code display option
- [ ] Add customizable QR code size options
- [ ] Add error correction level options (Low, Medium, Quartile, High)
- [ ] Implement file output with custom filename support

### Phase 5: Advanced Features
- [ ] Add color customization options for QR code foreground/background
- [ ] Implement quiet zones and border options for QR code output
- [ ] Add support for reading WiFi config from file (JSON/TOML)
- [ ] Implement batch processing for multiple WiFi networks
- [ ] Implement logging for debugging and verbose output options

### Phase 6: Input Validation and Security
- [ ] Implement input validation for SSID (length limits, special characters)
- [ ] Implement input validation for password strength and requirements

### Phase 7: Testing and Quality Assurance
- [ ] Create comprehensive unit tests for WiFi format generation
- [ ] Create integration tests for QR code generation
- [ ] Add cross-platform compatibility testing
- [ ] Optimize performance for large batch operations

### Phase 8: Documentation and Distribution
- [ ] Add documentation with usage examples and help text
- [ ] Add version information and build metadata
- [ ] Create installation and build instructions

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