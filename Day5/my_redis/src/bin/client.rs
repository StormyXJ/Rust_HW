use lazy_static::lazy_static;
use std::net::SocketAddr;
use std::io::Write;
use my_redis::FliterLayer;
lazy_static! {
    static ref CLIENT: volo_gen::my_redis::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8084".parse().unwrap();
        volo_gen::my_redis::ItemServiceClientBuilder::new("my_redis")
            .layer_outer(FliterLayer)
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main(){
    tracing_subscriber::fmt::init();
    
    loop{
        let mut readin=String::new();
        print!("my_redis> ");
        let _=std::io::stdout().flush();
        std::io::stdin()
            .read_line(&mut readin)
            .expect("\nmy_redis> Failed to read!");
        
        let words=readin.clone();
        let mut words=words.splitn(2, ' ');
        let part1=words.next().unwrap_or("").trim();
        let part2 = words.next().unwrap_or("").trim();
        
        
        tracing::debug!("command:{}", part1);
        match part1.to_lowercase().as_str(){
            "get"=>{
                let resp=CLIENT.get_item(volo_gen::my_redis::GetRequest{
                        key:String::from(part2.trim()).into(),
                }).await;
                match resp{
                    Ok(res)=>{
                        if let Some(ret)=res.ret{
                            println!("my_redis> \"{}\"",ret);
                        }else{
                            println!("my_redis> (nil)");
                        }
                    },
                    Err(e) => tracing::error!("{:?}", e),
                }
            },
            "set"=>{
                let mut part2s=part2.splitn(3,'\"');
                // let _ =part2s.next().unwrap_or("");
                let set_key=part2s.next().unwrap_or("").trim();
                let set_value=part2s.next().unwrap_or("").trim();
                let set_ex=part2s.next().unwrap_or("").trim();
                let ex_num;
                // println!("{}   {}   {}",set_key,set_value,set_ex);
                if set_ex.to_lowercase().contains("ex"){
                    let tmp:Vec<&str>=set_ex.split(" ").collect();
                    ex_num = Some(tmp[1].parse::<i64>().unwrap()*1000);
                }else if set_ex.to_lowercase().contains("px"){
                    let tmp:Vec<&str>=set_ex.split(" ").collect();
                    ex_num = Some(tmp[1].parse::<i64>().unwrap());
                }else{
                    ex_num = None;
                }
                // println!("{}   {}   {}",set_key,set_value,ex_num.unwrap_or(""));
                // let mut part2s=readin.splitn(3,' ');
                // let _ =part2s.next().unwrap_or("");
                // let set_key=part2s.next().unwrap_or("");
                // let set_value=part2s.next().unwrap_or("");
                let resp=CLIENT.set_item(volo_gen::my_redis::SetRequest{key:String::from(set_key).into(), 
                                                                        value:String::from(set_value).into(),
                                                                        ex:ex_num}).await;
                match resp{
                    Ok(res) =>{
                        if res.ret{
                            println!("my_redis> \"OK\"");
                        }
                        // tracing::info!("{:?}", res);
                    } 
                    Err(e) => tracing::error!("{:?}", e),
                }
            },
            "ping"=>{
                let req:volo_gen::my_redis::PingRequest;
                // println!("{}",part2.is_empty());
                match part2.is_empty(){
                    true=>{
                        req=volo_gen::my_redis::PingRequest{key: Some(String::from("PONG").into())};
                    },
                    false=>{
                        req=volo_gen::my_redis::PingRequest{key: Some(String::from(part2).into())};
                    }
                }
                let resp=CLIENT.ping_item(req).await;
                match resp{
                    Ok(res) =>{
                        println!("my_redis> {}",res.value);
                        // tracing::info!("{:?}", res);
                    }, 
                    Err(e) => tracing::error!("{:?}", e),
                }
            },
            "del"=>{
                let req=volo_gen::my_redis::DelRequest{key:String::from(part2).into()};
                let resp=CLIENT.del_item(req).await;
                match resp{
                    Ok(res)=>{
                        println!("my_redis> {}",res.num);
                    },
                    Err(e) => tracing::error!("{:?}", e),
                }
            },
            "exit"=>{
                break;
            }
            _ =>{
                tracing::error!("No such operation or used mistakenly");
                continue;
            }
        }
    }
}