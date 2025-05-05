use std::{collections::BTreeMap, process::id};

use darling::FromMeta;
use proc_macro::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    parse::Parse, parse_quote, punctuated::Punctuated, spanned::Spanned, token, Attribute, DeriveInput, Expr, ExprArray, ExprLit, FieldsUnnamed, Generics, Ident, ImplItem, ImplItemConst, ItemEnum, ItemImpl, ItemStruct, Lit, LitInt, Path, Token, Type, Variant, VisPublic, Visibility
};

fn get_variants(item: TokenStream) -> TokenStream {
    let item: syn::ItemEnum = match syn::parse(item) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into_token_stream().into(),
    };
    let ident = item.ident.clone();
    let mut variants = Vec::new();
    for variant in item.variants.iter() {
        if variant.fields.len() > 0 {
            return syn::Error::new(variant.span(), "Variants cannot have fields")
                .into_compile_error()
                .into();
        }
        if variant.discriminant.is_some() {
            return syn::Error::new(variant.span(), "Variants cannot have discriminants")
                .into_compile_error()
                .into();
        }
        variants.push(variant.ident.clone());
    }
    let count = variants.len();
    ItemImpl {
        attrs: vec![],
        defaultness: None,
        unsafety: None,
        impl_token: token::Impl(item.span()),
        generics: Generics {
            gt_token: None,
            lt_token: None,
            params: Punctuated::new(),
            where_clause: None,
        },
        trait_: None,
        self_ty: Box::new(parse_quote! {#ident}),
        brace_token: token::Brace(item.span()),
        items: vec![ImplItem::Const(ImplItemConst {
            attrs: vec![],
            vis: parse_quote!(pub),
            defaultness: None,
            const_token: token::Const(item.span()),
            ident: parse_quote!(VARIANTS),
            colon_token: token::Colon(item.span()),
            ty: parse_quote!([#ident;#count]),
            eq_token: token::Eq(ident.span()),
            expr: Expr::Array(ExprArray {
                attrs: vec![],
                bracket_token: token::Bracket(item.span()),
                elems: Punctuated::from_iter(
                    variants
                        .into_iter()
                        .map(|a| -> Expr { parse_quote!(#ident::#a) }),
                ),
            }),
            semi_token: token::Semi(item.span()),
        })],
    }
    .to_token_stream()
    .into()
}

#[proc_macro_attribute]
pub fn gen_variants(_: TokenStream, mut item: TokenStream) -> TokenStream {
    item.extend(get_variants(item.clone()));

    item
}
fn _error_types(kind: TokenStream) -> TokenStream {
    let x: ItemEnum = match syn::parse(kind) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error().into(),
    };
    let ident = x.ident;

    quote! {

        #[macro_use]
        mod ErrDef {
            use std::fmt::Debug;
            use std::backtrace::Backtrace;
            use super::*;

        pub struct Error {
            kind: #ident,
            message: Option<String>,
            backtrace: std::backtrace::Backtrace,
        }
        impl Error {
            pub fn new<T:ToString>(kind:#ident,message:Option<T>,backtrace:std::backtrace::Backtrace) -> Self {
                Self {
                    kind,
                    message: message.map(|m| m.to_string()),
                    backtrace
                }
            }

        }
        #[macro_export]
        macro_rules! new_error {
            
            ($kind:ident) => {
                $crate::error::Error::new::<String>($crate::error::ErrorKind::$kind, None, std::backtrace::Backtrace::capture())
            };
            ($kind:ident, $e:literal) => {
                $crate::error::Error::new($crate::error::ErrorKind::$kind, Some($e), std::backtrace::Backtrace::capture())
            };
            ($kind:ident, $e:expr) => {
                $crate::error::Error::new($crate::error::ErrorKind::$kind, Some($e), std::backtrace::Backtrace::capture())
            };
        }
        impl Debug for Error {
            fn fmt(&self,f:&mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f,"{:?}: ",self.kind)?;
                if let Some(ref s) = self.message {
                    write!(f,"{}",s)?;
                }
                write!(f,"\n{}",self.backtrace)?;

                Ok(())
            }
        }
        }
        #[macro_use]
        pub use ErrDef::*;
    }
    .into()
}
#[proc_macro_attribute]
pub fn gen_error_types(_: TokenStream, mut item: TokenStream) -> TokenStream {
    item.extend(_error_types(item.clone()));

    item
}

fn _gen_flags((attrs,item):(TokenStream,TokenStream)) -> Result<TokenStream,syn::Error> {
    let mut item_enum: ItemEnum = syn::parse(item)?;
    let struct_ident: Ident = syn::parse(attrs)?;
    let enum_ident = item_enum.ident.clone();
    let vis = item_enum.vis.clone();
    let count = item_enum.variants.len();
    let mident = Ident::from_string(&format!("__{struct_ident}__")).unwrap();
    let uintsize = {
        let mut i = 8;
        while count > i {
            i += 1;
        }
        i
    };
    let mut map = BTreeMap::<usize,Variant>::new();
    let mut disc = 1;
    for var in item_enum.variants.clone().into_iter() {
        if !var.fields.is_empty() {
            return Err(syn::Error::new(var.fields.span(), "Invalid Variant: No fields allowed for flags"));
        }
        let discriminant = match var.discriminant {
            Some((_,Expr::Lit(ExprLit { lit: Lit::Int(ref a),..}))) =>  a.base10_parse()?,
            None => {
                let r = disc;
                disc *= 2;
                r
            },
            Some((_,e)) => return Err(syn::Error::new(e.span(), "Invalid value for expression: must be a literal int")),

        };
        match map.insert(discriminant, var) {
            Some(v) => {
                return Err(syn::Error::new(v.span(),"Duplicate keys beweeen variants"));
            },
            None => {
                map.get_mut(&discriminant).unwrap().discriminant = Some((token::Eq(struct_ident.span()),parse_quote!(#discriminant)));
            },
        }
    }
    let repr_vis =
    Visibility::Public(VisPublic { pub_token: Token!(pub)(struct_ident.span())});
    item_enum.vis = repr_vis.clone();
    let tp = Type::from_string(&format!("u{}",uintsize)).unwrap();
    item_enum.attrs.push(
        Attribute {
            pound_token:token::Pound(struct_ident.span()),
            style: syn::AttrStyle::Outer,
            bracket_token: token::Bracket(struct_ident.span()),
            path: Path::from_string("repr").unwrap(),
            tokens: quote! { (#tp) }

        }
    );

    item_enum.attrs.push(
        Attribute {
            pound_token:token::Pound(struct_ident.span()),
            style: syn::AttrStyle::Outer,
            bracket_token: token::Bracket(struct_ident.span()),
            path: Path::from_string("derive").unwrap(),
            tokens: quote! { (Clone,Copy,PartialEq,Eq) }
        }
    );
    let item_struct = ItemStruct {
        attrs: vec![],
        vis:repr_vis.clone(),
        struct_token: token::Struct(struct_ident.span()),
        ident:struct_ident.clone(),
        generics: Generics { lt_token: None, params: Punctuated::new(), gt_token: None, where_clause: None },
        fields: syn::Fields::Unnamed( parse_quote!((#tp))),
        semi_token: Some(token::Semi(struct_ident.span())),
    };



    // todo!("{:?}",ident.to_string());

    return Ok(quote! {
        #[automatically_derived]
        #[allow(dead_code)]
        #vis mod #mident {
            use super::{FlagValue,FlagPrimitive,FlagValues,FlagIterator};

            #item_enum

            impl FlagValue<#tp> for #enum_ident {
                const MASK: #tp = 0;
                type Collection = #struct_ident;

                #[inline]
                fn to_primitive(&self) -> #tp {
                    *self as #tp
                }
                #[inline]
                unsafe fn from_primitive(value: &#tp) -> Self {
                    *value as #enum_ident
                }
            }

            #[derive(Clone,Copy)]
            #[repr(transparent)]
            #item_struct

            impl From<#enum_ident> for #struct_ident {
                fn from(value:#enum_ident) -> Self {
                    Self(value as #tp)
                }
            }
            impl FlagValues<#enum_ident,#tp> for #struct_ident {
                fn into(self) -> #tp {
                    self.0 
                }
                fn from(value:#tp) -> Self {
                    Self(value)
                }
            } 

            // impl IntoIterator for #struct_ident {
            //     type Item = #enum_ident;
            //     // type IntoIter = todo!();
            //     fn into_iter(self) -> Self::IntoIter {

            //     }

            // }
        }
        pub use #mident::*;
    }.into());

}

fn handle_error<T,E:Fn(T) -> Result<TokenStream, syn::Error>>(input:T,f:E) -> TokenStream {
    match f(input) {
        Ok(t) => t,
        Err(e) => e.to_compile_error().into()
    }
}

#[proc_macro_attribute]
pub fn gen_flags(attrs:TokenStream, item: TokenStream) -> TokenStream {
      handle_error((attrs,item.clone()), _gen_flags)

}
