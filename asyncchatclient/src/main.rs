fn main() {
    match asyncchatclient::run() {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => ratio, 
    }
}
