fn main() {
    let vec_char = vec!['a', 'b', 'c', 'd', 'z'];
    let iter = vec_char.iter().map(|c| char::from_u32(if (*c as u32)+1 > 122 {97} else{(*c as u32)+1}));
    let res: Vec<char>= match iter.collect(){
        Some(v) => v,
        None => return
    };
    println!("\nThe 'z' will be converted to 'a'\n{:?}=>{:?}",vec_char,res);
}

