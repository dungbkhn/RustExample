/*
use std::ops::Range;
use std::ops::Rem;
use std::cmp::PartialEq;

fn generate_even<T>(a: T, b: T) -> impl Iterator<Item = T>
  where
    Range<T>: Iterator<Item = T>,
    T: Copy + Rem<Output = T> + From<u8> + PartialEq
{
    let zero: T = 0_u8.into();
    let two: T = 2_u8.into();
    (a..b).filter(move |&x| x % two == zero)
}



fn main() {
    let even_u8s = generate_even(0_u8, 11_u8);
    let mut even_i16s = generate_even(0_i16, 11000_i16);
    let even_u16s = generate_even(0_u16, 11_u16);
    // and so on
    let even_u128s = generate_even(0_u128, 11_u128);
    //let mut iter = even_i16s.iter();
    assert_eq!(2, even_u8s.nth(10).unwrap());
}
*/

use std::ops::Range;
use std::ops::Rem;
use std::cmp::PartialEq;
use std::fmt;
use std::convert::{TryFrom, TryInto};

fn generate_even<T>(a: T, b: T) -> impl Iterator<Item = T>
  where
    Range<T>: Iterator<Item = T>,
    T: Copy + Rem<Output = T> + TryFrom<u8> + PartialEq + fmt::Debug,
    <T as TryFrom<u8>>::Error: fmt::Debug
{
    let zero: T = 0_u8.try_into().unwrap();
    let two: T = 2_u8.try_into().unwrap();
    (a..b).filter(move |&x| x % two == zero)
}

fn main() {
    let mut even_i8s = generate_even(-11_i8, 11_i8);
    let even_u8s = generate_even(0_u8, 11_u8);
    let even_i16s = generate_even(0_i16, 11_i16);
    let even_u16s = generate_even(0_u16, 11_u16);
    // and so on
    let even_u128s = generate_even(0_u128, 11_u128);
    assert_eq!(2, even_i8s.nth(0).unwrap());
}
