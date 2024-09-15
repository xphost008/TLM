mod rust_lib;
mod launcher;
mod main_method;
mod read_ini;
mod version;
mod launch;
mod account;
mod plugin;
mod privacy;
mod download;

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
    println!("|        11. 加载插件                        |");
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
    println!("|        9. 移除Java                         |");
    println!("|        10. 设置自定义信息                  |");
    println!("|        11. 设置额外JVM参数                 |");
    println!("|        12. 设置额外Game参数                |");
    println!("|        q. 退出当前页                       |");
    println!("----------------------------------------------");
}
fn show_account() {
    println!("----------------------------------------------");
    println!("|        请输入对应的数字启动对应的功能      |");
    println!("|        0. 查看当前账号信息                 |");
    println!("|        1. 选择账号                         |");
    println!("|        2. 添加离线登录                     |");
    println!("|        3. 通过正版名称获取UUID             |");
    println!("|        4. 添加微软登录                     |");
    println!("|        5. 添加外置登录                     |");
    println!("|        6. 检测Authlib的更新（有更新则下载）|");
    println!("|        7. 移除账号                         |");
    println!("|        8. 刷新账号                         |");
    println!("|        q. 退出当前页                       |");
    println!("----------------------------------------------");
}
fn show_download() {
    println!("----------------------------------------------");
    println!("|        请输入对应的数字启动对应的功能      |");
    println!("|        0. 查看当前下载设置                 |");
    println!("|        1. 查看当前MC版本                   |");
    println!("|        2. 查看当前Forge版本                |");
    println!("|        3. 查看当前Fabric版本               |");
    println!("|        4. 查看当前Quilt版本                |");
    println!("|        5. 查看当前NeoForge版本             |");
    println!("|        6. 下载MC                           |");
    println!("|        7. 设置下载源                       |");
    println!("|        8. 设置最大并发量                   |");
    println!("|        9. 下载自定义文件                   |");
    println!("|        10. 下载手动安装包                  |");
    println!("|        q. 退出当前页                       |");
    println!("----------------------------------------------");
}
fn output_tlm_version() {
    println!("当前TLM版本：{}", rust_lib::some_const::LAUNCHER_VERSION);
}
fn output_download() {
    show_download();
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
                0 => download::check_download(),
                1 => download::get_minecraft(),
                2 => download::get_forge(),
                3 => download::get_fabric(),
                4 => download::get_quilt(),
                5 => download::get_neoforge(),
                6 => println!("暂未实现！"),
                7 => download::set_download_source(),
                8 => download::set_max_thread(),
                9 => println!("暂未实现！"),
                10 => println!("暂未实现！"),
                _ => println!("{}", ansi_term::Color::Red.paint("请不要输入0-10以外的数字噢！")),
            }
        }else{
            println!("{}", ansi_term::Color::Red.paint("请不要输入数字以外的字符噢！"));
        }
    }
}
fn output_account() {
    show_account();
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
                0 => account::check_account(),
                1 => account::choose_account(),
                2 => account::add_offline(),
                3 => account::get_legal_uuid(),
                4 => account::add_microsoft(),
                5 => account::add_thirdparty(),
                6 => println!("暂未实现！"),
                7 => account::remove_account(),
                8 => account::refresh_account(),
                _ => println!("{}", ansi_term::Color::Red.paint("请不要输入0-8以外的数字噢！")),
            }
        }else{
            println!("{}", ansi_term::Color::Red.paint("请不要输入数字以外的字符噢！"));
        }
    }
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
                0 => launch::check_launch(),
                1 => launch::set_window_width(),
                2 => launch::set_window_height(),
                3 => launch::set_max_memory(),
                4 => launch::set_min_memory(),
                5 => launch::add_java(),
                6 => launch::choose_java(),
                7 => println!("暂未实现！"),
                8 => launch::do_scan_java(),
                9 => launch::remove_java(),
                10 => launch::set_custom_info(),
                11 => launch::set_additional_jvm(),
                12 => launch::set_additional_game(),
                _ => println!("{}", ansi_term::Color::Red.paint("请不要输入0-12以外的数字噢！")),
            }
        }else{
            println!("{}", ansi_term::Color::Red.paint("请不要输入数字以外的字符噢！"));
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
                0 => version::check_version(),
                1 => version::choose_version(),
                2 => version::choose_root(),
                3 => version::add_directory(),
                4 => version::set_isolation(),
                5 => println!("暂未实现！"),
                6 => version::rename_root(),
                7 => version::remove_root(),
                8 => version::rename_version(),
                9 => version::remove_version(),
                10 => println!("暂未实现！"),
                11 => println!("暂未实现！"),
                _ => println!("{}", ansi_term::Color::Red.paint("请不要输入0-11以外的数字噢！")),
            }
        }else{
            println!("{}", ansi_term::Color::Red.paint("请不要输入数字以外的字符噢！"));
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
    println!("                  （如果目录不存在则会输出回显【gameDir Not Found】）");
    println!("  -v | --version <设置/启动游戏版本，有默认值=-1>");
    println!("                  （仅支持[值]为：【游戏版本名称、数字】）");
    println!("                  （如果是游戏版本名称，则请输入完整的/versions/<版本名称>中的版本名称字段）");
    println!("                  （如果版本不存在则会输出回显【version Not Found】）");
    println!("  -jp | --javaPath <设置/启动游戏Java路径>，有默认值=-1");
    println!("                  （此处可以填入【Java路径、数字】）");
    println!("                  （如果是路径，则请输入完整的Java路径）");
    println!("                  （如果Java不存在则会输出回显【javaPath Not Found】）");
    println!("  -a | --account <设置/启动游戏账号，有默认值=-1>");
    println!("                  （仅支持[值]为：【Offline、Microsoft、ThirdParty、数字】）");
    println!("                  （如果是Offline，则后面跟玩家用户名和UUID。如果UUID不想手动填可以填default）");
    println!("                  （如果是Microsoft，则会直接要求重新登录一次，按照要求登录即可！）");
    println!("                  （如果是ThirdParty，则后面要求填入【1. 服务器、2. 账号、3. 密码、4. clientToken（如果有则填，没有则填default）】！）");
    println!("                  （其中，在登录外置登录时，会提示需要输入一个值作为你要登录的角色。请注意！");
    println!("                  （如果账号格式不对，则会输出回显【account is appear!】）");
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
    command_judge_launch(vec![], true);
}


