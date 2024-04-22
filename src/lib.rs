//! This example demonstrates how to deserialize enum nodes using an intermediate
//! custom deserializer.
//! The `elem` node can either be a `Foo` or a `Bar` node, depending on the `type`.
//! The `type` attribute is used to determine which variant to deserialize.
//! This is a workaround for [serde's issue](https://github.com/serde-rs/serde/issues/1905)
//!
//! note: to use serde, the feature needs to be enabled
//! run example with:
//!    cargo run --example flattened_enum --features="serialize"

use core::str;
use std::fmt;
use std::iter::Map;

use quick_xml::de::from_str;
use serde::de::value::MapAccessDeserializer;
use serde::de::{Deserializer, Error, MapAccess, Visitor};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct DataSchemaModel {
    #[serde(rename = "Model")]
    pub model: Model,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Model {
    #[serde(rename = "Element")]
    pub element: Vec<ElementEnum>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Element {
    #[serde(rename = "@Name")] //, default = "default_name")]
    pub name: Option<String>,
    pub el: ElementEnum,
}

#[derive(Debug, PartialEq)]
pub enum ElementEnum {
    SqlDatabaseOptions(SqlDatabaseOptions),
    SqlDefaultConstraint(SqlDefaultConstraint),
    SqlPrimaryKeyConstraint(SqlPrimaryKeyConstraint),
    SqlRoleMembership(SqlRoleMembership),
    SqlUser(SqlUser),
    SqlTable(SqlTable),
    SqlUniqueConstraint(SqlUniqueConstraint),
    SqlProcedure(SqlProcedure),
    SqlPermissionStatement(SqlPermissionStatement),
    SqlSchema(SqlSchema),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlDatabaseOptions {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlDefaultConstraint {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlPrimaryKeyConstraint {}
#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlRoleMembership {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlUser {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlTable {
    #[serde(rename = "Property")]
    pub properties: Property,
    #[serde(rename = "Relationship")]
    pub columns_relationship: SqlTableColumnRelationship,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlTableColumnRelationship {
    #[serde(rename = "Entry")]
    pub entry: Vec<SqlTableColumnRelationshipEntry>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlTableColumnRelationshipEntry {
    #[serde(rename = "Element")]
    pub element: SqlSimpleColumnTableElement,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlSimpleColumnTableElement {
    #[serde(rename = "Property")]
    pub property: Option<Property>,
    #[serde(rename = "Relationship")]
    pub columns: Vec<SqlSimpleColumnRelationshipEntry>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlSimpleColumnRelationshipEntry {
    #[serde(rename = "Entry")]
    pub entry: SqlSimpleColumnRelationshipEntryReference,
}
#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlSimpleColumnRelationshipEntryReference {
    #[serde(rename = "Element")]
    pub element_type_specifier: ElementTypeSpecifier,
}
#[derive(Debug, Deserialize, PartialEq)]
pub struct ElementTypeSpecifier {
    #[serde(rename = "Property")]
    pub property: Option<Property>,
    #[serde(rename = "Relationship")]
    pub type_specifier_rela: TypeSpecifierRelationship,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TypeSpecifierRelationship {
    #[serde(rename = "Entry")]
    pub entry: TypeSpecifierRelationshipEntry,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TypeSpecifierRelationshipEntry {
    #[serde(rename = "References")]
    pub element: TypeSpecifierRelationshipEntryElement,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TypeSpecifierRelationshipEntryElement {
    #[serde(rename = "@Name")]
    pub name: String,
}

fn deserialize_sql_simple_column_type<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    struct Relationship {
        #[serde(rename = "Entry")]
        entry: Entry,
    }
    #[derive(Deserialize, Debug)]
    struct Entry {
        #[serde(rename = "References")]
        reference: Reference,
    }
    #[derive(Deserialize, Debug)]
    struct Reference {
        #[serde(rename = "@Name")]
        name: String,
    }
    println!("DESERIALIZING SQL SIMPLE COLUMN TYPE");
    let s: Reference = Deserialize::deserialize(deserializer)?;
    println!("SQL SIMPLE COLUMN TYPE: {:#?}", s);
    Ok("home".to_string())
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Property {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlUniqueConstraint {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlProcedure {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlPermissionStatement {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlSchema {}

impl<'de> Deserialize<'de> for ElementEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ElemVisitor;

        impl<'de> Visitor<'de> for ElemVisitor {
            type Value = ElementEnum;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an object with a `type` field")
            }

            fn visit_map<A>(self, mut map: A) -> Result<ElementEnum, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Some((key, value)) = map.next_entry::<String, String>()? {
                    return match key.as_str() {
                        "@Type" => {
                            let value = value.as_str();
                            let mad = MapAccessDeserializer::new(map);
                            match value {
                                "SqlDatabaseOptions" => {
                                    let f = SqlDatabaseOptions::deserialize(mad)?;
                                    Ok(ElementEnum::SqlDatabaseOptions(f))
                                }
                                "SqlDefaultConstraint" => {
                                    let f = SqlDefaultConstraint::deserialize(mad)?;
                                    Ok(ElementEnum::SqlDefaultConstraint(f))
                                }
                                "SqlPrimaryKeyConstraint" => {
                                    let f = SqlPrimaryKeyConstraint::deserialize(mad)?;
                                    Ok(ElementEnum::SqlPrimaryKeyConstraint(f))
                                }
                                "SqlRoleMembership" => {
                                    let f = SqlRoleMembership::deserialize(mad)?;
                                    Ok(ElementEnum::SqlRoleMembership(f))
                                }
                                "SqlUser" => {
                                    let f = SqlUser::deserialize(mad)?;
                                    Ok(ElementEnum::SqlUser(f))
                                }
                                "SqlTable" => {
                                    println!("Will do table");
                                    let f = SqlTable::deserialize(mad)?;
                                    Ok(ElementEnum::SqlTable(f))
                                }
                                "SqlUniqueConstraint" => {
                                    let f = SqlUniqueConstraint::deserialize(mad)?;
                                    Ok(ElementEnum::SqlUniqueConstraint(f))
                                }
                                "SqlProcedure" => {
                                    let f = SqlProcedure::deserialize(mad)?;
                                    Ok(ElementEnum::SqlProcedure(f))
                                }
                                "SqlPermissionStatement" => {
                                    let f = SqlPermissionStatement::deserialize(mad)?;
                                    Ok(ElementEnum::SqlPermissionStatement(f))
                                }
                                "SqlSchema" => {
                                    let f = SqlSchema::deserialize(mad)?;
                                    Ok(ElementEnum::SqlSchema(f))
                                }
                                _ => {
                                    panic!("unknown type attribute `{}`", value);
                                }
                            }
                        }
                        _ => Err(Error::custom(format!("unknown type attribute `{}`", key))),
                    };
                }
                Err(Error::custom("expected `type` attribute"))
            }
        }
        deserializer.deserialize_map(ElemVisitor)
    }
}
