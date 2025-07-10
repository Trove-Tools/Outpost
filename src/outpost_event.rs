use scrypto::prelude::*;

use crate::outpost_account::Listing;
/// This component acts as the central hub for all trade emitted events, such as listing creation, listing updates, listing cancellations, and listing purchases.

#[derive(ScryptoSbor, ScryptoEvent)]
struct ListingCreated {
    listing: Listing,
    outpost_account: ComponentAddress,
    nft_id: NonFungibleGlobalId,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct ListingUpdated {
    listing: Listing,
    outpost_account: ComponentAddress,
    nft_id: NonFungibleGlobalId,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct ListingCanceled {
    listing: Listing,
    outpost_account: ComponentAddress,
    nft_id: NonFungibleGlobalId,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct ListingPurchased {
    listing: Listing,
    outpost_account: ComponentAddress,
    nft_id: NonFungibleGlobalId,
}

#[blueprint]
#[events(ListingCreated, ListingUpdated, ListingCanceled, ListingPurchased)]
mod event {

    struct Event {
        emitter_badge_auth: ResourceAddress,
    }

    impl Event {
        pub fn create_event_listener(emitter_badge_auth: ResourceAddress) -> Global<Event> {
            let (event_address_reservation, _event_component_address) =
                Runtime::allocate_component_address(Event::blueprint_id());

            Self { emitter_badge_auth }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .with_address(event_address_reservation)
                .globalize()
        }

        pub fn listing_event(
            &self,
            listing: Listing,
            nft_id: NonFungibleGlobalId,
            emitter_badge: Proof,
        ) {
            emitter_badge.check(self.emitter_badge_auth);
            Runtime::emit_event(ListingCreated {
                listing: listing.clone(),
                outpost_account: listing.outpost_account,
                nft_id,
            });
        }

        pub fn update_listing_event(
            &self,
            listing: Listing,
            nft_id: NonFungibleGlobalId,
            emitter_badge: Proof,
        ) {
            emitter_badge.check(self.emitter_badge_auth);
            Runtime::emit_event(ListingUpdated {
                listing: listing.clone(),
                outpost_account: listing.outpost_account,
                nft_id,
            });
        }

        pub fn cancel_listing_event(
            &self,
            listing: Listing,
            nft_id: NonFungibleGlobalId,
            emitter_badge: Proof,
        ) {
            emitter_badge.check(self.emitter_badge_auth);
            Runtime::emit_event(ListingCanceled {
                listing: listing.clone(),
                outpost_account: listing.outpost_account,
                nft_id,
            });
        }

        pub fn purchase_listing_event(
            &self,
            listing: Listing,
            nft_id: NonFungibleGlobalId,
            emitter_badge: Proof,
        ) {
            emitter_badge.check(self.emitter_badge_auth);

            Runtime::emit_event(ListingPurchased {
                listing: listing.clone(),
                outpost_account: listing.outpost_account,
                nft_id,
            });
        }

        pub fn multi_listing_event(&self, listings: Vec<Listing>, emitter_badge: Proof) {
            emitter_badge.check(self.emitter_badge_auth);

            // Use zip to iterate over both vectors simultaneously
            for listing in listings {
                Runtime::emit_event(ListingCreated {
                    listing: listing.clone(),
                    outpost_account: listing.outpost_account,
                    nft_id: listing.nfgid, // Wrap single NFT ID in a vector to match event structure
                });
            }
        }

        pub fn multi_purchase_event(&self, listings: Vec<Listing>, emitter_badge: Proof) {
            emitter_badge.check(self.emitter_badge_auth);

            // Use zip to iterate over both vectors simultaneously
            for listing in listings {
                Runtime::emit_event(ListingPurchased {
                    listing: listing.clone(),
                    outpost_account: listing.outpost_account,
                    nft_id: listing.nfgid, // Wrap single NFT ID in a vector to match event structure
                });
            }
        }
    }
}