// use std::cell::RefCell;
// thread_local! {
//     static X: RefCell<String> = RefCell::new(String::from("Test"));
// }
fn test(){
    let a = privacy::gen_machine_code();
    println!("{}", a);
    let b = privacy::encrypt(a.as_str(), 5);
    println!("{}", b);
    // rust_lib::some_var::DOWNLOAD_SOURCE.set(2);
    // let a = rust_lib::download_mod::get_neoforge_version("1.20.1");
    // println!("{}", serde_json::to_string_pretty(&a.unwrap()).unwrap());
    // let l = mlua::Lua::new();
    // l.load("print(\"你好，中国！\")").exec().unwrap();
    // println!("{}", rust_lib::main_mod::get_file("D:\\mc.txt").unwrap());
    // let x = X.with_borrow_mut(|e| {
    //     e
    // });
    // println!("{}", x);
    // x = format!("{}", "111").as_mut();
    // println!("{}", X.with_borrow(|e| e.clone()));
    // let b = X.with_borrow(|e| e.clone());
    // println!("{}", b);
    // let b = X.with_borrow(|e| e.clone());
    // println!("{}", b);
    // X.set("Meow".to_string());
    // let b = X.with_borrow(|e| e.clone());
    // println!("{}", b);
    // unsafe { rust_lib::some_var::DOWNLOAD_SOURCE = 1; }
    // let s = rust_lib::download_mod::get_forge_versions("1.10.2").unwrap();
    // println!("{}", serde_json::to_string_pretty(&s).unwrap());
    // use quick_xml::events::Event;
    // use quick_xml::reader::Reader;

    // let xml = String::from_utf8(rust_lib::account_mod::UrlMethod::new("https://maven.minecraftforge.net/net/minecraftforge/forge/maven-metadata.xml").get_default().unwrap()).unwrap();
    // let mut reader = Reader::from_str(xml.as_str());
    // reader.config_mut().trim_text(true);
    // let mut versioning = false;
    // let mut versions = false;
    // let mut version = false;
    // let mut count = 0;
    // let mut buf = Vec::new();
    // loop {
    //     match reader.read_event_into(&mut buf) {
    //         Ok(Event::Start(e)) => {
    //             match e.name().as_ref() {
    //                 b"versioning" => versioning = true,
    //                 b"versions" if versioning => versions = true,
    //                 b"version" if versions => version = true,
    //                 _ => ()
    //             }
    //         }
    //         Ok(Event::End(e)) => {
    //             match e.name().as_ref() {
    //                 b"versioning" => versioning = false,
    //                 b"versions" => versions = false,
    //                 b"version" => version = false,
    //                 _ => ()
    //             }
    //         }
    //         Ok(Event::Text(e)) => {
    //             if versioning && versions && version {
    //                 let text = e.unescape().unwrap().into_owned();
    //                 println!("{}", text);
    //             }
    //         }
    //         Ok(Event::Eof) => break,
    //         _ => ()
    //     }
    // }
}
fn load_plugin() {
    let ph = main_method::CURRENT_DIR.get().expect("Cannot read Current Dir Value");
    let pp = std::path::Path::new(ph.as_str());
    let pp = pp.join("TankLauncherModule").join("plugins");
    if !pp.exists() || pp.exists() && pp.is_file() {
        std::fs::create_dir_all(pp.clone()).expect("Cannot create dir!");
    }
    let mut res: Vec<String> = Vec::new();
    let walk = walkdir::WalkDir::new(pp.clone()).min_depth(1).max_depth(1);
    for i in walk.into_iter().filter_map(|e| e.ok()) {
        let ext = i.path().extension().unwrap().to_str().unwrap().to_string();
        if ext.eq("lua") {
            res.push(format!("{}", i.file_name().to_str().unwrap().to_string()));
        }
    }
    if res.len() <= 0 {
        println!("{}", ansi_term::Color::Yellow.paint("你目前还没有写任何一个插件噢！请尝试添加一个插件吧！"));
        return;
    }
    println!("----------------------------------------------");
    println!("请输入你要选择的插件：");
    for i in 0..res.len() {
        println!("{}. {}", i + 1, res[i]);
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
    let j = res[input_num].clone();
    let path = pp.join(j).to_str().unwrap().to_string();
    let res = plugin::load_lua_plugin(path.as_str());
    if let Err(e) = res {
        println!("{}", ansi_term::Color::Red.paint(format!("{:?}", e)))
    }
}
fn command_judge_launch(a: Vec<String>, is_panic: bool) {
    fn struct_to_launch(jiegou: account::AccountStruct) -> rust_lib::launcher_mod::LaunchAccount {
        if jiegou.get_online() == 0 {
            rust_lib::launcher_mod::LaunchAccount::new_offline(
                jiegou.get_name().as_str(),
                jiegou.get_uuid().as_str())
        } else if jiegou.get_online() == 1 {
            rust_lib::launcher_mod::LaunchAccount::new_microsoft(
                jiegou.get_name().as_str(), 
                jiegou.get_uuid().as_str(), 
                jiegou.get_access_token().as_str())
        } else if jiegou.get_online() == 2 {
            rust_lib::launcher_mod::LaunchAccount::new_thirdparty(
                jiegou.get_name().as_str(), 
                jiegou.get_uuid().as_str(), 
                jiegou.get_access_token().as_str(), 
                jiegou.get_base().as_str(), 
                jiegou.get_url().as_str())
        } else {
            panic!("Cannot solve AccountStruct online mode!");
        }
    }
    let mut root_dir = String::new();
    let mut version = String::new();
    let mut java_path = String::new();
    let mut account = account::AccountStruct::new();
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
                let mut account_user_uuid = String::new();
                i += 1;
                let account_user_name = a.get(i + 1).expect("Cannot get user name param!").clone();
                i += 1;
                if a.get(i + 1).expect("Cannot get user uuid param!").clone().eq("default") {
                    account_user_uuid.push_str(rust_lib::main_mod::generate_bukkit_uuid(account_user_name.as_str()).as_str());
                }else{
                    account_user_uuid.push_str(a[i + 1].clone().as_str());
                }
                account.set_account(account_user_name.as_str(), account_user_uuid.as_str(), "", "", "", "", "", 0);
            }else if a.get(i + 1).expect("Cannot get account param!").clone().eq("Microsoft"){
                let account_result = account::set_microsoft(privacy::MS_CLIENT_ID);
                account.copy(account_result);
            }else if a.get(i + 1).expect("Cannot get account param!").clone().eq("ThirdParty") {
                i += 1;
                let account_server = a.get(i + 1).expect("Cannot get user name param!").clone();
                i += 1;
                let account_username = a.get(i + 1).expect("Cannot get user name param!").clone();
                i += 1;
                let account_password = a.get(i + 1).expect("Cannot get user name param!").clone();
                i += 1;
                let account_client_token = a.get(i + 1).expect("Cannot get user name param!");
                let account_client_token = if account_client_token.eq("default") { String::new() } else { account_client_token.clone() };
                let account_result = account::set_thirdparty(account_server.as_str(), account_username.as_str(), account_password.as_str(), account_client_token.as_str());
                account.copy(account_result);
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
    if account.is_not_init() {
        if account::CHOOSE_ACCOUNT.with_borrow(|e| e.clone()) < 0 {
            if is_panic {
                panic!("account param connot be empty!")
            }else{
                println!("账号不能为空！");
                return;
            }
        }
        account.copy(account::CURRENT_ACCOUNT.with_borrow(|e| e.clone()));
    }
    root_dir = if root_dir.is_empty() {
        if version::CHOOSE_VERSION.with_borrow(|e| e.clone()) < 0 {
            if is_panic {
                panic!("game dir param cannot be empty!");
            }else{
                println!("游戏目录不能为空！");
                return;
            }
        }
        version::CURRENT_VERSION.with_borrow(|e| e.clone())
    } else {
        root_dir
    };
    version = format!("{}\\versions\\{}", root_dir.clone(), if version.is_empty() {
        if version::CHOOSE_VERSION_SEL.with_borrow(|e| e.clone()) < 0 {
            if is_panic {
                panic!("version name param cannot be empty!");
            }else{
                println!("游戏版本不能为空！");
                return;
            }
        }
        version::CURRENT_VERSION_SEL.with_borrow(|e| e.clone())
    } else {
        version
    });
    let game_dir = if isolation.eq("true") { version.clone() } else if isolation.eq("false") { root_dir.clone() } else {
        launcher::is_isolation(root_dir.clone(), version.clone()).clone()
    };
    java_path = if java_path.is_empty() {
        if launch::CHOOSE_JAVA.with_borrow(|e| e.clone()) < 0 {
            if is_panic {
                panic!("java path param cannot be empty!");
            }else{
                println!("Java路径不能为空！");
                return;
            }
        }
        launch::CURRENT_JAVA.with_borrow(|e| e.clone())
    }else{
        java_path
    };
    if account.get_name().is_empty() || account.get_uuid().is_empty() {
        if is_panic {
            panic!("account param cannot be empty!");
        }else{
            println!("账号名称或UUID不能为空！");
            return;
        }
    }
    window_width = if window_width == 0 { launch::WINDOW_WIDTH.with_borrow(|e| e.clone()) } else { window_width };
    window_height = if window_height == 0 { launch::WINDOW_HEIGHT.with_borrow(|e| e.clone()) } else { window_height };
    min_memory = if min_memory == 0 { launch::MIN_MEMORY.with_borrow(|e| e.clone()) } else { min_memory };
    max_memory = if max_memory == 0 { launch::MAX_MEMORY.with_borrow(|e| e.clone()) } else { max_memory };
    custom_info = if custom_info.is_empty() { launch::CUSTOM_INFO.with_borrow(|e| e.clone()) } else { custom_info.clone() };
    additional_jvm = if additional_jvm.is_empty() { launch::ADDITIONAL_JVM.with_borrow(|e| e.clone()) } else { additional_jvm.clone() };
    additional_game = if additional_game.is_empty() { launch::ADDITIONAL_GAME.with_borrow(|e| e.clone()) } else { additional_game.clone() };
    let account = struct_to_launch(account);
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
fn unsafe_init() {
    let appdata_dir = dirs::data_dir().expect("Cannot get AppData dir!").as_path().display().to_string();
    main_method::APP_DATA.set(appdata_dir.clone()).expect("Cannot set AppData Value");
    let appdata_path = std::path::Path::new(appdata_dir.as_str()).join("TankLauncherModule");
    let other_ini_path = appdata_path.join("Other.ini").to_string_lossy().to_string();
    main_method::OTHER_INI.set(read_ini::IniFile::new(other_ini_path.as_str())).expect("Cannot set Other ini Value");
    let current_path = std::env::current_exe().expect("Cannot get current exe dir!").parent().expect("Cannot get current exe dir!").to_path_buf();
    main_method::CURRENT_DIR.set(current_path.to_string_lossy().to_string()).expect("Cannot set Current Dir Value");
    let config_path = std::path::Path::new(current_path.as_path()).join("TankLauncherModule").join("configs");
    let tlm_ini_path = config_path.join("TankLauncherModule.ini");
    let tlm_path_str = tlm_ini_path.to_str().expect("Cannot get current exe dir!");
    main_method::TLM_INI.set(read_ini::IniFile::new(tlm_path_str)).expect("Cannot set TLM ini Value");
    let c_other_ini = main_method::OTHER_INI.get().expect("Cannot read Other ini file");
    let c_tlm_ini = main_method::TLM_INI.get().expect("Cannot read Other ini file");
    let authlib_path = appdata_path.join("Authlib-Injector.jar");
    if authlib_path.exists() && authlib_path.is_file() {
        rust_lib::some_var::AUTHLIB_PATH.set(authlib_path.to_string_lossy().to_string());
    }
    let account_json_path = appdata_path.join("AccountJSON.json");
    if !account_json_path.exists() {
        let acc = serde_json::from_str::<serde_json::Value>("{\"account\":[]}").unwrap();
        account::ACCOUNT_JSON.set(serde_json::Value::Object(acc.as_object().unwrap().clone()));
        let acc = serde_json::to_string_pretty(&acc).unwrap();
        rust_lib::main_mod::set_file(account_json_path.to_str().unwrap(), acc);
    } else {
        let acc = rust_lib::main_mod::get_file(account_json_path.to_str().unwrap()).expect("Config AccountJSON.json is error!");
        let acc = serde_json::from_str::<serde_json::Value>(acc.as_str()).expect("Config AccountJSON.json is error!");
        let acc = acc.as_object().expect("Config AccountJSON.json is error!");
        let anum = c_other_ini.read_int("Account", "SelectAccount", -1);
        account::CHOOSE_ACCOUNT.set(anum);
        if anum >= 0 {
            let current = acc["account"][anum as usize].clone();
            account::CURRENT_ACCOUNT.with_borrow_mut(|e| {
                let atype = current["type"]
                    .as_str()
                    .expect("Config AccountJSON.json is error!");
                if atype.eq("offline") {
                    e.set_account(current["name"]
                        .as_str()
                        .expect("Config AccountJSON.json is error!"), current["uuid"]
                        .as_str()
                        .expect("Config AccountJSON.json is error!"), "", "", "", "", "", 0);
                } else if atype.eq("microsoft") {
                    e.set_account(current["name"]
                        .as_str()
                        .expect("Config AccountJSON.json is error!"), current["uuid"]
                        .as_str()
                        .expect("Config AccountJSON.json is error!"), current["access_token"]
                        .as_str()
                        .expect("Config AccountJSON.json is error!"), "", "", "", "", 1);
                } else if atype.eq("thirdparty") {
                    e.set_account(current["name"]
                        .as_str()
                        .expect("Config AccountJSON.json is error!"), current["uuid"]
                        .as_str()
                        .expect("Config AccountJSON.json is error!"), current["access_token"]
                        .as_str()
                        .expect("Config AccountJSON.json is error!"), "", current["client_token"]
                        .as_str()
                        .expect("Config AccountJSON.json is error!"), current["server"]
                        .as_str()
                        .expect("Config AccountJSON.json is error!"), current["base_code"]
                        .as_str()
                        .expect("Config AccountJSON.json is error!"), 2);
                } else {
                    panic!("Cannot solve AccountJSON type value")
                }
            });
        }
        account::ACCOUNT_JSON.set(serde_json::Value::Object(acc.clone()));
    }
    let mc_json_path = config_path.join("MCJSON.json");
    if !mc_json_path.exists(){
        let ver = serde_json::from_str::<serde_json::Value>("{\"mc\":[]}").unwrap();
        version::VERSION_JSON.set(serde_json::Value::Object(ver.as_object().unwrap().clone()));
        let ver = serde_json::to_string_pretty(&ver).unwrap();
        rust_lib::main_mod::set_file(mc_json_path.to_str().unwrap(), ver);
    }else{
        let ver = rust_lib::main_mod::get_file(mc_json_path.to_str().unwrap()).expect("Config MCJSON.json is error!");
        let ver = serde_json::from_str::<serde_json::Value>(ver.as_str()).expect("Config MCJSON.json is error!");
        let cmc = c_tlm_ini.read_int("MC", "SelectMC", -1);
        version::CHOOSE_VERSION.set(cmc);
        if cmc != -1 {
            let current = ver["mc"][cmc as usize]["path"].as_str().expect("Config MCJSON.json is error!");
            version::CURRENT_VERSION.set(current.to_string());
        }
        version::VERSION_JSON.set(ver.clone());
    }
    let mc_sel_path = config_path.join("MCSelJSON.json");
    if !mc_sel_path.exists(){
        let ver = serde_json::from_str::<serde_json::Value>("{\"mcsel\":[]}").unwrap();
        version::VERSION_SEL_JSON.set(serde_json::Value::Object(ver.as_object().unwrap().clone()));
        let ver = serde_json::to_string_pretty(&ver).unwrap();
        rust_lib::main_mod::set_file(mc_sel_path.to_str().unwrap(), ver);
    }else{
        let ver = rust_lib::main_mod::get_file(mc_sel_path.to_str().unwrap()).expect("Config MCSelJSON.json is error!");
        let ver = serde_json::from_str::<serde_json::Value>(ver.as_str()).expect("Config MCSelJSON.json is error!");
        let cmv = c_tlm_ini.read_int("MC", "SelectVer", -1);
        version::CHOOSE_VERSION_SEL.set(cmv);
        if cmv != -1 {
            let current = ver["mcsel"][cmv as usize]["path"].as_str().expect("Config MCJSON.json is error!");
            version::CURRENT_VERSION_SEL.set(current.to_string());
        }
        version::VERSION_SEL_JSON.set(ver.clone());
    }
    version::reload_version();
    let java_path = config_path.join("JavaJSON.json");
    if !java_path.exists() {
        let java = serde_json::from_str::<serde_json::Value>("{\"java\":[]}").unwrap();
        launch::JAVA_JSON.set(serde_json::Value::Object(java.as_object().unwrap().clone()));
        let java = serde_json::to_string_pretty(&java).unwrap();
        rust_lib::main_mod::set_file(java_path.to_str().unwrap(), java);
    } else {
        let java = rust_lib::main_mod::get_file(java_path.to_str().unwrap()).expect("Config JavaJSON.json is error!");
        let java = serde_json::from_str::<serde_json::Value>(java.as_str()).expect("Config JavaJSON.json is error!");
        let cj = c_tlm_ini.read_int("Java", "SelectJava", -1);
        launch::CHOOSE_JAVA.set(cj);
        if cj >= 0 {
            let current = java["java"][cj as usize].as_object().expect("Config JavaJSON.json is error!");
            let current_path = current.get("path").expect("Config JavaJSON.json is error!")
                                            .as_str().expect("Config JavaJSON.json is error!");
            launch::CURRENT_JAVA.set(current_path.to_string());
            let current_bits = current.get("bits").expect("Config JavaJSON.json is error!")
                                            .as_str().expect("Config JavaJSON.json is error!");
            launch::CURRENT_BITS.set(current_bits.to_string());
            let current_bits = current.get("version").expect("Config JavaJSON.json is error!")
                                        .as_str().expect("Config JavaJSON.json is error!");
            launch::CURRENT_VERSION.set(current_bits.to_string());
        }
        launch::JAVA_JSON.set(java.clone());
    }
    let iso = c_tlm_ini.read_int("Version", "SelectIsolation", 4);
    if iso > 4 || iso < 1 {
        panic!("Config TankLauncherModule.ini SelectIsolation is error!");
    }
    version::IS_ISOLATION.set(iso);
    let ww = c_tlm_ini.read_int("Document", "WindowWidth", 854);
    if ww < 854 {
        panic!("Window Width is error!");
    }
    launch::WINDOW_WIDTH.set(ww as usize);
    let wh = c_tlm_ini.read_int("Document", "WindowHeight", 480);
    if wh < 480 {
        panic!("Window Height is error!");
    }
    launch::WINDOW_HEIGHT.set(wh as usize);
    let nm = c_tlm_ini.read_int("Document", "MinMemory", 256);
    if nm < 256 || nm > 1024 {
        panic!("Min Memory is error!");
    }
    launch::MIN_MEMORY.set(nm as usize);
    let xm = c_tlm_ini.read_int("Document", "MaxMemory", 4096);
    if xm < 1024 {
        panic!("Max Memory is error!");
    }
    launch::MAX_MEMORY.set(xm as usize);
    let ci = c_tlm_ini.read_str("Version", "CustomInfo", "");
    launch::CUSTOM_INFO.set(ci.clone());
    if ci.is_empty() {
        launch::CUSTOM_INFO.set(String::from("Tank Launcher Module"));
    }
    launch::ADDITIONAL_JVM.set(c_tlm_ini.read_str("Version", "AdditionalJVM", ""));
    launch::ADDITIONAL_GAME.set(c_tlm_ini.read_str("Version", "AdditionalGame", ""));
    let tt = c_tlm_ini.read_int("Version", "ThreadBiggest", 64);
    if tt <= 0 || tt > 256 {
        panic!("Thread Biggest is error!");
    }
    rust_lib::some_var::BIGGEST_THREAD.set(tt);
    let su = c_tlm_ini.read_int("Version", "SelectDownloadSource", 1);
    if su < 1 || su > 2 {
        panic!("Download Source is error!");
    }
    rust_lib::some_var::DOWNLOAD_SOURCE.set(su);
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
            command_judge_launch(a, true);
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
                0 => output_help(),
                1 => test(),
                2 => launch_game(),
                3 => println!("暂未实现！"),
                4 => output_version(),
                5 => outout_launch(),
                6 => println!("暂未实现！"),
                7 => println!("暂未实现！"),
                8 => output_account(),
                9 => output_download(),
                10 => println!("暂未实现！"),
                11 => load_plugin(),
                _ => println!("{}", ansi_term::Color::Red.paint("请不要输入0-11以外的数字噢！")),
            }
        }else{
            println!("{}", ansi_term::Color::Red.paint("请不要输入数字以外的字符噢！"));
        }
    }
}
fn main(){
    tank_launcher_module_test_main();
}