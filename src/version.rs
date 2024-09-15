thread_local! {
    pub static VERSION_JSON: std::cell::RefCell<serde_json::Value> = std::cell::RefCell::new(serde_json::Value::Null);
    pub static VERSION_SEL_JSON: std::cell::RefCell<serde_json::Value> = std::cell::RefCell::new(serde_json::Value::Null);
    pub static CHOOSE_VERSION: std::cell::RefCell<i32> = std::cell::RefCell::new(-1);
    pub static CHOOSE_VERSION_SEL: std::cell::RefCell<i32> = std::cell::RefCell::new(-1);
    pub static CURRENT_VERSION: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    pub static CURRENT_VERSION_SEL: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    pub static IS_ISOLATION: std::cell::RefCell<i32> = std::cell::RefCell::new(4);
}

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
    IS_ISOLATION.set(input_num); 
    crate::main_method::TLM_INI
        .get()
        .expect("Cannot get TLM ini Value")
        .write_str("Version", "SelectIsolation", IS_ISOLATION.with_borrow(|e| e.clone())
            .to_string()
            .as_str());
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}
pub fn check_version() {
    println!("当前全局设置为：");
    if CHOOSE_VERSION.with_borrow(|e| e.clone()) < 0 {
        println!("{}", ansi_term::Color::Yellow.paint("你还暂未选择任一文件列表！"));
    }else{
        let p = VERSION_JSON.with_borrow(|e| e.clone());
        let p = p
                .as_object()
                .expect("JSON Parse Error!")
                .get("mc")
                .expect("JSON Parse Error!")
                .get(CHOOSE_VERSION.with_borrow(|e| e.clone()) as usize)
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
        if CHOOSE_VERSION_SEL.with_borrow(|e| e.clone()) < 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未选择任一游戏版本！"));
        }else{
            let p = VERSION_SEL_JSON.with_borrow(|e| e.clone());
            let p = p
                    .as_object()
                    .expect("JSON Parse Error!")
                    .get("mcsel")
                    .expect("JSON Parse Error!")
                    .get(CHOOSE_VERSION_SEL.with_borrow(|e| e.clone()) as usize)
                    .expect("JSON Parse Error!");
            let ph = p
                    .get("path")
                    .expect("JSON Parse Error!")
                    .as_str()
                    .expect("JSON Parse Error!");
            println!("\n你当前选中的游戏版本是：\n{}", ph);
        }
    }
    let ib = IS_ISOLATION.with_borrow(|e| e.clone());
    println!("\n当前全局版本隔离为：{}", if ib == 1 { 
        "不开启版本隔离"
    } else if ib == 2 {
        "隔离【正式版/快照版/远古Beta版/远古Alpha版】等版本"
    } else if ib == 3 {
        "隔离【Forge/Fabric/Quilt/NeoForge】等版本"
    } else {
        "隔离全部版本"
    });
    println!("当前你选中的版本独立设置为：");
}
pub fn reload_version() {
    if CHOOSE_VERSION.with_borrow(|e| e.clone()) < 0 { return; }
    VERSION_SEL_JSON.with_borrow_mut(|ver_root| {
        let ver_root = ver_root.get_mut("mcsel").expect("Cannot get mcsel!").as_array_mut().expect("Cannot get mcsel");
        ver_root.clear();
        let mc_root = VERSION_JSON.with_borrow(|e| e.clone());
        let mc_root = mc_root.as_object().unwrap();
        let path = mc_root["mc"][CHOOSE_VERSION.with_borrow(|e| e.clone()) as usize]["path"]
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
    });
}
pub fn rename_root() {
    VERSION_JSON.with_borrow_mut(|ver_obj| {
        let ver_obj = ver_obj["mc"].as_array_mut().expect("JSON Parse Error!");
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
    });
}
pub fn remove_root() {
    VERSION_JSON.with_borrow_mut(|ver_obj| {
        let ver_obj = ver_obj["mc"].as_array_mut().expect("JSON Parse Error!");
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
        let cv = CHOOSE_VERSION.with_borrow(|e| e.clone());
        if cv > input_num as i32 {
            CHOOSE_VERSION.set(cv - 1);
        } else if cv == input_num as i32 {
            CHOOSE_VERSION.set(-1);
            CHOOSE_VERSION_SEL.set(-1);
            CURRENT_VERSION.set(String::new());
            CURRENT_VERSION_SEL.set(String::new());
            VERSION_SEL_JSON.set(serde_json::from_str::<serde_json::Value>("{\"mcsel\":[]}").unwrap());
        }
        crate::main_method::TLM_INI
            .get()
            .expect("Cannot read TLM ini Value")
            .write_str("MC", "SelectMC", CHOOSE_VERSION.with_borrow(|e| e.clone())
                .to_string()
                .as_str());
        crate::main_method::TLM_INI
            .get()
            .expect("Cannot read TLM ini Value")
            .write_str("MC", "SelectVer", CHOOSE_VERSION_SEL.with_borrow(|e| e.clone())
                .to_string()
                .as_str());
        ver_obj.remove(input_num);
    });
    reload_version();
    save_version();
    println!("{}", ansi_term::Color::Green.paint("移除成功！"));
}
pub fn rename_version() {
    VERSION_SEL_JSON.with_borrow(|ver_obj| {
        let ver_obj = ver_obj["mcsel"].as_array().expect("JSON Parse Error!");
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
        let path = format!("{}\\versions\\{}", CURRENT_VERSION.with_borrow(|e| e.clone()).clone(), ver_obj[input_num]["path"].as_str().unwrap());
        let path = std::path::Path::new(path.as_str());
        let parent = path.parent().unwrap();
        let path2 = parent.join(input_str);
        let path2 = path2.as_path();
        std::fs::rename(path, path2).expect("Cannot rename file path!");
    });
    reload_version();
    println!("{}", ansi_term::Color::Green.paint("修改成功！"));
}
pub fn remove_version() {
    VERSION_SEL_JSON.with_borrow(|ver_obj| {
        let ver_obj = ver_obj["mcsel"].as_array().expect("JSON Parse Error!");
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
        let path = format!("{}\\versions\\{}", CURRENT_VERSION.with_borrow(|e| e.clone()).clone(), ver_obj[input_num]["path"].as_str().unwrap());
        let path = path.as_str();
        std::fs::remove_dir_all(path).expect("Cannot remove version dir!");
        let cvs = CHOOSE_VERSION_SEL.with_borrow(|e| e.clone());
        if (input_num as i32) < cvs {
            CHOOSE_VERSION_SEL.set(cvs - 1);
        } else if (input_num as i32) == cvs {
            CHOOSE_VERSION_SEL.set(-1);
            CURRENT_VERSION_SEL.set(String::new());
        }
        crate::main_method::TLM_INI
            .get()
            .expect("Cannot read TLM ini Value")
            .write_str("MC", "SelectVer", CHOOSE_VERSION_SEL.with_borrow(|e| e.clone())
                .to_string()
                .as_str());
    });
    reload_version();
    println!("{}", ansi_term::Color::Green.paint("删除完成！"));
}
pub fn choose_version() {
    if CHOOSE_VERSION.with_borrow(|e| e.clone()) < 0 {
        println!("{}", ansi_term::Color::Yellow.paint("你还暂未添加任意文件夹哦！请添加一个再来！"));
        return;
    }
    VERSION_SEL_JSON.with_borrow(|ver_obj| {
        let ver_obj = ver_obj["mcsel"].as_array().expect("JSON Parse Error!");
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
        crate::main_method::TLM_INI.get()
            .expect("Cannot read TLM ini Value")
            .write_str("MC", "SelectVer", input_num.to_string().as_str());
        CHOOSE_VERSION_SEL.set(input_num as i32);
        CURRENT_VERSION_SEL.set(ver_obj[input_num]["path"].as_str().unwrap().to_string());
    }); 
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}
pub fn choose_root() {
    VERSION_JSON.with_borrow(|ver_obj| {
        let ver_obj = ver_obj["mc"].as_array().expect("JSON Parse Error!");
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
        CHOOSE_VERSION.set(input_num as i32);
        CHOOSE_VERSION_SEL.set(-1);
        let act_ver = ver_obj[input_num]["path"].as_str().unwrap().to_string();
        CURRENT_VERSION.set(act_ver.clone());
        crate::main_method::TLM_INI.get()
            .expect("Cannot read TLM ini Value")
            .write_str("MC", "SelectMC", input_num.to_string().as_str());
        crate::main_method::TLM_INI.get()
            .expect("Cannot read TLM ini Value")
            .write_str("MC", "SelectVer", "-1");
    });
    reload_version();
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
    VERSION_JSON.with_borrow_mut(|ver_obj| {
        //添加根文件夹
        let ver_obj = ver_obj.get_mut("mc").expect("JSON Parse Error!");
        let ver_obj = ver_obj.as_array_mut().expect("JSON Parse Error!");
        let mut push_obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        let push_obj = push_obj.as_object_mut().unwrap();
        push_obj.insert(String::from("name"), serde_json::Value::String(input_name.clone()));
        push_obj.insert(String::from("path"), serde_json::Value::String(input_dir.clone().replace("/", "\\")));
        ver_obj.push(serde_json::Value::Object(push_obj.clone()));
    });
    save_version();
    println!("{}", ansi_term::Color::Green.paint("添加成功！"));
}
pub fn save_version() {
    let version_json = VERSION_JSON.with_borrow(|e| e.clone());
    let version_sel_json = VERSION_SEL_JSON.with_borrow(|e| e.clone());
    let cdir = crate::main_method::CURRENT_DIR.get().expect("Cannot read Current Dir Value!");
    crate::rust_lib::main_mod::set_file(
        format!("{}\\TankLauncherModule\\configs\\MCJSON.json", cdir.clone()).as_str(), 
        serde_json::to_string_pretty(&version_json.clone()).unwrap());
    crate::rust_lib::main_mod::set_file(
        format!("{}\\TankLauncherModule\\configs\\MCSelJSON.json", cdir.clone()).as_str(), 
        serde_json::to_string_pretty(&version_sel_json.clone()).unwrap());
}