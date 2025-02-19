use dioxus::prelude::*;

fn main() {
    dioxus_native::launch(app);
}

fn app() -> Element {
    let mut new_cards = use_signal(|| {
        vec![
            "Task 1".to_string(),
            "Task 2".to_string(),
            "Task 3".to_string(),
        ]
    });

    let mut active_cards = use_signal(|| Vec::<String>::new());

    let mut done_cards = use_signal(|| Vec::<String>::new());

    rsx! {
      head {
        style { {CSS} }
      }
      body {
        div { class: "board",
            div { class: "column",
                h2 { "New" }
                {
                    new_cards.iter().enumerate().map(|(index, card)| rsx! {
                        div {
                            class: "card",
                            key: "{index}",
                            onclick: move |_| {
                                // Remove the card from "To Do" and push it to "Done".
                                let card_value = new_cards.with_mut(|cards| cards.remove(index));
                                active_cards.with_mut(|done| done.push(card_value));
                            },
                            "{card}"
                        }
                    })
                }
            },
            div { class: "column",
                h2 { "Active" }
                {
                    active_cards.iter().enumerate().map(|(index, card)| rsx! {
                        div {
                            class: "card",
                            key: "{index}",
                            onclick: move |_| {
                                let card_value = active_cards.with_mut(|cards| cards.remove(index));
                                done_cards.with_mut(|done| done.push(card_value));
                            },
                            "{card}"
                        }
                    })
                }
            },
            // "Done" Column
            div { class: "column",
                h2 { "Done" }
                {
                    done_cards.iter().enumerate().map(|(index, card)| rsx! {
                        div {
                            class: "card",
                            key: "{index}",
                            onclick: move |_| {
                              done_cards.with_mut(|done| done.remove(index));
                            },
                            "{card}"
                        }
                    })
                }
            }
        }
      }
    }
}

const CSS: &str = r#"
.board {
    display: flex;
    gap: 20px;
    padding: 20px;
}
.column {
    flex: 1;
    background: #f0f0f0;
    padding: 10px;
    border-radius: 8px;
}
.card {
    background: #fff;
    padding: 10px;
    margin-bottom: 10px;
    border-radius: 4px;
    cursor: pointer;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}
"#;
