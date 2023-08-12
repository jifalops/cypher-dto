use quote::{__private::TokenStream, quote};
use std::fmt::Display;
use syn::{PathArguments, Type};

/// Keeping this flat because it's temporary. Can be refactored after `neo4rs` uses `serde`.

#[derive(Clone)]
pub enum FieldType {
    DateTimeUtc(Type),
    OptionDateTimeUtc(Type),
    Num(Type, Num),
    OptionNum(Type, Num),
    OptionOther(Type),
    Other(Type),
}
impl Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FieldType::DateTimeUtc(_) => "FieldType::DateTimeUtc".to_owned(),
            FieldType::OptionDateTimeUtc(_) => "FieldType::OptionDateTimeUtc".to_owned(),
            FieldType::Num(_, num) => format!("FieldType::Num ({:?})", num),
            FieldType::OptionNum(_, num) => format!("FieldType::OptionNum ({:?})", num),
            FieldType::OptionOther(_) => "FieldType::OptionOther".to_owned(),
            FieldType::Other(_) => "FieldType::Other".to_owned(),
        };
        write!(f, "{}", s)
    }
}

impl FieldType {
    pub fn parse(ty: Type) -> Self {
        if is_option(&ty) {
            let inner = inner_type(&ty).unwrap();
            if is_datetime_utc(inner) {
                FieldType::OptionDateTimeUtc(ty)
            } else if let Some(num) = Num::from_type(inner) {
                FieldType::OptionNum(ty, num)
            } else {
                FieldType::OptionOther(ty)
            }
        } else if is_datetime_utc(&ty) {
            FieldType::DateTimeUtc(ty)
        } else if let Some(num) = Num::from_type(&ty) {
            FieldType::Num(ty, num)
        } else {
            FieldType::Other(ty)
        }
    }

    pub fn as_type(&self) -> &Type {
        match self {
            FieldType::DateTimeUtc(ty) => ty,
            FieldType::OptionDateTimeUtc(ty) => ty,
            FieldType::Num(ty, _) => ty,
            FieldType::OptionNum(ty, _) => ty,
            FieldType::OptionOther(ty) => ty,
            FieldType::Other(ty) => ty,
        }
    }

    pub fn is_option(&self) -> bool {
        match self {
            FieldType::OptionDateTimeUtc(_) => true,
            FieldType::OptionNum(_, _) => true,
            FieldType::OptionOther(_) => true,
            _ => false,
        }
    }
}

