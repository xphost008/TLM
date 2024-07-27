mod rust_lib;
mod launcher;
mod main_method;
mod read_ini;
mod version;
mod launch;

fn show_title(){
    println!(r" _____                _      _                                 _                  ___  ___            _         _       ");
    println!(r"|_   _|              | |    | |                               | |                 |  \/  |           | |       | |      ");
    println!(r"  | |    __ _  _ __  | | __ | |      __ _  _   _  _ __    ___ | |__    ___  _ __  | .  . |  ___    __| | _   _ | |  ___ ");
    println!(r"  | |   / _` || '_ \ | |/ / | |     / _` || | | || '_ \  / __|| '_ \  / _ \| '__| | |\/| | / _ \  / _` || | | || | / _ \");
    println!(r"  | |  | (_| || | | ||   <  | |____| (_| || |_| || | | || (__ | | | ||  __/| |    | |  | || (_) || (_| || |_| || ||  __/");
    println!(r"  \_/   \__,_||_| |_||_|\_\ \_____/ \__,_| \__,_||_| |_| \___||_| |_| \___||_|    \_|  |_/ \___/  \__,_| \__,_||_| \___|");
}
fn show_main_menu(){
    println!("----------------------------------------------");
    println!("|        欢迎来到Tank Launcher Module        |");
    println!("|        请输入对应的数字启动对应的功能      |");
    println!("|        0. 查看帮助文档                     |");
    println!("|        1. 查看统计信息                     |");
    println!("|        2. 启动游戏                         |");
    println!("|        3. 查看当前信息                     |");
    println!("|        4. 版本设置                         |");
    println!("|        5. 启动设置                         |");
    println!("|        6. 联机部分                         |");
    println!("|        7. 资源部分                         |");
    println!("|        8. 账号部分                         |");
    println!("|        9. 下载部分                         |");
    println!("|        10. 杂项部分                        |");
    println!("|        q. 退出启动器                       |");
    println!("----------------------------------------------");
}
fn show_version(){
    println!("----------------------------------------------");
    println!("|        请输入对应的数字启动对应的功能      |");
    println!("|        0. 查看版本信息                     |");
    println!("|        1. 选择游戏版本                     |");
    println!("|        2. 选择文件列表                     |");
    println!("|        3. 添加文件夹                       |");
    println!("|        4. 设置隔离                         |");
    println!("|        5. 补全该版本类库                   |");
    println!("|        6. 重命名文件列表                   |");
    println!("|        7. 移除文件列表                     |");
    println!("|        8. 重命名游戏版本                   |");
    println!("|        9. 删除游戏版本                     |");
    println!("|        10. 独立设置版本                    |");
    println!("|        11. 导出整合包                      |");
    println!("|        q. 退出当前页                       |");
    println!("----------------------------------------------");
}
fn show_launch() {
    println!("----------------------------------------------");
    println!("|        请输入对应的数字启动对应的功能      |");
    println!("|        0. 查看启动设置                     |");
    println!("|        1. 设置窗口宽度                     |");
    println!("|        2. 设置窗口高度                     |");
    println!("|        3. 设置最大内存                     |");
    println!("|        4. 设置最小内存                     |");
    println!("|        5. 添加Java                         |");
    println!("|        6. 选择Java                         |");
    println!("|        7. 下载Java                         |");
    println!("|        8. 扫描Java                         |");
    println!("|        9. 设置额外JVM参数                  |");
    println!("|        10. 设置额外Game参数                |");
    println!("|        q. 退出当前页                       |");
    println!("----------------------------------------------");
}
fn output_tlm_version() {
    println!("当前TLM版本：{}", rust_lib::some_const::TLM_VERSION);
}
fn outout_launch() {
    show_launch();
    loop {
        print!(">>> ");
        use std::io::Write;
        std::io::stdout().flush().expect("Cannot flush message!");
        let mut main_choice = String::new();
        std::io::stdin().read_line(&mut main_choice).expect("Cannot read stdin!");
        main_choice = main_choice.trim().to_string();
        if main_choice.eq("q") {
            show_main_menu(); 
            return; 
        }
        let conv = main_choice.parse::<i8>();
        if let Ok(t) = conv {
            match t {
                0 => {
                    launch::check_launch();
                },
                1 => {
                    launch::set_window_width();
                },
                2 => {
                    launch::set_window_height();
                },
                3 => {
                    launch::set_max_memory();
                }
                4 => {
                    launch::set_min_memory();
                }
                5 => {
                    launch::add_java();
                }
                6 => {
                    launch::choose_java();
                }
                7 => {
                    println!("暂未实现！");
                }
                8 => {
                    println!("暂未实现！");
                }
                9 => {
                    println!("暂未实现！");
                }
                10 => {
                    println!("暂未实现！");
                }
                11 => {
                    println!("暂未实现！");
                }
                _ => println!("{}", ansi_term::Color::Green.paint("请不要输入0-11以外的数字噢！")),
            }
        }else{
            println!("{}", ansi_term::Color::Green.paint("请不要输入数字以外的字符噢！"));
        }
    }
}
fn output_version() {
    show_version();
    loop {
        print!(">>> ");
        use std::io::Write;
        std::io::stdout().flush().expect("Cannot flush message!");
        let mut main_choice = String::new();
        std::io::stdin().read_line(&mut main_choice).expect("Cannot read stdin!");
        main_choice = main_choice.trim().to_string();
        if main_choice.eq("q") {
            show_main_menu(); 
            return; 
        }
        let conv = main_choice.parse::<i8>();
        if let Ok(t) = conv {
            match t {
                0 => {
                    version::check_version();
                },
                1 => {
                    version::choose_version();
                },
                2 => {
                    version::choose_root();
                },
                3 => {
                    version::add_directory();
                }
                4 => {
                    version::set_isolation();
                }
                5 => {
                    println!("暂未实现！");
                }
                6 => {
                    println!("暂未实现！");
                }
                7 => {
                    println!("暂未实现！");
                }
                8 => {
                    println!("暂未实现！");
                }
                9 => {
                    println!("暂未实现！");
                }
                10 => {
                    println!("暂未实现！");
                }
                11 => {
                    println!("暂未实现！");
                }
                _ => println!("{}", ansi_term::Color::Green.paint("请不要输入0-11以外的数字噢！")),
            }
        }else{
            println!("{}", ansi_term::Color::Green.paint("请不要输入数字以外的字符噢！"));
        }
    }
}
fn output_help(){
    println!("直接双击TLM即可进入启动器界面。但如果你想从命令行启动，请参见以下：\n");
    println!("用法：tlm [首值] [参数] [值]");
    println!("其中，[首值]包括：\n");
    println!("  -l | --launch <启动游戏>");
    println!("                  （该参数用处是直接通过后面跟的参数进行启动游戏。）");
    println!("                  （使用末尾接的设置或者使用默认设置）");
    println!("  -s | --setting <对启动器进行设置>");
    println!("                  （该参数用处是对游戏部分参数进行设置）");
    println!("                  （使用末尾接的设置）");
    println!("                  （不填则说明不用设置）");
    println!("  -d | --download <下载游戏>");
    println!("                  （该参数用处是下载游戏、模组整合包或者是自定义文件。）");
    println!("                  （使用末尾接的设置或者使用默认设置）");
    println!("  -h | --help <显示TTT帮助界面>");
    println!("                  （后面无需接任何参数）");
    println!("  -ps | --printSetting <直接打印输出当前设置>");
    println!("                  （后面无需接任何参数）\n");
    println!("  -v | --version <显示当前TLM版本号。>");
    println!("                  （后面无需接任何参数）\n");
    println!("首值的意思就是必须在首位的值。不能在别的位的值。");
    println!("首值的具体含义用于描述你要【设置】还是【启动】还是【下载】亦或是别的意图。\n");
    println!("以下，如果有的参数值有空格的话，需要使用双引号引起来。");
    println!("以下，如果描述后面接了【有默认值】四个字，则认为该参数后面可以接【default】使用默认的值，或者不填也可以。");
    println!("以下，所有参数均仅能够按照当前帮助文档输出的从上往下顺序进行赋值，否则可能会遇到很多意想不到的bug！\n");
    println!("以下是对于[-l | --launch]首值后跟的值：\n");
    println!("  -gd | --gameDir <设置/启动游戏根目录，有默认值=-1>");
    println!("                  （仅支持[值]为：【游戏目录路径、数字】）");
    println!("                  （如果是路径，则请输入完整的游戏目录路径）");
    println!("                  （如果是数字，请输入一个可用的索引）");
    println!("                  （如果目录不存在则会输出回显【gameDir Not Found】）");
    println!("  -v | --version <设置/启动游戏版本，有默认值=-1>");
    println!("                  （仅支持[值]为：【游戏版本名称、数字】）");
    println!("                  （如果是游戏版本名称，则请输入完整的/versions/<版本名称>中的版本名称字段）");
    println!("                  （如果是数字，请输入一个可用的索引）");
    println!("                  （如果版本不存在则会输出回显【version Not Found】）");
    println!("  -jp | --javaPath <设置/启动游戏Java路径>，有默认值=-1");
    println!("                  （此处可以填入【Java路径、数字】）");
    println!("                  （如果是路径，则请输入完整的Java路径）");
    println!("                  （如果是数字，请输入一个可用的索引）");
    println!("                  （如果Java不存在则会输出回显【javaPath Not Found】）");
    println!("  -a | --account <设置/启动游戏账号，有默认值=-1>");
    println!("                  （仅支持[值]为：【Offline、Microsoft、ThirdParty、数字】）");
    println!("                  （如果是Offline，则后面跟玩家用户名和UUID。如果UUID不想手动填可以填default）");
    println!("                  （如果是Microsoft，则会直接要求重新登录一次，按照要求登录即可！）");
    println!("                  （如果是ThirdParty，则后面要求填入【1. 服务器、2. 账号、3. 密码、4. 角色序号】！）");
    println!("                  （如果是仅数字，请输入一个可用的索引）");
    println!("                  （如果在填入仅数字的时候，账号access_token已过期，则会自动调用refresh_token进行重置。如果refresh_token也过期了，则会爆出【account Is Expire】。）");
    println!("                  （如果在填入仅数字的时候，账号索引超出范围或者不存在，则会输出回显【account Not Found】）");
    println!("  -w | --width <设置窗口宽度，有默认值=854>");
    println!("  -h | --height <设置窗口高度，有默认值=480>");
    println!("  -nm | --minMemory <设置游戏最小内存，有默认值=256>");
    println!("  -xm | --maxMemory <设置游戏最大内存，有默认值=4096>");
    println!("  -c | --customInfo <设置自定义信息，有默认值=\"Tank Launcher Module\">");
    println!("  -i | --isolation <设置是否版本隔离，有默认值=true>");
    println!("                  （这个后面需要跟布尔值，true或是false）");
    println!("                  （如果填入错误，则会爆出【isolation Value Error】）");
    println!("  -ps | --preScript <设置前置启动脚本，有默认值=空>");
    println!("  -as | --afterScript <设置后置启动脚本，有默认值=空>");
    println!("  -aj | --additionalJVM <设置额外JVM参数，有默认值=空>");
    println!("  -ag | --additionalGame <设置额外Game参数，有默认值=空>\n");
    println!("以下是对于[-s | --setting]首值后跟的值：\n");
    println!("  -gd | --gameDir <在游戏目录配置文件末尾添加一个游戏目录>");
    println!("                  （如果目录不存在则会新建一个目录文件夹）");
    println!("  -gi | --gameIndex <设置当前游戏目录索引>");
    println!("                  （该处是只当启动游戏时默认会按照当前索引下的游戏目录进行启动）");
    println!("  -vi | --versionIndex <设置当前游戏版本索引>");
    println!("                  （该处是只当启动游戏时默认会按照当前索引下的游戏版本进行启动）");
    println!("  -jp | --javaPath <在Java路径配置文件末尾添加一个Java路径>");
    println!("                  （如果Java不存在则会输出回显【javaPath Not Found】）");
    println!("  -ji | --javaIndex <设置当前Java路径索引>");
    println!("                  （该处是只当启动游戏时默认会按照当前索引下的Java路径进行启动）");
    println!("  -a | --account <在账号配置文件末尾添加一个账号>");
    println!("                  （与启动部分一样，只是这里仅支持Offline、Microsoft、ThirdParty，不支持数字。）");
    println!("  -ai | --accountIndex <设置当前账号索引>");
    println!("                  （如果账号不存在，则会输出回显【account Not Found】）");
    println!("  -w | --width <设置窗口宽度>");
    println!("  -h | --height <设置窗口高度>");
    println!("  -nm | --minMemory <设置游戏最小内存");
    println!("  -xm | --maxMemory <设置游戏最大内存");
    println!("  -c | --customInfo <设置自定义信息>");
    println!("  -i | --isolation <设置版本隔离选项>");
    println!("                  （只能填入1-4数字：）");
    println!("                  （1. 不使用版本隔离。）");
    println!("                  （2. 仅隔离正式版/快照版/远古版。）");
    println!("                  （3. 仅隔离Forge/Fabric/Quilt/NeoForge等版。）");
    println!("                  （4. 隔离全部版本。）");
    println!("  -ps | --preScript <设置前置启动脚本>");
    println!("  -as | --afterScript <设置后置启动脚本>");
    println!("  -aj | --additionalJVM <设置额外JVM参数>");
    println!("  -ag | --additionalGame <设置额外Game参数>");
    println!("  -t | --thread <设置最大下载并发数量>");
    println!("                  （该参数适用于下载的任意操作，包括但不限于下载自定义文件、下载MC。）");
    println!("  -sp | --savePath <设置【自定义下载任意文件】的默认保存路径>\n");
    println!("以下是对于[-d | --download]首值后跟的值：\n");
    println!("（使用TTT启动器的命令行形式下载，会在下载完成之后自动退出并释放线程）");
    println!("（使用-d形式进行下载可能会有点麻烦，因为其中涉及到许多参数，并且有部分参数是互斥的，有其中一个参数就不能有另一个参数。）\n");
    println!("（以下是对于【-u | --url】参数的解释：）");
    println!("  -u | --url <调用TTT高速下载引擎下载任意文件>");
    println!("                  （后面接网址）");
    println!("                  （此参数与【-dg | --downloadGame】参数互斥）");
    println!("  -sp | --savePath <输入保存路径，有默认值=空>");
    println!("                  （后面请输入完整路径，或者default调用默认值）");
    println!("                  （该参数为必须参数，不填的话会输出回显【savePath Is Invalid】！）");
    println!("                  （此参数与【-dg | --downloadGame】参数互斥）");
    println!("  -s | --sha1 <输入文件sha1值>，如果你有的话。");
    println!("                  （该参数为可选参数，如果不填则默认不校验文件完整性。）");
    println!("  -n | --name <输入保存名称>，建议连后缀一同输上。");
    println!("                  （该参数为可选参数，如果不填则默认为网址后缀名。）");
    println!("                  （此参数与【-dg | --downloadGame】参数互斥）");
    println!("  -t | --thread <输入最大线程>。有默认值=32\n");
    println!("（以下是对于【-dg | --downloadGame】参数的解释：）");
    println!("  -dg | --downloadGame <输入MC版本>");
    println!("                  （必须填入，你必须率先查看过MC此时更新的版本。）");
    println!("                  （如果你只想下载最新版，你可以填入release，会默认导入最新正式版进行下载。）");
    println!("                  （如果你想下载最新快照版，你可以填入snapshot，会默认导入最新快照版进行下载。）");
    println!("                  （此参数与【-u | --url】参数互斥）");
    println!("  -ml | --modLoader <输入模组加载器>");
    println!("                  （可选填入，如果你想下载的话。）");
    println!("                  （仅支持4个值，Forge、Fabric、Quilt、NeoForge）");
    println!("                  （此参数与【-u | --url】参数互斥）");
    println!("  -mlv | --modLoaderVersion <输入模组加载器版本>");
    println!("                  （该选项从上个参数中获取，如果你填了上个参数，则这个参数必须紧跟在它后面。）");
    println!("                  （你必须提前了解该模组加载器和你想装的版本号。）");
    println!("                  （如果你只想下载最新版，你可以填入latest，会默认导入最新正式版进行下载。）");
    println!("                  （此参数与【-u | --url】参数互斥）");
    println!("  -gd | --gameDir <输入保存路径，有默认值>");
    println!("                  （该参数为可选参数，如果不填或填入default则为配置文件中的gameDir路径。）");
    println!("                  （此参数与【-u | --url】参数互斥）");
    println!("  -n | --name <输入保存名称，有默认值>");
    println!("                  （该参数为可选参数，如果不填或填入default则为默认名称。）");
    println!("                  （此参数与【-u | --url】参数互斥）");
    println!("  -t | --thread <输入最大内存>，有默认值=32\n");
    println!("该启动器目前暂不支持【--长参数=值】形式，仅支持【-短参数 值】或者【--长参数 值】")
}
fn launch_game(){
    //TODO: 按照默认设置启动游戏！
}

fn test() {
    let v = rust_lib::main_mod::get_file_bit(r"D:\Workspace\DelphiWork\DelphiWorkReset\LittleLimboLauncher\Win64\Release\LittleLimboLauncher.exe".to_string());
    println!("{:?}", v);
    println!("{}", true.to_string());
}
fn command_judge_launch(a: Vec<String>) {
    let mut root_dir = String::new();
    let mut version = String::new();
    let mut java_path = String::new();
    let mut account_user_name = String::new();
    let mut account_user_uuid = String::new();
    let mut window_width: usize = 0;
    let mut window_height = 0;
    let mut min_memory = 0;
    let mut max_memory = 0;
    let mut custom_info = String::new();
    let mut isolation = String::new();
    let mut additional_jvm = String::new();
    let mut additional_game = String::new();
    let mut i = 2;
    while i < a.len() {
        let s = a[i].clone();
        if s.eq("-gd") || s.eq("--gameDir") {
            root_dir = a.get(i + 1).expect("Cannot get game dir param!").clone();
            i += 1;
        } else if s.eq("-v") || s.eq("--version") {
            version = a.get(i + 1).expect("Cannot get version param!").clone();
            i += 1;
        } else if s.eq("-jp") || s.eq("--javaPath") {
            java_path = a.get(i + 1).expect("Cannot get java path param!").clone();
            i += 1;
        } else if s.eq("-a") || s.eq("--account") {
            if a.get(i + 1).expect("Cannot get account param!").clone().eq("Offline") {
                i += 1;
                account_user_name = a.get(i + 1).expect("Cannot get user name param!").clone();
                i += 1;
                if a.get(i + 1).expect("Cannot get user uuid param!").clone().eq("default") {
                    account_user_uuid = rust_lib::main_mod::generate_bukkit_uuid(account_user_name.as_str());
                }else{
                    account_user_uuid = a[i + 1].clone();
                }
            }
            i += 1;
        } else if s.eq("-w") || s.eq("--width") {
            window_width = a.get(i + 1).expect("Cannot get width param!").clone().parse::<usize>().expect("Cannot parse width param!");
            i += 1;
        } else if s.eq("-h") || s.eq("--height") {
            window_height = a.get(i + 1).expect("Cannot get height param!").clone().parse::<usize>().expect("Cannot parse height param!");
            i += 1;
        } else if s.eq("-nm") || s.eq("--minMemory") {
            min_memory = a.get(i + 1).expect("Cannot get height param!").clone().parse::<usize>().expect("Cannot parse height param!");
            i += 1;
        } else if s.eq("-xm") || s.eq("--maxMemory") {
            max_memory = a.get(i + 1).expect("Cannot get height param!").clone().parse::<usize>().expect("Cannot parse height param!");
            i += 1;
        } else if s.eq("-c") || s.eq("--customInfo") {
            custom_info = a.get(i + 1).expect("Cannot get custom info param!").clone();
            i += 1;
        } else if s.eq("-i") || s.eq("--isolation") {
            let g = a.get(i + 1).expect("Cannot get isolation param!").clone();
            if g.eq("false") {
                isolation = "false".to_string();
            } else if g.eq("true") {
                isolation = "true".to_string();
            } else {
                panic!("Cannot parse isolation param!");
            }
            i += 1;
        } else if s.eq("-as") || s.eq("--afterScript") {
            panic!("Not support Control!")
        } else if s.eq("-ps") || s.eq("--preScript") {
            panic!("Not support Control!")
        } else if s.eq("-aj") || s.eq("--additionalJVM") {
            additional_jvm = a.get(i + 1).expect("Cannot get additional JVM param").clone();
            i += 1;
        } else if s.eq("-ag") || s.eq("--additionalGame") {
            additional_game = a.get(i + 1).expect("Cannot get additional Game param").clone();
            i += 1;
        } else {
            panic!("Not support Argument!");
        }
        i += 1;
    }
    unsafe {
        root_dir = if root_dir.is_empty() {
            if version::CHOOSE_VERSION < 0 {
                panic!("game dir param cannot be empty!");
            }
            version::CURRENT_VERSION.clone()
        } else {
            root_dir
        };
        if version.is_empty() {
            if version::CHOOSE_VERSION_SEL < 0 {
                panic!("game dir param cannot be empty!");
            }
            version = version::CURRENT_VERSION_SEL.clone();
        } else {
            version = format!("{}\\versions\\{}", root_dir, version);
        }
        let game_dir = if isolation.eq("true") { version.clone() } else if isolation.eq("false") { root_dir.clone() } else {
            launcher::is_isolation(root_dir.clone(), version.clone()).clone()
        };
        java_path = if java_path.is_empty() {
            if launch::CHOOSE_JAVA < 0 {
                panic!("java path param cannot be empty!");
            }
            launch::CURRENT_JAVA.clone()
        }else{
            java_path
        };
        if account_user_name.is_empty() {
            panic!("user name param cannot be empty!");
        }
        if account_user_uuid.is_empty() {
            panic!("user uuid param cannot be empty!");
        }
        window_width = if window_width == 0 { launch::WINDOW_WIDTH } else { window_width };
        window_height = if window_height == 0 { launch::WINDOW_HEIGHT } else { window_height };
        min_memory = if min_memory == 0 { launch::MIN_MEMORY } else { min_memory };
        max_memory = if max_memory == 0 { launch::MAX_MEMORY } else { max_memory };
        custom_info = if custom_info.is_empty() { "Tank Launcher Module".to_string() } else { custom_info.clone() };
        additional_jvm = if additional_jvm.is_empty() { String::new() } else { additional_jvm.clone() };
        additional_game = if additional_game.is_empty() { String::new() } else { additional_game.clone() };
        let account = rust_lib::launcher_mod::LaunchAccount::new_offline(account_user_name.as_str(), account_user_uuid.as_str());
        let mut option = rust_lib::launcher_mod::LaunchOption::new(
                account, 
                java_path.as_str(), 
                root_dir.as_str(), 
                version.as_str(), 
                game_dir.as_str());
        option.set_window_width(window_width);
        option.set_window_height(window_height);
        option.set_custom_info(custom_info.as_str());
        option.set_min_memory(min_memory);
        option.set_max_memory(max_memory);
        option.set_additional_jvm(additional_jvm.as_str());
        option.set_additional_game(additional_game.as_str());
        launcher::start_launch(option);
    }
}
fn unsafe_init() {
    unsafe {
        let dir = dirs::data_dir().expect("Cannot get AppData dir!");
        let dir = dir.as_path().display().to_string();
        main_method::APP_DATA = dir.clone();
        let other_ini_path = format!("{}\\TankLauncherModule\\Other.ini", dir.clone());
        main_method::OTHER_INI.set_path(other_ini_path.as_str());
        let current_path = std::env::current_exe().expect("Cannot get current exe dir!");
        let current_path = current_path.parent().expect("Cannot get current exe dir!");
        let current_path = current_path.to_path_buf();
        main_method::CURRENT_DIR = current_path.to_string_lossy().to_string();
        let config_path = std::path::Path::new(current_path.as_path()).join("TankLauncherModule").join("configs");
        let tlm_ini_path = config_path.join("TankLauncherModule.ini");
        let tlm_path_str = tlm_ini_path.to_str().expect("Cannot get current exe dir!");
        main_method::TLM_INI.set_path(tlm_path_str);
        let mc_json_path = config_path.join("MCJSON.json");
        if !mc_json_path.exists(){
            let ver = serde_json::from_str::<serde_json::Value>("{\"mc\":[]}").unwrap();
            version::VERSION_JSON = serde_json::Value::Object(ver.as_object().unwrap().clone());
            let ver = serde_json::to_string_pretty(&ver).unwrap();
            rust_lib::main_mod::set_file(mc_json_path.to_str().unwrap(), ver);
        }else{
            version::CHOOSE_VERSION = main_method::TLM_INI.read_int("MC", "SelectMC", -1);
            let ver = rust_lib::main_mod::get_file(mc_json_path.to_str().unwrap()).expect("Config MCJSON.json is error!");
            let ver = serde_json::from_str::<serde_json::Value>(ver.as_str()).expect("Config MCJSON.json is error!");
            let ver = ver.as_object().expect("Config MCJSON.json is error!");
            if version::CHOOSE_VERSION != -1 {
                let current = ver.get("mc").expect("Config MCJSON.json is error!");
                let current = current.get(version::CHOOSE_VERSION as usize).expect("Config MCJSON.json is error!");
                let current = current.get("path").expect("Config MCJSON.json is error!");
                let current = current.as_str().expect("Config MCJSON.json is error!");
                version::CURRENT_VERSION = current.to_string();
            }
            version::VERSION_JSON = serde_json::Value::Object(ver.clone());
        }
        let mc_sel_path = config_path.join("MCSelJSON.json");
        if !mc_sel_path.exists(){
            let ver = serde_json::from_str::<serde_json::Value>("{\"mcsel\":[]}").unwrap();
            version::VERSION_SEL_JSON = serde_json::Value::Object(ver.as_object().unwrap().clone());
            let ver = serde_json::to_string_pretty(&ver).unwrap();
            rust_lib::main_mod::set_file(mc_sel_path.to_str().unwrap(), ver);
        }else{
            version::CHOOSE_VERSION_SEL = main_method::TLM_INI.read_int("MC", "SelectVer", -1);
            let ver = rust_lib::main_mod::get_file(mc_sel_path.to_str().unwrap()).expect("Config MCSelJSON.json is error!");
            let ver = serde_json::from_str::<serde_json::Value>(ver.as_str()).expect("Config MCSelJSON.json is error!");
            let ver = ver.as_object().expect("Config MCSelJSON.json is error!");
            if version::CHOOSE_VERSION_SEL != -1 {
                let current = ver.get("mcsel").expect("Config MCJSON.json is error!");
                let current = current.get(version::CHOOSE_VERSION_SEL as usize).expect("Config MCJSON.json is error!");
                let current = current.get("path").expect("Config MCJSON.json is error!");
                let current = current.as_str().expect("Config MCJSON.json is error!");
                version::CURRENT_VERSION_SEL = current.to_string();
            }
            version::VERSION_SEL_JSON = serde_json::Value::Object(ver.clone());
        }
        let java_path = config_path.join("JavaJSON.json");
        if !java_path.exists() {
            let java = serde_json::from_str::<serde_json::Value>("{\"java\":[]}").unwrap();
            launch::JAVA_JSON = serde_json::Value::Object(java.as_object().unwrap().clone());
            let java = serde_json::to_string_pretty(&java).unwrap();
            rust_lib::main_mod::set_file(java_path.to_str().unwrap(), java);
        } else {
            launch::CHOOSE_JAVA = main_method::TLM_INI.read_int("Java", "SelectJava", -1);
            let java = rust_lib::main_mod::get_file(java_path.to_str().unwrap()).expect("Config JavaJSON.json is error!");
            let java = serde_json::from_str::<serde_json::Value>(java.as_str()).expect("Config JavaJSON.json is error!");
            let java = java.as_object().expect("Config MCSelJSON.json is error!");
            if launch::CHOOSE_JAVA != -1 {
                let current = java.get("java").expect("Config JavaJSON.json is error!");
                let current = current.get(launch::CHOOSE_JAVA as usize).expect("Config JavaJSON.json is error!");
                let current_path = current.get("path").expect("Config JavaJSON.json is error!");
                let current_path = current_path.as_str().expect("Config JavaJSON.json is error!");
                launch::CURRENT_JAVA = current_path.to_string();
                let current_bits = current.get("bits").expect("Config JavaJSON.json is error!")
                                                .as_str().expect("Config JavaJSON.json is error!");
                launch::CURRENT_BITS = current_bits.to_string();
                let current_bits = current.get("version").expect("Config JavaJSON.json is error!")
                                            .as_str().expect("Config JavaJSON.json is error!");
                launch::CURRENT_VERSION = current_bits.to_string();
            }
            launch::JAVA_JSON = serde_json::Value::Object(java.clone());
        }
        let iso = main_method::TLM_INI.read_int("Version", "SelectIsolation", 4);
        if iso > 4 || iso < 1 {
            panic!("Config TankLauncherModule.ini SelectIsolation is error!");
        }
        version::IS_ISOLATION = iso;
        let ww = main_method::TLM_INI.read_int("Document", "WindowWidth", 854);
        if ww < 854 {
            panic!("Window Width is error!");
        }
        launch::WINDOW_WIDTH = ww as usize;
        let wh = main_method::TLM_INI.read_int("Document", "WindowHeight", 480);
        if wh < 480 {
            panic!("Window Height is error!");
        }
        launch::WINDOW_HEIGHT = wh as usize;
        let nm = main_method::TLM_INI.read_int("Document", "MinMemory", 256);
        if nm < 256 || nm > 1024 {
            panic!("Min Memory is error!");
        }
        launch::MIN_MEMORY = nm as usize;
        let xm = main_method::TLM_INI.read_int("Document", "MaxMemory", 4096);
        if xm < 1024 {
            panic!("Max Memory is error!");
        }
        launch::MAX_MEMORY = xm as usize;
    }
}
fn tank_launcher_module_test_main(){
    unsafe_init();
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 1 {
        if a[1].eq("-h") || a[1].eq("--help") {
            if a.len() > 2 {
                panic!("Argument is more than 2!");
            }
            output_help();
        }else if a[1].eq("-l") || a[1].eq("--launch") {
            command_judge_launch(a);
        }else if a[1].eq("-s") || a[1].eq("--setting") {

        }else if a[1].eq("-d") || a[1].eq("--download") {

        }else if a[1].eq("-ps") || a[1].eq("--printSetting") {

        }else if a[1].eq("-v") || a[1].eq("--version") {
            if a.len() > 2 {
                panic!("Argument is more then 2!");
            }
            output_tlm_version();
        }else{
            panic!("{} is Unknown Arguments!", a[1]);
        }
        return;
    }
    show_title();
    show_main_menu();
    loop {
        use std::io::Write;
        print!(">>> ");
        std::io::stdout().flush().expect("Cannot flush message!");
        let mut main_choice = String::new();
        std::io::stdin().read_line(&mut main_choice).expect("Cannot read stdin!");
        main_choice = main_choice.trim().to_string();
        if main_choice.eq("q") { return; }
        let conv = main_choice.parse::<i8>();
        if let Ok(t) = conv {
            match t {
                0 => {
                    output_help();
                },
                1 => {
                    test();
                },
                2 => {
                    launch_game();
                },
                3 => {
                    println!("暂未实现！");
                },
                4 => {
                    output_version();
                }
                5 => {
                    outout_launch();
                }
                6 => {
                    println!("暂未实现！");
                }
                7 => {
                    println!("暂未实现！");
                }
                8 => {
                    println!("暂未实现！");
                }
                9 => {
                    println!("暂未实现！");
                }
                10 => {
                    println!("暂未实现！");
                }
                _ => println!("{}", ansi_term::Color::Green.paint("请不要输入0-10以外的数字噢！")),
            }
        }else{
            println!("{}", ansi_term::Color::Green.paint("请不要输入数字以外的字符噢！"));
        }
    }
}
fn main(){
    tank_launcher_module_test_main();
}