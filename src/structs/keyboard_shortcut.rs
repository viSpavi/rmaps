
pub struct KeyShortcut {
    keys: Vec<char>
}

impl KeyShortcut {
    pub fn new(keys: Vec<char>) -> KeyShortcut {
        KeyShortcut {
            keys
        }
    }
    pub fn equals_permut(&self, keys: &[char]) -> bool {
        //check if all keys are contained in self.keys
        for key in keys {
            if !self.keys.contains(key) {
                return false
            }
        }
        true
    }
}