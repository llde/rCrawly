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
        let mut amen = self.client.get(uri).send().unwrap();
        let mut resp = String::new();
        amen.read_to_string(&mut resp);
        LoadResult{succeded: true, uri: uri.to_string(), parsed: Parsed::parse(resp)}
    }
}


//TODO generify all with traits
//TODO implement Drop trait