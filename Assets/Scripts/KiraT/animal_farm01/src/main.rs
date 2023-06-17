pub mod animals;
use animals :: template_animal :: animal;

fn main() {

    animal_purchased();

}

fn animal_purchased() {

    let _fox : [&str; 5] = animal("Fox", "Luna", "Female", "8", "1");
    //let fox_slice = &fox[0..6];

    let _sheep : [&str; 5] = animal( "Sheep", "Alex" , "Male", "13", "10");


}

