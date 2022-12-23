use warp::{http, Filter};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use tokio::time::delay_for;
use std::time::Duration;
use tokio::sync::mpsc;
use std::convert::TryInto;

use indexmap::{IndexMap, IndexSet};
use rand::Rng;

use std::time::Instant;
use std::{
    marker::PhantomData,
    time::{SystemTime, UNIX_EPOCH},
};

type Items = IndexMap<String, (String,[u8; 16])>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    address: String,
}

#[derive(Clone)]
struct Store {
  grocery_list: Arc<RwLock<Items>>,
  
}

impl Store {
    fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(IndexMap::new())),            
        }
    }
}

async fn add_grocery_list_item(
    item: Item,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
		let timestamp = SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap();
		let timestamp = timestamp.as_millis();
		let bytes = timestamp.to_be_bytes();
        store.grocery_list.write().insert(item.name, (item.address,bytes));
		println!("PostData");
        Ok(warp::reply::with_status(
            "Added items to the g list",
            http::StatusCode::CREATED,
        ))
}

async fn update_grocery_list_item(
    item: Item,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
		let timestamp = SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap();
		let timestamp = timestamp.as_millis();
		let bytes = timestamp.to_be_bytes();
        store.grocery_list.write().insert(item.name, (item.address,bytes));

        Ok(warp::reply::with_status(
            "Updated items to the g list",
            http::StatusCode::CREATED,
        ))
}

/*
async fn delete_grocery_list_item(
    item: Item,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.grocery_list.write().remove(&item.name);


        Ok(warp::reply::with_status(
            "Remove items to the g list",
            http::StatusCode::CREATED,
        ))
}
*/

async fn get_grocery_list(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut result: HashMap<String, String> = HashMap::new();
        let r = store.grocery_list.read();
		let len = r.len();
		//let aaaaa = r.get_index(0);
        //for (key,(value,t)) in r.iter() {
        //    result.insert(key, value);
        //}
        
        //println!("element {:?}", aaaaa);
        
        if len > 2 {
				let mut random_number = rand::thread_rng().gen_range(1..=len);
				random_number -= 1;
				
				let mut n1 = random_number;
				let mut n2 = n1;
				
				if random_number != len-1 {
					n2 = n1 + 1;		
				} else {
					n2 = 0;
				}
				
				println!("n1 {} n2 {}", n1,n2);
				let element1 = r.get_index(n1);
				match element1 {
					Some(e) => {result.insert(e.0.to_string(), e.1.0.to_string());},
					_ => {},
				}		
				let element2 = r.get_index(n2);
				match element2 {
					Some(e) => {result.insert(e.0.to_string(), e.1.0.to_string());},
					_ => {},
				}	
		}		
		else if len > 0 {
			let mut n1 = 0;
			let mut n2 = n1;
			let element1 = r.get_index(n1);
			match element1 {
				Some(e) => {result.insert(e.0.to_string(), e.1.0.to_string());},
				_ => {},
			}
			if len==2 {
				n2 = 1;
				let element2 = r.get_index(n2);
				match element2 {
					Some(e) => {result.insert(e.0.to_string(), e.1.0.to_string());},
					_ => {},
				}
			}
			println!("n1 {} n2 {}", n1,n2);
		}
		
		for (k,v) in result.iter() {
            println!("element in result: {} {} {}",len,k,v);;
        }
        
        Ok(warp::reply::json(
            &result
        ))
}

fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
	if std::env::args().len() < 1 {
		panic!("We need 1 params!");
	}
	
	let ipv6_adr : String = std::env::args().nth(1).unwrap().to_string();
	let vt : Vec<&str> = ipv6_adr.split(':').collect();
	
	let z1 = u16::from_str_radix(vt[0], 16);
	let z2 = u16::from_str_radix(vt[1], 16);
	let z3 = u16::from_str_radix(vt[2], 16);
	let z4 = u16::from_str_radix(vt[3], 16);
	let z5 = u16::from_str_radix(vt[4], 16);
	let z6 = u16::from_str_radix(vt[5], 16);
	let z7 = u16::from_str_radix(vt[6], 16);
	let z8 = u16::from_str_radix(vt[7], 16);
	
    let store = Store::new();
    let store_clone = store.clone();
    let store_filter = warp::any().map(move || store_clone.clone());

    let add_items = warp::post()
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(add_grocery_list_item);

    let get_items = warp::get()
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_grocery_list);
        
	/*
    let remove_items = warp::delete()
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(delete_grocery_list_item);
	*/
	
    let update_items = warp::put()
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(update_grocery_list_item);


    //let routes = add_items.or(get_items).or(update_items).or(remove_items);
	let routes = add_items.or(get_items).or(update_items);
	
	let socket_v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(z1.unwrap(),z2.unwrap(),z3.unwrap(),z4.unwrap(),z5.unwrap(),z6.unwrap(),z7.unwrap(),z8.unwrap())), 3030);
    
    let routes_clone = routes.clone();
    
    let (send, mut recver) = mpsc::unbounded_channel();
    
    tokio::spawn(async move { 	
			warp::serve(routes_clone)
			.run(socket_v6)
			.await;
	});
		
    tokio::spawn(async move{ 				
		loop{
			send.send(true).unwrap();			
			delay_for(Duration::from_millis(60000)).await;			
		}
	});
			
	
	loop {
		tokio::select!{
			Some(m) = recver.recv() => {				
				println!("===========================");
				let mut v = Vec::<String>::new();
				
				{
					let r = store.grocery_list.read();				
					for (key,(value,t)) in r.iter() {            
						let timestamp = u128::from_be_bytes(*t);
						let timestamp = Duration::from_millis(timestamp as u64); // TODO: handle overflow
						let latency = SystemTime::now()
							.duration_since(UNIX_EPOCH + timestamp)
							.unwrap();
						println!("show {} - latency: {}ms", key, latency.as_millis());
						if latency.as_millis() > 330000 {
							v.push((*key.clone()).to_string());
						}
					}
				}
				
				for k in &v {
					//println!("{i}");
					store.grocery_list.write().remove(k);
				}
				
				println!("===========================");
			}
		}
	}
}

/*


let timestamp = SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap();
                    let timestamp = timestamp.as_millis();
                    let bytes = timestamp.to_be_bytes();
                    println!("{:?}",bytes);
let timestamp = u128::from_be_bytes(bytes);
                        let timestamp = Duration::from_millis(timestamp as u64); // TODO: handle overflow
                        let latency = SystemTime::now()
                            .duration_since(UNIX_EPOCH + timestamp)
                            .unwrap();
                            println!("latency: {}ms", latency.as_millis());
*/
/*
 counter : Arc<RwLock<u64>>,
 counter : Arc::new(RwLock::new(0)),
 let r = store.counter.read();
 let i = (*r).try_into().unwrap();
 let mut r = store.counter.write();
 *r = *r + 1;
 
 */
//let r : std::sync::RwLock<u8> = store.counter::try_unwrap();
//let t = r.unwrap();
//let r = store.grocery_list.read();
//let rwlck : parking_lot::lock_api::RwLock<u8,T> = Arc::try_unwrap(store.counter.clone()).unwrap();
//let mut lck = store.counter.lock().unwrap();
//let w = lck.take().unwrap();
//let mut w = r.write().unwrap();
/*
let timestamp = SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap();
                    let timestamp = timestamp.as_millis();
                    let bytes = timestamp.to_be_bytes();
			delay_for(Duration::from_millis(15000)).await;
			let timestamp = u128::from_be_bytes(bytes);
                        let timestamp = Duration::from_millis(timestamp as u64); // TODO: handle overflow
                        let latency = SystemTime::now()
                            .duration_since(UNIX_EPOCH + timestamp)
                            .unwrap();
                            println!("latency: {}ms", latency.as_millis());
*/
