pub fn check_download() {
    println!("您目前选择的下载源是：{}\n选择的下载最大并发量是：{}", 
        if crate::rust_lib::some_var::DOWNLOAD_SOURCE.with_borrow(|e| e.clone()) == 2 { "BMCLAPI" } else { "Official" }, 
        crate::rust_lib::some_var::BIGGEST_THREAD.with_borrow(|e| e.clone()))
}

pub fn get_minecraft() {
    println!("----------------------------------------------");
    println!("请输入你想要显示的方式：");
    println!("1. 正式版");
    println!("2. 快照版");
    println!("3. 远古Beta版");
    println!("4. 远古Alpha版");
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
    let tok = tokio::runtime::Runtime::new().unwrap();
    tok.block_on(async move {
        let g = crate::rust_lib::download_mod::get_mc_versions().await;
        match g {
            Ok(e) => {
                let e = e["versions"].as_array().expect("Cannot get MC JSON versions!");
                let mut res: Vec<String> = Vec::new();
                for i in 0..e.len(){
                    let te = e[i]["type"].as_str().expect("Cannot get MC JSON versions type!");
                    let ne = e[i]["id"].as_str().expect("Cannot get MC JSON versions id!");
                    if input_num == 1 && te.eq("release") || input_num == 2 && te.eq("snapshot") || input_num == 3 && te.eq("old_beta") || input_num == 4 && te.eq("old_alpha") {
                        res.push(format!("{}. {}", i, ne));
                    }
                }
                for i in res.into_iter() {
                    println!("{}", i);
                }
            },
            Err(e) => {
                match e {
                    -1 => {
                        println!("{}", ansi_term::Color::Red.paint("无法获取MC元数据，请确保您已接入网络。"));
                    }
                    _ => {
                        println!("{}", ansi_term::Color::Red.paint(format!("出现了未知错误，错误代码：{}，请立即联系作者！", e)));
                    }
                }
            }
        }
    });
}

pub fn get_forge() {
    println!("请输入你想查看的MC版本（例如1.20.4）");
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str).expect("Cannot read int!");
    let input_str = input_str.trim();
    let tok = tokio::runtime::Runtime::new().unwrap();
    tok.block_on(async move {
        let g = crate::rust_lib::download_mod::get_forge_versions(input_str).await;
        match g {
            Ok(e) => {
                let e = e["forge"].as_array().expect("Cannot get Forge JSON versions!");
                let mut res: Vec<String> = Vec::new();
                for i in 0..e.len(){
                    let j = e[i]["version"].as_str().expect("Cannot get Forge JSON version!");
                    res.push(format!("{}. {}", i, j));
                }
                for i in res.into_iter() {
                    println!("{}", i);
                }
            },
            Err(e) => {
                match e {
                    -1 => {
                        println!("{}", ansi_term::Color::Red.paint("无法获取Forge元数据，请确保您已接入网络。"));
                    }
                    -2 => {
                        println!("{}", ansi_term::Color::Red.paint("无法获取Forge版本，请确保你填入的是正确的MC版本。"));
                    }
                    _ => {
                        println!("{}", ansi_term::Color::Red.paint(format!("出现了未知错误，错误代码：{}，请立即联系作者！", e)));
                    }
                }
            }
        }
    });
}

pub fn get_fabric() {
    println!("请输入你想查看的MC版本（例如1.20.4）");
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str).expect("Cannot read int!");
    let input_str = input_str.trim();
    let tok = tokio::runtime::Runtime::new().unwrap();
    tok.block_on(async move {
        let g = crate::rust_lib::download_mod::get_fabric_version(input_str).await;
        match g {
            Ok(e) => {
                let e = e["fabric"].as_array().expect("Cannot get Fabric JSON versions!");
                let mut res: Vec<String> = Vec::new();
                for i in 0..e.len(){
                    let j = e[i]["version"].as_str().expect("Cannot get Fabric JSON version!");
                    res.push(format!("{}. {}", i, j));
                }
                for i in res.into_iter() {
                    println!("{}", i);
                }
            },
            Err(e) => {
                match e {
                    -1 => {
                        println!("{}", ansi_term::Color::Red.paint("无法获取Fabric元数据，请确保您已接入网络。"));
                    }
                    -3 => {
                        println!("{}", ansi_term::Color::Red.paint("无法获取Fabric版本，请确保你填入的是正确的MC版本。"));
                    }
                    _ => {
                        println!("{}", ansi_term::Color::Red.paint(format!("出现了未知错误，错误代码：{}，请立即联系作者！", e)));
                    }
                }
            }
        }
    });
}

