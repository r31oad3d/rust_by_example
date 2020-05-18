pub fn test4_1() {
    ['🐱', '🐶', '🐦'].iter().for_each(|&animal| match animal {
        '🐱' => println!("meow"),
        '🐶' => println!("bark"),
        '🐦' => println!("chrip"),
        _ => println!(),
    });
}
