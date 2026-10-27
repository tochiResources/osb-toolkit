use osb_runtime::storyboarding3d::{
    bake_input, Bake3dInput, BakedCommand, BakedSprite, BakedStoryboard,
};
use serde::Serialize;

use crate::error_json;
use crate::helpers::{build_storyboard_scene, storyboard_json};

/// A serializable view of a baked sprite: the keyframe channels are flattened
/// into `.osb`-compatible command spans by `BakedSprite::commands`.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BakedSpriteDto<'a> {
    sprite_path: &'a str,
    origin: u8,
    additive: bool,
    depth: f64,
    z_depth: f64,
    layer: u8,
    commands: Vec<BakedCommand>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BakedStoryboardDto<'a> {
    sprites: Vec<BakedSpriteDto<'a>>,
}

pub(crate) fn baked_dto(baked: &BakedStoryboard) -> BakedStoryboardDto<'_> {
    BakedStoryboardDto {
        sprites: baked.sprites.iter().map(sprite_dto).collect(),
    }
}

fn sprite_dto(sprite: &BakedSprite) -> BakedSpriteDto<'_> {
    BakedSpriteDto {
        sprite_path: &sprite.sprite_path,
        origin: sprite.origin.osb_id(),
        additive: sprite.additive,
        depth: sprite.depth,
        z_depth: sprite.z_depth,
        layer: sprite.layer,
        commands: sprite.commands(),
    }
}

/// Bakes a `Bake3dInput` JSON string into the baked DTO JSON.
pub fn bake_to_json(input: &str) -> String {
    let input: Bake3dInput = match serde_json::from_str(input) {
        Ok(input) => input,
        Err(error) => return error_json(&error.to_string()),
    };
    let baked = bake_input(&input);
    serde_json::to_string(&baked_dto(&baked)).unwrap_or_else(|error| error_json(&error.to_string()))
}

/// Bakes a `{ worlds2d, worlds3d, options, options2d }` scene JSON string into the
/// wire `Storyboard` JSON (in-process, no runtime handle).
pub fn bake_scene_to_json(input: &str) -> String {
    let value: serde_json::Value = match serde_json::from_str(input) {
        Ok(value) => value,
        Err(error) => return error_json(&error.to_string()),
    };
    match build_storyboard_scene(&value) {
        Ok((scene, options2d, options3d)) => {
            let storyboard = scene.bake(&options2d, &options3d);
            storyboard_json(&storyboard).to_string()
        }
        Err(message) => error_json(&message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bakes_a_centered_sprite_to_the_storyboard_centre() {
        let json = bake_to_json(
            r#"{"nodes":[{"sprite":"sb/dot.png"}],
                "camera":{"position":[{"time":0,"value":[0,0,5]}]},
                "options":{"endTime":0}}"#,
        );
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["sprites"][0]["spritePath"], "sb/dot.png");
        assert_eq!(value["sprites"][0]["origin"], 4);
        assert!(value["sprites"][0]["commands"]
            .as_array()
            .unwrap()
            .iter()
            .any(|command| command["kind"] == "move"));
    }

    #[test]
    fn reports_an_error_for_invalid_input() {
        let json = bake_to_json("not json");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(value["error"].is_string());
    }
}
