use std::collections::HashMap;

#[derive(Clone)]
struct Fib {
    cache: HashMap<i32, i32>
}

impl Fib {
    fn new(cache: HashMap<i32, i32>) -> Fib {
        Fib { cache }
    }

    fn calc(&self, n: i32) -> (i32, Fib) {
        let found = self.cache.get(&n);

        match found {
            Some(&v) => (v, (*self).clone()),
            None => {
                let (res, f_res) = if n <= 1 {
                    (1, (*self).clone())
                } else {
                    let (a, f1) = self.calc(n - 2);
                    let (b, f2) = f1.calc(n - 1);
                    (a + b, f2)
                };

                let mut new_cache = f_res.cache.clone();
                new_cache.insert(n, res);
                (res, Fib {cache: new_cache })
            }
        }
    }
}

fn main() {

    let mut ref_fib = Fib::new(HashMap::new());

    for i in 0..100 {
        let (step_result, new_fib) = ref_fib.calc(i);
        ref_fib = new_fib;
        println!("fib({}) = {}", i, step_result.to_string());
    }
}
