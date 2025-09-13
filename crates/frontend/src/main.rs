use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;
use engine::Engine;

mod resources {
    use std::sync::{Arc, Mutex};

    use bevy::{ecs::resource::Resource, tasks::Task};

    use protocol::StepResult;
    use transport::TransportResult;

    #[derive(Resource, Default)]
    pub enum TurnStage {
        #[default]
        PlayerStage,
        NetworkStage {
            task: Task<TransportResult<StepResult>>,
        },
    }

    #[derive(Resource, Default)]
    pub struct TurnInfo {
        pub stage: TurnStage,
    }

    #[derive(Resource)]
    pub struct GlobalState {
        pub state: Arc<Mutex<transport::LocalState>>,
    }
}

mod components {
    use bevy::ecs::component::Component;

    #[derive(Component)]
    pub struct Player;

    #[derive(Component)]
    pub struct Npc {
        pub entity_id: u32,
    }

    #[derive(Component)]
    pub struct Background;
}

mod systems {
    use bevy::{
        prelude::*,
        tasks::{IoTaskPool, futures_lite::future},
    };
    use engine::Engine;
    use protocol::PlayerAction;

    use crate::{
        components::{Background, Npc, Player},
        resources::{GlobalState, TurnInfo, TurnStage},
    };

    pub fn setup(
        mut commands: Commands,
        global_state: ResMut<GlobalState>,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        commands.spawn(Camera2d);

        let sprite_texture = asset_server.load("sprites.png");

        let mut layout = TextureAtlasLayout::new_empty(UVec2::new(2679, 651));

        layout.add_texture(bevy::math::URect::new(1, 1, 13, 13));

        layout.add_texture(bevy::math::URect::new(1353, 1, 1365, 13));

        layout.add_texture(bevy::math::URect::new(1353, 40, 1365, 52));

        layout.add_texture(bevy::math::URect::new(1, 66, 13, 78));
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        {
            let state = global_state.state.lock().unwrap();

            for (position, tile) in state.dungeon().iter() {
                let screen_x = position.x as f32 * 96.0;
                let screen_y = position.y as f32 * -96.0;

                let tile_index = match tile {
                    corelib::Tile::Floor => 3,
                    corelib::Tile::Empty => 0,
                };

                commands.spawn((
                    Sprite::from_atlas_image(
                        sprite_texture.clone(),
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: tile_index,
                        },
                    ),
                    Transform::from_translation(Vec3::new(
                        screen_x, screen_y, -1.0,
                    ))
                    .with_scale(Vec3::splat(8.0)),
                    Background,
                ));
            }

            let player = state.player();
            let player_position = player.position();
            let player_screen_x = player_position.x as f32 * 96.0;
            let player_screen_y = player_position.y as f32 * -96.0;

            commands.spawn((
                Sprite::from_atlas_image(
                    sprite_texture.clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: 1,
                    },
                ),
                Transform::from_translation(Vec3::new(
                    player_screen_x,
                    player_screen_y,
                    1.0,
                ))
                .with_scale(Vec3::splat(8.0)),
                Player,
            ));

            for entity in state.entities() {
                let position = entity.position();

                let screen_x = position.x as f32 * 96.0;
                let screen_y = position.y as f32 * -96.0;

                commands.spawn((
                    Sprite::from_atlas_image(
                        sprite_texture.clone(),
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: 2,
                        },
                    ),
                    Transform::from_translation(Vec3::new(
                        screen_x, screen_y, 0.5,
                    ))
                    .with_scale(Vec3::splat(8.0)),
                    Npc { entity_id: entity.id().into() },
                ));
            }
        };
    }

    pub fn input_turn(
        keys: Res<ButtonInput<KeyCode>>,
        mut turn_info: ResMut<TurnInfo>,
        global_state: ResMut<GlobalState>,
    ) {
        if !matches!(turn_info.stage, TurnStage::PlayerStage) {
            return;
        }

        let direction = if keys.just_pressed(KeyCode::KeyW) {
            protocol::Direction::North
        } else if keys.just_pressed(KeyCode::KeyS) {
            protocol::Direction::South
        } else if keys.just_pressed(KeyCode::KeyA) {
            protocol::Direction::West
        } else if keys.just_pressed(KeyCode::KeyD) {
            protocol::Direction::East
        } else {
            return;
        };

        let state_arc = global_state.state.clone();
        let task_pool = IoTaskPool::get();
        let task = task_pool.spawn(async move {
            let mut engine = Engine::new_local_game(state_arc);
            engine.apply_step(PlayerAction::Move(direction)).await
        });

        turn_info.stage = TurnStage::NetworkStage { task };
    }

    pub fn poll_turn_result(
        mut turn_info: ResMut<TurnInfo>,
        global_state: ResMut<GlobalState>,
        mut player_query: Query<&mut Transform, (With<Player>, Without<Npc>)>,
        mut npc_query: Query<(&mut Transform, &Npc), Without<Player>>,
    ) {
        if let TurnStage::NetworkStage { task } = &mut turn_info.stage {
            let status = future::block_on(future::poll_once(task));

            if let Some(chunk_data) = status {
                if let Ok(_step_result) = chunk_data {
                    // Update player and NPC positions after turn completion
                    let state = global_state.state.lock().unwrap();

                    // Update player position
                    if let Ok(mut player_transform) = player_query.single_mut()
                    {
                        let player = state.player();
                        let player_position = player.position();
                        let player_screen_x = player_position.x as f32 * 96.0;
                        let player_screen_y = player_position.y as f32 * -96.0;
                        player_transform.translation =
                            Vec3::new(player_screen_x, player_screen_y, 1.0);
                    }

                    // Update NPC positions
                    for (mut npc_transform, npc_component) in
                        npc_query.iter_mut()
                    {
                        if let Some(entity) = state
                            .entities()
                            .iter()
                            .find(|e| e.id() == npc_component.entity_id.into())
                        {
                            let position = entity.position();
                            let screen_x = position.x as f32 * 96.0;
                            let screen_y = position.y as f32 * -96.0;
                            npc_transform.translation =
                                Vec3::new(screen_x, screen_y, 0.5);
                        }
                    }
                }
                turn_info.stage = TurnStage::PlayerStage;
            }
        }
    }
}

fn main() {
    let state = Engine::new_state();
    let state = Arc::new(Mutex::new(state));

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, systems::setup)
        .add_systems(
            FixedUpdate,
            (systems::input_turn, systems::poll_turn_result),
        )
        .init_resource::<resources::TurnInfo>()
        .insert_resource(resources::GlobalState { state })
        .run();
}
