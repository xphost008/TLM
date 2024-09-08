pub fn is_isolation(root_path: String, sel_path: String) -> String {
    let real_path = crate::rust_lib::launcher_mod::get_mc_real_path(sel_path.clone(), ".json").expect("Cannot read version json!");
    let sel_cont = crate::rust_lib::main_mod::get_file(real_path.as_str()).expect("Read version file content error!").to_lowercase();
    let pd = (sel_cont.contains("neoforge")) || (sel_cont.contains("forge")) || sel_cont.contains("quilt") || sel_cont.contains("fabric");
    let iso = crate::version::IS_ISOLATION.with_borrow(|e| e.clone());
    return if iso == 4 {
        sel_path.clone()
    }else if iso == 3 {
        if pd {
            sel_path.clone()
        }else{
            root_path.clone()
        }
    }else if iso == 2 {
        if pd {
            sel_path.clone()
        }else{
            root_path.clone()
        }
    }else {
        root_path.clone()
    }
}

pub fn start_launch(option: crate::rust_lib::launcher_mod::LaunchOption){
    let catch = std::panic::catch_unwind(move || {
        let option = option.clone();
        let launch = crate::rust_lib::launcher_mod::launch_game(option.clone(), move |command| {
            let binding = crate::main_method::APP_DATA.with_borrow(|e| e.clone());
            let output_file_path = format!("{}\\TankLauncherModule\\run.bat", binding.clone());
            let game_path = option.get_game_path();
            let cmd_str = command.iter().map(|e| format!("\"{}\"", e.to_string())).collect::<Vec<String>>().join(" ");
            if crate::rust_lib::main_mod::set_file(output_file_path.as_str(), cmd_str.clone()) {
                println!("参数拼接成功！正在为您启动游戏！");
                let cmd = std::process::Command::new("cmd")
                    .arg("/c")
                    .arg(output_file_path.as_str())
                    .current_dir(game_path)
                    .spawn();
                if let Ok(mut o) = cmd {
                    if let Ok(e) = o.wait() {
                        if let Some(f) = e.code() {
                            println!("程序已退出，退出代码为：{}", f);
                            if !crate::rust_lib::main_mod::delete_file(output_file_path.as_str()) {
                                println!("{}", ansi_term::Color::Yellow.paint("文件删除失败！"));
                            }
                        }
                    }
                }
            }else{
                println!("{}", ansi_term::Color::Red.paint("启动失败，文件输出失败！"));
            }
        });
        if let Err(e) = launch {
            match e {
                -1 => {
                    println!("{}", ansi_term::Color::Red.paint("账号名称格式错误！"));
                }
                -2 => {
                    println!("{}", ansi_term::Color::Red.paint("账号UUID格式错误"));
                }
                -3 => {
                    println!("{}", ansi_term::Color::Red.paint("账号AccessToken错误"));
                }
                -4 => {
                    println!("{}", ansi_term::Color::Red.paint("账号未购买正版"));
                }
                -5 => {
                    println!("{}", ansi_term::Color::Red.paint("账号第三方的AccessToken或者URL错误。"));
                }
                -6 => {
                    println!("{}", ansi_term::Color::Red.paint("账号base64编码错误"));
                }
                -7 => {
                    println!("{}", ansi_term::Color::Red.paint("Java路径错误（文件未找到）"));
                }
                -8 => {
                    println!("{}", ansi_term::Color::Red.paint("游戏根路径错误（文件夹未找到）"));
                }
                -9 => {
                    println!("{}", ansi_term::Color::Red.paint("游戏版本路径错误（文件夹未找到）"));
                }
                -10 => {
                    println!("{}", ansi_term::Color::Red.paint("游戏实际路径错误（文件夹未找到）"));
                }
                -11 => {
                    println!("{}", ansi_term::Color::Red.paint("窗口宽度错误（小于854或大于屏幕宽度）"));
                }
                -12 => {
                    println!("{}", ansi_term::Color::Red.paint("窗口高度错误（小于480或大于屏幕高度）"));
                }
                -13 => {
                    println!("{}", ansi_term::Color::Red.paint("最大或最小内存错误（最大内存小于1024或大于系统内存 或 最小内存小于256或大于1024）"));
                }
                -14 => {
                    println!("{}", ansi_term::Color::Red.paint("自定义信息错误（未填写，必须要个默认值！）"));
                }
                _ => {
                    println!("{}", ansi_term::Color::Red.paint("出现了未知错误！请立刻联系TLM开发者！"));
                }
            }
        }
    });
    if let Err(e) = catch {
        let cp = e.downcast_ref::<&str>();
        println!("{}", ansi_term::Color::Red.paint(format!("{}{}", "启动时参数检测无误，但是拼接参数时出现了错误！错误提示：\n", cp.unwrap())));
    }
}