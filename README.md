[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking) [![crates.io](https://img.shields.io/crates/v/bevy_ss_anim)](https://crates.io/crates/bevy_ss_anim) [![docs.rs](https://docs.rs/bevy_ss_anim/badge.svg)](https://docs.rs/bevy_ss_anim)

# Bevy Sprite Sheet Animation
This Crate Currently Can Create An Animated Sprite From A Sprite Sheet Generated In Adobe Animate Using The Data Formats:

- Sparrow V1
- Sparrow V2
- Starling
- Json
- Json Array
- Edge Animate

## Disclaimer!
This Is In Early Development So Stuff Is Subject To Change

## How To Use This Crate
This Example Uses Sparrow, If Other Data Formats Are Supported You Can Of Course Also Use Them Similarly To This

```rust ignore
// bevy version: 0.11.2
use bevy::prelude::*;
use bevy_ss_anim;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (jump, bevy_ss_anim::update_animations))
        .run();
}

fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // spawn camera since nothing would get rendered without it
    commands.spawn(Camera2dBundle::default());

    // in assets/images/ you would have the player.png and player.xml files
    // path to png and xml, texture atlases, asset server
    let bundle = bevy_ss_anim::AnimatedSpriteBundle::from_sparrow("images/player", &mut texture_atlases, &asset_server);

    if let Some(mut bundle) = bundle {
        // animation name, animation prefix in xml, fps, looped, offset
        bundle.animated_sprite.add_animation_by_prefix("idle", "Idle", 24, true, Vec2::default());
        bundle.animated_sprite.add_animation_by_prefix("jump", "Jump", 24, false, Vec2::new(-5f32, 25f32));

        bundle.sprite_sheet_bundle.transform.scale = Vec3::new(0.5, 0.5, 0.5);

        // animation name, forced, texture atlas sprite, transform
        bundle.animated_sprite.play_animation("idle", true, &mut bundle.sprite_sheet_bundle.sprite, &mut bundle.sprite_sheet_bundle.transform);

        commands.spawn(bundle);
    }
}

fn jump(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut bevy_ss_anim::AnimatedSprite, &mut TextureAtlasSprite, &mut Transform)>,
) {
    for (mut animated_sprite, mut sprite, mut transform) in query.iter_mut() {
        if animated_sprite.animation_is_finished {
            animated_sprite.play_animation("idle", true, &mut sprite, &mut transform);
        }

        if input.just_pressed(KeyCode::Space) && animated_sprite.current_animation().name != "jump" {
            animated_sprite.play_animation("jump", true, &mut sprite, &mut transform);
        }
    }
}

```
