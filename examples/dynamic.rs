use cursive::view::{Nameable, Resizable};
use cursive::views::{DummyView, Panel, ResizedView};
use cursive::{Cursive, CursiveExt};
use cursive_aligned_view::{Alignable, AlignedView};

use std::time::Duration;

type ChildView = ResizedView<Panel<DummyView>>;

fn main() {
    let mut siv = Cursive::default();

    let panel = Panel::new(DummyView)
        .title("Hello, world!")
        .fixed_width(20)
        .align_top_left()
        .with_name("panel")
        .full_screen();

    let sink = siv.cb_sink().clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(1));
        sink.send(Box::new(|siv| {
            siv.call_on_name("panel", |view: &mut AlignedView<ChildView>| {
                view.set_top_center()
            });
        }))
        .expect("alignment change failed");

        std::thread::sleep(Duration::from_secs(1));
        sink.send(Box::new(|siv| {
            siv.call_on_name("panel", |view: &mut AlignedView<ChildView>| {
                view.set_top_right()
            });
        }))
        .expect("alignment change failed");

        std::thread::sleep(Duration::from_secs(1));
        sink.send(Box::new(|siv| {
            siv.call_on_name("panel", |view: &mut AlignedView<ChildView>| {
                view.set_center_left()
            });
        }))
        .expect("alignment change failed");

        std::thread::sleep(Duration::from_secs(1));
        sink.send(Box::new(|siv| {
            siv.call_on_name("panel", |view: &mut AlignedView<ChildView>| {
                view.set_center()
            });
        }))
        .expect("alignment change failed");

        std::thread::sleep(Duration::from_secs(1));
        sink.send(Box::new(|siv| {
            siv.call_on_name("panel", |view: &mut AlignedView<ChildView>| {
                view.set_center_right()
            });
        }))
        .expect("alignment change failed");

        std::thread::sleep(Duration::from_secs(1));
        sink.send(Box::new(|siv| {
            siv.call_on_name("panel", |view: &mut AlignedView<ChildView>| {
                view.set_bottom_left()
            });
        }))
        .expect("alignment change failed");

        std::thread::sleep(Duration::from_secs(1));
        sink.send(Box::new(|siv| {
            siv.call_on_name("panel", |view: &mut AlignedView<ChildView>| {
                view.set_bottom_center()
            });
        }))
        .expect("alignment change failed");

        std::thread::sleep(Duration::from_secs(1));
        sink.send(Box::new(|siv| {
            siv.call_on_name("panel", |view: &mut AlignedView<ChildView>| {
                view.set_bottom_right()
            });
        }))
        .expect("alignment change failed");

        std::thread::sleep(Duration::from_secs(1));
        sink.send(Box::new(|siv| {
            siv.quit();
        }))
        .expect("alignment change failed");
    });

    siv.add_layer(panel);
    siv.run()
}
