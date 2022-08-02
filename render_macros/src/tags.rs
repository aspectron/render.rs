use crate::element_attributes::ElementAttributes;
use proc_macro_error::abort;
use quote::quote;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
//use syn::spanned::Spanned;
use proc_macro2::TokenTree;
//use proc_macro2::Punct;
//use proc_macro2::Spacing;
//use syn::{Path, PathSegment};
//use syn::token::Colon2;
//use proc_macro2::Span;
use syn::Ident;
use syn::parse_str;

pub struct OpenTag {
    pub name: TagName,
    pub attributes: ElementAttributes,
    pub self_closing: bool,
    pub is_custom_element: bool,
}

#[derive(Debug)]
pub struct TagName{
    pub parts:Vec<String>
}
impl TagName{
    pub fn to_string(&self)->String{
        self.parts.join("-")
    }

    pub fn is_custom_element_name(&self)->bool{
        let name = self.to_string();
        let first_letter = name.get(0..1).unwrap();
        first_letter.to_uppercase() == first_letter
    }
}
impl Parse for TagName {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut parts = vec![];
        let mut valid_ident = true;
        input.step(|cursor| {
            let mut rest = *cursor;
            while let Some((tt, next)) = rest.token_tree() {
                println!("tttttt: {:?}", tt);
                match &tt {
                    TokenTree::Ident(ident)=> {
                        if valid_ident{
                            parts.push(ident.to_string());
                            rest = next;
                            valid_ident = false;
                        }else{
                            return Ok(((), rest));
                        }
                    }
                    TokenTree::Punct(punct)=>{
                        match punct.as_char(){
                            '-'=>{
                                valid_ident = true;
                                rest = next;
                            }
                            //' ' | '>' | '/'=> {
                            _=>{
                                return Ok(((), rest));
                            }
                        }
                    }
                    _ => {
                        return Ok(((), rest));
                    }
                }
            }
            Err(cursor.error("no `>` was found"))
        })?;

        Ok(TagName{
            parts
        })
    }
}

impl ToTokens for TagName {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.to_string();
        if self.is_custom_element_name(){
            let name = Ident::new(&name, proc_macro2::Span::call_site());
            return proc_macro2::TokenStream::from(quote!(#name)).to_tokens(tokens)
        }
        proc_macro2::TokenStream::from(quote!(#name)).to_tokens(tokens)
    }
}

fn name_or_fragment(maybe_name: Result<syn::Path>) -> syn::Path {
    maybe_name.unwrap_or_else(|_| syn::parse_str::<syn::Path>("::render::Fragment").unwrap())
}

fn is_custom_element_name(tag: &TagName) -> bool {
    //match path.get_ident() {
    //    None => true,
    //    Some(ident) => {
            let name = tag.to_string();
            let first_letter = name.get(0..1).unwrap();
            first_letter.to_uppercase() == first_letter
    //    }
    //}
}
fn skip_tag_name(input:ParseStream)->Result<()>{
    input.step(|cursor| {
        let mut rest = *cursor;
        while let Some((tt, next)) = rest.token_tree() {
            match &tt {
                TokenTree::Punct(punct) if punct.as_char() == ' ' => {
                    //*tt = proc_macro2::TokenTree::Punct(Punct::new('#', Spacing::from(punct.spacing())));
                    return Ok(((), next));
                }
                TokenTree::Punct(punct) if punct.as_char() == '>' => {
                    //*tt = proc_macro2::TokenTree::Punct(Punct::new('#', Spacing::from(punct.spacing())));
                    return Ok(((), rest));
                }
                TokenTree::Punct(punct) if punct.as_char() == '/' => {
                    //*tt = proc_macro2::TokenTree::Punct(Punct::new('#', Spacing::from(punct.spacing())));
                    return Ok(((), rest));
                }
                _ => rest = next,
            }
        }
        Err(cursor.error("no `>` was found"))
    })?;
    Ok(())
}

impl Parse for OpenTag {
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("input1: {:?}", input);
        input.parse::<syn::Token![<]>()?;
        //println!("\n\n========input2: {:?}", input);
        //let s = ParseStream{};
        //skip_tag_name(input)?;
        println!("\n\n========input2: {:?}", input);
        let tag_name = input.parse::<TagName>()?;
        /*
        let mut seg = Punctuated::<PathSegment, Colon2>::new();
        let path_seg = parse_str::<PathSegment>("flow")?;
        //seg.insert(0, PathSegment::from(Ident::new("ssss", Span::call_site())));
        seg.insert(0, path_seg);
        seg.insert(1, parse_str::<PathSegment>("select")?);
        let maybe_name = Ok(Path{
            leading_colon:None,
            segments:seg
        });//syn::Path::parse_mod_style(input);
        println!("maybe_name: {:?}", maybe_name);
        let name = name_or_fragment(maybe_name);
        */
        let is_custom_element = tag_name.is_custom_element_name();
        let attributes = ElementAttributes::parse(input, is_custom_element)?;
        let self_closing = input.parse::<syn::Token![/]>().is_ok();
        input.parse::<syn::Token![>]>()?;

        Ok(Self {
            name:tag_name,
            attributes,
            self_closing,
            is_custom_element,
        })
    }
}

pub struct ClosingTag {
    name: TagName,
}

impl ClosingTag {
    pub fn validate(&self, open_tag: &OpenTag) {
        let open_tag_path = &open_tag.name;
        let open_tag_path_str = quote!(#open_tag_path).to_string();
        let self_path = &self.name;
        let self_path_str = quote!(#self_path).to_string();
        if self_path_str != open_tag_path_str {
            abort!(
                "Expected closing tag for: <{}>",
                &open_tag_path_str
            );
        }
    }
}

impl Parse for ClosingTag {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<syn::Token![<]>()?;
        input.parse::<syn::Token![/]>()?;
        println!("sssss");
        let tag_name = input.parse::<TagName>()?;
        //let maybe_name = input.parse::<syn::Path>();
        /*
        let mut seg = Punctuated::<PathSegment, Colon2>::new();
        let path_seg = parse_str::<PathSegment>("flow")?;
        seg.insert(0, path_seg);
        seg.insert(1, parse_str::<PathSegment>("select")?);
        let maybe_name = Ok(Path{
            leading_colon:None,
            segments:seg
        });
        println!("sssss3333111");
        skip_tag_name(input)?;
        */
        input.parse::<syn::Token![>]>()?;
        println!("sssss3333");
        //let name = name_or_fragment(maybe_name);
        println!("sssss333344444: {:?}", tag_name);
        Ok(Self {
            name:tag_name,
        })
    }
}
