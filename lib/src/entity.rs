use crate::{format_query_fields, Stamps};
use neo4rs::{Query, Row};

/// The full or partial fields on a node or relationship that may have timestamps.
///
/// This is the basic unit of query building used by [NodeEntity], [NodeId], [RelationEntity], and [RelationId].
pub trait FieldSet: TryFrom<Row> {
    /// The primary label for a node, or the type of a relationship.
    fn typename() -> &'static str;

    /// The fields in this set.
    fn field_names() -> &'static [&'static str];

    /// Determines which [field_names] are for created/updated timestamps, if any.
    fn timestamps() -> (Stamps, Vec<&'static str>) {
        Stamps::from_fields(Self::field_names())
    }

    /// Formats the field names as a query string.
    ///
    /// `struct Foo { bar: u8 }` would be `bar: $bar`.
    ///
    /// This is a special case of [to_query_fields], where the prefix is `None` and the mode is [StampMode::Read], and is known at compile time.
    fn as_query_fields() -> &'static str;

    /// Wraps the fields formatted by [as_query_fields] with [typename] and a pair of curly braces.
    ///
    /// `Foo { bar: $bar }`
    fn as_query_obj() -> &'static str;

    /// Formats the field names as a query string.
    ///
    /// `struct Foo { bar: u8 }` would be `bar: $bar`.
    ///
    /// Prefixes apply to the placeholders only (e.g. bar: $prefix_bar).
    fn to_query_fields(prefix: Option<&str>, mode: StampMode) -> String {
        let (stamps, other_fields) = Self::timestamps();
        let stamps = stamps.as_query_fields(prefix, mode);
        let other_fields = format_query_fields(other_fields, prefix);
        if stamps.is_empty() {
            return other_fields;
        }
        if other_fields.is_empty() {
            return stamps;
        }
        [other_fields, stamps].join(", ")
    }

    /// Adds all field values to the query parameters, matching placeholders in [as_query_fields()].
    fn add_values_to_params(&self, query: Query, prefix: Option<&str>, mode: StampMode) -> Query;

    /// Formatted like `typename() { as_query_fields() }`, or for a fieldless relationship, just `typename()`.
    fn to_query_obj(prefix: Option<&str>, mode: StampMode) -> String {
        let fields = Self::to_query_fields(prefix, mode);
        if fields.is_empty() {
            return Self::typename().to_owned();
        }
        format!("{} {{ {} }}", Self::typename(), fields)
    }
}

/// Controls which timestamps are hardcoded in a query (e.g. `datetime()`),
/// and which use placeholders (e.g. `$created_at`).
pub enum StampMode {
    /// Any timestamp fields ([Stamps]) are treated as normal values.
    ///
    /// Corresponding placeholders are added to the query fields (e.g. $created, $updated).
    Read,
    /// Stamp fields are added to the query fields with a hardcoded value (e.g. `datetime()`).
    ///
    /// Applies to both [Stamps::Created] and [Stamps::Both].
    Create,
    /// Stamp fields are added to the query fields with a hardcoded value of `datetime()`.
    ///
    /// Applies to both [Stamps::Updated] and [Stamps::Both].
    Update,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::{format_param, Error, Neo4jMap};
    use chrono::{DateTime, Utc};

    pub struct Foo {
        pub name: String,
        pub age: u8,
    }

    pub struct Bar {
        pub created: DateTime<Utc>,
        pub updated: DateTime<Utc>,
    }

    /// A fieldless relationship.
    pub struct Baz;

    //
    // Foo impl
    //
    impl FieldSet for Foo {
        fn typename() -> &'static str {
            "Foo"
        }

