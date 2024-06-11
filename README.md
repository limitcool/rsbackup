[English](README-en.md) | 简体中文
# rsbackup

[![crates.io](https://img.shields.io/crates/v/rsbackup.svg)](https://crates.io/crates/rsbackup)
[![Docs](https://docs.rs/rsbackup/badge.svg)](https://docs.rs/rsbackup)
[![MSRV](https://img.shields.io/badge/rustc-1.78.0+-ab6000.svg)]

rsbackup 是一个用 Rust 编写的跨平台文件备份工具，旨在简化备份流程。它支持 Windows、Linux 和 macOS，并且使用 GPL 协议进行开源。

## 安装
你可以使用Cargo从crates.io安装rsbackup：
```bash
cargo install rsbackup
```
您可以从 [GitHub](https://github.com/limitcool/rsbackup) 下载 rsbackup 的源码，然后使用 cargo 工具进行编译和安装：

```bash
git clone https://github.com/limitcool/rsbackup.git
cd rsbackup
cargo build --release
cargo install --path .
```

或者，您也可以直接从 [GitHub](https://github.com/limitcool/rsbackup) 下载 rsbackup 的二进制文件，然后将其放到您的系统路径中：

```bash
# Linux
wget https://github.com/limitcool/rsbackup/releases/download/v0.1.0/rsbackup-v0.1.0-x86_64-linux.tar.xz
xz -d rsbackup-v0.1.0-x86_64-linux.tar.xz
tar -xvf rsbackup-v0.1.0-x86_64-linux.tar
chmod +x rsbackup-linux-x86_64
mv rsbackup-linux-x86_64 /usr/local/bin/rsbackup
```



## 使用说明

1. **修改配置文件**：修改名为 `config.yaml` 的配置文件，指定备份目录、目标位置和排除规则。

   ```yaml
   CheckFrequency: 86400
   BackupItems:
   - BackupDirectory: ''
     BackupDestination: ''
     Exclude: []
     PreBackupCommand: ""
     AfterBackupCommand: ""
   ```

   您可以根据需要添加更多的备份项。

2. **执行备份**：在命令行中执行以下命令来启动备份过程：

   ```bash
   rsbackup
   ```

   rsbackup 将读取配置文件并开始执行备份操作。

## Roadmap

rsbackup 目前只支持将文件备份到本地目录。未来计划支持以下目的地：

- 阿里云盘
- 腾讯云 COS
- 阿里云 OSS
- WebDAV

这些功能将在后续版本中陆续推出。欢迎开发者贡献代码，您可以通过提交 Pull Request 的方式参与项目的建设和完善。

## 协议

rsbackup 使用 GPL（GNU 通用公共许可证）开源协议进行发布。您可以在 [GitHub 仓库](https://github.com/limitcool/rsbackup) 中查看源代码并参与贡献。
