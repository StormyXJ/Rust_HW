#![feature(impl_trait_in_assoc_type)]
use lazy_static::lazy_static;
use std::{collections::HashMap,
		  sync::Mutex,
		  time::{SystemTime, UNIX_EPOCH},
		  };
use tokio::sync::broadcast;
use anyhow::anyhow;
lazy_static! {
	static ref GLOBAL_HASH_MAP: Mutex<HashMap<String, (String,i64)>> = Mutex::new(HashMap::new());
}

lazy_static! {
	static ref GLOBAL_CHANNEL: Mutex<HashMap<String, Vec<broadcast::Sender<String>>>> = Mutex::new(HashMap::new());
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

	async fn sub_channel(&self, _req: volo_gen::my_redis::SubscribeRequest) 
	-> ::core::result::Result<volo_gen::my_redis::SubscribeResponse, ::volo_thrift::AnyhowError>
	{	
		let channels = String::from(_req.channels);
		let (tx,mut rx)=broadcast::channel(10);
		{
			let mut local_map = GLOBAL_CHANNEL.lock().unwrap();
			if let Some(_)=local_map.get(&channels){
				let entry = local_map.entry(channels).or_insert(vec![]);
				entry.push(tx);
				// local_map.get(&channels).unwrap().borrow_mut().push((tx,rx));
				// oldChannel.push((tx,rx));
				// local_map.insert(channels.to_string(),oldChannel);
			}else{
				local_map.insert(channels.to_string(),vec![tx]);
			}
			drop(local_map);
		}
		// let mut msg=String::new();
		let thread =tokio::task::spawn(async move {
			let message = rx.recv().await;
			match message {
				Ok(message) => Ok(message),
				Err(_) => Err(::volo_thrift::AnyhowError::from(anyhow::Error::msg("Subscribe receive error"))),
			}
		});
		let msg = thread.await;
		match msg{
			Ok(tmp) =>{
				Ok(volo_gen::my_redis::SubscribeResponse{success: tmp.unwrap().into()})
			},
			Err(e)=>Err(e.into())
		}
		// Ok(volo_gen::my_redis::SubscribeResponse{success: msg.into()})
		// let channels = String::from(_req.channels);
		// let mut channels :Vec<&str>= channels.split_whitespace().collect();
		// let (tx,rx)=mspc::channel();
		// {
		// 	let local_map = GLOBAL_CHANNEL.lock().unwrap();
		// 	for chan in channels{
		// 		if let Some(oldChannel)=local_map.get(chan.to_string()){
					
		// 			oldChannel.insert((tx,rx));
		// 		}else{
		// 			local_map.insert(chan.to_string(),vec![(tx,rx)]);
		// 		}
		// 	}
		// }
		// Ok(Default::default())
	}

	async fn pub_channel(&self, _req: volo_gen::my_redis::PublishRequest) 
	-> ::core::result::Result<volo_gen::my_redis::PublishResponse, ::volo_thrift::AnyhowError>{
		let channel = String::from(_req.channel);
		let msg=String::from(_req.msg);
		// println!("before lock");
		let local_map = GLOBAL_CHANNEL.try_lock().unwrap();
		// println!("after lock");
		if let Some(clients)=local_map.get(&channel){
			for client in clients{
				let _= client.send(msg.clone());
			}
			Ok(volo_gen::my_redis::PublishResponse{num: clients.len() as i32})
		}else{
			Ok(volo_gen::my_redis::PublishResponse{num: 0})
		}
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
		// println!("{:?}",cx);
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