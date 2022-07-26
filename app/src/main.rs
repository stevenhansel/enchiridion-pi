use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, UseStateHandle};

mod hooks;
pub use hooks::use_interval;

// https://stackoverflow.com/questions/53214434/how-to-return-a-rust-closure-to-javascript-via-webassembly
// https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html#passing-rust-closures-to-imported-javascript-functions
// https://rustwasm.github.io/wasm-bindgen/examples/closures.html

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = getImages, catch)]
    pub async fn get_images() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = listenMediaUpdate, catch)]
    pub async fn listen_media_update(callback: &Closure<dyn FnMut()>) -> Result<JsValue, JsValue>;
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    let images: UseStateHandle<Vec<String>> = use_state(|| vec![]);
    let active_image_index: UseStateHandle<Option<usize>> = use_state(|| None);
    let millis = use_state(|| 0);
    // let _unlisten_media_update: UseStateHandle<Option<js_sys::Function>> = use_state(|| None);

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

    fn attach_media_update_listener() {
        spawn_local(async move {
            let cb = Closure::wrap(Box::new(|| log::info!("test pingedd")) as Box<dyn FnMut()>);

            // TODO: handle unlistening part in the useeffect cleanup
            match listen_media_update(&cb).await {
                Ok(data) => data.unchecked_into::<js_sys::Function>(),
                Err(err) => {
                    panic!("Something went wrong when receiving the stuff, {:#?}", err)
                }
            };

            cb.forget();
        });
    }

    fn initialize_millis(millis: UseStateHandle<u32>) {
        millis.set(3000);
    }

    {
        let images = images.clone();
        use_effect_with_deps(
            move |_| {
                fetch_images(images);
                || ()
            },
            (),
        );
    }

    {
        use_effect_with_deps(
            move |_| {
                attach_media_update_listener();
                || ()
            },
            (),
        )
    }

    {
        let millis = millis.clone();

        use_effect_with_deps(
            move |_| {
                initialize_millis(millis);
                || ()
            },
            (*images).clone(),
        );
    }

    {
        let active_image_index = active_image_index.clone();
        let is_index_available = match *active_image_index {
            Some(_) => true,
            None => false,
        };
        let mut is_reached_end = false;

        use_interval(
            move || {
                let updated_image_index = if is_reached_end {
                    0
                } else {
                    *active_image_index + 1
                };

                active_image_index.set(updated_image_index);
            },
            *millis,
        );
    }

    html! {
        <div class="container">
            if let Some(index) = *active_image_index {
                <img class="image" src={images.get(index).unwrap().clone()} />
                <div class="contributor">
                    <p>{"Computer Engineering BINUS"}</p>
                    <p>{"Lukas Linardi, Steven Hansel"}</p>
                </div>
            }
        </div>
    }
}
