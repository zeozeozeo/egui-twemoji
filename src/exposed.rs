use egui::RichText;

// ugh...

#[derive(Clone, Default, PartialEq)]
pub struct ExposedRichText {
    pub text: String,
    pub size: Option<f32>,
    pub extra_letter_spacing: f32,
    pub line_height: Option<f32>,
    pub family: Option<egui::FontFamily>,
    pub text_style: Option<egui::TextStyle>,
    pub background_color: egui::Color32,
    pub expand_bg: f32,
    pub text_color: Option<egui::Color32>,
    pub code: bool,
    pub strong: bool,
    pub weak: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub italics: bool,
    pub raised: bool,
}

impl From<RichText> for ExposedRichText {
    fn from(value: RichText) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<ExposedRichText> for RichText {
    fn from(value: ExposedRichText) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl ExposedRichText {
    pub fn new_keep_properties(text: impl Into<String>, old: &RichText) -> Self {
        Self {
            text: text.into(),
            ..old.clone().into()
        }
    }
}
