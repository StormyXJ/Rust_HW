#![feature(impl_trait_in_assoc_type)]
use lazy_static::lazy_static;
use std::{collections::HashMap,
		  sync::Mutex,
		  time::{SystemTime, UNIX_EPOCH}};
use anyhow::anyhow;
lazy_static! {
	static ref GLOBAL_HASH_MAP: Mutex<HashMap<String, (String,i64)>> = Mutex::new(HashMap::new());
}
pub struct S;

#[volo::async_trait]
impl volo_gen::my_redis::ItemService for S {
	async fn ping_item(&self, _req: volo_gen::my_redis::PingRequest) 
	-> ::core::result::Result<volo_gen::my_redis::PingResponse, ::volo_thrift::AnyhowError>
	{	
		let resp:volo_gen::my_redis::PingResponse;

		if let Some(outstr)=_req.key{
			// println!("from lib {}",outstr);
			resp=volo_gen::my_redis::PingResponse{value: outstr};
			return Ok(resp);
		}
		Ok(Default::default())
	}

	async fn del_item(&self, _req: volo_gen::my_redis::DelRequest) 
	-> ::core::result::Result<volo_gen::my_redis::DelResponse, ::volo_thrift::AnyhowError>
	{	
		let keys=_req.key.split_whitespace();
		let mut cnt:i32=0;
		for k in keys{
			if GLOBAL_HASH_MAP.lock().unwrap().contains_key(k) {
				GLOBAL_HASH_MAP.lock().unwrap().remove(k);
				cnt+=1;
			}
		}
		Ok(volo_gen::my_redis::DelResponse{num:cnt})
	}

	async fn set_item(&self, _req: volo_gen::my_redis::SetRequest) 
	-> ::core::result::Result<volo_gen::my_redis::SetResponse, ::volo_thrift::AnyhowError>
	{	
		if let Some(time)=_req.ex{
			let dead_time = time+SystemTime::now()
							.duration_since(UNIX_EPOCH)
							.unwrap()
							.as_millis() as i64;
			GLOBAL_HASH_MAP.lock().unwrap().insert(_req.key.to_string(), (_req.value.trim().to_string(),dead_time));
		}else{
			GLOBAL_HASH_MAP.lock().unwrap().insert(_req.key.to_string(), (_req.value.trim().to_string(),0));
		}
		
		Ok(volo_gen::my_redis::SetResponse{ret: true})
	}

	async fn get_item(&self, _req: volo_gen::my_redis::GetRequest) 
	-> ::core::result::Result<volo_gen::my_redis::GetResponse, ::volo_thrift::AnyhowError>
	{	
		let mut local_map=GLOBAL_HASH_MAP.lock().unwrap();
		if let Some(value)=local_map.get(&_req.key.to_string()) {
			// println!("cur {}",SystemTime::now()
			// 				.duration_since(UNIX_EPOCH)
			// 				.unwrap()
			// 				.as_millis() as i64);
			// println!("life {}",value.1);
			if value.1<SystemTime::now()
						.duration_since(UNIX_EPOCH)
						.unwrap()
						.as_millis() as i64 && value.1!=0{
							local_map.remove(&_req.key.to_string());
						}else{
							return Ok(volo_gen::my_redis::GetResponse{ret:Some(String::from(value.0.clone()).into())});
						}
			
		}
		Ok(volo_gen::my_redis::GetResponse { ret: None})
	}
}


#[derive(Clone)]
pub struct CmdFliter<S>(S);

#[volo::service]
impl<Cx,Req,S> volo::Service<Cx,Req> for CmdFliter<S>
where
	Req: std::fmt::Debug +Send+'static,
	S: Send+'static+volo::Service<Cx,Req>+Sync,
	S::Response: std::fmt::Debug,
	S::Error: std::fmt::Debug,
	Cx: Send+'static,
	anyhow::Error: Into<S::Error>
{
	async fn call(&self,cx:&mut Cx,req:Req)->Result<S::Response,S::Error> {
		let req_msg = format!("{:?}",req);
		if !(req_msg.contains("114514") || req_msg.contains("1919810")){
			self.0.call(cx,req).await
		}else{
			tracing::error!("The input is nasty!");
			Err(anyhow!("It's smelly~").into())
			// Err(S::Error::from(anyhow::Error::msg("It's smelly~")))
			// self.0.call(cx,req).await/////////////////
			
		}
	}
}

pub struct FliterLayer;

impl<S> volo::Layer<S> for FliterLayer{
	type Service = CmdFliter<S>;

	fn layer(self,inner: S)->Self::Service{
		CmdFliter (inner)
	}
}