use std::env;

use dacpac::simple::SimpleDacPacModel;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _filename = &args[1];

    let fname = std::path::Path::new(&*args[1]);
    let zipfile = std::fs::File::open(fname).unwrap();

    let dacpac = dacpac::from_dacpac_file(&zipfile).unwrap();
    println!("{:#?}", dacpac);
    println!("\n\n------------\n\n");

    let sdm = SimpleDacPacModel::from(&dacpac);

    println!("SDM: {:#?}", sdm);
}
