#[macro_use]
extern crate serde_derive;

extern crate redis;
extern crate serde;

#[macro_use]
extern crate serde_json;

use redis::Commands;

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    for i in 0..100000 {
        let jdata = r#"{"app_id":"my-app-id","dev_id":"my-dev-id","hardware_serial":"0102030405060708","port":1,"counter":2,"is_retry":false,"confirmed":false,"payload_raw":"AQIDBA==","payload_fields":{},"metadata":{"time":"1970-01-01T00:00:00Z","frequency":868.1,"modulation":"LORA","data_rate":"SF7BW125","coding_rate":"4/5","gateways":[{"gtw_id":"ttn-herengracht-ams","timestamp":12345,"time":"1970-01-01T00:00:00Z","channel":0,"rssi":-25,"snr":5,"rf_chain":0,"latitude":52.1234,"longitude":6.1234,"altitude":6}],"latitude":52.2345,"longitude":6.2345,"altitude":2}}"#;
        parse(&con, jdata);
    }
}

fn parse(con: &redis::Connection, jdata: &str){
    let data: serde_json::Value = serde_json::from_str(jdata).unwrap();


    let dev_eui: &str = data["hardware_serial"].as_str().unwrap();
    let payload: &str = data["payload_raw"].as_str().unwrap();
    let port: u64 = data["port"].as_u64().unwrap();
    let rx_time = data["metadata"]["time"].as_str().unwrap();

    let enc_data = json!({
        "device": dev_eui,
        "content": payload,
        "port": port,
        "datetime": rx_time,
    });

    let _ : () = con.lpush("rx", enc_data.to_string() as String).unwrap();
}
