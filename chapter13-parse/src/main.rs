use std::f64;
use std::ops::Add;
use std::str::FromStr;
use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

pub trait ParseT {
    type Error;
    fn parse_t(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let re = Regex::new("^[0-9]+").unwrap();
        if let Some(captures) = re.captures(s) {
            captures.get(0)
                .map_or(0, |s| s.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }
}

impl Parse for f64 {
    fn parse(s: &str) -> Self {
        let regex = Regex::new(r"[0-9,.]+").unwrap();
        if let Some(captures) = regex.captures(s) {
            captures.get(0)
                .map_or(0.0, |s| s.as_str().parse().unwrap_or(0.0))
        } else {
            0.0
        }
    }
}


impl<T> ParseT for T
where
    T: FromStr + Default,
{
    type Error = String;

    fn parse_t(s: &str) -> Result<Self, Self::Error> {
        let regex = Regex::new("[0-9,.]+").unwrap();
        if let Some(cap) = regex.captures(s) {
            cap.get(0)
                .map_or(Err("failed to capture".to_string()),
                        |s| s.as_str().parse().map_err(|_err| "failed to parse captured string".to_string()))
        } else {
            Err("failed to parse string".to_string())
        }
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self {
            real,
            imagine,
        }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}


fn main() {
    println!("result u8: {}", u8::parse("255 hello world"));
    println!("result f64: {}", f64::parse("12.35555 hello world"));
    println!("result T for f64: {:?}", f64::parse_t("12.35555 hello world"));
    println!("result T for u8: {:?}", u8::parse_t("255 hello world"));
    println!("result T for f64: {:?}", f64::parse_t("hello world"));
    println!("result T for u8: {:?}", u8::parse_t("hello world"));
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2f64, 3.0);
    println!("{:?}", c1 + c2)
}
