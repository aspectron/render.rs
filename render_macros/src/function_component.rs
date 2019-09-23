use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn to_component(f: syn::ItemFn) -> TokenStream {
    let struct_name = f.sig.ident;
    let (impl_generics, ty_generics, where_clause) = f.sig.generics.split_for_impl();
    let inputs = f.sig.inputs;
    let block = f.block;
    let vis = f.vis;

    let inputs_block = if inputs.len() > 0 {
        quote!({ #inputs })
    } else {
        quote!(;)
    };

    let inputs_reading = if inputs.len() == 0 {
        quote!()
    } else {
        let input_names: Vec<_> = inputs
            .iter()
            .filter_map(|argument| match argument {
                syn::FnArg::Typed(typed) => Some(typed),
                syn::FnArg::Receiver(rec) => {
                    rec.span().unwrap().error("Don't use `self` on components");
                    None
                }
            })
            .map(|value| {
                let pat = &value.pat;
                quote!(#pat)
            })
            .collect();
        quote!(
            let #struct_name { #(#input_names),* } = self;
        )
    };

    TokenStream::from(quote! {
        #[derive(Debug)]
        #vis struct #struct_name#impl_generics #inputs_block

        impl#impl_generics ::render::Renderable for #struct_name #ty_generics #where_clause {
            fn render(self) -> String {
                #inputs_reading
                #block
            }
        }
    })
}