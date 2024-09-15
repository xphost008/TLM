pub static APP_DATA: std::sync::OnceLock<String> = std::sync::OnceLock::new();
pub static CURRENT_DIR: std::sync::OnceLock<String> = std::sync::OnceLock::new();
pub static TLM_INI: std::sync::OnceLock<crate::read_ini::IniFile> = std::sync::OnceLock::new();
pub static OTHER_INI: std::sync::OnceLock<crate::read_ini::IniFile> = std::sync::OnceLock::new();