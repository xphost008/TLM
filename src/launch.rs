thread_local! {
    pub static WINDOW_HEIGHT: std::cell::RefCell<usize> = std::cell::RefCell::new(480);
    pub static WINDOW_WIDTH: std::cell::RefCell<usize> = std::cell::RefCell::new(854);
    pub static MAX_MEMORY: std::cell::RefCell<usize> = std::cell::RefCell::new(4096);
    pub static MIN_MEMORY: std::cell::RefCell<usize> = std::cell::RefCell::new(256);
    pub static ADDITIONAL_JVM: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    pub static ADDITIONAL_GAME: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    pub static CUSTOM_INFO: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    pub static JAVA_JSON: std::cell::RefCell<serde_json::Value> = std::cell::RefCell::new(serde_json::Value::Null);
    pub static CHOOSE_JAVA: std::cell::RefCell<i32> = std::cell::RefCell::new(-1);
    pub static CURRENT_JAVA: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    pub static CURRENT_BITS: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    pub static CURRENT_VERSION: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
}

fn add_java_simple(path: String) -> bool {
    let a = JAVA_JSON.with_borrow_mut(|java_obj| {
        let java_obj = java_obj["java"].as_array_mut().expect("Cannot Parse Java JSON");
        for i in java_obj.iter() {
            let j = i["path"].as_str().expect("Cannot Parse Java JSON");
            if j.eq(path.as_str()) {
                return true;
            }
        }
        let bits = crate::rust_lib::main_mod::get_file_bit(path.clone());
        if let None = bits {
            println!("{}", ansi_term::Color::Red.paint("读取不了该Java的位数，请尝试换一个Java！"));
            return false;
        }
        let bits = if bits.unwrap() { "64" } else { "32" };
        let version = crate::rust_lib::main_mod::get_file_version(path.clone());
        if let None = version {
            println!("{}", ansi_term::Color::Red.paint("读取不了该Java的版本，请尝试换一个Java！"));
            return false;
        }
        let version = version.unwrap();
        let mut push_obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        let push_obj = push_obj.as_object_mut().unwrap();
        push_obj.insert(String::from("path"), serde_json::Value::String(path.to_string().replace("/", "\\")));
        push_obj.insert(String::from("bits"), serde_json::Value::String(bits.to_string()));
        push_obj.insert(String::from("version"), serde_json::Value::String(version.clone()));
        java_obj.push(serde_json::Value::Object(push_obj.clone()));
        true
    });
    a
}

fn recursion_search_java(path: String) -> bool {
    let path = std::path::Path::new(path.as_str());
    if !path.exists() || path.exists() && path.is_file() {
        return false;
    }
    let walk = walkdir::WalkDir::new(path).min_depth(1).max_depth(1);
    for i in walk.into_iter().filter_map(|e| e.ok()) {
        let path = i.path();
        let ne = path.to_str();
        if let None = ne { continue; }
        let ne = ne.unwrap();
        if path.is_dir() {
            let nel = ne.to_lowercase();
            if nel.contains("c:\\windows") || nel.contains("recycle.bin") || nel.contains("xboxgames") || nel.contains("steamlibrary") {
                continue;
            }
            recursion_search_java(ne.to_string());
        }
        if path.is_file() {
            let fname = path.file_name().unwrap();
            if fname.eq("java.exe") || fname.eq("javaw.exe") {
                add_java_simple(path.to_str().unwrap().to_string());
            }
        }
    }
    true
}

fn full_scan_java() {
    for i in 65..=90 {
        let d = format!("{}:\\", (i as u8 as char).to_string());
        if let Ok(_) = std::fs::read_dir(&d) {
            recursion_search_java(d.clone());
        }
    }
}

