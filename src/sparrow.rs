// Import necessary modules and crates
use std::collections::HashMap;

use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use serde_xml_rs;

// Struct representing a subtexture within the XML data
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SubTexture {
    // Attributes of a subtexture
    pub name: String,

    pub x: u32,
    pub y: u32,
    #[serde(alias = "w")]
    pub width: u32,
    #[serde(alias = "h")]
    pub height: u32,

    #[serde(default, rename = "frameX")]
    pub frame_x: i32,
    #[serde(default, rename = "frameY")]
    pub frame_y: i32,
    #[serde(default, rename = "frameWidth")]
    pub frame_width: u32,
    #[serde(default, rename = "frameHeight")]
    pub frame_height: u32,
}

// Struct representing the entire XML data
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct XMLData {
    #[serde(rename = "SubTexture")]
    pub subtextures: Vec<SubTexture>,
}

// Struct containing animation data
#[derive(Debug, Default, Clone)]
pub struct AnimationData {
    // Animation properties
    pub name: String,
    pub fps: u8,
    pub looped: bool,
    pub offset: Vec2,
    pub indices: Vec<usize>,
    pub current_index: usize,
    pub timer: Timer,
}

// Component representing an animated sprite
#[derive(Debug, Default, Component)]
pub struct AnimatedSprite {
    // Animation and sprite data
    pub animation_is_finished: bool,
    pub animation_is_paused: bool,
    animations: Vec<AnimationData>,
    frames: HashMap<String, usize>,
    offsets: Vec<Vec2>,
    current_animation_index: Option<usize>,
    xml_data: XMLData,
}

// Bundle for creating an AnimatedSprite
#[derive(Bundle)]
pub struct AnimatedSpriteBundle {
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub animated_sprite: AnimatedSprite,
}

