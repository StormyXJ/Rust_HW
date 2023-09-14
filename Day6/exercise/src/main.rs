// use reqwest::blocking::Client;
use std::io::Read;
use error_chain::error_chain;
use colored::*;

// use hyper::StatusCode;
error_chain!{
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
fn main() -> Result<()>{
    // let client = Client::new();
    // let resp=client.get("https://www.baidu.com");
    // let resp=reqwest::blocking::get("")
    // println!("{:?}",resp);
    // let url="http://httpbin.org/status/404";
    let url ="http://zjuers.com/black.html";
    let mut resp = reqwest::blocking::get(url)?;
    let mut body =String::new();
    resp.read_to_string(&mut body)?;
    // let mut body =resp.text()?;
    println!("{}",url);
    let status_text=resp.status().canonical_reason().unwrap_or("Unexpected status code");
    if resp.status().as_u16()<300{
        println!("{:?}   {} {}",resp.version(),
                          resp.status().as_str().green(),
                          status_text.green()
                            );
    }else{
        println!("{:?}   {} {}",resp.version(),
                          resp.status().as_str().red(),
                          status_text.red()
                            );
    }
    
    for (key,value) in resp.headers() {
        println!("{}: {:?}", key.to_string().blue(),value);
    }
    let mut bind =reqwest::blocking::Body::from(body);
    let mut body = bind.buffer();
    for tmp in body{
        println!("{:?}",tmp);
    }
    // println!("{:?}",body);
    Ok(())
}


