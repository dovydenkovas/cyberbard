use egui::Ui;

pub struct EditableHeader {
    text: String,
    is_editing: bool,
}

impl EditableHeader {
    pub fn new(text: String) -> EditableHeader {
        EditableHeader {
            text,
            is_editing: false,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<String> {
        if self.is_editing {
            if ui.text_edit_singleline(&mut self.text).lost_focus() {
                self.is_editing = false;
                Some(self.text.clone())
            } else {
                None
            }
        } else if ui.heading(&self.text).double_clicked() {
            self.is_editing = true;
            None
        } else {
            None
        }
    }
}
