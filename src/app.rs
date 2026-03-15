use image::RgbaImage;
use js_sys::Uint8Array;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Event, HtmlInputElement};

#[component]
pub fn App() -> impl IntoView {
    // Create signal for the target image
    let (rgba_image, set_rgba_image) = signal::<Option<RgbaImage>>(None);
    // Image loading subroutine.
    let on_file_change = move |ev: Event| {
        // First, find the input element that was clicked.
        let input = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        // Now find the file that was loaded by the user.
        let Some(file) = input.files().and_then(|f| f.get(0)) else {
            return;
        };
        // Need to use a local async here to read it in.
        spawn_local(async move {
            // Read the file to a buffer.
            let Ok(buf) = JsFuture::from(file.array_buffer()).await else {
                return;
            };
            let bytes = Uint8Array::new(&buf).to_vec();
            // Now use the image crate to load it as an RBGA image.
            if let Ok(img) = image::load_from_memory(&bytes) {
                set_rgba_image.set(Some(img.to_rgba8()));
            }
        });
    };

    // Image display subroutine
    let display_image = |img: RgbaImage| {
        // Write the bytes of the image into a vector
        let mut png_bytes: Vec<u8> = Vec::new();
        img.write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            image::ImageFormat::Png,
        )
        .unwrap();
        // Use java script magic to convert this into a string url
        let uint8_arr = Uint8Array::from(png_bytes.as_slice());
        let parts = js_sys::Array::of1(&uint8_arr.buffer());
        let opts = web_sys::BlobPropertyBag::new();
        opts.set_type("image/png");
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &opts).unwrap();
        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        // Display the image as an `img` html element.
        view! {
            <p>{format!("Loaded {}×{} image", img.width(), img.height())}</p>
            <img src=url />
        }
    };

    // Create a NodeRef.
    // We will later attach this to the (invisible) input element so we can reference it on the button.
    let input_ref = NodeRef::<leptos::html::Input>::new();

    // When the button is clicked, 'click' the input.
    let on_button_click = move |_| {
        if let Some(input) = input_ref.get() {
            input.click();
        }
    };

    view! {
        <h1>"Refraktor"</h1>
        <div class="image_display">
        <Show
            when=move || {rgba_image.get().is_some()}
            fallback=move || view!{
                <button on:click=on_button_click>"Load Image"</button>
            }
        >
            {move || rgba_image.get().map(display_image)}
        </Show>
        <input
            type="file"
            accept="image/*"
            node_ref=input_ref
            on:change=on_file_change
        />
        </div>
    }
}
