#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod fighter;

mod system;
mod custom_vars;

mod imports;

#[skyline::main(name = "sf6_shotos")]
pub fn main() {
    fighter::install();

    system::install();
    custom_vars::install();
}