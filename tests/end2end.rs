use crossbeam::channel::{Receiver, Sender};
use cursive::backend::puppet::{observed::ObservedScreen, Backend};
use cursive::event::Event;
use cursive::view::Boxable;
use cursive::views::{DummyView, Panel};
use cursive::Vec2;
use cursive_aligned_view::{Alignable, AlignedView};
use insta::assert_display_snapshot;

fn setup_test_environment<F>(cb: F) -> (Receiver<ObservedScreen>, Sender<Option<Event>>)
where
    F: FnOnce(&mut cursive::Cursive),
{
    let backend = Backend::init(Some(Vec2::new(80, 24)));
    let frames = backend.stream();
    let input = backend.input();
    let mut siv = cursive::Cursive::new(|| backend);
    cb(&mut siv);
    input
        .send(Some(Event::Refresh))
        .expect("Refresh not accepted, backend not valid");
    siv.step();
    (frames, input)
}

#[test]
fn end2end_top_left() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_top_left(panel);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn top_center() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_top_center(panel);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn top_right() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_top_right(panel);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn center_left() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_center_left(panel);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn center() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_center(panel);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn center_right() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_center_right(panel);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn bottom_left() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_bottom_left(panel);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn bottom_center() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_bottom_center(panel);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn bottom_right() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView).title("Hello, world!").fixed_width(20);
        let aligned = AlignedView::with_bottom_right(panel);
        siv.add_fullscreen_layer(aligned);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn align_top_left() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView)
            .title("Hello, world!")
            .fixed_width(20)
            .align_top_left();
        siv.add_fullscreen_layer(panel);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn align_top_center() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView)
            .title("Hello, world!")
            .fixed_width(20)
            .align_top_center();
        siv.add_fullscreen_layer(panel);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn align_top_right() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView)
            .title("Hello, world!")
            .fixed_width(20)
            .align_top_right();
        siv.add_fullscreen_layer(panel);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn align_center_left() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView)
            .title("Hello, world!")
            .fixed_width(20)
            .align_center_left();
        siv.add_fullscreen_layer(panel);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn align_center() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView)
            .title("Hello, world!")
            .fixed_width(20)
            .align_center();
        siv.add_fullscreen_layer(panel);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn align_center_right() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView)
            .title("Hello, world!")
            .fixed_width(20)
            .align_center_right();
        siv.add_fullscreen_layer(panel);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn align_bottom_left() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView)
            .title("Hello, world!")
            .fixed_width(20)
            .align_bottom_left();
        siv.add_fullscreen_layer(panel);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn align_bottom_center() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView)
            .title("Hello, world!")
            .fixed_width(20)
            .align_bottom_center();
        siv.add_fullscreen_layer(panel);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
#[test]
fn align_bottom_right() {
    let (frames, _) = setup_test_environment(|siv| {
        let panel = Panel::new(DummyView)
            .title("Hello, world!")
            .fixed_width(20)
            .align_bottom_right();
        siv.add_fullscreen_layer(panel);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}
