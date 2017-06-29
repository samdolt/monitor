extern crate redis;
extern crate monitor;
extern crate serde_json as json;
extern crate serde;
extern crate base64;
extern crate influxdb;

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use slog::Drain;
use slog::Logger;

use redis::Commands;




static PORTS: &'static [&str] = &["lora:rx:1", "lora:rx:2", "lora:rx:3"];


fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = slog::Logger::root(
        drain,
        o!("binary" => "monitor-importer", "version" => env!("CARGO_PKG_VERSION"))
    );

    info!(_log, "Starting program");
    run(&_log).expect("Something wrong");
}


fn run(logger: &Logger) -> redis::RedisResult<()> {
    let redis_client = redis::Client::open("redis://127.0.0.1")?;
    let redis = redis_client.get_connection()?;
    let redis_log = logger.new(o!("redis-version" => "unknow"));
    info!(redis_log, "Connected to Redis");

    let influx = influxdb::Connection::connect(("http://127.0.0.1/monitor",logger.new(o!()))).unwrap();
    //let influx = influxdb::Connection::connect("http://127.0.0.1/monitor").unwrap();

    loop {
        //let keys: Vec<&str> = vec!("lora:rx:1", "lora:rx:2");

        let (key, message_str): (String, String) = redis.brpop(PORTS, 0)?;

        // key can be between "lora:rx:1" and "lora:rx:223"
        let port: u8 = (&key[8..])
            .parse()
            .expect("Redis key doesn't respect lora:rx:$FPort format");
        let message: json::Value = json::from_str(&message_str).expect("Wrong json");

        match port {
            0 => panic!("Invalid port 0"),
            3 => waspmote_parse(&message, &influx),
            1 => waspmote_parse(&message, &influx),
            224...255 => panic!("Invalid port >223"),
            _ => println!("New message: {},  {}", port, message),
        }

    }

}

use influxdb::Connection;

fn waspmote_parse(message: &json::Value, conn: &influxdb::Connection) {
    use monitor::waspmote::decode;

    println!("New waspmote message with:");

    let lines = decode(&base64::decode(message["payload"].as_str().unwrap()).unwrap(),
                       message["device"].as_str().unwrap());


    conn.write(&lines).unwrap();

}
