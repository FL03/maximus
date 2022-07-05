/*
   Appellation: maximus
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use crate::core::*;
mod core;

pub mod api;
pub mod clients;
pub mod data;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
