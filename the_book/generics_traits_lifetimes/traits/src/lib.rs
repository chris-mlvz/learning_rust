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

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
}
