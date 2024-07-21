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
        println!("输入了不正确的数字！");
        return;
    }
    let input_num = input_num.unwrap();
    if input_num < 1 || input_num > 4 {
        println!("输入了不正确的数字！");
        return;
    }
    unsafe { IS_ISOLATION = input_num; }
    save_version();
    println!("设置完成！");
}
pub fn check_version() {
    unsafe {
        println!("当前全局设置为：");
        if CHOOSE_VERSION < 0 {
            println!("你还暂未选择任一文件列表！");
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
                println!("你还暂未选择任一游戏版本！");
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
                println!("\n你当前选中的游戏版本路径是：\n{}", ph);
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
pub fn choose_version() {
    unsafe {
        let ver_obj = VERSION_SEL_JSON.as_object().expect("JSON Parse Error!");
        let ver_obj = ver_obj.get("mcsel").expect("JSON Parse Error!");
        let ver_obj = ver_obj.as_array().expect("JSON Parse Error!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..ver_obj.len() {
            let j = ver_obj[i].as_object().expect("JSON Parse Error!");
            let p = j.get("path").expect("JSON Parse Error!");
            let p = p.as_str().expect("JSON Parse Error!");
            res.push(format!("{}. {}", i + 1, p));
        }
        if res.len() == 0 {
            println!("你还暂未选择任意文件夹哦！请选择一个再来！");
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
            println!("输入了错误的数字，请重新输入！1");
            return;
        }
        let input_num = input_num.unwrap();
        if input_num > ver_obj.len() || input_num < 1 {
            println!("输入了错误的数字，请重新输入！2");
            return;
        }
        let input_num = input_num - 1;
        if !crate::main_method::TLM_INI.write_str("MC", "SelectVer", input_num.to_string().as_str()) {
            println!("写出INI文件失败，请重试！");
            return;
        }
        CHOOSE_VERSION_SEL = input_num as i32;
        CURRENT_VERSION_SEL = ver_obj[input_num].get("path").unwrap().to_string();
    }
    save_version();
    println!("设置完成！");
}
pub fn choose_root() {
    unsafe {
        let ver_obj = VERSION_JSON.get("mc").expect("JSON Parse Error!");
        let ver_obj = ver_obj.as_array().expect("JSON Parse Error!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..ver_obj.len() {
            let j = ver_obj[i].as_object().expect("JSON Parse Error!");
            let n = j.get("name").expect("JSON Parse Error!");
            let p = j.get("path").expect("JSON Parse Error!");
            let n = n.as_str().expect("JSON Parse Error!");
            let p = p.as_str().expect("JSON Parse Error!");
            res.push(format!("{}. {} - {}", i + 1, n, p));
        }
        if res.len() == 0 {
            println!("你还暂未添加任意文件夹哦！请添加一个再来！");
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
            println!("输入了错误的数字，请重新输入！1");
            return;
        }
        let input_num = input_num.unwrap();
        if input_num > ver_obj.len() || input_num < 1 {
            println!("输入了错误的数字，请重新输入！2");
            return;
        }
        let input_num = input_num - 1;
        if !crate::main_method::TLM_INI.write_str("MC", "SelectMC", input_num.to_string().as_str()) {
            println!("写出INI文件失败，请重试！");
            return;
        }
        CHOOSE_VERSION = input_num as i32;
        let act_ver = ver_obj[input_num].clone();
        let act_ver = act_ver.get("path").expect("JSON Parse Error!");
        let act_ver = act_ver.as_str().expect("JSON Parse Error!");
        let act_ver = act_ver.replace("\\\\", "\\");
        let path = std::path::Path::new(act_ver.as_str());
        let path = path.join("versions");
        let ver_obj = VERSION_SEL_JSON.get_mut("mcsel").expect("JSON Parse Error!").as_array_mut().expect("JSON Parse Error!");
        ver_obj.clear();
        let walk = walkdir::WalkDir::new(path).max_depth(1).min_depth(1);
        for i in walk.into_iter().filter_map(|e| e.ok()) {
            let i = i.path().to_str().unwrap();
            let mut push_obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
            let push_obj = push_obj.as_object_mut().unwrap();
            push_obj.insert(String::from("path"), serde_json::from_str(format!("\"{}\"", i.replace("\\", "\\\\").replace("/", "\\\\")).as_str()).unwrap());
            ver_obj.push(serde_json::Value::Object(push_obj.clone()));
        }
        CURRENT_VERSION = act_ver.clone();
    }
    save_version();
    println!("设置完成！");
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
    let binding_dir = input_dir.clone();
    let path = std::path::Path::new(binding_dir.as_str());
    if !path.exists() || path.exists() && path.is_file() {
        println!("路径输入错误，请不要输入不存在的路径！");
        return;
    }
    unsafe {
        //添加根文件夹
        let ver_obj = VERSION_JSON.get_mut("mc").expect("JSON Parse Error!");
        let ver_obj = ver_obj.as_array_mut().expect("JSON Parse Error!");
        let mut push_obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        let push_obj = push_obj.as_object_mut().unwrap();
        push_obj.insert(String::from("name"), serde_json::from_str(format!("\"{}\"", input_name.clone()).as_str()).unwrap());
        push_obj.insert(String::from("path"), serde_json::from_str(format!("\"{}\"", input_dir.clone().replace("\\", "\\\\").replace("/", "\\\\")).as_str()).unwrap());
        ver_obj.push(serde_json::Value::Object(push_obj.clone()));
    }
    save_version();
    println!("添加完成！")
}
pub fn save_version() {
    unsafe {
        crate::rust_lib::main_mod::set_file(
            format!("{}\\TankLauncherModule\\configs\\MCJSON.json", crate::main_method::CURRENT_DIR).as_str(), 
            serde_json::to_string_pretty(&VERSION_JSON.clone()).unwrap());
        crate::rust_lib::main_mod::set_file(
            format!("{}\\TankLauncherModule\\configs\\MCSelJSON.json", crate::main_method::CURRENT_DIR).as_str(), 
            serde_json::to_string_pretty(&VERSION_SEL_JSON.clone()).unwrap());
        if !crate::main_method::TLM_INI.write_str("Version", "SelectIsolation", IS_ISOLATION.to_string().as_str()) {
            panic!("Write str to TankLauncherModule.ini is error in isolation!");
        }
    }
}