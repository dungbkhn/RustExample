use std::fmt;


struct Point {
    x: u8,
    y: u8,
}

impl Point {
	fn A() -> u16{
		5
	}

}


impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct newstruct<I: std::fmt::Display + Clone, P: std::fmt::Debug> {
    pub id: I,
    pub c: P,
}

type SessionID = u8;
type Session = newstruct<String, ()>;

#[derive(Debug, Clone)]
pub struct CloseFrame {
    pub code: u8,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
    Close(Option<CloseFrame>),
}

#[derive(Debug, Clone)]
pub enum RawMessage {
    Text(String),
    Binary(Vec<u8>),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Close(Option<CloseFrame>),
}

impl From<Message> for RawMessage {
    fn from(message: Message) -> Self {
        match message {
            Message::Text(text) => Self::Text(text),
            Message::Binary(bytes) => Self::Binary(bytes),
            Message::Close(frame) => {
            	
		    match frame {
		    	Some(ref cf) => println!("{:?}",cf),
		    	None => println!("error None here"),
		    }
		    //call pointer to fn <CloseFrame as From<CloseFrame>>::from
		    //function above converts CloseFrame to CloseFrame
		    //let fr = frame.map(CloseFrame::from);
		    Self::Close(frame)
            },
        }
    }
}

fn A() -> CloseFrame{
		CloseFrame {
			code:12,
			reason:String::from("hello"),
		}
	}

pub fn test(){
	//let frame = CloseFrame::from;	
	//println!("{:?}",frame);	
	let cf = A();
	let m = Message::Close(Some(cf));
	let rm : RawMessage = RawMessage::from(m);
	println!("{:?}",rm);
	
	//let o = Some(cf);
	//let m = o.map(|a| CloseFrame::from);
	
}



fn main(){
	let origin = Point { x: 0, y: 0 };

	assert_eq!(format!("The origin is: {origin}"), "The origin is: (0, 0)");
	
	test();
	
	let maybe_some_string = Some(String::from("Hello, World!"));
	// `Option::map` takes self *by value*, consuming `maybe_some_string`
	let maybe_some_same_string = maybe_some_string.map(String::from);

	assert_eq!(maybe_some_same_string, Some(String::from("Hello, World!")));
}
