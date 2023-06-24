mod armys;
mod factory;
mod weapons;

use factory::factory::factory;

fn main() {
    println!("The Assembly Factory\n");
    factory();
}
