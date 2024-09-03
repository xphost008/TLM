pub static mut VERSION_JSON: serde_json::Value = serde_json::Value::Null;
pub static mut VERSION_SEL_JSON: serde_json::Value = serde_json::Value::Null;
pub static mut CHOOSE_VERSION: i32 = -1;
pub static mut CHOOSE_VERSION_SEL: i32 = -1;
pub static mut CURRENT_VERSION: String = String::new();
pub static mut CURRENT_VERSION_SEL: String = String::new();
pub static mut IS_ISOLATION: i32 = 4;
pub fn set_isolation() {
    println!("----------------------------------------------");
    println!("请输入你想要的隔离选项：");
    println!("1. 不开启版本隔离");
    println!("2. 隔离【正式版/快照版/远古Beta版/远古Alpha版】等版本");
    println!("3. 隔离【Forge/Fabric/Quilt/NeoForge】等版本");
    println!("4. 隔离全部版本");
    println!("----------------------------------------------");
    let mut input_num = String::new();
    std::io::stdin().read_line(&mut input_num).expect("Cannot read int!");
    let input_num = input_num.trim().parse::<i32>();
    if let Err(_) = input_num {
        println!("{}", ansi_term::Color::Red.paint("输入了不正确的数字！"));
        return;
    }
    let input_num = input_num.unwrap();
    if input_num < 1 || input_num > 4 {
        println!("{}", ansi_term::Color::Red.paint("输入了不正确的数字！"));
        return;
    }
    unsafe { 
        IS_ISOLATION = input_num; 
        crate::main_method::TLM_INI.write_str("Version", "SelectIsolation", IS_ISOLATION.to_string().as_str());
    }
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}
pub fn check_version() {
    unsafe {
        println!("当前全局设置为：");
        if CHOOSE_VERSION < 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未选择任一文件列表！"));
        }else{
            let p = VERSION_JSON
                    .as_object()
                    .expect("JSON Parse Error!")
                    .get("mc")
                    .expect("JSON Parse Error!")
                    .get(CHOOSE_VERSION as usize)
                    .expect("JSON Parse Error!");
            let ph = p
                    .get("path")
                    .expect("JSON Parse Error!")
                    .as_str()
                    .expect("JSON Parse Error!");
            let ne = p
                    .get("name")
                    .expect("JSON Parse Error!")
                    .as_str()
                    .expect("JSON Parse Error!");
            println!("\n你当前选中的文件列表是：\n名称：{}\n路径：{}", ne, ph);
            if CHOOSE_VERSION_SEL < 0 {
                println!("{}", ansi_term::Color::Yellow.paint("你还暂未选择任一游戏版本！"));
            }else{
                let p = VERSION_SEL_JSON
                        .as_object()
                        .expect("JSON Parse Error!")
                        .get("mcsel")
                        .expect("JSON Parse Error!")
                        .get(CHOOSE_VERSION_SEL as usize)
                        .expect("JSON Parse Error!");
                let ph = p
                        .get("path")
                        .expect("JSON Parse Error!")
                        .as_str()
                        .expect("JSON Parse Error!");
                println!("\n你当前选中的游戏版本是：\n{}", ph);
            }
        }
        println!("\n当前全局版本隔离为：{}", if IS_ISOLATION == 1 { 
            "不开启版本隔离"
        } else if IS_ISOLATION == 2 {
            "隔离【正式版/快照版/远古Beta版/远古Alpha版】等版本"
        } else if IS_ISOLATION == 3 {
            "隔离【Forge/Fabric/Quilt/NeoForge】等版本"
        } else {
            "隔离全部版本"
        });
        println!("当前你选中的版本独立设置为：");
    }
}
pub fn reload_version() {
    unsafe {
        if CHOOSE_VERSION < 0 { return; }
        let ver_root = VERSION_SEL_JSON.get_mut("mcsel").expect("Cannot get mcsel!").as_array_mut().expect("Cannot get mcsel");
        ver_root.clear();
        let mc_root = VERSION_JSON.as_object().unwrap();
        let path = mc_root["mc"][CHOOSE_VERSION as usize]["path"]
                .as_str()
                .expect("Cannot get mc path pairs!");
        let ver = format!("{}\\versions", path);
        let path = std::path::Path::new(ver.as_str());
        if !path.exists() || path.exists() && path.is_file() { return; }
        let walk = walkdir::WalkDir::new(path).min_depth(1).max_depth(1);
        for i in walk.into_iter().filter_map(|e| e.ok()) {
            // TODD: 判断版本是否有误！
            let ccf = i.file_name().to_str().unwrap().to_string();
            let mut p_obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
            let p_obj = p_obj.as_object_mut().unwrap();
            p_obj.insert(String::from("path"), serde_json::Value::String(ccf.clone()));
            ver_root.push(serde_json::Value::Object(p_obj.clone()));
        }
    }
}
pub fn rename_root() {
    unsafe {
        let ver_obj = VERSION_JSON["mc"].as_array_mut().expect("JSON Parse Error!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..ver_obj.len() {
            let j = ver_obj[i].as_object().expect("JSON Parse Error!");
            let n = j["name"].as_str().expect("JSON Parse Error!");
            let p = j["path"].as_str().expect("JSON Parse Error!");
            res.push(format!("{}. {} - {}", i + 1, n, p));
        }
        if res.len() == 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未添加任意文件夹哦！请添加一个再来！"));
            return;
        }
        println!("----------------------------------------------");
        println!("请输入你要重命名的文件列表：");
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
        println!("请输入要修改的名称：");
        let mut input_str = String::new();
        std::io::stdin().read_line(&mut input_str).expect("Cannot read stdin!");
        let input_str = input_str.trim();
        let ver_obj = ver_obj[input_num].as_object_mut().unwrap();
        ver_obj.remove("name");
        ver_obj.insert(String::from("name"), serde_json::Value::String(input_str.to_string()));
    }
    save_version();
    println!("{}", ansi_term::Color::Green.paint("修改成功！"));
}
pub fn remove_root() {
    unsafe {
        let ver_obj = VERSION_JSON["mc"].as_array_mut().expect("JSON Parse Error!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..ver_obj.len() {
            let j = ver_obj[i].clone();
            let n = j["name"].as_str().expect("JSON Parse Error!");
            let p = j["path"].as_str().expect("JSON Parse Error!");
            res.push(format!("{}. {} - {}", i + 1, n, p));
        }
        if res.len() == 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未添加任意文件夹哦！请添加一个再来！"));
            return;
        }
        println!("----------------------------------------------");
        println!("请输入你要移除的文件列表：");
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
        if CHOOSE_VERSION > input_num as i32 {
            CHOOSE_VERSION -= 1;
        } else if CHOOSE_VERSION == input_num as i32 {
            CHOOSE_VERSION = -1;
            CHOOSE_VERSION_SEL = -1;
            CURRENT_VERSION = String::new();
            CURRENT_VERSION_SEL = String::new();
            VERSION_SEL_JSON = serde_json::from_str::<serde_json::Value>("{\"mcsel\":[]}").unwrap();
        }
        crate::main_method::TLM_INI.write_str("MC", "SelectMC", CHOOSE_VERSION.to_string().as_str());
        ver_obj.remove(input_num);
    }
    reload_version();
    save_version();
    println!("{}", ansi_term::Color::Green.paint("移除成功！"));
}
pub fn rename_version() {
    unsafe {
        let ver_obj = VERSION_SEL_JSON["mcsel"].as_array().expect("JSON Parse Error!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..ver_obj.len() {
            let j = ver_obj[i].clone();
            let p = j["path"].as_str().expect("JSON Parse Error!");
            res.push(format!("{}. {}", i + 1, p));
        }
        if res.len() == 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未选择任意文件夹哦！请选择一个再来！"));
            return;
        }
        println!("----------------------------------------------");
        println!("请输入你要重命名的游戏版本：");
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
        println!("请输入要修改的名称：");
        let mut input_str = String::new();
        std::io::stdin().read_line(&mut input_str).expect("Cannot read stdin!");
        let input_str = input_str.trim();
        if input_str.is_empty() {
            println!("{}", ansi_term::Color::Red.paint("要修改的名称不能为空！请重新输入！"));
            return;
        }
        let p_vec = vec!["<", ">", ":", "\"", "/", "\\", "|", "?", "*"];
        for i in p_vec.into_iter() {
            if input_str.contains(i) {
                println!("你的文件名中包含非法字符！不允许重命名！请重试！");
                return;
            }
        }
        let path = format!("{}\\versions\\{}", CURRENT_VERSION.clone(), ver_obj[input_num]["path"].as_str().unwrap());
        let path = std::path::Path::new(path.as_str());
        let parent = path.parent().unwrap();
        let path2 = parent.join(input_str);
        let path2 = path2.as_path();
        std::fs::rename(path, path2).expect("Cannot rename file path!");
    }
    reload_version();
    println!("{}", ansi_term::Color::Green.paint("修改成功！"));
}
pub fn remove_version() {
    unsafe {
        let ver_obj = VERSION_SEL_JSON["mcsel"].as_array().expect("JSON Parse Error!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..ver_obj.len() {
            let j = ver_obj[i].clone();
            let p = j["path"].as_str().expect("JSON Parse Error!");
            res.push(format!("{}. {}", i + 1, p));
        }
        if res.len() == 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未选择任意文件夹哦！请选择一个再来！"));
            return;
        }
        println!("----------------------------------------------");
        println!("请输入你要移除的游戏版本：");
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
        let path = format!("{}\\versions\\{}", CURRENT_VERSION.clone(), ver_obj[input_num]["path"].as_str().unwrap());
        let path = path.as_str();
        std::fs::remove_dir_all(path).expect("Cannot remove version dir!");
        if (input_num as i32) < CHOOSE_VERSION_SEL {
            CHOOSE_VERSION_SEL -= 1;
        } else if (input_num as i32) == CHOOSE_VERSION_SEL {
            CHOOSE_VERSION_SEL = -1;
            CURRENT_VERSION_SEL = String::new();
        }
        crate::main_method::TLM_INI.write_str("MC", "SelectVer", CHOOSE_VERSION_SEL.to_string().as_str());
    }
    reload_version();
    save_version();
    println!("{}", ansi_term::Color::Green.paint("删除完成！"));
}
pub fn choose_version() {
    unsafe {
        if CHOOSE_VERSION < 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未添加任意文件夹哦！请添加一个再来！"));
            return;
        }
        let ver_obj = VERSION_SEL_JSON["mcsel"].as_array().expect("JSON Parse Error!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..ver_obj.len() {
            let j = ver_obj[i].clone();
            let p = j["path"].as_str().expect("JSON Parse Error!");
            res.push(format!("{}. {}", i + 1, p));
        }
        if res.len() == 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你的文件夹里没有一个游戏版本噢！请去下载一个再来吧！！"));
            return;
        }
        println!("----------------------------------------------");
        println!("请输入你要选择的游戏版本：");
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
        crate::main_method::TLM_INI.write_str("MC", "SelectVer", input_num.to_string().as_str());
        CHOOSE_VERSION_SEL = input_num as i32;
        CURRENT_VERSION_SEL = ver_obj[input_num]["path"].to_string();
    }
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}
pub fn choose_root() {
    unsafe {
        let ver_obj = VERSION_JSON["mc"].as_array().expect("JSON Parse Error!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..ver_obj.len() {
            let j = ver_obj[i].as_object().expect("JSON Parse Error!");
            let n = j["name"].as_str().expect("JSON Parse Error!");
            let p = j["path"].as_str().expect("JSON Parse Error!");
            res.push(format!("{}. {} - {}", i + 1, n, p));
        }
        if res.len() == 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未添加任意文件夹哦！请添加一个再来！"));
            return;
        }
        println!("----------------------------------------------");
        println!("请输入你要选择的文件列表：");
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
        CHOOSE_VERSION = input_num as i32;
        CHOOSE_VERSION_SEL = -1;
        let act_ver = ver_obj[input_num]["path"].as_str().unwrap().to_string();
        CURRENT_VERSION = act_ver.clone();
        crate::main_method::TLM_INI.write_str("MC", "SelectMC", input_num.to_string().as_str());
        crate::main_method::TLM_INI.write_str("MC", "SelectVer", "-1");
        reload_version();
    }
    save_version();
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}
pub fn add_directory() {
    println!("请输入mc根路径，或者将文件夹拖入以自动填写路径：");
    let mut input_dir = String::new();
    std::io::stdin().read_line(&mut input_dir).expect("Cannot read stdin!");
    input_dir = input_dir.trim().to_string();
    println!("请输入根路径标识名称，可以输入中文！");
    let mut input_name = String::new();
    std::io::stdin().read_line(&mut input_name).expect("Cannot read stdin!");
    input_name = input_name.trim().to_string();
    let path = std::path::Path::new(input_dir.as_str());
    if !path.exists() || path.exists() && path.is_file() {
        println!("{}", ansi_term::Color::Red.paint("你输入的文件夹不存在，请重新输入！"));
        return;
    }
    unsafe {
        //添加根文件夹
        let ver_obj = VERSION_JSON.get_mut("mc").expect("JSON Parse Error!");
        let ver_obj = ver_obj.as_array_mut().expect("JSON Parse Error!");
        let mut push_obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        let push_obj = push_obj.as_object_mut().unwrap();
        push_obj.insert(String::from("name"), serde_json::Value::String(input_name.clone()));
        push_obj.insert(String::from("path"), serde_json::Value::String(input_dir.clone().replace("/", "\\")));
        ver_obj.push(serde_json::Value::Object(push_obj.clone()));
    }
    save_version();
    println!("{}", ansi_term::Color::Green.paint("添加成功！"));
}
pub fn save_version() {
    unsafe {
        crate::rust_lib::main_mod::set_file(
            format!("{}\\TankLauncherModule\\configs\\MCJSON.json", crate::main_method::CURRENT_DIR).as_str(), 
            serde_json::to_string_pretty(&VERSION_JSON.clone()).unwrap());
        crate::rust_lib::main_mod::set_file(
            format!("{}\\TankLauncherModule\\configs\\MCSelJSON.json", crate::main_method::CURRENT_DIR).as_str(), 
            serde_json::to_string_pretty(&VERSION_SEL_JSON.clone()).unwrap());
    }
}