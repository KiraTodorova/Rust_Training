pub mod office_template {


pub fn office<'a>(company_name : &'a str, number_workers : &'a str, salary : &'a str, building_area : &'a str, company_founded : &'a str) -> [&'a str; 5] {

    let office : [&str; 5] = [company_name, number_workers, salary, building_area, company_founded];

    println!("{}", "\n".to_string()
    + "\n"
    + "You have built a: " + office[0]
    + "\n"
    + "Which has " + office[1] + " of workers"
    + "\n"
    + "The salary is " + office[2] + " KR"
    + "\n"
    + "The building area is " + office[3] + "m^2"
    + "\n"
    + "The company was founded in " + office[4]);

    office

}


}