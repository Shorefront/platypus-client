
use leptos::prelude::*;
use tmflib::common::related_party::RelatedParty;
use tmflib::HasRelatedParty;

#[component]
pub fn RelatedParty<'a>(rp : &'a RelatedParty) -> impl IntoView {
    // Item has an array of related parties, need to render them all.
    // We won't group these as that will depend on the use case, but we will
    // render them in a list.

    let name = rp.name.clone().unwrap_or("Unknown".to_string());
    view! {
        <label>{name}</label>
        <input type="checkbox" checked=true  disabled=false />
            <br/>
    }
}

#[component]
pub fn RelatedPartyList<T : HasRelatedParty, 'a>(item: &'a T) -> impl IntoView {
    // Render a list of related parties
    let rp = item.get_party(0).cloned().unwrap_or_default();
    view! {
        <fieldset>
            <legend>"Related Parties"</legend>
            <RelatedParty rp={&rp} />
        </fieldset>
    }
}