fn java_search_regedit() -> bool {
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let soft = hklm.open_subkey("SOFTWARE\\JavaSoft");
    if let Err(_) = soft {
        return false;
    }
    let soft = soft.unwrap();
    let name = soft.enum_keys();
    for i in name.into_iter() {
        if let Err(_) = i {
            return false;
        }
        let i = i.unwrap();
        let soft = soft.open_subkey(i.clone());
        if let Err(_) = soft {
            return false;
        }
        let soft = soft.unwrap();
        let name = soft.enum_keys();
        for j in name.into_iter() {
            if let Err(_) = j {
                return false;
            }
            let j = j.unwrap();
            let soft = soft.open_subkey(j.clone());
            if let Err(_) = soft {
                return false;
            }
            let soft = soft.unwrap();
            let java_home: Result<String, std::io::Error> = soft.get_value("JavaHome");
            if let Err(_) = java_home {
                return false;
            }
            let java_home = java_home.unwrap();
            if !recursion_search_java(java_home.clone()) {
                return false;
            }
        }
    }
    true
}

fn simple_scan_java() {
    let binding = crate::main_method::APP_DATA.with_borrow(|e| e.clone());
    let appdata_path = std::path::Path::new(binding.as_str());
    let appdata_path = appdata_path.parent().expect("Cannot get appdata parent path!");
    let appdata_path = appdata_path.join("Local");
    if !appdata_path.exists() || appdata_path.exists() && appdata_path.is_file() {
        panic!("Cannot get appdata Local Path!");
    }
    if !java_search_regedit() {
        println!("{}", ansi_term::Color::Red.paint("搜索注册表时发生了错误，你可能没有以管理员启动该程序！"));
    }
    let ev = std::env::var("PATH");
    if let Ok(e) = ev {
        let v = e.split(";").collect::<Vec<&str>>();
        for i in v.into_iter() {
            if i.to_lowercase().contains("c:\\windows") || i.to_lowercase().contains("recycle.bin") { continue; }
            let p = std::path::Path::new(i);
            if p.exists() && p.is_dir(){
                if !recursion_search_java(i.to_string()) {
                    println!("{}", ansi_term::Color::Red.paint("搜索环境变量PATH时发生了错误，你可能没有以管理员启动该程序！"));
                }
            }
        }
    }else{
        println!("{}", ansi_term::Color::Red.paint("搜索环境变量时发生了错误，你可能没有以管理员启动该程序！"));
    }
    let c64 = std::path::Path::new("C:\\Program File\\Java");
    if c64.exists() && c64.is_dir() {
        if !recursion_search_java(c64.to_str().unwrap().to_string()){
            println!("{}", ansi_term::Color::Red.paint("搜索C:\\Program File\\Java时发生了错误！"));
        }
    }
    let c32 = std::path::Path::new("C:\\Program Files (x86)\\Java");
    if c32.exists() && c32.is_dir() {
        if !recursion_search_java(c32.to_str().unwrap().to_string()){
            println!("{}", ansi_term::Color::Red.paint("搜索C:\\Program Files (x86)\\Java时发生了错误！"));
        }
    }
    let g = appdata_path.join("Packages").join("Microsoft.4297127D64EC6_8wekyb3d8bbwe").join("LocalCache").join("Local").join("runtime");
    if g.exists() && g.is_dir() {
        if !recursion_search_java(g.to_str().unwrap().to_string()){
            println!("{}", ansi_term::Color::Red.paint("搜索官方启动器目录时发生了错误！"));
        }
    }
    let at = format!("{}\\TankLauncherModule\\Java", binding.clone());
    let t = std::path::Path::new(at.as_str());
    if t.exists() && t.is_dir() {
        if !recursion_search_java(t.to_str().unwrap().to_string()){
            println!("{}", ansi_term::Color::Red.paint("搜索官方启动器目录时发生了错误！"));
        }
    }
}

