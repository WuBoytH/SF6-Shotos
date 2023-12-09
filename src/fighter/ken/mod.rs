mod acmd;
mod status;
mod agent_init;

mod hadoken;

pub fn install() {
    let agent = &mut smashline::Agent::new("ken");
    acmd::install(agent);
    status::install(agent);
    agent_init::install(agent);
    agent.install();

    hadoken::install();
}