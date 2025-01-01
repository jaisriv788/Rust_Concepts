#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Running the {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let first_call = ChatMessage {
        content: "Video-1",
        time: String::from("320 sec"),
    };

    let second_call = ChatMessage {
        content: String::from("Video-2"),
        time: String::from("250 sec"),
    };

    let third_call = ChatMessage {
        content: DigitalContent::VideoFile,
        time: String::from("190 sec"),
    };

    let fourth_call = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("190 sec"),
    };

    //to check the enum intigration
    third_call.consume_entertainment();
    fourth_call.consume_entertainment();

    println!("{}", first_call.retrieve_time());
    println!("{}", second_call.retrieve_time());
    println!("{}", third_call.retrieve_time());
    println!("{}", fourth_call.retrieve_time());
}
