use std::fmt::Debug;

#[derive(Debug)]
struct cautrucA{
	f:u8,
}

fn test<T>(t:T)
where T: Debug {
	let a=T::from(t);
	println!("Hello from foo of test: {:?}",a);
}

fn main() {
    let a = cautrucA{
    	f:32,
    };
    test::<cautrucA>(a);
}
