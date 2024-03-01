use super::FieldType;
use quote::{__private::TokenStream, quote};

/// The code for extracting a field from a [neo4rs::Row] or other BoltMap impl.
///
/// It uses `value.get()` where value is a BoltMap such as a Row, Node, Relation, or UnboundedRelation.
pub fn field_from_boltmap(name: &str, typ: &FieldType) -> TokenStream {
    match typ {
        FieldType::DateTimeUtc(_ty) => {
            quote!(
                value.get::<::chrono::DateTime<::chrono::FixedOffset>>(#name)
                    .map(|dt| dt.into())
                    .map_err(|e| ::cypher_dto::Error::MissingField(#name.to_owned()))?
            )
        }
        FieldType::OptionDateTimeUtc(_ty) => {
            quote!(
                value.get::<::chrono::DateTime<::chrono::FixedOffset>>(#name)
                    .map(|dt| dt.into()).ok()
            )
        }
        FieldType::Num(_ty, num) => {
            // Build from the inside out.
            // Example: cypher-dto/lib/src/entity.rs#L425
            let mut tokens = quote!(
                value.get(#name).map_err(|e| ::cypher_dto::Error::MissingField(#name.to_owned()))?
            );

            // Handle `value.get::<type>()`
            if let Some(type_arg) = num.map_getter_type_arg() {
                let cast_type = type_arg.to_type();
                tokens = quote!(
                    value.get::<#cast_type>(#name).map_err(|e| ::cypher_dto::Error::MissingField(#name.to_owned()))?
                );
            }

            // Handle `some_type::try_from(value.get())`
            if num.map_uses_try_from() {
                let try_from_type = num.to_type();
                tokens = quote!(
                    #try_from_type::try_from(#tokens)
                    .map_err(|_| ::cypher_dto::Error::TypeMismatch(#name.to_owned()))?
                );
            }

            // Handle `value.get() as type`
            if let Some(type_arg) = num.map_cast() {
                let cast_type = type_arg.to_type();
                tokens = quote!(#tokens as #cast_type);
            }

            tokens
        }
        FieldType::OptionNum(_ty, num) => {
            // Build from the inside out.
            // Example: cypher-dto/lib/src/entity.rs#L595
            //
            // i32_opt: match value.get::<i64>("i32_opt") {
            //     Some(v) => Some(
            //         v.try_into()
            //             .map_err(|_| Error::TypeMismatch("i32_opt".to_owned()))?,
            //     ),
            //     None => None,
            // },
            // i64_opt: match value.get("i64_opt") {
            //     Some(v) => Some(v),
            //     None => None,
            // },
            // i128_opt: match value.get::<i64>("i128_opt") {
            //     Some(v) => Some(
            //         v.try_into()
            //             .map_err(|_| Error::TypeMismatch("i128_opt".to_owned()))?,
            //     ),
            //     None => None,
            // },
            // f32_opt: match value.get::<f64>("f32_opt") {
            //     Some(v) => Some(v as f32),
            //     None => None,
            // },
            // f64_opt: match value.get("f64_opt") {
            //     Some(v) => Some(v),
            //     None => None,
            // },
            let mut get_call = quote!(
                value.get(#name)
            );
            let mut some_inner = quote!(v);

            // Handle `value.get::<type>()`
            if let Some(type_arg) = num.map_getter_type_arg() {
                let cast_type = type_arg.to_type();
                get_call = quote!(
                    value.get::<#cast_type>(#name)
                );
            }

            // Handle `some_type::try_from(value.get())`
            if num.map_uses_try_from() {
                // let try_from_type = num.to_type();
                some_inner = quote!(
                    v.try_into()
                    .map_err(|_| ::cypher_dto::Error::TypeMismatch(#name.to_owned()))?
                );
            }

            // Handle `value.get() as type`
            if let Some(type_arg) = num.map_cast() {
                let cast_type = type_arg.to_type();
                some_inner = quote!(#some_inner as #cast_type);
            }

            quote!(
              match #get_call {
                  Ok(v) => Some(#some_inner),
                  Err(_) => None,
              }
            )
        }
        FieldType::OptionOther(_ty) => {
            quote!(
                value.get(#name)
            )
        }
        FieldType::Other(_ty) => {
            quote!(
                value.get(#name).map_err(|e| ::cypher_dto::Error::MissingField(#name.to_owned()))?
            )
        }
    }
}
