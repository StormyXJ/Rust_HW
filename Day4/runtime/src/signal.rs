use std::{task::{Wake}, 
    sync::{Mutex,Condvar, Arc}};

pub enum State{
    Empty,
    Waiting,
    Notified,
}
pub struct Signal{
    state: Mutex<State>,
    cond: Condvar,
}
impl Signal{
    pub fn new()->Signal{
        Signal{
            state:Mutex::new(State::Empty),
            cond:Condvar::new(),    
        }
        
    }
    pub fn wait(&self){
        // println!("signal::wait");
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

    pub fn notify(&self){
        // println!("signal::notify");
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
        println!("\nwake from signal\n");
        self.notify();
    }
}