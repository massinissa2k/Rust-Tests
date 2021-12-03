mod sous_dossier;
mod tests;
//mod voiture;
use sous_dossier::voiture;

macro_rules! say_hello {
    () => {
        println!("Macro says hello");
    };
    ($name: expr) => {
        println!("Macro says hello {}", $name);
    };
    ($name: expr, $age: expr) => {
        println!("Macro says hello {} age: {}", $name, $age);
    };
}

fn main() {
    //tests::tests();
    struct Distance(isize);

    let distance = Distance(25);

    let Distance(longueur) = distance;
    println!("La distance est {}", longueur);

    let voiture = voiture::Voiture {
        name: String::from("Ma-Voiture"),
        portes: 5,
        roues: 4,
    };

    println!();
    println!();

    println!("Ma Voiture: ");
    println!(
        "   Ma voiture nom: {}, portes: {}, roues: {}",
        voiture.name, voiture.portes, voiture.roues
    );

    println!("Ma Voiture destructurée: ");
    let mut voiture = voiture::Voiture::new(String::from("Coucou-La-Voiture"));
    voiture.rename(String::from("Voiture renommée"));
    //let voiture::Voiture { name, .. } = voiture;
    println!("   Nom de Ma Voiture: {}", voiture.name);

    test_voiture_info(voiture);
    let n = "Massi";
    say_hello!(n);
    say_hello!(n, 40);
}

fn test_voiture_info<T: voiture::Vehicule>(vehicule: T) {
    println!("      Portes via trait: {}", vehicule.get_doors_number());
    println!("      Roues via trait: {}", vehicule.get_wheels_number());
}

#[test]
fn u_test() {
    assert_eq!(21, 21);
}

#[test]
#[should_panic]
fn u_test_2() {
    assert_eq!(21, 121);
}