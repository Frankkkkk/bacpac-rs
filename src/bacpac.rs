use std::io::Read;

use crate::{
    simple::{self, SimpleDacPacModel},
    DacPacModel,
};
use anyhow::Result;

#[derive(Debug)]
pub struct BacPacModel {
    file: std::fs::File,
    pub simple_dacpac: SimpleDacPacModel,
}

#[derive(Debug)]
pub struct TableData {
    pub headers: Vec<TableColumnHeader>,
    pub rows: Vec<TableRowData>,
}

#[derive(Debug)]
pub struct TableColumnHeader {
    pub name: String,
    pub ty: simple::SimpleColumnType,
}

#[derive(Debug)]
pub struct TableRowData {
    pub data: Vec<simple::SimpleColumnValue>,
}

impl BacPacModel {
    pub fn from_file(file: std::fs::File) -> Result<BacPacModel> {
        let dc = DacPacModel::from_file(&file)?;
        Ok(BacPacModel {
            file,
            simple_dacpac: SimpleDacPacModel::from(&dc),
        })
    }
    pub fn read_data(&self, table_name: String) -> Result<TableData> {
        let table = self
            .simple_dacpac
            .tables
            .iter()
            .find(|t| t.name == table_name);

        let table = match table {
            Some(t) => t,
            None => return Err(anyhow::anyhow!("Table {table_name} not found")),
        };

        let mut headers: Vec<TableColumnHeader> = vec![];
        for row in table.columns.iter() {
            headers.push(TableColumnHeader {
                name: row.name.clone(),
                ty: row.ty.clone(),
            });
        }
        println!("Headers: {:#?}", headers);

        let archive = zip::ZipArchive::new(&self.file).unwrap();

        let folder_prefix = format!("Data/{}", table_name);

        let mut bcp_files: Vec<String> = vec![];
        for file in archive.file_names() {
            println!("File: {:#?}", file);
            println!("Starts with: {:#?}", file.starts_with(&folder_prefix));
            println!("Ends with: {:#?}", file.ends_with(".BCP"));
            if file.starts_with(&folder_prefix) && file.ends_with(".BCP") {
                bcp_files.push(file.to_string());
            }
        }
        bcp_files.sort();

        println!("Files: {:#?}", bcp_files);

        let rows = Self::parse_bcp(&headers, &bcp_files, &archive);

        Err(anyhow::anyhow!("Not implemented"))
    }

    fn parse_bcp(
        headers: &Vec<TableColumnHeader>,
        bcp_files: &Vec<String>,
        archive: &zip::ZipArchive<std::fs::File>,
    ) -> Vec<TableRowData> {
        let mut rows: Vec<TableRowData> = vec![];

        for file in bcp_files {
            let mut bcp_file = archive.by_name(file).unwrap();
            let b = bcp_file.read(buf)
            println!("File: {:#?}", b);

            let mut contents = String::new();
            bcp_file.read_to_string(&mut contents).unwrap();

            let lines = contents.lines();
            for line in lines {
                let mut row_data: Vec<simple::SimpleColumnValue> = vec![];
                let mut line = line.to_string();
                line.pop(); // Remove the last character which is a delimiter
                let mut line = line.split("\t");
                for header in headers {
                    let value = line.next().unwrap();
                    let value = match header.ty {
                        simple::SimpleColumnType::Int => {
                            simple::SimpleColumnValue::Int(value.parse().unwrap())
                        }
                        simple::SimpleColumnType::BigInt => {
                            simple::SimpleColumnValue::BigInt(value.parse().unwrap())
                        }
                        simple::SimpleColumnType::Nvarchar(_) => {
                            simple::SimpleColumnValue::Nvarchar(value.to_string())
                        }
                        simple::SimpleColumnType::Varchar(_) => {
                            simple::SimpleColumnValue::Varchar(value.to_string())
                        }
                        simple::SimpleColumnType::DateTime2(_) => {
                            simple::SimpleColumnValue::DateTime2(value.to_string())
                        }
                    };
                    row_data.push(value);
                }
                rows.push(TableRowData { data: row_data });
            }
        }

        rows
    }
}
