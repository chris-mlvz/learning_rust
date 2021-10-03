// Exposing Paths with the pub Keyword
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Starting Relative Paths with super
fn serve_order() {}

mod back_of_house {
    fn incorrect_order() {
        cook_order();
        super::serve_order()
    }
    fn cook_order() {}
}
