fn main() {
    let arr = vec![1,2,3];

    for i in arr.iter() {
        println!("{}", i);
        
    }
    
    println!("================================================");
    
    for i in arr.into_iter() {
        println!("{}", i);
    }

    println!("================================================");

    // println!("{:?}", arr);

    let mut arr = vec![1,2,3];
    for i in arr.iter_mut() {
        *i *= 2;
    }

    println!("{:?}", arr);

    let mut counter = Counter::new();
    let mut i = 6;
    while i > 0  {
        println!("{:?}", counter.next());
        i -= 1;
    }
}


struct  Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }else {
            None
        }
    }
}