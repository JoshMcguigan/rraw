extern crate rraw;

use rraw::authorize;
use rraw::new;
use rraw::listing::Thing;
use rraw::listing::Listing;
use rraw::comments;

fn main() {
    match authorize() {
        Ok(auth_data) => {
            let subreddit = "programming";
            match new(&auth_data.access_token, subreddit) {
                Ok(listing) => {
                    match listing {
                        Listing::Listing {children} => {
                            for child in &children {
                                match child {
                                    &Thing::Link(ref link) => {
                                        println!("{:?}: {:?}", link.id, link.title);
                                        println!("{:?}", comments(&auth_data.access_token, &link.subreddit, &link.id));
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                },
                Err(e) => println!("error = {}", e)
            }
        },
        Err(e) => println!("error = {}", e)
    };
}
