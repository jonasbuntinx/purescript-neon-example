
use neon::prelude::*;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::io::{self};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};

#[derive(Debug)]
struct ProgramError(String);

impl From<io::Error> for ProgramError {
    fn from(err: io::Error) -> Self {
        Self(err.to_string())
    }
}

impl<T> From<TryLockError<T>> for ProgramError {
    fn from(err: TryLockError<T>) -> Self {
        Self(err.to_string())
    }
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for ProgramError {}

#[derive(Clone)]
struct Program(Arc<Mutex<App>>);

impl Program {
    fn new() -> Self {
        return Self(Arc::new(Mutex::new(App::new())));
    }

    fn lock(&self) -> Result<MutexGuard<App>, ProgramError> {
        return Ok(self.0.try_lock()?);
    }

    fn run(self) -> Result<(), ProgramError> {
        self.lock()?.run();
        return Ok(());
    }

    fn add_plugins<T: PluginGroup>(self, group: T) -> Result<Self, ProgramError> {
        self.lock()?.add_plugins(group);
        return Ok(self);
    }

    fn add_startup_system<Params>(self, system: impl IntoSystemDescriptor<Params>) -> Result<Self, ProgramError> {
        self.lock()?.add_startup_system(system);
        return Ok(self);
    }
}

impl Finalize for Program {}
unsafe impl Send for Program {}

fn mk_app(mut cx: FunctionContext) -> JsResult<JsBox<Program>> {
    return Ok(cx.boxed(Program::new()));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("icon.png"),
        ..default()
    });
}

fn add_plugins(mut cx: FunctionContext) -> JsResult<JsBox<Program>> {
    let program = Program::clone(&&cx.argument::<JsBox<Program>>(0)?);

    let result = program
        .add_plugins(DefaultPlugins
            .build()
            .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
        )
        .or_else(|err| cx.throw_error(err.to_string()))?;

    return Ok(cx.boxed(result));
}


fn add_startup_system(mut cx: FunctionContext) -> JsResult<JsBox<Program>> {
    let program = Program::clone(&&cx.argument::<JsBox<Program>>(0)?);

    let result = program
        .add_startup_system(setup)
        .or_else(|err| cx.throw_error(err.to_string()))?;

    return Ok(cx.boxed(result));
}

fn run(mut cx: FunctionContext) -> JsResult<JsObject> {
    let program = Program::clone(&&cx.argument::<JsBox<Program>>(0)?);

    program
        .run()
        .or_else(|err| cx.throw_error(err.to_string()))?;

    return Ok(cx.empty_object());
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("mkApp", mk_app)?;
    cx.export_function("addPlugins", add_plugins)?;
    cx.export_function("addStartupSystem", add_startup_system)?;
    cx.export_function("run", run)?;
    return Ok(());
}