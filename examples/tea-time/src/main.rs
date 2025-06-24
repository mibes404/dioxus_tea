//! Tea-making application that demonstrates the use of the `TeaModel` trait.

#![warn(clippy::pedantic)]

use crate::model::{AppState, AppStatusUpdate, MakeTeaError, Status};
use dioxus::prelude::*;
use dioxus_tea::TeaModel;

fn main() {
    launch(rsx_components::App);
}

/// Define the application state and actions for the tea-making application.
mod model {
    use std::fmt::Display;

    #[derive(Default, Clone, PartialEq)]
    pub enum Status {
        #[default]
        FetchingCup,
        EmptyCup,
        TeaBag(TeaType),
        Water(u8),
        TeaReady,
        Error(MakeTeaError),
    }

    impl Display for Status {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let status_message = match self {
                Status::FetchingCup => "Fetching a cup...",
                Status::EmptyCup => "Empty cup. Add a tea bag.",
                Status::TeaBag(tea_type) => &format!("Tea bag added: {tea_type}"),
                Status::Water(temperature) => {
                    &format!("Water added at {temperature}°C. Waiting for tea to brew...")
                }
                Status::TeaReady => "Tea is ready!",
                Status::Error(error) => &format!("Error: {error}"),
            };
            write!(f, "{status_message}")
        }
    }

    #[derive(Clone, PartialEq)]
    pub enum TeaType {
        Black,
        Green,
        White,
        Oolong,
    }

    impl TeaType {
        pub fn temp_range(&self) -> (u8, u8) {
            match self {
                TeaType::Black => (100, 100),
                TeaType::Green => (70, 79),
                TeaType::White => (70, 82),
                TeaType::Oolong => (85, 93),
            }
        }
    }

    impl Display for TeaType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let tea_name = match self {
                TeaType::Black => "Black",
                TeaType::Green => "Green",
                TeaType::White => "White",
                TeaType::Oolong => "Oolong",
            };
            write!(f, "{tea_name}")
        }
    }

    #[derive(Clone, PartialEq)]
    pub enum MakeTeaError {
        MissingTeaBag,
        WaterTooHot,
        WaterTooCold,
        MissingWater,
        CupNotEmpty,
    }

    impl Display for MakeTeaError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let error_message = match self {
                MakeTeaError::MissingTeaBag => "No tea bag added",
                MakeTeaError::WaterTooHot => "Water is too hot",
                MakeTeaError::WaterTooCold => "Water is too cold",
                MakeTeaError::MissingWater => "No water added",
                MakeTeaError::CupNotEmpty => "The cup is not empty",
            };
            write!(f, "{error_message}")
        }
    }

    #[derive(Default, Clone, PartialEq)]
    pub struct AppState {
        pub status: Status,
    }

    pub enum AppStatusUpdate {
        CupFetched,
        AddWater(u8),
        AddTeaBag(TeaType),
        Done,
    }
}

// Implement the `TeaModel` trait for `AppState` to handle actions and update the state accordingly.
impl TeaModel for AppState {
    type Action = AppStatusUpdate;

    fn update(action: Self::Action, mut writer: Write<Self>) {
        match action {
            AppStatusUpdate::CupFetched => {
                // when the cup is fetched, we start with an empty cup
                writer.status = Status::EmptyCup;
            }
            AppStatusUpdate::AddWater(temperature) => {
                let Status::TeaBag(tea_type) = &writer.status else {
                    // if there is no tea bag, we can't make tea
                    writer.status = Status::Error(MakeTeaError::MissingTeaBag);
                    return;
                };

                // check that the water temperature is within a valid range for a good cup of tea
                let (low, high) = tea_type.temp_range();
                if temperature < low {
                    writer.status = Status::Error(MakeTeaError::WaterTooCold);
                } else if temperature > high {
                    writer.status = Status::Error(MakeTeaError::WaterTooHot);
                } else {
                    writer.status = Status::Water(temperature);
                }
            }
            AppStatusUpdate::AddTeaBag(tea_type) => {
                if matches!(writer.status, Status::EmptyCup) {
                    writer.status = Status::TeaBag(tea_type);
                } else {
                    // if we are not in a state to add a tea bag, we can't add it
                    writer.status = Status::Error(MakeTeaError::CupNotEmpty);
                }
            }
            AppStatusUpdate::Done => {
                if let Status::Water(_) = &writer.status {
                    // if we have water and a tea bag, we can finish making tea
                    writer.status = Status::TeaReady;
                } else {
                    // if we are not in a state to make tea, we can't finish
                    writer.status = Status::Error(MakeTeaError::MissingWater);
                }
            }
        }
    }
}

