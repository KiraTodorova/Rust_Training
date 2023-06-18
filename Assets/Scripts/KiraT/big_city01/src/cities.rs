pub mod city_template{

pub fn city<'a>(city_name : &'a str, population : &'a str, workers : &'a str, jobless : &'a str, average_age : &'a str, death_rate : &'a str) -> [&'a str; 6] {
    
    let population : [&str; 6] = [city_name, population, workers, jobless, average_age, death_rate];

    println!("{}", "\n".to_string() + "\n"
    
    + "You have created: " + population[0]
    + "\n"
    + "Which has a population of: " + population[1]
    + "\n"
    + "Which " + population[2] + "are workers"
    + "\n"
    + "And " + population[3] + "are jpbless"
    + "\n"
    + "The average age is: " + population[4]
    + "\n"
    + "The death rate is: " + population[5]);

    population
}

}