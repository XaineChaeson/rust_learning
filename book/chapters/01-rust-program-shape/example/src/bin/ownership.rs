fn main() {
    let topic = String::from("ownership");

    print_topic(&topic);
    println!("still available after borrowing: {topic}");

    let moved_topic = topic;
    println!("new owner: {moved_topic}");
}

fn print_topic(topic: &str) {
    println!("today's topic: {topic}");
}
