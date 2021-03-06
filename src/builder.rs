use std::fs::File;
use std::io::prelude::*;

use actor::LuaActor;
use rlua::Error as LuaError;

pub struct LuaActorBuilder {
    started: Option<String>,
    handle: Option<String>,
    stopped: Option<String>,
}

impl Default for LuaActorBuilder {
    fn default() -> LuaActorBuilder {
        let noop = Some("return".to_string());
        LuaActorBuilder {
            started: noop.clone(),
            handle: noop.clone(),
            stopped: noop.clone(),
        }
    }
}

impl LuaActorBuilder {
    pub fn new() -> Self {
        LuaActorBuilder::default()
    }

    pub fn on_started(&mut self, filename: &str) -> &mut Self {
        self.started = Some(read_to_string(filename));
        self
    }

    pub fn on_started_with_lua(&mut self, script: &str) -> &mut Self {
        self.started = Some(script.to_string());
        self
    }

    pub fn on_handle(&mut self, filename: &str) -> &mut Self {
        self.handle = Some(read_to_string(filename));
        self
    }
    pub fn on_handle_with_lua(&mut self, script: &str) -> &mut Self {
        self.handle = Some(script.to_string());
        self
    }

    pub fn on_stopped(&mut self, filename: &str) -> &mut Self {
        self.stopped = Some(read_to_string(filename));
        self
    }

    pub fn on_stopped_with_lua(&mut self, script: &str) -> &mut Self {
        self.stopped = Some(script.to_string());
        self
    }

    pub fn build(&self) -> Result<LuaActor, LuaError> {
        LuaActor::new(
            self.started.clone(),
            self.handle.clone(),
            self.stopped.clone(),
        )
    }
}

fn read_to_string(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found");
    let mut body = String::new();
    f.read_to_string(&mut body).expect("Failed to read file");

    body
}
