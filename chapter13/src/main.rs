use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

struct Memo<T: Eq + Hash + Clone, U> {
    computation: Box<Fn(&T) -> U>,
    cache: HashMap<T, U>,
}

impl<T: Eq + Hash + Clone, U> Memo<T, U> {
    fn new(computation: Box<Fn(&T) -> U>) -> Memo<T, U> {
        Memo {
            computation,
            cache: HashMap::new(),
        }
    }

    fn get(&mut self, k: T) -> &U {
        let cache = &mut self.cache;
        let computation = &self.computation;

        let klone = k.clone();

        cache.entry(k).or_insert_with(|| (computation)(&klone))
    }
}

fn main() {
    let mut memo = Memo::new(Box::new(|param: &String| {
        println!("expensive computation {}", param);
        format!("result {}", param)
    }));

    let res = memo.get(String::from("ayy"));
    println!("{:?}", res);

    let res = memo.get(String::from("lmao"));
    println!("{:?}", res);

    let res = memo.get(String::from("wow"));
    println!("{:?}", res);

    let res = memo.get(String::from("ayy"));
    println!("{:?}", res);

    let res = memo.get(String::from("wow"));
    println!("{:?}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    static mut CALLS: u8 = 0;

    #[test]
    fn computes_once() {
        unsafe {
            let mut memo = Memo::new(Box::new(|param: &i32| {
                CALLS += 1;
                param * param
            }));

            memo.get(1);
            memo.get(1);
            assert_eq!(CALLS, 1);
            memo.get(2);
            memo.get(2);
            assert_eq!(CALLS, 2);
        }
    }
}
