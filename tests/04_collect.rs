#[cfg(test)]
mod tests {
    struct SeaCreature {
        // String is a struct
        animal_type: String,
        name: String,
        arms: i32,
        legs: i32,
        weapon: String,
    }

    #[test]
    fn test_collect() {
        // SeaCreature's data is on stack
        let ferris = SeaCreature {
            // String struct is also on stack,
            // but holds a reference to data on heap
            animal_type: String::from("crab"),
            name: String::from("Ferris"),
            arms: 2,
            legs: 4,
            weapon: String::from("claw"),
        };

        let sarah = SeaCreature {
            animal_type: String::from("octopus"),
            name: String::from("Sarah"),
            arms: 8,
            legs: 0,
            weapon: String::from("brain"),
        };

        println!(
            "{} is a {}. They have {} arms, {} legs, and a {} weapon",
            ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
        );
        println!(
            "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
            sarah.name, sarah.animal_type, sarah.arms, sarah.legs
        );
    }
}

#[test]
fn test_location() {
    struct Location(i32, i32);

    // This is still a struct on a stack
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
}

#[test]
fn test_enum() {
    enum Species {
        Crab,
        Octopus,
        Fish,
        Clam,
    }

    struct SeaCreature {
        species: Species,
        name: String,
        arms: i32,
        legs: i32,
        weapon: String,
    }

    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    match ferris.species {
        Species::Crab => println!("{} is a crab, weapon: {}", ferris.name, ferris.weapon),
        Species::Octopus => println!("{} is a octopus", ferris.name),
        Species::Fish => println!("{} is a fish", ferris.name),
        Species::Clam => println!("{} is a clam", ferris.name),
    }
}