mod rsx_components {
    use super::model::{AppState, AppStatusUpdate, Status, TeaType};
    use dioxus::prelude::*;
    use dioxus_sdk::time::sleep;
    use dioxus_tea::{TeaModelSignal, use_tea_model};
    use std::time::Duration;

    const MAIN_CSS: Asset = asset!("/assets/main.css");
    const FAVICON: Asset = asset!("/assets/favicon.ico");

    #[component]
    pub(super) fn App() -> Element {
        // Initialize the tea model and provide it to the context
        let app_state = use_tea_model::<AppState>();
        use_context_provider(|| app_state);

        rsx! {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            DemoApp {}
        }
    }

    #[component]
    fn DemoApp() -> Element {
        // Demonstrates how to use the tea model from the context. Alternatively, you could pass the
        // `TeaModelSignal<AppState>` directly to this component.
        let app_state = use_context::<TeaModelSignal<AppState>>();

        use_hook(|| {
            // run this code once when the component mounts
            spawn(async move {
                // Simulate fetching a cup
                sleep(Duration::from_secs(1)).await;
                app_state.send(AppStatusUpdate::CupFetched);
            });
        });

        use_effect(move || {
            let current_status = &app_state.read().status;
            if matches!(current_status, Status::Water(_)) {
                spawn(async move {
                    // wait for 2 seconds to simulate making tea
                    sleep(Duration::from_secs(2)).await;
                    app_state.send(AppStatusUpdate::Done);
                });
            }
        });

        let app_state_r = app_state.read();
        let message = app_state_r.status.to_string();

        rsx! {
            div {
                id: "title",
                div { class: "heading", "Tea Time 🫖" }
                div { "{message}" }
            }

            TeaOptions {
                app_state
            }
        }
    }

    #[component]
    fn TeaOptions(app_state: TeaModelSignal<AppState>) -> Element {
        let app_state_r = app_state.read();

        rsx! {
            if matches!(app_state_r.status, Status::EmptyCup) {
                div {
                    class: "tea-options",
                        button {
                            onclick: move |_| app_state.send(AppStatusUpdate::AddTeaBag(TeaType::Black)),
                            "Add Black Tea Bag"
                        }
                        button {
                            onclick: move |_| app_state.send(AppStatusUpdate::AddTeaBag(TeaType::Green)),
                            "Add Green Tea Bag"
                        }
                        button {
                            onclick: move |_| app_state.send(AppStatusUpdate::AddTeaBag(TeaType::White)),
                            "Add White Tea Bag"
                        }
                        button {
                            onclick: move |_| app_state.send(AppStatusUpdate::AddTeaBag(TeaType::Oolong)),
                            "Add Oolong Tea Bag"
                        }
                    }
            } else if matches!(app_state_r.status, Status::TeaBag(_)) {
                div {
                    class: "tea-options",
                    button {
                        onclick: move |_| app_state.send(AppStatusUpdate::AddWater(100)),
                        "Add Water (100°C)"
                    }
                    button {
                        onclick: move |_| app_state.send(AppStatusUpdate::AddWater(90)),
                        "Add Water (90°C)"
                    }
                    button {
                        onclick: move |_| app_state.send(AppStatusUpdate::AddWater(80)),
                        "Add Water (80°C)"
                    }
                    button {
                        onclick: move |_| app_state.send(AppStatusUpdate::AddWater(70)),
                        "Add Water (70°C)"
                    }
                }
            } else if matches!(app_state_r.status, Status::Error(_) | Status::TeaReady) {
                div {
                    class: "tea-options",
                    button {
                        onclick: move |_| app_state.send(AppStatusUpdate::CupFetched),
                        "Try again"
                    }
                }
            }
        }
    }
}
