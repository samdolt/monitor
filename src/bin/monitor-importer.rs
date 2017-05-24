extern crate redis;
extern crate monitor;
extern crate serde_json as json;
extern crate serde;
extern crate base64;

use monitor::glue::lora::Message;

use redis::Commands;

use serde::Deserialize;
use serde::Serialize;


static PORTS: &'static [&str] = &["lora:rx:1", "lora:rx:2", "lora:rx:3"];


fn main() {
    run().expect("Something wrong");
}


fn run() -> redis::RedisResult<()> {
    let redis_client = redis::Client::open("redis://127.0.0.1")?;
    let redis = redis_client.get_connection()?;



    loop {
        //let keys: Vec<&str> = vec!("lora:rx:1", "lora:rx:2");

        let (key, message_str): (String, String) = redis.brpop(PORTS, 0)?;

        // key can be between "lora:rx:1" and "lora:rx:223"
        let port: u8 = (&key[8..]).parse().expect("Redis key doesn't respect lora:rx:$FPort format");
        let message: json::Value = json::from_str(&message_str).expect("Wrong json");

        match port {
            0 => panic!("Invalid port 0"),
            3 => waspmote_parse(&message),
            _ => println!("New message: {},  {}", port, message),
            224...255 => panic!("Invalid port >223"),
        }

    }

}

use std::process::Command;

fn waspmote_parse(message: &json::Value) {
    use monitor::waspmote::decode;

    println!("New waspmote message with:");

    let measurements = decode(&base64::decode(message["payload"].as_str().unwrap()).unwrap());

    let device = message["device"].as_str().unwrap();

    for measurement in measurements {
        influx_add(&measurement.name, device, measurement.value);
    }



}

fn influx_add(topic: &str, device: &str, value: f64 ) {

    println!("   {}={} from {}", topic, value, device);

    Command::new("curl")
        .arg("-i")
        .arg("-XPOST")
        .arg("http://localhost:8086/write?db=monitor")
        .arg("--data-binary")
        .arg(format!("{},device={} value={}", topic, device, value))
        .spawn()
        .expect("failed to execute process");

}
