use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BackupItem {
    pub backup_directory: String,
    pub backup_destination: String,
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BackupConfig {
    pub check_frequency: u64, // 修改为秒级的整数类型
    pub backup_items: Vec<BackupItem>,
}

impl BackupConfig {
    pub fn config_file() -> PathBuf {
        let proj_dirs = ProjectDirs::from("com", "initcool", "rsbackup")
            .expect("Failed to get project directories");
        let mut config_file = PathBuf::from(proj_dirs.config_dir());
        std::fs::create_dir_all(&config_file).expect("");
        config_file.push("config.yaml");
        return config_file;
    }
    pub fn new() -> Result<BackupConfig, Box<dyn Error>> {
        let mut config = BackupConfig {
            backup_items: vec![BackupItem {
                backup_directory: "".to_string(),
                backup_destination: "".to_string(),
                exclude: Some(vec![]),
            }],
            check_frequency: 86400,
        };
        match std::fs::File::open(BackupConfig::config_file()) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();

                config = serde_yaml::from_str(&content)?;
            }
            Err(_e) => {
                let file = fs::File::create(BackupConfig::config_file())?;
                serde_yaml::to_writer(file, &config)?;
                info!("Write config to file successfully!");
                let config_dir = BackupConfig::config_file();
                info!("Please go to the directory '{}' to modify the backup configuration file settings.", config_dir.display());
                eprintln!("Please go to the directory '{}' to modify the backup configuration file settings.", config_dir.display());
            }
        }
        Ok(config)
    }
    #[allow(dead_code)]
    fn create_file(
        &self,
        project_path: &Path,
        file_name: &str,
        content: &[u8],
    ) -> io::Result<()> {
        let file_path = project_path.join(file_name);
        let mut file = File::create(&file_path)?;
        file.write_all(content)?;
        Ok(())
    }
}

#[allow(dead_code)]
pub fn load_config(filename: &str) -> Result<BackupConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: BackupConfig = serde_yaml::from_str(&contents)?;
    Ok(config)
}
