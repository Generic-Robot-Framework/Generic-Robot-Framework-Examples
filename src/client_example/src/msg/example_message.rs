extern crate generic_robot_framework;
use generic_robot_framework::register_message;

register_message!(Position2DMessage {
    x: u64,
    y: u64,
});
