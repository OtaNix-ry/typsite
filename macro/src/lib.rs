use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{LitBool, LitStr, parse_macro_input};

struct RewritePassArgs {
    name: Ident,
    id: String,
    atom: bool,
    pure: bool,
}

impl syn::parse::Parse for RewritePassArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<syn::Token![,]>()?;
        let tag_key = input.parse::<Ident>()?;
        if tag_key != "id" {
            return Err(syn::Error::new(tag_key.span(), "expected 'id'"));
        }
        input.parse::<syn::Token![=]>()?;
        let id = input.parse::<LitStr>()?.value();

        input.parse::<syn::Token![,]>()?;
        let atom_key = input.parse::<Ident>()?;
        if atom_key != "atom" {
            return Err(syn::Error::new(atom_key.span(), "expected 'atom'"));
        }
        input.parse::<syn::Token![=]>()?;
        let atom = input.parse::<LitBool>()?.value();

        let pure = input
            .parse::<syn::Token![,]>()
            .and_then(|_| input.parse::<Ident>())
            .and_then(|pure_key| {
                if pure_key != "pure" {
                    Err(syn::Error::new(pure_key.span(), "expected 'pure'"))
                } else {
                    input.parse::<syn::Token![=]>()
                }
            })
            .and_then(|_| input.parse::<LitBool>())
            .unwrap_or(LitBool::new(true, name.span()))
            .value();

        Ok(Self {
            name,
            id,
            atom,
            pure,
        })
    }
}

#[proc_macro]
pub fn rewrite_pass(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RewritePassArgs);

    let name = input.name;
    let reg_func = format_ident!("{}_pass_reg", name.to_string().to_lowercase());

    let id = LitStr::new(&input.id, name.span());
    let atom = LitBool::new(input.atom, name.span());
    let pure = LitBool::new(input.pure, name.span());

    let expanded = quote! {
        struct #name {}
        impl #name {
            fn default() -> Self {
                Self {}
            }
        }
        impl Id for #name {
            fn id(&self) -> &str {
                #id
            }
        }
        impl Atom for #name {
            fn atom(&self) -> bool {
                #atom
            }
        }
        impl Purity for #name {
            fn pure(&self) -> bool {
                #pure
            }
        }

        #[::ctor::ctor]
        fn #reg_func() {
            let instance = #name::default();
            register_rewrite_pass(instance);
        }
    };
    TokenStream::from(expanded)
}
