use leptos::*;
use leptos::logging::log;
use web_sys::MouseEvent;
use std::cell::RefCell;
use rusty_dumb_tools::calculator::*;
fn main() {
    mount_to_body(move || view! { <App/> });
}
fn App() -> impl IntoView {
  let calculator_ref = RefCell::new(DumbCalculator::new_full());
  let (clicked_value, set_clicked_value) = create_signal(String::from(""));
  let on_clicked = move |ev: MouseEvent| {
    let value = event_target_value(&ev);
    log!("* clicked value [{}]", value);
    set_clicked_value.set(value);
  };
  view! {
    {
      move || view! {
        <div class="test-class"> {
          let mut calculator = calculator_ref.borrow_mut();
          let value = clicked_value.get();
          if !value.is_empty() {
            calculator.push(value.as_str()).unwrap();
          }
          let result_value = calculator.get_display_sized(10);
          format!("[{}]", result_value)
        } </div>
      }
    }
    <button on:click=on_clicked value="1">1</button>
    <button on:click=on_clicked value="+">+</button>
    <button on:click=on_clicked value="2">2</button>
    <button on:click=on_clicked value="=">=</button>
  }
}
