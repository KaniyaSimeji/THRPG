use mlua::prelude::{LuaResult, LuaTable};
use mlua::Lua;

fn he(_: &Lua, name: String) -> LuaResult<()> {
    println!("hello {}", name);
    Ok(())
}

#[mlua::lua_module]
fn thrpg(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("he", lua.create_function(he)?)?;
    Ok(exports)
}
