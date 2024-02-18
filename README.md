
# A Simple WASM Calculator in Rust Using Leptos, and With DumbCalculator


![WASM Calculator](wasm_calculator.png)

## Development Environment

Here I will assume program development tools, like
* Of cause, the [Rust](https://www.rust-lang.org/tools/install) programming language itself.
* The popular [VSCode](https://code.visualstudio.com/download) program development editor / IDE, with the extensions:
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
* Preferably the popular source control tool [GIT](https://git-scm.com/downloads).

## Rust Crates Used

* [Leptos](https://leptos.dev/) -- a Rust framework to develop WASM app in Rust
* [DumbCalculator](https://docs.rs/rusty_dumb_tools/0.1.6/rusty_dumb_tools/calculator/struct.DumbCalculator.html) of [rusty_dumb_tools](https://github.com/trevorwslee/rusty_dumb_tools) -- a simple Rust component that acts like a simple physical calculator. 

## Preparation for WASM Development

WASM development in Rust can be enabled with the [Trunk](https://trunkrs.dev/) tool.
Indeed, Trunk is used here, and you install Trunk like

```
cargo install trunk
```

After installing Trunk, you will also need to add the Rust target `wasm32-unknown-unknown`, like
```
rustup target add wasm32-unknown-unknown
```

## Start with `wasm_calculator` Rust Project

To get kick-started, create a new Rust project `wasm_calculator`, like
```
cargo new wasm_calculator
```

Open the just crated folder `wasm_calculator` with VSCode like
```
cd wasm_calculator
code .
```

In VSCode, open and edit `Cargo.toml` adding the necessary dependencies, like
```
...
[dependencies]
leptos = { version = "0.6.5", features = ["csr"] }
rusty_dumb_tools = {version = "0.1.6"}
```

Add `index.html`, which is sort of the template for your final WASM 'index.html`
```
<!DOCTYPE html>
<html>
  <head><meta charset="UTF-8"></head>
  <body></body>
</html>
```

Your WASM code will be "mounted" to  `<body>` of this `index.html`, lets see it working
```
trunk serve --open
```

This will run the Trunk server serving the `index.html` merging with whatever WASM code that is specified in `main.rs`

The server will keep running, and hot update the page whenever `index.html` or `main.rs` changed

Say, change `<body>` of `index.html` to 
```
	  <body><h3>&mdash; WASM Calculator &mdash;</h3></body>
```

Notice that the browser page is changed accordingly.

The initial generate `main.rs` is actually not WASM code to be "mounted" to `<body>`.
To "mount" some simple WASM code (written in Rust), can change `main.rs` like
```
use leptos::*;
fn main() {
    mount_to_body(move || {
        view! {
            <div style="color:red">Hello, World!</div>
        }
    });
}
```

Again, your modification will be hot-deployed, and you should see that
```
<div style="color:red">Hello, World!</div>
```
is "mounted" to `<body>`, after `<h3>&mdash; WASM Calculator &mdash;</h3>`  

Here is some little insights from the above code:
- `mount_to_body` is the function provided by Leptos to "mount" WASM code (written i Rust) code to `<body>`
- `mount_to_body` can accept a closure which accepts no argument and returns the result of calling the `view!` macro, which is of cause also provided by Leptos.
- Inside `view!`, you write "HTML", like `<div style="color:red">Hello, World!</div>`, which even looks like plain HTML, is in fact valid Rust code to be pre-processed by the macro `view!`.

In fact, normally, you will be coding your WASM code, in an `App()` function, and "mount" it like
```
use leptos::*;
fn main() {
    mount_to_body(move || view! { <App/> });
}
fn App() -> impl IntoView {
    view! {
        <div style="color:red">Hello, World!</div>
    }
}
``` 

As mentioned above, things inside `view!` will be pre-processed to be transformed to Rust code, hence, you should be able to include regular Rust code inside `view!` like
```
fn App() -> impl IntoView {
    let color = "red";
    let who = "World"; 
    view! {
        <div style={format!("color:{}", color)}>Hello, {who}!</div>
    }
}
```
Apparently, you can enclose regular Rust code inside `{}` as above `{format!("color:{}", color)}` and `Hello, {who}!`

You can even "nest" `view!` like
```
fn App() -> impl IntoView {
  let color = "red";
  let who = "World"; 
  view! {
    {
      view ! {
        <div style={format!("color:{}", color)}>Hello, {who}!</div>
      }
    }
  }
}
```
Why "new" `view!`? Hopefully, it will become apparent in later sections. 

Lets try to put the styling as CSS. You can put your CSS in `index.html` and use it in `App()` like

`index.html`
```
<!DOCTYPE html>
<style>
  .test-class {
    color: green;
  }
</style>
...
```

`main.rs`
```
...
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
```

Now, lets add two buttons to it to make it interactive.

Note that Leptos will render HTML once only -- like the above `App()` will only be called once to "generate" initial HTML code -- any updates are trigger by Leptos "signals".

Therefore, to make it interactive, not only you will need add some interactive HTML elements, like `<button>`, you will also need to make use of "signals" like

First, lets add two `<button>`s and be able to log to the browser's console when any of the button is clicked:
```
```