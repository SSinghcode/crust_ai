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
    let merged_class = tw_merge!(
        "whitespace-pre-wrap p-4 [&_span]:text-xs rounded-md bg-muted overflow-x-auto",
        class
    );

    view! {
        <pre class=merged_class>
            <code inner_html=highlight_code(&code, Some(language.as_ref()), None) />
        </pre>
    }
}
