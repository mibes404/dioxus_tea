//! Implementation of the Tea-model for Dioxus.
//! Example usage can be found in the `examples/tea-time` directory.
//!
//! Usage:
//! ```rust, nocompile
//! #[derive(Default, Clone, PartialEq)]
//! pub struct AppState {
//!    pub status: Status,
//! }
//!
//! pub enum AppStatusUpdate {
//!     CupFetched,
//!     AddWater(u8),
//!     AddTeaBag(TeaType),
//!     Done,
//! }
//!
//! impl TeaModel for AppState {
//!     type Action = AppStatusUpdate;
//!
//!     fn update(action: Self::Action, mut writer: Write<Self>) {
//!         match action {
//!            // handle actions and update the state accordingly
//!            AppStatusUpdate::CupFetched => {
//!                 // when the cup is fetched, we start with an empty cup
//!                 writer.status = Status::EmptyCup;
//!             }
//!             // other actions
//!         }
//!     }
//! }
//!
//! #[component]
//!  pub fn App() -> Element {
//!     let app_state = use_tea_model::<AppState>();
//!     app_state.send(AppStatusUpdate::CupFetched);
//! }
//! ```

#![warn(clippy::pedantic)]

use dioxus::hooks::UnboundedReceiver;
use dioxus::prelude::{
    Coroutine, Readable, ReadableRef, Signal, Writable, Write, use_coroutine, use_signal,
};
use futures_util::StreamExt;

/// Trait representing a TEA model in Dioxus.
pub trait TeaModel: 'static + Default + Clone + PartialEq {
    /// The type of actions that can be processed by this model.
    type Action;

    /// Updates the model state based on the provided action.
    fn update(action: Self::Action, writer: Write<Self>);
}

/// A signal that holds the state of a `TeaModel` and provides an internal coroutine for processing actions.
#[derive(Clone, PartialEq)]
pub struct TeaModelSignal<T: TeaModel> {
    inner: Signal<T>,
    co: Coroutine<<T as TeaModel>::Action>,
}

impl<T: TeaModel> Copy for TeaModelSignal<T> {}

impl<T: TeaModel> TeaModelSignal<T> {
    #[must_use]
    /// Returns a reference to the underlying signal for reading the model state.
    pub fn read(&self) -> ReadableRef<Signal<T>> {
        self.inner.read()
    }

    /// Sends an action to the coroutine for processing.
    pub fn send(&self, action: T::Action) {
        self.co.send(action);
    }
}

#[must_use]
/// Creates a new `TeaModelSignal` for the given `TeaModel`.
pub fn use_tea_model<T: TeaModel>() -> TeaModelSignal<T> {
    let mut inner = use_signal(|| T::default());

    let co = use_coroutine(move |mut rx: UnboundedReceiver<T::Action>| async move {
        loop {
            if let Some(action) = rx.next().await {
                let writer = inner.write();
                T::update(action, writer);
            }
        }
    });

    TeaModelSignal { inner, co }
}
