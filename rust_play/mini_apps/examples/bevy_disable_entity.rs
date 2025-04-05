use bevy::ecs::entity_disabling::Disabled;
use bevy::prelude::*;

#[derive(Resource, Default)]
struct TextState {
    /// 存储需要切换显示/隐藏的文本实体
    entity: Option<Entity>,
    /// 当前是否可见（即没有 Disabled 组件）
    visible: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 初始化用于存储文本实体状态的资源
        .init_resource::<TextState>()
        .add_startup_system(setup)
        .add_system(button_system)
        .run();
}

/// 在启动时创建 UI 元素：一个文本和一个按钮
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut text_state: ResMut<TextState>,
) {
    // 注意：DefaultPlugins 已经会自动创建 UI 摄像机，如果需要可手动添加：
    commands.spawn_bundle(UiCameraBundle::default());

    // 创建一个文本实体，初始时插入 Disabled，使其不会被默认查询渲染
    let text_entity = commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "Hello, GUI!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            style: Style {
                // 将文本放在屏幕的上部
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(100.0),
                    left: Val::Px(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Disabled) // 初始时禁用文本
        .id();

    // 记录文本实体的 ID 以及当前状态（不可见）
    text_state.entity = Some(text_entity);
    text_state.visible = false;

    // 创建一个按钮，用于切换文本显示状态
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(80.0)),
                margin: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // 将按钮放在屏幕底部
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(100.0),
                    left: Val::Px(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::DARK_GRAY),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Toggle Text",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

/// 当按钮被点击时，该系统会切换文本实体的状态：
/// - 如果当前文本可见，则插入 Disabled 组件，文本将被隐藏（默认查询过滤器会排除带 Disabled 的实体）；  
/// - 如果当前文本隐藏，则移除 Disabled 组件，文本会显示出来。
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_state: ResMut<TextState>,
    mut commands: Commands,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if let Some(text_entity) = text_state.entity {
                    if text_state.visible {
                        // 文本当前可见，则禁用它
                        commands.entity(text_entity).insert(Disabled);
                        text_state.visible = false;
                        info!("文本已隐藏");
                    } else {
                        // 文本当前隐藏，则移除 Disabled，使其显示
                        commands.entity(text_entity).remove::<Disabled>();
                        text_state.visible = true;
                        info!("文本已显示");
                    }
                }
                *color = BackgroundColor(Color::GRAY);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::DARK_GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::DARK_GRAY);
            }
        }
    }
}
