pub mod weapons {

    pub fn create_weapons<'a>(weapon_name: [&'a str; 4], weapons: &mut [&'a str; 4]) {
        for i in 0..4 {
            weapons[i] = weapon_name[i];
        }
    }
}
