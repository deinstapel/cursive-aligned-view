use cursive::Cursive;
use cursive::view::Boxable;
use cursive::views::{Panel, DummyView};
use cursive_aligned_view::Alignable;

fn main() {
    let mut siv = Cursive::default();

    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .fixed_width(20)
        .align_top_right();

    siv.add_fullscreen_layer(panel);
    siv.run()
}
