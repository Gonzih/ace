use ace_components::Dynamic;
use anyhow::Result;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use deno_core::error::AnyError;
use deno_core::{op, Extension, JsRuntime, OpState, Resource, ResourceId, RuntimeOptions};
use log::info;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

use ace_components::*;

struct Bridge<'a> {
    state: &'a OpState,
    rid: ResourceId,
    ace_state: Option<Rc<AceState>>,
}

impl<'a> Bridge<'a> {
    pub fn from_state(state: &'a OpState, rid: ResourceId) -> Self {
        let ace_state = state.resource_table.get::<AceState>(rid).ok();

        Bridge {
            state,
            rid,
            ace_state,
        }
    }

    pub fn world_addr(&self) -> Option<usize> {
        self.ace_state.as_ref().map(|ace| ace.world_addr).flatten()
    }

    pub fn entity(&self) -> Option<Entity> {
        self.ace_state.as_ref().map(|ace| ace.entity).flatten()
    }

    pub fn log(self, s: String) -> Result<(), AnyError> {
        println!("Log: {}", s);

        Ok(())
    }

    pub fn move_by(self, x: f32, y: f32, z: f32) -> Result<(), AnyError> {
        if let Some(world_addr) = self.world_addr() {
            let world_ptr = world_addr as *mut World;
            unsafe {
                let world = &mut *world_ptr;

                let mut system_state: SystemState<Query<(Entity, &mut Transform, &Dynamic)>> =
                    SystemState::new(world);

                let mut query = system_state.get_mut(world);

                for (entity, mut transform, _) in query.iter_mut() {
                    if let Some(run_entity) = self.entity() {
                        if entity == run_entity {
                            transform.translation.x += x;
                            transform.translation.y += y;
                            transform.translation.z += z;
                        }
                    } else {
                        error!("Entity wasn't set in state!")
                    }
                }

                system_state.apply(world);
            }
            info!("Move: {} {} {}", x, y, z);
        } else {
            error!("World addr wasn't set in state!");
        }

        Ok(())
    }

    pub fn spawn(self) -> Result<Entity, AnyError> {
        if let Some(world_addr) = self.world_addr() {
            let world_ptr = world_addr as *mut World;

            unsafe {
                let world = &mut *world_ptr;

                let mut system_state: SystemState<Commands> = SystemState::new(world);
                let mut commands = system_state.get_mut(world);
                let id = commands.spawn().id();

                system_state.apply(world);

                Ok(id)
            }
        } else {
            Err(deno_core::error::custom_error(
                "SpawnError",
                "Could not find world_addr in Bridge",
            ))
        }
    }

    pub fn insert<T: Component>(self, entity: Entity, component: T) -> Result<(), AnyError> {
        if let Some(world_addr) = self.world_addr() {
            let world_ptr = world_addr as *mut World;

            unsafe {
                let world = &mut *world_ptr;

                let mut system_state: SystemState<Commands> = SystemState::new(world);
                let mut commands = system_state.get_mut(world);
                commands.entity(entity).insert(component);

                system_state.apply(world);

                Ok(())
            }
        } else {
            Err(deno_core::error::custom_error(
                "SpawnError",
                "Could not find world_addr in Bridge",
            ))
        }
    }
}

// #[op]
// async fn op_sample(
//     state: Rc<RefCell<OpState>>,
//     nums: Vec<f64>,
// ) -> Result<f64, deno_core::error::AnyError> {
//     // Sum inputs
//     let sum = nums.iter().fold(0.0, |a, v| a + v);
//     // return as a Result<f64, AnyError>
//     Ok(sum)
// }

#[op]
fn v1_op_spawn(state: &mut OpState, rid: ResourceId) -> Result<Entity, AnyError> {
    Bridge::from_state(state, rid).spawn()
}

#[op]
fn v1_op_insert_clickable(
    state: &mut OpState,
    rid: ResourceId,
    entity: Entity,
) -> Result<(), AnyError> {
    Bridge::from_state(state, rid).insert(entity, Clickable::new())
}

#[op]
fn v1_op_insert_hoverable(
    state: &mut OpState,
    rid: ResourceId,
    entity: Entity,
) -> Result<(), AnyError> {
    Bridge::from_state(state, rid).insert(entity, Hoverable::new())
}

#[op]
fn v1_op_insert_draggable(
    state: &mut OpState,
    rid: ResourceId,
    entity: Entity,
) -> Result<(), AnyError> {
    Bridge::from_state(state, rid).insert(entity, Draggable::new())
}

#[op]
fn v1_op_log(state: &mut OpState, rid: ResourceId, s: String) -> Result<(), AnyError> {
    Bridge::from_state(state, rid).log(s)
}

#[op]
fn v1_op_move_by(
    state: &mut OpState,
    rid: ResourceId,
    x: f32,
    y: f32,
    z: f32,
) -> Result<(), AnyError> {
    Bridge::from_state(state, rid).move_by(x, y, z)
}

#[derive(Serialize, Deserialize)]
pub struct AceState {
    pub entity: Option<Entity>,
    pub world_addr: Option<usize>,
}

impl Resource for AceState {
    fn close(self: Rc<Self>) {}
}

pub struct DenoRT {
    pub rt: JsRuntime,
    pub rid: ResourceId,
}

impl DenoRT {
    pub fn from_file(path: String) -> Result<Self> {
        let mut rt = Self::new()?;

        let init_code = std::fs::read_to_string(path)?;
        rt.execute_script("<from_file>", &init_code)?;

        Ok(rt)
    }

    pub fn new() -> Result<Self> {
        let ext = Extension::builder()
            .ops(vec![
                v1_op_spawn::decl(),
                v1_op_log::decl(),
                v1_op_move_by::decl(),
                v1_op_insert_clickable::decl(),
                v1_op_insert_hoverable::decl(),
                v1_op_insert_draggable::decl(),
            ])
            .build();

        let mut rt = JsRuntime::new(RuntimeOptions {
            extensions: vec![ext],
            ..Default::default()
        });

        let deno_state = AceState {
            world_addr: None,
            entity: None,
        };

        let state = rt.op_state();
        let rid = state.borrow_mut().resource_table.add(deno_state);

        rt.execute_script(
            "<ace_state_rid>",
            &format!("const global = {{ ace__state_rid: {} }};", rid),
        )?;

        Ok(Self { rt, rid })
    }

    pub fn set_ace_state(&mut self, world_addr: usize, entity: Entity) -> Result<()> {
        let deno_state = AceState {
            world_addr: Some(world_addr),
            entity: Some(entity),
        };

        let state = self.rt.op_state();
        state
            .borrow_mut()
            .resource_table
            .replace::<AceState>(self.rid, deno_state);

        Ok(())
    }

    pub fn unset_ace_state(&mut self) -> Result<()> {
        let deno_state = AceState {
            world_addr: None,
            entity: None,
        };

        let state = self.rt.op_state();
        state
            .borrow_mut()
            .resource_table
            .replace::<AceState>(self.rid, deno_state);

        Ok(())
    }

    pub fn execute_script(&mut self, name: &str, code: &str) -> Result<()> {
        self.rt.execute_script(name, code)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linking_works() {
        let rt = DenoRT::from_file("tests/fixtures/js_add/add.js".to_string());

        rt.expect("deno init");
    }
}
