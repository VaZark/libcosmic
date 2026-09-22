// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Presentation model for controls that can be composed into a COSMIC Quick Settings surface.
//!
//! The types in this module are intentionally semantic rather than widget-based. Applications
//! keep ownership of their authoritative state and expose only the small projection needed by
//! Quick Settings. A host such as `cosmic-panel` can then choose the concrete layout and widgets.
//!
//! The model is serializable so it can cross the applet/process boundary without exposing the
//! application's complete internal state.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Stable identifier for a Quick Settings control.
///
/// IDs are intended to be persisted by the host for ordering and visibility, so applications
/// should not derive them from an item's position in the model.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(String);

impl Id {
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A snapshot of the controls an application exposes to Quick Settings.
///
/// The host owns layout and rendering. The application only describes the controls and their
/// current presentation state.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Model {
    items: Vec<Control>,
}

impl Model {
    #[must_use]
    pub fn new(items: Vec<Control>) -> Self {
        Self { items }
    }

    #[must_use]
    pub fn items(&self) -> &[Control] {
        &self.items
    }

    pub fn push(&mut self, control: impl Into<Control>) {
        self.items.push(control.into());
    }

    #[must_use]
    pub fn get(&self, id: &Id) -> Option<&Control> {
        self.items.iter().find(|control| control.id() == id)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// A semantic Quick Settings control.
///
/// Concrete geometry is deliberately not part of this API. The host is responsible for mapping a
/// control kind to its grid span and for choosing compact/expanded presentation based on available
/// space.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Control {
    Toggle(Toggle),
    Slider(Slider),
    Select(Select),
    WithDropdown(WithDropdown),
}

impl Control {
    #[must_use]
    pub fn id(&self) -> &Id {
        match self {
            Self::Toggle(control) => &control.id,
            Self::Slider(control) => &control.id,
            Self::Select(control) => &control.id,
            Self::WithDropdown(control) => control.primary.id(),
        }
    }

    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Toggle(control) => &control.label,
            Self::Slider(control) => &control.label,
            Self::Select(control) => &control.label,
            Self::WithDropdown(control) => control.primary.label(),
        }
    }
}

/// Common metadata for a binary control.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Toggle {
    pub id: Id,
    pub label: String,
    pub icon: Option<String>,
    pub value: bool,
    pub enabled: bool,
}

impl Toggle {
    #[must_use]
    pub fn new(id: impl Into<Id>, label: impl Into<String>, value: bool) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            value,
            enabled: true,
        }
    }

    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl From<Toggle> for Control {
    fn from(value: Toggle) -> Self {
        Self::Toggle(value)
    }
}

/// Numeric control represented by a slider.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Slider {
    pub id: Id,
    pub label: String,
    pub icon: Option<String>,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: Option<f64>,
    pub enabled: bool,
}

impl Slider {
    #[must_use]
    pub fn new(
        id: impl Into<Id>,
        label: impl Into<String>,
        value: f64,
        min: f64,
        max: f64,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            value,
            min,
            max,
            step: None,
            enabled: true,
        }
    }

    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    #[must_use]
    pub fn step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl From<Slider> for Control {
    fn from(value: Slider) -> Self {
        Self::Slider(value)
    }
}

/// One selectable value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectOption {
    pub id: String,
    pub label: String,
}

impl SelectOption {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}

/// A control that selects one value from a set of options.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Select {
    pub id: Id,
    pub label: String,
    pub icon: Option<String>,
    pub selected: Option<String>,
    pub options: Vec<SelectOption>,
    pub enabled: bool,
}

impl Select {
    #[must_use]
    pub fn new(
        id: impl Into<Id>,
        label: impl Into<String>,
        selected: Option<String>,
        options: Vec<SelectOption>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            selected,
            options,
            enabled: true,
        }
    }

    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl From<Select> for Control {
    fn from(value: Select) -> Self {
        Self::Select(value)
    }
}

/// A primary control that can reveal additional semantic controls.
///
/// This supports controls such as a Wi-Fi toggle that can expand into a list of network-related
/// controls without giving the applet responsibility for popup geometry or rendering.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WithDropdown {
    pub primary: Box<Control>,
    pub items: Vec<Control>,
    pub expanded: bool,
}

impl WithDropdown {
    #[must_use]
    pub fn new(primary: impl Into<Control>, items: Vec<Control>) -> Self {
        Self {
            primary: Box::new(primary.into()),
            items,
            expanded: false,
        }
    }

    #[must_use]
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }
}

impl From<WithDropdown> for Control {
    fn from(value: WithDropdown) -> Self {
        Self::WithDropdown(value)
    }
}

/// User intent sent from a Quick Settings host back to the application.
///
/// The application remains authoritative: hosts should update their rendered state from a new
/// model snapshot rather than assuming that an action succeeded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Action {
    Activate,
    SetToggle(bool),
    SetValue(f64),
    Select(String),
    SetExpanded(bool),
}

/// A Quick Settings action targeted at one stable control ID.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: Id,
    pub action: Action,
}

impl Event {
    #[must_use]
    pub fn new(id: impl Into<Id>, action: Action) -> Self {
        Self {
            id: id.into(),
            action,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_looks_up_controls_by_stable_id() {
        let mut model = Model::default();
        model.push(Toggle::new("wifi", "Wi-Fi", true));
        model.push(Slider::new("volume", "Volume", 0.7, 0.0, 1.0));

        assert_eq!(model.get(&Id::from("wifi")).map(Control::label), Some("Wi-Fi"));
        assert_eq!(
            model.get(&Id::from("volume")).map(Control::label),
            Some("Volume")
        );
        assert!(model.get(&Id::from("missing")).is_none());
    }

    #[test]
    fn dropdown_uses_primary_control_identity() {
        let control = Control::from(WithDropdown::new(
            Toggle::new("wifi", "Wi-Fi", true),
            vec![Select::new(
                "wifi-network",
                "Network",
                Some("home".into()),
                vec![SelectOption::new("home", "Home")],
            )
            .into()],
        ));

        assert_eq!(control.id().as_str(), "wifi");
        assert_eq!(control.label(), "Wi-Fi");
    }
}
