use std::fmt::Debug;

#[derive(Clone,Copy,Debug)]
struct CautrucA<T>{
	f1:u8,
	f2:u32,
	f3:T,
}

impl<T> CautrucA<T>{
	fn foo(t:T){
		println!("hello from foo cautrucAb");
	}
	
	fn test<K> (k:K)
	where K: MyTrait<T> + Debug
	{
		K::foo();
		println!("hello from test of CautrucA : {:?}",k);
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

impl<T> std::fmt::Display for CautrucA<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", 5, 5)
    }
}

impl<T: std::fmt::Debug> std::error::Error for CautrucA<T> {
	
}

fn test<K,T> ()
where K: MyTrait<T>
{
	K::foo();
	println!("hello from test of main");
}

#[derive(Clone,Copy,Debug)]
struct CautrucB<U>{
	f1:u8,
	f2:u32,
	f3:U,
}

trait MyTraitB<V>{
	fn foo(v:V){
		println!("hello from foo mytrait B");
	}
}

impl<V,U> MyTraitB<V> for CautrucB<U>{
	fn foo(v:V){
		println!("hello from foo mytrait B for cautruc B");
	}
}

fn test_traiB<I, U>()
	where I: MyTraitB<U>{	
	println!("hello");
}

trait DungSink<Item> {
    type OKKKK; 
}

fn test_DungSink<S, M, E>()
	where 	M: std::fmt::Debug,
			E: std::error::Error,
			S: DungSink<M, OKKKK = E>{	
	println!("hello form testDungSink");
}


struct C{
	
}

impl<u32> DungSink<u32> for C{
	type OKKKK=CautrucA<u32>;
}


trait DungTrait1 {
    type OK;    
}

trait DungTrait2<Item>: DungTrait1 {
      
}


struct D{
	
}

impl DungTrait1 for D{
	type OK = CautrucA<u32>;
}

impl<Item> DungTrait2<Item> for D{

}

fn test_DungTrait<S, M, E>()
	where 	M: std::fmt::Debug,
			E: std::error::Error,
			S: DungTrait2<M, OK = E>{	
	println!("hello form test_DungTrait");
}


fn main() {
	let a = CautrucA{ f1:23,f2:56,f3:45};
	let b = a;
	CautrucA::foo(b);
	println!("Hello, world! {}",CautrucA::from(a).f1.to_string());

	test::<CautrucA<u32>,u32>();
	
	let c = CautrucA::from(a);
	CautrucA::test::<CautrucA<u32>>(b);
	println!("Hello, world2222! {}",c.f1.to_string());
	
	test_DungSink::<C,u32,CautrucA<u32>>();
	
	test_DungTrait::<D,u32,CautrucA<u32>>();
}
