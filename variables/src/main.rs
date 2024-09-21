fn main() {

    let test = Box::new([0u8; 100000000000]);

    println!("{:?}", test);

    let test = vec![0u8; 100000000000];

    println!("{:?}",test);
}