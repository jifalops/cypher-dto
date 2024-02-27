use super::entity::Entity;
use crate::derive::{self, EntityType};
use proc_macro::TokenStream;
use quote::{__private::TokenStream as TokenStream2, quote};
use syn::{DeriveInput, Ident};

pub struct Relation {
    inner: Entity,
    id: Entity,
}
impl Relation {
    pub fn new(input: DeriveInput) -> Self {
        let (inner, id) = Entity::new(input, EntityType::Relation);
        Self { inner, id }
    }

    pub fn to_token_stream(&self) -> TokenStream {
        let main_impl = self.main_entity_tokens();
        let id_impl = self.id_entity_tokens();
        quote! {
            #main_impl
            #id_impl
        }
        .into()
    }

    fn main_entity_tokens(&self) -> TokenStream2 {
        let main_ident = self.inner.ident();
        let idents: Vec<&Ident> = self
            .inner
            .fields()
            .inner()
            .iter()
            .map(|f| f.ident())
            .collect();
        let from_boltmaps: Vec<&TokenStream2> = self
            .inner
            .fields()
            .inner()
            .iter()
            .map(|f| f.from_boltmap())
            .collect();
        let id_ident = self.id.ident();
        let id_idents = self.id.fields().inner().iter().map(|f| f.ident());
        let entity_impl = self.inner.entity_impl();
        let builder_impl = self.inner.builder_impl();
        quote! {
            #entity_impl
            impl ::cypher_dto::RelationEntity for #main_ident {
                type Id = #id_ident;
                fn identifier(&self) -> Self::Id {
                    #id_ident {
                        #( #id_idents: self.#id_idents.clone(), )*
                    }
                }
            }
            impl TryFrom<::neo4rs::Relation> for #main_ident {
                type Error = ::cypher_dto::Error;
                fn try_from(value: ::neo4rs::Relation) -> ::std::result::Result<Self, Self::Error> {
                    Ok(Self {
                        #(#idents: #from_boltmaps),*
                    })
                }
            }
            impl TryFrom<::neo4rs::UnboundedRelation> for #main_ident {
                type Error = ::cypher_dto::Error;
                fn try_from(value: ::neo4rs::UnboundedRelation) -> ::std::result::Result<Self, Self::Error> {
                    Ok(Self {
                        #(#idents: #from_boltmaps),*
                    })
                }
            }
            #builder_impl
        }
    }

    fn id_entity_tokens(&self) -> TokenStream2 {
        let main_ident = self.inner.ident();
        let id_ident = self.id.ident();
        let comment = format!(
            "The unique identifier for a [`{}`] relationship.",
            main_ident
        );
        let idents: Vec<&Ident> = self.id.fields().inner().iter().map(|f| f.ident()).collect();
        let types = self.id.fields().inner().iter().map(|f| f.typ().as_type());
        let from_boltmaps: Vec<&TokenStream2> = self
            .id
            .fields()
            .inner()
            .iter()
            .map(|f| f.from_boltmap())
            .collect();
        let entity_impl = self.id.entity_impl();
        let serde = derive::derive_serde();
        let vis = self.id.vis();
        quote! {
            #[doc = #comment]
            #[derive(Clone, Debug, PartialEq, #serde)]
            #vis struct #id_ident {
                #( #idents: #types, )*
            }
            #entity_impl
            impl ::cypher_dto::RelationId for #id_ident {
                type T = #main_ident;
            }
            impl From<#main_ident> for #id_ident {
                fn from(value: #main_ident) -> Self {
                    Self {
                        #( #idents: value.#idents, )*
                    }
                }
            }
            impl TryFrom<::neo4rs::Relation> for #id_ident {
                type Error = ::cypher_dto::Error;
                fn try_from(value: ::neo4rs::Relation) -> ::std::result::Result<Self, Self::Error> {
                    Ok(Self {
                        #(#idents: #from_boltmaps),*
                    })
                }
            }
            impl TryFrom<::neo4rs::UnboundedRelation> for #id_ident {
                type Error = ::cypher_dto::Error;
                fn try_from(value: ::neo4rs::UnboundedRelation) -> ::std::result::Result<Self, Self::Error> {
                    Ok(Self {
                        #(#idents: #from_boltmaps),*
                    })
                }
            }
        }
    }
}

// impl RelationEntity for WorkedAt {
//     type Id = WorkedAtId;
//     fn identifier(&self) -> Self::Id {
//         WorkedAtId {
//             until: self.until.clone(),
//         }
//     }
// }
// impl TryFrom<Relation> for WorkedAt {
//     type Error = Error;
//     fn try_from(value: Relation) -> Result<Self, Self::Error> {
//         Ok(Self {
//             until: Neo4jMap::Relation(&value).get_timestamp("until")?,
//         })
//     }
// }
// impl TryFrom<UnboundedRelation> for WorkedAt {
//     type Error = Error;
//     fn try_from(value: UnboundedRelation) -> Result<Self, Self::Error> {
//         Ok(Self {
//             until: Neo4jMap::UnboundedRelation(&value).get_timestamp("until")?,
//         })
//     }
// }

// //
// // WorkedAtId
// //

// #[derive(Clone, Debug, PartialEq)]
// pub struct WorkedAtId {
//     until: DateTime<Utc>,
// }
// impl RelationId for WorkedAtId {
//     type T = WorkedAt;
// }
// impl From<WorkedAt> for WorkedAtId {
//     fn from(value: WorkedAt) -> Self {
//         WorkedAtId {
//             until: value.until.clone(),
//         }
//     }
// }
// impl TryFrom<Relation> for WorkedAtId {
//     type Error = Error;
//     fn try_from(value: Relation) -> Result<Self, Self::Error> {
//         Ok(Self {
//             until: Neo4jMap::Relation(&value).get_timestamp("until")?,
//         })
//     }
// }
// impl TryFrom<UnboundedRelation> for WorkedAtId {
//     type Error = Error;
//     fn try_from(value: UnboundedRelation) -> Result<Self, Self::Error> {
//         Ok(Self {
//             until: Neo4jMap::UnboundedRelation(&value).get_timestamp("until")?,
//         })
//     }
// }
