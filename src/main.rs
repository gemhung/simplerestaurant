mod config;

use config::settings;
fn main() {
    settings::init_settings();
    println!("Hello, world!");
}
