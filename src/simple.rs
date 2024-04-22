#[derive(Debug)]
pub struct SimpleDacPacModel {
    pub tables: Vec<SimpleTable>,
    //pub views: Vec<SimpleView>,
    // TODO: continue
}

#[derive(Debug)]
pub struct SimpleTable {
    pub name: String,
    pub columns: Vec<SimpleTableColumn>,
}

#[derive(Debug)]
pub struct SimpleTableColumn {
    pub name: String,
    pub ty: SimpleColumnType,
    pub nullable: bool,
    pub default: Option<String>,
    //pub identity: bool,
    //pub computed: bool,
    //pub primary_key: bool,
    //pub unique: bool,
    //pub check: Option<String>,
    //pub foreign_key: Option<SimpleForeignKey>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleColumnType {
    Int,
    BigInt,
    Nvarchar(i32),
    Varchar(i32),
    DateTime2(i8),
}

#[derive(Debug)]
pub enum SimpleColumnValue {
    Int(i32),
    BigInt(i64),
    Nvarchar(String),
    Varchar(String),
    DateTime2(String),
}

pub fn remove_delimiters(name: &String) -> String {
    //This shit works.. somewhat.. To fix it.. someday
    name.replace("[", "").replace("]", "")
}
