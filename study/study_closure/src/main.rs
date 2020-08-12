fn main() {
    // let func1 = fn(i32) ->i32;

    let list_of_numbers = vec![1,2,3];
    let list_of_strings_1 = list_of_numbers.iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>();

    let list_of_strings_2 = list_of_numbers.iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    assert_eq!(list_of_strings_1,list_of_strings_2);

    enum Status {
        Value(u32),
        Stop,
    }

    let _list_of_statuses = (0u32..20)
        .map(Status::Value)
        .collect::<Vec<Status>>();

    fn _returns_closure() -> Box<Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}

