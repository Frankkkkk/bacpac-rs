use std::fmt;

use serde::de::value::MapAccessDeserializer;
use serde::de::{Error, MapAccess, Visitor};
use serde::Deserialize;
use tracing::{debug, error, field, span, Level};

#[derive(Debug, PartialEq)]
pub struct SqlTable {
    pub name: String,
    pub properties: Option<crate::Property>,
    pub columns_relationship: SqlTableColumnRelationship,
    pub schema_relationship: SqlTableSchemaRelationship,
}

#[derive(Debug, PartialEq)]
enum ColumnsOrSchema {
    Columns(SqlTableColumnRelationship),
    Schema(SqlTableSchemaRelationship),
}

impl<'de> Deserialize<'de> for SqlTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SqlTableVisitor;

        impl<'de> Visitor<'de> for SqlTableVisitor {
            type Value = SqlTable;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an object with a `type` field")
            }

            fn visit_map<A>(self, mut map: A) -> Result<SqlTable, A::Error>
            where
                A: MapAccess<'de>,
            {
                let span = span!(Level::INFO, "SqlTable", name = field::Empty);
                let _guard = span.enter();

                let mut name: Option<String> = None;
                let mut columns: SqlTableColumnRelationship =
                    SqlTableColumnRelationship { entry: Vec::new() };
                let mut schema: Option<SqlTableSchemaRelationship> = None;
                let mut prop: Option<crate::Property> = None;

                while let Some(key) = &map.next_key::<String>()? {
                    match key.as_str() {
                        "Property" => {
                            prop = Some(map.next_value::<crate::Property>()?);
                            debug!("Property: {:#?}", prop);
                        }
                        "@Name" => {
                            let table_name = map.next_value::<String>()?;
                            span.record("name", table_name.clone());
                            debug!("Name: {:#?}", table_name);
                            name = Some(table_name);
                        }
                        "Relationship" => {
                            let span = span!(Level::INFO, "Relationship");
                            let _guard = span.enter();
                            debug!("Relationship >>>");

                            let xx = map.next_value::<ColumnsOrSchema>()?;
                            debug!("<<<ColumnsOrSchema: {:#?}", xx);
                            match xx {
                                ColumnsOrSchema::Columns(c) => {
                                    columns = c;
                                }
                                ColumnsOrSchema::Schema(s) => {
                                    schema = Some(s);
                                }
                            }
                        }
                        _ => {
                            let value = map.next_value::<String>()?;
                            error!("Unknown key: {:#?} -> {:#?}", key, value);
                        }
                    }
                }
                debug!("Returning");

                match (name, schema) {
                    (Some(name), Some(schema)) => Ok(SqlTable {
                        name,
                        properties: prop,
                        columns_relationship: columns,
                        schema_relationship: schema,
                    }),
                    (None, _) => Err(Error::custom("SqlTable has no Name attribute")),
                    (Some(name), None) => Err(Error::custom(format!("No Schema for table {name}"))),
                }
            }
        }
        deserializer.deserialize_map(SqlTableVisitor)
    }
}

impl<'de> Deserialize<'de> for ColumnsOrSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ColumnsOrSchemaVisitor;

        impl<'de> Visitor<'de> for ColumnsOrSchemaVisitor {
            type Value = ColumnsOrSchema;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a ColumnsOrSchema object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<ColumnsOrSchema, A::Error>
            where
                A: MapAccess<'de>,
            {
                let span = span!(Level::INFO, "ColumnsOrSchema");
                let _guard = span.enter();

                while let Some((key, value)) = map.next_entry::<String, String>()? {
                    let mad = MapAccessDeserializer::new(&mut map);
                    match key.as_str() {
                        "@Name" => {
                            match value.as_str() {
                                "Columns" => {
                                    debug!("Columns");
                                    let f = SqlTableColumnRelationship::deserialize(mad)?;
                                    debug!("Columns: {:#?}", f);
                                    return Ok(ColumnsOrSchema::Columns(f));
                                }
                                "Schema" => {
                                    debug!("Schema");
                                    let schema = SqlTableSchemaRelationship::deserialize(mad)?;
                                    return Ok(ColumnsOrSchema::Schema(schema));
                                }
                                _ => {
                                    return Err(Error::custom(format!("Unknown Name={value}. Expected either `Columns` or `Schema`")));
                                }
                            }
                        }
                        _ => continue,
                    }
                }
                error!("Didn't return because no Columns or Schema");
                Err(Error::custom(
                    "expected `Name` attribute on Relationship element",
                ))
            }
        }
        deserializer.deserialize_map(ColumnsOrSchemaVisitor)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SqlTableColumnRelationship {
    #[serde(rename = "Entry")]
    pub entry: Vec<SqlTableColumnRelationshipEntry>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SqlTableColumnRelationshipEntry {
    #[serde(rename = "Element")]
    pub element: SqlSimpleColumnTableElement,
}

#[derive(Debug, Deserialize, PartialEq)]
//#[serde(deny_unknown_fields)]
pub struct SqlSimpleColumnTableElement {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Property")]
    pub properties: Option<Vec<crate::Property>>,
    #[serde(rename = "Relationship")]
    pub relationship: SqlSimpleColumnRelationshipEntry,
    #[serde(rename = "Annotation")]
    pub annotation: Option<SqlSimpleColumnAnnotation>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SqlSimpleColumnAnnotation {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub ty: String,
    #[serde(rename = "Property")]
    pub property: crate::Property,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SqlSimpleColumnRelationshipEntry {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Entry")]
    pub entry: SqlSimpleColumnRelationshipEntryReference,
}
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SqlSimpleColumnRelationshipEntryReference {
    #[serde(rename = "Element")]
    pub element_type_specifier: ElementTypeSpecifier,
}
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ElementTypeSpecifier {
    #[serde(rename = "@Type")]
    pub ty: String,
    #[serde(rename = "Property")]
    pub property: Option<crate::Property>,
    #[serde(rename = "Relationship")]
    pub type_specifier_rela: TypeSpecifierRelationship,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TypeSpecifierRelationship {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Entry")]
    pub entry: TypeSpecifierRelationshipEntry,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TypeSpecifierRelationshipEntry {
    #[serde(rename = "References")]
    pub element: SqlTableReference,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SqlTableReference {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@ExternalSource")]
    pub external_source: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SqlTableSchemaRelationship {
    #[serde(rename = "Entry")]
    pub entry: SqlTableSchemaRelationshipEntry,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SqlTableSchemaRelationshipEntry {
    #[serde(rename = "References")]
    pub references: SqlTableReference,
}
