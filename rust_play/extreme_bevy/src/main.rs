mod args;
mod components;
mod input;
use bevy::{
    math::VectorSpace, prelude::*, render::camera::ScalingMode, tasks::IoTaskPool, utils::HashMap,
};
use bevy_asset_loader::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::{
    matchbox_socket::{PeerId, WebRtcSocket},
    prelude::*,
};
use clap::Parser;

use args::Args;
use components::*;
use input::*;

const MAP_SIZE: u32 = 41;
const GRID_WIDTH: f32 = 0.05;
const PLAYER_RADIUS: f32 = 0.5;
const BULLET_RADIUS: f32 = 0.025;

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(path = "bullet.png")]
    bullet: Handle<Image>,
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    AssetLoading,
    Matchmaking,
    InGame,
}

// The first generic parameter, u8, is the input type: 4-directions + fire fits
// easily in a single byte
// The second parameter is the address type of peers: Matchbox' WebRtcSocket
// addresses are called `PeerId`s
type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);

    // Horizontal lines
    for i in 0..=MAP_SIZE {
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                0.,
                i as f32 - MAP_SIZE as f32 / 2.,
                0.,
            )),
            sprite: Sprite {
                color: Color::srgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(MAP_SIZE as f32, GRID_WIDTH)),
                ..default()
            },
            ..default()
        });
    }

    // Vertical lines
    for i in 0..=MAP_SIZE {
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                i as f32 - MAP_SIZE as f32 / 2.,
                0.,
                0.,
            )),
            sprite: Sprite {
                color: Color::srgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(GRID_WIDTH, MAP_SIZE as f32)),
                ..default()
            },
            ..default()
        });
    }
}

fn spawn_players(mut commands: Commands) {
    // player 0
    commands
        .spawn((
            Player { handle: 0 },
            BulletReady(true),
            MoveDir(-Vec2::X),
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(-2., 0., 100.)),
                sprite: Sprite {
                    color: Color::srgb(0., 0.47, 1.),
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
        ))
        .add_rollback();

    // player 1
    commands
        .spawn((
            Player { handle: 1 },
            BulletReady(true),
            MoveDir(Vec2::X),
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(2., 0., 100.)),
                sprite: Sprite {
                    color: Color::srgb(0., 0.4, 0.),
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
        ))
        .add_rollback();
}

fn start_matchbox_socket(mut commands: Commands, args: Res<Args>) {
    // let room_url = "ws://box.vitalyr.com/extreme_bevy?next=2";
    let room_url = args.room_url.clone();
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

fn move_players(
    mut player: Query<(&mut Transform, &Player, &mut MoveDir)>,
    inputs: Res<PlayerInputs<Config>>,
    time: Res<Time>,
) {
    for (mut transform, player, mut move_dir) in &mut player {
        let (input, _) = inputs[player.handle];
        let direction = direction(input);

        if direction == Vec2::ZERO {
            continue;
        }
        move_dir.0 = direction;

        let move_speed = 7.;
        let move_delta = (direction * move_speed) * time.delta_seconds();

        let old_pos = transform.translation.xy();
        let limit = Vec2::splat(MAP_SIZE as f32 / 2. - 0.5);
        let new_pos = (old_pos + move_delta).clamp(-limit, limit);

        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;
    }
}

fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if socket.get_channel(0).is_err() {
        // we've already started
        return;
    }

    // check for new connections
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    // create a GGRS P2P session
    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the channel out of the socket (required bacause GGRS takes ownership of it)
    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start ggrs session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));
    next_state.set(GameState::InGame);
}

fn camera_follow(
    local_players: Res<LocalPlayers>,
    players: Query<(&Player, &Transform)>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    for (player, player_transform) in &players {
        // only follow the local player
        if !local_players.0.contains(&player.handle) {
            continue;
        }

        let pos = player_transform.translation;

        for mut transform in &mut cameras {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

fn fire_bullets(
    mut commands: Commands,
    inputs: Res<PlayerInputs<Config>>,
    images: Res<ImageAssets>,
    mut players: Query<(&Transform, &Player, &mut BulletReady, &MoveDir)>,
) {
    for (transform, player, mut bullet_ready, move_dir) in &mut players {
        let (input, _) = inputs[player.handle];
        if fire(input) && bullet_ready.0 {
            let player_pos = transform.translation.xy();
            let pos = player_pos + move_dir.0 * PLAYER_RADIUS + BULLET_RADIUS;
            commands
                .spawn((
                    Bullet,
                    *move_dir,
                    SpriteBundle {
                        transform: Transform::from_translation(pos.extend(200.))
                            .with_rotation(Quat::from_rotation_arc_2d(Vec2::X, move_dir.0)),
                        texture: images.bullet.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(0.3, 0.1)),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .add_rollback();
            bullet_ready.0 = false;
        }
    }
}

fn reload_bullet(
    inputs: Res<PlayerInputs<Config>>,
    mut players: Query<(&mut BulletReady, &Player)>,
) {
    for (mut bullet_ready, player) in players.iter_mut() {
        let (input, _) = inputs[player.handle];
        if !fire(input) {
            bullet_ready.0 = true;
        }
    }
}

fn move_bullet(mut bullets: Query<(&mut Transform, &MoveDir), With<Bullet>>, time: Res<Time>) {
    for (mut transform, move_dir) in &mut bullets {
        let speed = 17.;
        let delta = move_dir.0 * speed * time.delta_seconds();
        transform.translation += delta.extend(0.);
    }
}

fn kill_player(
    mut commands: Commands,
    players: Query<(Entity, &Transform), With<Player>>,
    bullets: Query<&Transform, With<Bullet>>,
) {
    for (player, player_transform) in &players {
        for bullet_transform in &bullets {
            let distance = Vec2::distance(
                player_transform.translation.xy(),
                bullet_transform.translation.xy(),
            );

            if distance < PLAYER_RADIUS + BULLET_RADIUS {
                commands.entity(player).despawn_recursive();
            }
        }
    }
}

fn synctest_mode(args: Res<Args>) -> bool {
    args.synctest
}

fn p2p_mode(args: Res<Args>) -> bool {
    !args.synctest
}

fn main() {
    let args = Args::parse();
    eprintln!("{args:?}");

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // fill the entire browser window
                    fit_canvas_to_parent: true,
                    // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GgrsPlugin::<Config>::default(),
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::Matchmaking),
        )
        .rollback_component_with_copy::<MoveDir>()
        .rollback_component_with_clone::<Transform>()
        .rollback_component_with_copy::<BulletReady>()
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .insert_resource(args)
        .add_systems(
            OnEnter(GameState::Matchmaking),
            (setup, start_matchbox_socket.run_if(p2p_mode)),
        )
        .add_systems(OnEnter(GameState::InGame), spawn_players)
        .add_systems(
            Update,
            (
                wait_for_players.run_if(in_state(GameState::Matchmaking).and_then(p2p_mode)),
                camera_follow.run_if(in_state(GameState::InGame)),
            ),
        )
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(
            GgrsSchedule,
            (
                move_players,
                reload_bullet,
                fire_bullets.after(move_players).after(reload_bullet),
                move_bullet.after(fire_bullets),
                kill_player.after(move_bullet).after(move_players),
            ),
        )
        .run();
}
