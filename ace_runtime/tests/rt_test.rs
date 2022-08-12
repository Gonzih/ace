// use ace_components::Dynamic;
use ace_components::*;
use ace_runtime::*;
use bevy::prelude::*;

#[test]
fn simple_timer_tick() {
    let mut world = World::new();

    world.init_resource::<AgenticRuntimeResource>();
    world
        .spawn()
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(Dynamic::test_timer());

    let mut init_stage = SystemStage::single_threaded();
    init_stage.add_system(
        init_dynamic_components
            .label(SystemLabels::InitDynComponents)
            .before(SystemLabels::RunDynSystem),
    );

    let mut run_stage = SystemStage::single_threaded();
    run_stage.add_system(
        run_dynamic_system
            .exclusive_system()
            .label(SystemLabels::RunDynSystem)
            .after(SystemLabels::InitDynComponents),
    );

    init_stage.run(&mut world);
    run_stage.run(&mut world);

    let mut query = world.query::<(&Transform, &Dynamic)>();

    assert_eq!(query.iter(&world).len(), 1);

    for (transform, _) in query.iter(&world) {
        assert_eq!(transform.translation, Vec3::new(0.1, 0.0, 0.0));
    }
}
