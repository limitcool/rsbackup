English| [简体中文](README.md)

# rsbackup

[![crates.io](https://img.shields.io/crates/v/rsbackup.svg)](https://crates.io/crates/rsbackup)
[![Docs](https://docs.rs/rsbackup/badge.svg)](https://docs.rs/rsbackup)
[MSRV](https://img.shields.io/badge/rustc-1.78.0+-ab6000.svg)

rsbackup is a cross-platform file backup tool written in Rust, designed to simplify the backup process. It supports Windows, Linux, and macOS, and is open source under the GPL license.

## Installation

You can install rsbackup from crates.io using Cargo:

```bash
cargo install rsbackup
```

Alternatively, you can download the source code of rsbackup from [GitHub](https://github.com/limitcool/rsbackup) and compile and install it using Cargo:

```bash
git clone https://github.com/limitcool/rsbackup.git
cd rsbackup
cargo build --release
cargo install --path .
```

Or, you can directly download the precompiled binary of rsbackup from [GitHub](https://github.com/limitcool/rsbackup) and place it in your system's PATH:

```bash
# Linux
wget https://github.com/limitcool/rsbackup/releases/download/v0.1.0/rsbackup-v0.1.0-x86_64-linux.tar.xz
xz -d rsbackup-v0.1.0-x86_64-linux.tar.xz
tar -xvf rsbackup-v0.1.0-x86_64-linux.tar
chmod +x rsbackup-linux-x86_64
mv rsbackup-linux-x86_64 /usr/local/bin/rsbackup
```

## Usage

1. **Modify Configuration File**: Modify the `config.yaml` file to specify the backup directory, destination, and exclude rules.

   ```yaml
   CheckFrequency: 86400
   BackupItems:
   - BackupDirectory: ''
     BackupDestination: ''
     Exclude: []
     PreBackupCommand: ""
     AfterBackupCommand: ""
   ```

   You can add more backup items as needed.

2. **Run Backup**: Execute the following command in the terminal to start the backup process:

   ```bash
   rsbackup
   ```

   rsbackup will read the configuration file and begin the backup operation.

## Roadmap

rsbackup currently only supports backing up files to local directories. The following destinations are planned for future versions:

- Alibaba Cloud Drive
- Tencent Cloud COS
- Alibaba Cloud OSS
- WebDAV

These features will be released in subsequent versions. Contributions from developers are welcome, and you can participate in the project's development and improvement by submitting Pull Requests.

## License

rsbackup is released under the GPL (GNU General Public License). You can view the source code and contribute on [GitHub](https://github.com/limitcool/rsbackup).
