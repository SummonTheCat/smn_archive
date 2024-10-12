use crate::core::structs::*;

pub fn test_forms() {
    println!("------ TESTING FORMS ------");
    test_forms_string();
    test_forms_world();
    test_forms_refgroup();
}

fn test_forms_string() {
    println!("------ Testing FormString ------");

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
    println!("------ Testing FormWorld ------");

    let form_world = FormWorld::new(
        FormID::from(2), 
        StrSml::from("WrldBeach"),
        GlobalID::from((ArchiveID::from(1), FormID::from(200))),
        StrSml::from("MapBeach"),
        vec![
            GlobalID::from("00100543"), 
            GlobalID::from("00100544")
        ],
        vec![
            Vec3Int::from((1, 2, 0)),
            Vec3Int::from((1000, -300, 50)),
        ]
    );

    println!("{:?}", form_world.to_string());
    println!("{:?}", form_world.to_bytes());
    println!("ByteCount: {:?}", form_world.get_byte_count());
}

fn test_forms_refgroup() {
    println!("------ Testing FormRefGroup ------");

    let form_refgroup = FormRefGroup::new(
        FormID::from(3),
        StrSml::from("RefWorlds"),
        vec![GlobalID::from("00100002"), GlobalID::from("00100007")]
    );

    println!("{:?}", form_refgroup.to_string());
    println!("{:?}", form_refgroup.to_bytes());
    println!("ByteCount: {:?}", form_refgroup.get_byte_count());
}

