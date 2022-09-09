// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
//
// Notes:
// * Optionally use generics for each state

struct LuggageId(usize);
struct Luggage<T> {
    id: LuggageId,
    state: T,
}

struct BeginCustody {}
struct CheckIn {}
struct OnLoad {}
struct OffLoad {}
struct AwaitingPickUp {}
struct EndCustody;

impl Luggage<BeginCustody> {
    fn new(id: LuggageId) -> Self {
        Self {
            id: id,
            state: BeginCustody {},
        }
    }

    fn check_in(self) -> Luggage<CheckIn> {
        self.next(CheckIn {})
    }
}

impl<T> Luggage<T> {
    fn next<U>(self, state: U) -> Luggage<U> {
        Luggage {
            id: self.id,
            state: state,
        }
    }
}

impl Luggage<CheckIn> {
    fn onload(self) -> Luggage<OnLoad> {
        println!("Onload phase.");
        self.next(OnLoad {})
    }
}

impl Luggage<OnLoad> {
    fn offload(self) -> Luggage<OffLoad> {
        println!("Offload phase.");
        self.next(OffLoad {})
    }
}

impl Luggage<OffLoad> {
    fn carousel(self) -> Luggage<AwaitingPickUp> {
        println!("Carousel phase.");
        self.next(AwaitingPickUp {})
    }
}

impl Luggage<AwaitingPickUp> {
    fn pickup(self) -> Luggage<EndCustody> {
        println!("PickUp phase.");
        self.next(EndCustody)
    }
}

fn main() {
    let id = LuggageId(666);
    let luggage = Luggage::new(id);

    let luggage = luggage.check_in().onload().offload().carousel().pickup();
}
