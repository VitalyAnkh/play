use bevy::prelude::*;
use bevy::ui::Interaction; // Interaction 只有 Pressed、Hovered 和 None

/// 自定义的“禁用”组件，用于标记需要被默认过滤掉的 UI 实体
#[derive(Component, Clone, Debug)]
struct HiddenUI;

/// 用于记录需要切换显示/隐藏的文本实体及其当前状态
#[derive(Resource, Default)]
struct TextState {
    entity: Option<Entity>,
    visible: bool,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // 注册 HiddenUI 组件作为默认禁用组件
    {
        let world = app.world_mut();
        world.register_disabling_component::<HiddenUI>();
    }

    // 在 Startup 阶段添加 UI 布局，在 Update 阶段处理按钮交互
    app.add_systems(Startup, setup);
    app.add_systems(Update, button_system);

    app.run();
}

/// 在启动时创建 UI 元素：一个文本和一个按钮
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut text_state: ResMut<TextState>,
) {
    // 添加 2D 摄像机（新版推荐 Camera2dBundle）
    commands.spawn(Camera2dBundle::default());

    // 创建文本实体，初始时附加 HiddenUI，从而默认不显示
    let text_entity = commands
        .spawn((
            TextBundle::from_section(
                "Hello, GUI!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            HiddenUI,
        ))
        .id();
    text_state.entity = Some(text_entity);
    text_state.visible = false;

    // 创建按钮实体，用于切换文本显示状态
    commands
        .spawn((
            Button,
            Style {
                width: Val::Px(200.0),
                height: Val::Px(80.0),
                margin: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                left: Val::Px(100.0),
                bottom: Val::Px(100.0),
                ..Default::default()
            },
            // 使用 RGB 设置按钮背景色
            BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
        ))
        .with_children(|parent| {
            // 按钮的子实体：显示按钮文字
            parent.spawn(TextBundle::from_section(
                "Toggle Text",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
        });
}

/// 按钮交互系统：检测按钮的 Interaction 状态（Pressed、Hovered 或 None），
/// 当状态为 Pressed 时切换文本实体的 HiddenUI 组件，从而实现显示或隐藏文本
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_state: ResMut<TextState>,
    mut commands: Commands,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(text_entity) = text_state.entity {
                    if text_state.visible {
                        // 文本当前显示，则插入 HiddenUI 使其隐藏
                        commands.entity(text_entity).insert(HiddenUI);
                        text_state.visible = false;
                        info!("Text hidden");
                    } else {
                        // 文本当前隐藏，则移除 HiddenUI 使其显示
                        commands.entity(text_entity).remove::<HiddenUI>();
                        text_state.visible = true;
                        info!("Text shown");
                    }
                }
                *bg_color = BackgroundColor(Color::rgb(0.5, 0.5, 0.5));
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::rgb(0.2, 0.2, 0.2));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::rgb(0.2, 0.2, 0.2));
            }
        }
    }
}
