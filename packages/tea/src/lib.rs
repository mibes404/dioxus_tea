//! Implementation of the Tea-model for Dioxus.
//! Example usage can be found in the `examples/tea-time` directory.

#![warn(clippy::pedantic)]

use dioxus::hooks::UnboundedReceiver;
use dioxus::prelude::{
    Coroutine, Readable, ReadableRef, Signal, Writable, Write, use_coroutine, use_signal,
};
use futures_util::StreamExt;

pub trait TeaModel: 'static + Default + Clone + PartialEq {
    type Action;

    fn update(action: Self::Action, writer: Write<Self>);
}

#[derive(Clone, PartialEq)]
pub struct TeaModelSignal<T: TeaModel> {
    inner: Signal<T>,
    co: Coroutine<<T as TeaModel>::Action>,
}

impl<T: TeaModel> Copy for TeaModelSignal<T> {}

impl<T: TeaModel> TeaModelSignal<T> {
    #[must_use]
    pub fn read(&self) -> ReadableRef<Signal<T>> {
        self.inner.read()
    }

    pub fn send(&self, action: T::Action) {
        self.co.send(action);
    }
}

#[must_use]
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
