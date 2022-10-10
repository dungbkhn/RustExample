

fn add_one(x: i32) -> i32 {
    x + 1
}
    
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn k(i32) -> u32{
	5
}

fn main() {
    fn add_two(x: i32) -> i32 {
	    x + 2
    }
    
    let f=add_one;
    
    let f2=add_two;
    
    let answer = do_twice(f, 5);

    println!("The answer is: {}", answer);
    
    let answer2 = do_twice(f2, 5);

    println!("The answer is: {}", answer2);
    
    println!("The answer is: {}", f2(20));
    
    println!("The answer is: {}", k(20));
}
