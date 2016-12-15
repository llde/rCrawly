pub use super::Parsing::Parsed;
use hyper::error::Error;

pub struct LoadResult{
    pub uri : String,
    pub parsed : Option<Parsed>,
    pub exception : Option<Error>,
}

impl LoadResult{
    pub fn new_error(uri : String, err : Error) -> LoadResult{
        LoadResult{uri: uri, parsed : None, exception : Some(err)}
    }
    pub fn new(uri : String, content : Parsed) -> LoadResult{
        LoadResult{uri : uri, parsed : Some(content), exception : None}
    }
}

//TODO make really Send and Sync
unsafe impl Send for LoadResult{}
unsafe impl Sync for LoadResult{}