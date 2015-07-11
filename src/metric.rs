

#[derive(Debug)]
pub enum Kind {
    Guage,
    Counter,
    Unknown
}

#[derive(Debug)]
pub struct Metric {
    pub name: String,
    pub value: i64,
    pub kind: Kind
}

impl Metric {
    pub fn new(line: String) -> Result<Metric, String> {
        let bits = try!(split_by(&line, '|'));
        let mut m = try!(Metric::parse(bits[0].to_string()));

        match bits[1] {
            "g" => m.kind = Kind::Guage,
            "c" => m.kind = Kind::Counter,
            _ => m.kind = Kind::Unknown
        }
        Ok(m)
    }

    pub fn parse(line: String) -> Result<Metric, String> {
        let bits = try!(split_by(&line, ':'));
        let value = try!(bits[1].trim().parse::<i64>().map_err(|e| e.to_string()));
        return Ok(Metric{name: bits[0].to_string(), value: value, kind: Kind::Unknown})
    }    
}

fn split_by<'a>(line: &'a String, c: char) -> Result<Vec<&'a str>, String> {
    let bits: Vec<&str> = line.trim_right().split(c).collect();
    if bits.len() != 2 {
        return Err(format!("Improperly formed metrics line. {}", line).to_owned());
    }
    return Ok(bits);
}

