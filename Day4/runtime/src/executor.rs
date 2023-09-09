use std::{future::Future, 
    task::{Waker,Context,Wake,Poll}, 
    sync::{Mutex, Arc},
    cell::RefCell,
    collections::VecDeque,};
use futures::{future::{LocalBoxFuture}, FutureExt};
use scoped_tls::scoped_thread_local;


use crate::signal::*;

scoped_thread_local!(static SIGNAL: Arc<Signal>);
scoped_thread_local!(static RUNNABLE: Mutex<VecDeque<Arc<Task>>>);
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
        RUNNABLE.with(|runnable| runnable.lock().unwrap().push_back(self.clone()));
        self.signal.notify();
    }
}

pub fn block_on<F:Future>(future:F)->F::Output{
    let mut main_fut=std::pin::pin!(future);
    let signal =Arc::new(Signal::new());
    let waker=Waker::from(signal.clone());
    let mut cx=Context::from_waker(&waker);
    let runnable=Mutex::new(VecDeque::with_capacity(1024));
    SIGNAL.set(&signal, ||{
        RUNNABLE.set(&runnable,||{
            loop{
                if let Poll::Ready(output)=main_fut.as_mut().poll(&mut cx){
                    return output;
                }
                // println!("runnable len from blockon:{}",runnable.lock().unwrap().len());
                while let Some(task)=runnable.lock().unwrap().pop_front(){
                    let waker=Waker::from(task.clone());
                    let mut cx=Context::from_waker(&waker);
                    let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                }
                signal.wait();
            }
        })
    })
}

pub fn my_spawn(fut: impl Future<Output=()>+'static){
    let t = Arc::new(Task{
        future:RefCell::new(fut.boxed_local()),
        signal:Arc::new(Signal::new()),
    });
    // RUNNABLE.lock().unwrap().push_back(t);
    RUNNABLE.with(|runnable| {
        runnable.lock().unwrap().push_back(t)
    });

}