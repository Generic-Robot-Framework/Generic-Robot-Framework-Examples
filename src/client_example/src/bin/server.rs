use client_example::msg::example_message::Position2DMessage;

use generic_robot_framework::models::tnode::Node;
use generic_robot_framework::models::tpub::Publisher;

fn main() {
    // Creating and registering the Node
    let _node = Node::register("my_server_node", 1000);

    // Creating a Publisher
    let publisher: Publisher<Position2DMessage> = Publisher::new("my_new_topic");

    // Creating a Position2DMessage
    let message = Position2DMessage {
        x: 1,
        y: 2
    };

    // Publishing the message
    publisher.publish(message);
}