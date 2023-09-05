struct Buffer<T>{
    vec_t : Vec<T>,
}


impl <T: std::ops::Add<Output = T>+ std::ops::AddAssign<T>+Copy+ std::default::Default> Buffer<T>{
    fn new(vec_t: Vec<T>) -> Buffer<T>{
        Buffer{vec_t}
    }

    fn sum_t(&self) -> T{
        let mut sum :T = Default::default();
        for i in 0..self.vec_t.len(){
            sum+=self.vec_t[i];
        }
        sum
    }
}
fn main() {
    let v=vec![1,2,3,4,5];
    let buffer : Buffer<i32> = Buffer::new(v);
    println!("The sum of [1,2,3,4,5] is {}",buffer.sum_t());

    let v=vec![1.3,2.2,3.3,4.4,5.1];
    let buffer : Buffer<f32> = Buffer::new(v);
    println!("The sum of [1.3,2.2,3.3,4.4,5.1] is {}",buffer.sum_t());

}
