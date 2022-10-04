use std::io;
// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;

// A tuple struct with resources that implements the `Clone` trait
#[derive(Clone, Copy, Debug)]
struct Pair(i32, i32);

/*
first version we are instructing the compiler that

Find a lifetime 'b such that both &self and the reference other are valid during it (probably the shorter of the two lifetimes if they overlap)

Make sure that the returned reference is only used within that lifetime 'b because outside it might become a dangling reference.
*/

//part phai co lifetime it nhat la bang a neu ko part se tro thanh dangling reference
//vay ham larger o day ta co b va a ko dinh dang gi toi nhau vi neu 'b:'a thi cung ko sao ma 'a:'b thi cung ko sao

struct Important<'a> {
    part: &'a str,
}

impl<'a> Important<'a> {
    fn larger<'b>(&'b self, other: &'b str) -> &'b str {
        if self.part.len() > other.len() {
            self.part
        } else {
            other
        }
    }
}

/*

second:

struct Important<'a> {
    part: &'a str,
}

impl<'a> Important<'a> {
    fn larger(&self, other: &'a str) -> &str {
        if self.part.len() > other.len() {
            self.part
        } else {
            other
        }
    }
}


the second example would desugar to this if we expand it by reversing Rust's lifetime elision rules:

struct Important<'a> {
    part: &'a str,
}

impl<'a> Important<'a> {
    fn larger<'b>(self: &'b Important<'a>, other: &'a str) -> &'b str {
        if self.part.len() > other.len() {
            self.part
        } else {
            other
        }
    }
}
The reason why self is bounded by 'a is because &'b Important<'a> implies 'a: 'b or in plain terms "'a outlives 'b" (a song lau hon b) which must be true otherwise the reference may get invalidated. If we expand the first example the type of self again becomes &'b Important<'a>. The only difference between the first and second example is the type of other, in the first it's &'b str and in the second it's &'a str but that's an irrelevant detail since the return value has the type &'b str which is constrained by the 'b lifetime and we know that 'a: 'b so functionally both examples are the same.

*/

fn main() {
    println!("Hello, world!");
    
    let is_i64 = {
	    returns_i64()
    };

    let is_unit = {
	    returns_i64();
    };
    
    print!("{:#?}",is_unit);
    
    let guess : u32 = "42".parse().expect("");
    
    
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    
    // Instantiate `Unit`
    let p1 = Pair(1,2);
    // Copy, not move
    let p2 = p1;
    
    println!("p1 is: {:?}", p1);
    println!("p2 is: {:?}", p2);
    
    //let v = vec![1, 2, 3];
    //this cause panic
    //v[99];
    
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}


async fn hello_world() {
    println!("hello, world!");
}


fn returns_i64() -> i64 {
    1i64
}
fn returns_unit() {
    1i64;
}


