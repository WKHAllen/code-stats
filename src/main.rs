//! Code statistics viewer.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![allow(non_snake_case)]

mod classes;
mod components;
mod icons;
mod services;

use components::App;

fn main() {
    dioxus_desktop::launch(App);
}
