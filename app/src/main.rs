use yew::{use_state, html, function_component, Callback, MouseEvent, use_effect_with_deps};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod hooks;
pub use hooks::use_interval;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokePing, catch)]
    pub async fn ping(message: String) -> Result<JsValue, JsValue>;
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    let images = vec![
        "https://bm5cdn.azureedge.net/banner/20220512142607OSI1200113.jpg",
        "https://bm5cdn.azureedge.net/banner/20220512102929BN001153664.jpeg",
        "https://bm5cdn.azureedge.net/banner/20220428094623OSI1200113.jpg"
    ];
    let active_image_index = use_state(|| 0);
    let millis = use_state(||0);

    let increment_active_image_index = {
        let active_image_index = active_image_index.clone();
        let updated_image_index = if *active_image_index == images.len() - 1 {
            0
        } else {
            *active_image_index + 1
        };

        Callback::from(move |_: MouseEvent| active_image_index.set(updated_image_index))
    };

    let decrement_active_image_index  = {
        let active_image_index = active_image_index.clone();
        let updated_image_index = if *active_image_index == 0 {
            images.len() - 1
        } else {
            *active_image_index - 1
        };

        Callback::from(move |_: MouseEvent| active_image_index.set(updated_image_index))
    };
    
    fn call_tauri_ping() {
        log::info!("calling the tauri");
        let message = String::from("Hello from Yew");
        spawn_local(async move {
            match ping(message).await {
                Ok(message) => {
                    log::info!("{}", message.as_string().unwrap());
                }
                Err(e) => {
                    log::info!("{}", e.as_string().unwrap());
                }
            }
        })
    }

    let trigger_tauri_ping = {
        Callback::from(move |_: MouseEvent| call_tauri_ping())
    };

    {
        let millis = millis.clone();

        use_effect_with_deps(move |_| {
            millis.set(2000); || ()
        }, ());
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
        <div>
            <button onclick={trigger_tauri_ping}>{"ping"}</button>
            <button onclick={increment_active_image_index}>{"increment"}</button>
            <button onclick={decrement_active_image_index}>{"decrement"}</button>
            <img src={images[*active_image_index]} />
        </div>
    }
}
