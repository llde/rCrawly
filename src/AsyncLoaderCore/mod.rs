use super::LoaderCore::loader::GondorLoader;
use super::LoaderCore::loadResult::LoadResult;
use std::sync::{Arc, Condvar, Mutex,RwLock};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::collections::VecDeque;
use std::cell::RefCell;

pub trait Observable {

    fn notifyListenes(&self);
    fn addListeners(&mut self,  listener :  FnMut() -> ());
}

enum STATUS{
    INIT,
    SUCCEEDED,
    FAILED,
    RUNNING,
    ABORTED,

}

trait Task<T>  : Send{
    fn call(self: Box<Self>) -> T;
}

impl<T,F : Send + FnOnce() -> T> Task<T> for F{
    fn call(self : Box<F>) -> T{
        (*self)()
    }
}


//TODO Mutex<> o RwLock<>
pub struct Future<T>
{
    rec  : RwLock<Option<Receiver<T>>>,
    stat : RwLock<STATUS>,
    funz : RwLock<Option<Box<Task<T>>>>
}


impl <T> Future<T>
    where T: Send + 'static
{
    pub fn get(&self) -> Option<T>{
        //TODO Report the error in case of failure.
        //TODO cache te result for a allow more return.
        //TODO drop the receiver after first read.
        let opt = self.rec.read().unwrap();
        if let Some(ref receiver) = *opt{
            Some(receiver.recv().unwrap())
        }
        else{
            None
        }
    }

    pub fn run(&self) -> (){
        let mut funct1 = self.funz.write().unwrap().take().unwrap();
        let (tx,rx) : (Sender<T>, Receiver<T>) = mpsc::channel();
        *self.rec.write().unwrap() = Some(rx);
        *self.stat.write().unwrap() = STATUS::RUNNING;
        let ret = funct1.call();
        tx.send(ret).unwrap();
        *self.stat.write().unwrap() = STATUS::SUCCEEDED;
    }

    pub fn new<F>(runnable : F) -> Future<T>  where
        F: FnOnce() -> T, F: Send + 'static , T: Send + 'static    {
        Future{rec:RwLock::new(Option::None) , stat: RwLock::new(STATUS::INIT), funz : RwLock::new(Some(Box::new(runnable)))}
    }
}

unsafe impl <T> Send for Future<T>{}
unsafe impl <T> Sync for Future<T>{}


pub struct ThreadPoolExecutor<T>{
    workers : Arc<Mutex<VecDeque<Arc<Future<T>>>>>,
    status: STATUS,
    task_count : u32,
    num_thread: u32,
    current_threads : Arc<RwLock<u32>>,
}
impl <T> ThreadPoolExecutor<T> where T: Send + 'static{
//TODO AVOID LOCK POISON
    //TODO TRY TO AVOID THE T PARAMETER
    fn new(num_threads : u32) -> ThreadPoolExecutor<T>{
        ThreadPoolExecutor{workers: Arc::new(Mutex::new(VecDeque::new())), status : STATUS::INIT, task_count : 0, num_thread : num_threads, current_threads:Arc::new(RwLock::new(0))}
    }

    fn submit<F>(&self, function : F ) -> Arc<Future<T>> where F : FnOnce() -> T + Send +'static {
        let fut = Arc::new(Future::new(function));
        self.workers.lock().unwrap().push_back(fut.clone());
        println!("{} {}", *self.current_threads.read().unwrap(), self.num_thread);
        if *self.current_threads.read().unwrap() < self.num_thread{
            {
                let mut guard = self.current_threads.write().unwrap();
                *guard += 1;
            }
            let mut arc = self.workers.clone();
            let mut arc_curr_thread = self.current_threads.clone();
            thread::spawn(move ||{
               loop{
                   let mut option = None;
                   {
                       option = arc.lock().unwrap().pop_front();
                   }
                   if let Some(el) = option{
                       el.run();
                   }
                   else{
                       break;
                   }
               }
               *arc_curr_thread.write().unwrap() -= 1;
            });
        }
        fut
    }
}


pub struct AsyncLoader {
    loaders : Arc<Mutex<Vec<GondorLoader>>>,
    workers : ThreadPoolExecutor<LoadResult>,
    active  : AtomicBool,
}


impl AsyncLoader{
    pub fn new(num : u32) -> AsyncLoader{  //      unimplemented!();

        let mut loaders = Vec::new();
        for n in 0..(num+1){
            loaders.push(GondorLoader::new());
        }
        AsyncLoader{loaders : Arc::new(Mutex::new(loaders)), workers : ThreadPoolExecutor::new(num), active : AtomicBool::new(true)}
    }

    pub fn loadAsync(&self ,uri1 : &str) -> Arc<Future<LoadResult>>{
        let arc = self.loaders.clone();
        let str = uri1.to_string();
        let fut = self.workers.submit(move || {
            let loader;
            let arc1 = arc;
            {
                let ref mut sss = *arc1.as_ref().lock().unwrap();
                loader = sss.remove(0);
            }
            let result = loader.load(&str);
            {
                let ref mut sss = *arc1.as_ref().lock().unwrap();
                sss.push(loader);
                return result;
            }
        });
        fut
    }

//TODO shutdown
    pub fn shutdown(&mut self, now : bool){}


}