use unicode_segmentation::UnicodeSegmentation;

/// The grapheme content of a string, for easier editing of visual string content
pub struct TextValue {
    graphemes: Vec<String>,
}

impl TextValue {
    pub fn new(string: &str) -> Self {
        let graphemes = UnicodeSegmentation::graphemes(string, true)
            .map(String::from)
            .collect();
        Self { graphemes }
    }

    pub fn len(&self) -> usize {
        self.graphemes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.graphemes.len() == 0
    }

    pub fn remove(&mut self, index: usize) {
        self.graphemes.remove(index);
    }

    pub fn insert(&mut self, index: usize, c: char) {
        self.graphemes.insert(index, c.to_string());
    }

    pub fn to_string(&self) -> String {
        self.graphemes.concat()
    }
}
