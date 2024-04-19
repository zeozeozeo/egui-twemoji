mod exposed;

use egui::{ImageSource, RichText, Sense};
use exposed::ExposedRichText;
use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq, Clone)]
enum TextSegment {
    Text(RichText),
    Emoji(String),
}

fn segment_text(input: &RichText) -> Vec<TextSegment> {
    let mut result = Vec::new();
    let mut text = String::new();

    for grapheme in UnicodeSegmentation::graphemes(input.text(), true) {
        if emojis::get(grapheme).is_some() {
            if !text.is_empty() {
                result.push(TextSegment::Text(
                    ExposedRichText::new_keep_properties(text.clone(), input).into(),
                ));
                text.clear();
            }
            result.push(TextSegment::Emoji(grapheme.to_string()));
        } else {
            text.push_str(grapheme);
        }
    }

    if !text.is_empty() {
        result.push(TextSegment::Text(
            ExposedRichText::new_keep_properties(text.clone(), input).into(),
        ));
    }

    result
}

#[derive(Default, Clone)]
struct LabelState {
    segments: Vec<TextSegment>,
    is_saved: bool,
}

impl LabelState {
    fn from_text(text: impl Into<RichText>) -> Self {
        let rich_text = text.into();
        Self {
            segments: segment_text(&rich_text),
            is_saved: false,
        }
    }

    fn load(ctx: &egui::Context, id: egui::Id, text: &RichText) -> Self {
        ctx.data_mut(|d| {
            d.get_temp(id)
                .unwrap_or_else(|| Self::from_text(text.clone()))
        })
    }

    fn save(self, ctx: &egui::Context, id: egui::Id) {
        ctx.data_mut(|d| d.insert_temp(id, self));
    }
}

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct EmojiLabel {
    text: RichText,
    wrap: Option<bool>,
    truncate: bool,
    sense: Option<Sense>,
    selectable: Option<bool>,
}

#[cfg(all(feature = "svg", feature = "png"))]
compile_error!("features 'svg' and 'png' are mutually exclusive and cannot be enabled together");

fn get_source_for_emoji(emoji: &str) -> Option<ImageSource> {
    #[cfg(feature = "svg")]
    {
        let svg_data = twemoji_assets::svg::SvgTwemojiAsset::from_emoji(emoji)?;
        let source = ImageSource::Bytes {
            uri: format!("{emoji}.svg").into(),
            bytes: egui::load::Bytes::Static(svg_data.as_bytes()),
        };
        Some(source)
    }

    #[cfg(feature = "png")]
    {
        let png_data: &[u8] = twemoji_assets::png::PngTwemojiAsset::from_emoji(emoji)?;
        let source = ImageSource::Bytes {
            uri: format!("{emoji}.png").into(),
            bytes: egui::load::Bytes::Static(png_data),
        };
        Some(source)
    }
}

impl EmojiLabel {
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: text.into(),
            wrap: None,
            truncate: false,
            sense: None,
            selectable: None,
        }
    }

    pub fn text(&self) -> &str {
        self.text.text()
    }

    /// If `true`, the text will wrap to stay within the max width of the [`Ui`].
    ///
    /// Calling `wrap` will override [`Self::truncate`].
    ///
    /// By default [`Self::wrap`] will be `true` in vertical layouts
    /// and horizontal layouts with wrapping,
    /// and `false` on non-wrapping horizontal layouts.
    ///
    /// Note that any `\n` in the text will always produce a new line.
    ///
    /// You can also use [`egui::Style::wrap`].
    #[inline]
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = Some(wrap);
        self.truncate = false;
        self
    }

    /// If `true`, the text will stop at the max width of the [`Ui`],
    /// and what doesn't fit will be elided, replaced with `…`.
    ///
    /// If the text is truncated, the full text will be shown on hover as a tool-tip.
    ///
    /// Default is `false`, which means the text will expand the parent [`Ui`],
    /// or wrap if [`Self::wrap`] is set.
    ///
    /// Calling `truncate` will override [`Self::wrap`].
    #[inline]
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.wrap = None;
        self.truncate = truncate;
        self
    }

    /// Can the user select the text with the mouse?
    ///
    /// Overrides [`egui::style::Interaction::selectable_labels`].
    #[inline]
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
        self
    }

    /// Make the label respond to clicks and/or drags.
    ///
    /// By default, a label is inert and does not respond to click or drags.
    /// By calling this you can turn the label into a button of sorts.
    /// This will also give the label the hover-effect of a button, but without the frame.
    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = Some(sense);
        self
    }

    pub fn show(self, ui: &mut egui::Ui) {
        let id = egui::Id::new(self.text());
        let mut state = LabelState::load(ui.ctx(), id, &self.text);

        if !state.is_saved {
            state.is_saved = true;
            state.clone().save(ui.ctx(), id);
        }

        let font_height = ui.text_style_height(&egui::TextStyle::Body);

        ui.horizontal_wrapped(|ui| {
            for segment in &state.segments {
                ui.spacing_mut().item_spacing.x = 0.0;
                match segment {
                    TextSegment::Text(text) => {
                        ui.label(text.clone());
                    }
                    TextSegment::Emoji(emoji) => {
                        let Some(source) = get_source_for_emoji(emoji) else {
                            continue;
                        };

                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                            let rect = ui
                                .add(egui::Image::new(source).max_height(font_height))
                                .rect;

                            // for emoji selection and copying:
                            ui.put(
                                rect,
                                egui::Label::new(
                                    RichText::new(emoji).color(egui::Color32::TRANSPARENT),
                                ),
                            );
                        });
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emoji_segmentation() {
        let text = "Hello😤world";
        let segments = segment_text(&RichText::new(text));
        assert!(
            segments
                == vec![
                    TextSegment::Text("Hello".into()),
                    TextSegment::Emoji("😤".to_owned()),
                    TextSegment::Text("world".into())
                ]
        );
        let text = "😅 2,*:привет|3 🤬";
        let segments = segment_text(&RichText::new(text));
        assert!(
            segments
                == vec![
                    TextSegment::Emoji("😅".to_owned()),
                    TextSegment::Text(" 2,*:привет|3 ".into()),
                    TextSegment::Emoji("🤬".to_owned()),
                ]
        );
        let text = "Hello world 🥰!";
        let segments = segment_text(&RichText::new(text));
        assert!(
            segments
                == vec![
                    TextSegment::Text("Hello world ".into()),
                    TextSegment::Emoji("🥰".to_owned()),
                    TextSegment::Text("!".into())
                ]
        );
    }
}
