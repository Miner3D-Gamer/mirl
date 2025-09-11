use crate::platform::keycodes::KeyCode;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A singular key combination
pub struct KeyBind<T> {
    /// The keys required to activate this keybind besides shift, alt, and control
    pub keys: Vec<KeyCode>,
    /// If shift needs to be pressed for this keybind to be considered pressed (also contributes to its priority)
    pub requires_shift: bool,
    /// If alt needs to be pressed for this keybind to be considered pressed (also contributes to its priority)
    pub requires_alt: bool,
    /// If control needs to be pressed for this keybind to be considered pressed (also contributes to its priority)
    pub requires_control: bool,
    /// The Enum you use for matching the action if the keybind is pressed
    pub action: T,
}
impl<T> KeyBind<T> {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(
        requires_shift: bool,
        requires_alt: bool,
        requires_control: bool,
        keys: Vec<KeyCode>,
        action: T,
    ) -> Self {
        Self {
            keys,
            requires_shift,
            requires_alt,
            requires_control,
            action,
        }
    }
    /// If the conditions for this keybind is met
    #[must_use]
    pub fn is_keybind_activated(
        &self,
        newly_pressed: &[KeyCode],
        shift_pressed: bool,
        alt_pressed: bool,
        control_pressed: bool,
    ) -> bool {
        self.keys.iter().all(|k| newly_pressed.contains(k))
            && (shift_pressed || !self.requires_shift)
            && (alt_pressed || !self.requires_alt)
            && (control_pressed || !self.requires_control)
    }
    /// Get the priority of a keybind -> Higher priority inputs need to be processed first
    #[must_use]
    pub const fn get_priority(&self) -> usize {
        self.requires_shift as usize
            + self.requires_alt as usize
            + self.requires_control as usize
    }
    /// If another keybind has the same or more keys
    #[must_use]
    pub fn does_other_keybind_eat_this_one(&self, other: &Self) -> bool {
        (other.requires_control || !self.requires_control)
            && (other.requires_shift || !self.requires_shift)
            && (other.requires_alt || !self.requires_alt)
            && self.keys.iter().all(|x| other.keys.contains(x))
    }
    /// Remove all keycodes used from the given list
    pub fn remove_required_keys(&self, list: &mut Vec<KeyCode>) {
        list.retain(|x| !self.keys.contains(x));
    }
}

/// If shift + control + left are pressed, shift + left should be discarded
#[must_use]
pub fn sort_actions<T: Clone>(actions: Vec<KeyBind<T>>) -> Vec<KeyBind<T>> {
    let mut actions = actions;

    actions.sort_by_key(KeyBind::get_priority);

    actions.reverse();
    let mut new_actions: Vec<KeyBind<T>> = Vec::new();
    for i in actions.iter().rev() {
        if !actions.iter().any(|y| {
            !std::ptr::eq(i, y) // God damn you self checking
                && i.does_other_keybind_eat_this_one(y)
        }) {
            new_actions.push((*i).clone());
        }
    }
    // if !new_actions.is_empty() {
    //     println!("{:#?} \n {}", new_actions, new_actions.len());
    // }

    new_actions
}
