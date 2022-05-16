use yew::{use_state, html, function_component, Callback, MouseEvent, use_effect_with_deps, UseStateHandle};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod hooks;
pub use hooks::use_interval;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = getImages, catch)]
    pub async fn get_images() -> Result<JsValue, JsValue>;
}

struct Image {
    index: usize,
    url: String,
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    let images: UseStateHandle<Vec<String>> = use_state(|| vec!["".to_string()]);
    let active_image_index: UseStateHandle<usize> = use_state(|| 0);
    let millis = use_state(||0);

    fn fetch_images(images: UseStateHandle<Vec<String>>) {
        spawn_local(async move {
            match get_images().await {
                Ok(data) => {
                    let elements: Vec<String> = data.into_serde().unwrap();
                    images.set(elements);
                }
                Err(e) => {
                    log::info!("{}", e.as_string().unwrap());
                }
            }
        });
    }

    fn initialize_millis(millis: UseStateHandle<u32>) {
        millis.set(3000);
    }

    {
        let images = images.clone();
        use_effect_with_deps(move |_| {
            fetch_images(images); || ()
        }, ());
    }

    {
        let millis = millis.clone();

        use_effect_with_deps(move |_| {
            initialize_millis(millis); || ()
        }, (*images).clone());
    }

    {
        let active_image_index = active_image_index.clone();
        let is_reached_end = *active_image_index == images.len() - 1;

        use_interval(move || {
            let updated_image_index = if is_reached_end {
                0
            } else {
                *active_image_index + 1
            };
            
            active_image_index.set(updated_image_index);
        }, *millis);
    }

    html! {
        <div class="container">
            <img class="image" src={images.get(*active_image_index).unwrap().clone()} />
            <div class="contributor">
                <p>{"Computer Engineering BINUS"}</p>
                <p>{"Lukas Linardi, Steven Hansel"}</p>
            </div>
        </div>
    }
}
