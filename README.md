# egui-twemoji

An [egui](https://egui.rs/) widget that renders colored [Twemojis](https://github.com/twitter/twemoji). Based on [twemoji-assets](https://github.com/cptpiepmatz/twemoji-assets).

![demo](/media/demo.png)

# How to use

Make sure you've installed `egui_extras` image loaders (required for rendering SVG and PNG emotes):

```rust
// don't do this every frame - only when the app is created!
egui_extras::install_image_loaders(&cc.egui_ctx);
```

And then:

```rust
use egui_twemoji::EmojiLabel;

fn show_label(ui: &mut egui::Ui) {
    EmojiLabel::new("⭐ egui-twemoji 🐦✨").show(ui);
}
```

For a more sophisticated example, see the `demo` example (`cargo run --example demo`)

`EmojiLabel` supports all functions that a normal 
[Label](https://docs.rs/egui/latest/egui/widgets/struct.Label.html) does.

# Features

* `svg`: use SVG emoji assets (`egui_extras/svg` is required)
* `png`: use PNG emoji assets (`egui_extras/image` is required)

By default, the `svg` feature is activated.

# License

Unlicense OR MIT OR Apache-2.0
