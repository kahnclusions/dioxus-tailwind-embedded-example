#![allow(non_snake_case)]
use dioxus::prelude::*;
use dogs::Doggo;
use serde::{Deserialize, Serialize};

pub mod dogs;
pub mod ui;

pub fn root(cx: Scope) -> Element {
    cx.render(rsx! {
            header {
                class: "sticky top-0 left-0 right-0 h-[40px] bg-black text-white text-center font-display flex flex-row items-center justify-center text-2xl",
                "fetch"
            }
            div {
                class: "mx-auto my-3 max-w-[400px]",
                "Select a dog breed to fetch:",
                Doggo {}
            }
        })
}
