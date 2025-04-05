use bevy::{
    prelude::*,
    text::{FontStyle, LineBreak, TextBounds},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// This example demonstrates various Unicode bidirectional text scenarios in Bevy
// It shows different text alignment and directionality combinations
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera setup
    commands.spawn(Camera2d::default());

    // Load font that supports both LTR and RTL scripts
    let font = asset_server.load("fonts/NotoSans-Regular.ttf");

    // Define common text styles
    let base_style = TextStyle {
        font: font.clone(),
        font_size: 24.0,
        color: Color::WHITE,
    };

    let heading_style = TextStyle {
        font: font.clone(),
        font_size: 32.0,
        color: Color::GOLD,
    };

    // Example 1: English (LTR) text
    spawn_text_example(
        &mut commands,
        "1. Left-to-Right Text (English)",
        "This is a simple left-to-right text example in English.",
        TextAlignment::Left,
        Vec2::new(-400.0, 300.0),
        &heading_style,
        &base_style,
    );

    // Example 2: Arabic (RTL) text
    spawn_text_example(
        &mut commands,
        "2. Right-to-Left Text (Arabic)",
        "هذا مثال على النص العربي من اليمين إلى اليسار.",
        TextAlignment::Right,
        Vec2::new(-400.0, 200.0),
        &heading_style,
        &base_style,
    );

    // Example 3: Mixed LTR and RTL text
    spawn_text_example(
        &mut commands,
        "3. Mixed Direction Text (English and Arabic)",
        "This is English text with العربية in the middle and then back to English.",
        TextAlignment::Left,
        Vec2::new(-400.0, 100.0),
        &heading_style,
        &base_style,
    );

    // Example 4: Mixed RTL and LTR with numbers
    spawn_text_example(
        &mut commands,
        "4. RTL with Numbers and Latin Characters",
        "النص العربي مع أرقام 123 وبعض Latin text ثم العودة للعربية.",
        TextAlignment::Right,
        Vec2::new(-400.0, 0.0),
        &heading_style,
        &base_style,
    );

    // Example 5: RTL with Latin text forcing direction
    spawn_text_example(
        &mut commands,
        "5. RTL with Unicode Control Characters",
        "هذا النص العربي يحتوي على \u{202A}forced LTR text\u{202C} بداخله.",
        TextAlignment::Right,
        Vec2::new(-400.0, -100.0),
        &heading_style,
        &base_style,
    );

    // Example 6: Hebrew text example
    spawn_text_example(
        &mut commands,
        "6. Hebrew Text Example",
        "זוהי דוגמה לטקסט בעברית הנקרא מימין לשמאל.",
        TextAlignment::Right,
        Vec2::new(-400.0, -200.0),
        &heading_style,
        &base_style,
    );

    // Example 7: Center-aligned mixed text
    spawn_text_example(
        &mut commands,
        "7. Center-aligned Mixed Text",
        "English centered with العربية في الوسط and more English.",
        TextAlignment::Center,
        Vec2::new(-400.0, -300.0),
        &heading_style,
        &base_style,
    );
}

// Helper function to spawn text examples
fn spawn_text_example(
    commands: &mut Commands,
    title: &str,
    content: &str,
    position: Vec2,
    heading_style: &TextStyle,
    base_style: &TextStyle,
) {
    // Create a background for better visibility
    commands.spawn((
        Sprite {
            color: Color::srgba(0.1, 0.1, 0.1, 0.8),
            custom_size: Some(Vec2::new(800.0, 80.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(position.x + 400.0, position.y, 0.0)),
    ));

    // Add the title and text content
    commands.spawn((
        Text::new(content),
        Transform::from_translation(Vec3::new(position.x + 10.0, position.y, 1.0)),
    ));
}

// Notes on Unicode Bidirectional Algorithm:
//
// 1. Unicode BIDI Control Characters:
//    - LRE (U+202A): Left-to-Right Embedding
//    - RLE (U+202B): Right-to-Left Embedding
//    - LRO (U+202D): Left-to-Right Override
//    - RLO (U+202E): Right-to-Left Override
//    - PDF (U+202C): Pop Directional Formatting
//    - LRI (U+2066): Left-to-Right Isolate
//    - RLI (U+2067): Right-to-Left Isolate
//    - FSI (U+2068): First Strong Isolate
//    - PDI (U+2069): Pop Directional Isolate
//
// 2. Bevy's Text Rendering:
//    - Bevy uses the cosmic-text library for text layout and rendering
//    - cosmic-text implements the Unicode Bidirectional Algorithm
//    - Text alignment (Left/Center/Right) affects how the overall text block is positioned
//    - Individual character ordering for bidirectional text is handled automatically
//
// 3. Font Considerations:
//    - Ensure the font used supports the scripts you need (Arabic, Hebrew, etc.)
//    - NotoSans is a good choice as it has wide script support
//
// 4. Testing Bidirectional Text:
//    - Always test with real-world text examples in the target languages
//    - Verify that numbers, punctuation, and mixed scripts render correctly
