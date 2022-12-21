fn main() {
    let decoded = bs58::decode("he11owor1d").into_vec().unwrap();
	let encoded = bs58::encode(decoded).into_string();
	assert_eq!("he11owor1d", encoded);
	
	let adr=String::from("1Asqgq7HQu8VewMCE4NisEoxZXfxP121i5");
	let dc = bs58::decode(adr).into_vec().unwrap();
	println!("{:?}",dc);
}
