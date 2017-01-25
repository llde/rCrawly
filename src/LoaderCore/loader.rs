extern crate hyper_rustls;
use hyper::net::HttpsConnector;
use self::hyper_rustls::TlsClient;
use hyper::{Client,Url};
use super::loadResult::LoadResult;
use super::Parsing::{Parsed,Parse};
use std::io::Read;

pub struct GondorLoader {
    client : Client,
}

impl GondorLoader{
    pub fn new() -> GondorLoader{
        GondorLoader {client: Client::with_connector(HttpsConnector::new(TlsClient::new()))}
    }
    pub fn load(&self, uri: Url) -> LoadResult{
        let mut amen = self.client.get(uri.as_str()).send();
        if let Ok(mut content) = amen{
            let mut resp = String::new();
            content.read_to_string(&mut resp);
            LoadResult::new(uri, Parsed::parse(resp))
        }
        else if let Err(error) = amen{
            LoadResult::new_error(uri, error)
        }
        else{
            unreachable!();
        }
    }
}


//TODO generify all with traits
//TODO implement Drop trait