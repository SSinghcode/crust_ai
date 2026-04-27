use icons::{ArrowUp, Menu, Paperclip};
use leptos::prelude::*;
use leptos::web_sys::MouseEvent;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::button_group::ButtonGroup;
use crate::components::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign};

#[derive(Clone, Copy, PartialEq)]
enum ChatMode {
    Developer,
    General,
}

#[component]
pub fn HomePage() -> impl IntoView {
    let mode = RwSignal::new(ChatMode::Developer);
    let input = RwSignal::new(String::new());

    let dev_variant = Signal::derive(move || {
        if mode.get() == ChatMode::Developer { ButtonVariant::Default } else { ButtonVariant::Ghost }
    });
    let gen_variant = Signal::derive(move || {
        if mode.get() == ChatMode::General { ButtonVariant::Default } else { ButtonVariant::Ghost }
    });

    let on_send = move |_: MouseEvent| {
        let text = input.get_untracked();
        if !text.trim().is_empty() {
            input.set(String::new());
        }
    };

    view! {
        <div class="flex flex-col h-full">

            // ── Header ──────────────────────────────────────────────
            <header class="flex items-center justify-between px-4 py-3 border-b bg-background shrink-0">
                <button class="p-1 rounded-md hover:bg-accent text-muted-foreground">
                    <Menu class="size-5" />
                </button>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    "🦀 Developer mode"
                </Button>
            </header>

            // ── Empty state ──────────────────────────────────────────
            <div class="flex-1 flex flex-col items-center justify-center gap-3 text-center">
                <span class="text-5xl select-none">"🦀"</span>
                <p class="text-sm text-muted-foreground">"What can I help you with?"</p>
            </div>

            // ── Input area ───────────────────────────────────────────
            <div class="px-4 pt-3 pb-2 border-t bg-background shrink-0">
                <InputGroup class="rounded-xl shadow-sm">
                    <textarea
                        data-slot="input-group-control"
                        class="flex-1 py-3 px-3 w-full text-sm bg-transparent rounded-xl outline-none resize-none min-h-[52px] max-h-40 placeholder:text-muted-foreground"
                        placeholder="Ask a developer question..."
                        prop:value=move || input.get()
                        on:input=move |ev| input.set(event_target_value(&ev))
                        on:keydown=move |ev| {
                            if ev.key() == "Enter" && !ev.shift_key() {
                                ev.prevent_default();
                                let text = input.get_untracked();
                                if !text.trim().is_empty() {
                                    input.set(String::new());
                                }
                            }
                        }
                    />
                    <InputGroupAddon align=InputGroupAddonAlign::BlockEnd class="border-t px-2 py-2">
                        <div class="flex items-center justify-between w-full">
                            <div class="flex items-center gap-1">
                                <ButtonGroup>
                                    <Button size=ButtonSize::Sm variant=dev_variant on:click=move |_| mode.set(ChatMode::Developer)>
                                        "🦀 Developer"
                                    </Button>
                                    <Button size=ButtonSize::Sm variant=gen_variant on:click=move |_| mode.set(ChatMode::General)>
                                        "⊕ General"
                                    </Button>
                                </ButtonGroup>
                                <Button size=ButtonSize::Icon variant=ButtonVariant::Ghost class="size-8">
                                    <Paperclip class="size-4" />
                                </Button>
                            </div>
                            <Button size=ButtonSize::Icon class="size-8 rounded-full" on:click=on_send>
                                <ArrowUp class="size-4" />
                            </Button>
                        </div>
                    </InputGroupAddon>
                </InputGroup>
            </div>
        </div>
    }
}
