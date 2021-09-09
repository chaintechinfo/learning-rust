use crate::domain::entity::{Pokemon, PokemonNumber, PokemonName, PokemonTypes};
use crate::repository::pokemon::{InsertError, Repository};

use std::convert::TryFrom;
use std::sync::Arc;

pub struct Request {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(name), Ok(types)) => match repo.insert(number, name, types) {
            Ok(Pokemon{
                number,
                name,
                types,
            }) => Ok(Response {
                number: u16::from(number), 
                name: String::from(name),
                types: Vec::<String>::from(types),
            }),
            Err(InsertError::Conflict) => Err(Error::Conflict),
            Err(InsertError::Unknown) => Err(Error::Unknown),
        }
        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::pokemon::InMemoryRepository;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());

        // let number = 20;
        // let req = Request {
        //     number,
        //     name: String::from("Pikachu"),
        //     types: vec![String::from("Electric")],
        // };
        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        );

        let res = execute(repo, req);

        // assert_eq!(res, number);
        match res {
            Ok(Response {
                number,
                name,
                types,
            }) => {
                assert_eq!(number, u16::from(PokemonNumber::pikachu()));
                assert_eq!(name, String::from(PokemonName::pikachu()));
                assert_eq!(types, Vec::<String>::from(PokemonTypes::pikachu()));
            },
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());

        // let req = Request {
        //     number: 25,
        //     name: String::from(""),
        //     types: vec![String::from("Electric")],
        // };
        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::bad(),
            PokemonTypes::pikachu(),
        );

        let res = execute(repo, req);

        match res {
            Err(Error::BadRequest) => {}, 
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
        // let number = PokemonNumber::try_from(25).unwrap();
        // let name = PokemonName::try_from(String::from("Pikachu")).unwrap();
        // let types = PokemonTypes::try_from(vec![String::from("Electric")]).unwrap();
        let repo = Arc::new(InMemoryRepository::new());

        repo.insert(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        ).ok();

        // let req = Request { 
        //     number: 25, 
        //     name: String::from("Charmander"),
        //     types: vec![String::from("Fire")],
        // };
        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::charmander(),
            PokemonTypes::charmander(),
        );

        let res = execute(repo, req);

        match res {
            Err(Error::Conflict) => {}, 
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_hanppens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        // let number = 25;
        // let req = Request {
        //     number,
        //     name: String::from("Pikachu"),
        //     types: vec![String::from("Electric")],
        // };
        let req = Request::new(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        );

        let res = execute(repo, req);

        match res {
            Err(Error::Unknown) => {},
            _ => unreachable!(),
        }
    }

    impl Request {
        fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
            Self { 
                number: u16::from(number), 
                name: String::from(name), 
                types: Vec::<String>::from(types),
            }
        }
    }
}
