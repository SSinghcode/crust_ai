use icons::{ArrowUp, Menu, Paperclip};
use leptos::html::Div;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::MouseEvent;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::button_group::ButtonGroup;
use crate::components::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign};
use crate::domain::chat::models::{Message, MessageRole};
use crate::domain::chat::server_fns::get_reply;

#[derive(Clone, Copy, PartialEq)]
enum ChatMode {
    Developer,
    General,
}

#[component]
pub fn HomePage() -> impl IntoView {
    let sidenav_open = use_context::<RwSignal<bool>>()
        .expect("HomePage: sidenav_open context not found");

    let mode = RwSignal::new(ChatMode::Developer);
    let input = RwSignal::new(String::new());
    let messages: RwSignal<Vec<Message>> = RwSignal::new(Vec::new());
    let is_loading = RwSignal::new(false);
    let next_id = RwSignal::new(0u64);
    let container = NodeRef::<Div>::new();

    // Scroll to bottom whenever messages or loading state change
    Effect::new(move |_| {
        let _ = messages.get();
        let _ = is_loading.get();
        if let Some(el) = container.get() {
            el.set_scroll_top(el.scroll_height());
        }
    });

    let dev_variant = Signal::derive(move || {
        if mode.get() == ChatMode::Developer { ButtonVariant::Default } else { ButtonVariant::Ghost }
    });
    let gen_variant = Signal::derive(move || {
        if mode.get() == ChatMode::General { ButtonVariant::Default } else { ButtonVariant::Ghost }
    });

    let send_message = move || {
        let text = input.get_untracked().trim().to_string();
        if text.is_empty() || is_loading.get_untracked() {
            return;
        }

        let user_id = next_id.get_untracked();
        next_id.update(|n| *n += 1);

        messages.update(|msgs| {
            msgs.push(Message { id: user_id, role: MessageRole::User, content: text.clone() });
        });
        input.set(String::new());
        is_loading.set(true);

        spawn_local(async move {
            let reply = get_reply(text).await
                .unwrap_or_else(|_| "Something went wrong. Please try again.".to_string());

            let reply_id = next_id.get_untracked();
            next_id.update(|n| *n += 1);

            messages.update(|msgs| {
                msgs.push(Message { id: reply_id, role: MessageRole::Assistant, content: reply });
            });
            is_loading.set(false);
        });
    };

    view! {
        <div class="flex flex-col h-full">

            // ── Header ──────────────────────────────────────────────
            <header class="flex items-center justify-between px-4 py-3 border-b bg-background shrink-0">
                <button
                    class="p-1 rounded-md hover:bg-accent text-muted-foreground"
                    on:click=move |_| sidenav_open.update(|v| *v = !*v)
                >
                    <Menu class="size-5" />
                </button>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    {move || if mode.get()==ChatMode::Developer{"🦀 Developer mode"} else{"⊕ General mode"}}
                </Button>
            </header>

            // ── Messages / empty state ───────────────────────────────
            <div node_ref=container class="flex-1 overflow-y-auto px-4">
                {move || {
                    if messages.get().is_empty() && !is_loading.get() {
                        // Empty state — centred, like Claude/ChatGPT landing
                        view! {
                            <div class="h-full flex flex-col items-center justify-center gap-3 text-center">
                                <span class="text-5xl select-none">"🦀"</span>
                                <p class="text-xl font-semibold">"What can I help you with?"</p>
                                <p class="text-sm text-muted-foreground">
                                    "Ask a question, explore code, or just start a conversation."
                                </p>
                            </div>
                        }
                        .into_any()
                    } else {
                        view! {
                            <div class="flex flex-col gap-4 py-4">
                                // Message list
                                <For
                                    each=move || messages.get()
                                    key=|msg| msg.id
                                    children=|msg| match msg.role {
                                        MessageRole::User => view! {
                                            <div class="flex justify-end">
                                                <div class="bg-primary text-primary-foreground rounded-2xl rounded-br-sm px-4 py-2.5 max-w-[80%] text-sm whitespace-pre-wrap">
                                                    {msg.content}
                                                </div>
                                            </div>
                                        }
                                        .into_any(),
                                        MessageRole::Assistant => view! {
                                            <div class="flex gap-3 items-start">
                                                <div class="size-7 rounded-full bg-primary/10 flex items-center justify-center text-sm shrink-0 mt-0.5">
                                                    "🦀"
                                                </div>
                                                <div class="bg-muted rounded-2xl rounded-tl-sm px-4 py-2.5 max-w-[80%] text-sm whitespace-pre-wrap">
                                                    {msg.content}
                                                </div>
                                            </div>
                                        }
                                        .into_any(),
                                    }
                                />

                                // Typing indicator while waiting for reply
                                <Show when=move || is_loading.get()>
                                    <div class="flex gap-3 items-start">
                                        <div class="size-7 rounded-full bg-primary/10 flex items-center justify-center text-sm shrink-0 mt-0.5">
                                            "🦀"
                                        </div>
                                        <div class="bg-muted rounded-2xl rounded-tl-sm px-4 py-3.5">
                                            <div class="flex gap-1 items-center">
                                                <span class="size-1.5 rounded-full bg-muted-foreground/60 animate-bounce [animation-delay:0ms]" />
                                                <span class="size-1.5 rounded-full bg-muted-foreground/60 animate-bounce [animation-delay:150ms]" />
                                                <span class="size-1.5 rounded-full bg-muted-foreground/60 animate-bounce [animation-delay:300ms]" />
                                            </div>
                                        </div>
                                    </div>
                                </Show>
                            </div>
                        }
                        .into_any()
                    }
                }}
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
                                send_message();
                            }
                        }
                    />
                    <InputGroupAddon align=InputGroupAddonAlign::BlockEnd class="border-t px-2 py-2">
                        <div class="flex items-center justify-between w-full">
                            <div class="flex items-center gap-1">
                                <ButtonGroup>
                                    <Button
                                        size=ButtonSize::Sm
                                        variant=dev_variant
                                        on:click=move |_| mode.set(ChatMode::Developer)
                                    >
                                        "🦀 Developer"
                                    </Button>
                                    <Button
                                        size=ButtonSize::Sm
                                        variant=gen_variant
                                        on:click=move |_| mode.set(ChatMode::General)
                                    >
                                        "⊕ General"
                                    </Button>
                                </ButtonGroup>
                                <Button size=ButtonSize::Icon variant=ButtonVariant::Ghost class="size-8">
                                    <Paperclip class="size-4" />
                                </Button>
                            </div>
                            <Button
                                size=ButtonSize::Icon
                                class="size-8 rounded-full"
                                on:click=move |_: MouseEvent| send_message()
                            >
                                <ArrowUp class="size-4" />
                            </Button>
                        </div>
                    </InputGroupAddon>
                </InputGroup>
            </div>
        </div>
    }
}
