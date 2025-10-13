use dioxus::prelude::*;
use rust_extensions::AsStr;
use std::marker::PhantomData;

use crate::validators::*;

pub struct InputValueComponent<'s, TValue: 'static + AsStr + ValueValidator> {
    caption: &'s str,
    value: Option<&'s TValue>,
    read_only: bool,
    on_input: Option<EventHandler<String>>,
    on_press_enter: Option<EventHandler<()>>,
    ph: PhantomData<TValue>,
}

impl<'s, TValue: 'static + AsStr + ValueValidator> InputValueComponent<'s, TValue> {
    pub fn new(caption: &'s str) -> Self {
        Self {
            caption,
            value: None,
            read_only: false,
            on_input: None,
            on_press_enter: None,
            ph: Default::default(),
        }
    }

    pub fn set_value(mut self, value: &'s TValue) -> Self {
        self.value = Some(value);
        self
    }

    pub fn set_read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn on_input(mut self, on_input: impl Into<EventHandler<String>>) -> Self {
        self.on_input = Some(on_input.into());
        self
    }

    pub fn on_press_enter(mut self, on_input: impl Into<EventHandler<()>>) -> Self {
        self.on_press_enter = Some(on_input.into());
        self
    }

    pub fn render(self) -> Element {
        let mut style = if self.read_only {
            "color: lightgray;font-weight: bold;"
        } else {
            ""
        };

        let mut validation_error_class = "";

        let mut highlight_red = false;
        let mut validation_message = "";
        let value_as_str = match self.value {
            Some(value) => {
                if let Err(reason) = value.validate_value() {
                    match reason {
                        ValueValidationResult::Empty => {}
                        ValueValidationResult::IllegalChars => {
                            highlight_red = true;
                            validation_message = "invalid value";
                        }
                        ValueValidationResult::MinValueViolation => {
                            highlight_red = true;
                            validation_message = "min value violation";
                        }
                        ValueValidationResult::MaxValueViolation => {
                            highlight_red = true;
                            validation_message = "max value violation";
                        }
                    }
                }
                Some(value.as_str())
            }
            None => None,
        };

        if highlight_red {
            style = "color: red";
            validation_error_class = "edit-wrapper-validation-error";
        }

        let on_enter_pressed = self.on_press_enter;

        let validation_message = if validation_message.len() > 0 {
            rsx! {
                div { class: "edit-validation-err-message", {validation_message} }
            }
        } else {
            rsx! {}
        };

        if let Some(on_input) = self.on_input {
            rsx! {
                div { class: "edit-wrapper {validation_error_class}",
                    label { {self.caption} }
                    {validation_message}

                    input {
                        class: "form-control",
                        r#type: "text",
                        style: "{style}",
                        value: value_as_str,
                        readonly: self.read_only,
                        oninput: move |e| {
                            let value = e.value();
                            on_input.call(value);
                        },

                        onkeyup: move |e| {
                            match e.key() {
                                Key::Enter => {
                                    if let Some(on_enter_pressed) = on_enter_pressed.as_ref() {
                                        on_enter_pressed.call(());
                                    }
                                }
                                _ => {}
                            }
                        },
                    }
                }
            }
        } else {
            rsx! {
                div { class: "edit-wrapper {validation_error_class}",
                    label { {self.caption} }

                    input {
                        class: "form-control",
                        r#type: "text",
                        style: "{style}",
                        value: value_as_str,
                        readonly: self.read_only,

                        onkeyup: move |e| {
                            match e.key() {
                                Key::Enter => {
                                    if let Some(on_enter_pressed) = on_enter_pressed.as_ref() {
                                        on_enter_pressed.call(());
                                    }
                                }
                                _ => {}
                            }
                        },
                    }
                }
            }
        }
    }
}
