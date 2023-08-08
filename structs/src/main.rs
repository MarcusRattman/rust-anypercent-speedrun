use std::fmt;
struct House {
    material: Material,
    residents: i32,
    address: String,
}
impl House {
    pub fn new(mat: Material, res: i32, adrs: &str) -> Self {
        return Self {
            material: mat,
            residents: res,
            address: String::from(adrs),
        };
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

fn add_resident(house: &mut House) {
    house.residents += 1;
    println!("Added resident to the House:\n{house}");
}

fn change_material(house: &mut House, material: Option<Material>) {
    let mat = material.unwrap_or(house.material);
    house.material = mat;
    println!("Changed material of the House:\n{house}")
}

fn main() {
    let house = House {
        material: Material::Brick,
        residents: 10,
        address: String::from("Kolotushkina, Pushkina st.")
    };

    println!("House 1:\n{house}");

    let mut second_house = House::new(Material::Concrete, 32, "Pushkina, Kolotushkina st.");

    println!("House 2:\n{second_house}");
    add_resident(&mut second_house);
    change_material(&mut second_house, Some(Material::Brick));
}
