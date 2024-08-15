//! # egui-twemoji
//!
//! An [egui](https://egui.rs/) widget that renders colored [Twemojis](https://github.com/twitter/twemoji).
//! Based on [twemoji-assets](https://github.com/cptpiepmatz/twemoji-assets).
//!
//! ![demo](https://github.com/zeozeozeo/egui-twemoji/blob/master/media/demo.png?raw=true)
//!
//! # How to use
//!
//! Make sure you've installed `egui_extras` image loaders (required for rendering SVG and PNG emotes):
//!
//! ```ignore
//! // don't do this every frame - only when the app is created!
//! egui_extras::install_image_loaders(&cc.egui_ctx);
//! ```
//!
//! And then:
//!
//! ```rust
//! use egui_twemoji::EmojiLabel;
//!
//! fn show_label(ui: &mut egui::Ui) {
//!     EmojiLabel::new("⭐ egui-twemoji 🐦✨").show(ui);
//! }
//! ```
//!
//! For a more sophisticated example, see the `demo` example (`cargo run --example demo`)
//!
//! `EmojiLabel` supports all functions that a normal
//! [Label](https://docs.rs/egui/latest/egui/widgets/struct.Label.html) does.
//!
//! # Features
//!
//! * `svg`: use SVG emoji assets (`egui_extras/svg` is required)
//! * `png`: use PNG emoji assets (`egui_extras/image` is required)
//!
//! By default, the `svg` feature is activated.
//!
//! # License
//!
//! Unlicense OR MIT OR Apache-2.0

#![warn(missing_docs)]

mod exposed;

use egui::{ImageSource, Layout, RichText, Sense};
use exposed::ExposedRichText;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(all(feature = "svg", feature = "png"))]
compile_error!("features 'svg' and 'png' are mutually exclusive and cannot be enabled together");

/// Represents a segment of text which can be either plain text or an emoji.
///
/// * `Text` variant wraps the `RichText` struct, which includes text and its styling information.
/// * `Emoji` variant contains a `String` representing the emoji character.
#[derive(PartialEq, Clone)]
enum TextSegment {
    Text(RichText),
    Emoji(String),
}

#[inline]
fn is_emoji(text: &str) -> bool {
    #[cfg(feature = "svg")]
    return twemoji_assets::svg::SvgTwemojiAsset::from_emoji(text).is_some();

    #[cfg(feature = "png")]
    return twemoji_assets::png::PngTwemojiAsset::from_emoji(text).is_some();
}

