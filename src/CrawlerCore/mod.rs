use super::AsyncLoaderCore::{AsyncLoader,Future};
use super::LoaderCore::*;
use super::LoaderCore::Parsing::Parse;
use std::sync::{Arc,Mutex};
use std::collections::VecDeque;
use std::thread;

pub struct DagonCrawler{
    async : Arc<AsyncLoader>,
    to_load : Arc<Mutex<Vec<String>>>,
    loaded : Arc<Mutex<Vec<String>>>,
    errors : Vec<String>,
    progression : Arc<Mutex<VecDeque<Arc<Future<LoadResult>>>>>,
    status : u32, //TODO ENUM
}


impl DagonCrawler{
    pub fn new(to_load : Vec<String>, loaded: Vec<String>, errors : Vec<String>) -> DagonCrawler{
        DagonCrawler{async : Arc::new(AsyncLoader::new(50)), to_load : Arc::new(Mutex::new(to_load)), loaded: Arc::new(Mutex::new(loaded)), errors: errors, progression : Arc::new(Mutex::new(VecDeque::new())), status : 1}
        //unimplemented!()
    }

    pub fn start(&self){
        //TODO status.
        let mut to_load_arc = self.to_load.clone();
        let mut to_load_arc1 = self.to_load.clone();
        let mut async_arc = self.async.clone();
        let mut progr_arc = self.progression.clone();
        let mut progr_arc1 = self.progression.clone();
        let mut loaded_arc = self.loaded.clone();
        thread::spawn(move || {
            //Producer
            loop{
                let url = to_load_arc.lock().unwrap().pop();
                if let Some(s) = url{
                    println!("Submitted: {}", s);
                    let x = async_arc.loadAsync(s);
                    progr_arc.lock().unwrap().push_back(x);
                }
            }
        });
        thread::spawn(move || {
            //Consumer
            loop {
                let el = progr_arc1.lock().unwrap().pop_front();
                if let Some(elt) = el {
                    if let Some(result) = elt.get() {
                        let url = result.uri;
                        println!("Read: {}", url);
                      //  self.loaded.push(url);
                        loaded_arc.lock().push(url);
                        let links = result.parsed.links;
                        for link in links{
                            to_load_arc1.lock().unwrap().push(link);
                        }
                        //TODO everything else.
                    }
                    else{
                        progr_arc1.lock().unwrap().push_back(elt);
                    }
                }
            }
        });
    }

    pub fn add(&self, url : String){
        //TODO controls
        self.to_load.lock().unwrap().push(url);
    }

    fn cancel(&self){unimplemented!()}

    fn get(&self){unimplemented!()}
}