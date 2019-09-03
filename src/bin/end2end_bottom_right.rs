use cursive::Cursive;
use cursive::view::Boxable;
use cursive::views::{Panel, DummyView};
use cursive_aligned_view::AlignedView;

fn main() {
    let mut siv = Cursive::default();

    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .fixed_width(20);
    let aligned = AlignedView::bottom_right(panel);

    siv.add_fullscreen_layer(aligned);
    siv.run()
}
