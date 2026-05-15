use leptos::prelude::*;
use tw_merge::tw_merge;

use super::highlight_code::highlight_code;
use super::highlight_language::HighlightLanguage;

#[component]
pub fn SyntectHighlighterCode(
    #[prop(into)] code: String,
    language: HighlightLanguage,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let copied = RwSignal::new(false);

    #[cfg(not(feature = "ssr"))]
    let code_for_copy = code.clone();

    let on_copy = move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(clipboard) = web_sys::window().and_then(|w| Some(w.navigator().clipboard())) {
                let _ = clipboard.write_text(&code_for_copy);
            }
        }
        copied.set(true);
        set_timeout(move || copied.set(false), std::time::Duration::from_secs(2));
    };

    let lang_label = language.as_ref().to_lowercase();
    let highlighted = highlight_code(&code, Some(language.as_ref()), None);
    let outer_class = tw_merge!("rounded-lg overflow-hidden border border-white/10", class);

    view! {
        <div class=outer_class>
            // ── Top bar: language label + copy button ────────────────
            <div class="flex items-center justify-between px-4 py-2 bg-zinc-900 border-b border-white/10">
                <span class="text-xs text-zinc-400 font-mono">{lang_label}</span>
                <button
                    class="text-xs text-zinc-400 hover:text-zinc-100 transition-colors font-mono"
                    on:click=on_copy
                >
                    {move || if copied.get() { "✓ Copied!" } else { "Copy" }}
                </button>
            </div>

            // ── Code body ────────────────────────────────────────────
            <pre class="whitespace-pre-wrap p-4 [&_span]:text-xs bg-zinc-900 overflow-x-auto">
                <code inner_html=highlighted />
            </pre>
        </div>
    }
}
