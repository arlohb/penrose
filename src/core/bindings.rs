//! Setting up and responding to user defined key/mouse bindings
use crate::{
    core::{data_types::Point, manager::WindowManager, xconnection::Xid},
    PenroseError, Result,
};

#[cfg(feature = "keysyms")]
use penrose_keysyms::XKeySym;

use std::{collections::HashMap, convert::TryFrom};

use strum::EnumIter;

/// Some action to be run by a user key binding
pub type KeyEventHandler<X> = Box<dyn FnMut(&mut WindowManager<X>) -> Result<()>>;

/// An action to be run in response to a mouse event
pub type MouseEventHandler<X> = Box<dyn FnMut(&mut WindowManager<X>, &MouseEvent) -> Result<()>>;

/// User defined key bindings
pub type KeyBindings<X> = HashMap<KeyCode, KeyEventHandler<X>>;

/// User defined mouse bindings
pub type MouseBindings<X> = HashMap<(MouseEventKind, MouseState), MouseEventHandler<X>>;

pub(crate) type CodeMap = HashMap<String, u8>;

/// Abstraction layer for working with key presses
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyPress {
    /// A raw character key
    Utf8(String),
    /// Return / enter key
    Return,
    /// Escape
    Escape,
    /// Tab
    Tab,
    /// Backspace
    Backspace,
    /// Delete
    Delete,
    /// PageUp
    PageUp,
    /// PageDown
    PageDown,
    /// Up
    Up,
    /// Down
    Down,
    /// Left
    Left,
    /// Right
    Right,
}

#[cfg(feature = "keysyms")]
impl TryFrom<XKeySym> for KeyPress {
    type Error = PenroseError;

    fn try_from(s: XKeySym) -> std::result::Result<KeyPress, PenroseError> {
        Ok(match s {
            XKeySym::XK_Return | XKeySym::XK_KP_Enter | XKeySym::XK_ISO_Enter => KeyPress::Return,
            XKeySym::XK_Escape => KeyPress::Escape,
            XKeySym::XK_Tab | XKeySym::XK_ISO_Left_Tab | XKeySym::XK_KP_Tab => KeyPress::Tab,
            XKeySym::XK_BackSpace => KeyPress::Backspace,
            XKeySym::XK_Delete | XKeySym::XK_KP_Delete => KeyPress::Delete,
            XKeySym::XK_Page_Up | XKeySym::XK_KP_Page_Up => KeyPress::PageUp,
            XKeySym::XK_Page_Down | XKeySym::XK_KP_Page_Down => KeyPress::PageDown,
            XKeySym::XK_Up | XKeySym::XK_KP_Up => KeyPress::Up,
            XKeySym::XK_Down | XKeySym::XK_KP_Down => KeyPress::Down,
            XKeySym::XK_Left | XKeySym::XK_KP_Left => KeyPress::Left,
            XKeySym::XK_Right | XKeySym::XK_KP_Right => KeyPress::Right,
            s => KeyPress::Utf8(s.as_utf8_string()?),
        })
    }
}

/// A u16 X key-code bitmask
pub type KeyCodeMask = u16;

/// A u8 X key-code enum value
pub type KeyCodeValue = u8;

/// A key press and held modifiers
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct KeyCode {
    /// The held modifier mask
    pub mask: KeyCodeMask,
    /// The key code that was held
    pub code: KeyCodeValue,
}

impl KeyCode {
    /// Create a new [KeyCode] from this one that removes the given mask
    pub fn ignoring_modifier(&self, mask: KeyCodeMask) -> KeyCode {
        KeyCode {
            mask: self.mask & !mask,
            code: self.code,
        }
    }
}

/// Known mouse buttons for binding actions
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MouseButton {
    /// 1
    Left,
    /// 2
    Middle,
    /// 3
    Right,
    /// 4
    ScrollUp,
    /// 5
    ScrollDown,
}

impl From<MouseButton> for u8 {
    fn from(b: MouseButton) -> u8 {
        match b {
            MouseButton::Left => 1,
            MouseButton::Middle => 2,
            MouseButton::Right => 3,
            MouseButton::ScrollUp => 4,
            MouseButton::ScrollDown => 5,
        }
    }
}

/// Known modifier keys for bindings
#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum ModifierKey {
    /// Control
    Ctrl,
    /// Alt
    Alt,
    /// Shift
    Shift,
    /// Meta / super / windows
    Meta,
}

impl TryFrom<&str> for ModifierKey {
    type Error = PenroseError;

    fn try_from(s: &str) -> Result<Self> {
        match s {
            "C" => Ok(Self::Ctrl),
            "A" => Ok(Self::Alt),
            "S" => Ok(Self::Shift),
            "M" => Ok(Self::Meta),
            _ => Err(PenroseError::UnknownModifier(s.into())),
        }
    }
}

/// A mouse state specification indicating the button and modifiers held
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct MouseState {
    /// The [MouseButton] being held
    pub button: MouseButton,
    /// All [ModifierKey]s being held
    pub modifiers: Vec<ModifierKey>,
}

impl MouseState {
    /// Construct a new MouseState
    pub fn new(button: MouseButton, mut modifiers: Vec<ModifierKey>) -> Self {
        modifiers.sort();
        Self { button, modifiers }
    }
}

/// The types of mouse events represented by a MouseEvent
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MouseEventKind {
    /// A button was pressed
    Press,
    /// A button was released
    Release,
    /// The mouse was moved while a button was held
    Motion,
}

/// A mouse movement or button event
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MouseEvent {
    /// The ID of the window that was contained the click
    pub id: Xid,
    /// Absolute coordinate of the event
    pub rpt: Point,
    /// Coordinate of the event relative to top-left of the window itself
    pub wpt: Point,
    /// The modifier and button code that was received
    pub state: MouseState,
    /// Was this press, release or motion?
    pub kind: MouseEventKind,
}

impl MouseEvent {
    /// Construct a new [MouseEvent] from raw data
    pub fn new(
        id: Xid,
        rx: i16,
        ry: i16,
        ex: i16,
        ey: i16,
        state: MouseState,
        kind: MouseEventKind,
    ) -> Self {
        MouseEvent {
            id,
            rpt: Point::new(rx as u32, ry as u32),
            wpt: Point::new(ex as u32, ey as u32),
            state,
            kind,
        }
    }
}
