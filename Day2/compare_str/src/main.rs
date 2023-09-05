use std::cmp::Ordering;

fn compare_str(x: &str,y: &str) -> bool {
    let mut bytes_x = x.bytes();
    let mut bytes_y = y.bytes();

    let min_len = if x.len() > y.len() { y.len() } else { x.len() };
    for _i in 0..min_len{
        let bx=match bytes_x.next(){
            Some(bx) => bx,
            None => return false,
        };
        let by=match bytes_y.next(){
            Some(by) => by,
            None => return false,
        };

        //本来用if实现的, 但是cargo clippy建议我用这个方法..
        //不过这个地方跟整体实现没什么关系
        match bx.cmp(&by){
            Ordering::Greater => return true,
            Ordering::Less => return false,
            Ordering::Equal => continue,
        }

        //原写法
        // if bx > by{
        //     return true;
        // }else if bx < by{
        //     return false;
        // }
        
        
    }
    false
}


fn main() {
    let str1="🤡";
    let str2="✌";
    match compare_str(str1,str2).cmp(&true){
        Ordering::Equal => println!("My compare:  \"{}\"的字典序比\"{}\"大",str1,str2),
        Ordering::Greater | Ordering::Less => println!("My compare:  \"{}\"的字典序小于等于\"{}\"",str1,str2),
    }

    //用std的比较结果
    match str1.cmp(str2) {
        Ordering::Less | Ordering::Equal => println!("std compare: \"{}\"的字典序小于等于\"{}\"",str1,str2),
        Ordering::Greater => println!("std compare: \"{}\"的字典序比\"{}\"大",str1,str2),
    }
}
