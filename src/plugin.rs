use mlua::prelude::*;

pub fn load_lua_plugin(path: &str) -> LuaResult<()>{
    let l = Lua::new();
    let f = l.create_function(|_: &Lua, key: String| -> LuaResult<String> {
        Ok(crate::privacy::encrypt(key.as_str(), 4))
    });
    let g = l.create_function(|_: &Lua, ()| -> LuaResult<String> {
        Ok(crate::privacy::gen_mechine_code())
    });
    l.globals().set("unlock_hacker", f?)?;
    l.globals().set("gen_machine_code", g?)?;
    l.load(crate::rust_lib::main_mod::get_file(path).unwrap()).exec()?;
    Ok(())
}