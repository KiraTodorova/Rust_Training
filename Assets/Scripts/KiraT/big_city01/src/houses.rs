pub mod house_template {

pub fn house<'a>(house_type : &'a str, house_address : &'a str, habitants : &'a str, house_area : &'a str, house_built_year : &'a str) -> [&'a str; 5] {

    let house : [&str; 5] = [house_type, house_address, habitants, house_area, house_built_year];

    println!("{}", "\n".to_string() + "\n"
    + "You have built a: " + house[0]
    + "\n"
    + "Which is in: " + house[1]
    + "\n"
    + "It has " + house[2] + " habitants"
    + "\n"
    + "The house has an area of " + house[3] + "m^2"
    + "\n"
    + "The house was built in " + house[4]
    + "\n"
    + "\n"
    );

    house

}


}