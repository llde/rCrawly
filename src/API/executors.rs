
use std::cmp::{Ord, Ordering};
use std::option;
use std::collections::binary_heap::BinaryHeap;
use std::marker::{Sync,Send};
use std::marker::PhantomData;
use std::sync::{atomic, Arc, Condvar, Mutex};
use std::thread::{JoinHandle, spawn, Thread};
use std::any::{Any, TypeId};
use std::mem;

enum StatusExecutor{
    INIT,
    RUNNING,
    SUSPENDED,
    SHUTDOWN
}

//TODO completare forse?
pub trait Executor<T>{
    fn execute(&mut self ,funct : Box<FnMut() -> T>) -> Box<Future<T>>;

    fn execute_with_priority(&mut self,funct : Box<FnMut() -> T>, priority : u8) -> Box<Future<T>>;

    fn clean(&mut self) -> ();

    fn thread_active_queue(&self) -> &Mutex<Vec<Future<T>>>;

    fn thread_blocked_queue(&self) -> &Mutex<Vec<Future<T>>>;

    fn pause(&mut self) ->();

    fn resume(&mut self) ->();

    fn shutdown(&mut self) ->();

    fn setMaxSize(&mut self, num : u32) -> ();
}


pub struct Executors{}  //Utility for Executor creations

impl Executors{
    fn threadPoolExecutor(x : u32){
    }

}

//Mutex or RWLock
pub struct ThreadPoolExecutor<T>{
    maxThreads : Mutex<u32>,
    usedThread : Mutex<u32>,
    status : Mutex<StatusExecutor>,
    threads : Mutex<Vec<Box<Thread>>>,
}
impl <T> ThreadPoolExecutor<T>{
    pub fn new(maxThread : u32)  -> ThreadPoolExecutor<T>{
        ThreadPoolExecutor{maxThreads : Mutex::new(maxThread) ,usedThread : Mutex::new(0),status : Mutex::new(StatusExecutor::INIT), threads : Mutex::new(Vec::new())}
    }
}


impl <T> Executor<T> for ThreadPoolExecutor<T>{
    fn execute(&mut self, funct : Box<FnMut() -> T>) -> Box<Future<T>>{
        unimplemented!()
    }

    fn execute_with_priority(&mut self,funct : Box<FnMut() -> T>, prtiority : u8) -> Box<Future<T>>{self.execute(funct)}

    fn clean(&mut self) -> (){unimplemented!()}

    fn thread_active_queue(&self) ->  &Mutex<Vec<Future<T>>>{ unimplemented!()}

    fn thread_blocked_queue(&self) -> &Mutex<Vec<Future<T>>>{ unimplemented!()}

    fn pause(&mut self){unimplemented!()}

    fn resume(&mut self){unimplemented!()}

    fn shutdown(&mut self){unimplemented!()}

    fn setMaxSize(&mut self, num : u32) -> (){
        *self.maxThreads.get_mut().unwrap() = num;
    }
}

unsafe impl <T> Sync for ThreadPoolExecutor<T>{}
unsafe impl <T> Send for ThreadPoolExecutor<T>{}

*/