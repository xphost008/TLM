use mlua::prelude::*;

pub fn load_lua_plugin(path: &str) -> LuaResult<()>{
    let l = Lua::new();
    let f = l.create_function(|_: &Lua, values: mlua::Variadic<String>| -> LuaResult<()> {
        println!("{}", values.join(", "));
        Ok(())
    });
    l.globals().set("print", f?)?;
    let c = crate::rust_lib::main_mod::get_file(path);
    if let Some(e) = c {
        l.load(e.clone()).exec()?;
    }else{
        println!("Err!")
    }
    Ok(())
}