pub fn do_scan_java() {
    println!("----------------------------------------------");
    println!("请输入你要查询的方式：");
    println!("1. 特定查询【耗时较快】（将通过顺序【注册表】->【环境变量】->【C:/Program File/Java】->【C:/Program File (x86)/Java】->【官方启动器Java下载】->【TLM特定Java下载】路径进行查询。）");
    println!("2. 全盘扫描【耗时较久】（将直接进行扫盘行动！直接扫描你的所有盘符中的所有文件夹，但是不会碰【Recycle.bin】、【C:/Windows】路径下的文件。）");
    println!("----------------------------------------------");
    let mut input_num = String::new();
    std::io::stdin().read_line(&mut input_num).expect("Cannot read stdin!");
    let input_num = input_num.trim();
    let start = std::time::Instant::now();
    if input_num.eq("1") {
        simple_scan_java();
    } else if input_num.eq("2"){
        full_scan_java();
    } else {
        println!("{}", ansi_term::Color::Red.paint("请不要输入错误的值！"));
        return;
    }
    save_launch();//{}
    println!("{}{}{}", ansi_term::Color::Green.paint("搜索完成！耗时："), ansi_term::Color::Green.paint(start.elapsed().as_secs_f64().to_string()), ansi_term::Color::Green.paint("秒"));
}

pub fn check_launch(){
    println!("当前全局设置为：");
    if CHOOSE_JAVA.with_borrow(|e| e.clone()) < 0 {
        println!("{}", ansi_term::Color::Yellow.paint("你还暂未选择任一Java！"));
    } else {
        let path = CURRENT_JAVA.with_borrow(|e| e.clone());
        let bits = CURRENT_BITS.with_borrow(|e| e.clone());
        let version = CURRENT_VERSION.with_borrow(|e| e.clone());
        println!("\n你目前选择的Java是：\n(Java {} x{}) {}\n", version, bits, path);
    }
    println!("当前设置的窗口宽度为：{}", WINDOW_WIDTH.with_borrow(|e| e.clone()));
    println!("当前设置的窗口高度为：{}", WINDOW_HEIGHT.with_borrow(|e| e.clone()));
    println!("当前设置的最小内存为：{}", MIN_MEMORY.with_borrow(|e| e.clone()));
    println!("当前设置的最大内存为：{}", MAX_MEMORY.with_borrow(|e| e.clone()));
}

pub fn choose_java() {
    JAVA_JSON.with_borrow_mut(|java_obj| {
        let java_obj = java_obj["java"].as_array().expect("Cannot read java json!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..java_obj.len() {
            let j = java_obj[i].as_object().expect("Cannot read java json!");
            let p = j["path"].as_str().expect("Cannot read java json!");
            let b = j["bits"].as_str().expect("Cannot read java json!");
            let v = j["version"].as_str().expect("Cannot read java json!");
            res.push(format!("{}. (Java {} x{}) {}", (i + 1).to_string(), v, b, p));
        }
        if res.len() == 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未添加任意Java哦！请添加一个再来！"));
            return;
        }
        println!("----------------------------------------------");
        println!("请输入你要选择的Java：");
        for i in res.iter() {
            println!("{}", i);
        }
        println!("----------------------------------------------");
        let mut input_num = String::new();
        std::io::stdin().read_line(&mut input_num).expect("Cannot read num!");
        let input_num = input_num.trim().parse::<usize>();
        if let Err(_) = input_num {
            println!("{}", ansi_term::Color::Red.paint("输入了错误的数字，请重新输入！"));
            return;
        }
        let input_num = input_num.unwrap();
        if input_num > res.len() || input_num < 1 {
            println!("{}", ansi_term::Color::Red.paint("输入了错误的数字，请重新输入！"));
            return;
        }
        let input_num = input_num - 1;
        CHOOSE_JAVA.set(input_num as i32);
        crate::main_method::TLM_INI.with_borrow(move |e| e.write_str("Java", "SelectJava", input_num.to_string().as_str()));
        let current = java_obj[input_num].as_object().unwrap();
        CURRENT_JAVA.set(current["path"].as_str().unwrap().to_string());
        CURRENT_BITS.set(current["bits"].as_str().unwrap().to_string());
        CURRENT_VERSION.set(current["version"].as_str().unwrap().to_string());
    });
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

