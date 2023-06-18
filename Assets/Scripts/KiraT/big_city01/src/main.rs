mod cities;

mod municipals;

mod offices;

mod houses;

mod cities_creation;

use cities_creation :: cities_creation :: stockholm_city_creation;
use cities_creation :: cities_creation :: malmoe_city_creation;

fn main() {

   start();

}


fn start(){

    stockholm_city_creation();
    
    malmoe_city_creation();

}