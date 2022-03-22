use anyhow::Context;
use mlua::Lua;

pub async fn lua_async_run<T>(lua_state: Lua, content: T) -> anyhow::Result<()>
where
    T: AsRef<[u8]>,
{
    lua_state
        .load(&content)
        .set_name("thrpg")?
        .exec()
        .context("Don't run")
}
