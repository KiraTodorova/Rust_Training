pub mod weapons {
    /*  Pretty much the same thing as the create army function
        but just use two arrays containing 4 string references
    */
    pub fn create_weapons<'a>(weapon_name: [&'a str; 4], weapons: &mut [&'a str; 4]) {
        for i in 0..4 {
            weapons[i] = weapon_name[i];
        }
    }
}
