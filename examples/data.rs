use std::env;

use dacpac::{bacpac, simple::SimpleDacPacModel};

fn main() {
    let args: Vec<String> = env::args().collect();
    let _filename = &args[1];

    let fname = std::path::Path::new(&*args[1]);
    let zipfile = std::fs::File::open(fname).unwrap();

    let bacpacc = bacpac::BacPacModel::from_file(zipfile).unwrap();
    //for table in sdm.tables {
    //    println!("{:#?}", table.name);
    // }

    let _table_data = bacpacc.read_data("dbo.Table".to_string()).unwrap();
}