impl AnimatedSpriteBundle {
    pub fn new(
        path: &str,
        texture_atlases: &mut Assets<TextureAtlas>,
        asset_server: &AssetServer,
    ) -> Option<Self> {
        // Load XML content from file
        let content = std::fs::read_to_string(format!("assets/{path}.xml")).ok()?;

        // Deserialize XML data
        let xml_data = serde_xml_rs::from_str(&content).ok()?;

        // Load texture atlas and prepare sprite sheet bundle
        let texture_atlas_handle = texture_atlases.add(
            TextureAtlas::new_empty(
                asset_server.load(format!("{path}.png")), 
                Vec2::default()
            )
        );

        let texture_atlas = texture_atlases.get_mut(&texture_atlas_handle)?;

        // Prepare animated sprite data
        let mut animated_sprite = AnimatedSprite {
            xml_data: xml_data,
            ..default()
        };

        // Add subtextures to the texture atlas and animated sprite data
        for subtexture in animated_sprite.xml_data.subtextures.iter() {
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
            animated_sprite.offsets.insert(
                index,
                Vec2::new(
                    subtexture.frame_x as f32 * 0.5,
                    subtexture.frame_y as f32 * 0.5,
                )
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
}

// Implementation of methods for the AnimatedSprite struct
impl AnimatedSprite {
    // Method to add an animation using specific frames
    pub fn add_animation_by_frames(
        &mut self,
        animation_name: &str,
        frames: Vec<String>,
        fps: u8,
        looped: bool,
        offset: Vec2,
    ) {
        if frames.len() == 0 {
            println!("\x1b[38;5;196mAnimation ({animation_name}) wasn't created because it had 0 frames\x1b[0;0;0m");
            return;
        }

        // Check if animation already exists with this name and remove it
        if let Some(index) = self.animations.iter().position(|animation| animation.name == animation_name) {
            self.animations.remove(index);
        }

        // Add the new animation
        self.animations.push(
            AnimationData {
                name: animation_name.to_string(),
                fps: fps,
                looped: looped,
                offset: offset,
                indices: frames.iter().filter_map(|frame| self.frames.get(frame)).copied().collect(),
                current_index: 0,
                timer: Timer::from_seconds(1f32 / (fps as f32), TimerMode::Once)
            }
        );
    }

    // Method to add an animation using frames with a specific prefix
    pub fn add_animation_by_prefix(
        &mut self,
        animation_name: &str,
        prefix: &str,
        fps: u8,
        looped: bool,
        offset: Vec2,
    ) {
        // Collect frames with the specified prefix and sort them
        let mut frames: Vec<String> = self.frames.keys().filter(|frame| frame.starts_with(prefix)).cloned().collect();
        frames.sort();
        // Add the animation using the collected frames
        self.add_animation_by_frames(
            animation_name,
            frames,
            fps,
            looped,
            offset,
        );
    }

    // Method to play a specific animation
    pub fn play_animation(
        &mut self,
        animation_name: &str,
        forced: bool,
        sprite: &mut TextureAtlasSprite,
        transform: &mut Transform,
    ) {
        // Search for the animation index
        let mut anim_index: Option<usize> = None;
        for (index, animation) in self.animations.iter().enumerate() {
            if animation.name == animation_name {
                anim_index = Some(index);
                break;
            }
        }

        // Check if animation exists, else return
        if anim_index.is_none() {
            println!("\x1b[38;5;196mAnimation ({animation_name}) doesn't exist\x1b[0;0;0m");
            return;
        }

        // Handle the current animation
        if let Some(current_animation_index) = self.current_animation_index {
            if let Some(current_animation) = self.animations.get_mut(current_animation_index) {
                if !forced && current_animation.name == animation_name {
                    return;
                }

                current_animation.timer.reset();

                current_animation.current_index = 0;

                // Remove frame offset
                transform.translation -= Vec3::new(
                    self.offsets[sprite.index].x,
                    self.offsets[sprite.index].y,
                    0f32,
                ) * transform.scale;

                // Remove animation offset
                transform.translation -= Vec3::new(
                    current_animation.offset.x,
                    current_animation.offset.y,
                    0f32,
                ) * transform.scale;
            }
        }

        // Reset animation status and set the new animation index
        self.animation_is_finished = false;
        self.animation_is_paused = false;
        self.current_animation_index = anim_index;

        let animation = &mut self.animations[self.current_animation_index.unwrap()];

        animation.current_index = 0;
        sprite.index = animation.indices[animation.current_index];

        // Set frame offset
        transform.translation += Vec3::new(
            self.offsets[sprite.index].x,
            self.offsets[sprite.index].y,
            0f32,
        ) * transform.scale;

        // Set animation offset
        transform.translation += Vec3::new(
            animation.offset.x,
            animation.offset.y,
            0f32,
        ) * transform.scale;
    }

    // Method to move to the next frame of the current animation
    pub fn next_frame(
        &mut self,
        sprite: &mut TextureAtlasSprite,
        transform: &mut Transform,
    ) {
        let animation = &mut self.animations[self.current_animation_index.unwrap()];

        animation.timer.reset();

        if animation.current_index >= animation.indices.len() - 1 {
            if !animation.looped {
                self.animation_is_finished = true;
                return;
            }

            // Remove offset
            transform.translation -= Vec3::new(
                self.offsets[sprite.index].x,
                self.offsets[sprite.index].y,
                0f32,
            ) * transform.scale;

            // Loop to the first frame
            animation.current_index = 0;
            sprite.index = animation.indices[animation.current_index];

            // Set offset
            transform.translation += Vec3::new(
                self.offsets[sprite.index].x,
                self.offsets[sprite.index].y,
                0f32,
            ) * transform.scale;
        } else {
            // Remove offset
            transform.translation -= Vec3::new(
                self.offsets[sprite.index].x,
                self.offsets[sprite.index].y,
                0f32,
            ) * transform.scale;

            // Move to the next frame
            animation.current_index += 1;
            sprite.index = animation.indices[animation.current_index];

            // Set offset
            transform.translation += Vec3::new(
                self.offsets[sprite.index].x,
                self.offsets[sprite.index].y,
                0f32,
            ) * transform.scale;
        }
    }

    pub fn update_frame(
        &mut self,
        mut sprite: &mut TextureAtlasSprite, 
        mut transform: &mut Transform,
        time: &Time,
    ) {
        // Check if animation is finished or paused, if yes, skip
        if self.animation_is_finished || self.animation_is_paused {
            return;
        }

        if let Some(index) = self.current_animation_index {
            let animation = &mut self.animations[index];
            animation.timer.tick(time.delta());

            if animation.timer.just_finished() {
                self.next_frame(&mut sprite, &mut transform);
            }
        }
    }

    // Method to pause the current animation
    pub fn pause(
        &mut self
    ) {
        self.animation_is_paused = true;
    }

    // Method to resume the current animation
    pub fn resume(
        &mut self
    ) {
        self.animation_is_paused = false;
    }

    pub fn current_animation(
        &self
    ) -> AnimationData {
        if let Some(index) = self.current_animation_index {
            return self.animations[index].clone();
        } else {
            println!("No Animation Playing");
            return AnimationData::default();
        }
    }
}

// System to update animations
pub fn update_animations(
    mut query: Query<(&mut AnimatedSprite, &mut TextureAtlasSprite, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut animated_sprite, mut sprite, mut transform) in query.iter_mut() {
        animated_sprite.update_frame(&mut sprite, &mut transform, &time);
    }
}
