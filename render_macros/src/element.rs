use crate::children::Children;
use crate::element_attributes::ElementAttributes;
use crate::tags::{ClosingTag, OpenTag, TagName};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};

pub struct Element {
    name: TagName,
    attributes: ElementAttributes,
    children: Children,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let open_tag = input.parse::<OpenTag>()?;

        let children = if open_tag.self_closing {
            Children::default()
        } else {
            let children = input.parse::<Children>()?;
            let closing_tag = input.parse::<ClosingTag>()?;
            closing_tag.validate(&open_tag);
            children
        };
        println!("ssss4555 : input:{:?}", input);
        let r = Ok(Element {
            name: open_tag.name,
            attributes: open_tag.attributes,
            children,
        });

        println!("ssss6666666666: input:{:?}", input);
        r
    }
}

impl Element {
    pub fn is_custom_element(&self) -> bool {
        //match self.name.get_ident() {
        //    None => true,
        //    Some(ident) => {
                let name = self.name.to_string();
                let first_letter = name.get(0..1).unwrap();
                first_letter.to_uppercase() == first_letter
        //    }
        //}
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        println!("s######### 111111");
        let declaration = if self.is_custom_element() {
            println!("s######### 222222");
            let attrs = self.attributes.for_custom_element(&self.children);
            quote! { #name #attrs }
        } else {
            println!("s######### 333333");
            let attrs = self.attributes.for_simple_element();
            let children_tuple = self.children.as_option_of_tuples_tokens();
            println!("s######### 444444");
            quote! {
                ::render::SimpleElement {
                    tag_name: String::from(#name),
                    attributes: #attrs,
                    contents: #children_tuple,
                }
            }
        };
        println!("s######### 55555555");
        declaration.to_tokens(tokens);
    }
}
