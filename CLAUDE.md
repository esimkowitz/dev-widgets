# dev-widgets

Dioxus 0.7 app (desktop via Tauri, web via WASM) providing developer utilities organized by category.

## Code Style

- Add `#![allow(non_snake_case)]` at module level for PascalCase component names
- Components: PascalCase (`NumberBaseConverter`)
- Signals/variables: snake_case (`format_state`)
- Constants: UPPER_SNAKE_CASE (`WIDGET_ENTRY`, `ICON`)

## Widget Structure

Every widget follows this pattern:

```rust
pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Full Title",
    short_title: "Short",
    description: "What it does",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<BsCalculator> = WidgetIcon { icon: BsCalculator };

#[component]
pub fn MyWidget() -> Element {
    rsx! { /* ... */ }
}
```

Categories (converter, generator, encoder_decoder, media) implement `WidgetRoute` trait for route discovery.

## Dioxus Patterns

**State management (prefer in this order):**

- `use_signal()` - local component state
- `use_context()` - shared state within widget tree
- `use_persistent()` - localStorage-backed (dioxus-sdk)

**Reading/writing signals:**

```rust
let value = *signal.read();        // read
signal.set(new_value);             // write
signal.with_mut(|v| v.field = x);  // mutate in place
```

**Shared state pattern:**

```rust
// Parent provides
use_context_provider(|| Signal::new(SharedState::default()));
// Children consume
let state = use_context::<Signal<SharedState>>();
```

## Components

Reusable inputs in `src/components/inputs.rs`:

- `TextInput`, `TextAreaForm` - text fields with optional handlers
- `SelectForm<T>` - enum-based dropdown (T: SelectFormEnum)
- `SwitchInput` - toggle with label
- `NumberInput` - numeric with +/- buttons

## Development

```bash
npm run serve:web      # web with hot reload
npm run serve:desktop  # desktop with hot reload
dx build --platform web --release  # production build
```

## Styling

**Tailwind CSS v4** with **DaisyUI v5**. No `tailwind.config.*` file — plugins are defined directly in `src/main.css` using the v4 CSS-based configuration:

```css
@import "tailwindcss";
@plugin "@tailwindcss/typography";
@plugin "daisyui" { /* options */ }
```

Dark mode is handled by `public/js/darkmode.js`.

## Adding a Widget

1. Create file in appropriate category folder (e.g., `src/pages/converter/my_widget.rs`)
2. Define `WIDGET_ENTRY`, `ICON`, and component function
3. Add module declaration in category's `mod.rs`
4. Add route variant to category's `Route` enum with `#[route("/my-widget")]`
5. Implement match arm in `get_widget_entry()`
