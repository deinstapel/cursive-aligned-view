use cursive_aligned_view as _; // This needs to be imported to enable the blueprints.
use serde_json::json;

fn main() {
    // This is the same layout as the `simple` example, but using a builder config.
    let config = json! ({
        "Panel": {
            "title": "Hello, world!",
            "view": "DummyView",
            "with": [
                {"fixed_width": 20},
                "align_center",
                "full_screen",
            ]
        },
    });

    let context = cursive::builder::Context::new();

    let mut siv = cursive::default();
    siv.add_layer(context.build(&config).unwrap());
    siv.run();
}
