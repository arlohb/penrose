use crate::core::{
    client::Client,
    data_types::{Change, Region, ResizeAction},
    xconnection::Xid,
};

use std::{cmp, fmt};

/// When and how a Layout should be applied.
///
/// The default layout config that only triggers when clients are added / removed and follows user
/// defined config options.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LayoutConf {
    /// If true, this layout function will not be called to produce resize actions
    pub floating: bool,
    /// Should gaps be dropped regardless of config
    pub gapless: bool,
    /// Should this layout be triggered by window focus as well as add/remove client
    pub follow_focus: bool,
    /// Should cycling clients wrap at the first and last client?
    pub allow_wrapping: bool,
}

impl Default for LayoutConf {
    fn default() -> Self {
        Self {
            floating: false,
            gapless: false,
            follow_focus: false,
            allow_wrapping: true,
        }
    }
}

/// A function that can be used to position Clients on a Workspace.
///
/// Will be called with the current client list, the active client ID (if there is one), the size
/// of the screen that the workspace is shown on and the current values of n_main and ratio for
/// this layout.
pub type LayoutFunc = fn(&[&Client], Option<Xid>, &Region, u32, f32) -> Vec<ResizeAction>;

/// Responsible for arranging Clients within a Workspace.
///
/// A Layout is primarily a function that will be passed an array of Clients to apply resize actions
/// to. Only clients that should be tiled for the current monitor will be passed so no checks are
/// required to see if each client should be handled. The region passed to the layout function
/// represents the current screen dimensions that can be utilised and gaps/borders will be added to
/// each client by the WindowManager itself so there is no need to handle that in the layouts
/// themselves.
///
/// Layouts are expected to have a "main area" that holds the clients with primary focus and any
/// number of secondary areas for the remaining clients to be tiled.
///
/// The user can increase/decrease the size of the main area by setting `ratio` via key bindings
/// which should determine the relative size of the main area compared to other cliens.  Layouts
/// maintain their own state for number of clients in the main area and ratio which will be passed
/// through to the layout function when it is called.
#[derive(Clone)]
pub struct Layout {
    pub(crate) conf: LayoutConf,
    pub(crate) symbol: String,
    max_main: u32,
    ratio: f32,
    f: Option<LayoutFunc>,
}

impl cmp::PartialEq<Layout> for Layout {
    // Ignoring 'f'
    fn eq(&self, other: &Layout) -> bool {
        self.conf == other.conf
            && self.symbol == other.symbol
            && self.max_main == other.max_main
            && self.ratio == other.ratio
    }
}

impl fmt::Debug for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Layout")
            .field("kind", &self.conf)
            .field("symbol", &self.symbol)
            .field("max_main", &self.max_main)
            .field("ratio", &self.ratio)
            .field("f", &stringify!(&self.f))
            .finish()
    }
}

impl Layout {
    /// Create a new Layout for a specific monitor
    pub fn new(
        symbol: impl Into<String>,
        conf: LayoutConf,
        f: LayoutFunc,
        max_main: u32,
        ratio: f32,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            conf,
            max_main,
            ratio,
            f: Some(f),
        }
    }

    /// A default floating layout that will not attempt to manage windows
    pub fn floating(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            conf: LayoutConf {
                floating: true,
                gapless: false,
                follow_focus: false,
                allow_wrapping: true,
            },
            f: Some(super::layouts::floating),
            max_main: 1,
            ratio: 1.0,
        }
    }

    /// Apply the layout function held by this `Layout` using the current max_main and ratio
    pub fn arrange(
        &self,
        clients: &[&Client],
        focused: Option<Xid>,
        r: &Region,
    ) -> Vec<ResizeAction> {
        (self.f.expect("missing layout function"))(clients, focused, r, self.max_main, self.ratio)
    }

    /// Increase/decrease the number of clients in the main area by 1
    pub fn update_max_main(&mut self, change: Change) {
        match change {
            Change::More => self.max_main += 1,
            Change::Less => {
                if self.max_main > 0 {
                    self.max_main -= 1;
                }
            }
        }
    }

    /// Increase/decrease the size of the main area relative to secondary.
    /// (clamps at 1.0 and 0.0 respectively)
    pub fn update_main_ratio(&mut self, change: Change, step: f32) {
        match change {
            Change::More => self.ratio += step,
            Change::Less => self.ratio -= step,
        }

        if self.ratio < 0.0 {
            self.ratio = 0.0
        } else if self.ratio > 1.0 {
            self.ratio = 1.0;
        }
    }
}

/*
 * Utility functions for simplifying writing layouts
 */

/// number of clients for the main area vs secondary
pub fn client_breakdown<T>(clients: &[T], n_main: u32) -> (u32, u32) {
    let n = clients.len() as u32;
    if n <= n_main {
        (n, 0)
    } else {
        (n_main, n - n_main)
    }
}
