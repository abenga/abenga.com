#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod views;

fn main() {
    views::rocket().launch();
}
