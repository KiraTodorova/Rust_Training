mod armys;
mod factory;
mod weapons;

use factory::factory::factory;

fn main() {
    factory::<Vec<String>, fn(&Vec<String>) -> Vec<String>>();
}
