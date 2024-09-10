use crate::core::structs::*;

pub fn test_types() {
    test_types_id();   
    test_types_str();
    test_types_misc();
}

fn test_types_id() {
    println!("Testing ID Types...");

    let arch_id = ArchiveID::from(1);
    println!("{:?}", arch_id.to_string());
    println!("{:?}", arch_id.to_bytes());
    println!("{:?}", arch_id.to_u8());
    println!("ByteCount: {:?}", arch_id.get_byte_count());
    
    let form_id = FormID::from(245);
    println!("{:?}", form_id.to_string());
    println!("{:?}", form_id.to_bytes());
    println!("{:?}", form_id.to_u16());
    println!("ByteCount: {:?}", form_id.get_byte_count());

    let global_id = GlobalID::from((arch_id, form_id));
    println!("{:?}", global_id.to_string());
    println!("{:?}", global_id.to_bytes());
    println!("{:?}", global_id.get_byte_count());
}

fn test_types_str(){
    println!("Testing String Types...");

    let string_small = StrSml::from("Hello");
    println!("{:?}", string_small.to_string());
    println!("{:?}", string_small.to_bytes());

    let string_large = StrLrg::from("Hello, World!");
    println!("{:?}", string_large.to_string());
    println!("{:?}", string_large.to_bytes());
}

fn test_types_misc() {
    println!("Testing Misc Types...");

    let version = Version::from(1.3);
    println!("{:?}", version.to_string());
    println!("{:?}", version.to_bytes());

    let form_type_string = FormType::from(0);
    println!("{:?}", form_type_string.to_string());

    let form_type_world = FormType::from("WORLD");
    println!("{:?}", form_type_world.to_string());

    let form_type_refgroup = FormType::REFGROUP;
    println!("{:?}", form_type_refgroup.to_string());
}