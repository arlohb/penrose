//! User facing configuration of the penrose [WindowManager][crate::core::manager::WindowManager].

use crate::{
    core::{layouts::side_stack, Layout, LayoutConf},
    draw::Color,
    PenroseError,
};

/// The main user facing configuration details.
///
/// See [ConfigBuilder] for details of what can be overwritten.
///
/// # Example
/// ```
/// use penrose::{Config, draw::Color};
/// use std::convert::TryFrom;
///
/// let config = Config::default();
///
/// assert_eq!(config.border_px(), &2);
/// assert_eq!(config.focused_border(), &Color::try_from("#cc241d").unwrap());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    /// the initial available workspaces.
    ///
    /// # Constraints
    /// You must provide at least one workspace per screen
    pub workspaces: Vec<String>,

    /// the window classes that will always be considered floating
    pub floating_classes: Vec<String>,

    /// the [Layout] functions to be used by each [Workspace][crate::core::workspace::Workspace]
    ///
    /// # Constraints
    /// You must provide at least one layout function
    pub layouts: Vec<Layout>,

    /// the focused border color as a hex literal
    pub focused_border: Color,
    /// the unfocused border color as a hex literal
    pub unfocused_border: Color,
    /// the border width of each window in pixels
    pub border_px: u32,
    /// the gap between tiled windows in pixels
    pub gap_px: u32,
    /// the percentage of the screen to grow the main region by when incrementing
    pub main_ratio_step: f32,
    /// whether or not space should be reserved for a status bar
    pub show_bar: bool,
    /// whether or not the reserved space for a status bar is at the top of the screen
    pub top_bar: bool,
    /// the height of the space to be reserved for a status bar in pixels
    pub bar_height: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            workspaces: vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            floating_classes: vec!["dmenu", "dunst"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            layouts: vec![
                Layout::new("[side]", LayoutConf::default(), side_stack, 1, 0.6),
                Layout::floating("[----]"),
            ],
            focused_border: "#cc241d".try_into().unwrap(),
            unfocused_border: "#3c3836".try_into().unwrap(),
            border_px: 2,
            gap_px: 5,
            main_ratio_step: 0.05,
            show_bar: true,
            top_bar: true,
            bar_height: 18,
        }
    }
}

impl Config {
    /// Create a range from 1 -> n_workspaces for use in keybindings
    pub fn ws_range(&self) -> std::ops::Range<usize> {
        1..(self.workspaces.len() + 1)
    }

    /// Validates the configuration and returns an error if it is invalid
    pub fn validate(self) -> Result<Self, PenroseError> {
        if self.workspaces.is_empty() {
            return Err(PenroseError::InvalidConfig(
                "workspaces must not be empty".to_string(),
            ));
        }

        if self.layouts.is_empty() {
            return Err(PenroseError::InvalidConfig(
                "layouts must not be empty".to_string(),
            ));
        }

        if !(0.0..=1.0).contains(&self.main_ratio_step) {
            return Err(PenroseError::InvalidConfig(
                "main_ratio_step must be between 0.0 and 1.0".to_string(),
            ));
        }

        Ok(self)
    }
}
