use cursive::view::Resizable as _;
use cursive::views::{DummyView, Panel};
use cursive::{Cursive, CursiveExt};
use cursive_aligned_view::Alignable as _;

fn main() {
    let mut siv = Cursive::default();

    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .fixed_width(20)
        .align_center()
        .full_screen();

    siv.add_layer(panel);
    siv.run()
}
