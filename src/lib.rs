//! # Align cursive views
//!
//! This crate provides an `AlignedView` for
//! [gyscos/cursive](https://github.com/gyscos/cursive) views which makes it
//! possible to align the child view (center, left, right, top, bottom). The
//! `AlignedView` uses the `required_size` reported by the child view and fills
//! the rest of the available space with the views background color.
//!
//! ## Aligning a child view
//!
//! The easiest way to align a view is via the `Alignable` trait:
//!
//! ```rust
//! use cursive::Cursive;
//! use cursive::view::Boxable;
//! use cursive::views::{Panel, DummyView};
//! use cursive_aligned_view::Alignable;
//!
//! fn main() {
//!     let mut siv = Cursive::default();
//!
//!     let panel = Panel::new(DummyView)
//!         .title("Hello, world!")
//!         .fixed_width(20)
//!         .align_center();
//!
//!     siv.add_fullscreen_layer(panel);
//!     // siv.run()
//! }
//! ```
//!
//! This is the preferred way as it is *chainable* and consistent with cursive's
//! `Boxable` and `Identifiable` traits.
//!
//! As an alternative you can use the `AlignedView` constructors directly:
//!
//! ```rust
//! use cursive::Cursive;
//! use cursive::view::Boxable;
//! use cursive::views::{Panel, DummyView};
//! use cursive_aligned_view::AlignedView;
//!
//! fn main() {
//!     let mut siv = Cursive::default();
//!
//!     let panel = Panel::new(DummyView)
//!         .title("Hello, world!")
//!         .fixed_width(20);
//!     let aligned = AlignedView::with_center(panel);
//!
//!     siv.add_fullscreen_layer(aligned);
//!     // siv.run()
//! }
//! ```
//!
//! ## Supported Alignments
//!
//! | Alignment     | Construction method   |
//! |---------------|-----------------------|
//! | top left      | `align_top_left`      |
//! | top center    | `align_top_center`    |
//! | top right     | `align_top_right`     |
//! | center left   | `align_center_left`   |
//! | center        | `align_center`        |
//! | center right  | `align_center_right`  |
//! | bottom left   | `align_bottom_left`   |
//! | bottom center | `align_bottom_center` |
//! | bottom right  | `align_bottom_right`  |

use cursive::view::{View, Selector};
use cursive::event::{AnyCb, Event, EventResult};
use cursive::direction::Direction;
use cursive::align::{Align, HAlign, VAlign};
use cursive::{Printer, Vec2, Rect};

/// Use this trait to extend all `cursive::view::View` instances to support
/// the `align_...` methods.
///
/// This trait provides *chainable* constructors for the `AlignedView`.
///
/// # Usage Example
///
/// ```rust
/// use cursive::Cursive;
/// use cursive::view::Boxable;
/// use cursive::views::{Panel, DummyView};
/// use cursive_aligned_view::Alignable;
///
/// fn main() {
///     let mut siv = Cursive::default();
///
///     let panel = Panel::new(DummyView)
///         .title("Hello, world!")
///         .fixed_width(20)
///         .align_top_center(); // constructing `AlignedView`
///
///     siv.add_fullscreen_layer(panel);
///     // siv.run()
/// }
/// ```
pub trait Alignable: View + Sized {
    /// Align a child view at the top-left of the parent.
    fn align_top_left(self) -> AlignedView<Self> {
        AlignedView::with_top_left(self)
    }

    /// Align a child view at the top-center of the parent.
    fn align_top_center(self) -> AlignedView<Self> {
        AlignedView::with_top_center(self)
    }

    /// Align a child view at the top-right of the parent.
    fn align_top_right(self) -> AlignedView<Self> {
        AlignedView::with_top_right(self)
    }

    /// Align a child view at the center-left of the parent.
    fn align_center_left(self) -> AlignedView<Self> {
        AlignedView::with_center_left(self)
    }

    /// Align a child view at the center of the parent.
    fn align_center(self) -> AlignedView<Self> {
        AlignedView::with_center(self)
    }

