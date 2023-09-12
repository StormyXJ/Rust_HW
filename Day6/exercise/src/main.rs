use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
    let resp=client.get("www.baidu.com");
    println!("{:?}",resp);
}
