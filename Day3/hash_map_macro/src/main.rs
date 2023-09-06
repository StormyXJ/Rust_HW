use std::collections::HashMap;
macro_rules! hash_map {
    ($($key:expr=>$value:expr),*)=>{
        {
            let mut tmp_map = HashMap::new();
            $(
                tmp_map.insert($key,$value);
            )*
            tmp_map
        }
    }
}

fn main() {
    let t_map = hash_map!{
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    println!("{:?}",t_map);
}
