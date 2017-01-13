use std::sync::{Arc, Condvar, Mutex,RwLock};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::collections::VecDeque;
use std::cell::RefCell;



enum STATUS{
    INIT,
    RUNNING,
    FAILED,
    CANCELLED,
    SUSPENDED,
    TERMINATED,
    HANDLED
}

trait Task<T>  : Send{
    fn call(self: Box<Self>) -> T;
}

impl<T,F : Send + FnOnce() -> T> Task<T> for F{
    fn call(self : Box<F>) -> T{
        (*self)()
    }
}

//TODO Arc<Mutex<>> o Arc<RwLock<>>
pub struct Future<T>
{
    rec  : Arc<RwLock<Option<Receiver<T>>>>,
    stat : Arc<RwLock<STATUS>>,
    funz : Arc<RwLock<Option<Box<Task<T>>>>>
}


impl <T> Future<T>
where T: Send + 'static
{
    pub fn get(&self) -> T{
        let ref res = self.rec;
        let opt = res.read().unwrap();
        return opt.as_ref().unwrap().recv().unwrap();
    }

    pub fn run(&self) -> (){
        let mut funct1 = self.funz.write().unwrap().take().unwrap();
        let mut status = self.stat.clone();
        let (tx,rx) : (Sender<T>, Receiver<T>) = mpsc::channel();
        *self.rec.write().unwrap() = Some(rx);
        thread::spawn(move || {
            *status.write().unwrap() = STATUS::RUNNING;
            let ret = funct1.call();
            tx.send(ret).unwrap();
            *status.write().unwrap() = STATUS::TERMINATED;
        });
    }

    pub fn new<F>(runnable : F) -> Future<T>  where
        F: FnOnce() -> T, F: Send + Sync + 'static , T: Send + 'static    {
        Future{rec:Arc::new(RwLock::new(Option::None)) , stat: Arc::new(RwLock::new(STATUS::INIT)), funz : Arc::new(RwLock::new(Some(Box::new(runnable))))}
    }
}

unsafe impl <T> Send for Future<T>{}
unsafe impl <T> Sync for Future<T>{}