pub fn get_quilt() {
    println!("请输入你想查看的MC版本（例如1.20.4）");
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str).expect("Cannot read int!");
    let input_str = input_str.trim();
    let tok = tokio::runtime::Runtime::new().unwrap();
    tok.block_on(async move {
        let g = crate::rust_lib::download_mod::get_quilt_version(input_str).await;
        match g {
            Ok(e) => {
                let e = e["quilt"].as_array().expect("Cannot get Quilt JSON versions!");
                let mut res: Vec<String> = Vec::new();
                for i in 0..e.len(){
                    let j = e[i]["version"].as_str().expect("Cannot get Quilt JSON version!");
                    res.push(format!("{}. {}", i, j));
                }
                for i in res.into_iter() {
                    println!("{}", i);
                }
            },
            Err(e) => {
                match e {
                    -1 => {
                        println!("{}", ansi_term::Color::Red.paint("无法获取Quilt元数据，请确保您已接入网络。"));
                    }
                    -4 => {
                        println!("{}", ansi_term::Color::Red.paint("无法获取Quilt版本，请确保你填入的是正确的MC版本。"));
                    }
                    _ => {
                        println!("{}", ansi_term::Color::Red.paint(format!("出现了未知错误，错误代码：{}，请立即联系作者！", e)));
                    }
                }
            }
        }
    });
}

pub fn get_neoforge() {
    println!("请输入你想查看的MC版本（例如1.20.4）");
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input_str).expect("Cannot read int!");
    let input_str = input_str.trim();
    let tok = tokio::runtime::Runtime::new().unwrap();
    tok.block_on(async move {
        let g = crate::rust_lib::download_mod::get_neoforge_version(input_str).await;
        match g {
            Ok(e) => {
                let e = e["neoforge"].as_array().expect("Cannot get NeoForge JSON versions!");
                let mut res: Vec<String> = Vec::new();
                for i in 0..e.len(){
                    let j = e[i]["version"].as_str().expect("Cannot get NeoForge JSON version!");
                    res.push(format!("{}. {}", i, j));
                }
                for i in res.into_iter() {
                    println!("{}", i);
                }
            },
            Err(e) => {
                match e {
                    -1 => {
                        println!("{}", ansi_term::Color::Red.paint("无法获取NeoForge元数据，请确保您已接入网络。"));
                    }
                    -5 => {
                        println!("{}", ansi_term::Color::Red.paint("无法获取NeoForge版本，请确保你填入的是正确的MC版本。"));
                    }
                    _ => {
                        println!("{}", ansi_term::Color::Red.paint(format!("出现了未知错误，错误代码：{}，请立即联系作者！", e)));
                    }
                }
            }
        }
    });
}

pub fn set_max_thread() {
    let mut input_num = String::new();
    println!("请输入你要设置下载最大并发数（Rust并不限制并发数，不过这里还是规定256为最大的）：");
    std::io::stdin().read_line(&mut input_num).expect("Cannot read number!");
    let input_num = input_num.trim().parse::<i32>();
    if let Err(_) = input_num {
        println!("{}", ansi_term::Color::Red.paint("请不要输入错误的值！"));
        return;
    }
    let input_num = input_num.unwrap();
    if input_num <= 0 || input_num > 256 {
        println!("{}", ansi_term::Color::Red.paint("请不要输入错误的值！"));
        return;
    }
    crate::rust_lib::some_var::BIGGEST_THREAD.set(input_num);
    crate::main_method::TLM_INI.with_borrow(|e| e.write_str("Version", "ThreadBiggest", input_num.to_string().as_str()));
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}

pub fn set_download_source() {
    println!("----------------------------------------------");
    println!("请输入你想要选择的下载源：");
    println!("1. 官方源");
    println!("2. BMCLAPI");
    println!("----------------------------------------------");
    let mut input_num = String::new();
    std::io::stdin().read_line(&mut input_num).expect("Cannot read number!");
    let input_num = input_num.trim().parse::<i32>();
    if let Err(_) = input_num {
        println!("{}", ansi_term::Color::Red.paint("请不要输入错误的值！"));
        return;
    }
    let input_num = input_num.unwrap();
    if input_num < 1 || input_num > 2 {
        println!("{}", ansi_term::Color::Red.paint("请不要输入错误的值！"));
        return;
    }
    crate::rust_lib::some_var::DOWNLOAD_SOURCE.set(input_num);
    crate::main_method::TLM_INI.with_borrow(|e| e.write_str("Version", "SelectDownloadSource", input_num.to_string().as_str()));
    println!("{}", ansi_term::Color::Green.paint("设置成功！"));
}