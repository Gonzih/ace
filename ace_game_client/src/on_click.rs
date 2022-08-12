use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

//================ SYSTEM LABELS ================
// ================ PLUGIN  ================
pub struct SelectorPlugin;

impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .add_startup_system(setup_selection_resource)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(handle_click)
        .add_system(debug_colorizer);
        //.add_system(cast_ray);
    }
}

//================ CONSTANTS ================
// ================ RESOURCES ================

struct SelectedThing {
    eid: Entity,
    previous_thing: Option<Entity>,
}
// ================ COMPONENTS ================
//================ EVENTS ================
// ================ BUNDLES ================
// ================ STARTUP SYSTEMS ================
// ================ SYSTEMS ================
// ================ CONSTANTS================

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(cast_ray)
        .run();
}

fn setup_selection_resource(mut commands: Commands) {
    let null_object = commands.spawn().id();
    let selection_resource = SelectedThing {
        eid: null_object,
        previous_thing: None,
    };
    commands.insert_resource(selection_resource);
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(
            Mat4::look_at_rh(
                Vec3::new(-30.0, 30.0, 100.0),
                Vec3::new(0.0, 10.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )
            .inverse(),
        ),
        ..Default::default()
    });
}

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */
    let ground_size = 200.1;
    let ground_height = 0.1;

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            -ground_height,
            0.0,
        )))
        .insert(Collider::cuboid(ground_size, ground_height, ground_size));

    /*
     * Create the cubes
     */
    let num = 4;
    let rad = 1.0;

    let shift = rad * 2.0 + rad;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;
    let centerz = shift * (num / 2) as f32;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

    for j in 0usize..4 {
        for i in 0..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx + offset;
                let y = j as f32 * shift + centery + 3.0;
                let z = k as f32 * shift - centerz + offset;

                // Build the rigid body.
                commands
                    .spawn_bundle(TransformBundle::from(Transform::from_xyz(x, y, z)))
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::cuboid(rad, rad, rad));
            }
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}

fn handle_click(
    mut commands: Commands,
    windows: Res<Windows>,
    rapier_context: Res<RapierContext>,
    bodies: Query<&RigidBody>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selected_thing: ResMut<SelectedThing>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        warn!("raycast not currently restricted to only active camera ");
        for (camera, camera_transform) in cameras.iter() {
            let (ray_pos, ray_dir) =
                ray_from_mouse_position(windows.get_primary().unwrap(), camera, camera_transform);

            let hit = rapier_context.cast_ray(
                ray_pos,
                ray_dir,
                f32::MAX,
                true,
                InteractionGroups::default(),
                None,
            );

            if let Some((entity, _toi)) = hit {
                if let Ok(rigid_body) = bodies.get(entity) {
                    if *rigid_body == RigidBody::Dynamic {
                        selected_thing.previous_thing = Some(selected_thing.eid.clone());
                        selected_thing.eid = entity.clone();
                    }
                }
            }
        }
    }
}

fn debug_colorizer(mut commands: Commands, mut selected_thing: ResMut<SelectedThing>) {
    if let Some(eid) = selected_thing.previous_thing {
        let color = Color::BLUE;
        commands.entity(eid).remove::<ColliderDebugColor>();
        commands
            .entity(selected_thing.eid)
            .insert(ColliderDebugColor(color));
        selected_thing.previous_thing = None;
    }
}

fn cast_ray(
    mut commands: Commands,
    windows: Res<Windows>,
    rapier_context: Res<RapierContext>,
    bodies: Query<&RigidBody>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    // We will color in read the colliders hovered by the mouse.
    for (camera, camera_transform) in cameras.iter() {
        // First, compute a ray from the mouse position.
        let (ray_pos, ray_dir) =
            ray_from_mouse_position(windows.get_primary().unwrap(), camera, camera_transform);

        // Then cast the ray.
        let hit = rapier_context.cast_ray(
            ray_pos,
            ray_dir,
            f32::MAX,
            true,
            InteractionGroups::default(),
            None,
        );

        if let Some((entity, _toi)) = hit {
            // Color in red the entity we just hit.
            // But don't color it if the rigid-body is not dynamic.
            if let Ok(rb) = bodies.get(entity) {
                if *rb == RigidBody::Dynamic {
                    let color = Color::BLUE; // Color in blue.
                    commands.entity(entity).insert(ColliderDebugColor(color));
                }
            }
        }
    }
}

// Credit to @doomy on discord.
fn ray_from_mouse_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> (Vec3, Vec3) {
    let mouse_position = window.cursor_position().unwrap_or(Vec2::new(0.0, 0.0));

    let x = 2.0 * (mouse_position.x / window.width() as f32) - 1.0;
    let y = 2.0 * (mouse_position.y / window.height() as f32) - 1.0;

    let camera_inverse_matrix =
        camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    let near = camera_inverse_matrix * Vec3::new(x, y, -1.0).extend(1.0);
    let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

    let near = near.truncate() / near.w;
    let far = far.truncate() / far.w;
    let dir: Vec3 = far - near;
    (near, dir)
}
