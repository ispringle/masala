///# Masala
///
/// An autocurrying macro for Rust.
///
/// ## Usage
///
/// This crate requires nightly:
///
///```rust
///#![feature(type_alias_impl_trait, min_type_alias_impl_trait)]
///use masala::curry;
///
///#[curry]
///fn add(a: isize, b: isize) -> isize {
///    a + b
///}
///
///fn main() {
///    println!("{}", add(33)(42));
///}
///```
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Block, FnArg, ItemFn, Pat, ReturnType, Type, TypeGenerics};

#[proc_macro_attribute]
pub fn curry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(item as ItemFn);
    cook_curry(parsed).into()
}

fn cook_curry(parsed: ItemFn) -> proc_macro2::TokenStream {
    let fn_body = parsed.block;
    let sig = parsed.sig;
    let (impl_generics, ty_generics, _) = sig.generics.split_for_impl();
    let vis = parsed.vis;
    let fn_name = sig.ident;
    let fn_args = sig.inputs;
    let fn_return_type = sig.output;

    let arg_idents = extract_arg_idents(fn_args.clone());
    let first_ident = &arg_idents.first().unwrap();

    let curried_body = generate_body(&arg_idents[1..], fn_body.clone());

    let arg_types = extract_arg_types(fn_args.clone());
    let first_type = &arg_types.first().unwrap();
    let type_aliases = generate_type_aliases(
        &arg_types[1..],
        extract_return_type(fn_return_type),
        &fn_name,
        ty_generics.clone(),
    );
    let return_type = format_ident!(
        "{}{}",
        title_case(&fn_name.to_string()),
        format!("T{}", type_aliases.len() - 1)
    );

    quote! {
        #(#type_aliases);* ;
        #vis fn #fn_name #impl_generics (#first_ident: #first_type) -> #return_type #ty_generics {
            #curried_body ;
        }
    }
}

fn extract_arg_idents(fn_args: Punctuated<FnArg, syn::token::Comma>) -> Vec<Box<Pat>> {
    fn_args.into_iter().map(extract_arg_pat).collect::<Vec<_>>()
}

fn extract_arg_pat(a: FnArg) -> Box<Pat> {
    match a {
        FnArg::Typed(p) => p.pat,
        _ => panic!("Not supported on types with `self!`"),
    }
}

fn extract_arg_types(fn_args: Punctuated<FnArg, syn::token::Comma>) -> Vec<Box<Type>> {
    fn_args.into_iter().map(extract_type).collect::<Vec<_>>()
}

fn extract_return_type(a: ReturnType) -> Box<Type> {
    match a {
        ReturnType::Type(_, p) => p,
        _ => panic!("Not supported on functions without return types!"),
    }
}

fn extract_type(a: FnArg) -> Box<Type> {
    match a {
        FnArg::Typed(p) => p.ty,
        _ => panic!("Not supported on types with `self!`"),
    }
}

fn generate_body(fn_args: &[Box<Pat>], body: Box<Block>) -> proc_macro2::TokenStream {
    quote! {
        return #( move |#fn_args| )* #body
    }
}

fn generate_type_aliases(
    fn_arg_types: &[Box<Type>],
    fn_return_type: Box<Type>,
    fn_name: &syn::Ident,
    type_generics: TypeGenerics,
) -> Vec<proc_macro2::TokenStream> {
    let type_t0 = format_ident!("{}T0", title_case(&fn_name.to_string()));
    let mut type_aliases = vec![quote! { type #type_t0 #type_generics = #fn_return_type}];
    for (i, t) in (1..).zip(fn_arg_types.into_iter().rev()) {
        let func = title_case(&fn_name.to_string());
        let p = format_ident!("{}{}", func, format!("T{}", i - 1));
        let n = format_ident!("{}{}", func, format!("T{}", i));
        type_aliases.push(quote! {
            type #n #type_generics = impl Fn(#t) -> #p #type_generics
        });
    }
    type_aliases
}

fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
