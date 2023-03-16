#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

fn allergen_from_u32(n: u32) -> Option<Allergen> {
    match n {
        0 => Some(Allergen::Eggs),
        1 => Some(Allergen::Peanuts),
        2 => Some(Allergen::Shellfish),
        3 => Some(Allergen::Strawberries),
        4 => Some(Allergen::Tomatoes),
        5 => Some(Allergen::Chocolate),
        6 => Some(Allergen::Pollen),
        7 => Some(Allergen::Cats),
        _ => None,
    }
}

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & (1 << (*allergen as u32)) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..=7).fold(vec![], |mut allergies, i| {
            if self.score & (1 << i) != 0 {
                if let Some(allergen) = allergen_from_u32(i) {
                    allergies.push(allergen);
                }
            }
            allergies
        })
    }
}

