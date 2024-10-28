// Used Form IDs:
// ArchiveID: 1
//
// FormRefGroup (World List):
//   CollWrldList: FormID 1
//
// FormStrings:
//   StrTitle: FormID 1000
//
// FormWorlds:
//   WrldBeach: FormID 2000
//   WrldForest: FormID 2001
//
// FormWorldParts:
//   WrldBeachPart1: FormID 3000
//   WrldBeachPart2: FormID 3001
//   WrldForestPart1: FormID 3002
//   WrldForestPart2: FormID 3003
//   WrldForestPart3: FormID 3004
//
// FormWeather:
//   WeatherRain: FormID 4000
//
// Entities (Models):
//   Beach Models: FormID 5000
//     Model IDs: 5001 - 5004
//   Forest Models: FormID 6000
//     Model IDs: 6001 - 6004
//


use std::{env, path::PathBuf};
use crate::core::{
    io::{read_form, write_archive_skeleton, write_form},
    structs::{
        Archive, ArchiveID, EntID, EntInstance, FormID, FormRefGroup, FormString, FormWeather,
        FormWorld, FormWorldPart, GlobalID, LangCode, SmlColor, StrLrg, StrSml, Vec3Float, Vec3Int,
        Version,
    },
};

pub fn test_sample() {
    // Set up the archive path
    let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
    let archive_path = current_dir.join("archives").join("test_sample.smn");
    let path = archive_path.to_str().unwrap();
    let archive_id = ArchiveID::from(1);

    // Create and write the archive skeleton
    let archive = Archive::new(
        archive_id,
        Version::from((1, 2)),
        StrLrg::from("Test Archive"),
    );
    let _ = write_archive_skeleton(&path, &archive);

    // Create and write a FormRefGroup (World List)
    let form_refgroup = FormRefGroup::new(
        FormID::from(1), // FormID for RefGroup
        StrSml::from("CollWrldList"),
        vec![
            GlobalID::from((archive_id, FormID::from(2000))), // WrldBeach
            GlobalID::from((archive_id, FormID::from(2001))), // WrldForest
        ],
    );
    let _ = write_form(&path, &form_refgroup);

    // Create and write a FormString (Title)
    let form_string = FormString::new(
        FormID::from(1000),
        StrSml::from("StrTitle"),
        vec![LangCode::EN, LangCode::FR],
        vec![
            StrLrg::from("Welcome to Amonal"),
            StrLrg::from("Bienvenue à Amonal"),
        ],
    );
    let _ = write_form(&path, &form_string);

    // Create and write FormWorld instances
    let worlds = vec![
        FormWorld::new(
            FormID::from(2000), // WrldBeach
            StrSml::from("WrldBeach"),
            GlobalID::from((archive_id, FormID::from(4000))), // WeatherRain
            StrSml::from("BeachOfAmonal"),
            vec![
                GlobalID::from((archive_id, FormID::from(3000))), // WrldBeachPart1
                GlobalID::from((archive_id, FormID::from(3001))), // WrldBeachPart2
            ],
            vec![Vec3Int::from((0, 0, 0)), Vec3Int::from((5000, 0, 0))],
        ),
        FormWorld::new(
            FormID::from(2001), // WrldForest
            StrSml::from("WrldForest"),
            GlobalID::from((archive_id, FormID::from(4000))), // WeatherRain
            StrSml::from("ForestOfAmonal"),
            vec![
                GlobalID::from((archive_id, FormID::from(3002))), // WrldForestPart1
                GlobalID::from((archive_id, FormID::from(3003))), // WrldForestPart2
                GlobalID::from((archive_id, FormID::from(3004))), // WrldForestPart3
            ],
            vec![
                Vec3Int::from((10000, 0, 0)),
                Vec3Int::from((15000, 0, 0)),
                Vec3Int::from((20000, 0, 0)),
            ],
        ),
    ];

    for world in worlds {
        let _ = write_form(&path, &world);
    }

    // Write FormWorldPart instances
    let world_parts = vec![
        FormWorldPart::new(
            FormID::from(3000), // WrldBeachPart1
            StrSml::from("WrldBeachPart1"),
            vec![
                EntInstance::from((
                    EntID::from((
                        GlobalID::from((archive_id, FormID::from(5000))), // Beach Models
                        FormID::from(5001), // Model ID
                    )),
                    Vec3Float::from((100.0, 50.0, 0.0)),
                    Vec3Float::from((0.0, 0.0, 45.0)), // Rotation in degrees
                    1.0,
                )),
                EntInstance::from((
                    EntID::from((
                        GlobalID::from((archive_id, FormID::from(5000))),
                        FormID::from(5002),
                    )),
                    Vec3Float::from((200.0, 75.0, 0.0)),
                    Vec3Float::from((0.0, 0.0, 90.0)),
                    1.0,
                )),
            ],
        ),
        FormWorldPart::new(
            FormID::from(3001), // WrldBeachPart2
            StrSml::from("WrldBeachPart2"),
            vec![
                EntInstance::from((
                    EntID::from((
                        GlobalID::from((archive_id, FormID::from(5000))),
                        FormID::from(5003),
                    )),
                    Vec3Float::from((300.0, 100.0, 0.0)),
                    Vec3Float::from((0.0, 0.0, 135.0)),
                    1.0,
                )),
                EntInstance::from((
                    EntID::from((
                        GlobalID::from((archive_id, FormID::from(5000))),
                        FormID::from(5004),
                    )),
                    Vec3Float::from((400.0, 125.0, 0.0)),
                    Vec3Float::from((0.0, 0.0, 180.0)),
                    1.0,
                )),
            ],
        ),
        FormWorldPart::new(
            FormID::from(3002), // WrldForestPart1
            StrSml::from("WrldForestPart1"),
            vec![
                EntInstance::from((
                    EntID::from((
                        GlobalID::from((archive_id, FormID::from(6000))), // Forest Models
                        FormID::from(6001),
                    )),
                    Vec3Float::from((10050.0, 75.0, 0.0)),
                    Vec3Float::from((0.0, 0.0, 0.0)),
                    1.0,
                )),
                EntInstance::from((
                    EntID::from((
                        GlobalID::from((archive_id, FormID::from(6000))),
                        FormID::from(6002),
                    )),
                    Vec3Float::from((10100.0, 100.0, 0.0)),
                    Vec3Float::from((0.0, 0.0, 15.0)),
                    1.0,
                )),
            ],
        ),
        FormWorldPart::new(
            FormID::from(3003), // WrldForestPart2
            StrSml::from("WrldForestPart2"),
            vec![
                EntInstance::from((
                    EntID::from((
                        GlobalID::from((archive_id, FormID::from(6000))),
                        FormID::from(6003),
                    )),
                    Vec3Float::from((15100.0, 50.0, 0.0)),
                    Vec3Float::from((0.0, 0.0, 30.0)),
                    1.0,
                )),
                EntInstance::from((
                    EntID::from((
                        GlobalID::from((archive_id, FormID::from(6000))),
                        FormID::from(6004),
                    )),
                    Vec3Float::from((15200.0, 75.0, 0.0)),
                    Vec3Float::from((0.0, 0.0, 45.0)),
                    1.0,
                )),
            ],
        ),
        FormWorldPart::new(
            FormID::from(3004), // WrldForestPart3
            StrSml::from("WrldForestPart3"),
            vec![
                EntInstance::from((
                    EntID::from((
                        GlobalID::from((archive_id, FormID::from(6000))),
                        FormID::from(6001), // Reusing model
                    )),
                    Vec3Float::from((20100.0, 50.0, 0.0)),
                    Vec3Float::from((0.0, 0.0, 60.0)),
                    1.0,
                )),
                EntInstance::from((
                    EntID::from((
                        GlobalID::from((archive_id, FormID::from(6000))),
                        FormID::from(6002),
                    )),
                    Vec3Float::from((20200.0, 100.0, 0.0)),
                    Vec3Float::from((0.0, 0.0, 75.0)),
                    1.0,
                )),
            ],
        ),
    ];

    for part in world_parts {
        let _ = write_form(&path, &part);
    }

    // Create and write a FormWeather with sound properties
    let weather_form = {
        let form_id = FormID::from(4000); // WeatherRain
        let form_name = StrSml::from("WeatherRain");

        // GI Lighting properties
        let gi_lighting_color = vec![
            SmlColor::from((200, 200, 200, 255)), // Day: Overcast gray
            SmlColor::from((180, 180, 200, 255)), // Dusk: Slightly bluish gray
            SmlColor::from((100, 100, 120, 255)), // Night: Dark gray
            SmlColor::from((190, 190, 210, 255)), // Dawn: Light gray
        ];
        let gi_lighting_intensity = vec![0.8, 0.6, 0.3, 0.7]; // Lower intensities due to overcast

        let gi_shadow_intensity = vec![0.2, 0.1, 0.0, 0.1]; // Less shadow under overcast skies

        // Precipitation properties
        let precipitation_preset = vec![
            GlobalID::from((archive_id, FormID::from(7000))), // Rain
            GlobalID::from((archive_id, FormID::from(7000))), // Rain
            GlobalID::from((archive_id, FormID::from(7001))), // Snow at night
            GlobalID::from((archive_id, FormID::from(7001))), // Snow at dawn
        ];
        let precipitation_intensity = vec![0.5, 0.7, 0.9, 0.6]; // Varying intensities

        // Wind properties
        let wind_speed = vec![5.2, 10.5, 15.0, 7.4]; // Speeds in m/s
        let wind_turbulence = vec![0.3, 0.5, 0.7, 0.4];
        let wind_direction = vec![
            Vec3Float::from((1.0, 0.0, 0.0)),  // East wind
            Vec3Float::from((0.7, 0.7, 0.0)),  // Southeast wind
            Vec3Float::from((0.0, 1.0, 0.0)),  // South wind
            Vec3Float::from((-0.7, 0.7, 0.0)), // Southwest wind
        ];

        // Skybox properties
        let skybox_texture = vec![
            StrSml::from("textures/env/skybox/overcast_day"),
            StrSml::from("textures/env/skybox/overcast_dusk"),
            StrSml::from("textures/env/skybox/overcast_night"),
            StrSml::from("textures/env/skybox/overcast_dawn"),
        ];
        let skybox_cloud_density = vec![0.9, 1.0, 1.0, 0.8]; // Heavy clouds
        let skybox_sun_color = gi_lighting_color.clone();
        let skybox_sun_intensity = vec![0.5, 0.3, 0.1, 0.4];

        // Fog properties
        let fog_density = vec![0.6, 0.7, 0.8, 0.5];
        let fog_height = vec![100.0, 80.0, 60.0, 90.0];
        let fog_scattering = vec![0.4, 0.5, 0.6, 0.4];
        let fog_color = vec![
            SmlColor::from((180, 180, 190, 255)), // Day
            SmlColor::from((170, 170, 180, 255)), // Dusk
            SmlColor::from((120, 120, 130, 255)), // Night
            SmlColor::from((175, 175, 185, 255)), // Dawn
        ];

        // Sound properties
        let sound_ambient_profile = vec![
            GlobalID::from((archive_id, FormID::from(8000))), // Day
            GlobalID::from((archive_id, FormID::from(8001))), // Dusk
            GlobalID::from((archive_id, FormID::from(8002))), // Night
            GlobalID::from((archive_id, FormID::from(8003))), // Dawn
        ];
        let sound_env_reverb = vec![0.6, 0.5, 0.7, 0.6];
        let sound_env_dampening = vec![0.3, 0.4, 0.5, 0.3];
        let sound_env_echo_delay = vec![0.2, 0.25, 0.3, 0.2];

        // Create the FormWeather instance
        FormWeather::new(
            form_id,
            form_name,
            gi_lighting_color,
            gi_lighting_intensity,
            gi_shadow_intensity,
            precipitation_preset,
            precipitation_intensity,
            wind_speed,
            wind_turbulence,
            wind_direction,
            skybox_texture,
            skybox_cloud_density,
            skybox_sun_color,
            skybox_sun_intensity,
            fog_density,
            fog_height,
            fog_scattering,
            fog_color,
            sound_ambient_profile,
            sound_env_reverb,
            sound_env_dampening,
            sound_env_echo_delay,
        )
    };
    let _ = write_form(&path, &weather_form);

    // Read and display the FormWeather
    match read_form(&path, FormID::from(4000)) {
        Ok(form) => println!("{:?}\n", form),
        Err(_) => println!("FormWeather with ID 4000 not found\n"),
    }

    // Read and display some forms
    let form_ids_to_read = vec![FormID::from(100), FormID::from(2000), FormID::from(2001)];
    for form_id in form_ids_to_read {
        match read_form(&path, form_id) {
            Ok(form) => println!("{:?}\n", form),
            Err(_) => println!("Form with ID {:?} not found\n", form_id),
        }
    }

    println!("------------------------");
}
