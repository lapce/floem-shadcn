//! Shared keymap types and builder for text editing components.
//!
//! This module provides common types and utilities used by both TextInput and TextArea.

use std::collections::HashMap;

use floem_editor_core::command::{EditCommand, MoveCommand};
use ui_events::keyboard::{Key, Modifiers, NamedKey};

/// Cursor blink interval in milliseconds
pub const CURSOR_BLINK_INTERVAL_MS: u64 = 500;

/// A command that can be executed on a text editor
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Edit(EditCommand),
    Move(MoveCommand),
    /// Select all text
    SelectAll,
    /// Copy selected text to clipboard
    Copy,
    /// Cut selected text to clipboard
    Cut,
    /// Paste text from clipboard
    Paste,
}

/// A key press with modifiers
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct KeyPress {
    pub key: Key,
    pub modifiers: Modifiers,
}

/// Builder for creating keymaps with various binding sets.
pub struct KeymapBuilder {
    keymaps: HashMap<KeyPress, Command>,
}

impl KeymapBuilder {
    /// Create a new empty keymap builder.
    pub fn new() -> Self {
        Self {
            keymaps: HashMap::new(),
        }
    }

    /// Add common bindings shared by all text editors.
    /// Includes: basic left/right navigation, backspace/delete, word navigation,
    /// word deletion, select all, clipboard operations.
    pub fn with_common_bindings(mut self) -> Self {
        #[cfg(target_os = "macos")]
        let cmd_or_ctrl = Modifiers::META;
        #[cfg(not(target_os = "macos"))]
        let cmd_or_ctrl = Modifiers::CONTROL;

        // =======================================================================
        // Basic navigation (left/right arrows)
        // =======================================================================
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::ArrowLeft),
                modifiers: Modifiers::default(),
            },
            Command::Move(MoveCommand::Left),
        );
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::ArrowRight),
                modifiers: Modifiers::default(),
            },
            Command::Move(MoveCommand::Right),
        );

        // =======================================================================
        // Word navigation (Alt/Option + arrows)
        // =======================================================================
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::ArrowLeft),
                modifiers: Modifiers::ALT,
            },
            Command::Move(MoveCommand::WordBackward),
        );
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::ArrowRight),
                modifiers: Modifiers::ALT,
            },
            Command::Move(MoveCommand::WordForward),
        );
        #[cfg(not(target_os = "macos"))]
        {
            self.keymaps.insert(
                KeyPress {
                    key: Key::Named(NamedKey::ArrowLeft),
                    modifiers: Modifiers::CONTROL,
                },
                Command::Move(MoveCommand::WordBackward),
            );
            self.keymaps.insert(
                KeyPress {
                    key: Key::Named(NamedKey::ArrowRight),
                    modifiers: Modifiers::CONTROL,
                },
                Command::Move(MoveCommand::WordForward),
            );
        }

        // =======================================================================
        // Basic editing
        // =======================================================================
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::Backspace),
                modifiers: Modifiers::default(),
            },
            Command::Edit(EditCommand::DeleteBackward),
        );
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::Delete),
                modifiers: Modifiers::default(),
            },
            Command::Edit(EditCommand::DeleteForward),
        );

        // =======================================================================
        // Word deletion
        // =======================================================================
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::Backspace),
                modifiers: Modifiers::ALT,
            },
            Command::Edit(EditCommand::DeleteWordBackward),
        );
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::Delete),
                modifiers: Modifiers::ALT,
            },
            Command::Edit(EditCommand::DeleteWordForward),
        );
        #[cfg(not(target_os = "macos"))]
        {
            self.keymaps.insert(
                KeyPress {
                    key: Key::Named(NamedKey::Backspace),
                    modifiers: Modifiers::CONTROL,
                },
                Command::Edit(EditCommand::DeleteWordBackward),
            );
            self.keymaps.insert(
                KeyPress {
                    key: Key::Named(NamedKey::Delete),
                    modifiers: Modifiers::CONTROL,
                },
                Command::Edit(EditCommand::DeleteWordForward),
            );
        }

        // =======================================================================
        // Line deletion (delete to beginning)
        // =======================================================================
        #[cfg(target_os = "macos")]
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::Backspace),
                modifiers: Modifiers::META,
            },
            Command::Edit(EditCommand::DeleteToBeginningOfLine),
        );

        // =======================================================================
        // Select All
        // =======================================================================
        self.keymaps.insert(
            KeyPress {
                key: Key::Character("a".into()),
                modifiers: cmd_or_ctrl,
            },
            Command::SelectAll,
        );

        // =======================================================================
        // Clipboard operations (Cmd/Ctrl+C, Cmd/Ctrl+X, Cmd/Ctrl+V)
        // =======================================================================
        self.keymaps.insert(
            KeyPress {
                key: Key::Character("c".into()),
                modifiers: cmd_or_ctrl,
            },
            Command::Copy,
        );
        self.keymaps.insert(
            KeyPress {
                key: Key::Character("x".into()),
                modifiers: cmd_or_ctrl,
            },
            Command::Cut,
        );
        self.keymaps.insert(
            KeyPress {
                key: Key::Character("v".into()),
                modifiers: cmd_or_ctrl,
            },
            Command::Paste,
        );

        self
    }

    /// Add single-line specific bindings.
    /// Home/End go to document start/end, Cmd+Left/Right go to document start/end.
    pub fn with_single_line_bindings(mut self) -> Self {
        // Home -> Document Start
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::Home),
                modifiers: Modifiers::default(),
            },
            Command::Move(MoveCommand::DocumentStart),
        );
        // End -> Document End
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::End),
                modifiers: Modifiers::default(),
            },
            Command::Move(MoveCommand::DocumentEnd),
        );
        // Cmd+Left (macOS) -> Document Start
        #[cfg(target_os = "macos")]
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::ArrowLeft),
                modifiers: Modifiers::META,
            },
            Command::Move(MoveCommand::DocumentStart),
        );
        // Cmd+Right (macOS) -> Document End
        #[cfg(target_os = "macos")]
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::ArrowRight),
                modifiers: Modifiers::META,
            },
            Command::Move(MoveCommand::DocumentEnd),
        );

        self
    }

    /// Add multi-line specific bindings.
    /// Includes: up/down arrows, Home/End for line start/end, Enter, Tab,
    /// document navigation with Cmd+Up/Down or Ctrl+Home/End.
    pub fn with_multi_line_bindings(mut self) -> Self {
        // =======================================================================
        // Up/Down navigation
        // =======================================================================
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::ArrowUp),
                modifiers: Modifiers::default(),
            },
            Command::Move(MoveCommand::Up),
        );
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::ArrowDown),
                modifiers: Modifiers::default(),
            },
            Command::Move(MoveCommand::Down),
        );

        // =======================================================================
        // Line start/end navigation
        // =======================================================================
        // Home -> Line Start
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::Home),
                modifiers: Modifiers::default(),
            },
            Command::Move(MoveCommand::LineStart),
        );
        // End -> Line End
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::End),
                modifiers: Modifiers::default(),
            },
            Command::Move(MoveCommand::LineEnd),
        );
        // Cmd+Left (macOS) -> Line Start
        #[cfg(target_os = "macos")]
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::ArrowLeft),
                modifiers: Modifiers::META,
            },
            Command::Move(MoveCommand::LineStart),
        );
        // Cmd+Right (macOS) -> Line End
        #[cfg(target_os = "macos")]
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::ArrowRight),
                modifiers: Modifiers::META,
            },
            Command::Move(MoveCommand::LineEnd),
        );

        // =======================================================================
        // Document start/end navigation
        // =======================================================================
        // Cmd+Up (macOS) -> Document Start
        #[cfg(target_os = "macos")]
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::ArrowUp),
                modifiers: Modifiers::META,
            },
            Command::Move(MoveCommand::DocumentStart),
        );
        // Ctrl+Home (non-macOS) -> Document Start
        #[cfg(not(target_os = "macos"))]
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::Home),
                modifiers: Modifiers::CONTROL,
            },
            Command::Move(MoveCommand::DocumentStart),
        );
        // Cmd+Down (macOS) -> Document End
        #[cfg(target_os = "macos")]
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::ArrowDown),
                modifiers: Modifiers::META,
            },
            Command::Move(MoveCommand::DocumentEnd),
        );
        // Ctrl+End (non-macOS) -> Document End
        #[cfg(not(target_os = "macos"))]
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::End),
                modifiers: Modifiers::CONTROL,
            },
            Command::Move(MoveCommand::DocumentEnd),
        );

        // =======================================================================
        // Enter and Tab
        // =======================================================================
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::Enter),
                modifiers: Modifiers::default(),
            },
            Command::Edit(EditCommand::InsertNewLine),
        );
        self.keymaps.insert(
            KeyPress {
                key: Key::Named(NamedKey::Tab),
                modifiers: Modifiers::default(),
            },
            Command::Edit(EditCommand::InsertTab),
        );

        self
    }

    /// Add Unix Emacs-style keybindings (Ctrl+letter).
    /// If `multiline` is true, includes Ctrl+N/P for up/down and uses LineStart/LineEnd.
    /// If `multiline` is false, uses DocumentStart/DocumentEnd.
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    pub fn with_emacs_bindings(mut self, multiline: bool) -> Self {
        // Ctrl+H -> Delete backward (backspace)
        self.keymaps.insert(
            KeyPress {
                key: Key::Character("h".into()),
                modifiers: Modifiers::CONTROL,
            },
            Command::Edit(EditCommand::DeleteBackward),
        );
        // Ctrl+D -> Delete forward
        self.keymaps.insert(
            KeyPress {
                key: Key::Character("d".into()),
                modifiers: Modifiers::CONTROL,
            },
            Command::Edit(EditCommand::DeleteForward),
        );
        // Ctrl+A -> Move to beginning of line
        self.keymaps.insert(
            KeyPress {
                key: Key::Character("a".into()),
                modifiers: Modifiers::CONTROL,
            },
            Command::Move(if multiline {
                MoveCommand::LineStart
            } else {
                MoveCommand::DocumentStart
            }),
        );
        // Ctrl+E -> Move to end of line
        self.keymaps.insert(
            KeyPress {
                key: Key::Character("e".into()),
                modifiers: Modifiers::CONTROL,
            },
            Command::Move(if multiline {
                MoveCommand::LineEnd
            } else {
                MoveCommand::DocumentEnd
            }),
        );
        // Ctrl+F -> Move forward (right)
        self.keymaps.insert(
            KeyPress {
                key: Key::Character("f".into()),
                modifiers: Modifiers::CONTROL,
            },
            Command::Move(MoveCommand::Right),
        );
        // Ctrl+B -> Move backward (left)
        self.keymaps.insert(
            KeyPress {
                key: Key::Character("b".into()),
                modifiers: Modifiers::CONTROL,
            },
            Command::Move(MoveCommand::Left),
        );
        // Ctrl+K -> Kill to end of line
        self.keymaps.insert(
            KeyPress {
                key: Key::Character("k".into()),
                modifiers: Modifiers::CONTROL,
            },
            Command::Edit(EditCommand::DeleteToEndOfLine),
        );

        if multiline {
            // Ctrl+N -> Move down
            self.keymaps.insert(
                KeyPress {
                    key: Key::Character("n".into()),
                    modifiers: Modifiers::CONTROL,
                },
                Command::Move(MoveCommand::Down),
            );
            // Ctrl+P -> Move up
            self.keymaps.insert(
                KeyPress {
                    key: Key::Character("p".into()),
                    modifiers: Modifiers::CONTROL,
                },
                Command::Move(MoveCommand::Up),
            );
        }

        self
    }

    /// No-op on Windows where Emacs bindings are not standard.
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    pub fn with_emacs_bindings(self, _multiline: bool) -> Self {
        self
    }

    /// Build the final keymap.
    pub fn build(self) -> Keymap {
        Keymap {
            keymaps: self.keymaps,
        }
    }
}

impl Default for KeymapBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A keymap containing key bindings for a text editor.
pub struct Keymap {
    pub keymaps: HashMap<KeyPress, Command>,
}

impl Keymap {
    /// Create a keymap for single-line text input.
    pub fn single_line() -> Self {
        KeymapBuilder::new()
            .with_common_bindings()
            .with_single_line_bindings()
            .with_emacs_bindings(false)
            .build()
    }

    /// Create a keymap for multi-line text area.
    pub fn multi_line() -> Self {
        KeymapBuilder::new()
            .with_common_bindings()
            .with_multi_line_bindings()
            .with_emacs_bindings(true)
            .build()
    }

    /// Look up a command for a key press, trying with and without shift modifier.
    pub fn get(&self, key: &Key, modifiers: &Modifiers) -> Option<&Command> {
        let keypress = KeyPress {
            key: key.clone(),
            modifiers: *modifiers,
        };

        self.keymaps.get(&keypress).or_else(|| {
            let mut modified = keypress.clone();
            modified.modifiers.set(Modifiers::SHIFT, false);
            self.keymaps.get(&modified)
        })
    }
}
