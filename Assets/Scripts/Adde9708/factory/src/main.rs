mod armys;
mod factory;
mod weapons;

use factory::factory::factory;

fn main() {
    for _ in 0..50 * 50 {
        factory::<Vec<String>, fn(&Vec<String>) -> Vec<String>>();
    }
}
