// Import necessary modules and crates
use bevy::prelude::*;

use serde::Deserialize;
use serde_json;

use crate::AnimatedSpriteBundle;
use crate::AnimatedSprite;
use crate::FrameOffset;

#[derive(Debug, Default, Deserialize)]
struct Frame {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Debug, Default, Deserialize)]
struct SpriteSourceSize {
    x: u32,
    y: u32,
    // we dont need these
    // w: u32,
    // h: u32,
}

#[derive(Debug, Default, Deserialize)]
struct FrameData {
    frame: Frame,
    rotated: bool,
    #[serde(rename = "spriteSourceSize")]
    sprite_source_size: SpriteSourceSize,
}

#[derive(Debug, Default, Deserialize)]
struct Frames {
    // Each frame name will be a field in this struct
    // Use a BTreeMap to preserve the order of frames
    frames: std::collections::BTreeMap<String, FrameData>,
}

pub fn create_animated_sprite_bundle(
    path: &str,
    texture_atlases: &mut Assets<TextureAtlas>,
    asset_server: &AssetServer,
) -> Option<AnimatedSpriteBundle> {
    // Load Json content from file
    let content = std::fs::read_to_string(format!("assets/{path}.json")).ok()?;

    // Remove the BOM if present (UTF-8 BOM is 0xEF, 0xBB, 0xBF)
    let content = content.trim_start_matches('\u{FEFF}').to_string();

    // Deserialize Json data
    let json_data: Frames = serde_json::from_str(&content).ok()?;

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

    // Add frames to the texture atlas and animated sprite data
    for frame in json_data.frames.iter() {
        // Add texture to atlas
        let index = texture_atlas.add_texture(
            Rect::new(
                frame.1.frame.x as f32,
                frame.1.frame.y as f32,
                (frame.1.frame.x + frame.1.frame.w) as f32,
                (frame.1.frame.y + frame.1.frame.h) as f32,
            )
        );

        // Insert texture index into frames and set frame offset
        animated_sprite.frames.insert(
            frame.0.clone(),
            index
        );
        
        animated_sprite.frame_offsets.insert(
            index,
            FrameOffset {
                position_offset: Vec2::new(
                    frame.1.sprite_source_size.x as f32 * -0.5, // negative because for some reason
                    frame.1.sprite_source_size.y as f32 * -0.5, // the json has the inverted sign
                ),
                rotation_offset: if frame.1.rotated {std::f32::consts::PI * 0.5} else {0f32},
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