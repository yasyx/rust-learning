use std::rc::Rc;
use std::boxed::Box;
fn main() {
    let mut arr: [i32; 2] = [1, 2];

    let arr_ptr = arr.as_mut_ptr();
    let arr_ptr_addr = arr_ptr as usize;
    let arr_ptr2 = (arr_ptr_addr + 4) as *mut i32;
    unsafe {
        *arr_ptr2 = 10;
    }

    println!("{:?}", arr); // Output: [10, 2]

    let array: Rc<Box<[i32; 3]>> = Rc::new(Box::new([1, 2, 3]));

    println!("{:?}", array[0]); // Output:
}