pub fn remove_java() {
    JAVA_JSON.with_borrow_mut(|java_obj| {
        let java_obj = java_obj["java"].as_array_mut().expect("Cannot read java json!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..java_obj.len() {
            let j = java_obj[i].as_object().expect("Cannot read java json!");
            let p = j["path"].as_str().expect("Cannot read java json!");
            let b = j["bits"].as_str().expect("Cannot read java json!");
            let v = j["version"].as_str().expect("Cannot read java json!");
            res.push(format!("{}. (Java {} x{}) {}", (i + 1).to_string(), v, b, p));
        }
        if res.len() == 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未添加任意Java哦！请添加一个再来！"));
            return;
        }
        println!("----------------------------------------------");
        println!("请输入你要选择的Java：");
        for i in res.iter() {
            println!("{}", i);
        }
        println!("----------------------------------------------");
        let mut input_num = String::new();
        std::io::stdin().read_line(&mut input_num).expect("Cannot read num!");
        let input_num = input_num.trim().parse::<usize>();
        if let Err(_) = input_num {
            println!("{}", ansi_term::Color::Red.paint("输入了错误的数字，请重新输入！"));
            return;
        }
        let input_num = input_num.unwrap();
        if input_num > res.len() || input_num < 1 {
            println!("{}", ansi_term::Color::Red.paint("输入了错误的数字，请重新输入！"));
            return;
        }
        let input_num = input_num - 1;
        let cj = CHOOSE_JAVA.with_borrow(|e| e.clone());
        if cj > input_num as i32 {
            CHOOSE_JAVA.set(cj - 1);
        } else if cj == input_num as i32 {
            CHOOSE_JAVA.set(-1);
            CURRENT_JAVA.set(String::new());
            CURRENT_BITS.set(String::new());
            CURRENT_VERSION.set(String::new());
        }
        crate::main_method::TLM_INI.with_borrow(move |e| e.write_str("Java", "SelectJava", CHOOSE_JAVA.with_borrow(|e| e.clone()).to_string().as_str()));
        java_obj.remove(input_num);
    });
    save_launch();
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

pub fn add_java() {
    println!("请输入Java路径，请精确到java.exe或javaw.exe，或者将文件拖入以自动填写路径：");
    let mut input_dir = String::new();
    std::io::stdin().read_line(&mut input_dir).expect("Cannot read stdin!");
    input_dir = input_dir.trim().to_string();
    let path = std::path::Path::new(input_dir.as_str());
    if !path.exists() || path.exists() && path.is_dir() {
        println!("{}", ansi_term::Color::Red.paint("路径输入错误，请不要输入不存在的路径！"));
        return;
    }
    if path.file_name().unwrap().ne("java.exe") && path.file_name().unwrap().ne("javaw.exe") {
        println!("{}", ansi_term::Color::Red.paint("只允许输入java.exe或javaw.exe噢！请不要输入别的！"));
        return;
    }
    if !add_java_simple(input_dir.clone()) {
        return;
    }
    save_launch();
    println!("{}", ansi_term::Color::Green.paint("添加成功！"));
}

