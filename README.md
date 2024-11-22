# 🔧 Fixr - Portable Disk Management Tool

![License](https://img.shields.io/badge/license-MIT-blue.svg)  
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)  

Fixr is a command-line tool designed to manage and repair portable disks on Windows systems efficiently.

## ✨ Features

- 📝 List all portable disks in the system
- 🔍 View disk details (size, usage, etc.)
- 🛠️ Perform disk repair operations
- 🎨 Colorful and user-friendly interface

## 🚀 Installation

```bash
# Clone the repository
git clone https://github.com/username/fixr.git

# Navigate to the project directory
cd fixr

# Build the application
cargo build --release

# The executable will be available in the target/release directory
```

## 📖 Usage

### Listing Portable Disks

```bash
# Simple list
fixr list

# Detailed list (with size information)
fixr list --verbose
```

### Viewing Disk Information

```bash
# Display details of the F: drive
fixr info F:
```

### Repairing a Disk

```bash
# Basic repair
fixr fix F:

# Force repair (use with caution!)
fixr fix F: --force
```

## 🔍 Command Details

### `list` Command
- Lists all portable disks in the system
- Displays detailed information with the `--verbose` parameter

### `info` Command
- Shows detailed information for the specified disk:
  - Total space
  - Used space
  - Free space
  - Usage percentage

### `fix` Command
- Repairs the specified disk
- Supports forced repair with the `--force` parameter
- Utilizes Windows' `chkdsk` tool

## ⚠️ Important Notes

1. Back up your important data before performing repair operations.  
2. Use the `--force` parameter carefully.  
3. The program may require Administrator privileges to run.  

## 🛠️ Development

### Requirements

- Rust 1.75 or later
- Windows operating system
- Cargo and related tools

### Dependencies

- **clap**: Command-line argument handling  
- **colored**: Terminal text coloring  
- **windows**: Windows API integration  

## 📝 License

This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.

## 🤝 Contributing

1. Fork this repository.  
2. Create a new branch (`git checkout -b feature/newFeature`).  
3. Commit your changes (`git commit -am 'Added a new feature'`).  
4. Push your branch (`git push origin feature/newFeature`).  
5. Create a Pull Request.  

## 📞 Contact

For questions or suggestions, please open an issue on GitHub.  
