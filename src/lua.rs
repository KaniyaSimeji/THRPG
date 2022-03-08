use anyhow::Context;
use mlua::Lua;
pub async fn lua_run(lua_state: Lua, content: &str) -> anyhow::Result<()> {
    lua_state
        .load(content)
        .set_name("thrpg")?
        .exec()
        .context("run miss")
}
