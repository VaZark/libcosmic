// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Quick Settings presentation data shared between an application and its shell host.
//!
//! Applications remain authoritative for their state. They expose only the small amount of data
//! required for the host to recreate a control using existing libcosmic widgets. User interaction
//! is sent back separately as a QuickSettingsEvent.

use serde::{Deserialize, Serialize};

/// Snapshot of the controls an application exposes to Quick Settings.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct QuickSettingsModel {
    pub controls: Vec<QuickSettingControl>,
}

/// One control exposed by an application.
///
/// Layout and concrete widget selection remain host responsibilities.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuickSettingControl {
    /// Stable identifier used to target actions and persist host-side configuration.
    pub id: String,
    pub label: String,
    /// Optional secondary text such as the active device or a short description.
    pub secondary: Option<String>,
    /// Optional symbolic icon name suitable for icon::from_name.
    pub icon: Option<String>,
    pub kind: QuickSettingKind,
}

/// Minimal state required for the host to recreate a control with existing libcosmic widgets.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QuickSettingKind {
    /// Stateless action rendered as a button.
    Button,

    /// Boolean state rendered with a toggler or equivalent compact representation.
    Toggle {
        value: bool,
    },

    /// Numeric state rendered with a slider.
    Slider {
        value: f64,
        min: f64,
        max: f64,
        /// Optional slider breakpoints, matching the capability already used by COSMIC audio.
        breakpoints: Option<Vec<f64>>,
    },

    /// One selected value from a fixed set of options.
    ///
    /// The host may render this using an existing segmented control, menu, or dropdown depending
    /// on the available space.
    SingleSelect {
        selected: Option<String>,
        options: Vec<QuickSettingOption>,
    },
}

/// One possible value of a SingleSelect control.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickSettingOption {
    pub id: String,
    pub label: String,
    pub secondary: Option<String>,
    pub icon: Option<String>,
}

/// User intent sent from the Quick Settings host back to the owning application.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QuickSettingsAction {
    /// Activate a stateless button.
    Activate,
    /// Set the value of a toggle.
    SetToggle(bool),
    /// Update a slider value while it is being manipulated.
    SetValue(f64),
    /// Notify the application that a slider interaction has finished.
    Release,
    /// Select one option of a single-select control by option ID.
    Select(String),
}

/// An action targeted at one Quick Settings control.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuickSettingsEvent {
    pub id: String,
    pub action: QuickSettingsAction,
}
