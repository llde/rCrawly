pub use super::Parsing::Parsed;
use hyper::error::Error;
use hyper::Url;
pub struct LoadResult{
    pub uri : Url,
    pub parsed : Option<Parsed>,
    pub exception : Option<Error>,
}

impl LoadResult{
    pub fn new_error(uri : Url, err : Error) -> LoadResult{
        LoadResult{uri: uri, parsed : None, exception : Some(err)}
    }
    pub fn new(uri : Url, content : Parsed) -> LoadResult{
        LoadResult{uri : uri, parsed : Some(content), exception : None}
    }

    pub fn new_check(uri:Url) -> LoadResult{
        LoadResult{uri : uri, parsed :  None , exception : None}
    }
}

//TODO make really Send and Sync
unsafe impl Send for LoadResult{}
unsafe impl Sync for LoadResult{}