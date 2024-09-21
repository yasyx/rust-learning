struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy
{
    fn new(query: T) -> Self {
        Cacher { query, value: None }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let mut cacher = Cacher::new(|x| x * 2);
    println!("{}", cacher.value(3));
    println!("{}", cacher.value(3));
    panic!()
}


fn do_stuff<T>(value: &T) {
    let cloned = value.clone();
}