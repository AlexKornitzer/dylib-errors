extern crate env_logger;
extern crate shared;

fn main() {
    env_logger::init();
    let _a = shared::A {};
    let _b = shared::library::Test::new();
}
