# MyRedis+http

## start

* in ./http_redis
    ```bash
    cargo run
    ```
* The server
    
    *I Didn't copy the redis from the last homework*, you can get it from "https://github.com/StormyXJ/Rust_HW/tree/main/Day5". And use the following command.
    ```bash
    cargo run --bin server
    ```

## use
* get

    "http://127.0.0.1:3000/get/keyname"
* set

    Fill the text in the html.
    "http://127.0.0.1:3000/set"
* del

    Fill the text in the html.
    "http://127.0.0.1:3000/del"

* ping
    * "http://127.0.0.1:3000/ping"
        
        result in PONG
    
    * "http://127.0.0.1:3000/ping/text"

        result in "text"
