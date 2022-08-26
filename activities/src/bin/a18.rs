// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
}

fn main() {
    let customer_1: Customer = Customer { age: 33 };
    let purchase_result = try_purchase(&customer_1);

    println!("Result: {:?}", purchase_result);

    let customer_2: Customer = Customer { age: 13 };
    let purchase_result = try_purchase(&customer_2);

    println!("Result: {:?}", purchase_result);
}

fn try_purchase(customer: &Customer) -> Result<String, String> {
    match customer.age {
        x if x < 21 => return Err("UnderAge.".to_owned()),
        _ => return Ok("Purchase allowed.".to_owned()),
    }
}
