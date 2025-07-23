struct Cacher<T,E>
where 
    T: Fn(E) -> E
    {
        query: T,
        value: Option<E>
    }

impl<T, E> Cacher<T, E> 
    where 
        T: Fn(E) -> E,
        E: Copy
    {
    fn new(query: T) -> Cacher<T,E> {
        Cacher {
            query,
            value: None
        }
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
    println!("Hello, world!");
}
