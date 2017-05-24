use std::f64;
use std::str;
use std::str::FromStr;

#[derive(Debug)]
pub struct Measurement
{
    pub name: String,
    pub value: f64,
}

trait AddMeasurement {
    fn add(&mut self, name: &str, value: &str);
}

impl AddMeasurement for Vec<Measurement> {
    fn add(&mut self, name: &str, value: &str){
        let data = f64::from_str(value).unwrap();
        let mut data_struct = Measurement {name: name.to_string(),  value: data};
        self.push(data_struct);
    }
}

pub fn decode(data: &[u8] ) -> Vec<Measurement>
{
    let mut measurements: Vec<Measurement> =  Vec::new();

    let data = String::from_utf8_lossy(&data);

    println!("{}", data);


    for item in  data.split('#')  {


        let datasplit: Vec<&str> = item.split(':').collect();

        if datasplit.len() != 2 {
            continue;
        }

        println!("item: {}, part1: {}, part2: {}", item, datasplit[0], datasplit[1]);

        match datasplit[0] {
            "BAT"       => measurements.add("battery", datasplit[1]),
            "PRES"      => measurements.add("pressure", datasplit[1]),
            "NO2"       => measurements.add("no2", datasplit[1]),
            "O3"        => measurements.add("o3", datasplit[1]),
            "PM1"       => measurements.add("particle_1um", datasplit[1]),
            "PM2_5"     => measurements.add("particle_2.5um", datasplit[1]),
            "PM10"      => measurements.add("particle_10um", datasplit[1]),
            "LUX"       => measurements.add("lux", datasplit[1]),
            "HUM"       => measurements.add("humidity", datasplit[1]),
            _ => (),
        }
    }
    measurements
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