/// Returns a vector of [`TextSegment`]s from a [`RichText`], segmented by emojis.
///
/// ## Example:
///
/// "hello 😤 world" -> `[TextSegment::Text("hello "), TextSegment::Emoji("😤"), TextSegment::Text(" world")]`
fn segment_text(input: &RichText) -> Vec<TextSegment> {
    let mut result = Vec::new();
    let mut text = String::new();

    for grapheme in UnicodeSegmentation::graphemes(input.text(), true) {
        if is_emoji(grapheme) {
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

/// The state of an [EmojiLabel], stored in egui's [`egui::Memory`].
/// This includes memoized text segments and whether the state was newly created.
#[derive(Default, Clone)]
struct LabelState {
    segments: Vec<TextSegment>,
    is_saved: bool,
}

impl LabelState {
    /// Create a new state from a [`RichText`], segmenting it by emojis.
    fn from_text(text: impl Into<RichText>) -> Self {
        let rich_text = text.into();
        Self {
            segments: segment_text(&rich_text),
            is_saved: false,
        }
    }

    /// Load the state from egui's [`egui::Memory`].
    fn load(ctx: &egui::Context, id: egui::Id, text: &RichText) -> Self {
        ctx.data_mut(|d| {
            d.get_temp(id)
                .unwrap_or_else(|| Self::from_text(text.clone()))
        })
    }

    /// Save the state to egui's [`egui::Memory`]. Only call this if [`Self::is_saved`] is `false`.
    fn save(self, ctx: &egui::Context, id: egui::Id) {
        ctx.data_mut(|d| d.insert_temp(id, self));
    }
}

/// An [egui](https://egui.rs/) widget that renders colored [Twemojis](https://github.com/twitter/twemoji).
///
/// ```rust
/// use egui_twemoji::EmojiLabel;
///
/// fn show_label(ui: &mut egui::Ui) {
///     EmojiLabel::new("⭐ egui-twemoji 🐦✨").show(ui);
/// }
/// ```
#[must_use = "You should put this widget in an ui by calling `.show(ui);`"]
pub struct EmojiLabel {
    /// The text to render.
    pub text: RichText,
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
    pub wrap: Option<bool>,
    /// If `true`, the text will stop at the max width of the [`Ui`],
    /// and what doesn't fit will be elided, replaced with `…`.
    ///
    /// If the text is truncated, the full text will be shown on hover as a tool-tip.
    ///
    /// Default is `false`, which means the text will expand the parent [`Ui`],
    /// or wrap if [`Self::wrap`] is set.
    ///
    /// Calling `truncate` will override [`Self::wrap`].
    pub truncate: bool,
    /// Make the label respond to clicks and/or drags.
    ///
    /// By default, a label is inert and does not respond to click or drags.
    /// By calling this you can turn the label into a button of sorts.
    /// This will also give the label the hover-effect of a button, but without the frame.
    pub sense: Option<Sense>,
    /// Can the user select the text with the mouse?
    ///
    /// Overrides [`egui::style::Interaction::selectable_labels`].
    pub selectable: Option<bool>,
    /// Whether the widget should recognize that it is in a horizontal layout and not create a new one.
    /// This fixes some wrapping issues with [`egui::Label`].
    ///
    /// In vertical layouts, the widget will create a new horizontal layout so text segments stay on the
    /// same line.
    pub auto_inline: bool,
}

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

#[inline]
fn empty_response(ctx: egui::Context) -> egui::Response {
    egui::Response {
        ctx,
        layer_id: egui::LayerId::background(),
        id: egui::Id::NULL,
        rect: egui::Rect::ZERO,
        interact_rect: egui::Rect::ZERO,
        sense: Sense::click(),
        enabled: false,
        contains_pointer: false,
        hovered: false,
        highlighted: false,
        clicked: false,
        fake_primary_click: false,
        long_touched: false,
        drag_started: false,
        dragged: false,
        drag_stopped: false,
        is_pointer_button_down_on: false,
        interact_pointer_pos: None,
        changed: false,
    }
}

impl EmojiLabel {
    /// Create a new [`EmojiLabel`] from a [`RichText`].
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: text.into(),
            wrap: None,
            truncate: false,
            sense: None,
            selectable: None,
            auto_inline: true,
        }
    }

    /// Get the text to render as a [str].
    pub fn text(&self) -> &str {
        self.text.text()
    }

    /// Get the text to render as a [`RichText`].
    pub fn rich_text(&self) -> &RichText {
        &self.text
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

    /// Whether the widget should recognize that it is in a horizontal layout and not create a new one.
    /// This fixes some wrapping issues with [`egui::Label`].
    ///
    /// In vertical layouts, the widget will create a new horizontal layout so text segments stay on the
    /// same line.
    #[inline]
    pub fn auto_inline(mut self, auto_inline: bool) -> Self {
        self.auto_inline = auto_inline;
        self
    }

    fn show_segments(&self, ui: &mut egui::Ui, state: &mut LabelState) -> egui::Response {
        let mut resp = empty_response(ui.ctx().clone());
        let font_height = ui.text_style_height(&egui::TextStyle::Body);

        for segment in &state.segments {
            ui.spacing_mut().item_spacing.x = 0.0;
            match segment {
                TextSegment::Text(text) => {
                    let mut label = egui::Label::new(text.clone()).truncate(self.truncate);
                    if let Some(wrap) = self.wrap {
                        label = label.wrap(wrap);
                    }
                    if let Some(selectable) = self.selectable {
                        label = label.selectable(selectable);
                    }
                    if let Some(sense) = self.sense {
                        label = label.sense(sense);
                    }
                    resp |= ui.add(label);
                }
                TextSegment::Emoji(emoji) => {
                    let Some(source) = get_source_for_emoji(emoji) else {
                        continue;
                    };

                    let image_rect = ui
                        .add(
                            egui::Image::new(source)
                                .fit_to_exact_size(egui::vec2(font_height, font_height)),
                        )
                        .rect;

                    // for emoji selection and copying:
                    resp |= ui.put(
                        image_rect,
                        egui::Label::new(RichText::new(emoji).color(egui::Color32::TRANSPARENT)),
                    );
                }
            }
        }
        resp
    }

    /// Add the label to an [`egui::Ui`].
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let id = egui::Id::new(self.text());
        let mut state = LabelState::load(ui.ctx(), id, &self.text);

        // if the state was newly created, write it back to memory:
        if !state.is_saved {
            state.is_saved = true;
            state.clone().save(ui.ctx(), id);
        }

        if ui.layout().is_horizontal() && self.auto_inline {
            self.show_segments(ui, &mut state)
        } else {
            ui.with_layout(
                Layout::left_to_right(egui::Align::Min).with_main_wrap(true),
                |ui| self.show_segments(ui, &mut state),
            )
            .inner
        }
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
