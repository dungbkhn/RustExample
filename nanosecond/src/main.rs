use std::thread;
use std::time::Duration;
use chrono::{Utc};
use chrono::{Local, DateTime, TimeZone};

struct Mes {
	M1 : (String, i64),
}

fn main() {
 

    let now = Utc::now();

    let sec = Duration::from_millis(1000);
    thread::sleep(sec);
    
    let then = Utc::now();

    let diff = then-now;

	//get i64 value
    println!("{}", diff.num_nanoseconds().unwrap_or(0));
    
    let now = Local::now();

    let sec = Duration::from_millis(2000);
    thread::sleep(sec);
    
    let then = Local::now();

    let diff = then-now;

	//get i64 value
    println!("{}", diff.num_milliseconds());
    
    let t1 = Local.timestamp(0,1);

    let t2 = Local.timestamp(0,1);

    let diff = now-t1;

	//get i64 value
    println!("{}", diff.num_nanoseconds().unwrap_or(0));
    
    if t1==t2 {
		println!("bang");
	}
	
	let m = Mes {
		M1 : (String::from("hello"),45),
	};
	
	println!("{}",m.M1.0);
}
