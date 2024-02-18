use crate::models::{get_input, Bills};

pub fn remove_bill(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }

    println!("Enter bill name to remove:");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };

    if bills.remove(&name) {
        println!("bill removed");
    } else {
        println!("bill not found");
    }
}
