pub mod template_animal {


pub fn animal<'a>(animal_type : &'a str, name : &'a str, gender : &'a str, age : &'a str , number_purchased : &'a str) -> [&'a str;5] {
    
    let animal :  [&str; 5] = [animal_type, name, gender, &age, &number_purchased];

    println!("{}" , "Purchased completed! You bought a ".to_string() + animal[0] + " with this info: " 
            + "\n" + "name: " + animal[1]
            + "\n" + "gender: " + animal[2] 
            + "\n" + "age: " + animal[3]
            + "\n" + "no. purchased: " + animal[4]
            + "\n");
    
    animal
    }
}