use std::sync::Arc;

mod api;
mod domain;
mod repository;
use repository::pokemon::InMemoryRepository;

#[macro_use]
extern crate rouille;
extern crate serde;

fn main() {
    // println!("Hello, Welcom to Pokemon's World!");
    let repo = Arc::new(InMemoryRepository::new());
    api::serve("localhost:8000", repo);
}