    /// Align a child view at the center-right of the parent.
    fn align_center_right(self) -> AlignedView<Self> {
        AlignedView::with_center_right(self)
    }

    /// Align a child view at the bottom-left of the parent.
    fn align_bottom_left(self) -> AlignedView<Self> {
        AlignedView::with_bottom_left(self)
    }

    /// Align a child view at the bottom-center of the parent.
    fn align_bottom_center(self) -> AlignedView<Self> {
        AlignedView::with_bottom_center(self)
    }

    /// Align a child view at the bottom-right of the parent.
    fn align_bottom_right(self) -> AlignedView<Self> {
        AlignedView::with_bottom_right(self)
    }
}

impl<T: View> Alignable for T {}

/// This struct aligns a child view with a given alignment.
///
/// The child view will have its minimum allowed size. Additionally, the child
/// may get cropped if it is larger than the available size.
///
/// The padded space around the child view is filled with the `View` color from
/// cursive's color palette.
///
/// # Usage
///
/// The `AlignedView` may be used in 2 different ways:
///
/// 1. Via the `Alignable` composition trait
/// 2. Via normal constructors
///
/// ## Using Alignable
///
/// ```rust
/// use cursive::Cursive;
/// use cursive::view::Boxable;
/// use cursive::views::{Panel, DummyView};
/// use cursive_aligned_view::Alignable;
///
/// fn main() {
///     let mut siv = Cursive::default();
///
///     let panel = Panel::new(DummyView)
///         .title("Hello, world!")
///         .fixed_width(20)
///         .align_top_center(); // `align_...` methods from `Alignable`
///
///     siv.add_fullscreen_layer(panel);
///     // siv.run()
/// }
/// ```
///
/// ## Constructors
///
/// ```rust
/// use cursive::Cursive;
/// use cursive::view::Boxable;
/// use cursive::views::{Panel, DummyView};
/// use cursive_aligned_view::AlignedView;
///
/// fn main() {
///     let mut siv = Cursive::default();
///
///     let panel = Panel::new(DummyView)
///         .title("Hello, world!")
///         .fixed_width(20);
///     let aligned = AlignedView::with_bottom_center(panel); // constructor
///
///     siv.add_fullscreen_layer(aligned);
///     // siv.run()
/// }
/// ```
pub struct AlignedView<T> {
    view: T,
    alignment: Align,
    last_size: Vec2,
    offset: Vec2,
    needs_relayout: bool,
}

impl<T: View> AlignedView<T> {
    fn new(view: T, alignment: Align) -> Self {
        Self {
            view,
            alignment,
            last_size: Vec2::new(0, 0),
            offset: Vec2::new(0, 0),
            needs_relayout: false,
        }
    }

    /// Wrap a child view and align it at the top-left of the parent.
    pub fn with_top_left(view: T) -> Self {
        Self::new(view, Align::new(HAlign::Left, VAlign::Top))
    }

    /// Wrap a child view and align it at the top-center of the parent.
    pub fn with_top_center(view: T) -> Self {
        Self::new(view, Align::new(HAlign::Center, VAlign::Top))
    }

    /// Wrap a child view and align it at the top-right of the parent.
    pub fn with_top_right(view: T) -> Self {
        Self::new(view, Align::new(HAlign::Right, VAlign::Top))
    }

    /// Wrap a child view and align it at the center-left of the parent.
    pub fn with_center_left(view: T) -> Self {
        Self::new(view, Align::new(HAlign::Left, VAlign::Center))
    }

    /// Wrap a child view and align it at the center of the parent.
    pub fn with_center(view: T) -> Self {
        Self::new(view, Align::new(HAlign::Center, VAlign::Center))
    }

    /// Wrap a child view and align it at the center-right of the parent.
    pub fn with_center_right(view: T) -> Self {
        Self::new(view, Align::new(HAlign::Right, VAlign::Center))
    }

