// This contains the main application logic, such as initializing the application, handling user input, and orchestrating interactions between different components of munshi.
use crate::{
    models::{Bills, get_input},
    commands::{
        create::add_bill,
        read::view_bills,
        update::update_bill,
        delete::remove_bill
    }
};

#[derive(Debug)]
pub enum MainMenu{
    AddBill,
    ViewBills,
    RemoveBill,
    UpdateBill,
    Exit,
}

impl MainMenu {
    pub fn show() {
        println!("Main Menu:");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Update Bill");
        println!("5. Exit");
        println!("Please enter your choice:");
    }

    pub fn from_str(input: &str) -> Option<Self> {
        match input.trim().to_owned().as_str() {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewBills),
            "3" => Some(MainMenu::RemoveBill),
            "4" => Some(MainMenu::UpdateBill),
            "5" => Some(MainMenu::Exit),
            _ => None,
        }
    }
}

pub(crate) fn run_program() -> Option<()> {
    let mut bills = Bills::new();

    loop {
        MainMenu::show();

        let input = get_input()?;

        match MainMenu::from_str(&input) {
            Some(MainMenu::AddBill) => add_bill(&mut bills),
            Some(MainMenu::ViewBills) => view_bills(&bills),
            Some(MainMenu::RemoveBill) => remove_bill(&mut bills),
            Some(MainMenu::UpdateBill) => update_bill(&mut bills),
            Some(MainMenu::Exit) => break,
            None => println!("Invalid input. Please try again."),
        }
    }
    None
}
