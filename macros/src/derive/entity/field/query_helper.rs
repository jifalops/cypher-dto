use super::{FieldType, StampType};
use quote::__private::TokenStream;
use quote::quote;
use syn::Ident;

/// The code for adding a field to a [neo4rs::Query::param].
///
/// Uses a [neo4rs::Query] named `query`, and may update its params with this field.
pub fn add_value_to_params(
    ident: &Ident,
    name: &str,
    typ: &FieldType,
    stamp: Option<StampType>,
) -> TokenStream {
    match typ {
        FieldType::DateTimeUtc(_ty) => {
            // Similar example: cypher-dto/lib/tests/common/entities/person.rs#L57
            if stamp.is_some() {
                match stamp.unwrap() {
                    StampType::Created => {
                        quote!(
                            match mode {
                                ::cypher_dto::StampMode::Create => query,
                                _ => query.param(
                                    &::cypher_dto::format_param(#name, prefix),
                                    self.#ident.fixed_offset()
                                )
                            }
                        )
                    }
                    StampType::Updated => {
                        quote!(
                            match mode {
                                ::cypher_dto::StampMode::Read => query.param(
                                    &::cypher_dto::format_param(#name, prefix),
                                    self.#ident.fixed_offset()
                                ),
                                _ => query,
                            }
                        )
                    }
                }
            } else {
                quote!(
                    query.param(
                        &::cypher_dto::format_param(#name, prefix),
                        self.#ident.fixed_offset()
                    )
                )
            }
        }
        FieldType::OptionDateTimeUtc(_ty) => {
            // Example: cypher-dto/lib/tests/common/entities/person.rs#L57
            if stamp.is_some() {
                match stamp.unwrap() {
                    StampType::Created => {
                        quote!(
                            match mode {
                                ::cypher_dto::StampMode::Create => query,
                                _ => query.param(
                                    &::cypher_dto::format_param(#name, prefix),
                                    self.#ident.map(|v| v.fixed_offset())
                                )
                            }
                        )
                    }
                    StampType::Updated => {
                        quote!(
                            match mode {
                                ::cypher_dto::StampMode::Read => query.param(
                                    &::cypher_dto::format_param(#name, prefix),
                                    self.#ident.map(|v| v.fixed_offset())
                                ),
                                _ => query,
                            }
                        )
                    }
                }
            } else {
                quote!(
                    query.param(
                        &::cypher_dto::format_param(#name, prefix),
                        self.#ident.map(|v| v.fixed_offset())
                    )
                )
            }
        }
        FieldType::Num(_ty, num) => {
            // Non-optional numbers that need a cast use `as X`.
            if let Some(cast) = num.param_cast() {
                // Example: cypher-dto/lib/src/entity.rs#L405
                let ty = cast.to_type();
                quote!(
                    query.param(
                        &::cypher_dto::format_param(#name, prefix),
                        self.#ident as #ty
                    )
                )
            } else {
                quote!(
                    query.param(
                        &::cypher_dto::format_param(#name, prefix),
                        self.#ident
                    )
                )
            }
        }
        FieldType::OptionNum(_ty, num) => {
            // Optional numbers that need a cast use `.map(T::from)`.
            if let Some(cast) = num.param_cast() {
                // Example: cypher-dto/lib/tests/common/entities/person.rs#L57
                let ty = cast.to_type();
                quote!(
                    query.param(
                        &::cypher_dto::format_param(#name, prefix),
                        self.#ident.map(#ty::from)
                    )
                )
            } else {
                quote!(
                    query.param(
                        &::cypher_dto::format_param(#name, prefix),
                        self.#ident
                    )
                )
            }
        }
        FieldType::OptionOther(_ty) => {
            // Options are `Copy`
            quote!(
                query.param(
                    &::cypher_dto::format_param(#name, prefix),
                    self.#ident
                )
            )
        }
        FieldType::Other(_ty) => {
            // Most fields are cloned.
            quote!(
                query.param(
                    &::cypher_dto::format_param(#name, prefix),
                    self.#ident.clone()
                )
            )
        }
    }
}
