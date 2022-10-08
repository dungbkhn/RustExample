/*
I am learning Rust and have run into the following problem, which is not obvious to me. I saw a std::thread::spawn in the library, looked at the implementation, and saw that some type requires an implementation of the Send trait to be able to send something to another thread. I'm trying to replicate the behavior with my own function and my own trait, but the compiler successfully compiles the code without complaining that the trait is not implemented for my types. What am I missing?

pub trait SomeTrait {
    fn bar(&self);
}

#[derive(Debug)]
struct StructWithoutTrait {
    _data: Box<i32>
}

#[derive(Debug)]
struct StructWithTrait {
    _data: Box<i32>
}

impl SomeTrait for StructWithTrait {
    fn bar(&self) {}
}

fn foo<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
    F: SomeTrait + 'static,
    T: SomeTrait + 'static
{
    f.bar();
    f()
}

impl<F, T> SomeTrait for F
where 
    F: FnOnce() -> T,
    T: SomeTrait
{
    fn bar(&self) {}
}

fn main() {
    let without_trait = StructWithoutTrait { _data: Box::new(1) };
    let with_trait = StructWithTrait { _data: Box::new(2) };
    let x = std::rc::Rc::new(1);

    foo(move || {
        println!("{:?}", without_trait);
        println!("{:?}", x);
        with_trait
    });
}

----

Send is an auto trait. Auto-traits are somewhat like compiler-implemented; the steps of determining whether a type T implements an auto trait AutoTrait are the following:

If there is an explicit impl AutoTrait for T, then T always impls AutoTrait (note that because Send is an unsafe trait you need unsafe impl Send for T, but the general principle stays the same).
Otherwise, if there is a negative impl impl !AutoTrait for T, then T does not implement AutoTrait. Note that this is an error to have both positive and negative impl of the same trait for the same type.
Otherwise, if any of the fields of T does not implement AutoTrait, T does not implement it either. For example, if T is defined as struct T(U);, and there's impl !AutoTrait for U, then T does not implement AutoTrait.
Otherwise, T implements AutoTrait.
Both auto traits and negative impls are highly unstable features, and not intended to use as of now outside of the standard library. If you really want, you can, although because the compiler must be able to implement the trait automatically, it cannot contain any associated item (methods, associated types, associated consts). If you remove the bar() method, this is how your code will look like:

#![feature(auto_traits, negative_impls)]

pub auto trait SomeTrait {}

#[derive(Debug)]
struct StructWithoutTrait {
    _data: Box<i32>
}

impl !SomeTrait for StructWithoutTrait {}

#[derive(Debug)]
struct StructWithTrait {
    _data: Box<i32>
}

fn foo<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
    F: SomeTrait + 'static,
    T: SomeTrait + 'static
{
    f()
}
fn main() {
    let without_trait = StructWithoutTrait { _data: Box::new(1) };
    let with_trait = StructWithTrait { _data: Box::new(2) };
    let x = std::rc::Rc::new(1);

    foo(move || {
        println!("{:?}", without_trait);
        println!("{:?}", x);
        with_trait
    });
}
*/



#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

//can not pub fn notify<T: Summary>(item1: &T, item2: &U) {
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
