use crate::archive_tools::structs::{ArchiveID, FormID, GlobalID, LangCode, StrLrg, StrSml, Version, FormType};

#[allow(unused)]
pub fn test_types() {
    println!("--- Testing Types ---");
    
    // - ArchiveID - //
    println!("-- ArchiveID --");

    let archive_id_1 = ArchiveID::from(1);
    let archive_id_2 = ArchiveID::from("002");
    let archive_id_3 = ArchiveID::from([0x03]);

    println!("ArchiveID 1: {:?}", archive_id_1.to_string());
    println!("ArchiveID 2: {:?}", archive_id_2.to_string());
    println!("ArchiveID 3: {:?}", archive_id_3.to_string());

    println!("ArchiveID 1 as u8: {}", archive_id_1.to_u8());
    println!("ArchiveID 2 as u8: {}", archive_id_2.to_u8());
    println!("ArchiveID 3 as u8: {}", archive_id_3.to_u8());

    println!("ArchiveID 1 as bytes: {:?}", archive_id_1.to_bytes());
    println!("ArchiveID 2 as bytes: {:?}", archive_id_2.to_bytes());
    println!("ArchiveID 3 as bytes: {:?}", archive_id_3.to_bytes());

    println!("ArchiveID 1 == ArchiveID 2: {}", archive_id_1 == archive_id_2);
    println!("ArchiveID 1 < ArchiveID 2: {}", archive_id_1 < archive_id_2);
    println!("ArchiveID 1 > ArchiveID 2: {}", archive_id_1 > archive_id_2);
    println!("ArchiveID 1 == ArchiveID 1: {}", archive_id_1 == archive_id_1);

    println!("ID Byte Count: {}", ArchiveID::get_byte_count(&archive_id_1));

    println!();

    // - FormID - //
    println!("-- FormID --");

    let form_id_1: FormID = FormID::from(1);
    let form_id_2: FormID = FormID::from("00002");
    let form_id_3: FormID = FormID::from([0x00, 0x03]);

    println!("FormID 1: {:?}", form_id_1.to_string());
    println!("FormID 2: {:?}", form_id_2.to_string());
    println!("FormID 3: {:?}", form_id_3.to_string());

    println!("FormID 1 as u16: {}", form_id_1.to_u16());
    println!("FormID 2 as u16: {}", form_id_2.to_u16());
    println!("FormID 3 as u16: {}", form_id_3.to_u16());

    println!("FormID 1 as bytes: {:?}", form_id_1.to_bytes());
    println!("FormID 2 as bytes: {:?}", form_id_2.to_bytes());
    println!("FormID 3 as bytes: {:?}", form_id_3.to_bytes());

    println!("FormID 1 == FormID 2: {}", form_id_1 == form_id_2);
    println!("FormID 1 < FormID 2: {}", form_id_1 < form_id_2);
    println!("FormID 1 > FormID 2: {}", form_id_1 > form_id_2);
    println!("FormID 1 == FormID 1: {}", form_id_1 == form_id_1);
    
    println!("ID Byte Count: {}", FormID::get_byte_count(&form_id_1));

    println!();

    // - GlobalID - //
    println!("-- GlobalID --");

    let global_id_1 = GlobalID::from((ArchiveID::from(1), FormID::from(1)));
    let global_id_2 = GlobalID::from("00100002");
    let global_id_3 = GlobalID::from([0x01, 0x02, 0x03]);

    println!("GlobalID 1: {:?}", global_id_1.to_string());
    println!("GlobalID 2: {:?}", global_id_2.to_string());
    println!("GlobalID 3: {:?}", global_id_3.to_string());

    println!("GlobalID 1 as bytes: {:?}", global_id_1.to_bytes());
    println!("GlobalID 2 as bytes: {:?}", global_id_2.to_bytes());
    println!("GlobalID 3 as bytes: {:?}", global_id_3.to_bytes());

    println!("GlobalID 1 == GlobalID 2: {}", global_id_1 == global_id_2);
    println!("GlobalID 1 < GlobalID 2: {}", global_id_1 < global_id_2);
    println!("GlobalID 1 > GlobalID 2: {}", global_id_1 > global_id_2);
    println!("GlobalID 1 == GlobalID 1: {}", global_id_1 == global_id_1);

    println!("Isolated ArchiveID: {:?}", ArchiveID::from(&global_id_1).to_string());
    println!("Isolated FormID: {:?}", FormID::from(&global_id_1).to_string());

    println!("ID Byte Count: {}", GlobalID::get_byte_count(&global_id_1));

    println!();
    
    // - StrSml - //
    println!("-- StrSml --");

    let str_sml_1 = StrSml::from("汉语试");

    println!("StrSml 1: {:?}", str_sml_1);
    println!("StrSml 1 as string: {:?}", str_sml_1.to_string());
    println!("StrSml 1 as bytes: {:?}", str_sml_1.to_bytes());

    println!("StrSml 1 byte count: {}", str_sml_1.get_byte_count());
    
    println!();

    // - StrLrg - //
    println!("-- StrLrg --");

    let str_lrg_1 = StrLrg::from("école");

    println!("StrLrg 1: {:?}", str_lrg_1);
    println!("StrLrg 1 as string: {:?}", str_lrg_1.to_string());
    println!("StrLrg 1 as bytes: {:?}", str_lrg_1.to_bytes());

    println!("StrLrg 1 byte count: {}", str_lrg_1.get_byte_count());

    println!();

    // - Version - //
    println!("-- Version --");

    let version_1 = Version::from(1.01);
    let version_2 = Version::from([0x01, 0x03]);

    println!("Version 1: {:?}", version_1.to_string());
    println!("Version 3: {:?}", version_2.to_string());

    println!("Version 1 as f32: {}", version_1.to_f32());
    println!("Version 3 as f32: {}", version_2.to_f32());

    println!("Version 1 as bytes: {:?}", version_1.to_bytes());
    println!("Version 3 as bytes: {:?}", version_2.to_bytes());

    println!("Version Byte Count: {}", Version::get_byte_count(&version_1));

    println!();

    // - LangCode - //
    println!("-- LangCode --");

    let lang_code_1 = LangCode::EN;
    let lang_code_2 = LangCode::from("FR");

    println!("LangCode 1: {:?}", lang_code_1.to_string());
    println!("LangCode 2: {:?}", lang_code_2.to_string());

    println!("LangCode 1 as u8: {}", lang_code_1.to_int());
    println!("LangCode 2 as u8: {}", lang_code_2.to_int());

    println!("LangCode 1 as bytes: {:?}", lang_code_1.to_byte());
    println!("LangCode 2 as bytes: {:?}", lang_code_2.to_byte());

    println!("LangCode Byte Count: {}", LangCode::BYTE_COUNT);

    println!();

    // - FormType - //
    println!("-- FormType --");

    let form_type_1 = FormType::from("STRING");
    let form_type_2 = FormType::WORLD;

    println!("FormType 1: {:?}", form_type_1.to_string());
    println!("FormType 2: {:?}", form_type_2.to_string());

    println!("FormType 1 as u32: {}", form_type_1.to_u8());
    println!("FormType 2 as u32: {}", form_type_2.to_u8());

    println!("FormType 1 as bytes: {:?}", form_type_1.to_byte());
    println!("FormType 2 as bytes: {:?}", form_type_2.to_byte());

    println!("FormType Byte Count: {}", FormType::get_byte_count(&form_type_1));

    println!();
}