//! Engaged Party Rendering


use leptos::prelude::*;
use crate::common::related_party::RelatedParty;
use tmflib::common::related_party::RelatedParty;

pub mod account_ref;
pub mod related_party_ref;

#[component]
pub fn EngagedParty<'a>(item : &'a RelatedParty) -> impl IntoView {
    // Item has an array of engaged parties, need to render them all.
    // We won't group these as that will depend on the use case, but we will
    // render them in a list.

    view! {
        <fieldset>
            <legend>"Engaged Party"</legend>
            <RelatedParty rp={item} />
        </fieldset>
    }
}