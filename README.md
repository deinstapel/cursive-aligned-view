<h1 align="center">Welcome to cursive-aligned-view 👋</h1>
<p align="center">
  <a href="https://github.com/deinstapel/cursive-aligned-view/actions">
    <img src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fdeinstapel.github.io%2Fcursive-aligned-view%2Fstable-build.json" alt="stable build">
  </a>
  <a href="https://github.com/deinstapel/cursive-aligned-view/actions">
    <img src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fdeinstapel.github.io%2Fcursive-aligned-view%2Fnightly-build.json" alt="nightly build">
  </a>
  <a href="https://crates.io/crates/cursive-aligned-view">
    <img alt="crates.io" src="https://img.shields.io/crates/v/cursive-aligned-view.svg">
  </a>
  <a href="https://docs.rs/cursive-aligned-view">
    <img alt="Docs.rs" src="https://docs.rs/cursive-aligned-view/badge.svg">
  </a>
  <a href="https://github.com/deinstapel/cursive-aligned-view/blob/master/LICENSE">
    <img alt="GitHub" src="https://img.shields.io/github/license/deinstapel/cursive-aligned-view.svg">
  </a>
  <a href="http://spacemacs.org">
    <img src="https://cdn.rawgit.com/syl20bnr/spacemacs/442d025779da2f62fc86c2082703697714db6514/assets/spacemacs-badge.svg" />
  </a>
  <a href="http://makeapullrequest.com">
    <img alt="PRs Welcome" src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg">
  </a>
  <br>
  <i>A view wrapper for
  <a href="https://github.com/gyscos/cursive">gyscos/cursive</a>
  views which aligns child views</i>
</p>

---

This project provides an `AlignedView` for [gyscos/cursive](https://github.com/gyscos/cursive) views which makes it possible to align the child view (center, left, right, top, bottom). The `AlignedView` uses the `required_size` reported by the child view and fills the rest of the available space with the views background color.

## How does it look like? `demo` [![terminalizer](https://img.shields.io/badge/GIF-terminalizer-blueviolet.svg)](https://github.com/faressoft/terminalizer)

<details>
  <summary>Expand to view</summary>
  <img src="assets/demo.gif" alt="aligned-view demo">
</details>

## Usage

Simply add to your `Cargo.toml`

```toml
[dependencies]
cursive-aligned-view = "^0"
```

### Aligning a child view

The easiest way to align a view is via the `Alignable` trait:

```rust
use cursive_aligned_view::Alignable;

let aligned = child_view.align_center();
```

This is the preferred way as it is *chainable* and consistent with cursive's `Boxable` and `Identifiable` traits.

As an alternative you can use the `AlignedView` constructors directly:

```rust
use cursive_aligned_view::AlignedView;

let aligned = AlignedView::with_center(child_view);
```

Look into the [documentation](https://docs.rs/cursive-aligned-view) for a detailed explanation on the API.

### Supported Alignments

| Alignment     | Construction method   |
|---------------|-----------------------|
| top left      | `align_top_left`      |
| top center    | `align_top_center`    |
| top right     | `align_top_right`     |
| center left   | `align_center_left`   |
| center        | `align_center`        |
| center right  | `align_center_right`  |
| bottom left   | `align_bottom_left`   |
| bottom center | `align_bottom_center` |
| bottom right  | `align_bottom_right`  |

## Troubleshooting

If you find any bugs/unexpected behaviour or you have a proposition for future changes open an issue describing the current behaviour and what you expected.

## Development [![cargo test](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fdeinstapel.github.io%2Fcursive-aligned-view%2Fcargo-test.json)](https://github.com/deinstapel/cursive-aligned-view/actions)

### Running the tests

#### Preparing integration tests

To perform the test it's advisable to install [`cargo-insta`](https://github.com/mitsuhiko/insta) in case your tests fail and you want to inspect the received output.
```
$ cargo install cargo-insta
```

You will also need insta to validate new tests you add or update older ones.

#### Running all test suites

Just run

```
$ cargo test
```

to execute all available tests.

---

Or if you want to interactively inspect failed tests
```
$ cargo insta test
```

#### shields.io endpoints

[shields.io](https://shields.io) endpoints are generated inside the `./target/shields` folder. They are used in this README.

## Authors

**Fin Christensen**

> [:octocat: `@fin-ger`](https://github.com/fin-ger)  
> [:elephant: `@fin_ger@weirder.earth`](https://weirder.earth/@fin_ger)  
> [:bird: `@fin_ger_github`](https://twitter.com/fin_ger_github)  

<br>

**Johannes Wünsche**

> [:octocat: `@jwuensche`](https://github.com/jwuensche)  
> [:elephant: `@fredowald@mastodon.social`](https://mastodon.social/web/accounts/843376)  
> [:bird: `@Fredowald`](https://twitter.com/fredowald)  

## Show your support

Give a :star: if this project helped you!
