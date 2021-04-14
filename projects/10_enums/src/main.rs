fn main() {
    enums_can_be_similar_to_cpp();
    enums_can_contain_any_value();
    enums_with_structs();
}

#[allow(dead_code)]
//#[allow(unused_variables)]
enum MyKind {
    V1,
    V2
}

//#[allow(dead_code)]
#[allow(unused_variables)]
fn enums_can_be_similar_to_cpp() {
    let my = MyKind::V1;

    // This won't compile though...
    /*if my == MyKind::V2 {
        println!("my is V2");
    } else if my == MyKind::V1 {
        println!("my is V1");
    }*/
}

// We can directly attach data to a enum.  This is a neat feature.
enum EnumWithValue {
    V1(String),
    V2(i32),
    V3(u64)
}

#[allow(unused_variables)]
fn enums_can_contain_any_value() {
    let x = EnumWithValue::V1(String::from("hello"));
    let y = EnumWithValue::V2(-234);
    let z = EnumWithValue::V3(234);
}

// Enums can contain tuples, etc, and even contain structs.
#[allow(dead_code)] struct QuitMessage; // unit struct (no data)
#[allow(dead_code)] struct MoveMessage { x: i32, y: i32 }
struct WriteMessage(String); // tuple struct
struct ChangeColourMessage(i32, i32, i32); // tuple struct

enum Message {
    Quit(QuitMessage),
    Move(MoveMessage),
    Write(WriteMessage),
    ChangeColour(ChangeColourMessage)
}
// They can also contain other enums!

impl Message {
    // Now we can have a member function which handles all these types.
    fn handle_message(&self) {
        //...
    }
}

#[allow(unused_variables)]
fn enums_with_structs() {
    let message1 = Message::Quit(QuitMessage{});        
    let message2 = Message::Move(MoveMessage{ x: 10, y: 0 });        
    let message3 = Message::Write(WriteMessage(String::from("message")));    
    let message4 = Message::ChangeColour(ChangeColourMessage(1, 2, 3));

    message1.handle_message();
    message2.handle_message();
    message3.handle_message();
    message4.handle_message();
}
