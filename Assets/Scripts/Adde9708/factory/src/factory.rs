pub mod factory {

    use crate::armys::armys::army;
    use crate::weapons::weapons::weapons;

    pub fn factory() {
        /* Declare the army varible as a Vec containing i32's
        and assign it the army function to take return value from it */
        let army: Vec<i32> = army(50_000, 20);
        let mut armys: Vec<Vec<i32>> = Vec::new();
        /*Same as above, but with a Vec containing &str
          instead of i32's and assign it to the weapons function which takes
          an array of &str values
        */
        let weapon: Vec<&str> = weapons(["Rifle", "Sniper", "LMG", "Pistol"]);
        let mut weapons: Vec<Vec<&str>> = Vec::new();

        weapons.push(weapon);
        armys.push(army);
        // Just create a few new variables to keep track of the number of armys
        // and number of weapons and the weapons per army
        let num_armys: f32 = armys.len() as f32;
        let num_weapons: f32 = weapons.len() as f32;
        let weapons_per_army: f32 = num_weapons / num_armys as f32;
        //TODO: finish this loop
        for (i, army) in armys.iter_mut().enumerate() {
            let start: f32 = i as f32 * weapons_per_army as f32;
        }
    }
}
