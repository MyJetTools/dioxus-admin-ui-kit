# dioxus-admin-ui-kit

UI component library for Dioxus 0.7 fullstack admin applications. Provides typed input components, table rendering, enum selectors, and validation helpers.

## Install

```toml
[dependencies]
dioxus-admin-ui-kit = { tag = "0.7.0", git = "https://github.com/MyJetTools/dioxus-admin-ui-kit.git" }
```

---

## Components

### `InputValueComponent<T>` — typed text input with validation

Builder-style API for validated text inputs. `T` must implement `AsStr + ValueValidator`.

```rust
use dioxus_admin_ui_kit::components::InputValueComponent;
use dioxus_admin_ui_kit::types::InputValue;

let value: InputValue<f64> = InputValue::new(0.5);

InputValueComponent::new("Swap Long")
    .set_value(&cs_ra.swap_long)
    .on_input(move |v| cs.write().swap_long.set_string_value(v))
    .render()
```

**Methods:**
- `.set_value(&T)` — current value to display
- `.set_read_only(bool)` — disable editing
- `.on_input(FnMut(String))` — called on every keystroke
- `.on_press_enter(EventHandler<()>)` — called on Enter key
- `.render()` — produces `Element`

Shows validation error inline (red border + message) when value fails validation.

---

### `input_bool` — checkbox input

```rust
use dioxus_admin_ui_kit::components::input_bool;

input_bool("Is Active", cs_ra.is_active, EventHandler::new(move |v| {
    cs.write().is_active = v;
}))
```

---

### `select_enum_value` — enum dropdown (required value)

Requires `T: Eq + Clone + Default + AsStr + FromStr + EnumIterator`.

```rust
use dioxus_admin_ui_kit::components::select_enum_value;

select_enum_value(
    "Day of Week",
    &cs_ra.day_selector,
    EventHandler::new(move |v: DayOfWeek| cs.write().day = v),
)
```

---

### `select_enum_value_opt` — enum dropdown (optional value)

Adds a "Not selected" option. Uses `SelectEnumValueOpt<T>`.

```rust
use dioxus_admin_ui_kit::components::select_enum_value_opt;
use dioxus_admin_ui_kit::types::SelectEnumValueOpt;

select_enum_value_opt(
    "Side",
    &cs_ra.side_selector,
    EventHandler::new(move |v: Option<PositionSide>| {
        cs.write().side = SelectEnumValueOpt::new(v);
    }),
)
```

---

### `RenderTable<T>` — typed table renderer

Implement `TableItem` for your model, then use `RenderTable` to render it.

**Step 1: implement `TableItem`**

```rust
use dioxus_admin_ui_kit::components::{TableItem, ValueToRender};

struct SwapProfileRow {
    pub id: String,
    pub instruments_count: usize,
}

impl TableItem for SwapProfileRow {
    const HEADER: &'static [&'static str] = &["ID", "Instruments"];
    const COLUMNS_AMOUNT: usize = 2;

    fn get_value<'s>(&'s self, index: usize) -> ValueToRender<'s> {
        match index {
            0 => self.id.as_str().into(),
            1 => self.instruments_count.to_string().into(),
            _ => unreachable!(),
        }
    }
}
```

**Step 2: render**

```rust
use dioxus_admin_ui_kit::components::RenderTable;

// Simple table
RenderTable::new(items.iter())
    .with_class(&["table", "table-striped", "table-hover"])
    .with_wrapped_div("table-responsive")
    .render()

// Table with action column
RenderTable::new(items.iter())
    .with_class(&["table", "table-striped"])
    .render_line_with_actions(
        rsx! { th { "Actions" } },
        |item| rsx! {
            button {
                class: "btn btn-sm btn-primary",
                onclick: move |_| { /* edit */ },
                "Edit"
            }
        },
    )
```

**`ValueToRender` variants:**
- `&'s str` → zero-copy string
- `String` → owned string
- `Element` → custom cell content (buttons, badges, etc.)

---

## Types

### `InputValue<T>` — validated input state

Holds raw string from `<input>` and parses/validates to `T`.

```rust
use dioxus_admin_ui_kit::types::InputValue;

// Create
let v: InputValue<f64> = InputValue::new(3.14);
let v: InputValue<f64> = InputValue::from_str("3.14");

// Read
let parsed: Option<f64> = v.get_value();

// Write (from oninput handler)
v.set_string_value(new_string);

// Validate
match v.validate() {
    Ok(()) => { /* valid */ }
    Err(ValueValidationResult::Empty) => { /* empty */ }
    Err(ValueValidationResult::IllegalChars) => { /* parse failed */ }
    Err(ValueValidationResult::MinValueViolation) => { /* below min */ }
    Err(ValueValidationResult::MaxValueViolation) => { /* above max */ }
}

// With constraints
let v = InputValue::new(0.0_f64)
    .set_min_value_mut(0.0)
    .set_max_value_mut(1.0);
```