        fn field_names() -> &'static [&'static str] {
            &["name", "age"]
        }

        fn as_query_fields() -> &'static str {
            "name: $name, age: $age"
        }

        fn as_query_obj() -> &'static str {
            "Foo { name: $name, age: $age }"
        }

        fn add_values_to_params(&self, query: Query, prefix: Option<&str>, _: StampMode) -> Query {
            query
                .param(&format_param("name", prefix), self.name.clone())
                .param(&format_param("age", prefix), self.age as i64)
        }
    }
    impl TryFrom<Row> for Foo {
        type Error = Error;
        fn try_from(value: Row) -> Result<Self, Self::Error> {
            Ok(Self {
                name: value
                    .get("name")
                    .map_err(|e| Error::MissingField("name".to_owned()))?,
                age: u8::try_from(
                    value
                        .get::<i64>("age")
                        .map_err(|e| Error::MissingField("age".to_owned()))?,
                )
                .map_err(|_| Error::TypeMismatch("age".to_owned()))?,
            })
        }
    }

    //
    // Bar impl
    //
    impl FieldSet for Bar {
        fn typename() -> &'static str {
            "Bar"
        }

        fn field_names() -> &'static [&'static str] {
            &["created", "updated"]
        }

        fn as_query_fields() -> &'static str {
            "created: $created, updated: $updated"
        }

        fn as_query_obj() -> &'static str {
            "Bar { created: $created, updated: $updated }"
        }

        fn add_values_to_params(
            &self,
            query: Query,
            prefix: Option<&str>,
            mode: StampMode,
        ) -> Query {
            match mode {
                StampMode::Create => query,
                StampMode::Read => query
                    .param(
                        &format_param("created", prefix),
                        self.created.fixed_offset(),
                    )
                    .param(
                        &format_param("updated", prefix),
                        self.updated.fixed_offset(),
                    ),
                StampMode::Update => query.param(
                    &format_param("created", prefix),
                    self.created.fixed_offset(),
                ),
            }
        }
    }
    impl TryFrom<Row> for Bar {
        type Error = Error;
        fn try_from(value: Row) -> Result<Self, Self::Error> {
            let map = Neo4jMap::Row(&value);
            Ok(Self {
                created: map.get_timestamp("created")?,
                updated: map.get_timestamp("updated")?,
            })
        }
    }

    //
    // Baz impl
    //
    impl FieldSet for Baz {
        fn typename() -> &'static str {
            "BAZ"
        }

        fn field_names() -> &'static [&'static str] {
            &[]
        }

        fn as_query_fields() -> &'static str {
            ""
        }

        fn as_query_obj() -> &'static str {
            "BAZ"
        }

        fn add_values_to_params(&self, query: Query, _: Option<&str>, _: StampMode) -> Query {
            query
        }
    }
    impl TryFrom<Row> for Baz {
        type Error = Error;
        fn try_from(_: Row) -> Result<Self, Self::Error> {
            Ok(Self)
        }
    }

    #[test]
    fn as_obj() {
        // Foo
        assert_eq!(
            Foo::to_query_obj(None, StampMode::Read),
            "Foo { name: $name, age: $age }"
        );
        // Bar
        assert_eq!(
            Bar::to_query_obj(None, StampMode::Read),
            "Bar { created: $created, updated: $updated }"
        );
        assert_eq!(
            Bar::to_query_obj(None, StampMode::Create),
            "Bar { created: datetime(), updated: datetime() }"
        );
        assert_eq!(
            Bar::to_query_obj(None, StampMode::Update),
            "Bar { created: $created, updated: datetime() }"
        );
        // Baz
        assert_eq!(Baz::to_query_obj(None, StampMode::Read), "BAZ");
    }

    #[test]
    fn params() {
        let foo = Foo {
            name: "foo".to_owned(),
            age: 42,
        };
        let bar = Bar {
            created: Utc::now(),
            updated: Utc::now(),
        };
        let baz = Baz;

        // Foo
        let mut q = Query::new(format!(
            "CREATE (n:{})",
            Foo::to_query_obj(None, StampMode::Create)
        ));
        q = foo.add_values_to_params(q, None, StampMode::Create);
        assert!(q.has_param_key("name"));
        assert!(q.has_param_key("age"));

        // Bar
        let mut q = Query::new(format!(
            "MATCH (n:{})",
            Bar::to_query_obj(None, StampMode::Read)
        ));
        q = bar.add_values_to_params(q, None, StampMode::Read);
        assert!(q.has_param_key("created"));
        assert!(q.has_param_key("updated"));

        let mut q = Query::new(format!(
            "CREATE (n:{})",
            Bar::to_query_obj(None, StampMode::Create)
        ));
        q = bar.add_values_to_params(q, None, StampMode::Create);
        assert!(!q.has_param_key("created"));
        assert!(!q.has_param_key("updated"));

        let mut q = Query::new(format!(
            "MERGE (n:{})",
            Bar::to_query_obj(None, StampMode::Update)
        ));
        q = bar.add_values_to_params(q, None, StampMode::Update);
        assert!(q.has_param_key("created"));
        assert!(!q.has_param_key("updated"));

        // Baz
        let mut q = Query::new(format!(
            "MATCH (s:{})
            MATCH (e:{})
            CREATE (s)-[r:{}]->(e)",
            Foo::to_query_obj(Some("s"), StampMode::Read),
            Bar::to_query_obj(Some("e"), StampMode::Read),
            Baz::to_query_obj(None, StampMode::Create),
        ));
        q = foo.add_values_to_params(q, Some("s"), StampMode::Read);
        q = bar.add_values_to_params(q, Some("e"), StampMode::Read);
        q = baz.add_values_to_params(q, None, StampMode::Create);
        assert!(q.has_param_key("s_name"));
        assert!(q.has_param_key("s_age"));
        assert!(q.has_param_key("e_created"));
    }

    #[test]
    fn number_types() {
        let num_types = NumTypes {
            usize_num: 1,
            isize_num: 2,
            u8_num: 3,
            u16_num: 4,
            u32_num: 5,
            u64_num: 6,
            u128_num: 7,
            i8_num: 8,
            i16_num: 9,
            i32_num: 10,
            i64_num: 11,
            i128_num: 12,
            f32_num: 13.0,
            f64_num: 14.0,

            usize_opt: Some(1),
            isize_opt: Some(2),
            u8_opt: Some(3),
            u16_opt: Some(4),
            u32_opt: Some(5),
            u64_opt: Some(6),
            u128_opt: Some(7),
            i8_opt: Some(8),
            i16_opt: Some(9),
            i32_opt: Some(10),
            i64_opt: Some(11),
            i128_opt: Some(12),
            f32_opt: Some(13.0),
            f64_opt: Some(14.0),
        };
        let mut q = Query::new(format!(
            "CREATE (n:{})",
            NumTypes::to_query_obj(None, StampMode::Create)
        ));
        q = num_types.add_values_to_params(q, None, StampMode::Create);
        assert!(q.has_param_key("usize_num"));
        assert!(q.has_param_key("isize_num"));
        assert!(q.has_param_key("u8_num"));
        assert!(q.has_param_key("u16_num"));
        assert!(q.has_param_key("u32_num"));
        assert!(q.has_param_key("u64_num"));
        assert!(q.has_param_key("u128_num"));
        assert!(q.has_param_key("i8_num"));
        assert!(q.has_param_key("i16_num"));
        assert!(q.has_param_key("i32_num"));
        assert!(q.has_param_key("i64_num"));
        assert!(q.has_param_key("i128_num"));
        assert!(q.has_param_key("f32_num"));
        assert!(q.has_param_key("f64_num"));
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct NumTypes {
        pub usize_num: usize,
        pub isize_num: isize,
        pub u8_num: u8,
        pub u16_num: u16,
        pub u32_num: u32,
        pub u64_num: u64,
        pub u128_num: u128,
        pub i8_num: i8,
        pub i16_num: i16,
        pub i32_num: i32,
        pub i64_num: i64,
        pub i128_num: i128,
        pub f32_num: f32,
        pub f64_num: f64,

        pub usize_opt: Option<usize>,
        pub isize_opt: Option<isize>,
        pub u8_opt: Option<u8>,
        pub u16_opt: Option<u16>,
        pub u32_opt: Option<u32>,
        pub u64_opt: Option<u64>,
        pub u128_opt: Option<u128>,
        pub i8_opt: Option<i8>,
        pub i16_opt: Option<i16>,
        pub i32_opt: Option<i32>,
        pub i64_opt: Option<i64>,
        pub i128_opt: Option<i128>,
        pub f32_opt: Option<f32>,
        pub f64_opt: Option<f64>,
    }
    impl Default for NumTypes {
        fn default() -> Self {
            Self {
                usize_num: 0,
                isize_num: 0,
                u8_num: 0,
                u16_num: 0,
                u32_num: 0,
                u64_num: 0,
                u128_num: 0,
                i8_num: 0,
                i16_num: 0,
                i32_num: 0,
                i64_num: 0,
                i128_num: 0,
                f32_num: 0.0,
                f64_num: 0.0,

                usize_opt: None,
                isize_opt: None,
                u8_opt: None,
                u16_opt: None,
                u32_opt: None,
                u64_opt: None,
                u128_opt: None,
                i8_opt: None,
                i16_opt: None,
                i32_opt: None,
                i64_opt: None,
                i128_opt: None,
                f32_opt: None,
                f64_opt: None,
            }
        }
    }
    impl FieldSet for NumTypes {
        fn typename() -> &'static str {
            "NumTypes"
        }

        fn field_names() -> &'static [&'static str] {
            &[
                "usize_num",
                "isize_num",
                "u8_num",
                "u16_num",
                "u32_num",
                "u64_num",
                "u128_num",
                "i8_num",
                "i16_num",
                "i32_num",
                "i64_num",
                "i128_num",
                "f32_num",
                "f64_num",
                "usize_opt",
                "isize_opt",
                "u8_opt",
                "u16_opt",
                "u32_opt",
                "u64_opt",
                "u128_opt",
                "i8_opt",
                "i16_opt",
                "i32_opt",
                "i64_opt",
                "i128_opt",
                "f32_opt",
                "f64_opt",
            ]
        }

        fn as_query_fields() -> &'static str {
            "usize_num: $usize_num, isize_num: $isize_num, u8_num: $u8_num, u16_num: $u16_num, u32_num: $u32_num, u64_num: $u64_num, u128_num: $u128_num, i8_num: $i8_num, i16_num: $i16_num, i32_num: $i32_num, i64_num: $i64_num, i128_num: $i128_num, f32_num: $f32_num, f64_num: $f64_num, usize_opt: $usize_opt, isize_opt: $isize_opt, u8_opt: $u8_opt, u16_opt: $u16_opt, u32_opt: $u32_opt, u64_opt: $u64_opt, u128_opt: $u128_opt, i8_opt: $i8_opt, i16_opt: $i16_opt, i32_opt: $i32_opt, i64_opt: $i64_opt, i128_opt: $i128_opt, f32_opt: $f32_opt, f64_opt: $f64_opt"
        }
        // Trusting copilot: ^^ and vv
        fn as_query_obj() -> &'static str {
            "NumTypes { usize_num: $usize_num, isize_num: $isize_num, u8_num: $u8_num, u16_num: $u16_num, u32_num: $u32_num, u64_num: $u64_num, u128_num: $u128_num, i8_num: $i8_num, i16_num: $i16_num, i32_num: $i32_num, i64_num: $i64_num, i128_num: $i128_num, f32_num: $f32_num, f64_num: $f64_num, usize_opt: $usize_opt, isize_opt: $isize_opt, u8_opt: $u8_opt, u16_opt: $u16_opt, u32_opt: $u32_opt, u64_opt: $u64_opt, u128_opt: $u128_opt, i8_opt: $i8_opt, i16_opt: $i16_opt, i32_opt: $i32_opt, i64_opt: $i64_opt, i128_opt: $i128_opt, f32_opt: $f32_opt, f64_opt: $f64_opt }"
        }

        fn add_values_to_params(&self, q: Query, prefix: Option<&str>, _: StampMode) -> Query {
            // These are the minimal conversions needed before neo4rs v0.7.0.
            q.param(&format_param("usize_num", prefix), self.usize_num as i64)
                .param(&format_param("isize_num", prefix), self.isize_num as i64)
                .param(&format_param("u8_num", prefix), self.u8_num as u16)
                .param(&format_param("u16_num", prefix), self.u16_num)
                .param(&format_param("u32_num", prefix), self.u32_num)
                .param(&format_param("u64_num", prefix), self.u64_num as i64)
                .param(&format_param("u128_num", prefix), self.u128_num as i64)
                .param(&format_param("i8_num", prefix), self.i8_num)
                .param(&format_param("i16_num", prefix), self.i16_num)
                .param(&format_param("i32_num", prefix), self.i32_num)
                .param(&format_param("i64_num", prefix), self.i64_num)
                .param(&format_param("i128_num", prefix), self.i128_num as i64)
                .param(&format_param("f32_num", prefix), self.f32_num)
                .param(&format_param("f64_num", prefix), self.f64_num)
                .param(
                    &format_param("usize_opt", prefix),
                    self.usize_opt.map(|v| v as i64),
                )
                .param(
                    &format_param("isize_opt", prefix),
                    self.isize_opt.map(|v| v as i64),
                )
                .param(
                    &format_param("u8_opt", prefix),
                    self.u8_opt.map(|v| v as u16),
                )
                .param(&format_param("u16_opt", prefix), self.u16_opt)
                .param(&format_param("u32_opt", prefix), self.u32_opt)
                .param(
                    &format_param("u64_opt", prefix),
                    self.u64_opt.map(|v| v as i64),
                )
                .param(
                    &format_param("u128_opt", prefix),
                    self.u128_opt.map(|v| v as i64),
                )
                .param(&format_param("i8_opt", prefix), self.i8_opt)
                .param(&format_param("i16_opt", prefix), self.i16_opt)
                .param(&format_param("i32_opt", prefix), self.i32_opt)
                .param(&format_param("i64_opt", prefix), self.i64_opt)
                .param(
                    &format_param("i128_opt", prefix),
                    self.i128_opt.map(|v| v as i64),
                )
                .param(&format_param("f32_opt", prefix), self.f32_opt)
                .param(&format_param("f64_opt", prefix), self.f64_opt)
        }
    }
    impl TryFrom<Row> for NumTypes {
        type Error = Error;
        fn try_from(value: Row) -> Result<Self, Self::Error> {
            Ok(Self {
                usize_num: usize::try_from(
                    value
                        .get::<i64>("usize_num")
                        .map_err(|e| Error::MissingField("usize_num".to_owned()))?,
                )
                .map_err(|_| Error::TypeMismatch("usize_num".to_owned()))?,
                isize_num: isize::try_from(
                    value
                        .get::<i64>("isize_num")
                        .map_err(|e| Error::MissingField("isize_num".to_owned()))?,
                )
                .map_err(|_| Error::TypeMismatch("isize_num".to_owned()))?,
                u8_num: u8::try_from(
                    value
                        .get::<i64>("u8_num")
                        .map_err(|e| Error::MissingField("u8_num".to_owned()))?,
                )
                .map_err(|_| Error::TypeMismatch("u8_num".to_owned()))?,
                u16_num: u16::try_from(
                    value
                        .get::<i64>("u16_num")
                        .map_err(|e| Error::MissingField("u16_num".to_owned()))?,
                )
                .map_err(|_| Error::TypeMismatch("u16_num".to_owned()))?,
                u32_num: u32::try_from(
                    value
                        .get::<i64>("u32_num")
                        .map_err(|e| Error::MissingField("u32_num".to_owned()))?,
                )
                .map_err(|_| Error::TypeMismatch("u32_num".to_owned()))?,
                u64_num: u64::try_from(
                    value
                        .get::<i64>("u64_num")
                        .map_err(|e| Error::MissingField("u64_num".to_owned()))?,
                )
                .map_err(|_| Error::TypeMismatch("u64_num".to_owned()))?,
                u128_num: u128::try_from(
                    value
                        .get::<i64>("u128_num")
                        .map_err(|e| Error::MissingField("u128_num".to_owned()))?,
                )
                .map_err(|_| Error::TypeMismatch("u128_num".to_owned()))?,
                i8_num: i8::try_from(
                    value
                        .get::<i64>("i8_num")
                        .map_err(|e| Error::MissingField("i8_num".to_owned()))?,
                )
                .map_err(|_| Error::TypeMismatch("i8_num".to_owned()))?,
                i16_num: i16::try_from(
                    value
                        .get::<i64>("i16_num")
                        .map_err(|e| Error::MissingField("i16_num".to_owned()))?,
                )
                .map_err(|_| Error::TypeMismatch("i16_num".to_owned()))?,
                i32_num: i32::try_from(
                    value
                        .get::<i64>("i32_num")
                        .map_err(|e| Error::MissingField("i32_num".to_owned()))?,
                )
                .map_err(|_| Error::TypeMismatch("i32_num".to_owned()))?,
                i64_num: value
                    .get("i64_num")
                    .map_err(|e| Error::MissingField("i64_num".to_owned()))?,
                i128_num: i128::try_from(
                    value
                        .get::<i64>("i128_num")
                        .map_err(|e| Error::MissingField("i128_num".to_owned()))?,
                )
                .map_err(|_| Error::TypeMismatch("i128_num".to_owned()))?,
                f32_num: value
                    .get::<f64>("f32_num")
                    .map_err(|e| Error::MissingField("f32_num".to_owned()))?
                    as f32,
                f64_num: value
                    .get("f64_num")
                    .map_err(|e| Error::MissingField("f64_num".to_owned()))?,

                usize_opt: match value.get::<i64>("usize_opt") {
                    Ok(v) => Some(
                        v.try_into()
                            .map_err(|_| Error::TypeMismatch("usize_opt".to_owned()))?,
                    ),
                    Err(_) => None,
                },
                isize_opt: match value.get::<i64>("isize_opt") {
                    Ok(v) => Some(
                        v.try_into()
                            .map_err(|_| Error::TypeMismatch("isize_opt".to_owned()))?,
                    ),
                    Err(_) => None,
                },
                u8_opt: match value.get::<i64>("u8_opt") {
                    Ok(v) => Some(
                        v.try_into()
                            .map_err(|_| Error::TypeMismatch("u8_opt".to_owned()))?,
                    ),
                    Err(_) => None,
                },
                u16_opt: match value.get::<i64>("u16_opt") {
                    Ok(v) => Some(
                        v.try_into()
                            .map_err(|_| Error::TypeMismatch("u16_opt".to_owned()))?,
                    ),
                    Err(_) => None,
                },

                u32_opt: match value.get::<i64>("u32_opt") {
                    Ok(v) => Some(
                        v.try_into()
                            .map_err(|_| Error::TypeMismatch("u32_opt".to_owned()))?,
                    ),
                    Err(_) => None,
                },
                u64_opt: match value.get::<i64>("u64_opt") {
                    Ok(v) => Some(
                        v.try_into()
                            .map_err(|_| Error::TypeMismatch("u64_opt".to_owned()))?,
                    ),
                    Err(_) => None,
                },
                u128_opt: match value.get::<i64>("u128_opt") {
                    Ok(v) => Some(
                        v.try_into()
                            .map_err(|_| Error::TypeMismatch("u128_opt".to_owned()))?,
                    ),
                    Err(_) => None,
                },
                i8_opt: match value.get::<i64>("i8_opt") {
                    Ok(v) => Some(
                        v.try_into()
                            .map_err(|_| Error::TypeMismatch("i8_opt".to_owned()))?,
                    ),
                    Err(_) => None,
                },
                i16_opt: match value.get::<i64>("i16_opt") {
                    Ok(v) => Some(
                        v.try_into()
                            .map_err(|_| Error::TypeMismatch("i16_opt".to_owned()))?,
                    ),
                    Err(_) => None,
                },
                i32_opt: match value.get::<i64>("i32_opt") {
                    Ok(v) => Some(
                        v.try_into()
                            .map_err(|_| Error::TypeMismatch("i32_opt".to_owned()))?,
                    ),
                    Err(_) => None,
                },
                i64_opt: match value.get("i64_opt") {
                    Ok(v) => Some(v),
                    Err(_) => None,
                },
                i128_opt: match value.get::<i64>("i128_opt") {
                    Ok(v) => Some(
                        v.try_into()
                            .map_err(|_| Error::TypeMismatch("i128_opt".to_owned()))?,
                    ),
                    Err(_) => None,
                },
                f32_opt: match value.get::<f64>("f32_opt") {
                    Ok(v) => Some(v as f32),
                    Err(_) => None,
                },
                f64_opt: match value.get("f64_opt") {
                    Ok(v) => Some(v),
                    Err(_) => None,
                },
            })
        }
    }
}
