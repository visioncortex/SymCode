use visioncortex::{ColorHsv, Sampler, Shape};

extern crate cfg_if;

cfg_if::cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

// Check Saturation and Value in HSV
pub(crate) fn is_black(color: &ColorHsv) -> bool {
    const BLACK_LIMIT: f64 = 0.25;
    color.s <= BLACK_LIMIT &&
    color.v <= BLACK_LIMIT
}

/// First attempt: When locating Finder patterns, make the shape square before checking if it is a circle to deal with distortion
pub(crate) fn make_shape_square(original: &Shape) -> Shape {
    if original.image.width == original.image.height {
        original.clone()
    } else {
        let max_side = std::cmp::max(original.image.width, original.image.height);
        Shape {
            image: Sampler::resample_image(&original.image, max_side, max_side)
        }
    }
}