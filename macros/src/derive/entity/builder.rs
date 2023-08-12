use super::{ArgHelper, Entity};
use quote::{__private::TokenStream, format_ident, quote};
use syn::Type;

pub fn impl_builder(entity: &Entity) -> TokenStream {
    let entity_ident = entity.ident();
    let entity_name = entity.name();
    let ident = format_ident!("{}Builder", entity_ident);
    let (idents, types, names, comments, _into_params, _from_boltmaps) = entity.fields.to_vectors();

    let mut opt_types = Vec::new();
    let (arg_types, arg_converts, _without_amps) =
        ArgHelper::unzip(types.iter().map(|t| ArgHelper::new(t.as_type())).collect());

    let mut assignments = Vec::new();
    let mut from_entity = Vec::new();
    let mut into_entity = Vec::new();

    for index in 0..idents.len() {
        let id = idents[index];
        let name = names[index];
        let ty = types[index];
        let arg_convert = &arg_converts[index];
        match ty.is_option() {
            true => {
                opt_types.push(ty.as_type().clone());
                assignments.push(quote!(#id));
                from_entity.push(quote!(value.#id));
                into_entity.push(quote!(value.#id));
            }
            false => {
                let t = ty.as_type();
                let s = format!("Option<{}>", quote!(#t));
                opt_types.push(match syn::parse_str::<Type>(&s) {
                    Ok(ty) => ty,
                    Err(e) => panic!(
                        "Failed to wrap type in Option for {}: {}. ({})",
                        ident, e, s
                    ),
                });
                assignments.push(quote!(Some(#id #arg_convert)));
                from_entity.push(quote!(Some(value.#id)));
                into_entity.push(quote!(value.#id.ok_or(::cypher_dto::Error::BuilderError(#entity_name.to_owned(), #name.to_owned()))?));
            }
        }
    }
    let vis = entity.vis();

    quote! {
        #vis struct #ident {
            #( #idents: #opt_types, )*
        }
        impl #ident {
            pub fn new() -> Self {
                Self {
                    #( #idents: None, )*
                }
            }
            #(
                #( #comments )*
                pub fn #idents(mut self, #idents: #arg_types) -> Self {
                    self.#idents = #assignments;
                    self
                }
            )*
            pub fn build(self) -> ::std::result::Result<#entity_ident, ::cypher_dto::Error> {
                self.try_into()
            }
        }
        impl Default for #ident {
            fn default() -> Self {
                Self::new()
            }
        }
        impl From<#entity_ident> for #ident {
            fn from(value: #entity_ident) -> Self {
                Self {
                    #( #idents: #from_entity, )*
                }
            }
        }
        impl TryFrom<#ident> for #entity_ident {
            type Error = ::cypher_dto::Error;
            fn try_from(value: #ident) -> ::std::result::Result<Self, Self::Error> {
                Ok(Self {
                    #( #idents: #into_entity, )*
                })
            }
        }
        impl #entity_ident {
            pub fn into_builder(self) -> #ident {
                self.into()
            }
        }
    }
}

// pub struct PersonBuilder {
//   id: Option<String>,
//   name: Option<String>,
//   age: Option<u8>,
//   created_at: Option<DateTime<Utc>>,
//   updated_at: Option<DateTime<Utc>>,
// }
// impl PersonBuilder {
//   pub fn new() -> Self {
//       Self {
//           id: None,
//           name: None,
//           age: None,
//           created_at: None,
//           updated_at: None,
//       }
//   }
//   pub fn id(mut self, id: String) -> Self {
//       self.id = Some(id);
//       self
//   }
//   pub fn name(mut self, name: String) -> Self {
//       self.name = Some(name);
//       self
//   }
//   pub fn age(mut self, age: u8) -> Self {
//       self.age = Some(age);
//       self
//   }
//   pub fn created_at(mut self, created_at: Option<DateTime<Utc>>) -> Self {
//       self.created_at = created_at;
//       self
//   }
//   pub fn updated_at(mut self, updated_at: Option<DateTime<Utc>>) -> Self {
//       self.updated_at = updated_at;
//       self
//   }
//   pub fn build(self) -> Result<Person, Error> {
//       self.try_into()
//   }
// }
// impl Default for PersonBuilder {
//   fn default() -> Self {
//       Self::new()
//   }
// }
// impl From<Person> for PersonBuilder {
//   fn from(value: Person) -> Self {
//       Self {
//           id: Some(value.id),
//           name: Some(value.name),
//           age: value.age,
//           created_at: value.created_at,
//           updated_at: value.updated_at,
//       }
//   }
// }
// impl TryFrom<PersonBuilder> for Person {
//   type Error = Error;
//   fn try_from(value: PersonBuilder) -> Result<Self, Self::Error> {
//       Ok(Person {
//           id: value
//               .id
//               .ok_or(Error::BuilderError("Person".to_owned(), "id".to_owned()))?,
//           name: value
//               .name
//               .ok_or(Error::BuilderError("Person".to_owned(), "id".to_owned()))?,
//           age: value.age,
//           created_at: value.created_at,
//           updated_at: value.updated_at,
//       })
//   }
// }
// impl Person {
//     pub fn into_builder(self) -> PersonBuilder {
//         self.into()
//     }
// }
