use super::AsyncLoaderCore::AsyncLoader;
use super::LoaderCore::*;
use super::LoaderCore::Parsing::Parse;
use super::API::Future;
use std::sync::{Arc,Mutex};
use std::collections::{VecDeque,HashSet};
use std::thread;
use hyper::Url;

pub trait Predicate<T>{
    fn accept(&self , other : &T) -> bool;
}

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
    pred : Arc<Box<Predicate<Url>>>
}



impl DagonCrawler{
    pub fn new(to_load : HashSet<Url>, loaded: HashSet<Url>, errors : HashSet<Url>, predicate : Box<Predicate<Url>>) -> DagonCrawler{
        DagonCrawler{async : Arc::new(AsyncLoader::new(50)), to_load : Arc::new(Mutex::new(to_load)), loaded: Arc::new(Mutex::new(loaded)), errors: Arc::new(Mutex::new(errors)), progression : Arc::new(Mutex::new(VecDeque::new())), status : Arc::new(Mutex::new(STATUS::INIT)), pred : Arc::new(predicate)}
    }

    pub fn start(&self){
        {
            let ref status = self.status.as_ref().lock().unwrap();
            if let STATUS::RUNNING = **status {
                return;
            } else if let STATUS::CANCELLED = **status {
                return;
            } else if let STATUS::TERMINATED = **status {
                return;
            }
        }
        let to_load_arc = self.to_load.clone();
        let async_arc = self.async.clone();
        let progr_arc = self.progression.clone();
        let loaded_arc = self.loaded.clone();
        let errors_arc  = self.errors.clone();
        let pred_arc  = self.pred.clone();
        *self.status.lock().unwrap() = STATUS::RUNNING;
        let status_arc = self.status.clone();
        thread::spawn(move || {
            //Producer and Consumer . //TODO Split
            let mut holder = HashSet::new();
            loop {
                if to_load_arc.lock().unwrap().len() == 0{
                    *status_arc.lock().unwrap() = STATUS::TERMINATED;
                    return;
                }
                for url in to_load_arc.lock().unwrap().iter() {
                    if holder.contains(url){continue;}
                    println!("Submitted: {}", url);
                    let x = async_arc.loadAsync(url.clone());
                    holder.insert(url.clone());
                    progr_arc.lock().unwrap().push_back(x);
                }
                let mut holderer = Vec::new();
                for (index, fut) in progr_arc.lock().unwrap().iter().enumerate(){
                    if fut.is_done() {
                        let res = fut.get().unwrap();
                        let base_url = res.uri;
                        if let Some(ex) = res.exception {
                            println!("Excepted : {} \n  Reason : {}", &base_url, ex);
                            let mut locks = (to_load_arc.lock().unwrap(), errors_arc.lock().unwrap());
                            locks.0.remove(&base_url);
                            holder.remove(&base_url);
                            locks.1.insert(base_url);
                        } else if let Some(par) = res.parsed {
                            println!("Suceeeded : {} \n ", &base_url);
                            for url in par.consume() {
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
                                } else if let Err(error) = opt {
                                    println!("{:?}", error);
                                }
                            }
                            {
                                let mut locks = (to_load_arc.lock().unwrap(), loaded_arc.lock().unwrap());
                                locks.0.remove(&base_url);
                                holder.remove(&base_url);
                                locks.1.insert(base_url);
                            }
                        }
                        holderer.push(index);
                    }
                }
                for index in holderer.into_iter().rev(){
                    progr_arc.lock().unwrap().remove(index);
                }
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