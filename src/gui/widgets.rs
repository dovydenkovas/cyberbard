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
        } else {
            if ui.heading(&self.text).double_clicked() {
                self.is_editing = true;
                None
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct EditableSlider {
    value: f32,
}

impl EditableSlider {
    pub fn new() -> EditableSlider {
        EditableSlider { value: 1.0 }
    }

    pub fn from(v: f32) -> EditableSlider {
        EditableSlider { value: v }
    }

    pub fn set_value(&mut self, v: f32) {
        self.value = v;
    }

    pub fn update(&mut self, ui: &mut Ui) -> Option<f32> {
        if ui
            .add(egui::Slider::new(&mut self.value, 0.0..=1.0).show_value(false))
            .changed()
        {
            Some(self.value)
        } else {
            None
        }
    }
}
