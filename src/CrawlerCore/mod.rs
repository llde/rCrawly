use super::AsyncLoaderCore::AsyncLoader;
use super::LoaderCore::*;
use super::LoaderCore::Parsing::Parse;
use super::API::Future;
use std::sync::{Arc,Mutex};
use std::collections::{VecDeque,HashSet};
use std::thread;
use hyper::Url;

enum STATUS{
    INIT,
    RUNNING,
    SUSPENDED,
    TERMINATED,
    CANCELLED,
}

pub struct DagonCrawler{
    //TODO get concurrent data structures for better performance
    async : Arc<AsyncLoader>,
    to_load : Arc<Mutex<HashSet<Url>>>,
    loaded : Arc<Mutex<HashSet<Url>>>,
    errors : Arc<Mutex<HashSet<Url>>>,
    progression : Arc<Mutex<VecDeque<Arc<Future<LoadResult>>>>>,
    status : Arc<Mutex<STATUS>>, //TODO ENUM
}



impl DagonCrawler{
    pub fn new(to_load : HashSet<Url>, loaded: HashSet<Url>, errors : HashSet<Url>) -> DagonCrawler{
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
            let mut holder = HashSet::new();
            loop {
                for url in to_load_arc.lock().unwrap().iter() {
                    if(holder.contains(url)){continue;}
                    println!("Submitted: {}", url);
                    let x = async_arc.loadAsync(url.clone());
                    holder.insert(url.clone());
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
                    //TODO make non-blocking. Require Future::is_done()
                    let base_url = res.uri;
                    if let Some(ex) = res.exception{
                        println!("Excepted : {} \n  Reason : {}", &base_url ,ex);
                        to_load_arc.lock().unwrap().remove(&base_url);
                        errors_arc.lock().unwrap().insert(base_url);

                    }
                    else if let Some(par) = res.parsed{
                        println!("Suceeeded : {} \n ", &base_url);
                        for url in par.consume(){
                            let opt = base_url.join(&url);
                            if let Ok(new_url) = opt {
                                let tl = to_load_arc.lock().unwrap().contains(&new_url);
                                if tl == true { continue; }
                                let ll = loaded_arc.lock().unwrap().contains(&new_url);
                                if ll == true { continue; }
                                let el = errors_arc.lock().unwrap().contains(&new_url);
                                if el == true { continue; }
                                //TODO CrawlerResult and control with the predicate
                                to_load_arc.lock().unwrap().insert(new_url);
                            }
                            else if let Err(error) = opt{
                                println!("{:?}", error);
                            }
                        }
                        {
                            let mut locks = (to_load_arc.lock().unwrap(), loaded_arc.lock().unwrap());
                            locks.0.remove(&base_url);
                            locks.1.insert(base_url);
                        }
                    }
                }
                progr_arc.lock().unwrap().clear();
            }
        });
    }

    pub fn add(&self, url : Url){
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