use crossbeam::channel::{Receiver, Sender};
use cursive::backends::puppet::{observed::ObservedScreen, Backend};
use cursive::event::Event;
use cursive::view::{Boxable, SizeConstraint};
use cursive::views::{DummyView, Panel, TextView};
use cursive::Vec2;
use cursive_aligned_view::{Alignable, AlignedView};
use cursive_core::align::{Align, HAlign, VAlign};
use insta::assert_display_snapshot;

fn setup_test_environment<F>(cb: F) -> (Receiver<ObservedScreen>, Sender<Option<Event>>)
where
    F: FnOnce(&mut cursive::Cursive),
{
    let backend = Backend::init(Some(Vec2::new(80, 24)));
    let frames = backend.stream();
    let input = backend.input();
    let mut siv = cursive::Cursive::new().into_runner(backend);
    cb(&mut siv);
    input
        .send(Some(Event::Refresh))
        .expect("Refresh not accepted, backend not valid");
    siv.step();
    (frames, input)
}

#[test]
fn squeeze_underfill() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(TextView::new("A very long text that will reach the limit of some screens")).title("Hello, world!");
        let aligned = AlignedView::with_top_left(panel).resized(SizeConstraint::Fixed(15), SizeConstraint::Fixed(3));
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn exact_match_fill() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(TextView::new("A very long text that will reach the limit of some screens")).title("Hello, world!");
        let aligned = AlignedView::with_top_left(panel).resized(SizeConstraint::Fixed(60), SizeConstraint::Fixed(3));
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn top_left() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_top_left(panel).resized(SizeConstraint::Full, SizeConstraint::Full);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn top_center() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_top_center(panel).resized(SizeConstraint::Full, SizeConstraint::Full);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn top_right() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_top_right(panel).resized(SizeConstraint::Full, SizeConstraint::Full);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn center_left() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_center_left(panel).resized(SizeConstraint::Full, SizeConstraint::Full);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn center() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_center(panel).resized(SizeConstraint::Full, SizeConstraint::Full);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn center_right() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_center_right(panel).resized(SizeConstraint::Full, SizeConstraint::Full);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn bottom_left() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_bottom_left(panel).resized(SizeConstraint::Full, SizeConstraint::Full);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn bottom_center() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_bottom_center(panel).resized(SizeConstraint::Full, SizeConstraint::Full);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn bottom_right() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_bottom_right(panel).resized(SizeConstraint::Full, SizeConstraint::Full);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn align_top_left() {
    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .align_top_left();
    assert_eq!(*panel.alignment(), Align::top_left())
}
#[test]
fn align_top_center() {
    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .align_top_center();
    assert_eq!(*panel.alignment(), Align::top_center())
}
#[test]
fn align_top_right() {
    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .align_top_right();
    assert_eq!(*panel.alignment(), Align::top_right())
}
#[test]
fn align_center_left() {
    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .align_center_left();
    assert_eq!(*panel.alignment(), Align::center_left())
}
#[test]
fn align_center() {
    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .align_center();
    assert_eq!(*panel.alignment(), Align::center())
}
#[test]
fn align_center_right() {
    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .align_center_right();
    assert_eq!(*panel.alignment(), Align::center_right())
}
#[test]
fn align_bottom_left() {
    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .align_bottom_left();
    assert_eq!(*panel.alignment(), Align::bot_left());
}
#[test]
fn align_bottom_center() {
    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .align_bottom_center();
    assert_eq!(*panel.alignment(), Align::bot_center());
}
#[test]
fn align_bottom_right() {
    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .align_bottom_right();
    assert_eq!(*panel.alignment(), Align::new(HAlign::Right, VAlign::Bottom));
}
