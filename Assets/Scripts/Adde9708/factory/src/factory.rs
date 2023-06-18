pub mod factory {

    use crate::armys::armys::army;
    use crate::weapons::weapons::weapons;

    pub fn factory<T, F>()
    where
        T: Clone,
        F: Fn(&Vec<String>) -> Vec<String>,
    {
        /* Declare the army varible as a Vec containing i32's
        and assign it the army function to take return value from it */
        let army: Vec<i32> = army(50_000, 20);
        let mut armys: Vec<Vec<i32>> = Vec::new();
        /*Same as above, but with a Vec containing &str
          instead of i32's and assign it to the weapons function which takes
          an array of &str values
        */
        let weapon: Vec<String> = weapons([
            "Rifle".to_string(),
            "Sniper".to_string(),
            "LMG".to_string(),
            "Pistol".to_string(),
        ]);
        let mut weapons: Vec<Vec<String>> = Vec::new();

        weapons.push(weapon);
        armys.push(army);

        // Just create a few new variables to keep track of the number of armys
        // and number of weapons and the weapons per army

        let num_armys: f32 = armys.len() as f32;

        let num_weapons: f32 = weapons.len() as f32;

        let weapons_per_army: f32 = num_weapons / num_armys as f32;

        for (i, army) in armys.iter_mut().enumerate() {
            let start: f32 = i as f32 * weapons_per_army as f32;
            let end: f32 = if i as f32 == num_armys - 1 as f32 {
                num_weapons
            } else {
                (i + 1) as f32 * weapons_per_army as f32
            };
            let weapons_slice: &[Vec<String>] = &weapons[start as usize..end as usize];

            let army_weapons: Vec<String> = weapons_slice
                .iter()
                .flat_map(|weapons: &Vec<String>| weapons.iter().take(5).cloned())
                .collect();

            println!("Assigning weapons {:?} to army {:?}", army_weapons, i + 1);
        }
    }
}
