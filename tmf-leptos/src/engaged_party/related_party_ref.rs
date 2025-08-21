//! RelatedPartyRef Module

use leptos::prelude::*;

use tmflib::HasRelatedParty;

pub fn related_part_ref<T : HasRelatedParty>(item : T) -> impl IntoView {
    let first_party = item.get_party(0).unwrap().clone();
    view!{
        <label for="related_party">Related Party</label>
        <input type="text" value={first_party.name} />
    }
}