/*
first version we are instructing the compiler that

Find a lifetime 'b such that both &self and the reference other are valid during it (probably the shorter of the two lifetimes if they overlap)

Make sure that the returned reference is only used within that lifetime 'b because outside it might become a dangling reference.
*/

//part phai co lifetime it nhat la bang a neu ko part se tro thanh dangling reference
//vay ham larger o day ta co b va a ko dinh dang gi toi nhau vi neu 'b:'a thi cung ko sao ma 'a:'b thi cung ko sao
/*
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
*/
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


the second example would desugar to this if we expand it by reversing Rust's lifetime elision rules:(cai nay sai)

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

Chinh xac exmaple 2 phai chuyen la:

struct Important<'a> {
    part: &'a str,
}

impl<'a> Important<'a> {
    fn larger<'a>(self: &'a Important<'a>, other: &'a str) -> &'a str {
        if self.part.len() > other.len() {
            self.part
        } else {
            other
        }
    }
}

lifetime 'a o phan khai bao va o phan cai dat ham la khac nhau

*/


struct Important2<'a> {
    part: &'a str,
}
//
//&'b self nghia la &'b Important<'a> nghia la 'a: 'b nghia la "'a outlives 'b" (a song lau hon b), do do dau ra la smaller -> compile ok
impl<'b,'a> Important2<'a> {
    fn larger(&'b self, other: &'a str) -> &'b str {
        if self.part.len() > other.len() {
            self.part
        } else {
            other
        }
    }
}

struct StrWrap<'a>(&'a str);

fn make_wrapper1(string: &str) -> StrWrap {
    StrWrap(string)
}

//or with rust 2018
fn make_wrapper2(string: &str) -> StrWrap<'_> {
    StrWrap(string)
}

//or
fn make_wrapper3<'a>(string: &'a str) -> StrWrap<'a> {
    StrWrap(string)
}

/*
Rust 2018 allows you to explicitly mark where a lifetime is elided, for types where this elision might otherwise be unclear. To do this, you can use the special lifetime '_ much like you can explicitly mark that a type is inferred with the syntax let x: _ = ..;.
Let's say, for whatever reason, that we have a simple wrapper around &'a str:

struct StrWrap<'a>(&'a str);
In Rust 2015, you might have written:

// Rust 2015

use std::fmt;


fn make_wrapper(string: &str) -> StrWrap {
    StrWrap(string)
}

impl<'a> fmt::Debug for StrWrap<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.0)
    }
}
In Rust 2018, you can instead write:

#![feature(rust_2018_preview)]


// Rust 2018

fn make_wrapper(string: &str) -> StrWrap<'_> {
    StrWrap(string)
}

impl fmt::Debug for StrWrap<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.0)
    }
}
More details
In the Rust 2015 snippet above, we've used -> StrWrap. However, unless you take a look at the definition of StrWrap, it is not clear that the returned value is actually borrowing something. Therefore, starting with Rust 2018, it is deprecated to leave off the lifetime parameters for non-reference-types (types other than & and &mut). Instead, where you previously wrote -> StrWrap, you should now write -> StrWrap<'_>, making clear that borrowing is occurring.
What exactly does '_ mean? It depends on the context! In output contexts, as in the return type of make_wrapper, it refers to a single lifetime for all "output" locations. In input contexts, a fresh lifetime is generated for each "input location". More concretely, to understand input contexts, consider the following example:

// Rust 2015

struct Foo<'a, 'b: 'a> {
    field: &'a &'b str,
}

impl<'a, 'b: 'a> Foo<'a, 'b> {
    // some methods...
}
We can rewrite this as:

#![feature(rust_2018_preview)]


// Rust 2018

impl Foo<'_, '_> {
    // some methods...
}
*/
//'long:'short meaning long outlives short
struct Foo<'a, 'b: 'a> {
    field: &'a &'b str,//tham chieu toi tham chieu
}

fn make_foo1<'a>(string: &'a &str) -> Foo<'a,'a>  {
    Foo{
    field:string,
   }
}

fn main (){
	
	//test ham 1
	
	/*let mystring1 = String::from("String1");		
	
	println!("out of braket {}",mystring1);
			
	{
		
		let myimportant = Important{part:&mystring1};
		let mut output;
		
		{
			let mystring2 = String::from("String22222");
			output = myimportant.larger(&mystring2);
			
			
		}
		println!("in braket {}",output);
	}*/
	
	//test ham 2
	let s1 = String::from("S1");
	
	println!("out of braket {}",s1);
	let myimportant2 = Important2{part:&s1};
	
	{
		
		
		let output2;
		let s2 = String::from("S2");
		
		{
			output2= myimportant2.larger(&s2);
		}
		
		println!("in braket {}",output2);
	}
	let s3 = String::from("hello world");

    	let hello = &s3[..];
	let shello=&hello;
	make_wrapper1(&s1);
	make_wrapper2(&s1);
	make_wrapper3(&s1);
	make_foo1(shello);
}
