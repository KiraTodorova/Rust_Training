pub mod factory {

    use crate::armys::armys::create_army;
    use crate::weapons::weapons::create_weapons;

    fn assigned_weapons(start: i32, end: i32, weapons: [&str; 4]) {
        print!("Assigned weapons: ");
        for j in start..end {
            print!("{} ", weapons[j as usize]);
            println!("\n");
        }
    }

    pub fn factory() {
        /*
           Create the weapons by creating the array of weapons to pass into
           the function and then call the function
        */
        let weapon_name: [&str; 4] = ["Rifles", "Snipers", "Pistols", "Light machine guns"];
        let mut weapons: [&str; 4] = [""; 4];
        create_weapons(weapon_name, &mut weapons);

        // Set some needed variables
        let soldiers_per_army: i32 = 10000;
        let vehicles_per_army: i32 = 20;
        let num_armies: i32 = 50;
        let num_weapons: i32 = 4;
        let weapons_per_army: f32 = num_weapons as f32 / num_armies as f32;

        // Loop until it reaches 50 created armies
        for i in 0..num_armies {
            // Create indexes for the upcoming for loop that assigns weapons
            let start: i32 = (i as f32 * weapons_per_army) as i32;
            let end: i32 = if i == num_armies - 1_i32 {
                num_weapons
            } else {
                ((i + 1_i32) as f32 * weapons_per_army) as i32
            };

            // Create an army name for all the created armies
            let army_name: String =
                format!("Army {} - Weapons {} to {}", i + 1_i32, start + 1_i32, end);

            create_army(soldiers_per_army, vehicles_per_army, &army_name);

            // Print the assigned weapons
            assigned_weapons(start, end, weapons);
        }
    }
}
