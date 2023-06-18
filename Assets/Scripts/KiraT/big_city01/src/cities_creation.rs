pub mod cities_creation{

    use crate :: cities :: city_template :: city;
    use crate :: municipals :: municipal_template :: municipal;
    use crate :: offices :: office_template :: office;
    use crate :: houses :: house_template :: house;


   pub fn stockholm_city_creation(){



        let _municipal : [&str; 10] = municipal("Stockholm Stad", "Huvudskommun", "32", "120.000", "500.000", "40", "40", "Stockholm, Ängeholm 52, Stokcholm 135 56", "Linda Ängabo", "80");
    
        let _city : [&str; 6] = city("Stockholm", "100.000", "70.000", "30.000", "65", "63");
    
        let _bibliary : [&str; 5] = office("Stokcholms Stads Bibliotek", "6", "35.000", "250", "1935");
    
        let _house : [&str; 5] = house("Radhus", "Ångbåtsvägen 3A", "5", "80", "1801");
    }

    pub fn malmoe_city_creation(){

        let _municipal : [&str; 10] = municipal("Malmö Komun", "Kommun", "28", "80.000", "700.000", "12", "35", "Malmö, Slagstavägen 91, Malmö 124 91", "Axel Håkan", "45");
    
        let _city : [&str; 6] = city("Malmö", "8.000", "4.500", "3.500", "71", "45");
    
        let _communal_second_hand : [&str; 5] = office("Malmö Stadsmission", "4", "18.000", "90", "1951");
    
        let _house : [&str; 5] = house("Lägenhet", "Falkskungegatan 13C", "1", "65", "2003");
    
    }


}