Implements `AsStr` + `ValueValidator` → pass directly to `InputValueComponent`.

---

### `InputValueOpt<T>` — optional validated input

Same as `InputValue<T>` but empty string = `None` (not an error).

```rust
use dioxus_admin_ui_kit::types::InputValueOpt;

let v: InputValueOpt<f64> = InputValueOpt::new(Some(3.14));
let v: InputValueOpt<f64> = InputValueOpt::new(None);

let parsed: Option<f64> = v.get_value();
v.set_value(new_string);

// Check if changed and valid (for enabling Save button)
match v.value_can_be_saved() {
    Some(true)  => { /* changed and valid */ }
    Some(false) => { /* changed but invalid */ }
    None        => { /* not changed */ }
}
```

---

### `SelectEnumValueOpt<T>` — optional enum selection state

```rust
use dioxus_admin_ui_kit::types::SelectEnumValueOpt;

let selector = SelectEnumValueOpt::new(Some(DayOfWeek::Monday));
let selector = SelectEnumValueOpt::<DayOfWeek>::new(None);

let current: Option<&DayOfWeek> = selector.get_value();
selector.set_value(Some(DayOfWeek::Tuesday));

// Require a selection (no null allowed)
let selector = SelectEnumValueOpt::<DayOfWeek>::default()
    .allow_null_result(false);
let valid: bool = selector.validate();
```

---

### `EnumIterator` trait — required for enum selectors

Implement on your enum to use with `select_enum_value` / `select_enum_value_opt`.

```rust
use dioxus_admin_ui_kit::types::EnumIterator;
use rust_extensions::AsStr;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DayOfWeek {
    #[default] Monday, Tuesday, Wednesday,
    Thursday, Friday, Saturday, Sunday,
}

impl AsStr for DayOfWeek {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Monday => "Monday", Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday", Self::Thursday => "Thursday",
            Self::Friday => "Friday", Self::Saturday => "Saturday",
            Self::Sunday => "Sunday",
        }
    }
}

impl FromStr for DayOfWeek {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        DayOfWeek::get_all().iter().find(|v| v.as_str() == s).copied().ok_or(())
    }
}

impl EnumIterator for DayOfWeek {
    type TItem = Self;
    fn get_value(&self) -> Self { *self }
    fn get_all() -> &'static [Self] {
        &[Self::Monday, Self::Tuesday, Self::Wednesday,
          Self::Thursday, Self::Friday, Self::Saturday, Self::Sunday]
    }
}
```

---

### `TimeOffset` — timezone offset enum

Pre-built enum covering all UTC offsets from -12:00 to +14:00.
Implements `EnumIterator`, `AsStr`, `FromStr`, `Serialize`/`Deserialize`.

```rust
use dioxus_admin_ui_kit::types::TimeOffset;

let offset = TimeOffset::Utc;
let offset = TimeOffset::try_from_str("+03:00").unwrap_or_default();
let minutes: i32 = offset.get_value_as_minutes(); // 180 for +03:00
let s: &str = offset.as_str();                    // "+03:00"
```

---

## Typical form state pattern

```rust
use dioxus_admin_ui_kit::types::InputValue;

#[derive(Clone)]
struct EditSwapEntryState {
    pub long: InputValue<f64>,
    pub short: InputValue<f64>,
    pub time: InputValue<String>,
    pub day: DayOfWeek,
}

impl EditSwapEntryState {
    pub fn is_valid(&self) -> bool {
        self.long.validate().is_ok()
            && self.short.validate().is_ok()
            && self.time.validate().is_ok()
    }
}

// In component:
let mut cs = use_signal(|| EditSwapEntryState { ... });
let cs_ra = cs.read();
let ok_disabled = !cs_ra.is_valid();

rsx! {
    {InputValueComponent::new("Long rate").set_value(&cs_ra.long)
        .on_input(move |v| cs.write().long.set_string_value(v)).render()}
    {InputValueComponent::new("Short rate").set_value(&cs_ra.short)
        .on_input(move |v| cs.write().short.set_string_value(v)).render()}

    button {
        class: "btn btn-success",
        disabled: ok_disabled,
        onclick: move |_| { /* save */ },
        "Save"
    }
}
```
