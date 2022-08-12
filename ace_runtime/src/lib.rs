use ace_components::Dynamic;
use anyhow::Result;
use bevy::core::FixedTimestep;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use handlebars::Handlebars;
use serde_json::json;
use std::sync::{Arc, Mutex};

mod rt;
pub use rt::*;

// ================ RT ================

unsafe impl Send for AgenticRT {}
unsafe impl Sync for AgenticRT {}

pub struct AgenticRT {
    rt: Arc<Mutex<DenoRT>>,
}

impl AgenticRT {
    pub fn new() -> Result<Self> {
        let rt = DenoRT::new()?;
        let rt = Arc::new(Mutex::new(rt));

        Ok(Self { rt })
    }
}

// ================ LABELS ================

#[derive(SystemLabel, Hash, Clone, Eq, PartialEq, Debug)]
pub enum SystemLabels {
    InitDynComponents,
    RunDynSystem,
}

// ================ RESOURCES ================

pub struct AgenticRuntimeResource {
    runtime: AgenticRT,
}

impl Default for AgenticRuntimeResource {
    fn default() -> Self {
        Self::new().expect("Could not init Agentic Runtime")
    }
}

impl AgenticRuntimeResource {
    pub fn new() -> Result<Self> {
        let runtime = AgenticRT::new()?;
        let mut resource = Self { runtime };

        resource.reset()?;

        Ok(resource)
    }

    pub fn reset(&mut self) -> Result<()> {
        self.runtime = AgenticRT::new()?;
        let setup = include_str!("ts/ace.js");
        let mut rt = self.runtime.rt.lock().expect("locking runtime");

        rt.execute_script("<reset>", setup)
    }

    pub fn tick(&self, e: Entity) -> Result<()> {
        let mut hb = Handlebars::new();
        let template = include_str!("js/execute.js");
        hb.register_template_string("execute", template)?;
        let code = hb.render("execute", &json!({ "method": "tick", "eid": e.id() }))?;

        let mut rt = self.runtime.rt.lock().expect("locking runtime");

        rt.execute_script("<tick>", &code)
    }

    pub fn init(&self, e: Entity, code: &str) -> Result<()> {
        let mut hb = Handlebars::new();
        let template = include_str!("js/wrapper.js");
        hb.register_template_string("wrapper", template)?;

        let code = hb.render("wrapper", &json!({ "code": code, "eid": e.id() }))?;

        let mut rt = self.runtime.rt.lock().expect("locking runtime");

        rt.execute_script("<init>", &code)
    }

    pub fn set_ace_state(&self, world_addr: usize, entity: Entity) -> Result<()> {
        let mut rt = self.runtime.rt.lock().expect("deno lock");

        rt.set_ace_state(world_addr, entity)
    }

    pub fn unset_ace_state(&self) -> Result<()> {
        let mut rt = self.runtime.rt.lock().expect("deno lock");

        rt.unset_ace_state()
    }
}

// ================ PLUGIN ================

pub struct AgenticRuntimePlugin;

impl Plugin for AgenticRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(
                    run_dynamic_system
                        .exclusive_system()
                        .label(SystemLabels::RunDynSystem)
                        .after(SystemLabels::InitDynComponents),
                ),
        )
        .add_system(init_dynamic_components.label(SystemLabels::InitDynComponents))
        .init_resource::<AgenticRuntimeResource>();
    }
}

pub fn init_dynamic_components(
    runtime: ResMut<AgenticRuntimeResource>,
    mut query: Query<(Entity, &mut Dynamic)>,
) {
    for (entity, mut dynamic) in query.iter_mut() {
        if !dynamic.init {
            runtime
                .init(entity, dynamic.code)
                .expect(&format!("init {:?}", entity));
            dynamic.init = true;
        }
    }
}

pub fn run_dynamic_system(world: &mut World) {
    let world_ref = world as *mut World;
    let world_addr = world_ref as usize;

    let mut system_state: SystemState<(
        Option<ResMut<AgenticRuntimeResource>>,
        Query<(Entity, &Dynamic)>,
    )> = SystemState::new(world);

    let (runtime_option, mut query) = system_state.get_mut(world);

    if let Some(runtime) = runtime_option {
        for (entity, _) in query.iter_mut() {
            runtime
                .set_ace_state(world_addr, entity)
                .expect("set ace state");
            runtime.tick(entity).expect("running tick on dynamic");
            runtime.unset_ace_state().expect("unset ace state");
        }
    } else {
        error!("Could not get runtime resource");
    }
}
