use super::LoaderCore::loader::GondorLoader;
use super::LoaderCore::loadResult::LoadResult;
use super::API::ThreadPoolExecutor;
use super::API::Future;
use std::sync::{Arc,Mutex};
use std::sync::atomic::{Ordering,AtomicBool};
use hyper::Url;
pub trait Observable {

    fn notifyListenes(&self);
    fn addListeners(&mut self,  listener :  FnMut() -> ());
}


pub struct AsyncLoader {
    loaders : Arc<Mutex<Vec<GondorLoader>>>,
    workers : ThreadPoolExecutor<LoadResult>,
    active  : AtomicBool,
}


impl AsyncLoader{
    pub fn new(num : u32) -> AsyncLoader{
        let mut loaders = Vec::new();
        for n in 0..(num+1){
            loaders.push(GondorLoader::new());
        }
        AsyncLoader{loaders : Arc::new(Mutex::new(loaders)), workers : ThreadPoolExecutor::new(num), active : AtomicBool::new(true)}
    }

    pub fn loadAsync(&self, uri1 : Url) -> Arc<Future<LoadResult>>{
        let arc = self.loaders.clone();
        let fut = self.workers.submit(move || {
            let loader = arc.as_ref().lock().unwrap().remove(0);
            let result = loader.load(uri1);
            arc.as_ref().lock().unwrap().push(loader);
            return result;
        });
        //TODO proper error handling
        if let Ok(fu) = fut{
            return fu;
        }
        unreachable!();
    }

    pub fn checkAsync(&self, uri : Url) -> Arc<Future<LoadResult>> {
        let arc = self.loaders.clone();
        let fut = self.workers.submit(move || {
            let loader = arc.as_ref().lock().unwrap().remove(0);
            let result = loader.check(uri);
            arc.as_ref().lock().unwrap().push(loader);
            return result;
        });
        //TODO proper error handling
        if let Ok(fu) = fut{
            return fu;
        }
        unreachable!();
    }

//TODO shutdown
    pub fn shutdown(&self, now : bool){
        self.active.store(true, Ordering::Relaxed);
        self.shutdown(now);
    }

}