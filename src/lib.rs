// Import necessary modules and crates
use std::collections::HashMap;

use bevy::prelude::*;

mod sparrow;
mod json;
mod json_array;

/// Struct containing animation data
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

#[derive(Debug, Default)]
struct FrameOffset {
    position_offset: Vec2,
    rotation_offset: f32,
}

/// Component representing an animated sprite
#[derive(Debug, Default, Component)]
pub struct AnimatedSprite {
    // Animation and sprite data
    pub animation_is_finished: bool,
    pub animation_is_paused: bool,
    animations: Vec<AnimationData>,
    frames: HashMap<String, usize>,
    frame_offsets: Vec<FrameOffset>,
    current_animation_index: Option<usize>,
}

/// Bundle for creating an AnimatedSprite
#[derive(Bundle)]
pub struct AnimatedSpriteBundle {
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub animated_sprite: AnimatedSprite,
}

impl AnimatedSpriteBundle {
    /// Creates an `AnimatedSpriteBundle` from a Sparrow v1 or Sparrow v2 data format.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the sprite sheet and data file.
    /// - `texture_atlases`: A mutable reference to the `Assets<TextureAtlas>` resource.
    /// - `asset_server`: A reference to the `AssetServer`.
    ///
    /// # Returns
    ///
    /// An `Option<Self>` containing the animated sprite bundle if successful, or `None` if an error occurs.
    pub fn from_sparrow(
        path: &str,
        texture_atlases: &mut Assets<TextureAtlas>,
        asset_server: &AssetServer,
    ) -> Option<Self> {
        return sparrow::create_animated_sprite_bundle(path, texture_atlases, asset_server);
    }

    /// Creates an `AnimatedSpriteBundle` from a Starling data format.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the sprite sheet and data file.
    /// - `texture_atlases`: A mutable reference to the `Assets<TextureAtlas>` resource.
    /// - `asset_server`: A reference to the `AssetServer`.
    ///
    /// # Returns
    ///
    /// An `Option<Self>` containing the animated sprite bundle if successful, or `None` if an error occurs.
    pub fn from_starling(
        path: &str,
        texture_atlases: &mut Assets<TextureAtlas>,
        asset_server: &AssetServer,
    ) -> Option<Self> {
        return sparrow::create_animated_sprite_bundle(path, texture_atlases, asset_server);
    }

    /// Creates an `AnimatedSpriteBundle` from a JSON data format.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the sprite sheet and data file.
    /// - `texture_atlases`: A mutable reference to the `Assets<TextureAtlas>` resource.
    /// - `asset_server`: A reference to the `AssetServer`.
    ///
    /// # Returns
    ///
    /// An `Option<Self>` containing the animated sprite bundle if successful, or `None` if an error occurs.
    pub fn from_json(
        path: &str,
        texture_atlases: &mut Assets<TextureAtlas>,
        asset_server: &AssetServer,
    ) -> Option<Self> {
        return json::create_animated_sprite_bundle(path, texture_atlases, asset_server);
    }

    /// Creates an `AnimatedSpriteBundle` from a JSON Array data format.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the sprite sheet and data file.
    /// - `texture_atlases`: A mutable reference to the `Assets<TextureAtlas>` resource.
    /// - `asset_server`: A reference to the `AssetServer`.
    ///
    /// # Returns
    ///
    /// An `Option<Self>` containing the animated sprite bundle if successful, or `None` if an error occurs.
    pub fn from_json_array(
        path: &str,
        texture_atlases: &mut Assets<TextureAtlas>,
        asset_server: &AssetServer,
    ) -> Option<Self> {
        return json_array::create_animated_sprite_bundle(path, false, texture_atlases, asset_server);
    }

    /// Creates an `AnimatedSpriteBundle` from an Edge Animate data format.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the sprite sheet and data file.
    /// - `texture_atlases`: A mutable reference to the `Assets<TextureAtlas>` resource.
    /// - `asset_server`: A reference to the `AssetServer`.
    ///
    /// # Returns
    ///
    /// An `Option<Self>` containing the animated sprite bundle if successful, or `None` if an error occurs.
    pub fn from_edge_animate(
        path: &str,
        texture_atlases: &mut Assets<TextureAtlas>,
        asset_server: &AssetServer,
    ) -> Option<Self> {
        return json_array::create_animated_sprite_bundle(path, true, texture_atlases, asset_server);
    }
}

// Implementation of methods for the AnimatedSprite struct
impl AnimatedSprite {
    /// Adds a new animation using specific frames.
    ///
    /// This method adds an animation to the `AnimatedSprite` using the provided frames,
    /// frames-per-second (fps), looped status, and offset.
    ///
    /// # Parameters
    ///
    /// - `animation_name`: Name of the animation to be added.
    /// - `frames`: Vector of frame names that compose the animation.
    /// - `fps`: Frames per second of the animation.
    /// - `looped`: Indicates whether the animation should loop.
    /// - `offset`: Offset applied to the animation.
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

