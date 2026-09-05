enum TrafficLight {
    Red,
    Yellow,
    Green
}
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn action(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("stop"),
        TrafficLight::Yellow => println!("go but fast"),
        TrafficLight::Green => println!("go"),
    }
}

fn main() {
    let current_light = TrafficLight::Green;
    action(current_light);

    let loopback = IpAddr::V4(String::from("127.0.0.1"));
    let ipv6 = IpAddr::V6(String::from("f2:56:g6:h9:g9"));

    println!("loopback address is {:?}", loopback);
    println!("mac address is: {:?}", ipv6);
}