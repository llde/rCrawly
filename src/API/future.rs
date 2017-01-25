//TODO Mutex<> o RwLock<>
use std::sync::{Arc, Mutex,RwLock,mpsc};
use std::sync::mpsc::{Sender, Receiver};

trait Task<T>  : Send{
    fn call(self: Box<Self>) -> T;
}

impl<T,F : Send + FnOnce() -> T> Task<T> for F{
    fn call(self : Box<F>) -> T{
        (*self)()
    }
}

enum STATUS{
    INIT,
    SUCCEEDED,
    FAILED,
    RUNNING,
    ABORTED,

}

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
