extern crate rraw;
extern crate serde_json;

use rraw::authorize;
use rraw::new;
use rraw::comments;


fn main() {
    match authorize() {
        Ok(auth_data) => {
            let subreddit = "programming";
            match new(&auth_data.access_token, subreddit) {
                Ok(links) => {
                    for link in links.iter() {
                        println!("{:?}", link.title);
                        println!("{:?}", comments(&auth_data.access_token, subreddit, &link.id))
                    }
                },
                Err(e) => println!("error = {:?}", e)
            }
        },
        Err(e) => println!("error = {:?}", e)
    };
}
