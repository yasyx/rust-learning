fn get_memory_location() -> (usize,usize){
    let string = "hello world";
    let ptr = string.as_ptr() as usize;
    let len = string.len();
    
    (ptr, len)
}

fn get_value_by_loation(ptr : usize, len : usize) {
    println!("{},{}",ptr,len);
    unsafe {
        let slice = std::slice::from_raw_parts(ptr as *const u8, len);
        println!("{:?}", std::str::from_utf8_unchecked(slice));
    }
}


fn main() {

    let (ptr, len) = get_memory_location();

    get_value_by_loation(ptr, len);

}
