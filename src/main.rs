#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
pub extern crate hyper;
use std::sync::Arc;
pub use hyper::Client;
pub use LoaderCore::loader::GondorLoader;
pub use LoaderCore::loadResult::LoadResult;
pub use AsyncLoaderCore::{AsyncLoader,Future};
pub use CrawlerCore::DagonCrawler;
pub use GUI::Crawly;
pub mod LoaderCore;
pub mod AsyncLoaderCore;
pub mod CrawlerCore;
pub mod GUI;
pub mod API;


fn main(){
    println!("Hello");
    let crawler = CrawlerCore::DagonCrawler::new(Vec::new(),Vec::new(),Vec::new());
    crawler.add("https://bugs.winehq.org/show_bug.cgi?id=1".to_string());
    crawler.start();
    loop{}
   // let mut async  = AsyncLoaderCore::AsyncLoader::new(50);
   // let mut vect : Vec<Arc<Future<LoadResult>>>  = Vec::new();
    //   Crawly::new();
  //  for i  in 0..100 {
       // let mut loader = GondorLoader::new();
    //    let temp = "https://bugs.winehq.org/show_bug.cgi?id=".to_string() + &format!("{:?}", i);
    //    vect.push(async.loadAsync(temp));
    //}

   // println!("{:?}", vect.len());
    //for fut in vect{
     //   let res = fut.get();
      //  if let Some(result) = res {
       //     println!("{:?}", result.uri);
        //}
    //}

}