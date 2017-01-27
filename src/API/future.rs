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
    rec  : RwLock<Option<T>>,
    stat : RwLock<STATUS>,
    funz : RwLock<Option<Box<Task<T>>>>
}


impl <T> Future<T>
where T: Send + 'static
{
    pub fn get(&self) -> Option<T>{
        self.rec.write().unwrap().take()
    }

    pub fn is_done(&self) -> bool{
        if let STATUS::SUCCEEDED = *self.stat.read().unwrap(){
            true
        }
        else{
            false
        }
    }


    pub fn run(&self) -> (){
        let mut funct1 = self.funz.write().unwrap().take().unwrap();
        *self.stat.write().unwrap() = STATUS::RUNNING;
        let ret = funct1.call();
        *self.rec.write().unwrap() = Some(ret);
        *self.stat.write().unwrap() = STATUS::SUCCEEDED;
    }

    pub fn new<F>(runnable : F) -> Future<T>  where
        F: FnOnce() -> T, F: Send + 'static , T: Send + 'static    {
        Future{rec:RwLock::new(Option::None) , stat: RwLock::new(STATUS::INIT), funz : RwLock::new(Some(Box::new(runnable)))}
    }
}

unsafe impl <T> Send for Future<T>{}
unsafe impl <T> Sync for Future<T>{}
