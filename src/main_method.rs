pub static mut APP_DATA: String = String::new();
pub static mut CURRENT_DIR: String = String::new();
pub static mut TLM_INI: crate::read_ini::IniFile = crate::read_ini::IniFile::new();
pub static mut OTHER_INI: crate::read_ini::IniFile = crate::read_ini::IniFile::new();