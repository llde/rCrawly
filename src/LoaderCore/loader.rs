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
        let amen = self.client.get(uri.as_str()).send();
        match amen{
            Ok(mut content) => {
                let mut resp = String::new();
                //TODO handle possible read_to_string failures
                content.read_to_string(&mut resp);
                LoadResult::new(uri, Parsed::parse(resp))
            }
            Err(error) => {
                LoadResult::new_error(uri, error)
            }
        }
    }


    pub fn check(&self, uri: Url) -> LoadResult{
        let amen = self.client.get(uri.as_str()).send();
        match amen{
            Ok(mut content) => {
                LoadResult::new_check(uri)
            }
            Err(error) => {
                LoadResult::new_error(uri, error)
            }
        }
    }
}


//TODO generify all with traits
//TODO implement Drop trait