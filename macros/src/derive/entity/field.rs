mod field_type;
mod map_helper;
mod query_helper;

pub use field_type::{ArgHelper, FieldType};

use quote::__private::TokenStream;
use syn::{Attribute, Field, Ident};

use crate::derive;

/// For fields that are one of the [cypher_dto::Stamps].
#[derive(Clone)]
pub enum StampType {
    Created,
    Updated,
}

#[derive(Clone)]
pub struct EntityField {
    ident: Ident,
    typ: FieldType,
    name: String,
    is_id: bool,
    _is_skip: bool,
    comments: Vec<Attribute>,
    into_param: TokenStream,
    from_boltmap: TokenStream,
    stamp_type: Option<StampType>,
}
impl EntityField {
    pub fn new(field: &Field, stamp: Option<StampType>) -> Self {
        let ident = field.ident.as_ref().unwrap().clone();
        let typ = FieldType::parse(field.ty.clone());
        if stamp.is_some() {
            match typ {
                FieldType::DateTimeUtc(_) => {},
                FieldType::OptionDateTimeUtc(_) => {},
                _ => panic!("Timestamp fields must be `Option<DateTime<Utc>>` or `DateTime<Utc>` (field: {}). Details: {}", ident, typ),
            }
        }
        let mut name = ident.to_string();
        let mut is_id = false;
        let mut is_skip = false;
        let mut comments = Vec::new();
        for attr in field.attrs.iter() {
            if attr.path().is_ident("name") {
                name = derive::parse_name(attr);
            } else if attr.path().is_ident("id") {
                is_id = true;
            } else if attr.path().is_ident("skip") {
                is_skip = true;
            } else if attr.path().is_ident("doc") {
                comments.push(attr.clone());
            }
        }
        if is_id && typ.is_option() {
            panic!("#[id] fields cannot be `Option`s (field: {})", ident);
        }
        let into_param = query_helper::add_value_to_params(&ident, &name, &typ, stamp.clone());
        let from_boltmap = map_helper::field_from_boltmap(&name, &typ);
        Self {
            ident,
            typ,
            name,
            is_id,
            _is_skip: is_skip,
            comments,
            into_param,
            from_boltmap,
            stamp_type: stamp,
        }
    }
    pub fn ident(&self) -> &Ident {
        &self.ident
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn typ(&self) -> &FieldType {
        &self.typ
    }
    pub fn is_id(&self) -> bool {
        self.is_id
    }
    pub fn _is_skip(&self) -> bool {
        self._is_skip
    }
    pub fn comments(&self) -> &Vec<Attribute> {
        &self.comments
    }
    pub fn into_param(&self) -> &TokenStream {
        &self.into_param
    }
    pub fn from_boltmap(&self) -> &TokenStream {
        &self.from_boltmap
    }
    pub fn stamp_type(&self) -> &Option<StampType> {
        &self.stamp_type
    }
}
