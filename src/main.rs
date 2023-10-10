use bevy::{prelude::*, winit::WinitSettings};
// use bevy::ui::{WidgetBundle, NodeComponents};
extern crate serde;
extern crate serde_hjson;
use serde_json::Value;
use serde::Deserialize;
// use serde_hjson::Value as HJsonValue;
use std::{fs, collections::HashMap};

// struct Keyboard;

fn main() {
    let caplabels = CapLabels::new();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, draw_keyboard)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
        // OrthographicCameraBundle::new_2d());
}

struct QmkInfo {
    qmk_path: String,
}

// Get qmk path from system environment variable
impl Default for QmkInfo {
    fn default() -> Self {
        let qmk_path = std::env::var("QMK_PATH").unwrap_or_else(|_| {
            println!("QMK_PATH not set. Defaulting to ~/code/qmk_firmware");
            String::from("/storage/users/anders/code/qmk_firmware")
        });
        Self { qmk_path }
    }
}

struct KeyboardInfo {
    keys: Vec<String>,
    key_positions: Vec<Vec<f64>>,
    name: String,
    layout: String,
}

impl KeyboardInfo {
    fn new() -> Self {
        let (keys, name, layout) = read_keymap();
        let positions = read_key_positions(name.clone(), layout.clone());
        Self {
            keys: keys.iter().map(|x| x.as_str().unwrap().to_string()).collect(),
            key_positions: positions
                .iter()
                .map(|x| vec![x["x"].as_f64().unwrap(), x["y"].as_f64().unwrap()])
                .collect(),
            name: name,
            layout: layout,
        }
    }
}

#[derive(Debug, Deserialize)]
struct KeyMapping {
    key: String,
    label: String,
    #[serde(default)]
    aliases: Vec<String>,
}

struct CapLabels {
    labels: Vec<String>,
    labels_extra: Vec<String>,
}

impl CapLabels {
    fn new() -> Self {
        let qmk_path = QmkInfo::default().qmk_path;
        let mut labelmap: HashMap<String, String>= HashMap::new();
        // let (keys, positions, name, layout) = read_files();
        let label_file = format!(
            "{}/data/constants/keycodes/keycodes_0.0.1_basic.hjson",
            qmk_path 
        );
        let label_file_string = fs::read_to_string(label_file).unwrap();
        println!("Label File String: {}", label_file_string);
        let label_file_extra = format!(
            "{}/data/constants/keycodes/extra/keycodes_us_0.0.1.hjson",
            qmk_path,
        );
        // println!("Label File: {}", label_file);
        // let labels: HashMap<String, KeyMapping> = serde_hjson::from_str(&label_file_string).expect("Failed to parse");
        // let labels_extra: HJsonValue = serde_hjson::from_str(&fs::read_to_string(label_file_extra).unwrap()).unwrap();
        Self {
            labels: vec![],
            labels_extra: vec![]
            // labels_extra: serde_hjson::from_str(&fs::read_to_string(label_file_extra).unwrap())
            //     .unwrap(),
        }
    }
}

fn read_keymap() -> (Vec<Value>, String, String) {
    // Load the JSON files
    // let qmk_path = QmkInfo::default().qmk_path;
    // Get path to keymap.json from argv or default to default keymap
    let keymap_path = std::env::args().nth(1).unwrap_or_else(|| {
        println!("No keymap specified. Quitting");
        format!("keymap.json")
        // Quit application if no keymap is specified
        // std::process::exit(1);
    });


    let keymap_data: Value = serde_json::from_str(&fs::read_to_string(keymap_path).unwrap()).unwrap();

    // Interpret the Data
    // For simplicity, let's assume a basic 1-dimensional array of keys in the keymap
    let keys = keymap_data["layers"][0].as_array().unwrap();

    // Get keyboard name and layout from keymap_data
    let keyboard_name = keymap_data["keyboard"].as_str().unwrap();
    let keyboard_layout = keymap_data["layout"].as_str().unwrap();

    // info.json path:
    // let info_path = format!("{}/keyboards/{}/info.json", qmk_path, keyboard_name);
    // let info_data: Value = serde_json::from_str(&fs::read_to_string(info_path).unwrap()).unwrap();
    // // Get key positions from info_data 
    // // Under "layouts" -> "<keyboard_layout>" -> "layout"
    // let key_positions = info_data["layouts"][keyboard_layout]["layout"].as_array().unwrap();

    (keys.clone(), keyboard_name.to_string(), keyboard_layout.to_string())

}

fn read_key_positions(name: String, layout: String) -> Vec<Value> {
    let qmk_path = QmkInfo::default().qmk_path;
    // info.json path:
    let info_path = format!("{}/keyboards/{}/info.json", qmk_path, name);
    let info_data: Value = serde_json::from_str(&fs::read_to_string(info_path).unwrap()).unwrap();
    // Get key positions from info_data 
    // Under "layouts" -> "<keyboard_layout>" -> "layout"
    let positions = info_data["layouts"][layout]["layout"].as_array().unwrap();
    positions.clone()
}

fn draw_keyboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let (keys, key_positions) = read_files();
    let pos_scale = 60.0;
    let keyboard_info = KeyboardInfo::new();

    println!("Keys: {:?}", keyboard_info.keys);

    for (key, key_position) in keyboard_info.keys.iter().zip(keyboard_info.key_positions.iter()) {

    // for key in keys {
        // let key_text = key.as_str().unwrap();
        let key_text = key;
        // let xpos = key_position["x"].as_f64().unwrap();
        let xpos = key_position[0];
        let ypos = key_position[1];

        // Create a UI button for each key
        commands.spawn(ButtonBundle {
            style: Style {
                // size: Size::new(Val::Px(60.0), Val::Px(60.0)),
                width: Val::Px(60.0),
                height: Val::Px(60.0),
                position_type: PositionType::Absolute,
                left: Val::Px(xpos as f32 * pos_scale),
                top: Val::Px(ypos as f32 * pos_scale),
                align_items: AlignItems::Center,
                border: UiRect::all(
                    Val::Px(5.0)),
                ..Default::default()
            },
            border_color: BorderColor(Color::rgb(0.9, 0.5, 0.5)),
            background_color: BackgroundColor(Color::rgb(0.9, 0.9, 0.9)),

            
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                key_text.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/DejaVuSansMono-Bold.ttf"),
                    font_size: 15.0,
                    color: Color::BLACK,

                },
            ));
        });
    }
}
