# Implementation of [The Elm Architecture](https://guide.elm-lang.org/architecture/) (TEA) in Dioxus

People may also know this as the "Model-Update-View" architecture, or redux-like architecture.
The benefit of this architecture is that it allows you to manage your application state in a predictable way,
making it easier to reason about and test.

The model is built in such a way that you can use regular Rust unit tests to test your application logic, i.e. validate
the `update` function of your model.

## Usage

Include the dependency in your `Cargo.toml`:

```toml
dioxus-tea = "0.1"
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

A unit test could look like this:

```rust, nocompile
#[test]
fn confirm_that_we_can_only_add_a_tea_bag_when_we_have_a_cup() {
    let mut app_state = AppState::default();
    app_state.update(AppStatusUpdate::AddTeaBag(TeaType::Black));
    assert_eq!(
        app_state.status,
        Status::Error(MakeTeaError::NoCup),
        "Cannot add a tea bag without a cup"
    );
}
```

### Run the example

```bash
dx serve --platform web --example tea-time --package dioxus-tea
```