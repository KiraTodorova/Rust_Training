pub mod armys {
    /*
     Create an army with just i32's and a string reference,
     instead of an array converted to a vec
    */
    pub fn create_army(num_soldiers: i32, num_vehicles: i32, army_name: &str) {
        println!("Creating army: {}", army_name);
        println!("Number of soldiers: {}", num_soldiers);
        println!("Number of vehicles: {}", num_vehicles);
    }
}
