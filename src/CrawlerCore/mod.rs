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
        let mut async_arc = self.async.clone();
        let mut progr_arc = self.progression.clone();
        let mut loaded_arc = self.loaded.clone();
        let mut errors_arc  = self.errors.clone();
        thread::spawn(move || {
            //Producer and Consumer . //TODO Split
            loop {
                for url in to_load_arc.lock().unwrap().iter() {
                    println!("Submitted: {}", url);
                    let x = async_arc.loadAsync(&url);
                    progr_arc.lock().unwrap().push_back(x);
                }

                for fut in progr_arc.lock().unwrap().iter(){
                    let res : LoadResult;
                    loop{
                        let x = fut.get();
                        if let Some(el) = x{
                            res = el;
                            break;
                        }
                    }
                    //TODO make non-blocking
                    if let Some(ex) = res.exception{
                        println!("Excepted : {} \n  Reason : {}", &res.uri,ex);
                        errors_arc.lock().unwrap().insert(res.uri);

                    }
                    else if let Some(par) = res.parsed{
                        println!("Suceeeded : {} \n ", &res.uri);
                        {
                            let mut locks = (to_load_arc.lock().unwrap(), loaded_arc.lock().unwrap());
                            locks.0.remove(&res.uri);
                            locks.1.insert(res.uri);
                        }
                        for url in par.consume(){
                            let tl = to_load_arc.lock().unwrap().contains(&url);
                            if tl == true{continue;}
                            let ll = loaded_arc.lock().unwrap().contains(&url);
                            if ll == true {continue;}
                            let el = errors_arc.lock().unwrap().contains(&url);
                            if el == true  {continue;}
                            to_load_arc.lock().unwrap().insert(url);
                        }
                    }
                }
                progr_arc.lock().unwrap().clear();
            }
        });
    }

    pub fn add(&self, url : String){
        //TODO State Controls
        let tl = self.to_load.lock().unwrap().contains(&url);
        if tl == true{return;}
        let ll = self.loaded.lock().unwrap().contains(&url);
        if ll == true {return;}
        let el = self.errors.lock().unwrap().contains(&url);
        if el == true  {return;}
        let cont = self.loaded.lock().unwrap().contains(&url);
        self.to_load.lock().unwrap().insert(url);
    }

    fn cancel(&self){unimplemented!()}

    fn get(&self){unimplemented!()}
}