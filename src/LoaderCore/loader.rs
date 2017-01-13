use hyper::Client;
use super::loadResult::LoadResult;
use super::Parsing::{Parsed,Parse};
use std::io::Read;

pub struct GondorLoader {
    client : Client,
}

impl GondorLoader{
    pub fn new() -> GondorLoader{
        GondorLoader {client: Client::new()}
    }
    pub fn load(&self, uri: &str) -> LoadResult{
        let mut amen = self.client.get(uri).send();
        if let Ok(mut content) = amen{
            let mut resp = String::new();
            content.read_to_string(&mut resp);
            LoadResult::new(uri.to_string(), Parsed::parse(resp))
        }
        else if let Err(error) = amen{
            LoadResult::new_error(uri.to_string(), error)
        }
        else{
            unreachable!();
        }
    }
}


//TODO generify all with traits
//TODO implement Drop trait