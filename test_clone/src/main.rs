#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point{
	fn setx(&mut self) {
        self.x = 1;
    }
	
	fn sety(&mut self) {
        self.y = 2;
    }
    
    fn getx(&self) -> i32 {
        self.x
    }
    
    fn gety(&self) -> i32 {
        self.y
    }
}

//no method in Copy because it is a marker trait
/*impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Point {
        self.clone() or *self  --> kho hieu voi primitive types
    }
}*/

#[derive(Debug, Clone)]
struct A{
		user: String,
		age: u16,
}

fn main(){
	let s1 = Point{x:5,y:10};
	
	/*
	let mut s2 = s1.clone();
	
	s2.setx();
	
	s2.sety();
	
	println!("{:?}",s1.getx());*/
	
	let mut s2 = s1;
	//s2.setx();
	println!("{:?}",s2.getx());
	
	let a = A {user:String::from("dungnt"),age:40};
	
	let b = a.clone();
	
	println!("{}",a.user);
	
	let mut str1 = String::from("aaa");
	
	let str2 = str1.clone();
	
	str1.push_str(" bbb");
	
	println!("{}",str2);
}
