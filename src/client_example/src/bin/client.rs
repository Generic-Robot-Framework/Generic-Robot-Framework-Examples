use std::sync::{Arc, Mutex};
use client_example::msg::example_message::Position2DMessage;
use generic_robot_framework::main_loop;
use generic_robot_framework::models::tnode::Node;
use generic_robot_framework::models::tpub::Publisher;
use generic_robot_framework::models::tsub::Subscriber;

fn main() {
    let atomic_data = Some(Arc::new(Mutex::new(0)));

    let node: Node = Node::new("my_topic".to_string(), 1000);
    let subscriber: Subscriber<Position2DMessage, Arc<Mutex<u64>>> = Subscriber::new("my_new_topic".to_string(), test_handle, atomic_data.clone());
    let publisher: Publisher<Position2DMessage> = Publisher::new("my_new_topic".to_string());

    let message = Position2DMessage {
        x: 1,
        y: 2
    };

    publisher.publish(message.clone());

    main_loop!(node, {
        println!("{}", atomic_data.as_ref().unwrap().lock().unwrap());

        node.sleep();
    });
}

fn test_handle(message: Position2DMessage, atomic_data: Option<Arc<Mutex<u64>>>) {
    println!("{}", message);

    let arc = atomic_data.unwrap();
    let mut data = arc.lock().unwrap();
    *data = message.y
}