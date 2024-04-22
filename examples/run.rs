use bacpac_rs::DataSchemaModel;
use bacpac_rs::ElementEnum;
use quick_xml::de::from_str;

fn main() {
    let file = "simple.xml";
    let contents = std::fs::read_to_string(file).expect("Something went wrong reading the file");

    let model: DataSchemaModel = from_str(&contents).unwrap();
    //println!("{:?}", model);

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
                panic!("")
            }
            _ => {}
        }
    }
}
