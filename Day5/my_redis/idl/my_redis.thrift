// DEL key [key ...]
//Removes the specified keys. A key is *ignored* if it does not exist.
//return :The number of keys that were removed in integer.

struct Item{
    1: required string _key,
    2: required string _value,
}

struct PingRequest {
    1: optional string _key,
}

struct PingResponse {
    1: required string _value,
}

struct DelRequest {
    1: required string _key,
}

struct DelResponse {
    1: required i32 _num,
}

struct SetRequest{
    1: required string _key,
    2: required string _value,

    3: optional i64 _EX, 
}

struct SetResponse{
    1: required bool _ret,
}

struct GetRequest{
    1: required string _key,
}

struct GetResponse{
    1: optional string _ret,
}

struct SubscribeRequest{
    1: required string channels,
}

struct SubscribeResponse{
    1: required string success,
}


service ItemService {
    PingResponse PingItem(1: PingRequest req),
    DelResponse DelItem(1: DelRequest req),
    SetResponse SetItem(1: SetRequest req),
    GetResponse GetItem(1: GetRequest req),
    SubscribeResponse SubChannel(1: SubscribeRequest req),
}