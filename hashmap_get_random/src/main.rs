use std::collections::HashMap;
use indexmap::{IndexMap, IndexSet};

fn main() {
	/*
	let mut hm = HashMap::<String,bool>::new();
	hm.insert("aaa".to_string(),true);
	hm.insert("bbb".to_string(),true);
	hm.insert("ccc".to_string(),true);
	hm.insert("ddd".to_string(),true);
    println!("Hello, world! {:?}",hm.keys().next());*/
    
    let mut letters = IndexMap::new();
	for ch in "a short treatise on fungi".chars() {
		*letters.entry(ch).or_insert(0) += 1;
	}
	
	assert_eq!(letters[&'s'], 2);
	assert_eq!(letters[&'t'], 3);
	assert_eq!(letters[&'u'], 1);
	assert_eq!(letters.get(&'y'), None);
	
	println!("Hello, world! {:?}",letters.get_index(0));
}
