use bevy::asset::AssetServer;
use bevy::prelude::Assets;
use bevy::prelude::*;
use bevy::render::texture::Image;
use image::{ImageBuffer, Rgba};
use std::path::Path;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, update_color)
        .add_systems(Update, save_color_to_image)
        .run();
}

#[derive(Component)]
struct ColorPicker;

#[derive(Component)]
struct Palette;

fn setup_ui(mut commands: Commands) {
    // Root UI node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ..default()
        })
        .with_children(|parent| {
            // Color picker
            parent.spawn((
                ColorPicker,
                NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(1.0, 0.0, 0.0)), // Initial color
                    ..default()
                },
            ));

            // Color palette
            parent.spawn((
                Palette,
                NodeBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(100.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.0, 0.0, 0.0)), // Background color
                    ..default()
                },
            ));
        });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn update_color(mut query: Query<(&mut BackgroundColor, &ColorPicker)>, time: Res<Time>) {
    for (mut color, _) in &mut query.iter_mut() {
        // Cycle through colors over time
        let t = time.elapsed_seconds();
        *color = BackgroundColor(Color::srgb(
            t.sin().abs(),
            t.cos().abs(),
            1.0 - t.sin().abs(),
        ));
    }
}

fn save_color_to_image(
    query: Query<&BackgroundColor, With<ColorPicker>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Ok(background_color) = query.get_single() {
        let color = background_color.0.as_rgba_f32();
        let r = (color[0] * 255.0) as u8;
        let g = (color[1] * 255.0) as u8;
        let b = (color[2] * 255.0) as u8;
        let a = (color[3] * 255.0) as u8;

        let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_pixel(100, 100, Rgba([r, g, b, a]));
        let path = Path::new("assets/color_picker_output.png");
        img.save(path).unwrap();

        // Load the image into Bevy's asset system
        let handle = asset_server.load("assets/color_picker_output.png");
        let dynamic_image = image::DynamicImage::ImageRgba8(img);
        let bevy_image = Image::from_dynamic(dynamic_image, true, RenderAssetUsages::default());
        images.set(handle.clone(), bevy_image);
    }
}
