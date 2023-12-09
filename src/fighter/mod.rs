pub mod common;

pub mod ryu;
pub mod ken;

pub fn install() {
    ryu::install();
    ken::install();
}