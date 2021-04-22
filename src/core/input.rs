use crate::core::spacetime::{*};
use crate::util::{*};

use legion::*;

/// Marker for entity selection by the player
// #[derive(Component, EcsComponent, Default)]
// #[storage(NullStorage)]
// #[name("selected")]
pub struct SelectedComponent;

pub enum InputEvent {
    Click(SelectType, Position),
    /// (_, from ,to)
    Select(SelectType, Position, Position),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SelectType {
    Left,
    Right,
}

pub enum MouseButton {
    Left, 
    Right
}

#[derive(Debug)] // TODO: This should be in a system manager sort of object
pub enum Exit {
    Stop,
    Restart,
}

/// Resource for selected entity - not guaranteed to be alive
/// `get()` will clear it if the entity is dead
#[derive(Default)]
pub struct SelectedEntity(Option<Entity>);

#[derive(Default, Clone)]
pub struct SelectedTiles(Option<Position>);

const TILE_SELECTION_LIMIT: f32 = 50.0;

struct DivineCommand {
    id: u32,
}

/// Command from the player through the UI
pub enum UiCommand {
    ExitGame(Exit),

    IssueDivineCommand(DivineCommand),

    // IssueSocietyCommand(SocietyHandle, SocietyCommand),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MouseClickType {
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum MouseStateC {
    Unpressed,
    Down(SelectType, Position),
    Dragging(SelectType, Position),
}

pub struct Selection {
    state: MouseStateC,
}

impl Selection {
    pub fn mouse_down(&mut self, select: SelectType, pos: Position) {
        // dont bother about multiple buttons being held down at once
        if let MouseStateC::Unpressed = self.state {
            self.state = MouseStateC::Down(select, pos);
        }
    }
    pub fn mouse_up(&mut self, select: SelectType, pos: Position) -> Option<InputEvent> {
        let evt = match self.state {
            MouseStateC::Down(prev_select, _) if select == prev_select => {
                // single selection at the mouse up location, ignoring the down location
                let evt = InputEvent::Click(select, pos);
                Some(evt)
            }
            MouseStateC::Dragging(prev_select, start) if select == prev_select => {
                // region selection from original mouse down location
                let evt = InputEvent::Select(select, start, pos);
                Some(evt)
            }
            _ => None,
        };

        if evt.is_some() {
            // consume mouse press
            self.state = MouseStateC::Unpressed;
        }

        evt
    }

    pub fn mouse_move(&mut self, select: SelectType, pos: Position) {
        match self.state {
            MouseStateC::Down(prev_select, _) if prev_select == select => {
                // start dragging
                self.state = MouseStateC::Dragging(select, pos);
            }
            _ => {}
        }
    }

    pub fn select_type(btn: MouseButton) -> Option<SelectType> {
        match btn {
            MouseButton::Left => Some(SelectType::Left),
            MouseButton::Right => Some(SelectType::Right),
            _ => None,
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            state: MouseStateC::Unpressed,
        }
    }
}