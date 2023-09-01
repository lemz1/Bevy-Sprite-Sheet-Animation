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
    filename: String,
    frame: Frame,
    rotated: bool,
    #[serde(rename = "spriteSourceSize")]
    sprite_source_size: SpriteSourceSize,
}

#[derive(Debug, Default, Deserialize)]
struct Frames {
    frames: Vec<FrameData>,
}

pub fn create_animated_sprite_bundle(
    path: &str,
    is_edge_animate: bool,
    texture_atlases: &mut Assets<TextureAtlas>,
    asset_server: &AssetServer,
) -> Option<AnimatedSpriteBundle> {
    // Load Json content from file
    let content = std::fs::read_to_string(format!("assets/{path}{}", if is_edge_animate {".eas"} else {".json"})).ok()?;

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
                frame.frame.x as f32,
                frame.frame.y as f32,
                (frame.frame.x + frame.frame.w) as f32,
                (frame.frame.y + frame.frame.h) as f32,
            )
        );

        // Insert texture index into frames and set frame offset
        animated_sprite.frames.insert(
            frame.filename.clone(),
            index
        );

        animated_sprite.frame_offsets.insert(
            index,
            FrameOffset {
                position_offset: Vec2::new(
                    frame.sprite_source_size.x as f32 * -0.5, // negative because for some reason
                    frame.sprite_source_size.y as f32 * -0.5, // the json has the inverted sign
                ),
                rotation_offset: if frame.rotated {std::f32::consts::PI * 0.5} else {0f32},
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