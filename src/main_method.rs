thread_local! {
    pub static APP_DATA: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    pub static CURRENT_DIR: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    pub static TLM_INI: std::cell::RefCell<crate::read_ini::IniFile> = std::cell::RefCell::new(crate::read_ini::IniFile::new());
    pub static OTHER_INI: std::cell::RefCell<crate::read_ini::IniFile> = std::cell::RefCell::new(crate::read_ini::IniFile::new());
}