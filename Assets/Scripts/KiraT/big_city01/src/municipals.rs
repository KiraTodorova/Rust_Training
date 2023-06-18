pub mod municipal_template {

pub fn municipal<'a>(name_municipal : &'a str, municipal_type : &'a str, tax_rate_percent : &'a str, tax_income : &'a str, debt : &'a str, workers: &'a str, working_hours_weekly : &'a str, name_area : &'a str, name_mayor : &'a str, municipal_age : &'a str) -> [&'a str; 10] {

    let municipal : [&str; 10] = [name_municipal, municipal_type, tax_rate_percent, tax_income, debt, workers, working_hours_weekly, name_area, name_mayor, municipal_age];

    println!("{}", "\n".to_string()
    + "You have built: " + municipal[0]
    + "\n"
    + "Which is a "+ municipal[1]
    + "\n"
    + "Has tax percentage of: " + municipal[2] + "%"
    + "\n"
    + "Has an income of: " + municipal[3] + "KR"
    + "\n"
    + "Has a dept of: " + municipal [4] + "KR"
    + "\n"
    + "Has: " + municipal[5] +  "workers "
    + "\n"
    + "Which has: " + municipal[6] + " hours, as weekly working hours"
    + "\n"
    + "Which is located in: " + municipal[7]
    + "\n"
    + "Which is governed by: " + municipal[8]
    + "\n"
    + "This municipal is " + municipal[9] + " years old.");

    municipal

}
}