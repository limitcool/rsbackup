use std::{fs::OpenOptions, io::BufWriter, process::exit};

use config::BackupConfig;
use storage::{local::LocalStorage, Storage};

use tracing::info;
use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::fmt;

mod config;
mod storage;
mod util;

fn main() {
    let version: &str = env!("CARGO_PKG_VERSION");
    info!("rsbackup version : {}", version);
    println!("rsbackup version : {}", version);
    // 创建 OpenOptions 并设置为追加模式
    let file = OpenOptions::new()
        .create(true) // 如果文件不存在则创建文件
        .append(true) // 以追加模式打开文件
        .open(util::log_file())
        .expect("failed to open log file");

    // 使用 BufWriter 包装文件
    let writer = BufWriter::new(file);

    // 创建一个非阻塞的文件 appender
    let (non_blocking, _guard) = NonBlocking::new(writer);

    // 将日志输出到控制台和文件
    let subscriber = fmt()
        .with_max_level(tracing::metadata::LevelFilter::DEBUG)
        .with_writer(non_blocking)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // 加载配置文件
    // let config = match config::load_config("config.yaml") {
    //     Ok(config) => config,
    //     Err(e) => {
    //         tracing::error!("Error loading config: {}", e);
    //         return;
    //     }
    // };
    let config = BackupConfig::new().unwrap();
    tracing::info!("Backup Check Frequency: {}", config.check_frequency);

    // 遍历配置项，执行备份操作
    let mut errors = Vec::new(); // 用于收集错误
    for (index, item) in config.backup_items.iter().enumerate() {
        if item.backup_destination.is_empty() || item.backup_directory.is_empty() {
            info!(
                "Please go to the directory '{}' to modify the backup configuration file settings.",
                BackupConfig::config_file().display()
            );
            eprintln!(
                "Please go to the directory '{}' to modify the backup configuration file settings.",
                BackupConfig::config_file().display()
            );
            exit(0)
        }
        tracing::info!("Backup Item {}", index + 1);
        tracing::info!("  Backup Directory: {}", item.backup_directory);
        tracing::info!("  Backup Destination: {}", item.backup_destination);
        if let Some(exclude) = &item.exclude {
            if !exclude.is_empty() {
                tracing::info!("  Exclude:");
                for ex in exclude {
                    tracing::info!("    {}", ex);
                }
            }
        }
        if item.pre_backup_command.is_some() {
            tracing::info!(
                "  Pre Backup Command: {}",
                item.pre_backup_command.as_ref().unwrap()
            );
            util::run_command(item.pre_backup_command.as_ref().unwrap());
        }

        // 创建本地存储实例，传入备份目录路径
        let storage = LocalStorage::new(&item.backup_destination);

        // 执行备份操作，传入源文件路径和备份目的地目录路径
        if let Err(err) = storage.store_file(
            &item.backup_directory,
            &item.backup_destination,
            item.exclude.as_ref().map_or(&[], |vec| vec.as_slice()),
        ) {
            tracing::error!("  Backup error: {}", err);
            errors.push(err); // 收集错误信息
        }

        if item.after_backup_command.is_some() {
            tracing::info!(
                "  After Backup Command: {}",
                item.after_backup_command.as_ref().unwrap()
            );
            util::run_command(item.after_backup_command.as_ref().unwrap());
        }
    }
    println!("task finish!");
    info!("task finish!");
    // 如果有错误发生，在此处处理
    if !errors.is_empty() {}
}
