
// main
fn main() {
    match asyncchat::run() {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => ratio, 
    }
}
