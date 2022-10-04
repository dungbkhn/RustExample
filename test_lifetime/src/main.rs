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
			
}
