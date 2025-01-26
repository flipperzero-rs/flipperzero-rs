//! Scenes and scene management.
//!
//! # Examples
//!
//! ```
//! use flipperzero::{
//!     gui::scene_manager::{Event, Scene},
//!     new_scene_manager, scenes,
//! };
//!
//! struct First;
//! impl Scene for First {
//!     ...
//! }
//!
//! struct Second;
//! impl Scene for First {
//!     ...
//! }
//!
//! // Define the "example" set of scenes.
//! scenes!(example, First, Second);
//!
//! fn do_stuff() {
//!     // Construct a `SceneManager` from the "example" set of scenes.
//!     let scene_manager = new_scene_manager!(example);
//! }
//! ```

use core::{marker::PhantomData, ptr};

use flipperzero_sys as sys;
use ufmt::derive::uDebug;

/// A scene.
pub trait Scene {
    fn on_enter();
    fn on_event(event: Event) -> bool;
    fn on_exit();
}

/// A [`SceneManager`] event.
#[derive(Debug, uDebug)]
pub enum Event {
    Custom(u32),
    Back,
    Tick,
}

impl Event {
    pub(crate) fn from_raw(event: sys::SceneManagerEvent) -> Self {
        match event.type_ {
            sys::SceneManagerEventType_SceneManagerEventTypeCustom => Event::Custom(event.event),
            sys::SceneManagerEventType_SceneManagerEventTypeBack => Event::Back,
            sys::SceneManagerEventType_SceneManagerEventTypeTick => Event::Tick,
            _ => panic!("Unknown event type {}", event.type_),
        }
    }
}

/// A set of [`Scene`]s.
///
/// This trait is implemented for you on the enum created by the [`scenes`] macro.
///
/// [`scenes`]: crate::scenes
pub trait Scenes: Copy {
    fn id(self) -> u32;
}

pub struct SceneHandlers(sys::SceneManagerHandlers);

impl SceneHandlers {
    pub const fn new(
        on_enter_handlers: &'static [sys::AppSceneOnEnterCallback],
        on_event_handlers: &'static [sys::AppSceneOnEventCallback],
        on_exit_handlers: &'static [sys::AppSceneOnExitCallback],
    ) -> Self {
        Self(sys::SceneManagerHandlers {
            on_enter_handlers: on_enter_handlers.as_ptr(),
            on_event_handlers: on_event_handlers.as_ptr(),
            on_exit_handlers: on_exit_handlers.as_ptr(),
            scene_num: on_enter_handlers.len() as u32,
        })
    }
}

pub struct SceneManager<S: Scenes> {
    inner: *mut sys::SceneManager,
    scenes: PhantomData<S>,
}

impl<S: Scenes> Drop for SceneManager<S> {
    fn drop(&mut self) {
        unsafe { sys::scene_manager_free(self.inner) };
    }
}

impl<S: Scenes> SceneManager<S> {
    /// Constructs a `SceneManager` configured with the given scenes and no context.
    pub(crate) fn new(scene_handlers: &'static SceneHandlers) -> Self {
        Self {
            inner: unsafe { sys::scene_manager_alloc(&scene_handlers.0, ptr::null_mut()) },
            scenes: PhantomData::default(),
        }
    }

    /// Calls the current Scene event handler with [`Event::Custom`].
    ///
    /// Returns `true` if the event was consumed, `false` otherwise.
    pub fn handle_custom_event(&mut self, custom_event: u32) -> bool {
        unsafe { sys::scene_manager_handle_custom_event(self.inner, custom_event) }
    }

    /// Calls the current Scene event handler with [`Event::Back`].
    ///
    /// If the event handler does not consume this event, [`Self::previous_scene`] is
    /// called.
    ///
    /// Returns `true` if the event was consumed or a previous scene was found, `false`
    /// otherwise.
    pub fn handle_back_event(&mut self) -> bool {
        unsafe { sys::scene_manager_handle_back_event(self.inner) }
    }

    /// Calls the current Scene event handler with [`Event::Tick`].
    ///
    /// Returns `true` if the event was consumed, `false` otherwise.
    pub fn handle_tick_event(&mut self) {
        unsafe { sys::scene_manager_handle_tick_event(self.inner) };
    }

    /// Adds and runs the next scene.
    pub fn next_scene(&mut self, next_scene: S) {
        unsafe { sys::scene_manager_next_scene(self.inner, next_scene.id()) };
    }

