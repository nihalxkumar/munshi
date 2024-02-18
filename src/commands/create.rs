pub use crate::models::{get_bill_amount, get_input, Bills, Bill};

pub fn add_bill(bills: &mut Bills) {
    println!("Bill name:");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };

    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };

    let bill = Bill { name, amount };
    bills.add(bill);

    println!("Bill added");
}