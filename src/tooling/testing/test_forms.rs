use crate::core::structs::*;

pub fn test_forms() {
    test_forms_string();
    test_forms_world();
    test_forms_refgroup();
}

fn test_forms_string() {
    let form_string = FormString::new(
        FormID::from(1), 
        StrSml::from("Welcome"), 
        vec![LangCode::EN, LangCode::FR], 
        vec![StrLrg::from("Hello, World!"), StrLrg::from("Bonjour, Monde!")], 
    );

    println!("{:?}", form_string.to_string());
    println!("{:?}", form_string.to_bytes());
    println!("ByteCount: {:?}", form_string.get_byte_count());
}

fn test_forms_world() {
    let form_world = FormWorld::new(
        FormID::from(2), 
        StrSml::from("WrldBeach"),
        StrSml::from("The Beach"),
        vec![GlobalID::from("00100543"), GlobalID::from("00100544")]
    );

    println!("{:?}", form_world.to_string());
    println!("{:?}", form_world.to_bytes());
    println!("ByteCount: {:?}", form_world.get_byte_count());
}

fn test_forms_refgroup() {
    let form_refgroup = FormRefGroup::new(
        FormID::from(3),
        StrSml::from("RefWorlds"),
        vec![GlobalID::from("00100002"), GlobalID::from("00100007")]
    );

    println!("{:?}", form_refgroup.to_string());
    println!("{:?}", form_refgroup.to_bytes());
    println!("ByteCount: {:?}", form_refgroup.get_byte_count());
}