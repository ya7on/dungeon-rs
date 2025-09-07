use bevy::{
    DefaultPlugins,
    app::{App, FixedUpdate, Startup},
};

mod resources {
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
}

mod components {
    use bevy::ecs::component::Component;

    #[derive(Component)]
    pub struct Player;
}

mod systems {
    use bevy::{
        core_pipeline::core_2d::Camera2d,
        ecs::system::{Commands, Res, ResMut},
        input::{ButtonInput, keyboard::KeyCode},
        tasks::{IoTaskPool, futures_lite::future},
    };

    use crate::resources::{TurnInfo, TurnStage};

    pub fn setup(mut commands: Commands) {
        commands.spawn(Camera2d);
    }

    pub fn input_turn(
        keys: Res<ButtonInput<KeyCode>>,
        mut turn_info: ResMut<TurnInfo>,
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

        let task_pool = IoTaskPool::get();
        let task = task_pool.spawn(async move {
            // TODO: send request to server
            todo!()
        });

        turn_info.stage = TurnStage::NetworkStage { task };
    }

    pub fn poll_turn_result(mut turn_info: ResMut<TurnInfo>) {
        if let TurnStage::NetworkStage { task } = &mut turn_info.stage {
            let status = future::block_on(future::poll_once(task));

            if let Some(chunk_data) = status {
                if let Ok(step_result) = chunk_data {
                    // TODO: handle step result
                }
                turn_info.stage = TurnStage::PlayerStage;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, systems::setup)
        .add_systems(
            FixedUpdate,
            (systems::input_turn, systems::poll_turn_result),
        )
        .init_resource::<resources::TurnInfo>()
        .run();
}
