use std::fmt;

pub enum MouseButton {
    Left,
    Middle,
    Right,
    ScrollUp,
    ScrollDown,
}

pub struct Button {
    actions:    Vec<ClickAction>,
    text:       String,
}

pub struct ClickAction {
    button:     MouseButton,
    command:    String,
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut content = self.text.to_string();

        for action in self.actions.iter().rev() {
            content = format!("{}{}%{{A}}", action, content);
        }

        write!(f, "{}", content)
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

        assert!(button.to_string() == "%{A1:reboot:}%{A3:halt:}test%{A}%{A}");
    }

    #[test]
    fn click_action_to_string() {
        let action = ClickAction {
            button: MouseButton::Left,
            command: "echo".to_string(),
        };

        assert!(action.to_string() == "%{A1:echo:}");
    }

    #[test]
    fn mouse_button_to_string() {
        assert!(MouseButton::Left.to_string() == "1");
        assert!(MouseButton::Middle.to_string() == "2");
        assert!(MouseButton::Right.to_string() == "3");
        assert!(MouseButton::ScrollUp.to_string() == "4");
        assert!(MouseButton::ScrollDown.to_string() == "5");
    }
}
