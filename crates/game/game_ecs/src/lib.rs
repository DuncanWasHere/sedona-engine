pub mod components;
pub mod entities;
pub mod events;
pub mod game;
pub mod game_events;
pub mod systems;
pub mod utils;
pub mod world;

pub use game::*;
pub use game_events::*;

mod tests {
    use crate::components::{
        ModelPath, NodeEntityRef, PlayerComponent, StringId, TransformComponent,
    };
    use crate::entities::PlayerEntity;
    use crate::utils::serialize::save_game_package;
    use crate::world::{World, WorldCreate};

    #[test]
    fn test_serialization() {
        let mut world = World::default();

        world.create(PlayerEntity {
            string_id: StringId(Some(String::from("player_default"))),
            player_data: PlayerComponent::default(),
            transform: TransformComponent::default(),
            model_path: ModelPath(String::from("player_default")),
            model: NodeEntityRef::default(),
        });

        save_game_package(&world, "../../../data/big_berg.ron");
    }
}
