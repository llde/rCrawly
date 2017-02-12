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

#[derive(Copy,Clone)]
enum STATUS{
    INIT,
    SUCCEEDED,
    FAILED,
    RUNNING,
    READED
}



pub enum ERRORS{
    ALREADY_STARTED,
    ALREADY_READED,
    FAILED,
    NOT_STARTED,
    NOT_COMPLETED,
    UNKNOW
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
    pub fn get(&self) -> Result<T,ERRORS>{
        let x = self.stat.read().unwrap().clone();
        if let STATUS::SUCCEEDED = x{
            if let Some(c) = self.rec.write().unwrap().take() {
                *self.stat.write().unwrap() = STATUS::READED;
                Ok(c)
            }
            else{
                panic!();
            }
        }
        else if let STATUS::READED = x{
            Err(ERRORS::ALREADY_READED)
        }
        else if let STATUS::INIT =  x{
            Err(ERRORS::NOT_STARTED)
        }
        else if let STATUS::RUNNING = x{
            Err(ERRORS::NOT_COMPLETED)
        }
        else{
            Err(ERRORS::UNKNOW)
        }
    }

    pub fn is_done(&self) -> bool{
        if let STATUS::SUCCEEDED = *self.stat.read().unwrap(){
            true
        }
        else{
            false
        }
    }


    pub fn run(&self) -> Result<(),ERRORS>{
        if let STATUS::SUCCEEDED = *self.stat.read().unwrap(){
            return Err(ERRORS::ALREADY_STARTED);
        }
        let funct = self.funz.write().unwrap().take().unwrap();
        *self.stat.write().unwrap() = STATUS::RUNNING;
        let ret = funct.call();
        *self.rec.write().unwrap() = Some(ret);
        *self.stat.write().unwrap() = STATUS::SUCCEEDED;
        Ok(())
    }

    pub fn new<F>(runnable : F) -> Future<T>  where
        F: FnOnce() -> T, F: Send + 'static , T: Send + 'static    {
        Future{rec:RwLock::new(Option::None) , stat: RwLock::new(STATUS::INIT), funz : RwLock::new(Some(Box::new(runnable)))}
    }
}

unsafe impl <T> Send for Future<T>{}
unsafe impl <T> Sync for Future<T>{}
