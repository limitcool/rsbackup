use directories::ProjectDirs;

use regex::Regex;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use tracing::info;

pub fn calculate_file_hash(file_path: &str) -> Result<String, String> {
    let mut file = fs::File::open(file_path).map_err(|e| format!("Error opening file: {}", e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Error reading file: {}", e))?;
    let hash = Sha256::digest(&buffer);
    Ok(format!("{:x}", hash))
}

// 检查路径是否应排除
pub fn should_exclude(path: &Path, exclude_patterns: &[Regex]) -> bool {
    let path_str = path.to_str().unwrap();
    let is_dir = path.is_dir();
    let path_with_slash = if is_dir {
        format!("{}/", path_str)
    } else {
        path_str.to_string()
    };

    info!("Checking path: {}", path_str);
    info!("Checking path with slash: {}", path_with_slash);

    exclude_patterns
        .iter()
        .any(|pattern| pattern.is_match(path_str) || is_dir && pattern.is_match(&path_with_slash))
}

// 获取项目目录的函数
pub fn get_project_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "initcool", "rsbackup")
        .expect("Failed to get project directories")
}
#[allow(dead_code)]
pub fn config_file() -> PathBuf {
    let proj_dirs = get_project_dirs();
    let config_path = PathBuf::from(proj_dirs.config_dir());
    fs::create_dir_all(&config_path).expect("Failed to create config directory");
    config_path.join("config.yaml")
}

pub fn log_file() -> PathBuf {
    let proj_dirs = get_project_dirs();
    let mut log_path = PathBuf::from(proj_dirs.data_dir()); // 开始于 config_dir
    fs::create_dir_all(&log_path).expect("Failed to create project directory");
    log_path.push("log"); // 添加 log 子目录
    fs::create_dir_all(&log_path).expect("Failed to create log directory");
    log_path.push("rsbackup.log"); // 添加日志文件名
    log_path
}