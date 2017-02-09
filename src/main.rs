#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
pub extern crate hyper;
use std::sync::Arc;
use std::collections::HashSet;
pub use hyper::{Client,Url};
pub use LoaderCore::loader::GondorLoader;
pub use LoaderCore::loadResult::LoadResult;
pub use AsyncLoaderCore::AsyncLoader;
pub use CrawlerCore::{Predicate,DagonCrawler};
pub use GUI::Crawly;
pub mod LoaderCore;
pub mod AsyncLoaderCore;
pub mod CrawlerCore;
pub mod GUI;
pub mod API;

pub struct PredDom{
    dominio : Url
}

impl Predicate<Url> for PredDom{
    fn accept(&self, other : &Url) -> bool{
        true
    }
}
fn main(){
    println!("Hello");
    let predicate = PredDom{dominio : Url::parse("https://bugs.winehq.org/").unwrap()};
    let crawler = CrawlerCore::DagonCrawler::new(HashSet::new(),HashSet::new(),HashSet::new(), Box::new(predicate));
    crawler.add(Url::parse("https://bugs.winehq.org/show_bug.cgi?id=1").unwrap());
    crawler.start();
    loop{}
  /* let mut async  = AsyncLoaderCore::AsyncLoader::new(50);
   let mut vect : Vec<Arc<Future<LoadResult>>>  = Vec::new();
   //Crawly::new();
   for i  in 0..1000 {
       let mut loader = GondorLoader::new();
       let temp = "https://bugs.winehq.org/show_bug.cgi?id=".to_string() + &format!("{:?}", i);
       vect.push(async.loadAsync(&temp));
    }

    println!("{:?}", vect.len());
    for fut in vect{
        let res = fut.get();
        if let Some(result) = res {
            println!("{:?}", result.uri);
            if let Some(ex) = result.exception{
                println!("Exception : {}", ex);
            }
       }
    }*/
}