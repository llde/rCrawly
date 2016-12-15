extern crate find_folder;

pub struct Crawly{
    name : String,
    version : u8,
}

impl Crawly{
    pub fn new() -> Crawly{
        let cr = Crawly{name : "Crawly".to_string(), version : 1};
        cr
    }
}