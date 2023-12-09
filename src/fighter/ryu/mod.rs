mod acmd;
mod status;
mod agent_init;
mod vtable_hook;
pub mod helper;

mod hadoken;

pub fn install() {
    let agent = &mut smashline::Agent::new("ryu");
    acmd::install(agent);
    status::install(agent);
    agent_init::install(agent);
    vtable_hook::install();
    agent.install();

    hadoken::install();
}