// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::cell::RefCell;
use std::rc::Rc;

struct Corporate {
    rentals: Rc<RefCell<Vec<RentalCar>>>,
}

struct StoreFront {
    rentals: Rc<RefCell<Vec<RentalCar>>>,
}

struct RentalCar {
    status: VehicleStatus,
    vehicle_type: VehicleType,
    vin: String,
}

enum VehicleType {
    Car,
    Truck,
}

#[derive(PartialEq, Debug)]
enum VehicleStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_vehicle_rental_status() {
        let mut rental_cars: Vec<RentalCar> = Vec::new();

        let rental_1 = RentalCar {
            status: VehicleStatus::Available,
            vehicle_type: VehicleType::Car,
            vin: "jugon666".to_owned(),
        };

        let rental_2 = RentalCar {
            status: VehicleStatus::Maintenance,
            vehicle_type: VehicleType::Truck,
            vin: "trucky".to_owned(),
        };

        rental_cars.push(rental_1);
        rental_cars.push(rental_2);

        let rental_cars = Rc::new(RefCell::new(rental_cars));

        let corporate = Corporate {
            rentals: Rc::clone(&rental_cars),
        };

        let storeFront = StoreFront {
            rentals: Rc::clone(&rental_cars),
        };

        {
            let mut rentals = storeFront.rentals.borrow_mut();
            let rental = rentals.get_mut(1);

            match rental {
                Some(x) => {
                    assert_eq!(x.status, VehicleStatus::Maintenance);
                    x.status = VehicleStatus::Unavailable;
                }
                None => (),
            }
        }

        {
            let mut rentals = corporate.rentals.borrow_mut();
            if let Some(x) = rentals.get_mut(1) {
                assert_eq!(x.status, VehicleStatus::Unavailable);
                x.status = VehicleStatus::Available
            }
        }

        {
            let mut rentals = storeFront.rentals.borrow_mut();
            if let Some(x) = rentals.get_mut(1) {
                assert_eq!(x.status, VehicleStatus::Available);
            }
        }
    }
}
