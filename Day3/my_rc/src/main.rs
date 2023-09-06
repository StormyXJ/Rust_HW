// use std::rc::Rc;
use std::ops::Deref;
use std::fmt;

// #[derive(Debug)]
struct MyRc<T>{
    data: *mut T,
    count: usize,
}

impl<T> MyRc<T>{
    fn new(x: T)-> MyRc<T>{
        MyRc{data: Box::into_raw(Box::new(x)),
             count: 1}
    } 
    
    fn clone(&mut self) -> Self{
        self.count += 1;

        MyRc{data: self.data, count: self.count}
    }

    fn strong_conut(&self) -> usize{
        self.count
    }
}

impl<T> Deref for MyRc<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target{
        
        unsafe{
            // println!("data from deref:{}", *self.data);
            &*self.data
        }
    }
}

impl<T: std::fmt::Display> fmt::Display for MyRc<T>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe{
            write!(f, "{}",*self.data)
        }
        
    }
}

impl<T> Drop for MyRc<T>{

    fn drop(&mut self) {
        if self.count != 0 {
            self.count -= 1;
        }else{
            unsafe{
                let _ = Box::from_raw(self.data);
            }
        }
    }
}

fn main() {
    let mut rc1=MyRc::new(5);
    // println!("rc1's data is: {:?}",rc1);
    println!("rc1's data is: {}",rc1);
    println!("current count is {}",MyRc::strong_conut(&rc1));
    {
        println!("----let rc2=rc1.clone()----");
        let rc2=MyRc::clone(&mut rc1);
        // println!("rc2's data is: {:?}",rc2);
        println!("rc2's data is: {}",rc2);
        println!("current count is {}",rc1.strong_conut());
    }
}
