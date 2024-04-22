use std::env;

use dacpac::DacPacModel;
use quick_xml::de::from_str;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting deserialization of XML");

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let model: DacPacModel = from_str(&contents).unwrap();
    println!("{:#?}", model);

    /*
    for e in model.model.element {
        match &e {
            ElementEnum::SqlTable(t) => {
                println!("FUCK YEAH!");
                println!("Table: {:#?}", t);
                /*
                for p in &t.properties {
                    println!("Property: {:?}", p);
                }
                */
            }
            _ => {}
        }
    }
    */
}
