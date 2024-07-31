use std::{fmt, ops::Deref, str};
use std::fmt::Formatter;

const MINI_STRING_MAX_LEN: usize = 30;

struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}


impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}


impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}


#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}


impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}


impl From<&str> for MyString {
    fn from(value: &str) -> Self {
        match value.len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(value.to_owned()),
            _ => Self::Inline(MiniString::new(value)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}


fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: MyString {}, MiniString: {}", len1, len2);
    let str1: MyString = "hello world".into();
    let str2: MyString = "这是一个很长长长长长长长长长长长长长长长长长长长长的字符串".into();

    println!("str1:{:?}, str2:{:?}", str1, str2);

    println!(
        "str1:{} ({}bytes, {} chars), str2:{} ({} bytes,{} chars)",
        str1,
        str1.len(),
        str1.chars().count(),
        str2,
        str2.len(),
        str2.chars().count()
    );
    assert!(str1.ends_with("world"));
    assert!(str2.starts_with("这"));
}
