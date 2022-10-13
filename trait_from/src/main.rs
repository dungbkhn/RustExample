#[derive(Clone,Copy)]
struct CautrucA<T>{
	f1:u8,
	f2:u32,
	f3:T,
}

impl<T> CautrucA<T>{
	fn foo(t:T){
		println!("hello from foo cautrucAb");
	}
	
	fn test<K> ()
	where K: MyTrait<T>
	{
		K::foo();
		println!("hello from test of CautrucA");
	}
}

trait MyTrait<T>{
	fn foo(){
		println!("hello from foo mytrait");
	}
}

impl<T> MyTrait<T> for CautrucA<T>{
	fn foo(){
		println!("hello from foo impl");
	}
}

fn test<K,T> ()
where K: MyTrait<T>
{
	K::foo();
	println!("hello from test of struct_A");
}

fn main() {
	let a = CautrucA{ f1:23,f2:56,f3:45};
	let b = a;
	CautrucA::foo(b);
    	println!("Hello, world! {}",CautrucA::from(a).f1.to_string());
    
    	test::<CautrucA<u32>,u32>();
    	
    	let c = CautrucA::from(a);
    	CautrucA::test::<CautrucA<u32>>();
    	println!("Hello, world2222! {}",c.f1.to_string());
}
