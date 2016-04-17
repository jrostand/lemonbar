use std::fmt;

/// The `MouseButton` type. Identifies mouse buttons for actions.
pub enum MouseButton {
    Left,
    Middle,
    Right,
    ScrollUp,
    ScrollDown,
}

/// Buttons contain a `Vec` of `ClickAction`s and some text. Typically used as the `text` of a `Block`.
pub struct Button {
    pub actions:    Vec<ClickAction>,
    pub text:       String,
}

/// ClickActions hold a `MouseButton` and the command to execute when that mouse button is
/// triggered on the `Block`.
pub struct ClickAction {
    pub button:     MouseButton,
    pub command:    String,
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for action in self.actions.iter() {
            try!(write!(f, "{}", action));
        }

        // Write the button text
        try!(write!(f, "{}", self.text.to_string()));

        // Write the button closing tags
        for _ in 0 .. self.actions.len() {
            try!(write!(f, "%{{A}}"));
        }

        Ok(())
    }
}

impl fmt::Display for ClickAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%{{A{}:{}:}}", self.button, self.command)
    }
}

impl fmt::Display for MouseButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let btn_number = match *self {
            MouseButton::Left => 1,
            MouseButton::Middle => 2,
            MouseButton::Right => 3,
            MouseButton::ScrollUp => 4,
            MouseButton::ScrollDown => 5,
        };

        write!(f, "{}", btn_number)
    }
}

#[cfg(test)]
mod tests {
    use super::{Button, ClickAction, MouseButton};

    #[test]
    fn button_to_string() {
        let left_action = ClickAction {
            button: MouseButton::Left,
            command: "reboot".to_string(),
        };

        let right_action = ClickAction {
            button: MouseButton::Right,
            command: "halt".to_string(),
        };

        let button = Button {
            actions: vec![left_action, right_action],
            text: "test".to_string(),
        };

        assert_eq!(button.to_string(), "%{A1:reboot:}%{A3:halt:}test%{A}%{A}");
    }

    #[test]
    fn click_action_to_string() {
        let action = ClickAction {
            button: MouseButton::Left,
            command: "echo".to_string(),
        };

        assert_eq!(action.to_string(), "%{A1:echo:}");
    }

    #[test]
    fn mouse_button_to_string() {
        assert_eq!(MouseButton::Left.to_string(), "1");
        assert_eq!(MouseButton::Middle.to_string(), "2");
        assert_eq!(MouseButton::Right.to_string(), "3");
        assert_eq!(MouseButton::ScrollUp.to_string(), "4");
        assert_eq!(MouseButton::ScrollDown.to_string(), "5");
    }
}
