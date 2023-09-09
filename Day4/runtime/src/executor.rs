use std::{future::Future, 
    task::{Waker,Context,Wake,Poll}, 
    sync::{Mutex, Arc, mpsc::{Sender, Receiver, channel}},
    cell::RefCell,
    collections::VecDeque,
    cmp::Ordering, thread::JoinHandle};
use futures::{future::{LocalBoxFuture}, FutureExt};
use scoped_tls::scoped_thread_local;


use crate::signal::*;

scoped_thread_local!(pub(crate) static EX: Executor);
// scoped_thread_local!(static SIGNAL: Arc<Signal>);
// scoped_thread_local!(static RUNNABLE: Mutex<VecDeque<Arc<Task>>>);
// thread_local!(static SIGNAL: Arc<Signal>=Arc::new(Signal::new()));
// thread_local! {static RUNNABLE: Mutex<VecDeque<Arc<Task>>> = Mutex::new(VecDeque::new());}


pub struct Task{
    future: RefCell<LocalBoxFuture<'static,()>>,
    signal: Arc<Signal>,
}

unsafe impl Send for Task{}
unsafe impl Sync for Task{}

impl Wake for Task{
    fn wake(self: Arc<Self>){
        EX.with(|ex| ex.run_queue.push(self.clone()));
        // RUNNABLE.with(|runnable| runnable.lock().unwrap().push_back(self.clone()));
        self.signal.notify();
    }
}

pub struct RunQueue{
    queue: Mutex<VecDeque<Arc<Task>>>,
}

impl RunQueue {
    pub fn new()->Self{
        RunQueue { queue: Mutex::new(VecDeque::with_capacity(1024)) }
    }

    pub fn push(&self,task:Arc<Task>){
        self.queue.lock().unwrap().push_back(task);
    }

    pub fn pop(&self)->Option<Arc<Task>>{
        self.queue.lock().unwrap().pop_front()
    }
}

pub struct Executor{
    run_queue: RunQueue,
    threads_pool:ThreadPool,
}

impl Executor{
    pub fn new(size:usize)->Self{
        Executor{run_queue:RunQueue::new() ,threads_pool:ThreadPool::new(size)}
    }
    pub fn block_on<F:Future>(&self,future:F)->F::Output{
        let signal =Arc::new(Signal::new());
        let waker=Waker::from(signal.clone());
        let mut cx=Context::from_waker(&waker);
        EX.set(self,||{
            let mut main_fut=std::pin::pin!(future);
            loop{
                if let Poll::Ready(output)=main_fut.as_mut().poll(&mut cx){
                    return output;
                }
                while  self.run_queue.queue.lock().unwrap().len() > 0{
                    let task=self.run_queue.pop();
                    self.threads_pool.sender.send(task).unwrap();
                    // let waker=Waker::from(task.clone());
                    // let mut cx=Context::from_waker(&waker);
                    // let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                }
                signal.wait();
            }
        })
    }

    pub fn my_spawn(fut: impl Future<Output=()>+'static){
        let t = Arc::new(Task{
            future:RefCell::new(fut.boxed_local()),
            signal:Arc::new(Signal::new()),
        });
        // RUNNABLE.lock().unwrap().push_back(t);
        EX.with(|ex|{
            ex.run_queue.push(t);
        })

    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new(1)
    }
}


struct ThreadPool{
    worker_pool: Vec<Worker>,
    sender: Sender<Option<Arc<Task>>>,
}

impl ThreadPool {
    pub fn new(size: usize)->Self{
        match size.cmp(&0){
            Ordering::Less | Ordering::Equal=>{
                panic!("There must be at least one thread!");
            } 
            Ordering::Greater=>{
                let (sender, receiver)=channel::<Option<Arc<Task>>>();
                let receiver=Arc::new(Mutex::new(receiver));

                let mut worker_pool=Vec::with_capacity(size);
                for id in 0..size{
                    worker_pool.push(Worker::new(id,Arc::clone(&receiver)));
                }
                ThreadPool{
                    worker_pool,
                    sender,
                }
            }
        }
    }

    pub fn join_all(&mut self){
        for _worker in &self.worker_pool{
            self.sender.send(Option::None).unwrap();
        }
        for worker in &mut self.worker_pool {
            match worker.handle.take(){
                Some(handle)=>handle.join().unwrap(),
                None => continue,
            }
            // if let Some(handle) = worker.handle.take() {
            //     handle.join().unwrap();
            // }
        }
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        self.join_all();
    }
}

struct Worker{
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker{
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Option<Arc<Task>>>>>) ->Self{
        let handle= std::thread::spawn(move||{
            loop{
                let task=receiver.lock().unwrap().recv().unwrap();
                match task{
                    Some(task)=>{
                        let waker=Waker::from(task.clone());
                        let mut cx=Context::from_waker(&waker);
                        let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                    },
                    None =>{
                        break;
                    }
                }
                
                
            }
        });

        Worker { id: id, handle: Some(handle) }
    }
}