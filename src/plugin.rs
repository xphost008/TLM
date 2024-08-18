use mlua::prelude::*;
pub fn load_lua_plugin(path: &str) -> LuaResult<()>{
    let l = Lua::new();
    let f = l.create_function(|_: &Lua, 
        (name, ll): (String, String)| -> LuaResult<i32> {
        println!("{}, {}", name, ll);
        return Ok(3);
    });
    l.globals().set("lua_test", f?)?;
    l.load(crate::rust_lib::main_mod::get_file(path).unwrap()).exec()?;
    Ok(())
}