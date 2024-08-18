pub static mut WINDOW_HEIGHT: usize = 480;
pub static mut WINDOW_WIDTH: usize = 854;
pub static mut MAX_MEMORY: usize = 4096;
pub static mut MIN_MEMORY: usize = 1024;
pub static mut ADDITIONAL_JVM: String = String::new();
pub static mut ADDITIONAL_GAME: String = String::new();
pub static mut CUSTOM_INFO: String = String::new();
pub static mut JAVA_JSON: serde_json::Value = serde_json::Value::Null;
pub static mut CHOOSE_JAVA: i32 = -1;
pub static mut CURRENT_JAVA: String = String::new();
pub static mut CURRENT_BITS: String = String::new();
pub static mut CURRENT_VERSION: String = String::new();

fn add_java_simple(path: String) -> bool {
    unsafe {
        let java_obj = JAVA_JSON.get_mut("java").expect("Cannot Parse Java JSON");
        let java_obj = java_obj.as_array_mut().expect("Cannot Parse Java JSON");
        for i in java_obj.iter() {
            let j = i.get("path").expect("Cannot Parse Java JSON").as_str().expect("Cannot Parse Java JSON");
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
    }
    return true;
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
    return true;
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
    return true;
}

fn simple_scan_java() {
    unsafe {
        let appdata_path = std::path::Path::new(crate::main_method::APP_DATA.as_str());
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
        let at = format!("{}\\TankLauncherModule\\Java", crate::main_method::APP_DATA.clone());
        let t = std::path::Path::new(at.as_str());
        if t.exists() && t.is_dir() {
            if !recursion_search_java(t.to_str().unwrap().to_string()){
                println!("{}", ansi_term::Color::Red.paint("搜索官方启动器目录时发生了错误！"));
            }
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
    unsafe {
        println!("当前全局设置为：");
        if CHOOSE_JAVA < 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未选择任一Java！"));
        } else {
            let path = CURRENT_JAVA.clone();
            let bits = CURRENT_BITS.clone();
            let version = CURRENT_VERSION.clone();
            println!("\n你目前选择的Java是：\n(Java {} x{}) {}\n", version, bits, path);
        }
        println!("当前设置的窗口宽度为：{}", WINDOW_WIDTH);
        println!("当前设置的窗口高度为：{}", WINDOW_HEIGHT);
        println!("当前设置的最小内存为：{}", MIN_MEMORY);
        println!("当前设置的最大内存为：{}", MAX_MEMORY);
    }
}

pub fn choose_java() {
    unsafe {
        let java_obj = JAVA_JSON.get("java").expect("Cannot read java json!");
        let java_obj = java_obj.as_array().expect("Cannot read java json!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..java_obj.len() {
            let j = java_obj[i].as_object().expect("Cannot read java json!");
            let p = j.get("path").expect("Cannot read java json!").as_str().expect("Cannot read java json!");
            let b = j.get("bits").expect("Cannot read java json!").as_str().expect("Cannot read java json!");
            let v = j.get("version").expect("Cannot read java json!").as_str().expect("Cannot read java json!");
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
        CHOOSE_JAVA = input_num as i32;
        crate::main_method::TLM_INI.write_str("Java", "SelectJava", CHOOSE_JAVA.to_string().as_str());
        let current = java_obj[input_num].as_object().unwrap();
        CURRENT_JAVA = current["path"].as_str().unwrap().to_string();
        CURRENT_BITS = current["bits"].as_str().unwrap().to_string();
        CURRENT_VERSION = current["version"].as_str().unwrap().to_string();
    }
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

pub fn remove_java() {
    unsafe {
        let java_obj = JAVA_JSON.get_mut("java").expect("Cannot read java json!");
        let java_obj = java_obj.as_array_mut().expect("Cannot read java json!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..java_obj.len() {
            let j = java_obj[i].as_object().expect("Cannot read java json!");
            let p = j.get("path").expect("Cannot read java json!").as_str().expect("Cannot read java json!");
            let b = j.get("bits").expect("Cannot read java json!").as_str().expect("Cannot read java json!");
            let v = j.get("version").expect("Cannot read java json!").as_str().expect("Cannot read java json!");
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
        if CHOOSE_JAVA > input_num as i32 {
            CHOOSE_JAVA -= 1;
        } else if CHOOSE_JAVA == input_num as i32 {
            CHOOSE_JAVA = -1;
            CURRENT_JAVA = String::new();
            CURRENT_BITS = String::new();
            CURRENT_VERSION = String::new();
        }
        crate::main_method::TLM_INI.write_str("Java", "SelectJava", CHOOSE_JAVA.to_string().as_str());
        java_obj.remove(input_num);
    }
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
    unsafe {
        ADDITIONAL_JVM = input_str.clone();
        crate::main_method::TLM_INI.write_str("Version", "AdditionalJVM", ADDITIONAL_JVM.as_str());
    }
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

pub fn set_additional_game() {
    let mut input_str = String::new();
    println!("请输入你要设置的额外Game参数");
    std::io::stdin().read_line(&mut input_str).expect("Cannot read number!");
    let input_str = input_str.trim().to_string();
    unsafe {
        ADDITIONAL_GAME = input_str.clone();
        crate::main_method::TLM_INI.write_str("Version", "AdditionalGame", ADDITIONAL_GAME.as_str());
    }
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

pub fn set_custom_info() {
    let mut input_str = String::new();
    println!("请输入你要设置的自定义信息（将会显示在进入游戏的左下角以及进入地图后按F3的左上角。）");
    std::io::stdin().read_line(&mut input_str).expect("Cannot read number!");
    let input_str = input_str.trim().to_string();
    unsafe {
        CUSTOM_INFO = input_str.clone();
        crate::main_method::TLM_INI.write_str("Version", "CustomInfo", CUSTOM_INFO.as_str());
    }
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
    unsafe {
        WINDOW_HEIGHT = input_num;
        crate::main_method::TLM_INI.write_str("Document", "WindowHeight", WINDOW_HEIGHT.to_string().as_str());
    }
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
    unsafe {
        WINDOW_WIDTH = input_num;
        crate::main_method::TLM_INI.write_str("Document", "WindowHeight", WINDOW_WIDTH.to_string().as_str());
    }
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
    unsafe {
        MAX_MEMORY = input_num;
        crate::main_method::TLM_INI.write_str("Document", "MaxMemory", MAX_MEMORY.to_string().as_str());
    }
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
    unsafe {
        MIN_MEMORY = input_num;
        crate::main_method::TLM_INI.write_str("Document", "MinMemory", MIN_MEMORY.to_string().as_str());
    }
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

fn save_launch() {
    unsafe {
        crate::rust_lib::main_mod::set_file(
            format!("{}\\TankLauncherModule\\configs\\JavaJSON.json", crate::main_method::CURRENT_DIR).as_str(), 
            serde_json::to_string_pretty(&JAVA_JSON.clone()).unwrap());
    }
}