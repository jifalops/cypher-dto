use quote::__private::TokenStream;
use syn::{punctuated::Punctuated, token::Comma, Attribute, Data, Field, Fields, Ident};

use super::{EntityField, EntityType, FieldType, StampType};

pub struct EntityFields {
    inner: Vec<EntityField>,
}
impl EntityFields {
    pub fn new(data: Data, typ: EntityType) -> (Self, Self) {
        // Assert this is a struct with zero or more named fields. Tuple structs are not supported.
        let fields = match data {
            Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields_named) => Some(fields_named.named),
                Fields::Unit => match typ {
                    EntityType::Node => None, //panic!("Nodes must have fields"),
                    EntityType::Relation => None,
                },
                _ => panic!("Tuple structs are not supported"),
            },
            _ => panic!("Only structs are supported"),
        };
        let fields = match fields {
            Some(punc) => parse_entity_fields(punc),
            None => Vec::new(),
        };
        let ids = find_id_fields(&fields, typ);
        (Self { inner: fields }, Self { inner: ids })
    }

    pub fn inner(&self) -> &Vec<EntityField> {
        &self.inner
    }

    /// Helpful for using [quote::quote!].
    pub fn to_vectors(
        &self,
    ) -> (
        Vec<&Ident>,
        Vec<&FieldType>,
        Vec<&str>,
        Vec<&Vec<Attribute>>,
        Vec<&TokenStream>,
        Vec<&TokenStream>,
    ) {
        let len = self.inner.len();
        let mut idents = Vec::with_capacity(len);
        let mut types = Vec::with_capacity(len);
        let mut names = Vec::with_capacity(len);
        let mut comments = Vec::with_capacity(len);
        let mut into_params = Vec::with_capacity(len);
        let mut from_boltmaps = Vec::with_capacity(len);

        for field in self.inner.iter() {
            idents.push(field.ident());
            types.push(field.typ());
            names.push(field.name());
            comments.push(field.comments());
            into_params.push(field.into_param());
            from_boltmaps.push(field.from_boltmap());
        }

        (idents, types, names, comments, into_params, from_boltmaps)
    }
}

fn parse_entity_fields(fields: Punctuated<Field, Comma>) -> Vec<EntityField> {
    let mut entity_fields: Vec<EntityField> = Vec::new();
    let mut created_at: Option<usize> = None;
    let mut created: Option<usize> = None;
    let mut updated_at: Option<usize> = None;
    let mut updated: Option<usize> = None;
    for (index, field) in fields.iter().enumerate() {
        match field.ident.as_ref().unwrap().to_string().as_ref() {
            "created_at" => created_at = Some(index),
            "created" => created = Some(index),
            "updated_at" => updated_at = Some(index),
            "updated" => updated = Some(index),
            _ => (),
        }
    }
    let created_timestamp_field = created_at.or(created);
    let updated_timestamp_field = updated_at.or(updated);
    for (index, field) in fields.iter().enumerate() {
        let stamp_type = if Some(index) == created_timestamp_field {
            Some(StampType::Created)
        } else if Some(index) == updated_timestamp_field {
            Some(StampType::Updated)
        } else {
            None
        };
        let field = EntityField::new(field, stamp_type);
        entity_fields.push(field);
    }
    entity_fields
}

fn find_id_fields(fields: &Vec<EntityField>, typ: EntityType) -> Vec<EntityField> {
    let mut ids: Vec<EntityField> = Vec::new();
    let mut id_field: Option<&EntityField> = None;

    for field in fields.iter() {
        if field.is_id() {
            ids.push(field.clone());
        }
        if field.name() == "id" {
            id_field = Some(field);
        }
    }

    if ids.is_empty() {
        if let Some(field) = id_field {
            ids.push(field.clone());
        } else {
            ids = match typ {
                EntityType::Node => fields.clone(),
                EntityType::Relation => Vec::new(),
            };
        }
    }
    ids
}
