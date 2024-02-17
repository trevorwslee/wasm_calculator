use leptos::*;
use leptos::logging::log;
use web_sys::MouseEvent;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(move || {
        view! { <App/> }
    });
}
fn App() -> impl IntoView {
    let greeting = "World";
    let (clicked_value, set_clicked_value) = create_signal(String::from(""));
    let on_clicked = move |ev: MouseEvent| {
        let value = event_target_value(&ev);
        log!("* clicked value [{}]", value);
        set_clicked_value.set(value);
    };
     let ov = view! {
        {
            let iv = move || view! { 
                <div class="test-class">
                    {
                        format!("Hello, [{}]!", clicked_value.get())
                    }
                </div>
            };
            iv
        }
        <button on:click=on_clicked value="1">I am 1</button>
        <button on:click=on_clicked value="2">I am 2</button>
    };
    ov
}
