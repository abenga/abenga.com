#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod views;
pub mod lib;


fn main() {
    views::rocket().launch();
}
