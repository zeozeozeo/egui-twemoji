use egui::RichText;
use egui_twemoji::EmojiLabel;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "egui-twemoji demo",
        options,
        Box::new(|cc| {
            // this is important: we are going to be rendering the emojis as SVGs
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<ExampleApp>::default()
        }),
    )
}

#[derive(Default)]
struct ExampleApp {
    paste_field: String,
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                EmojiLabel::new("⭐ egui-twemoji 🐦 demo ✨").show(ui);
                if EmojiLabel::new(
                    RichText::new("👉 This 👈 is a strong 💪😈 RichText 🤑💰 label").strong(),
                )
                .show(ui)
                .hovered()
                {
                    EmojiLabel::new("hovered! 😸").show(ui);
                }
                EmojiLabel::new("Yes 👍, you 🤟 can 🎥 select 📝 and copy 🍝 this 👌").show(ui);

                ui.separator();
                EmojiLabel::new("Paste 🆒 text here 📝📜:").show(ui);
                ui.text_edit_multiline(&mut self.paste_field);

                ui.collapsing("Emoji Madness (laggy)", |ui| {
                    emoji_madness(ui);
                });

                ui.horizontal_wrapped(|ui| {
                    EmojiLabel::new("a a a a a a a a a a 👍 a a a a a a a a a a a a ").show(ui);
                    EmojiLabel::new("b b b b b b b 👍 b b b b b b 👍 b b b b 👍 b b b 👍 b b ")
                        .show(ui);
                });
            });
        });
    }
}

fn emoji_madness(ui: &mut egui::Ui) {
    EmojiLabel::new("🙅🙆🙇🙋🙌🙍🙎🙏✂✈✉✊✋✌✏❄❤🚀🚃🚄🚅").show(ui);
    EmojiLabel::new("🚇🚉🚌🚏🚑🚒🚓🚕🚗🚙🚚🚢🚤🚥🚧🚨🚩🚪🚫").show(ui);
    EmojiLabel::new("🚬🚲🚶🚽🛀⌚⌛⏰⏳☁☎☔☕♨♻♿⚓⚡⚽⚾⛄⛅").show(ui);
    EmojiLabel::new("⛪⛲⛳⛵⛺⭐⛽🌀🌁🌂🌃🌄🌅🌆🌇🌈🌉🌊🌋").show(ui);
    EmojiLabel::new("🌏🌙🌛🌟🌠🌰🌱🌴🌵🌷🌸🌹🌺🌻🌼🌽🌾🌿🍀🍁").show(ui);
    EmojiLabel::new("🍂🍃🍄🍅🍆🍇🍈🍉🍊🍌🍍🍎🍏🍑🍒🍓🍔🍕🍖").show(ui);
    EmojiLabel::new("🍗🍘🍙🍚🍛🍜🍝🍞🍟🍠🍡🍢🍣🍤🍥🍦🍧🍨🍩").show(ui);
    EmojiLabel::new("🍪🍫🍬🍭🍮🍯🍰🍱🍲🍳🍴🍵🍶🍷🍸🍹🍺🍻🎀🎁").show(ui);
    EmojiLabel::new("🎂🎃🎄🎅🎆🎇🎈🎉🎊🎋🎌🎍🎎🎏🎐🎑🎒🎓🎠").show(ui);
    EmojiLabel::new("🎡🎢🎣🎤🎥🎦🎧🎨🎩🎪🎫🎬🎭🎮🎯🎰🎱🎲").show(ui);
    EmojiLabel::new("🎳🎴🎵🎶🎷🎸🎹🎺🎻🎽🎾🎿🏀🏁🏂🏃🏄🏆").show(ui);
    EmojiLabel::new("🏈🏊🏠🏡🏢🏣🏥🏦🏧🏨🏩🏪🏫🏬🏭🏮🏯🏰").show(ui);
    EmojiLabel::new("🐌🐍🐎🐑🐒🐔🐗🐘🐙🐚🐛🐜🐝🐞🐟🐠🐡🐢").show(ui);
    EmojiLabel::new("🐣🐤🐥🐦🐧🐨🐩🐫🐬🐭🐮🐯🐰🐱🐲🐳🐴🐵").show(ui);
    EmojiLabel::new("🐶🐷🐸🐹🐺🐻🐼🐽🐾👀👂👃👄👅👆👇👈👉").show(ui);
    EmojiLabel::new("👊👋👌👍👎👏👐👑👒👓👔👕👖👗👘👙👚👛👜").show(ui);
    EmojiLabel::new("👝👞👟👠👡👢👣👤👦👧👨👩👪👫👮👯👰👱").show(ui);
    EmojiLabel::new("👴👶👷👸👹👺👻👼👽👾👿💀💁💂💃💄💅💆💇").show(ui);
    EmojiLabel::new("💈💉💊💋💌💍💎💏💐💑💒💓💔💕💖💗💘💙").show(ui);
    EmojiLabel::new("💚💛💜💝💞💟💠💡💢💣💤💥💦💧💨💩💪💫").show(ui);
    EmojiLabel::new("💬💮💯💰💲💳💵💸💺💻💼💽💾💿📀📃📅📆📈").show(ui);
    EmojiLabel::new("📉📌📍📎📓📔📕📖📞📟📠📡📣📦📧📫📰📱📷📹").show(ui);
    EmojiLabel::new("📺📻📼🔊🔋🔌🔎🔐🔑🔒🔓🔔🔜🔥🔦🔧🔨🔩🔪").show(ui);
    EmojiLabel::new("🔫🔮🗻🗼🗽🗾🗿😴🚁🚂🚆🚈🚊🚍🚎🚐🚔🚖🚘").show(ui);
    EmojiLabel::new("🚛🚜🚝🚞🚟🚠🚡🚣🚦🚮🚵🚿🛁🌍🌎🌜🌝🌞🌲").show(ui);
    EmojiLabel::new("🌳🍋🍐🍼🏇🏉🏤🐀🐁🐂🐃🐄🐅🐆🐇🐈🐉🐊🐋").show(ui);
    EmojiLabel::new("🐏🐐🐓🐕🐖🐪👬👭📬📭📯🔬🔭").show(ui);
}
