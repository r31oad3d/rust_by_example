#[cfg(test)]
mod test1 {
    use testlib2::compute::add;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
