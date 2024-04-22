//! This example demonstrates how to deserialize enum nodes using an intermediate
//! custom deserializer.
//! The `elem` node can either be a `Foo` or a `Bar` node, depending on the `type`.
//! The `type` attribute is used to determine which variant to deserialize.
//! This is a workaround for [serde's issue](https://github.com/serde-rs/serde/issues/1905)
//!
//! note: to use serde, the feature needs to be enabled
//! run example with:
//!    cargo run --example flattened_enum --features="serialize"

use anyhow::Result;
use core::{panic, str};
use simple::SimpleDacPacModel;
use std::fmt;
use std::fs::File;
use std::io::Read;

use quick_xml::de::from_str;
use serde::de::value::MapAccessDeserializer;
use serde::de::{Error, MapAccess, Visitor};
use serde::Deserialize;
use table::SqlTable;

pub mod bacpac;
pub mod simple;
pub mod table;

/// Deserializes a DacPac `model.xml` from an XML string
pub fn from_xml(xml: &str) -> DacPacModel {
    let dsm: DacPacModel = from_str(xml).unwrap();
    dsm
}

pub fn from_dacpac_file(file: &File) -> Result<DacPacModel> {
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut model_xml = archive.by_name("model.xml")?;

    let mut contents = String::new();
    model_xml.read_to_string(&mut contents).unwrap();

    Ok(from_xml(contents.as_str()))
}

impl DacPacModel {
    pub fn from_file(file: &File) -> Result<DacPacModel> {
        from_dacpac_file(file)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DacPacModel {
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
    SqlView(SqlView),
    SqlUniqueConstraint(SqlUniqueConstraint),
    SqlProcedure(SqlProcedure),
    SqlPermissionStatement(SqlPermissionStatement),
    SqlSchema(SqlSchema),
    SqlExternalFileFormat(SqlExternalFileFormat),
    SqlExternalDataSource(SqlExternalDataSource),
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
#[serde(deny_unknown_fields)]
pub struct Property {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Value")]
    pub value: Option<String>,
}

impl Property {
    pub fn get_value(&self) -> String {
        self.value.clone().unwrap_or_default()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlView {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlUniqueConstraint {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlProcedure {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlPermissionStatement {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlSchema {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlExternalFileFormat {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SqlExternalDataSource {}

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
                                    let f = SqlTable::deserialize(mad)?;
                                    Ok(ElementEnum::SqlTable(f))
                                }
                                "SqlView" => {
                                    let f = SqlView::deserialize(mad)?;
                                    Ok(ElementEnum::SqlView(f))
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
                                "SqlExternalFileFormat" => {
                                    let f = SqlExternalFileFormat::deserialize(mad)?;
                                    Ok(ElementEnum::SqlExternalFileFormat(f))
                                }
                                "SqlExternalDataSource" => {
                                    let f = SqlExternalDataSource::deserialize(mad)?;
                                    Ok(ElementEnum::SqlExternalDataSource(f))
                                }
                                _ => {
                                    todo!("Unknown SQL type attribute `{}`", value);
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

impl From<&DacPacModel> for simple::SimpleDacPacModel {
    fn from(dm: &DacPacModel) -> Self {
        let mut tables: Vec<simple::SimpleTable> = Vec::new();
        //let mut views: Vec<simple::SimpleView> = Vec::new();

        for e in &dm.model.element {
            #[allow(clippy::single_match)]
            match e {
                ElementEnum::SqlTable(t) => {
                    tables.push(simple::SimpleTable::from(t));
                }
                _ => {}
            }
        }
        SimpleDacPacModel { tables }
    }
}

impl From<&SqlTable> for simple::SimpleTable {
    fn from(st: &SqlTable) -> Self {
        let mut columns: Vec<simple::SimpleTableColumn> = Vec::new();

        for column in &st.columns_relationship.entry {
            columns.push(simple::SimpleTableColumn::from(&column.element));
        }

        simple::SimpleTable {
            name: simple::remove_delimiters(&st.name),
            columns,
        }
    }
}

impl From<&table::SqlSimpleColumnTableElement> for simple::SimpleTableColumn {
    fn from(st: &table::SqlSimpleColumnTableElement) -> Self {
        let mut nullable = false;
        st.properties.iter().flatten().for_each(|property| {
            if property.name.as_str() == "IsNullable" {
                nullable = property.value.as_ref().unwrap_or(&"".to_string()) != "False"
            }
        });

        let rname = simple::remove_delimiters(&st.name);
        let row_name_last = rname.split('.').last().unwrap();

        simple::SimpleTableColumn {
            name: row_name_last.to_string(),
            nullable,
            ty: simple::SimpleColumnType::from(&st.relationship.entry.element_type_specifier),
            default: None,
        }
    }
}

impl From<&table::ElementTypeSpecifier> for simple::SimpleColumnType {
    fn from(st: &table::ElementTypeSpecifier) -> Self {
        match st.type_specifier_rela.entry.element.name.as_str() {
            "[int]" => simple::SimpleColumnType::Int,
            "[bigint]" => simple::SimpleColumnType::BigInt,
            "[nvarchar]" => match &st.property {
                Some(prop) => match prop.name.as_str() {
                    "Length" => {
                        let n = prop.get_value().parse::<i32>().unwrap();
                        simple::SimpleColumnType::Nvarchar(n)
                    }
                    _ => simple::SimpleColumnType::Nvarchar(0),
                },
                None => simple::SimpleColumnType::Nvarchar(0),
            },
            "[varchar]" => match &st.property {
                Some(prop) => match prop.name.as_str() {
                    "Length" => {
                        let n = prop.get_value().parse::<i32>().unwrap();
                        simple::SimpleColumnType::Varchar(n)
                    }
                    _ => simple::SimpleColumnType::Varchar(0),
                },
                None => simple::SimpleColumnType::Varchar(0),
            },
            "[datetime2]" => match &st.property {
                Some(prop) => match prop.name.as_str() {
                    "Scale" => {
                        let n = prop.get_value().parse::<i8>().unwrap();
                        simple::SimpleColumnType::DateTime2(n)
                    }
                    _ => simple::SimpleColumnType::DateTime2(0),
                },
                None => simple::SimpleColumnType::DateTime2(0),
            },
            _ => panic!("Unknown type: {:#?}", st),
        }
    }
}
