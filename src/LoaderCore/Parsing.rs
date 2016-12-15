extern crate select;
pub use self::select::document::Document;
pub use self::select::predicate::{Attr, Class, Name};
pub struct Parsed{
    pub links : Vec<String>,
}

pub trait Parse{
    fn parse(parsing : String) -> Parsed;
    fn consume(self) -> Vec<String>;
}

impl Parse for Parsed{
    fn parse(parsing : String) -> Parsed{
        let dom = Document::from(&*parsing);
        let mut links = Vec::new();
        for node in dom.find(Name("a")).iter(){
            let link = node.attr("href");
            if let Some(lin) = link{
                links.push(lin.to_string());
            }
        }
        Parsed{links: links }
    }

    fn consume(self) -> Vec<String>{
        self.links
    }

}