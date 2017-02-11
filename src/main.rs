#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
pub extern crate hyper;
use std::sync::Arc;
use std::collections::HashSet;
use std::{time,thread};
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
unsafe impl Sync for PredDom{}
unsafe impl Send for PredDom{}
impl Predicate<Url> for PredDom{
    fn accept(&self, other : &Url) -> bool{
        //TODO accept IP domains
        //TODO remove option equal in favor of an explicit equal
        if other.domain() == self.dominio.domain(){
            true
        }
        else{false}
    }
}


fn main(){
    let predicate = PredDom{dominio : Url::parse("http://www.agriturismomelograno.com/").unwrap()};
    let crawler = CrawlerCore::DagonCrawler::new(HashSet::new(),HashSet::new(),HashSet::new(), Box::new(predicate));
    crawler.add(Url::parse("http://www.agriturismomelograno.com/").unwrap());
    crawler.start();
    thread::sleep(time::Duration::from_secs(90));
    for url in crawler.get_to_load().iter(){
        println!("{}", url);
    }
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