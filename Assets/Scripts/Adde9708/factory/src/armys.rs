pub mod armys {

    pub fn army(soldier: i32, vehicles: i32) -> Vec<i32> {
        let army: [i32; 2] = [soldier, vehicles];
        army.to_vec()
    }
}
