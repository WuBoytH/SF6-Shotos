pub mod acmd_imports {
    pub use {
        smash::{
            lua2cpp::*,
            hash40,
            phx::*,
            app::{lua_bind::*, sv_animcmd::*, *},
            lib::lua_const::*
        },
        smashline::*,
        smash_script::*,
        custom_var::*,
        wubor_utils::{wua_bind::*, vars::*}
    };
}

pub mod status_imports {
    pub use {
        smash::{
            lua2cpp::*,
            hash40,
            phx::*,
            app::{lua_bind::*, *},
            lib::{lua_const::*, L2CValue}
        },
        smashline::*,
        smash_script::*,
        custom_var::*,
        wubor_utils::{wua_bind::*, vars::*, table_const::*}
    };
}