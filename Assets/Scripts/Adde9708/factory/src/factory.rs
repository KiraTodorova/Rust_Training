pub mod factory {

    use crate::armys::armys::create_army;
    use crate::weapons::weapons::create_weapons;

    pub fn factory() {
        let weapon_name: [&str; 4] = ["Rifles", "Snipers", "Pistols", "Light machine guns"];
        let mut weapons: [&str; 4] = [""; 4];
        create_weapons(weapon_name, &mut weapons);

        let soldiers_per_army: i32 = 10000;
        let vehicles_per_army: i32 = 20;

        let num_armies: i32 = 50;
        let num_weapons: i32 = 4;
        let weapons_per_army: f32 = num_weapons as f32 / num_armies as f32;

        for i in 0..num_armies {
            let start: i32 = (i as f32 * weapons_per_army) as i32;
            let end: i32 = if i == num_armies - 1 {
                num_weapons
            } else {
                ((i + 1_i32) as f32 * weapons_per_army) as i32
            };

            let army_name: String = format!("Army {} - Weapons {} to {}", i + 1, start + 1, end);
            create_army(soldiers_per_army, vehicles_per_army, &army_name);

            print!("Assigned weapons: ");
            for j in start..end {
                print!("{} ", weapons[j as usize]);
            }
            println!("\n");
        }
    }
}
