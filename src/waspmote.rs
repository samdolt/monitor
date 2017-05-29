use std::f64;
use std::str;
use std::str::FromStr;

use influxdb::Lines;

trait AddMeasurement {
    fn add(self, name: &str, value: &str, device: &str) -> Lines;
}

impl AddMeasurement for Lines {
    fn add(self, name: &str, value: &str, device: &str) -> Lines {
        let data = f64::from_str(value).unwrap();

        self.add_line(name)
            .add_tag("device", device)
            .add_field("value", data)
            .build()
    }
}

pub fn decode(data: &[u8], device: &str) -> Lines {
    let mut lines = Lines::new();

    let data = String::from_utf8_lossy(data);

    println!("{}", data);


    for item in data.split('#') {


        let datasplit: Vec<&str> = item.split(':').collect();

        if datasplit.len() != 2 {
            continue;
        }

        println!("item: {}, part1: {}, part2: {}",
                 item,
                 datasplit[0],
                 datasplit[1]);

        lines = match datasplit[0] {
            "BAT" => lines.add("battery", datasplit[1], device),
            "PRES" => lines.add("pressure", datasplit[1], device),
            "NO2" => lines.add("no2", datasplit[1], device),
            "O3" => lines.add("o3", datasplit[1], device),
            "PM1" => lines.add("particle_1um", datasplit[1], device),
            "PM2_5" => lines.add("particle_2.5um", datasplit[1], device),
            "PM10" => lines.add("particle_10um", datasplit[1], device),
            "LUX" => lines.add("lux", datasplit[1], device),
            "HUM" => lines.add("humidity", datasplit[1], device),
            _ => lines,
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = "<=>†#065467057C10542A#01#1#BAT:46#PRES:96620.91#NO2:2.661#O3:0.000#";

        println!("{:?}", decode(data.as_bytes()));
    }
}
