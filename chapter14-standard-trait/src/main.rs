use std::fmt;
use std::fmt::{Formatter, write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::ops::{Deref, DerefMut};

fn print(v: impl Into<IpAddr>) {
    println!("{:?}", v.into())
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}


impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Elixir => "Elixir",
            Language::Haskell => "Haskell",
        }
    }
}

fn print_ref(v: impl AsRef<str>) {
    println!("{:?}", v.as_ref())
}


#[derive(Debug)]
struct Buffer<T>(Vec<T>);


impl<T> Buffer<T> {
    fn new(v: impl Into<Vec<T>>) -> Self {
        Self(v.into())
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[derive(Debug, Clone, Default)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}

impl Developer {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}


impl fmt::Display for Developer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({} years old): {:?} developer",
            self.name, self.age, self.lang
        )
    }
}

fn main() {
    let developer1 = Developer::default();
    let developer2: Developer = Default::default();
    let developer3 = Developer::new("czl");
    println!("developer1: {:?} ,developer2: {:?}, developer3: {:?}", developer1, developer2, developer3);
    println!("developer1: {} ,developer2: {}, developer3: {}", developer1, developer2, developer3);
    let mut arr = [1, 3, 4, 2];
    arr.sort();
    println!("arr: {:?}", arr);
    let mut buf = Buffer::new([1, 3, 5, 4]);
    buf.sort();
    println!("buf: {:?}", buf);

    let lang = Language::Rust;

    print_ref("Hello world");
    print_ref("hello".to_string());
    print_ref(lang);


    let v4: Ipv4Addr = "2.2.2.2".parse().unwrap();
    let v6: Ipv6Addr = "::1".parse().unwrap();
    print([1, 1, 1, 1]);
    print([0xfe80, 0, 0, 0, 0xaede, 0x48ff, 0xfe00, 0x1122]);
    print(v4);
    print(v6);
}
