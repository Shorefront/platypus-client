//! Product Offering Price Module
//! This module defines the Product Offering Price Model
//! It is part of TMForum Open API 6.2.0 Product Offering module
use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
use tmflib::{HasLastUpdate, HasName, HasValidity};
use tmflib::common::price::Price;
// use crate::model::common::table::GenericTable;
use crate::model::common::list::GenericListWithAdd;
// use tmf_leptos::common::has_description::HasDescription;
use tmf_leptos::common::{SingleRow, price};
use tmf_leptos::traits::has_name::NamedClass;
use tmf_leptos::traits::has_validity::HasValidity;
use tmf_leptos::traits::has_lastupdate::HasLastUpdate;
use tmf_leptos::common::price::Price;

use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn ProductOfferingPriceList() -> impl IntoView {
    let price1 = ProductOfferingPrice::new("Internet Pricing");
    let price2 = ProductOfferingPrice::new("WAN Pricing");
    let prices = vec![price1, price2];
    view! {
        <div class="list">
            <GenericListWithAdd items=prices />
        </div>
        <div class="detail">
            <Outlet />
        </div>
    }
}

#[component]
pub fn ProductOfferingPriceDetail() -> impl IntoView {
    view! {
        <p>"Product Offering Price"</p>
    }
}

#[component]
pub fn ProductOfferingPriceAdd() -> impl IntoView {
    let mut new_item = ProductOfferingPrice::new("New Price");
    let valid_dirty : bool = false;
    let (name, set_name) = signal(new_item.get_name());
    let (valid,valid_write) = signal(valid_dirty);
    let (version, set_version) = signal("1.0".to_string());
    let (last_update, set_last_update) = signal(new_item.get_last_update().unwrap_or_default());
    let price = Price::default();
    let (price_read, price_write) = signal(price);

    
    // Update proto-object
    name.with(|n| new_item.set_name(n));
    version.with(|v| {
        set_last_update.set(ProductOfferingPrice::get_timestamp());
        new_item.version = Some(v.clone())
    });
    last_update.with(|lu| new_item.set_last_update(lu));
    let mut validity = new_item.get_validity().unwrap_or_default();
    valid.with(|f| {
        // Flag has been set to dirty, so we know to update the validity in the proto-object
        if *f {
            new_item.set_validity(validity.clone());
            // Reset dirty flag
            valid_write.set(false);
        }
    });
    view! {
        <form>
            <NamedClass item=&new_item signal=set_name />
            <HasValidity period=&mut validity dirty=valid_write />
            <Price label="Price".to_string() read=price_read write=price_write />
            <fieldset>
                <legend>"Details"</legend>
                    <SingleRow id="version" label="Version" read=version write=set_version />
                    <HasLastUpdate lu_read=last_update lu_write=set_last_update />
                </fieldset>
            <div class="debug">"Will create new catalog called: " { name } " with version " {version} " and last update " {last_update} </div>
            <button type="submit">"Submit"</button>
        </form>
        <Outlet />
    }
}
