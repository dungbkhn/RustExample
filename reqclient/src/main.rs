use std::collections::HashMap;
use reqwest::header::CONTENT_TYPE;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	if std::env::args().len() < 1 {
		panic!("We need 1 params!");
	}
	
	let ipv6_adr : String = std::env::args().nth(1).unwrap().to_string();
	
	let mut string_server_adr = String::from("");
	string_server_adr.push_str("http://[");
	string_server_adr.push_str(&ipv6_adr);
	string_server_adr.push_str("]:3030");
	println!("string_server_adr:{}",string_server_adr);
	
	
	/*let mut stringtopush = String::from("");
	let peer_id = String::from("12fdgjkdkjghsghdfghdjklfhdfl");
	let adr = String::from("/[2405:4802:231:78f0:4c1a:7864:8e19:d252]/tcp/9000");
	stringtopush.push_str(&"{\"name\": \"");
	stringtopush.push_str(&peer_id);
	stringtopush.push_str(&"\", \"address\" : \"");
	stringtopush.push_str(&adr);
	stringtopush.push_str("\"}");
	
	let mut string_server_adr = String::from("");
	string_server_adr.push_str("http://[2405:4802:231:78f0:4c1a:7864:8e19:d252]:3030");
	
	let client = reqwest::Client::new();
	let res = client
		.post(string_server_adr.clone())
		.header(CONTENT_TYPE, "application/json")
		.body(stringtopush)
		.send()
		.await;
	
	println!("{:#?}", res);
	*/
	
    let resp = reqwest::get(string_server_adr)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    //println!("{:#?}", resp.unwrap());
    
    for (key, value) in &resp {
		println!("{} {}", key, value);		 
	}
	/*	
	let mut s = Some(String::from("abc"));
	
	s = match s {
		None => {None},
		Some(strs) => {Some(strs)},
	};
	
	println!("{}",s.unwrap());
	
	
	let mut t : Option<String> = Some(String::from("okok"));
	
	if let Some(k) = t {
		println!("t not none");
		t = Some(k);
	}
	else {
		println!("t none");
	}
	
	
	println!("{}",t.unwrap());
	*/
    Ok(())
}
