use crate::archive_tools::{structs::{FormBase, FormString, FormWorld}, types::{FormID, GlobalID, FormType, LangCode, StrLrg, StrSml}};

#[allow(unused)]
pub fn test_forms() {
    println!("--- Testing Forms ---");

    // - FormBase - //
    println!("-- FormBase --");

    let form_base_1 = FormBase {
        form_id: FormID::from("00001"),
        form_type: FormType::STRING,
        form_name: StrSml::from("StrTest1"),
    };

    println!("FormBase 1: \n{:?}", form_base_1.to_string());
    println!("FormBase 1 byte count: {}", form_base_1.get_byte_count());

    println!("FormBase 1 as bytes: {:?}", form_base_1.to_bytes());

    println!();

    // - FormString - //
    println!("-- FormString --");

    let form_string_1 = FormString::new(
        FormID::from("00002"),
        StrSml::from("StrTest2"),
        vec![LangCode::EN, LangCode::FR],
        vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")]
    );

    println!("FormString 1: \n{:?}", form_string_1.to_string());
    println!("FormString 1 byte count: {}", form_string_1.get_byte_count());

    println!("FormString 1 as bytes: {:?}", form_string_1.to_bytes());

    println!();

    // - FormWorld - //
    println!("-- FormWorld --");

    let form_world_1 = FormWorld::new(
        FormID::from("00002"),
        StrSml::from("WrldDesert"),
        StrSml::from("Desert of Derak"),
        vec![GlobalID::from("00100050"), GlobalID::from("00100051")]
    );

    println!("FormWorld 1: \n{:?}", form_world_1.to_string());
    println!("FormWorld 1 byte count: {}", form_world_1.get_byte_count());

    println!("FormWorld 1 as bytes: {:?}", form_world_1.to_bytes());

    println!();
}

