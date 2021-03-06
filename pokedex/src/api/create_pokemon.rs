use crate::api::Status;
use crate::repository::pokemon::Repository;
use crate::domain::create_pokemon;

use rouille;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Deserialize)]
struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

#[derive(Serialize)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn serve(repo: Arc<dyn Repository>, req: &rouille::Request) -> rouille::Response {
    // rouille::Response::json(&Response {
    //     message: String::from("Pokemon created")
    // })

    let req = match rouille::input::json_input::<Request>(req) {
        Ok(req) => create_pokemon::Request {
            number: req.number,
            name: req.name,
            types: req.types,
        },
        _ => return rouille::Response::from(Status::BadRequest),
    };

    // match create_pokemon::execute(repo, req) {
    //     create_pokemon::Response::Ok(number) => rouille::Response::json(&Response { number }),
    //     create_pokemon::Response::BadRequest => rouille::Response::from(Status::BadRequest),
    //     create_pokemon::Response::Conflict => rouille::Response::from(Status::Conflict),
    //     create_pokemon::Response::Error => rouille::Response::from(Status::InternalServerError),
    // }

    match create_pokemon::execute(repo, req) {
        Ok(create_pokemon::Response { 
            number, name, types
        }) => rouille::Response::json(&Response { 
            number, name, types 
        }),
        Err(create_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
        Err(create_pokemon::Error::Conflict) => rouille::Response::from(Status::Conflict),
        Err(create_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
    }
}