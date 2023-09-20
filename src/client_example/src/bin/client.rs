use std::sync::{Arc, Mutex};

use client_example::msg::example_message::Position2DMessage;

use generic_robot_framework::main_loop;
use generic_robot_framework::models::tnode::Node;
use generic_robot_framework::models::tsub::Subscriber;

fn main() {
    // Create some atomic data
    let atomic_data = Arc::new(Mutex::new(0));

    // Creating and registering the Node
    let node = Node::register("my_client_node", 1000);

    // Creating a Subscriber
    let _subscriber: Subscriber<Position2DMessage, Arc<Mutex<u64>>> = Subscriber::new(
        "my_new_topic",
        test_handle,
        Some(Arc::clone(&atomic_data))
    );

    // Main application loop, checks if the node is Ok
    main_loop!(node, {
        // Print our atomic data
        println!("{}", *atomic_data.lock().unwrap());

        // Sleeping at Node rate
        node.sleep();
    });
}

// Called when a new message is received
fn test_handle(message: Position2DMessage, atomic_data: Option<Arc<Mutex<u64>>>) {
    // Print the received message
    println!("Received:\n{}", message);

    // Getting the atomic data mutex
    let arc = atomic_data.unwrap();
    let mut data = arc.lock().unwrap();

    // Changing atomic data
    *data = message.y
}