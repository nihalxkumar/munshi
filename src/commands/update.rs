use crate::models::{get_bill_amount, get_input, Bills};

pub fn update_bill(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }

    println!("Enter bill name to update:");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };

    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };

    if bills.update(&name, amount) {
        println!("bill updated");
    } else {
        println!("bill not found");
    }
}
