use super::{ArgHelper, Entity};
use quote::{__private::TokenStream, quote};

pub fn impl_new_and_getters(entity: &Entity) -> TokenStream {
    let entity_ident = entity.ident();
    let mut idents = Vec::new();
    let mut types = Vec::new();
    let mut comments = Vec::new();
    let mut stamp_idents = Vec::new();
    let mut stamp_types = Vec::new();
    let mut stamp_comments = Vec::new();
    // Skip timestamp fields if they are optional.
    for field in entity.fields().inner() {
        if field.typ().is_option() && field.stamp_type().is_some() {
            stamp_idents.push(field.ident());
            stamp_types.push(field.typ().as_type());
            stamp_comments.push(field.comments());
        } else {
            idents.push(field.ident());
            types.push(field.typ().as_type());
            comments.push(field.comments());
        }
    }
    let (
        arg_type,
        arg_into_field_suffix,
        getter_return,
        field_into_getter_prefix_amp,
        field_into_getter_suffix,
    ) = ArgHelper::unzip(types.iter().map(|t| ArgHelper::new(t)).collect());
    let (
        _stamp_arg_type,
        _stamp_arg_into_field_suffix,
        stamp_getter_return,
        stamp_field_into_getter_prefix_amp,
        stamp_field_into_getter_suffix,
    ) = ArgHelper::unzip(stamp_types.iter().map(|t| ArgHelper::new(t)).collect());
    quote! {
       impl #entity_ident {
            pub fn new(#( #idents: #arg_type, )*) -> Self {
                Self {
                    #( #idents: #idents #arg_into_field_suffix, )*
                    #( #stamp_idents: None, )*
                }
            }

            #(
                #( #comments )*
                pub fn #idents(&self) -> #getter_return {
                    #field_into_getter_prefix_amp self.#idents #field_into_getter_suffix
                }
            )*
            #(
                #( #stamp_comments )*
                pub fn #stamp_idents(&self) -> #stamp_getter_return  {
                    #stamp_field_into_getter_prefix_amp self.#stamp_idents #stamp_field_into_getter_suffix
                }
            )*
       }
    }
}

// #[derive(Clone, Debug, PartialEq)]
// pub struct Person {
//     id: String,
//     name: String,
//     age: Option<u8>,
//     created_at: Option<DateTime<Utc>>,
//     updated_at: Option<DateTime<Utc>>,
// }
// impl Person {
//     pub fn new(id: &str, name: &str, age: Option<u8>) -> Self {
//         Self {
//             id: id.to_owned(),
//             name: name.to_owned(),
//             age,
//             created_at: None,
//             updated_at: None,
//         }
//     }
//     pub fn id(&self) -> &str {
//         &self.id
//     }
//     pub fn name(&self) -> &str {
//         &self.name
//     }
//     pub fn age(&self) -> Option<u8> {
//         self.age
//     }
//     pub fn created_at(&self) -> Option<DateTime<Utc>> {
//         self.created_at
//     }
//     pub fn updated_at(&self) -> Option<DateTime<Utc>> {
//         self.updated_at
//     }
// }
