pub static mut WINDOW_HEIGHT: usize = 480;
pub static mut WINDOW_WIDTH: usize = 854;
pub static mut MAX_MEMORY: usize = 4096;
pub static mut MIN_MEMORY: usize = 1024;
pub static mut JAVA_JSON: serde_json::Value = serde_json::Value::Null;
pub static mut CHOOSE_JAVA: i32 = -1;
pub static mut CURRENT_JAVA: String = String::new();
pub static mut CURRENT_BITS: String = String::new();
pub static mut CURRENT_VERSION: String = String::new();

fn add_java_simple(path: String) {

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
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未选择任意Java哦！请选择一个再来！"));
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
    unsafe {
        let java_obj = JAVA_JSON.get_mut("java").expect("Cannot Parse Java JSON");
        let java_obj = java_obj.as_array_mut().expect("Cannot Parse Java JSON");
        let mut push_obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        let push_obj = push_obj.as_object_mut().unwrap();
        push_obj.insert(String::from("path"), serde_json::from_str(format!("\"{}\"", input_dir.clone().replace("\\", "\\\\").replace("/", "\\\\")).as_str()).unwrap());
        let bits = crate::rust_lib::main_mod::get_file_bit(input_dir.clone());
        if let None = bits {
            println!("{}", ansi_term::Color::Red.paint("读取不了该Java的位数，请尝试换一个Java！"));
            return;
        }
        let bits = if bits.unwrap() { "64" } else { "32" };
        push_obj.insert(String::from("bits"), serde_json::from_str(format!("\"{}\"", bits).as_str()).unwrap());
        let version = crate::rust_lib::main_mod::get_file_version(input_dir.clone());
        if let None = version {
            println!("{}", ansi_term::Color::Red.paint("读取不了该Java的版本，请尝试换一个Java！"));
            return;
        }
        let version = version.unwrap();
        push_obj.insert(String::from("version"), serde_json::from_str(format!("\"{}\"", version.clone()).as_str()).unwrap());
        java_obj.push(serde_json::Value::Object(push_obj.clone()));
    }
    save_launch();
    println!("{}", ansi_term::Color::Green.paint("添加成功！"));
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