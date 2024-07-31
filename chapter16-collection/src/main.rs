use std::fmt;

fn main() {
    // let result: Vec<_> = vec![1, 2, 3, 3, 3, 3, 6]
    //     .iter()
    //     .map(|v| v * v)
    //     .filter(|v| *v > 1)
    //     .take(1)
    //     .collect();
    // println!("result: {:?}", result)

    let s = String::from("hello");
    print_slice(&s);

    print_slice1(&s);
    print_slice1(&s[..]);
    print_slice1(s.clone());

    print_slice2(&s);
    print_slice2(&s[..]);
    print_slice2(s);

    let arr = ['h', 'e', 'l', 'l', 'o'];
    let vec = vec!['h', 'e', 'l', 'l', 'o'];
    let s = String::from("hello");
    let s1 = &arr[1..3];
    let s2 = &vec[1..3];
    // &str 本身就是一个特殊的 slice
    let s3 = &s[1..3];
    println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);

    // &[char] 和 &[char] 是否相等取决于长度和内容是否相等
    assert_eq!(s1, s2);
    // &[char] 和 &str 不能直接对比，我们把 s3 变成 Vec<char>
    assert_eq!(s2, s3.chars().collect::<Vec<_>>());
    // &[char] 可以通过迭代器转换成 String，String 和 &str 可以直接对比
    assert_eq!(String::from_iter(s2), s3);
}


fn print_slice(s: &str) {
    println!("{:?}", s);
}


fn print_slice1<T: AsRef<str>>(s: T) {
    println!("{:?}", s.as_ref());
}

fn print_slice2<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}