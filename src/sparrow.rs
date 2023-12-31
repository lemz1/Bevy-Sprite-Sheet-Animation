// Import necessary modules and crates
use bevy::prelude::*;

use serde::Deserialize;
use serde_xml_rs;

use crate::AnimatedSpriteBundle;
use crate::AnimatedSprite;
use crate::FrameOffset;

// Struct representing a subtexture within the XML data
#[derive(Debug, Deserialize, PartialEq)]
struct SubTexture {
    // Attributes of a subtexture
    name: String,

    x: u32,
    y: u32,
    #[serde(alias = "w")]
    width: u32,
    #[serde(alias = "h")]
    height: u32,

    #[serde(default, rename = "frameX")]
    frame_x: i32,
    #[serde(default, rename = "frameY")]
    frame_y: i32,
    // we dont need these
    // #[serde(default, rename = "frameWidth")]
    // frame_width: u32,
    // #[serde(default, rename = "frameHeight")]
    // frame_height: u32,
}

// Struct representing the entire XML data
#[derive(Debug, Default, Deserialize, PartialEq)]
struct Frames {
    #[serde(rename = "SubTexture")]
    subtextures: Vec<SubTexture>,
}

pub fn create_animated_sprite_bundle(
    path: &str,
    texture_atlases: &mut Assets<TextureAtlas>,
    asset_server: &AssetServer,
) -> Option<AnimatedSpriteBundle> {
    // Load XML content from file
    let content = std::fs::read_to_string(format!("assets/{path}.xml")).ok()?;

    // Remove the BOM if present (UTF-8 BOM is 0xEF, 0xBB, 0xBF)
    let content = content.trim_start_matches('\u{FEFF}').to_string();

    // Deserialize XML data
    let xml_data: Frames = serde_xml_rs::from_str(&content).ok()?;

    // Load texture atlas and prepare sprite sheet bundle
    let texture_atlas_handle = texture_atlases.add(
        TextureAtlas::new_empty(
            asset_server.load(format!("{path}.png")), 
            Vec2::default()
        )
    );

    let texture_atlas = texture_atlases.get_mut(&texture_atlas_handle)?;

    // Prepare animated sprite data
    let mut animated_sprite = AnimatedSprite::default();

    // Add subtextures to the texture atlas and animated sprite data
    for subtexture in xml_data.subtextures.iter() {
        // Add texture to atlas
        let index = texture_atlas.add_texture(
            Rect::new(
                subtexture.x as f32,
                subtexture.y as f32,
                (subtexture.x + subtexture.width) as f32,
                (subtexture.y + subtexture.height) as f32,
            )
        );
        
        // Insert texture index into frames and set frame offset
        animated_sprite.frames.insert(
            subtexture.name.clone(),
            index
        );

        animated_sprite.frame_offsets.insert(
            index,
            FrameOffset {
                position_offset: Vec2::new(
                    subtexture.frame_x as f32 * 0.5,
                    subtexture.frame_y as f32 * 0.5,
                ),
                rotation_offset: 0f32
            }
        );
    }

    return Some(
        AnimatedSpriteBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..default()
            },
            animated_sprite: animated_sprite,
        }
    );
}