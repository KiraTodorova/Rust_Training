fn main() {
    println!("Hello, world!");
    luna_cheese();
    cheese_tax();
    adde_age();
    luna_age();
    kira_age();
    all_ages();
    
}

fn luna_cheese() {
    println!("Adde sucks!");
    println!("Luna is eating cheese!");
}

fn cheese_tax() {
    println!("Adde and Luna needs to pay the Cheese Tax to mii!");

}

fn adde_age() -> String {
    let a : String = "3".to_string(); 

    println!("{:?}","Adde is ".to_owned() + &a + " years old");

    a

}

fn luna_age() -> i32 {
    let a : i32 = 3;
    let b : i32 = 5;
    let c : i32 = 2;

   let d : i32 = a + b - c * a;

   println!("{:?}", "Luna is ".to_owned() + &d.to_string() + " years old");

    d


}

fn kira_age() -> i32 {

    let a : i32 = self::luna_age();
    let b : i32 = self::adde_age().parse().unwrap();

    let x : i32 = 5;
    let y : i32 = 0;
    let z : i32 = 2;

    let abc_xyz : i32 = a * b + x * z - y + -2;

    println!("{:?}", "Kira is: ".to_owned() +  &abc_xyz.to_string() + " years old");

    abc_xyz

}

fn all_ages() -> i32 {

    let a : i32 = self::luna_age();
    let b : i32 = self::adde_age().parse().unwrap();
    let c : i32 = self::kira_age();

    let d : i32 = a + b + c;

    println!("{:?}", "Everyone's age together is:  ".to_owned() + &d.to_string());

    d

}