//All lines in Rust needs ";" on the ending of every line to explicitdly end the line.
//i32 is an Integer of 32-bits.
//f32 is a Float of 32-bits.

fn main() {
    println!("Hello, world!");
    luna_cheese();
    cheese_tax();
    adde_age();
    luna_age();
    kira_age();
    all_ages();
    
}

//Functions are created with: fn name_of_function()
//All functions are implicit private, but can be made explicitdly public with: pub fn name_of_function(){}.
//All functions with Data have a "{" to open and a "}" to close.

fn luna_cheese() {
    println!("Adde sucks!");
    println!("Luna is eating cheese!");
}

fn cheese_tax() {
    println!("Adde and Luna needs to pay the Cheese Tax to mii!");

}

//Functions with a return type are create with: fn name_of_function() -> data_type.
//The data_type stated to be returned MUST be returned.
//The data_type that IS returned can be returned implicitdly or explicitdly. If b = "1", return type String or &str. Then b at the end of Data will be the return type. Or by explicitdly add a "3".to_string().

//"{:?}", ... or simply "{}", ... is a formating denotation for the terminal for debugging purposes only used by println();
//"{:?}" ... is a denotation to specify and overly clear for debuggin purposes, meaning it will print the exact object.
//"{:x}" or "{:X}" ... is a denotation for Hexa-decimal objects, so if small x in denotation, then small x in its counter part and vice-versa.

//A variable is implicitdly immutable/constant, but can be explicitdly mutable with: let mut name_of_object : data_type = data;

fn adde_age() -> String {
    let a : String = "3".to_string(); 

    println!("{:?}","Adde is ".to_owned() + &a + " years old");

    a

}


//To create an object with a specific data_type: let name_of_object : data_type = data.
//i32 is a data_type of an integer of 32-bits.
//i64 is a data_type of an integer of 64-bits.
//i128 is a data_type of an integer of 128-bits.

//i in i32 means it's signed, which means it has a chance to overflow and it can hold negative numbers.

//.to_owned() is a function to create an ownership of a data_type.
//An unowned data_type is only borrowable from another data_type and/or is either dead or out-of-scope with: &.

fn luna_age() -> i32 {
    let a : i32 = 3;
    let b : i32 = 5;
    let c : i32 = 2;

   let d : i32 = a + b - c * a;

   println!("{:?}", "Luna is ".to_owned() + &d.to_string() + " years old");

    d


}

//To create a function that takes the return type of another function, which the new function has a return ype of i32, it must need to create a new object: let object_name : data_ytpe = self::function_name;
//.parse() is a function used to parse a String-Integer like "3" to any Integer data_type., Cannot be done if it is not a number.
//.unwrap() is a function to skip Error-Handling and it Errors occurs, then the program panics and crashes and quits it.

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