    /// Wrap a child view and align it at the bottom-left of the parent.
    pub fn with_bottom_left(view: T) -> Self {
        Self::new(view, Align::new(HAlign::Left, VAlign::Bottom))
    }

    /// Wrap a child view and align it at the bottom-center of the parent.
    pub fn with_bottom_center(view: T) -> Self {
        Self::new(view, Align::new(HAlign::Center, VAlign::Bottom))
    }

    /// Wrap a child view and align it at the bottom-right of the parent.
    pub fn with_bottom_right(view: T) -> Self {
        Self::new(view, Align::new(HAlign::Right, VAlign::Bottom))
    }

    /// Set the alignment of this view to top-left.
    pub fn set_top_left(&mut self) {
        self.alignment = Align::new(HAlign::Left, VAlign::Top);
        self.needs_relayout = true;
    }

    /// Set the alignment of this view to top-center.
    pub fn set_top_center(&mut self) {
        self.alignment = Align::new(HAlign::Center, VAlign::Top);
        self.needs_relayout = true;
    }

    /// Set the alignment of this view to top-right.
    pub fn set_top_right(&mut self) {
        self.alignment = Align::new(HAlign::Right, VAlign::Top);
        self.needs_relayout = true;
    }

    /// Set the alignment of this view to center-left.
    pub fn set_center_left(&mut self) {
        self.alignment = Align::new(HAlign::Left, VAlign::Center);
        self.needs_relayout = true;
    }

    /// Set the alignment of this view to center.
    pub fn set_center(&mut self) {
        self.alignment = Align::new(HAlign::Center, VAlign::Center);
        self.needs_relayout = true;
    }

    /// Set the alignment of this view to center-right.
    pub fn set_center_right(&mut self) {
        self.alignment = Align::new(HAlign::Right, VAlign::Center);
        self.needs_relayout = true;
    }

    /// Set the alignment of this view to bottom-left.
    pub fn set_bottom_left(&mut self) {
        self.alignment = Align::new(HAlign::Left, VAlign::Bottom);
        self.needs_relayout = true;
    }

    /// Set the alignment of this view to bottom-center.
    pub fn set_bottom_center(&mut self) {
        self.alignment = Align::new(HAlign::Center, VAlign::Bottom);
        self.needs_relayout = true;
    }

    /// Set the alignment of this view to bottom-right.
    pub fn set_bottom_right(&mut self) {
        self.alignment = Align::new(HAlign::Right, VAlign::Bottom);
        self.needs_relayout = true;
    }

    /// Get the current alignment of this view.
    pub fn alignment(&self) -> &Align {
        &self.alignment
    }
}

impl<T: View> View for AlignedView<T> {
    fn draw(&self, printer: &Printer) {
        let offset_printer = printer.offset(self.offset).cropped(self.last_size);
        self.view.draw(&offset_printer);
    }

    fn layout(&mut self, size: Vec2) {
        self.offset = Vec2::new(
            self.alignment.h.get_offset(self.last_size.x, size.x),
            self.alignment.v.get_offset(self.last_size.y, size.y),
        );

        let x = std::cmp::min(size.x, self.last_size.x);
        let y = std::cmp::min(size.y, self.last_size.y);

        self.view.layout(Vec2::new(x, y));

        self.needs_relayout = false;
    }

    fn needs_relayout(&self) -> bool {
        self.needs_relayout || self.view.needs_relayout()
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.last_size = self.view.required_size(constraint);

        constraint
    }

    fn on_event(&mut self, ev: Event) -> EventResult {
        self.view.on_event(ev)
    }

    fn call_on_any<'a>(&mut self, sel: &Selector, cb: AnyCb<'a>) {
        self.view.call_on_any(sel, cb);
    }

    fn focus_view(&mut self, sel: &Selector) -> Result<(), ()> {
        self.view.focus_view(sel)
    }

    fn take_focus(&mut self, source: Direction) -> bool {
        self.view.take_focus(source)
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        self.view.important_area(view_size)
    }
}
