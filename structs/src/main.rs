use std::fmt;
struct House {
    material: Material,
    residents: i32,
    address: String,
}
impl House {
    pub fn new(mat: Material, res: i32, adrs: &str) -> Self {
        let house = Self {
            material: mat,
            residents: res,
            address: String::from(adrs),
        };

        println!("Built new house:\n{house}");
        return house;
    }

    pub fn add_resident(&mut self) {
        self.residents += 1;
        println!("Added resident to the House:\n{self}");
    }

    pub fn change_material(&mut self, material: Option<Material>) {
        self.material = material.unwrap_or(self.material);
        println!("Changed material of the House:\n{self}")
    }
}
#[derive(Clone, Copy)]
enum Material {
    Brick,
    Concrete,
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Material::Brick => write!(f, "Brick"),
            Material::Concrete => write!(f, "Concrete"),
        }
    }
}

impl fmt::Display for House {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material: {}\nNumber of residents: {}\nAddress: {}\n", self.material, self.residents, self.address)
    }
}

fn main() {
    let house = House::new(Material::Brick, 10, "Kolotushkina, Pushkina st.");
    let mut second_house = House::new(Material::Concrete, 32, "Pushkina, Kolotushkina st.");
    
    println!("House 1:\n{house}");
    println!("House 2:\n{second_house}");

    second_house.add_resident();
    second_house.change_material(None);
    second_house.change_material(Some(Material::Brick));
    second_house.add_resident();
}
