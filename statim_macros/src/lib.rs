use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::*;
use std::{collections::HashSet, sync::Mutex};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, DeriveInput, FnArg, ItemFn, Token,
    Type,
};

fn extract_function_args(args: &Punctuated<FnArg, Token![,]>) -> Vec<Type> {
    args.into_iter()
        .map(|a| match a {
            FnArg::Receiver(_) => unimplemented!(),
            FnArg::Typed(t) => *t.ty.clone(),
        })
        .collect()
}

struct FunctionNode {
    function_name: String,
    args_types: Vec<Type>,
}
impl Parse for FunctionNode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let function = ItemFn::parse(input)?;
        let function_name = format!("{}", function.sig.ident.clone());
        let args_types = extract_function_args(&function.sig.inputs);

        Ok(Self {
            function_name,
            args_types,
        })
    }
}
lazy_static! {
    static ref S: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[proc_macro_attribute]
pub fn command(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ii = input.clone();
    let ast = parse_macro_input!(ii as FunctionNode);
    let original_function_name = syn::Ident::new(
        &ast.function_name,
        syn::spanned::Spanned::span(&ast.function_name),
    );
    let dispacher_function_name = syn::Ident::new(
        &format!("{}_", &original_function_name),
        syn::spanned::Spanned::span(&original_function_name),
    );

    let call_args = ast
        .args_types
        .iter()
        .enumerate()
        .map(|(i, _t)| quote! {args.get(#i).cloned().unwrap().into()});

    let dispatcher = quote! {
        fn #dispacher_function_name(args: &[resp::Expr]) -> futures::future::BoxFuture<Result<String>> {
        Box::pin(async move { #original_function_name(#(#call_args),*).await })
    }
    };

    let input: proc_macro2::TokenStream = input.into();

    input
        .into_iter()
        .chain(dispatcher.into_iter())
        .collect::<proc_macro2::TokenStream>()
        .into()
}

#[proc_macro]
pub fn build_table(_input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro_derive(toParams)]
pub fn to_params(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = ast.ident;

    let syn::Data::Struct(mut data_struct) = ast.data else {panic!("");};
    let atts: Vec<_> = data_struct
        .fields
        .iter_mut()
        .map(|att| att.ident.as_mut().unwrap())
        .enumerate()
        .map(|(i, att)| quote! {#att: self.get(#i).unwrap().clone().into()})
        .collect();

    let res = quote! {
            impl Into<#struct_ident> for Vec<crate::resp::Expr> {
                fn into(self) -> #struct_ident {
                    #struct_ident {#(#atts),*}
                }
    }
    };

    res.into()
}