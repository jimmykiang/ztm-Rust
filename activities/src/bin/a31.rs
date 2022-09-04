// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

trait Material {
    fn cost_square_meter(&self) -> f64;
    fn square_meters(&self) -> f64;
    fn total_cost(&self) -> f64 {
        self.square_meters() * self.cost_square_meter()
    }
}

struct Carpet {
    square_meters: f64,
}

impl Material for Carpet {
    fn square_meters(&self) -> f64 {
        self.square_meters
    }

    fn cost_square_meter(&self) -> f64 {
        10.0
    }
}

struct Tile {
    square_meters: f64,
}

impl Material for Tile {
    fn square_meters(&self) -> f64 {
        self.square_meters
    }

    fn cost_square_meter(&self) -> f64 {
        15.0
    }
}

struct Wood {
    square_meters: f64,
}

impl Material for Wood {
    fn square_meters(&self) -> f64 {
        self.square_meters
    }

    fn cost_square_meter(&self) -> f64 {
        20.0
    }
}

fn total_cost(materials: &Vec<Box<dyn Material>>) -> f64 {
    materials.iter().map(|x| x.total_cost()).sum()
}

fn main() {
    let carpet = Box::new(Carpet { square_meters: 1.0 });
    let tile = Box::new(Tile { square_meters: 2.0 });
    let wood = Box::new(Wood {
        square_meters: 10.0,
    });

    let mut materials: Vec<Box<dyn Material>> = Vec::new();

    materials.push(carpet);
    materials.push(tile);
    materials.push(wood);

    let total = total_cost(&materials);
    println!("Total={:?}", total);
}
