extern crate proc_macro;
use proc_macro::{proc_macro_derive, TokenStream};


// IntoStaticStr, EnumString, Default, EnumIter, Debug, Display, PartialEq
#[proc_macro_derive(SelectFormEnum)]
pub fn derive_select_form_enum(_item: TokenStream) -> TokenStream {
    
    "".parse().unwrap()
}