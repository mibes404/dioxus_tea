# Implementation of The Elm Architecture (TEA) in Dioxus

People may also know this as the "Model-Update-View" architecture, or redux-like architecture.

## Usage

Include the dependency in your `Cargo.toml`:

```toml
dioxus-tea = { git = "https://github.com/mibes404/dioxus_tea.git" }
```

## Example of using the Dioxus "The Elm Architecture" (TEA) Model

A complete example can be found in the `examples/tea-time` directory.

Usage:

```rust, nocompile
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

impl TeaModel for AppState {
    type Action = AppStatusUpdate;

    fn update(&mut self, action: Self::Action) {
        match action {
           // handle actions and update the state accordingly
           AppStatusUpdate::CupFetched => {
                // when the cup is fetched, we start with an empty cup
                self.status = Status::EmptyCup;
            }
            // other actions
        }
    }   
}

#[component]
 pub fn App() -> Element {
    let app_state = use_tea_model::<AppState>();
    app_state.send(AppStatusUpdate::CupFetched);
}
```

### Run the example

```bash
cd examples/tea-time
dx serve --platform web
```