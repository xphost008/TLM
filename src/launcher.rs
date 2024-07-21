pub static mut OUTPUT_FILE_PATH: String = String::new();

pub fn is_isolation(root_path: String, sel_path: String) -> String {
    unsafe {
        let real_path = crate::rust_lib::launcher_mod::get_mc_real_path(sel_path.clone(), ".json").expect("Cannot read version json!");
        let sel_cont = crate::rust_lib::main_mod::get_file(real_path.as_str()).expect("Read version file content error!");
        let pd = (sel_cont.contains("neoforge")) || (sel_cont.contains("forge")) || sel_cont.contains("quilt") || sel_cont.contains("fabric");
        return if crate::version::IS_ISOLATION == 4 {
            sel_path.clone()
        }else if crate::version::IS_ISOLATION == 3 {
            if pd {
                sel_path.clone()
            }else{
                root_path.clone()
            }
        }else if crate::version::IS_ISOLATION == 2 {
            if pd {
                sel_path.clone()
            }else{
                root_path.clone()
            }
        }else {
            root_path.clone()
        }
    }
}

pub fn start_launch(option: crate::rust_lib::launcher_mod::LaunchOption){
    let catch = std::panic::catch_unwind(|| {
        let launch = crate::rust_lib::launcher_mod::launch_game(option, |command: Vec<&str>| {
            unsafe {
                let game_path = crate::launcher::is_isolation(crate::version::CURRENT_VERSION.clone(), crate::version::CURRENT_VERSION_SEL.clone()).clone();
                let cmd_str = command.iter().map(|e| format!("\"{}\"", e.to_string())).collect::<Vec<String>>().join(" ");
                if crate::rust_lib::main_mod::set_file(OUTPUT_FILE_PATH.as_str(), cmd_str.clone()) {
                    println!("参数拼接成功！正在为您启动游戏！");
                    let cmd = std::process::Command::new("cmd")
                        .arg("/c")
                        .arg(OUTPUT_FILE_PATH.as_str())
                        .current_dir(game_path)
                        .spawn();
                    if let Ok(mut o) = cmd {
                        if let Ok(e) = o.wait() {
                            if let Some(f) = e.code() {
                                println!("程序已退出，退出代码为：{}", f);
                                if !crate::rust_lib::main_mod::delete_file(OUTPUT_FILE_PATH.as_str()) {
                                    println!("文件删除失败！");
                                }
                            }
                        }
                    }
                }else{
                    println!("启动失败，文件输出失败！");
                }
            }
        });
        if let Err(e) = launch {
            match e {
                -1 => {
                    println!("账号名称格式错误！")
                }
                -2 => {
                    println!("账号UUID格式错误")
                }
                -3 => {
                    println!("账号AccessToken错误")
                }
                -4 => {
                    println!("账号未购买正版")
                }
                -5 => {
                    println!("账号第三方的AccessToken或者URL错误。")
                }
                -6 => {
                    println!("账号base64编码错误")
                }
                -7 => {
                    println!("Java路径错误（文件未找到）")
                }
                -8 => {
                    println!("游戏根路径错误（文件夹未找到）")
                }
                -9 => {
                    println!("游戏版本路径错误（文件夹未找到）")
                }
                -10 => {
                    println!("游戏实际路径错误（文件夹未找到）")
                }
                -11 => {
                    println!("窗口宽度错误（小于854或大于屏幕宽度）")
                }
                -12 => {
                    println!("窗口高度错误（小于480或大于屏幕高度）")
                }
                -13 => {
                    println!("最大或最小内存错误（最大内存小于1024或大于系统内存 或 最小内存小于256或大于1024）")
                }
                -14 => {
                    println!("自定义信息错误（未填写，必须要个默认值！）")
                }
                _ => {
                    println!("出现了未知错误！请立刻联系TLM开发者！")
                }
            }
        }
    });
    if let Err(e) = catch {
        let cp = e.downcast_ref::<&str>();
        println!("启动时参数检测无误，但是拼接参数时出现了错误！错误提示：\n{}", cp.unwrap());
    }
}