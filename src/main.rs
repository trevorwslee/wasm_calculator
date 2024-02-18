use leptos::*;
fn main() {
    mount_to_body(move || view! { <App/> });
}
fn App() -> impl IntoView {
  let who = "World"; 
  view! {
    {
      view ! {
        <div class="test-class">Hello, {who}!</div>
      }
    }
  }
}