/// Helps with using &str and &[T] instead of String and Vec in arg types and getter returns.
pub struct ArgHelper {
    /// The type to use for the argument.
    pub arg_type: Type,
    /// Convert from [arg_type] to the true type of the field, by adding this suffix.
    pub arg_into_field_suffix: TokenStream,
    /// The return type of the getter.
    pub getter_return: Type,
    /// Prefix `#ident` to `& #ident`.
    pub field_into_getter_prefix_amp: TokenStream,
    /// Convert the field into [getter_return], after `#ident`.
    pub field_into_getter_suffix: TokenStream,
}
impl ArgHelper {
    pub fn new(ty: &Type) -> Self {
        if is_outer_type(ty, "String") {
            let subs: Type = syn::parse_str("&str").unwrap();
            Self {
                arg_type: subs.clone(),
                arg_into_field_suffix: quote!(.to_owned()),
                getter_return: subs,
                field_into_getter_prefix_amp: quote!(&),
                field_into_getter_suffix: quote!(),
            }
        } else if is_outer_type(ty, "Vec") {
            let inner = inner_type(ty).unwrap();
            let subs: Type = syn::parse_str(&format!("&[{}]", quote!(#inner))).unwrap();
            Self {
                arg_type: subs.clone(),
                arg_into_field_suffix: quote!(.to_vec()),
                getter_return: subs,
                field_into_getter_prefix_amp: quote!(&),
                field_into_getter_suffix: quote!(),
            }
        } else if is_outer_type(ty, "Option") {
            let inner = inner_type(ty).unwrap();
            let mut getter_return =
                syn::parse_str(&format!("Option<&{}>", quote!(#inner))).unwrap();
            let mut field_into_getter_suffix = quote!(.as_ref());
            if Num::from_type(inner).is_some() {
                getter_return = ty.clone();
                field_into_getter_suffix = quote!();
            }
            Self {
                arg_type: ty.clone(),
                arg_into_field_suffix: quote!(),
                getter_return,
                field_into_getter_prefix_amp: quote!(),
                field_into_getter_suffix,
            }
        } else {
            Self {
                arg_type: ty.clone(),
                arg_into_field_suffix: quote!(),
                getter_return: syn::parse_str(&format!("&{}", quote!(#ty))).unwrap(),
                field_into_getter_prefix_amp: quote!(&),
                field_into_getter_suffix: quote!(),
            }
        }
    }
    pub fn unzip(
        vec: Vec<ArgHelper>,
    ) -> (
        Vec<Type>,
        Vec<TokenStream>,
        Vec<Type>,
        Vec<TokenStream>,
        Vec<TokenStream>,
    ) {
        let mut arg_type = Vec::new();
        let mut arg_into_field_suffix = Vec::new();
        let mut getter_return = Vec::new();
        let mut field_into_getter_prefix_amp = Vec::new();
        let mut field_into_getter_suffix = Vec::new();
        for helper in vec {
            arg_type.push(helper.arg_type);
            arg_into_field_suffix.push(helper.arg_into_field_suffix);
            getter_return.push(helper.getter_return);
            field_into_getter_prefix_amp.push(helper.field_into_getter_prefix_amp);
            field_into_getter_suffix.push(helper.field_into_getter_suffix);
        }
        (
            arg_type,
            arg_into_field_suffix,
            getter_return,
            field_into_getter_prefix_amp,
            field_into_getter_suffix,
        )
    }
}

/// Helper for [is_option] and [is_datetime_utc].
fn is_outer_type(ty: &Type, value: &str) -> bool {
    if let Type::Path(path) = ty {
        if let Some(segment) = path.path.segments.last() {
            if segment.ident == value {
                return true;
            }
        }
    }
    false
}

/// Whether the type is an Option<_>.
fn is_option(ty: &Type) -> bool {
    is_outer_type(ty, "Option")
}

fn is_datetime_utc(ty: &Type) -> bool {
    match inner_type(ty) {
        Some(inner) => is_outer_type(ty, "DateTime") && is_outer_type(inner, "Utc"),
        None => false,
    }
}

/// Can get the `T` from `Option<T>`.
fn inner_type(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Path(path) if path.qself.is_none() => {
            let last_segment = path.path.segments.last().unwrap();
            match &last_segment.arguments {
                PathArguments::AngleBracketed(args) => args.args.first().and_then(|arg| {
                    if let syn::GenericArgument::Type(ty) = arg {
                        Some(ty)
                    } else {
                        None
                    }
                }),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Number info for adding to a [neo4rs::Query::param] or extracting from a BoltMap (e.g. [neo4rs::Row]).
#[derive(Clone, Debug, PartialEq)]
pub enum Num {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Isize,
    Usize,
}
impl Num {
    fn from_type(ty: &Type) -> Option<Num> {
        let path = match ty {
            Type::Path(path) => path,
            _ => return None,
        };
        let last_segment = path.path.segments.last().unwrap();
        let name = last_segment.ident.to_string();
        match name.as_str() {
            "i8" => Some(Num::I8),
            "i16" => Some(Num::I16),
            "i32" => Some(Num::I32),
            "i64" => Some(Num::I64),
            "i128" => Some(Num::I128),
            "u8" => Some(Num::U8),
            "u16" => Some(Num::U16),
            "u32" => Some(Num::U32),
            "u64" => Some(Num::U64),
            "u128" => Some(Num::U128),
            "f32" => Some(Num::F32),
            "f64" => Some(Num::F64),
            "isize" => Some(Num::Isize),
            "usize" => Some(Num::Usize),
            _ => None,
        }
    }
    pub fn to_type(&self) -> Type {
        let s = match self {
            Num::I8 => "i8",
            Num::I16 => "i16",
            Num::I32 => "i32",
            Num::I64 => "i64",
            Num::I128 => "i128",
            Num::U8 => "u8",
            Num::U16 => "u16",
            Num::U32 => "u32",
            Num::U64 => "u64",
            Num::U128 => "u128",
            Num::F32 => "f32",
            Num::F64 => "f64",
            Num::Isize => "isize",
            Num::Usize => "usize",
        };
        syn::parse_str(s).unwrap()
    }

    /// Whether it needs to us "as X" when adding to a [neo4rs::Query::param].
    ///
    /// Based on cypher-dto/lib/src/entity.rs#L405
    pub fn param_cast(&self) -> Option<Num> {
        match self {
            Num::Usize => Some(Num::I64),
            Num::Isize => Some(Num::I64),
            Num::U8 => Some(Num::U16),
            Num::U64 => Some(Num::I64),
            Num::U128 => Some(Num::I64),
            Num::I128 => Some(Num::I64),
            _ => None,
        }
    }

    /// Whether it needs to use `try_from` when converting from a [neo4rs::Row] or other BoltMap impl.
    ///
    /// Based on cypher-dto/lib/src/entity.rs#L425
    pub fn map_uses_try_from(&self) -> bool {
        match self {
            Num::F32 => false,
            Num::F64 => false,
            Num::I64 => false,
            _ => true,
        }
    }

    /// Whether it uses `get::<X>` when converting from a [neo4rs::Row] or other BoltMap impl.
    ///
    /// Based on cypher-dto/lib/src/entity.rs#L425
    pub fn map_getter_type_arg(&self) -> Option<Num> {
        match self {
            Num::F32 => Some(Num::F64),
            Num::F64 => None,
            Num::I64 => None,
            _ => Some(Num::I64),
        }
    }

    /// Whether it uses `as X` when converting from a [neo4rs::Row] or other BoltMap impl.
    ///
    /// Based on cypher-dto/lib/src/entity.rs#L425
    pub fn map_cast(&self) -> Option<Num> {
        match self {
            Num::F32 => Some(Num::F32),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_string(ty: &Type) -> String {
        quote::quote!(#ty).to_string()
    }

    #[test]
    fn to_string_format() {
        assert_eq!(to_string(&syn::parse_str("String").unwrap()), "String");
        assert_eq!(
            to_string(&syn::parse_str("Option<String>").unwrap()),
            "Option < String >"
        );
    }

    #[test]
    fn type_parsing() {
        let ty = syn::parse_str("String").unwrap();
        assert!(!is_option(&ty));
        assert!(inner_type(&ty).is_none());
        assert!(!is_datetime_utc(&ty));

        let ty = syn::parse_str("Option<String>").unwrap();
        assert!(is_option(&ty));
        assert_eq!(to_string(inner_type(&ty).unwrap()), "String");
        assert!(!is_datetime_utc(&ty));

        let ty = syn::parse_str("Option<DateTime<Utc>>").unwrap();
        assert!(is_option(&ty));
        let inner = inner_type(&ty).unwrap();
        assert!(is_datetime_utc(inner));

        let ty = syn::parse_str("Option<Option<DateTime<Utc>>>").unwrap();
        assert!(is_option(&ty));
        let inner = inner_type(&ty).unwrap();
        assert!(is_option(inner));
        let inner = inner_type(inner).unwrap();
        assert!(is_datetime_utc(inner));
    }

    #[test]
    fn arg_helper() {
        let ty = syn::parse_str("String").unwrap();
        let helper = ArgHelper::new(&ty);
        assert_eq!(to_string(&helper.arg_type), "& str");
        assert_eq!(to_string(&helper.getter_return), "str");
        assert_eq!(helper.arg_into_field_suffix.to_string(), ". to_owned ()");

        let ty = syn::parse_str("Vec<String>").unwrap();
        let helper = ArgHelper::new(&ty);
        assert_eq!(to_string(&helper.arg_type), "& [String]");
        assert_eq!(to_string(&helper.getter_return), "[String]");
        assert_eq!(helper.arg_into_field_suffix.to_string(), ". to_vec ()");

        let ty = syn::parse_str("Option<String>").unwrap();
        let helper = ArgHelper::new(&ty);
        assert_eq!(to_string(&helper.arg_type), "Option < String >");
        assert_eq!(to_string(&helper.getter_return), "Option < String >");
        assert_eq!(helper.arg_into_field_suffix.to_string(), "");
    }
}
