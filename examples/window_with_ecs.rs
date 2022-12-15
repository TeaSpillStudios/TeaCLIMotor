use ecs_rust::{
    entity_manager::{EntityIdAccessor, EntityManager},
    system::System,
};
use taconite::*;

struct PrintTransformSystem {}

impl System for PrintTransformSystem {
    fn update(&mut self, manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
        let transforms = manager.borrow_components::<Transform>().unwrap();

        for transform in transforms.iter() {
            println!(
                "Position:\nx: {} y: {} z: {}\n\nRotation:\nx: {} y: {} z: {}\n",
                transform.position.x,
                transform.position.y,
                transform.position.z,
                transform.rotation.x,
                transform.rotation.y,
                transform.rotation.z
            )
        }
    }
}

fn main() {
    let mut taconite = Taconite::default();

    let entity = taconite.create_entity();
    taconite.add_component_to_entity(
        entity,
        Transform {
            rotation: Vector3::new(45., 45., 0.),
            ..Default::default()
        },
    );

    taconite.add_system(PrintTransformSystem {});

    taconite.start();
}
