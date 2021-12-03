pub trait Vehicule {
    fn get_wheels_number(&self) -> u8;
    fn get_doors_number(&self) -> u8;
}

pub struct Voiture {
    pub name: String,
    pub roues: u8,
    pub portes: u8,
}

impl Voiture {
    pub fn new(nom: String) -> Voiture {
        Voiture {
            name: nom,
            roues: 3,
            portes: 1,
        }
    }

    pub fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }
}

impl Vehicule for Voiture {
    fn get_wheels_number(&self) -> u8 {
        self.roues
    }
    fn get_doors_number(&self) -> u8 {
        self.portes
    }
}

impl Drop for Voiture {
    fn drop(&mut self) {
        println!("La voiture {} est detruite !", self.name);
    }
}
