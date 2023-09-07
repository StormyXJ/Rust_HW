// use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use std::ptr::NonNull;
// #[derive(Debug)]
struct RcInner<T> {
    count: RefCell<usize>,
    value: T,
}
struct MyRc<T> {
    data: Box<NonNull<RcInner<T>>>,
}

impl<T: Clone> MyRc<T> {
    fn new(x: T) -> MyRc<T> {
        // let ref_cell=RefCell::new((x,1 as usize));
        // let non_ref=NonNull::new(Box::into_raw(Box::new(ref_cell)));
        // MyRc{data: Box::new(NonNull::new(Box::into_raw(Box::new(RefCell::new((x,1 as usize))))).expect("Invalid"))}
        MyRc {
            data: Box::new(
                NonNull::new(Box::into_raw(Box::new({
                    RcInner {
                        count: RefCell::new(1 as usize),
                        value: x,
                    }
                })))
                .unwrap(),
            ),
        }
    }

    fn clone(&mut self) -> Self {
        unsafe {
            *(*self.data).as_ref().count.borrow_mut() += 1;
        }

        MyRc {
            data: Box::new(*self.data.clone()),
        }
    }

    fn strong_conut(&self) -> usize {
        unsafe { *(*self.data).as_ref().count.borrow() }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.data).as_ref().value }

        // println!("data from deref:{}", *self.data);
        // &*(*self.data).borrow()
    }
}

// impl<T: std::fmt::Display> fmt::Display for MyRc<T>{
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//             write!(f, "{}",*self.data)

//     }
// }

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            if *(*self.data).as_ref().count.borrow() != 0 {
                *(*self.data).as_ref().count.borrow_mut() -= 1;
            } else {
                let _ = (*self.data).as_ref().count;
                let _ = (*self.data).as_ptr();
                let _ = self.data;
            }
        }
    }
}

fn main() {
    let mut rc1 = MyRc::new(5);
    // println!("rc1's data is: {:?}",rc1);
    println!("rc1={}", *rc1);
    println!("cnt1={}", MyRc::strong_conut(&rc1));
    let mut rc2 = MyRc::clone(&mut rc1);
    println! {"cnt1={},cnt2={},rc1={},rc2={}",rc1.strong_conut(),rc2.strong_conut(),*rc1,*rc2};
    {
        let rc3 = MyRc::clone(&mut rc2);
        // println!("rc2's data is: {:?}",rc2);
        println!("rc3={}", *rc3);
        println!(
            "cnt1={},cnt2={},cnt3={}",
            rc1.strong_conut(),
            rc2.strong_conut(),
            rc3.strong_conut()
        );
    }
    println! {"cnt1={},cnt2={}",rc1.strong_conut(),rc2.strong_conut()};
}
