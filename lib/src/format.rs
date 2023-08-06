/// Utility function for formatting part of a cypher [Query].
///
/// [prefix] applies to placeholders only, not field names. For example, `foo: $foo` would become `foo: $prefix_foo`.
pub fn format_query_fields<I, S>(fields: I, prefix: Option<&str>) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let prefix = prefix_str(prefix);
    let mut formatted = Vec::new();
    for field in fields {
        if field.as_ref().is_empty() {
            continue;
        }
        formatted.push(format!("{}: ${}{}", field.as_ref(), prefix, field.as_ref()));
    }
    formatted.join(", ")
}

/// Utility function for formatting a query object with fields.
#[allow(dead_code)]
pub fn format_query_obj<I, S>(name: &str, fields: I, prefix: Option<&str>) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let fields = format_query_fields(fields, prefix);
    if fields.is_empty() {
        return name.to_owned();
    }
    format!("{} {{ {} }}", name, fields)
}

/// Formats a parameter name with or without a prefix. For example, `foo` would become `prefix_foo`.
pub fn format_param(name: &str, prefix: Option<&str>) -> String {
    if name.is_empty() {
        return "".to_owned();
    }
    format!("{}{}", prefix_str(prefix), name)
}

fn prefix_str(prefix: Option<&str>) -> String {
    match prefix {
        Some(p) => format!("{}_", p),
        None => "".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params() {
        assert_eq!(format_param("foo", None), "foo");
        assert_eq!(format_param("foo", Some("n")), "n_foo");
        assert_eq!(format_param("", Some("n")), "");
    }

    #[test]
    fn fields() {
        assert_eq!(
            format_query_fields(["foo", "bar"], None),
            "foo: $foo, bar: $bar"
        );
        assert_eq!(
            format_query_fields(["foo", "bar"], Some("n")),
            "foo: $n_foo, bar: $n_bar"
        );
        assert_eq!(format_query_fields(["foo"], None), "foo: $foo");
        assert_eq!(format_query_fields::<&[_; 0], &String>(&[], None), "");
        assert_eq!(format_query_fields::<&[_; 0], &String>(&[], Some("n")), "");
        assert_eq!(format_query_fields(["", ""], Some("n")), "");
    }

    #[test]
    fn obj() {
        assert_eq!(
            format_query_obj("Foo", ["foo", "bar"], None),
            "Foo { foo: $foo, bar: $bar }"
        );
        assert_eq!(
            format_query_obj("Foo", ["foo", "bar"], Some("n")),
            "Foo { foo: $n_foo, bar: $n_bar }"
        );
        assert_eq!(format_query_obj("Foo", ["foo"], None), "Foo { foo: $foo }");
        assert_eq!(
            format_query_obj::<&[_; 0], &String>("Foo", &[], None),
            "Foo"
        );
        assert_eq!(
            format_query_obj::<&[_; 0], &String>("FOO", &[], Some("n")),
            "FOO"
        );
    }
}
