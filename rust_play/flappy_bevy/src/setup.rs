use bevy::prelude::*;

use crate::{
    components::*,
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("texture/background.png"),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.,
            },
            ..default()
        },
        Transform::from_xyz(0., 0., 0.),
        Background,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("texture/base.png"),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, 112.)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.,
            },
            ..default()
        },
        Transform::from_xyz(0., -250.0, 1.),
        Ground,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("texture/game-over.png"),
            ..default()
        },
        Visibility::Hidden,
        Transform::from_xyz(0., 0., 1.),
        GameOverText,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("texture/space.png"),
            ..default()
        },
        Transform::from_xyz(0., -50., 1.),
        PressSpaceBarText,
    ));

    let number_layout = TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);
    let number_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(number_layout);

    for i in 0..3 {
        // 24.0 px is the width of a single digit, 2.0 px is the space between digits
        let staring_point = -350.0 + (i as f32 * (24.0 + 2.));
        commands.spawn((
            Sprite::from_atlas_image(
                asset_server.load("texture/numbers.png"),
                TextureAtlas {
                    layout: number_texture_atlas_layout.clone(),
                    index: 0,
                },
            ),
            Transform::from_xyz(staring_point, 200., 1.),
            ScoreText,
        ));
    }

    let bird_layout = TextureAtlasLayout::from_grid(UVec2::new(34, 24), 3, 1, None, None);
    let bird_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(bird_layout);

    commands.spawn((
        Sprite::from_atlas_image(
            asset_server.load("texture/bird.png"),
            TextureAtlas {
                layout: bird_texture_atlas_layout.clone(),
                index: 0,
            },
        ),
        Transform::from_xyz(0., 0., 2.),
        ScoreText,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("texture/pipe.png"),
            ..default()
        },
        Transform::from_xyz(350., -200.0, 0.5),
        LowerPipe,
    ));

    let mut transform = Transform::from_xyz(350., 250., 0.5);
    transform.rotate(Quat::from_rotation_z(std::f32::consts::PI));

    commands.spawn((
        Sprite {
            image: asset_server.load("texture/pipe.png"),
            ..default()
        },
        transform,
        UpperPipe,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("texture/cloud.jpg"),
            custom_size: Some(Vec2::new(WINDOW_WIDTH / 8., WINDOW_HEIGHT / 8.)),
            image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
            ..default()
        },
        Visibility::Hidden,
        Transform::from_xyz(0., 0., 2.),
        CloudBlue,
    ));
}
