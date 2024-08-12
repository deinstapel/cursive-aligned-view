use cursive::view::{Resizable, SizeConstraint};
use cursive::views::{DummyView, Panel, LinearLayout, TextView, Button};
use cursive::{Cursive, CursiveExt};
use cursive_aligned_view::Alignable;

fn main() {
    let mut siv = Cursive::default();
    
    let left = Panel::new(DummyView).title("Left panel");
    let bottom = Panel::new(DummyView).title("Bottom panel");
    let right_top = Panel::new(DummyView).title("Right top panel");
    let right_bottom = Panel::new(
        LinearLayout::vertical()
            .child(TextView::new("Press this button to quit"))
            .child(Button::new("Quit", |s| s.quit()))
            .align_center()
            .resized(SizeConstraint::Free, SizeConstraint::Free)
        ).title("Right bottom panel");
        
    let layout = LinearLayout::vertical().child(
        LinearLayout::horizontal().child(left).child(
            LinearLayout::vertical().child(right_top).child(right_bottom)
        )
    ).child(bottom);

    siv.add_fullscreen_layer(layout);
    siv.run();
}