    /// Runs the previous `Scene`.
    ///
    /// Returns `true` if a previous scene was found, `false` otherwise.
    pub fn previous_scene(&mut self) -> bool {
        unsafe { sys::scene_manager_previous_scene(self.inner) }
    }

    /// Searches the previous scene stack for the given scene.
    pub fn has_previous_scene(&self, scene: S) -> bool {
        unsafe { sys::scene_manager_has_previous_scene(self.inner, scene.id()) }
    }

    /// Searches the previous scene stack for the given scene, and switches to it.
    ///
    /// Returns `true` if a previous scene was found, `false` otherwise.
    pub fn search_and_switch_to_previous_scene(&mut self, scene: S) -> bool {
        unsafe { sys::scene_manager_search_and_switch_to_previous_scene(self.inner, scene.id()) }
    }

    /// Searches the previous scene stack for one of the given scenes, and switches to it.
    ///
    /// Returns `true` if one of the previous scenes was found, `false` otherwise.
    pub fn search_and_switch_to_previous_scene_one_of(&mut self, scenes: &[S]) -> bool {
        unsafe {
            sys::scene_manager_search_and_switch_to_previous_scene_one_of(
                self.inner,
                // SAFETY: This cast is fine as long as `S` is `repr(u32)`, which is the
                // case when `self` is constructed via `new_scene_manager!()`.
                scenes.as_ptr().cast(),
                scenes.len(),
            )
        }
    }

    /// Clears the scene stack and switches to the given scene.
    ///
    /// Returns `true` if the scene stack was non-empty.
    pub fn search_and_switch_to_another_scene(&mut self, scene: S) -> bool {
        unsafe { sys::scene_manager_search_and_switch_to_another_scene(self.inner, scene.id()) }
    }

    /// Exits from the current scene.
    pub fn stop(&mut self) {
        unsafe { sys::scene_manager_stop(self.inner) };
    }
}

/// Defines a set of scenes that can be used to construct a [`new_scene_manager`].
///
/// [`new_scene_manager`]: crate::new_scene_manager
#[macro_export]
macro_rules! scenes {
    ($name:ident, $($scene:ident),*) => {
        $crate::__macro_support::__paste! {
            #[derive(Clone, Copy, Debug)]
            #[repr(u32)]
            enum [< $name:camel Scene >] {
                $(
                    $scene,
                )*
            }

            impl $crate::gui::scene_manager::Scenes for [< $name:camel Scene >] {
                fn id(self) -> u32 {
                    self as u32
                }
            }

            $(
                unsafe extern "C" fn [< __ $name:lower _scene_ $scene:snake _on_enter >](
                    context: *mut core::ffi::c_void,
                ) {
                    <$scene as $crate::gui::scene_manager::Scene>::on_enter();
                }

                unsafe extern "C" fn [< __ $name:lower _scene_ $scene:snake _on_event >](
                    context: *mut core::ffi::c_void,
                    event: $crate::__macro_support::__sys::SceneManagerEvent,
                ) -> bool {
                    <$scene as $crate::gui::scene_manager::Scene>::on_event(
                        $crate::__macro_support::__scene_manager_event(event),
                    )
                }

                unsafe extern "C" fn [< __ $name:lower _scene_ $scene:snake _on_exit >](
                    context: *mut core::ffi::c_void,
                ) {
                    <$scene as $crate::gui::scene_manager::Scene>::on_exit();
                }
            )*

            const [< $name:upper _SCENES >]: $crate::gui::scene_manager::SceneHandlers =
                $crate::gui::scene_manager::SceneHandlers::new(
                    &[$(
                        Some([< __ $name:lower _scene_ $scene:snake _on_enter >]),
                    )*],
                    &[$(
                        Some([< __ $name:lower _scene_ $scene:snake _on_event >]),
                    )*],
                    &[$(
                        Some([< __ $name:lower _scene_ $scene:snake _on_exit >]),
                    )*],
                );
        }
    };
}

/// Constructs a new [`SceneManager`] from a set of previously-defined [`scenes`].
///
/// [`scenes`]: crate::scenes
#[macro_export]
macro_rules! new_scene_manager {
    ($name:ident) => {
        $crate::__macro_support::__paste! {
            $crate::__macro_support::__new_scene_manager::<[< $name:camel Scene >]>(
                &[< $name:upper _SCENES >],
            )
        }
    };
}
