// #[proc_macro_derive(HelloMacro)]
// pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
//     let ast: DeriveInput = syn::parse(input).unwrap();
//     println!("{}", stringify!(ast));
//     impl_hello_macro(&ast)
// }
//
// fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
//     let name = &ast.ident;
//     let gen = quote! {
//         impl HelloMacro for #name {
//             fn hello_macro() {
//
//                 println!("Hello, Macro! my name is {}", stringify!(#name));
//             }
//         }
//     };
//     gen.into()
// }
