#[derive(Clone)]
pub struct IniFile {
    path: String,
}

impl IniFile {
    pub const fn new() -> Self{
        Self {
            path: String::new(),
        }
    }
    pub fn set_path(&mut self, path: &str) {
        let p = std::path::Path::new(path);
        if !p.exists() || p.exists() && p.is_dir() {
            let p = crate::rust_lib::main_mod::set_file(path, String::new());
            if !p { panic!("无法创建INI文件！") }
        }
        self.path = path.to_string();
    }
    pub fn read_int(&self, section: &str, key: &str, default: i32) -> i32 {
        let re = ini::Ini::load_from_file(self.path.as_str());
        if let Err(_) = re {
            return default;
        }
        let re = re.unwrap().to_owned();
        let section = re.section(Some(section));
        if let None = section {
            return default;
        }
        let key = section.unwrap().get(key);
        if let None = key{
            return default;
        }
        let res = key.unwrap();
        let par = res.parse::<i32>();
        return if let Ok(t) = par {
            t.to_owned()
        } else {
            default
        }
    }
    pub fn read_str(&self, section: &str, key: &str, default: &str) -> String {
        let re = ini::Ini::load_from_file(self.path.as_str());
        if let Err(_) = re {
            return default.to_string();
        }
        let re = re.unwrap();
        let section = re.section(Some(section));
        if let None = section {
            return default.to_string();
        }
        let key = section.unwrap().get(key);
        return if let Some(t) = key {
            t.to_string()
        } else {
            default.to_string()
        }
    }
    pub fn write_str(&self, section: &str, key: &str, value: &str) -> bool {
        let re = ini::Ini::load_from_file(self.path.as_str());
        if let Err(_) = re {
            return false;
        }
        let re = re.unwrap();
        let mut nre = re.clone();
        nre.with_section(Some(section)).set(key, value);
        let suc = nre.write_to_file(self.path.as_str());
        if let Err(_) = suc {
            return false;
        }
        true
    }
}