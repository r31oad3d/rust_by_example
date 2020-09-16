#[derive(Debug)]
struct MyCell<T> {
    value: T,
}

impl<T> MyCell<T> {
    fn new(v: T) -> Self
    where
        T: Copy,
    {
        MyCell { value: v }
    }

    fn set(&self, v: T)
    where
        T: Copy,
    {
        unsafe {
            let p = &(self.value) as *const T as *mut T;
            *p = v;
        }
    }

    fn get(&self) -> T
    where
        T: Copy,
    {
        self.value
    }
}

struct Table<'arg> {
    cell: MyCell<&'arg isize>,
}

fn evil<'long, 'short>(t: &Table<'long>, s: &'short isize)
where
    'long: 'short,
{
    let u: &Table<'short> = t;
    u.cell.set(s);
}

fn innocent<'long>(t: &Table<'long>) {
    let foo: isize = 1;
    evil(t, &foo)
}

fn main() {
    let c = MyCell::new(10);
    c.set(11);
    println!("{:?}", c);
    let p = &c;
    p.set(12);
    println!("{:?}", c.get());
    println!("{:?}", p);

    let local = 100;
    let table = Table {
        cell: MyCell::new(&local),
    };
    innocent(&table);
    let p = table.cell.get();
    println!("{:?}", p);
}