    /// Adds a new animation using frames with a specific prefix.
    ///
    /// This method collects frames with the specified prefix, sorts them, and then
    /// adds the animation to the `AnimatedSprite`.
    ///
    /// # Parameters
    ///
    /// - `animation_name`: Name of the animation to be added.
    /// - `prefix`: Prefix used to identify frames for the animation.
    /// - `fps`: Frames per second of the animation.
    /// - `looped`: Indicates whether the animation should loop.
    /// - `offset`: Offset applied to the animation.
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

    /// Plays a specific animation on the `AnimatedSprite`.
    ///
    /// This method searches for the animation by name and plays it on the provided sprite
    /// and transform.
    ///
    /// # Parameters
    ///
    /// - `animation_name`: Name of the animation to be played.
    /// - `forced`: Forces the animation to play even if it's the current animation.
    /// - `sprite`: Reference to the sprite to which the animation is applied.
    /// - `transform`: Reference to the transform of the sprite.
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
                    self.frame_offsets[sprite.index].position_offset.x,
                    self.frame_offsets[sprite.index].position_offset.y,
                    0f32,
                ) * transform.scale;

                // Remove frame rotation
                transform.rotate_local_z(-self.frame_offsets[sprite.index].rotation_offset as f32);

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
            self.frame_offsets[sprite.index].position_offset.x,
            self.frame_offsets[sprite.index].position_offset.y,
            0f32,
        ) * transform.scale;

        // Set frame rotation
        transform.rotate_local_z(self.frame_offsets[sprite.index].rotation_offset as f32);

        // Set animation offset
        transform.translation += Vec3::new(
            animation.offset.x,
            animation.offset.y,
            0f32,
        ) * transform.scale;
    }

    /// Pauses the current animation.
    ///
    /// This method pauses the currently playing animation.
    pub fn pause(
        &mut self
    ) {
        self.animation_is_paused = true;
    }
    
    /// Resumes the current animation.
    ///
    /// This method resumes a paused animation.
    pub fn resume(
        &mut self
    ) {
        self.animation_is_paused = false;
    }
    
    /// Retrieves information about the current animation.
    ///
    /// This method returns an instance of `AnimationData` containing information
    /// about the currently playing animation.
    ///
    /// # Returns
    ///
    /// An `AnimationData` object representing the current animation.
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

    /// Moves to the next frame of the current animation.
    ///
    /// This method advances the animation to the next frame and updates the sprite and transform.
    ///
    /// # Parameters
    ///
    /// - `sprite`: Reference to the sprite being animated.
    /// - `transform`: Reference to the transform of the sprite.
    fn next_frame(
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
    
            // Remove frame offset
            transform.translation -= Vec3::new(
                self.frame_offsets[sprite.index].position_offset.x,
                self.frame_offsets[sprite.index].position_offset.y,
                0f32,
            ) * transform.scale;

            // Remove frame rotation
            transform.rotate_local_z(-self.frame_offsets[sprite.index].rotation_offset as f32);
    
            // Loop to the first frame
            animation.current_index = 0;
            sprite.index = animation.indices[animation.current_index];
    
            // Set frame offset
            transform.translation += Vec3::new(
                self.frame_offsets[sprite.index].position_offset.x,
                self.frame_offsets[sprite.index].position_offset.y,
                0f32,
            ) * transform.scale;

            // Set frame rotation
            transform.rotate_local_z(self.frame_offsets[sprite.index].rotation_offset as f32);
        } else {
            // Remove frame offset
            transform.translation -= Vec3::new(
                self.frame_offsets[sprite.index].position_offset.x,
                self.frame_offsets[sprite.index].position_offset.y,
                0f32,
            ) * transform.scale;

            // Remove frame rotation
            transform.rotate_local_z(-self.frame_offsets[sprite.index].rotation_offset as f32);
    
            // Move to the next frame
            animation.current_index += 1;
            sprite.index = animation.indices[animation.current_index];
    
            // Set frame offset
            transform.translation += Vec3::new(
                self.frame_offsets[sprite.index].position_offset.x,
                self.frame_offsets[sprite.index].position_offset.y,
                0f32,
            ) * transform.scale;

            // Set frame rotation
            transform.rotate_local_z(self.frame_offsets[sprite.index].rotation_offset as f32);
        }
    }
    
    /// Updates the frame of the current animation.
    ///
    /// This method updates the animation frame based on the elapsed time and advances
    /// to the next frame if necessary.
    ///
    /// # Parameters
    ///
    /// - `sprite`: Reference to the sprite being animated.
    /// - `transform`: Reference to the transform of the sprite.
    /// - `time`: Reference to the time information for timing the animation.
    fn update_frame(
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
