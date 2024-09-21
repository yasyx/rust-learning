use std::process::Output;


struct  MyFnOnce;





fn fn_once<F>(func: F)
where F: FnOnce(usize) ->  bool + Copy
{
    println!("{}", func(3));
    println!("{}", func(4)); 
}


fn fn_once1<F>(func: F)
where F: FnOnce(i32) 
{
    func(30);
    // func(31);
}

fn main () {
    // let x =  vec![1, 2, 3, 4];
    // fn_once(|z| z == x.len());



    // let mut s = String::new();


    // let update_string = |str| s.push_str(str);
    
    // exec(update_string);


    let y =  1231;


    let add = |c| {
        // y = y + c;
        println!("{}",  y + c);
    };
    
    // exec1(add);
    
    // add(3);

    // println!("{}", y);
    

    fn_once1(add);

    fn_once1(add);



    // println!("{}", s);


    // let s1 = String::from("a");
    // test(s1);

}



fn exec <'a, F: FnMut(&'a str)> (mut f : F) {
    f("hello world");
}

fn exec1 <'a, F: FnMut(i32)> (mut  f : F) {
    f(1);
}

fn test(mut str : String) {
    str.push_str("b");
    println!("{}", str);
}