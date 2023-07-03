use std::thread::{sleep};
use std::time::Duration;
use client_example::msg::example_message::ExampleMessage;
use generic_robot_framework::main_loop;
use generic_robot_framework::models::message::Message;
use generic_robot_framework::models::tnode::Node;
use generic_robot_framework::models::tpub::Publisher;
use generic_robot_framework::models::tsub::Subscriber;

fn main() {
    let node: Node = Node::new("my_topic".to_string(), 1000);
    let subscriber: Subscriber<ExampleMessage> = Subscriber::new("my_new_topic".to_string(), test_handle);
    let publisher: Publisher<ExampleMessage> = Publisher::new("my_new_topic".to_string());

    let message = ExampleMessage {
        x: 1,
        y: 2
    };

    publisher.publish(message);

    main_loop!(node, {
        println!("test");
        node.sleep();
    });
}

fn test_handle(message: ExampleMessage) {
    println!("{}", message)
}