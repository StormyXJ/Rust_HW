use std::thread;
mod signal;
mod executor;
use colored::*;
use crate::executor::*;

async fn demo(){
    println!("demo's thread id: {:?}",thread::current().id());
    let (tx1,rx1)=async_channel::bounded::<()>(1);
    let (tx2,rx2)=async_channel::bounded::<()>(1);

    // async_std::task::spawn(demo2(tx1));
    Executor::my_spawn(demo2(tx1));
    Executor::my_spawn(demo3(tx2));

    println!("spawn in demo done!");

    let _ =rx1.recv().await;
    // println!("\nrecv from {}","demo2".blue());

    let _ =rx2.recv().await;
    // println!("recv from {}","demo3".green());
    
}

async fn demo2(tx: async_channel::Sender<()>){
    
    println!("start {}","demo2".blue());
    println!("{}'s thread id: {:?}","demo2".blue(),thread::current().id());
    
    let mut _sum=0;
    for i in 0..1000{
        _sum+=i;
    }
    // std::thread::sleep(std::time::Duration::from_secs(5));
    println!("sum in {}: {}","demo2".blue(),_sum);

    let _ =tx.send(()).await;
}

async fn demo3(tx: async_channel::Sender<()>){
    println!("start {}","demo3".green());
    println!("{}'s thread id: {:?}","demo3".green(),thread::current().id());

    let mut _sum=0;
    for i in 1..1000{
        _sum+=i;
    }
    println!("sum in {}: {}","demo3".green(),_sum);

    let _ =tx.send(()).await;
}

fn main() {
    // EX=Executor::new();
    let ex=Executor::new(2);//1 and 2 can make diffenent output
                            //bigger than 2, the output will be various
    ex.block_on(demo());
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