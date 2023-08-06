use chrono::{DateTime, FixedOffset, Utc};

use crate::{format_query_fields, StampMode};

/// The standard timestamps an object uses, and their field names.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stamps {
    None,
    Created(&'static str),
    Updated(&'static str),
    Both(&'static str, &'static str),
}
impl Stamps {
    /// Parse a set of field names.
    ///
    /// `created_at` and `updated_at` have priority.
    /// `created` and `updated` are also supported.
    pub fn from_fields(fields: &[&'static str]) -> (Stamps, Vec<&'static str>) {
        let mut has_created_at = false;
        let mut has_updated_at = false;
        let mut has_created = false;
        let mut has_updated = false;
        let mut non_stamp_fields = Vec::new();
        for field in fields {
            match field.as_ref() {
                "created_at" => has_created_at = true,
                "updated_at" => has_updated_at = true,
                "created" => has_created = true,
                "updated" => has_updated = true,
                _ => non_stamp_fields.push(*field),
            }
        }
        let created_field = if has_created_at {
            if has_created {
                non_stamp_fields.push("created")
            }
            Some("created_at")
        } else if has_created {
            Some("created")
        } else {
            None
        };
        let updated_field = if has_updated_at {
            if has_updated {
                non_stamp_fields.push("updated")
            }
            Some("updated_at")
        } else if has_updated {
            Some("updated")
        } else {
            None
        };
        let stamps = match (created_field, updated_field) {
            (Some(created), Some(updated)) => Stamps::Both(created, updated),
            (Some(created), None) => Stamps::Created(created),
            (None, Some(updated)) => Stamps::Updated(updated),
            (None, None) => Stamps::None,
        };
        (stamps, non_stamp_fields)
    }

    /// Returns these timestamp fields as a query string, depending on the [StampMode].
    ///
    /// Similar to [QueryFields::as_query_fields], but with a Stamps instance.
    pub fn as_query_fields(&self, prefix: Option<&str>, mode: StampMode) -> String {
        if self == &Stamps::None {
            return "".to_owned();
        }
        match mode {
            // Use placeholders
            StampMode::Read => {
                let (created, updated) = self.field_names();
                format_query_fields([created, updated], prefix)
            }
            // Hardcode datetime()
            StampMode::Create => match self {
                Stamps::None => "".to_owned(),
                Stamps::Created(name) => format!("{}: datetime()", name),
                Stamps::Updated(name) => format!("{}: datetime()", name),
                Stamps::Both(created, updated) => {
                    format!("{}: datetime(), {}: datetime()", created, updated)
                }
            },
            // created: placeholder, updated: datetime()
            StampMode::Update => match self {
                Stamps::None => "".to_owned(),
                Stamps::Created(name) => format_query_fields([name], prefix),
                Stamps::Updated(name) => format!("{}: datetime()", name),
                Stamps::Both(created, updated) => {
                    format!(
                        "{}, {}: datetime()",
                        format_query_fields([created], prefix),
                        updated
                    )
                }
            },
        }
    }

    /// The created and updated field names. This may return empty strings.
    ///
    /// Convenience method rather than having to match [Stamps] every time.
    fn field_names(&self) -> (&'static str, &'static str) {
        match self {
            Stamps::None => ("", ""),
            Stamps::Created(name) => (*name, ""),
            Stamps::Updated(name) => ("", *name),
            Stamps::Both(created, updated) => (*created, *updated),
        }
    }
}

/// An abstraction over the different types of returned values from Neo4j.
pub enum Neo4jMap<'a> {
    Row(&'a neo4rs::Row),
    Node(&'a neo4rs::Node),
    Relation(&'a neo4rs::Relation),
    UnboundedRelation(&'a neo4rs::UnboundedRelation),
}
impl<'a> Neo4jMap<'a> {
    pub fn get_timestamp(&self, name: &str) -> Result<DateTime<Utc>, crate::Error> {
        match self {
            Neo4jMap::Row(value) => value
                .get::<DateTime<FixedOffset>>(name)
                .map(|dt| dt.into())
                .ok_or(crate::Error::MissingField(name.to_owned())),
            Neo4jMap::Node(value) => value
                .get::<DateTime<FixedOffset>>(name)
                .map(|dt| dt.into())
                .ok_or(crate::Error::MissingField(name.to_owned())),
            Neo4jMap::Relation(value) => value
                .get::<DateTime<FixedOffset>>(name)
                .map(|dt| dt.into())
                .ok_or(crate::Error::MissingField(name.to_owned())),
            Neo4jMap::UnboundedRelation(value) => value
                .get::<DateTime<FixedOffset>>(name)
                .map(|dt| dt.into())
                .ok_or(crate::Error::MissingField(name.to_owned())),
        }
    }
}
