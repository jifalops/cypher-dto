use super::{ArgHelper, Entity};
use quote::{__private::TokenStream, format_ident, quote};


pub fn impl_builder(entity: &Entity) -> TokenStream {
    let entity_ident = entity.ident();
    let _entity_name = entity.name();
    let ident = format_ident!("{}Builder", entity_ident);
    let (idents, types, names, comments, _into_params, _from_boltmaps) = entity.fields.to_vectors();

    let mut all_types = Vec::new();
    let (
        arg_type,
        arg_into_field_suffix,
        _getter_return,
        _field_into_getter_prefix_amp,
        _field_into_getter_suffix,
    ) = ArgHelper::unzip(types.iter().map(|t| ArgHelper::new(t.as_type())).collect());

    let mut assignments = Vec::new();
    let mut from_entity = Vec::new();
    let mut into_entity = Vec::new();

    for index in 0..idents.len() {
        let id = idents[index];
        let _name = names[index];
        let ty = types[index];
        let arg_convert = &arg_into_field_suffix[index];
        match ty.is_option() {
            true => {
                all_types.push(ty.as_type().clone());
                assignments.push(quote!(#id  #arg_convert));
                from_entity.push(quote!(value.#id));
                into_entity.push(quote!(value.#id));
            }
            false => {
                all_types.push(ty.as_type().clone());
                assignments.push(quote!(#id #arg_convert));
                from_entity.push(quote!(value.#id));
                into_entity.push(quote!(value.#id));
            }
        }
    }
    let vis = entity.vis();

    quote! {
        #vis struct #ident {
            #( #idents: #all_types, )*
        }
        impl #ident {
            #(
                #( #comments )*
                pub fn #idents(mut self, #idents: #arg_type) -> Self {
                    self.#idents = #assignments;
                    self
                }
            )*
            pub fn build(self) -> #entity_ident {
                self.into()
            }
        }
        impl From<#entity_ident> for #ident {
            fn from(value: #entity_ident) -> Self {
                Self {
                    #( #idents: #from_entity, )*
                }
            }
        }
        impl From<#ident> for #entity_ident {
            fn from(value: #ident) -> Self {
                Self {
                    #( #idents: #into_entity, )*
                }
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
