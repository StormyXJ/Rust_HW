use std::{future::Future, 
          task::{RawWakerVTable,Waker,RawWaker,Context,Wake,Poll}, 
          sync::{Mutex,Condvar, Arc},
          cell::RefCell,
          collections::VecDeque};
use futures::{future::BoxFuture};
use scoped_tls::scoped_thread_local;

scoped_thread_local!(static SIGNAL: Arc<Signal>);
scoped_thread_local!(static RUNNABLE: Mutex<VecDeque<Arc<Task>>>);
// thread_local!(static SIGNAL: Arc<Signal>=Arc::new(Signal::new()));
// thread_local! {static RUNNABLE: Mutex<VecDeque<Arc<Task>>> = Mutex::new(VecDeque::new());}
enum State{
    Empty,
    Waiting,
    Notified,
}
struct Signal{
    state: Mutex<State>,
    cond: Condvar,
}
impl Signal{
    fn new()->Signal{
        Signal{
            state:Mutex::new(State::Empty),
            cond:Condvar::new(),    
        }
        
    }
    fn wait(&self){
        let mut state=self.state.lock().unwrap();
        match *state{
            State::Notified => *state=State::Empty,
            State::Waiting =>{
                panic!("miltiple wait");
            },
            State::Empty=>{
                *state=State::Waiting;
                while let State::Waiting=*state{
                    state=self.cond.wait(state).unwrap();
                }
            }
        }
    }

    fn notify(&self){
        let mut state=self.state.lock().unwrap();
        match *state{
            State::Notified => {}
            State::Empty => *state = State::Notified,
            State::Waiting=>{
                *state=State::Empty;
                self.cond.notify_one();
            }
        }
    }
}

impl Wake for Signal{
    fn wake(self: Arc<Self>){
        self.notify();
    }
}

struct Task{
    future: RefCell<BoxFuture<'static,()>>,
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

fn block_on<F:Future>(future:F)->F::Output{
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
// struct Demo;
// impl Future for Demo{
//     type Output = ();
//     fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>)->std::task::Poll<Self::Output>{
//         println!("hello world");
//         std::task::Poll::Ready(())
//     }
// }

// fn dummy_waker() -> Waker{
//     static DATA: ()=();
//     unsafe{
//         Waker::from_raw(RawWaker::new(&DATA,&VTABLE))
//     }
// }

// const VTABLE: RawWakerVTable=
//     RawWakerVTable::new(vtable_clone,vtable_wake,vtable_wake_by_ref,vtable_drop);

// unsafe fn vtable_clone (_p:*const())->RawWaker{
//     RawWaker::new(_p,&VTABLE)
// }

// unsafe fn vtable_wake(_p:*const()){}
// unsafe fn vtable_wake_by_ref(_p:*const()){}
// unsafe fn vtable_drop(_p:*const()){}

// fn block_on<F:Future>(future:F)->F::Output{
//     let mut fut=std::pin::pin!(future);
//     let waker=dummy_waker();
//     let mut cx=Context::from_waker(&waker);
//     loop{
//         if let std::task::Poll::Ready(output)=fut.as_mut().poll(&mut cx){
//             return output;
//         }
//     }
// }
// fn block_on<F:Future>(future:F)->F::Output{
//     let mut fut=std::pin::pin!(future);
//     let signal =Arc::new(Signal::new());
//     let waker=Waker::from(signal.clone());

//     let mut cx=Context::from_waker(&waker);
//     loop{
//         if let Poll::Ready(output)=fut.as_mut().poll(&mut cx){
//             return output;
//         }
//         signal.wait();
//     }
// }


async fn demo(){
    let (tx,rx)=async_channel::bounded::<()>(1);
    // std::thread::spawn(move||{
    //     block_on(demo2(tx))
    // });
    async_std::task::spawn(demo2(tx));
    println!("hello world");
    let _ =rx.recv().await;
    
    // std::thread::spawn(move||{
    //     std::thread::sleep(std::time::Duration::from_secs(5));
    //     tx.send_blocking(())
    // });
    // let _ = rx.recv().await;
    // println!("hello world");
    
}

async fn demo2(tx: async_channel::Sender<()>){
    println!("hello world2");
    let _ =tx.send(()).await;
}

fn main() {
    block_on(demo());
}
