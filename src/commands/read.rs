use crate::models::Bills;

pub fn view_bills(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}
