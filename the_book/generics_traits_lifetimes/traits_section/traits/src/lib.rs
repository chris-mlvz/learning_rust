pub mod text {
    // * Traits: Defining Shared Behavior

    // * Defining a Trait
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }

        fn summarize_author(&self) -> String;
    }

    // * Implementing a Trait on a Type
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        // fn summarize(&self) -> String {
        //     format!("{}, by {} ({})", self.headline, self.author, self.location)
        // }
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    // * Traits as Parameters
    // pub fn notify(item: &impl Summary) {
    //     println!("Breaking news! {}", item.summarize());
    // }

    // * Trait Bound Syntax
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // * Two parameters with traits impl and trait bound syntax

    // * Two parameters than implement Summary
    // pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    // *Two parameters of the same type than implement Summary
    // pub fn notify<T: Summary>(item1: &T, item2: &T) {

    // * Specifying Multiple Trait Bounds with the + Syntax
    // pub fn notify(item: &(impl Summary + Display)) {
    // pub fn notify<T: Summary + Display>(item: &T) {

    // * Clearer Trait Bounds with where Clauses
    // * Normal
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // }
    // * Using where
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {
    // }

    // * Returning Types that Implement Traits
    // fn returns_summarizable() -> impl Summary {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from("of course, as you probably already know, people"),
    //         reply: false,
    //         retweet: false,
    //     }
    // }

    // ! Returning either a NewsArticle or a Tweet isn’t allowed due to
    // ! restrictions around how the impl Trait
    // fn returns_summarizable(switch: bool) -> impl Summary {
    //     if switch {
    //         NewsArticle {
    //             headline: String::from("Penguins win the Stanley Cup Championship!"),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         Tweet {
    //             username: String::from("horse_ebooks"),
    //             content: String::from("of course, as you probably already know, people"),
    //             reply: false,
    //             retweet: false,
    //         }
    //     }
    // }
}
