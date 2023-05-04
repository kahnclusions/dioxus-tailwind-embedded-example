#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    message: String,
    status: String,
}

pub fn use_doggo(cx: Scope, breed: String) -> Option<Result<&String, &reqwest::Error>> {
    let future = use_future(cx, (&breed,), |(breed,)| async move {
        reqwest::get(format!("https://dog.ceo/api/breed/{}/images/random", breed))
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });
    match future.value() {
        Some(Ok(response)) => Some(Ok(&response.message)),
        Some(Err(e)) => Some(Err(e.to_owned())),
        None => None,
    }
}

pub fn Doggo(cx: Scope) -> Element {
    let breed = use_state(cx, || "poodle".to_string());
    let f_doggo = use_doggo(cx, breed.to_string());
    cx.render(rsx! {
        div {
            select {
                name: "breed",
                value: "{breed}",
                oninput: move |evt| breed.set(evt.value.clone()),
                option {
                    value: "poodle",
                    "Poodle"
                },
                option {
                    value: "terrier",
                    "Terrier"
                },
                option {
                    value: "husky",
                    "Husky"
                },
            },
            cx.render(match f_doggo {
                None => rsx! {
                    div {
                        "Come here boy..."
                    }
                },
                Some(Ok(dog_pic)) => rsx! {
                    div {
                        "Your dog picture has arrived:",
                        img {
                            src: "{dog_pic}"
                        }
                    }
                },
                Some(Err(_)) => rsx! {
                    div {
                        "There was an error fetching your dog. He got distracted by another dog."
                    }
                },
            })
        }
    })
}
