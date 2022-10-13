#[derive(Clone,Copy)]
struct CautrucA{
	f1:u8,
	f2:u32,
}


trait MyTrait<T>{
	fn foo(t:T){
		println!("hello from foo");
	}
}

impl<T> MyTrait<T> for CautrucA{
	fn foo(t:T){
		println!("hello from foo A");
	}
}

fn test<K> ()
where K: MyTrait<CautrucA>
{
	println!("hello from test of struct_A");
}

fn main() {
	let a = CautrucA{ f1:23,f2:56};
	let b = a;
	CautrucA::foo(b);
    println!("Hello, world! {}",CautrucA::from(a).f1.to_string());
    
    test::<CautrucA>();
}
