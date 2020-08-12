//#![feature(nll)]

use std::any::Any;
use std::ops::Add;

pub fn func1() {
    //    5.times(|| {println!("hello world");});
    let mut x = 22;

    let p = &mut x;

    println!("{}", x);
    let arr = [1, 2, 3, 4, 5];

    assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    assert_eq!(&arr[1..], &[2, 3, 4, 5]);

    assert_eq!(arr.len(), 5);

    let mut x = 10;
    let ptr_x = &mut x as *mut i32;

    let y = Box::<i32>::new(20);
    let ptr_y = &*y as *const i32;
    std::mem::drop(ptr_y);
    unsafe {
        *ptr_x += *ptr_y;
    }

    assert_eq!(x, 30);
    //3 as !;

    //    fn Foo() -> ! {
    //        return 3 as !;
    //    }
    //Foo() as 3;
    fn get_a_number() -> Option<u32> {
        None
    }
    //    let num = match get_a_number() {
    //        Some(num) => num,
    //        None => panic!("ahhhhhhh"),
    //    };

    let mut buf = std::collections::VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);
    assert_eq!(buf.get(0), Some(&2));
    buf.push_back(3);
    assert_eq!(buf.get(2), Some(&3));

    let mut list1 = std::collections::LinkedList::new();
    list1.push_back('a');
    let mut list2 = std::collections::LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');

    list1.append(&mut list2);

    println!("{:?}", list1);
    println!("{:?}", list2);

    let mut hmap = std::collections::HashMap::new();
    let mut bmap = std::collections::BTreeMap::new();

    hmap.insert(3, 'c');
    hmap.insert(2, 'e');
    hmap.insert(1, 'a');
    hmap.insert(5, 'b');
    hmap.insert(6, 'd');

    bmap.insert(3, 'c');
    bmap.insert(2, 'e');
    bmap.insert(1, 'a');
    bmap.insert(5, 'b');
    bmap.insert(6, 'd');

    println!("{:?}", hmap);
    println!("{:?}", bmap);

    let mut hbooks = std::collections::HashSet::<i32>::new();
    let mut bbooks = std::collections::BTreeSet::<i32>::new();

    let mut heap = std::collections::BinaryHeap::<i32>::new();
    assert_eq!(heap.peek(), Option::None);
    let arr = [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45, 109];

    for &i in arr.iter() {
        heap.push(i);
    }

    assert_eq!(heap.peek(), Some(&109));

    println!("{:?}", heap);

    println!("{}", std::mem::size_of::<&[u32; 5]>());
    println!("{}", std::mem::size_of::<&[u32]>());
    println!("{}", std::mem::size_of::<&mut [u32]>());

    enum Void {}
    struct Foo;
    struct Baz {
        foo: Foo,
        qux: (),
        baz: [u8; 0],
    }
    println!("{}", std::mem::size_of::<Void>());
    println!("{}", std::mem::size_of::<Foo>());
    println!("{}", std::mem::size_of::<Baz>());
    println!("{}", std::mem::size_of::<()>());
    println!("{}", std::mem::size_of::<[(); 10]>());

    let res: Result<u32, Void> = Ok(0);
    //let Ok(num) = res;

    println!("{}", "aaaa".to_owned() + "bbbb");
    println!("{}", "aaaa".to_string() + "bbbb");
    println!("{}", "aaaa".to_string() + "bbbb");

    trait t1 {
        fn tfun() {
            println!("{}", "t1");
        }
    }

    trait t2 {
        fn tfun() {
            println!("{}", "t2");
        }
    }

    trait tt: t1 + t2 {
        fn tfun() {
            println!("{}", "tt");
        }
    }
    //t1::tfun();
    struct s1 {}
    impl t1 for s1 {}
    impl t2 for s1 {}
    impl tt for s1 {}
    <s1 as t1>::tfun();

    //    fn sum<T: Add<T, Output=T>>(a: T, b: T) -> T {
    //        a + b
    //    }
    //
    //    fn sum_v2<T>(a: T, b: T) -> T where T: Add<T, Output=T> {}
}
