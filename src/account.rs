use clipboard::ClipboardProvider;

pub static mut ACCOUNT_JSON: serde_json::Value = serde_json::Value::Null;
pub static mut CHOOSE_ACCOUNT: i32 = -1;
pub static mut CURRENT_ACCOUNT: AccountStruct = AccountStruct::new();

static mut TEMP_ACC: AccountStruct = AccountStruct::new();

#[derive(Clone)]
pub struct AccountStruct {
    name: String,
    uuid: String,
    access_token: String,
    refresh_token: String,
    client_token: String,
    url: String,
    base: String,
    online: i32
}
impl AccountStruct {
    pub const fn new() -> Self {
        Self {
            name: String::new(),
            uuid: String::new(),
            access_token: String::new(),
            refresh_token: String::new(),
            client_token: String::new(),
            url: String::new(),
            base: String::new(),
            online: 0,
        }
    }
    pub fn set_account(&mut self, name: &str, uuid: &str, access_token: &str, refresh_token: &str, client_token: &str, url: &str, base: &str, online: i32) {
        self.name = name.to_string();
        self.uuid = uuid.to_string();
        self.access_token = access_token.to_string();
        self.refresh_token = refresh_token.to_string();
        self.client_token = client_token.to_string();
        self.url = url.to_string();
        self.base = base.to_string();
        self.online = online;
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }
    pub fn get_access_token(&self) -> String {
        self.access_token.clone()
    }
    pub fn get_refresh_token(&self) -> String {
        self.refresh_token.clone()
    }
    pub fn get_client_token(&self) -> String {
        self.client_token.clone()
    }
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
    pub fn get_base(&self) -> String {
        self.base.clone()
    }
    pub fn get_online(&self) -> i32 {
        self.online
    }
    pub fn is_not_init(&self) -> bool {
        self.name.is_empty() || self.uuid.is_empty()
    }
    pub fn copy(&mut self, copy: AccountStruct) {
        self.name = copy.get_name();
        self.uuid = copy.get_uuid();
        self.access_token = copy.get_access_token();
        self.refresh_token = copy.get_refresh_token();
        self.url = copy.get_url();
        self.base = copy.get_base();
        self.online = copy.get_online();
    }
}
pub fn check_account() {
    unsafe {
        let name = CURRENT_ACCOUNT.get_name();
        let uuid = CURRENT_ACCOUNT.get_uuid();
        if name.is_empty() || uuid.is_empty() {
            println!("{}", ansi_term::Color::Yellow.paint("你还暂未选择任意账号噢！请选择一个再试试吧！"))
        }
        println!("您当前选择的账号名称是：\n{}\n账号UUID是：\n{}", name, uuid);
    }
}
fn sr_microsoft(client_id: &str, refresh_token: &str) -> AccountStruct {
    //使用tokio执行异步程序，但是阻塞了主线程。
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let login = crate::rust_lib::account_mod::AccountLogin::new_ms(client_id);
        let s = login.refresh_microsoft(refresh_token.to_string()).await;
        match s {
            Ok(e) => {
                unsafe {
                    TEMP_ACC.set_account(e.get_name().as_str(), e.get_uuid().as_str(), e.get_access_token().as_str(), e.get_refresh_token().as_str(), "", "", "", 1);
                }
            },
            Err(e) => {
                match e {
                    -3 => {
                        println!("登录超时（15分钟未完成登录），请重试！");
                        return;
                    },
                    -4 => {
                        println!("刷新密钥也过期了，请尝试重新登录一次该账号！");
                        return;
                    }
                    -5 => {
                        println!("在进行xbox登录时出现了错误，可能是没挂vβn的原因。");
                        return;
                    },
                    -6 => {
                        println!("在进行xsts登录时出现了错误，可能是没挂vβn的原因。");
                        return;
                    },
                    -7 => {
                        println!("在进行xsts登录时，由于该账户没有xbox账号，你可能需要自己注册一个。");
                        return;
                    },
                    -8 => {
                        println!("在进行xsts登录时，由于该国家/禁止被禁止，无法登录。");
                        return;
                    },
                    -9 => {
                        println!("该账号需要成人验证（韩国）。");
                        return;
                    },
                    -10 => {
                        println!("该账号设置未满18周岁，需要成人将该账户添加到家庭组中。");
                        return;
                    },
                    -11 => {
                        println!("你请求的xbox usercode与xsts usercode二者不一致，请重新尝试！");
                        return;
                    },
                    -12 => {
                        println!("在进行mc登录时出现了错误，可能是没挂vβn的原因。");
                        return;
                    },
                    -13 => {
                        println!("该账号暂未购买mc，请重新尝试！");
                        return;
                    }
                    _ => {
                        println!("出现了未知错误，请立即反馈给作者！错误代码：{}", e);
                        return;
                    }
                }
            }
        }
    });
    unsafe {
        return TEMP_ACC.clone();
    }
}
pub fn remove_account() {
    unsafe {
        let acc_obj = ACCOUNT_JSON["account"].as_array_mut().expect("JSON Parse Error!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..acc_obj.len() {
            let j = acc_obj[i].clone();
            let n = j["name"].as_str().expect("JSON Parse Error!");
            let o = j["type"].as_str().expect("JSON Parse Error!");
            let o = if o.eq("offline") { "（离线）" } else if o.eq("microsoft") { "（微软）" } else if o.eq("thirdparty") { "（外置）" } else { panic!("Cannot solve AccountJSON type value!") };
            res.push(format!("{}. {} {}", i + 1, o, n));
        }
        if res.len() == 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还没有添加任何一个账号噢！请去添加一个再来！"));
            return;
        }
        println!("----------------------------------------------");
        println!("请输入你要移除的账号：");
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
        if CHOOSE_ACCOUNT > input_num as i32 {
            CHOOSE_ACCOUNT -= 1;
        } else if CHOOSE_ACCOUNT == input_num as i32 {
            CHOOSE_ACCOUNT = -1;
            CURRENT_ACCOUNT.set_account("", "", "", "", "", "", "", 0);
        }
        crate::main_method::OTHER_INI.write_str("Account", "SelectAccount", CHOOSE_ACCOUNT.to_string().as_str());
        acc_obj.remove(input_num);
    }
    save_account();
    println!("{}", ansi_term::Color::Green.paint("移除成功！"));
}
pub fn set_microsoft(client_id: &str) -> AccountStruct {
    //使用tokio执行异步程序，但是阻塞了主线程。
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let login = crate::rust_lib::account_mod::AccountLogin::new_ms(client_id);
        let (user_code, device_code) = login.get_user_code().await.unwrap();
        println!("请复制你的用户代码，并将其粘贴到浏览器上：{}", user_code);
        let mut cb: clipboard::ClipboardContext = clipboard::ClipboardProvider::new().unwrap();
        cb.set_contents(user_code.to_owned()).unwrap();
        std::process::Command::new("explorer.exe").arg("https://www.microsoft.com/link").spawn().expect("Some Error appear!");
        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
            let s = login.login_microsoft(device_code.clone()).await;
            match s {
                Ok(e) => {
                    unsafe {
                        TEMP_ACC.set_account(e.get_name().as_str(), e.get_uuid().as_str(), e.get_access_token().as_str(), e.get_refresh_token().as_str(), "", "", "", 1);
                    }
                    break;
                },
                Err(e) => {
                    match e {
                        // -1错误是在获取用户代码时出现的错误，这里暂时不用管。
                        // -2错误是暂未完成登录，重新开始一次循环。因此不用捕获。
                        -2 => {
                            continue;
                        }
                        -3 => {
                            println!("{}", ansi_term::Color::Red.paint("登录超时（15分钟未完成登录），请重试！"));
                            break;
                        },  
                        // -4错误是刷新账号时出现的错误，这里不用捕获。
                        -5 => {
                            println!("{}", ansi_term::Color::Red.paint("在进行xbox登录时出现了错误，可能是没挂vβn的原因。"));
                            break;
                        },
                        -6 => {
                            println!("{}", ansi_term::Color::Red.paint("在进行xsts登录时出现了错误，可能是没挂vβn的原因。"));
                            break;
                        },
                        -7 => {
                            println!("{}", ansi_term::Color::Red.paint("在进行xsts登录时，由于该账户没有xbox账号，你可能需要自己注册一个。"));
                            break;
                        },
                        -8 => {
                            println!("{}", ansi_term::Color::Red.paint("在进行xsts登录时，由于该国家/禁止被禁止，无法登录。"));
                            break;
                        },
                        -9 => {
                            println!("{}", ansi_term::Color::Red.paint("该账号需要成人验证（韩国）。"));
                            break;
                        },
                        -10 => {
                            println!("{}", ansi_term::Color::Red.paint("该账号设置未满18周岁，需要成人将该账户添加到家庭组中。"));
                            break;
                        },
                        -11 => {
                            println!("{}", ansi_term::Color::Red.paint("你请求的xbox usercode与xsts usercode二者不一致，请重新尝试！"));
                            break;
                        },
                        -12 => {
                            println!("{}", ansi_term::Color::Red.paint("在进行mc登录时出现了错误，可能是没挂vβn的原因。"));
                            break;
                        },
                        -13 => {
                            println!("{}", ansi_term::Color::Red.paint("该账号暂未购买mc，请重新尝试！"));
                            break;
                        }
                        _ => {
                            println!("{}{}", ansi_term::Color::Red.paint("出现了未知错误，请立即反馈给作者！错误代码："), e);
                            break;
                        }
                    }
                }
            }
        }
    });
    unsafe {
        return TEMP_ACC.clone();
    }
}
fn sr_thirdparty(server: &str, access_token: &str, client_token: &str) -> AccountStruct {
    //使用tokio执行异步程序，但是阻塞了主线程。
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let login = crate::rust_lib::account_mod::AccountLogin::new_tp(server);
        let s = login.refresh_thirdparty(access_token.to_string(), client_token.to_string()).await;
        match s {
            Ok(e) => {
                unsafe {
                    TEMP_ACC.set_account(e.get_name().as_str(), e.get_uuid().as_str(), e.get_access_token().as_str(), "", e.get_client_token().as_str(), server, e.get_base().as_str(), 2);
                }
            },
            Err(e) => {
                match e {
                    -16 => {
                        println!("{}", ansi_term::Color::Red.paint("无法获取网址，请确保你已经连接网络。"));
                        return;
                    }
                    -17 => {
                        println!("{}", ansi_term::Color::Red.paint("令牌已失效，请重新登录一次！"));
                        return;
                    }
                    _ => {
                        println!("{}{}", ansi_term::Color::Red.paint("出现了未知错误，请立即反馈给作者！错误代码："), e);
                        return;
                    }
                }
            }
        }
    });
    unsafe {
        return TEMP_ACC.clone();
    }
}
pub fn set_thirdparty(server: &str, username: &str, password: &str, client_token: &str) -> AccountStruct {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let login = crate::rust_lib::account_mod::AccountLogin::new_tp(server);
        let res = login.login_thirdparty(String::from(username), String::from(password), String::from(client_token)).await;
        match res {
            Ok(e) => {
                let mut res: Vec<String> = Vec::new();
                for i in 0..e.len() {
                    res.push(format!("{}. {}", i + 1, e[i].get_name()));
                }
                if res.len() == 0 {
                    println!("{}", ansi_term::Color::Yellow.paint("你的第三方登录暂未选择任意皮肤噢！请选择一个再来！"));
                    return;
                }
                println!("----------------------------------------------");
                println!("请输入你要登录的第三方账号：");
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
                let res_acc = e[input_num].clone();
                unsafe {
                    TEMP_ACC.set_account(res_acc.get_name().as_str(), res_acc.get_uuid().as_str(), res_acc.get_access_token().as_str(), "", res_acc.get_client_token().as_str(), server, res_acc.get_base().as_str(), 2)
                }
            },
            Err(e) => {
                match e {
                    -14 => {
                        println!("{}", ansi_term::Color::Red.paint("无法获取第三方元数据网址，请确保你已连接网络。"));
                        return;
                    }
                    -15 => {
                        println!("{}", ansi_term::Color::Red.paint("账号或密码错误，或者是你登录次数过多而被暂时禁止登录，请稍后重试！"));
                        return;
                    }
                    _ => {
                        println!("{}{}", ansi_term::Color::Red.paint("出现了未知错误，请立即反馈给作者！错误代码："), e);
                        return;
                    }
                }
            }
        }
    });
    unsafe {
        return TEMP_ACC.clone();
    }
}
pub fn add_microsoft() {
    //使用tokio执行异步程序，但是阻塞了主线程。
    let acc = set_microsoft(crate::privacy::MS_CLIENT_ID);
    unsafe {
        let acc_obj = ACCOUNT_JSON.get_mut("account").expect("JSON Parse Error!");
        let acc_obj = acc_obj.as_array_mut().expect("JSON Parse Error!");
        let mut push_obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        let push_obj = push_obj.as_object_mut().unwrap();
        push_obj.insert(String::from("type"), serde_json::Value::String(String::from("microsoft")));
        push_obj.insert(String::from("name"), serde_json::Value::String(acc.get_name().clone()));
        push_obj.insert(String::from("uuid"), serde_json::Value::String(acc.get_uuid().clone()));
        push_obj.insert(String::from("access_token"), serde_json::Value::String(acc.get_access_token().clone()));
        push_obj.insert(String::from("refresh_token"), serde_json::Value::String(acc.get_refresh_token().clone()));
        acc_obj.push(serde_json::Value::Object(push_obj.clone()));
    }
    save_account();
    println!("{}", ansi_term::Color::Green.paint("添加成功！"));
}
pub fn refresh_account() {
    unsafe {
        let acc_obj = ACCOUNT_JSON["account"].as_array().expect("JSON Parse Error!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..acc_obj.len() {
            let j = acc_obj[i].clone();
            let n = j["name"].as_str().expect("JSON Parse Error!");
            let o = j["type"].as_str().expect("JSON Parse Error!");
            let o = if o.eq("offline") { "（离线）" } else if o.eq("microsoft") { "（微软）" } else if o.eq("thirdparty") { "（外置）" } else { panic!("Cannot solve AccountJSON type value!") };
            res.push(format!("{}. {} {}", i + 1, o, n));
        }
        if res.len() == 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还没有添加任何一个账号噢！请去添加一个再来！"));
            return;
        }
        println!("----------------------------------------------");
        println!("请输入你要刷新的账号：");
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
        let acc = acc_obj[input_num].clone();
        let tp = acc["type"].as_str().expect("JSON Parse Error!");
        if tp.eq("offline") {
            println!("{}", ansi_term::Color::Yellow.paint("离线模式不用刷新噢！"));
            return;
        }else if tp.eq("microsoft") {
            let re = acc.get("refresh_token").expect("JSON Parse Error!").as_str().expect("JSON Parse Error!");
            let rj = sr_microsoft(crate::privacy::MS_CLIENT_ID, re);
            let acc_obj = ACCOUNT_JSON.get_mut("account").expect("JSON Parse Error!");
            let acc_obj = acc_obj.as_array_mut().expect("JSON Parse Error!");
            let acc_obj = acc_obj.get_mut(input_num).expect("JSON Parse Error!").as_object_mut().expect("JSON Parse Error!");
            acc_obj.remove("name");
            acc_obj.remove("uuid");
            acc_obj.remove("access_token");
            acc_obj.remove("refresh_token");
            acc_obj.insert(String::from("name"), serde_json::Value::String(rj.get_name()));
            acc_obj.insert(String::from("uuid"), serde_json::Value::String(rj.get_uuid()));
            acc_obj.insert(String::from("access_token"), serde_json::Value::String(rj.get_access_token()));
            acc_obj.insert(String::from("refresh_token"), serde_json::Value::String(rj.get_refresh_token()));
        }else if tp.eq("thirdparty") {
            let rs = acc.get("server").expect("JSON Parse Error!").as_str().expect("JSON Parse Error!");
            let re = acc.get("access_token").expect("JSON Parse Error!").as_str().expect("JSON Parse Error!");
            let rc = acc.get("client_token").expect("JSON Parse Error!").as_str().expect("JSON Parse Error!");
            let rj = sr_thirdparty(rs, re, rc);
            let acc_obj = ACCOUNT_JSON.get_mut("account").expect("JSON Parse Error!");
            let acc_obj = acc_obj.as_array_mut().expect("JSON Parse Error!");
            let acc_obj = acc_obj.get_mut(input_num).expect("JSON Parse Error!").as_object_mut().expect("JSON Parse Error!");
            acc_obj.remove("access_token");
            acc_obj.insert(String::from("access_token"), serde_json::Value::String(rj.get_access_token()));
        }else{
            panic!("Cannot solve AccountJSON type value!")
        }
    }
    save_account();
    println!("{}", ansi_term::Color::Green.paint("刷新成功！"));
}
pub fn add_thirdparty() {
    println!("请输入你的外置登录服务器地址（填写示例：https://littleskin.cn/api/yggdrasil，必须以api/yggdrasil结尾并且末尾不能有/符号）：");
    let mut input_ser = String::new();
    std::io::stdin().read_line(&mut input_ser).expect("Cannot read stdin!");
    input_ser = input_ser.trim().to_string();
    if input_ser.is_empty() {
        println!("{}", ansi_term::Color::Red.paint("外置登录服务器不能为空，请重新输入！"));
        return;
    }
    println!("请输入你的外置登录账号（一般是邮箱）：");
    let mut input_acc = String::new();
    std::io::stdin().read_line(&mut input_acc).expect("Cannot read stdin!");
    input_acc = input_acc.trim().to_string();
    let regu = regex::Regex::new("^[A-Za-z0-9]+([-._][A-Za-z0-9]+)*@[A-Za-z0-9]+(-[A-Za-z0-9]+)*(.[A-Za-z]{2,6}|[A-Za-z]{2,4}.[A-Za-z]{2,3})$").unwrap();
    if !regu.is_match(input_acc.as_str()) {
        println!("{}", ansi_term::Color::Red.paint("外置登录账号不符合规范，请重新输入！"));
        return;
    }
    println!("请输入你的外置登录密码（不会显示）：");
    let mut input_pas = rpassword::read_password().expect("Cannot read stdin!");
    input_pas = input_pas.trim().to_string();
    if input_pas.is_empty() {
        println!("{}", ansi_term::Color::Red.paint("外置登录密码不能为空，请重新输入！"));
        return;
    }
    println!("请输入你的外置登录客户端密钥（如果没有可以不填，一般是标准的UUID形式）");
    let mut input_clt = String::new();
    std::io::stdin().read_line(&mut input_clt).expect("Cannot read stdin!");
    input_clt = input_clt.trim().to_string();
    let mut clt = String::new();
    if !input_clt.is_empty() {
        clt = input_clt.clone();
    }
    let acc = set_thirdparty(input_ser.as_str(), input_acc.as_str(), input_pas.as_str(), clt.as_str());
    unsafe {
        let acc_obj = ACCOUNT_JSON.get_mut("account").expect("JSON Parse Error!");
        let acc_obj = acc_obj.as_array_mut().expect("JSON Parse Error!");
        let mut push_obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        let push_obj = push_obj.as_object_mut().unwrap();
        push_obj.insert(String::from("type"), serde_json::Value::String(String::from("thirdparty")));
        push_obj.insert(String::from("server"), serde_json::Value::String(input_ser.clone()));
        push_obj.insert(String::from("name"), serde_json::Value::String(acc.get_name().clone()));
        push_obj.insert(String::from("uuid"), serde_json::Value::String(acc.get_uuid().clone()));
        push_obj.insert(String::from("access_token"), serde_json::Value::String(acc.get_access_token().clone()));
        push_obj.insert(String::from("client_token"), serde_json::Value::String(acc.get_client_token().clone()));
        push_obj.insert(String::from("base_code"), serde_json::Value::String(acc.get_base().clone()));
        acc_obj.push(serde_json::Value::Object(push_obj.clone()));
    }
    save_account();
    println!("{}", ansi_term::Color::Green.paint("添加成功！"));
}
pub fn choose_account() {
    unsafe {
        let acc_obj = ACCOUNT_JSON["account"].as_array().expect("JSON Parse Error!");
        let mut res: Vec<String> = Vec::new();
        for i in 0..acc_obj.len() {
            let j = acc_obj[i].clone();
            let n = j["name"].as_str().expect("JSON Parse Error!");
            let o = j["type"].as_str().expect("JSON Parse Error!");
            let o = if o.eq("offline") { "（离线）" } else if o.eq("microsoft") { "（微软）" } else if o.eq("thirdparty") { "（外置）" } else { panic!("Cannot solve AccountJSON type value!") };
            res.push(format!("{}. {} {}", i + 1, o, n));
        }
        if res.len() == 0 {
            println!("{}", ansi_term::Color::Yellow.paint("你还没有添加任何一个账号噢！请去添加一个再来！"));
            return;
        }
        println!("----------------------------------------------");
        println!("请输入你要选择的账号：");
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
        crate::main_method::OTHER_INI.write_str("Account", "SelectAccount", input_num.to_string().as_str());
        CHOOSE_ACCOUNT = input_num as i32;
        let o = acc_obj[input_num].as_object().unwrap();
        let t = o["type"].as_str().unwrap();
        if t.eq("offline") {
            CURRENT_ACCOUNT.set_account(o["name"]
                .as_str()
                .expect("Parse JSON Error!"), o["uuid"]
                .as_str()
                .expect("Parse JSON Error!"), "", "", "", "", "", 0);
        } else if t.eq("microsoft") {
            CURRENT_ACCOUNT.set_account(o["name"]
                .as_str()
                .expect("Parse JSON Error!"), o["uuid"]
                .as_str()
                .expect("Parse JSON Error!"), o["access_token"]
                .as_str()
                .expect("Parse JSON Error!"), "", "", "", "", 1);
        } else if t.eq("thirdparty") {
            CURRENT_ACCOUNT.set_account(o["name"]
                .as_str()
                .expect("Parse JSON Error!"), o["uuid"]
                .as_str()
                .expect("Parse JSON Error!"), o["access_token"]
                .as_str()
                .expect("Parse JSON Error!"), "", o["client_token"]
                .as_str()
                .expect("Parse JSON Error!"), o["server"]
                .as_str()
                .expect("Parse JSON Error!"), o["base_code"]
                .as_str()
                .expect("Parse JSON Error!"), 2);
        } else {
            panic!("Cannot solve AccountJSON type value!")
        }
    }
    println!("{}", ansi_term::Color::Green.paint("选择成功！"));
}
pub fn add_offline() {
    println!("请输入你的离线登录用户名：");
    let mut input_name = String::new();
    std::io::stdin().read_line(&mut input_name).expect("Cannot read stdin!");
    input_name = input_name.trim().to_string();
    let regu = regex::Regex::new("^[a-zA-Z0-9]{3,16}$").unwrap();
    if !regu.is_match(input_name.as_str()) {
        println!("{}", ansi_term::Color::Red.paint("离线模式用户名输入不规范，请重新输入！"));
        return;
    }
    println!("请输入你的离线登录UUID（如果没有可以留空）");
    let mut input_uuid = String::new();
    std::io::stdin().read_line(&mut input_uuid).expect("Cannot read stdin!");
    input_uuid = input_uuid.trim().to_string();
    if input_uuid.is_empty() {
        input_uuid = crate::rust_lib::main_mod::generate_bukkit_uuid(input_name.as_str());
    }
    let regu = regex::Regex::new("^[0-9a-f]{32}$").unwrap();
    if !regu.is_match(input_uuid.as_str()) {
        println!("{}", ansi_term::Color::Red.paint("离线模式UUID输入不规范，请重新输入！"));
        return;
    }
    unsafe {
        let acc_obj = ACCOUNT_JSON.get_mut("account").expect("JSON Parse Error!");
        let acc_obj = acc_obj.as_array_mut().expect("JSON Parse Error!");
        let mut push_obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        let push_obj = push_obj.as_object_mut().unwrap();
        push_obj.insert(String::from("type"), serde_json::Value::String(String::from("offline")));
        push_obj.insert(String::from("name"), serde_json::Value::String(input_name.clone()));
        push_obj.insert(String::from("uuid"), serde_json::Value::String(input_uuid.clone()));
        acc_obj.push(serde_json::Value::Object(push_obj.clone()));
    }
    save_account();
    println!("{}", ansi_term::Color::Green.paint("添加成功！"));
}
pub fn get_legal_uuid() {
    println!("请输入你的正版用户名：");
    let mut input_name = String::new();
    std::io::stdin().read_line(&mut input_name).expect("Cannot read stdin!");
    input_name = input_name.trim().to_string();
    let regu = regex::Regex::new("^[a-zA-Z0-9]{3,16}$").unwrap();
    if !regu.is_match(input_name.as_str()) {
        println!("{}", ansi_term::Color::Red.paint("离线模式用户名输入不规范，请重新输入！"));
        return;
    }
    println!("正在获取正版UUID，请确保您已接入网络。");
    let res = crate::rust_lib::main_mod::name_to_uuid(input_name.as_str());
    if let Some(e) = res {
        println!("{}您的正版UUID是：{}", ansi_term::Color::Green.paint("获取成功！"), e);
    }else{
        println!("{}", ansi_term::Color::Red.paint("获取失败，请确保您已连接网络。"));
    }
}
pub fn save_account() {
    unsafe {
        crate::rust_lib::main_mod::set_file(
            format!("{}\\TankLauncherModule\\AccountJSON.json", crate::main_method::APP_DATA).as_str(), 
            serde_json::to_string_pretty(&ACCOUNT_JSON.clone()).unwrap());
    }
}