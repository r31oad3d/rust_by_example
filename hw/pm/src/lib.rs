use proc_macro::TokenStream;

#[proc_macro]
pub fn hw(_: TokenStream) -> TokenStream {
    r#"println!("Hello World!");"#.parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