pub fn set_additional_jvm() {
    let mut input_str = String::new();
    println!("请输入你要设置的额外JVM参数");
    std::io::stdin().read_line(&mut input_str).expect("Cannot read number!");
    let input_str = input_str.trim().to_string();
    ADDITIONAL_JVM.set(input_str.clone());
    crate::main_method::TLM_INI.with_borrow(move |e| e.write_str("Version", "AdditionalJVM", input_str.as_str()));
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

pub fn set_additional_game() {
    let mut input_str = String::new();
    println!("请输入你要设置的额外Game参数");
    std::io::stdin().read_line(&mut input_str).expect("Cannot read number!");
    let input_str = input_str.trim().to_string();
    ADDITIONAL_GAME.set(input_str.clone());
    crate::main_method::TLM_INI.with_borrow(move |e| e.write_str("Version", "AdditionalGame", input_str.as_str()));
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

pub fn set_custom_info() {
    let mut input_str = String::new();
    println!("请输入你要设置的自定义信息（将会显示在进入游戏的左下角以及进入地图后按F3的左上角。）");
    std::io::stdin().read_line(&mut input_str).expect("Cannot read number!");
    let input_str = input_str.trim().to_string();
    if input_str.is_empty() {
        println!("{}", ansi_term::Color::Red.paint("请不要输入错误的值！"));
        return;
    }
    CUSTOM_INFO.set(input_str.clone());
    crate::main_method::TLM_INI.with_borrow(|e| e.write_str("Version", "CustomInfo", input_str.as_str()));
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

pub fn set_window_height() {
    let mut input_num = String::new();
    println!("请输入你要设置的窗口高度");
    std::io::stdin().read_line(&mut input_num).expect("Cannot read number!");
    let input_num = input_num.trim().parse::<usize>();
    if let Err(_) = input_num {
        println!("{}", ansi_term::Color::Red.paint("请不要输入错误的值！"));
        return;
    }
    let input_num = input_num.unwrap();
    WINDOW_HEIGHT.set(input_num);
    crate::main_method::TLM_INI.with_borrow(|e| e.write_str("Document", "WindowHeight", input_num.to_string().as_str()));
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}


pub fn set_window_width() {
    let mut input_num = String::new();
    println!("请输入你要设置的窗口宽度");
    std::io::stdin().read_line(&mut input_num).expect("Cannot read number!");
    let input_num = input_num.trim().parse::<usize>();
    if let Err(_) = input_num {
        println!("{}", ansi_term::Color::Red.paint("请不要输入错误的值！"));
        return;
    }
    let input_num = input_num.unwrap();
    WINDOW_WIDTH.set(input_num);
    crate::main_method::TLM_INI.with_borrow(|e| e.write_str("Document", "WindowHeight", input_num.to_string().as_str()));
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

pub fn set_max_memory() {
    let mut input_num = String::new();
    println!("请输入你要设置的最大内存");
    std::io::stdin().read_line(&mut input_num).expect("Cannot read number!");
    let input_num = input_num.trim().parse::<usize>();
    if let Err(_) = input_num {
        println!("{}", ansi_term::Color::Red.paint("请不要输入错误的值！"));
        return;
    }
    let input_num = input_num.unwrap();
    MAX_MEMORY.set(input_num);
    crate::main_method::TLM_INI.with_borrow(|e| e.write_str("Document", "MaxMemory", input_num.to_string().as_str()));
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

pub fn set_min_memory() {
    let mut input_num = String::new();
    println!("请输入你要设置的最大内存");
    std::io::stdin().read_line(&mut input_num).expect("Cannot read number!");
    let input_num = input_num.trim().parse::<usize>();
    if let Err(_) = input_num {
        println!("{}", ansi_term::Color::Red.paint("请不要输入错误的值！"));
        return;
    }
    let input_num = input_num.unwrap();
    MIN_MEMORY.set(input_num);
    crate::main_method::TLM_INI.with_borrow(|e| e.write_str("Document", "MinMemory", input_num.to_string().as_str()));
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

fn save_launch() {
    let java_json = JAVA_JSON.with_borrow(|e| e.clone());
    crate::rust_lib::main_mod::set_file(
        format!("{}\\TankLauncherModule\\configs\\JavaJSON.json", crate::main_method::CURRENT_DIR.with_borrow(|e| e.clone())).as_str(), 
        serde_json::to_string_pretty(&java_json.clone()).unwrap());
}