use super::AsyncLoaderCore::{AsyncLoader,Future};
use super::LoaderCore::*;
use super::LoaderCore::Parsing::Parse;
use std::sync::{Arc,Mutex};
use std::collections::{VecDeque,HashSet};
use std::thread;

enum STATUS{
    INIT,
    RUNNING,
    SUSPENDED,
    TERMINATED,
    CANCELLED,
}

pub struct DagonCrawler{
    async : Arc<AsyncLoader>,
    to_load : Arc<Mutex<HashSet<String>>>,
    loaded : Arc<Mutex<HashSet<String>>>,
    errors : Arc<Mutex<HashSet<String>>>,
    progression : Arc<Mutex<VecDeque<Arc<Future<LoadResult>>>>>,
    status : Arc<Mutex<STATUS>>, //TODO ENUM
}


impl DagonCrawler{
    pub fn new(to_load : HashSet<String>, loaded: HashSet<String>, errors : HashSet<String>) -> DagonCrawler{
        DagonCrawler{async : Arc::new(AsyncLoader::new(50)), to_load : Arc::new(Mutex::new(to_load)), loaded: Arc::new(Mutex::new(loaded)), errors: Arc::new(Mutex::new(errors)), progression : Arc::new(Mutex::new(VecDeque::new())), status : Arc::new(Mutex::new(STATUS::INIT))}
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
        let mut errors_arc  = self.errors.clone();
        thread::spawn(move || {
            //Producer
            loop {
           /*     let url = to_load_arc.lock().unwrap().into_iter().next();
                if let Some(s) = url {
                    println!("Submitted: {}", s);
                    let x = async_arc.loadAsync(&s);
               //     to_load_arc.lock().unwrap().remove(s);
                    progr_arc.lock().unwrap().push_back(x);
                }*/
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
                        if let Some(pars) = result.parsed {
                            loaded_arc.lock().unwrap().insert(url);
                            for link in pars.links {
                                to_load_arc1.lock().unwrap().insert(link);
                            }
                        }
                        else{
                  //          errors_arc.lock().unwrap().insert(result.uri);
                            println!("{}" , result.exception.unwrap());
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
        let cont = self.loaded.lock().unwrap().contains(&url);
        self.to_load.lock().unwrap().insert(url);
    }

    fn cancel(&self){unimplemented!()}

    fn get(&self){unimplemented!()}
}