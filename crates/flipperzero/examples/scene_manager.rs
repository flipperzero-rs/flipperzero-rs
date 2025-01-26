#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

use flipperzero::{
    gui::scene_manager::{Event, Scene},
    info, new_scene_manager, scenes,
};
use flipperzero_rt::{entry, manifest};

struct Start;
impl Scene for Start {
    fn on_enter() {
        info!("Start::on_enter called");
    }

    fn on_event(event: Event) -> bool {
        info!("Start::on_event({:?}) called", event);
        false
    }

    fn on_exit() {
        info!("Start::on_exit called");
    }
}

struct Read;
impl Scene for Read {
    fn on_enter() {
        info!("Read::on_enter called");
    }

    fn on_event(event: Event) -> bool {
        info!("Read::on_event({:?}) called", event);
        false
    }

    fn on_exit() {
        info!("Read::on_exit called");
    }
}

manifest!(name = "Rust scene manager example");
entry!(main);
scenes!(example, Start, Read);

fn main(_args: *mut u8) -> i32 {
    info!("Starting scene manager");
    let mut scene_manager = new_scene_manager!(example);

    // With no scene set, no events will be sent.
    info!("Sending custom event");
    let _ = scene_manager.handle_custom_event(7);
    info!("Sending tick event");
    scene_manager.handle_tick_event();
    info!("Sending back event");
    let _ = scene_manager.handle_back_event();

    // Setting the first scene triggers its `on_enter` callback.
    info!("Next scene: Start");
    scene_manager.next_scene(ExampleScene::Start);

    // Events are now sent to the first scene.
    info!("Sending custom event");
    let _ = scene_manager.handle_custom_event(42);
    info!("Sending tick event");
    scene_manager.handle_tick_event();

    // Adding another scene will exit the current one and enter the new one.
    info!("Next scene: Read");
    scene_manager.next_scene(ExampleScene::Read);

    // Events are now sent to the second scene.
    info!("Sending custom event");
    let _ = scene_manager.handle_custom_event(69);
    info!("Sending tick event");
    scene_manager.handle_tick_event();

    // The back event exits the second scene and re-enters the first scene.
    info!("Sending back event");
    let _ = scene_manager.handle_back_event();

    info!("Done!");
    0
}
