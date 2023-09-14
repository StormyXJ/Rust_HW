use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Form, Router,
};
use url::form_urlencoded;
use std::net::SocketAddr;
use volo_gen::my_redis::{ItemServiceClient, ItemServiceClientBuilder};
#[tokio::main]
async fn main() {
    let redis_addr:SocketAddr="127.0.0.1:8081".parse().unwrap();
    let redis_client = ItemServiceClientBuilder::new("redis_client")
                            .address(redis_addr)
                            .build();
    let addr=SocketAddr::from(([127,0,0,1],3000));
    let app = Router::new()
        .route("/ping/:txt", get(pingtxt).with_state(redis_client.clone()))  // http://127.0.0.1:3000
        .route("/ping",get(ping).with_state(redis_client.clone()))
        .route("/get/:key", get(getvalue).with_state(redis_client.clone())) // http://127.0.0.1:3000/foo
        .route("/del", 
        get(show_del_fmt).post(delkey).with_state(redis_client.clone())) // http://127.0.0.1:3000/foo/bar
        .route("/set",
        get(show_set_fmt).post(setkey).with_state(redis_client.clone()));
    // let addr = SocketAddr::from(([127,0,0,1],3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn pingtxt(State(redis_cli): State<ItemServiceClient>,Path(txt):Path<String>) -> (StatusCode,String) {
    let req=volo_gen::my_redis::PingRequest{key: Some(txt.into())};
    let resp=redis_cli.ping_item(req).await;
    match resp{
        Ok(res) =>{
            (StatusCode::OK,res.value.to_string())
            // println!("{}",res.value);
            // tracing::info!("{:?}", res);
        }, 
        Err(e) => (StatusCode::OK,"Something went wrong".to_string())
    }
}
async fn ping(State(redis_cli): State<ItemServiceClient>) -> (StatusCode,String){
    let req=volo_gen::my_redis::PingRequest{key: Some(String::from("PONG").into())};
    let resp=redis_cli.ping_item(req).await;
    match resp{
        Ok(res) =>{
            (StatusCode::OK,res.value.to_string())
            // println!("{}",res.value);
            // tracing::info!("{:?}", res);
        }, 
        Err(e) => (StatusCode::OK,"Something went wrong".to_string())
    }
    // (StatusCode::OK,String::from("PONG"))
}
async fn getvalue(State(redis_cli):State<ItemServiceClient>,Path(key):Path<String>) -> (StatusCode,String) {
    let resp=redis_cli.get_item(volo_gen::my_redis::GetRequest{
            key:key.into(),
    }).await;
    match resp{
        Ok(res)=>{
            if let Some(ret)=res.ret{
                (StatusCode::OK,String::from(ret))
                // println!("\"{}\"",ret);
            }else{
                (StatusCode::OK,String::from("(nil)"))
                // println!("(nil)");
            }
        },
        Err(e) => {
            (StatusCode::OK,"Something went wrong".to_string())
        },
    }
    
    
}
async fn delkey(State(redis_cli): State<ItemServiceClient>, key: String) -> (StatusCode,String) {
    let resp=redis_cli.del_item(volo_gen::my_redis::DelRequest{
        key:key.into()
    }).await;
    match resp{
        Ok(res) =>{
            (StatusCode::OK, res.num.to_string())
        },
        Err(e)=>{
            (StatusCode::OK, "Something went wrong".to_string())
        }
    }
}
async fn setkey(State(redis_cli):State<ItemServiceClient>, body: String)-> (StatusCode,String){
    let parsed: Vec<(String, String)> = form_urlencoded::parse(body.as_bytes())
        .into_owned()
        .collect();
    
    let mut key = String::new();
    let mut value = String::new();

    for (k, v) in parsed.iter() {
        if k == "key" {
            key = v.to_string();
        } else if k == "value" {
            value = v.to_string();
        }
    }
    let resp=redis_cli.set_item(volo_gen::my_redis::SetRequest{key:String::from(key).into(), 
                                                            value:String::from(value).into(),
                                                            ex:None}).await;
    match resp{
        Ok(res) =>{
            if res.ret{
                (StatusCode::OK,String::from("OK"))
            }else{
                (StatusCode::OK,String::from("Something went wrong"))
            }
            // tracing::info!("{:?}", res);
        },
        Err(e) => (StatusCode::OK,String::from("Something went wrong"))
    }
    // (StatusCode::OK,String::from("PONG"))
}

async fn show_set_fmt() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/set" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <label for="value">
                        Enter value:
                        <input type="text" name="value">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn show_del_fmt() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/del" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}



