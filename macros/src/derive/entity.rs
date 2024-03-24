mod builder;
mod field;
mod fields;
mod new_and_getters;

pub use field::{ArgHelper, EntityField, FieldType, StampType};
pub use fields::EntityFields;

use crate::derive::{self, EntityType};
use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};
use syn::{DeriveInput, Ident, Type};

pub struct Entity {
    vis: syn::Visibility,
    ident: Ident,
    name: String,
    labels: Vec<String>,
    fields: EntityFields,
}
impl Entity {
    pub fn new(input: DeriveInput, typ: EntityType) -> (Self, Self) {
        let vis = input.vis.clone();
        let ident = input.ident.clone();
        let mut labels = parse_entity_labels(&input);
        let name: String;
        if labels.is_empty() {
            name = parse_entity_name(&input, typ);
            labels.push(name.clone());
        } else {
            name = labels[0].clone();
        }
        let fields = EntityFields::new(input.data, typ);
        let id_ident = format_ident!("{}Id", &ident);
        (
            Self {
                vis: vis.clone(),
                ident,
                name: name.clone(),
                labels: labels.clone(),
                fields: fields.0,
            },
            Self {
                vis,
                ident: id_ident,
                name,
                labels,
                fields: fields.1,
            },
        )
    }
    pub fn vis(&self) -> &syn::Visibility {
        &self.vis
    }
    pub fn ident(&self) -> &Ident {
        &self.ident
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn _labels(&self) -> &[String] {
        &self.labels
    }
    pub fn fields(&self) -> &EntityFields {
        &self.fields
    }

    pub fn builder_impl(&self) -> TokenStream {
        builder::impl_builder(self)
    }

    pub fn entity_impl(&self) -> TokenStream {
        let struct_ident = &self.ident;
        let struct_name = &self.name;
        let struct_labels = &self.labels;
        let (idents, types, names, _comments, into_params, from_boltmaps) =
            self.fields.to_vectors();
        let types: Vec<&Type> = types.iter().map(|t| t.as_type()).collect();

        let new_and_getters = new_and_getters::impl_new_and_getters(self);

        let as_fields = names
            .iter()
            .map(|n| format!("{n}: ${n}"))
            .collect::<Vec<_>>()
            .join(", ");
        let as_obj = format!("{} {{ {} }}", struct_labels.join(":"), as_fields);

        quote! {
            impl ::cypher_dto::FieldSet for #struct_ident {
                fn typename() -> &'static str {
                    #struct_name
                }

                fn labels() -> &'static [&'static str] {
                    &[#(#struct_labels),*]
                }

                fn field_names() -> &'static [&'static str] {
                    &[#(#names),*]
                }

                fn as_query_fields() -> &'static str {
                    #as_fields
                }

                fn as_query_obj() -> &'static str {
                    #as_obj
                }

                fn add_values_to_params(&self, mut query: ::neo4rs::Query, prefix: Option<&str>, mode: ::cypher_dto::StampMode) -> ::neo4rs::Query {
                    #(query = #into_params;)*
                    query
                }
            }
            impl TryFrom<::neo4rs::Row> for #struct_ident {
                type Error = ::cypher_dto::Error;
                fn try_from(value: ::neo4rs::Row) -> ::std::result::Result<Self, Self::Error> {
                    Ok(Self {
                        #(#idents: #from_boltmaps),*
                    })
                }
            }
            #new_and_getters
            impl #struct_ident {
                fn into_values(self) -> (#(#types),*) {
                    (#(self.#idents),*)
                }
            }
        }
    }
}

fn parse_entity_name(input: &DeriveInput, typ: EntityType) -> String {
    // Determine the name from an attribute or the struct name.
    let mut name = String::new();
    for attr in input.attrs.iter() {
        if attr.path().is_ident("name") {
            name = derive::parse_name(attr);
        }
    }
    if name.is_empty() {
        name = match typ {
            EntityType::Node => input.ident.to_string(),
            EntityType::Relation => input.ident.to_string().to_case(Case::ScreamingSnake),
        };
    }
    name
}

fn parse_entity_labels(input: &DeriveInput) -> Vec<String> {
    // Determine the labels from an attribute or the struct name.
    let mut labels = Vec::new();
    let mut has_labels = false;
    let mut has_name = false;
    for attr in input.attrs.iter() {
        if attr.path().is_ident("labels") {
            has_labels = true;
            labels = derive::parse_labels(attr);
        }
        if attr.path().is_ident("name") {
            has_name = true;
        }
    }
    if has_labels && has_name {
        panic!(
            "Cannot specify both 'name' and 'labels' attributes on struct '{}'",
            input.ident
        )
    }
    labels
}
