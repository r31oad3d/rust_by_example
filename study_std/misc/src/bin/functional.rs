struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let add_v3 = |x: i32| x + 1;
    let add_v4 = |x: i32| x + 1;

    let example_1 = move |x: i32| x;

    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", v1.iter().sum::<i32>());
    println!(
        "{:?}",
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            .iter()
            .filter(|x| *x % 2 == 0)
            .sum::<i32>()
    );
    ['a', 'b', 'c', 'd']
        .iter()
        .for_each(|char| println!("{:?}", char));
    let mut cnt = Counter::new();
    println!("{:?}", cnt.next());
    let sum = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum::<u32>();
    println!("{:?}", sum);
}
