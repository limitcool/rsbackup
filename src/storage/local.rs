use regex::Regex;
use tracing::{error, info};
use walkdir::WalkDir;

use crate::util::{calculate_file_hash, should_exclude};

use super::Storage;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct LocalStorage {
    backup_directory: PathBuf,
}


impl LocalStorage {
    pub fn new(backup_directory: &str) -> Self {
        LocalStorage {
            backup_directory: PathBuf::from(backup_directory),
        }
    }
}

impl Storage for LocalStorage {
    fn store_file(
        &self,
        source_path: &str,
        destination_path: &str,
        exclude: &[String],
    ) -> Result<(), String> {
        // 转换排除模式为 Regex
        let exclude_patterns: Vec<Regex> = exclude
        .iter()
        .map(|p| {
            // 去除末尾的斜杠，以避免在正则表达式中添加不必要的字符
            let clean_pattern = p.trim_end_matches('/').to_string();


            // 转义特殊字符并替换 '*' 为 '.*'
            let escaped_pattern = regex::escape(&clean_pattern).replace("\\*", ".*");
            Regex::new(&escaped_pattern).map_err(|err| format!("Invalid regex pattern {}: {}", p, err))
        })
        .collect::<Result<_, _>>()?;
        for entry in WalkDir::new(source_path) {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            // 如果路径符合排除规则，跳过
            if should_exclude(entry_path, &exclude_patterns) {
                println!("Excluded: {}", entry_path.display());
                continue;
            }
            if entry.path().is_file() {
                // 计算源文件的哈希值
                let source_hash = calculate_file_hash(entry.path().to_str().unwrap())
                    .map_err(|err| format!("Error calculating source file hash: {}", err))?;

                // 获取源文件相对于源目录的路径
                let relative_path = entry
                    .path()
                    .strip_prefix(source_path)
                    .map_err(|err| format!("Error getting relative path: {}", err))?;
                // 获取相对目录并去除文件名
                let relative_dir = relative_path
                    .parent()
                    .ok_or("Error getting relative directory")?;
                // 创建目标文件路径的绑定，确保其生命周期覆盖整个方法
                // 创建目标目录的完整路径
                let destination_dir = Path::new(destination_path).join(relative_dir);

                // 如果目标目录路径不存在，则创建它
                if !destination_dir.exists() {
                    fs::create_dir_all(&destination_dir).map_err(|err| {
                        format!(
                            "Error creating directory {}: {}",
                            destination_dir.display(),
                            err
                        )
                    })?;
                }
                let destination_file_path = destination_dir.join(entry.file_name());
                // 计算目标文件的哈希值
                let destination_hash =
                    calculate_file_hash(&destination_file_path.to_str().unwrap())
                        .unwrap_or_default();

                // 如果源文件和目标文件哈希值不同，则执行备份操作
                if source_hash != destination_hash {
                    match fs::copy(entry.path(), &destination_file_path) {
                        Ok(_) => info!(
                            "File {} backed up successfully",
                            destination_file_path.display()
                        ),
                        Err(err) => error!(
                            "Error storing file {}: {}",
                            destination_file_path.display(),
                            err
                        ),
                    }
                } else {
                    info!(
                        "File {} already exists and has the same hash. No need to backup.",
                        destination_file_path.to_str().unwrap()
                    );
                }
            }
        }
        Ok(())
    }
    fn backup_destination(&self) -> &str {
        self.backup_directory
            .to_str()
            .expect("Invalid backup directory path")
    }
}
