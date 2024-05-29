pub mod local;

// mod cloud;
pub trait Storage {
    fn store_file(&self, source_path: &str, destination_path: &str, exclude: &[String]) -> Result<(), String>;
    fn backup_destination(&self) -> &str;
}
