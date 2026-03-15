#[derive(Debug, Clone)]
struct Refraktor {
    target: image::RgbaImage,
}

impl Refraktor {
    /// Creates a new refraktor with default settings operating on the given image.
    pub fn new(target: image::RgbaImage) -> Self {
        Self { target }
    }

    pub fn find_centers() {}
}
