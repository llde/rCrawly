use std::sync::{Arc,Mutex,RwLock};
use std::collections::VecDeque;
use std::thread;
use API::Future;

enum STATUS{
    RUNNING,
    SHUTDOWN,
    PAUSED,
}


pub enum ERRORS{
    IS_SHUTDOWN
}



pub struct ThreadPoolExecutor<T>{
    workers : Arc<Mutex<VecDeque<Arc<Future<T>>>>>,
    status: Arc<Mutex<STATUS>>,
    task_count : u32,
    num_thread: u32,
    current_threads : Arc<RwLock<u32>>,
}
impl <T> ThreadPoolExecutor<T> where T: Send + 'static{
    //TODO AVOID LOCK POISON
    //TODO TRY TO AVOID THE T PARAMETER
    pub fn new(num_threads : u32) -> ThreadPoolExecutor<T>{
        ThreadPoolExecutor{workers: Arc::new(Mutex::new(VecDeque::new())), status : Arc::new(Mutex::new(STATUS::RUNNING)), task_count : 0, num_thread : num_threads, current_threads:Arc::new(RwLock::new(0))}
    }


    pub fn submit<F>(&self, function : F ) -> Result<Arc<Future<T>>,ERRORS>
        where F : FnOnce() -> T + Send +'static {
        if let STATUS::SHUTDOWN = *self.lock().unwrap(){
            return Err(ERRORS::IS_SHUTDOWN);
        }
        let fut = Arc::new(Future::new(function));
        self.workers.lock().unwrap().push_back(fut.clone());
        println!("{} {}", *self.current_threads.read().unwrap(), self.num_thread);
        if *self.current_threads.read().unwrap() < self.num_thread{
            {
                let mut guard = self.current_threads.write().unwrap();
                *guard += 1;
            }
            let arc = self.workers.clone();
            let arc_curr_thread = self.current_threads.clone();
            thread::spawn(move ||{
                loop{
                    let option;
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
        Ok(fut)
    }

    pub fn shutdown(&self, now : bool) -> Result<(),ERRORS> {
        if let STATUS::SHUTDOWN = *self.status.lock().unwrap(){
            return Err(ERRORS::IS_SHUTDOWN);
        }
        *self.status.lock().unwrap() = STATUS::SHUTDOWN;
        if now {
            self.workers.lock().unwrap().clear();
        }
        Ok(())
    }
}

