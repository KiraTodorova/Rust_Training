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
        let num_armys: usize = armys.len();
        let num_weapons: usize = weapons.len();
        let weapons_per_army: usize = num_weapons / num_armys;
        //TODO: finish this loop
        for (i, army) in armys.iter_mut().enumerate() {
            let start = i * weapons_per_army;
        }
    }
}
