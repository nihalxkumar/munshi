// main.rs serves as a main entry point for our application.
// it will be responsible for setting up the application and beginning the execution of munshi.

mod models;
mod app;

mod commands;

fn main() {
    app::run_program();
}
