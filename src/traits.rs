/// Traits: Defining Shared Behaviour
///
/// A trait defines the functionality a particular type has and can share with other types. We can
/// use traits to defined shared behaviour in an abstract way. We can use trait bounds to specify
/// that a generic type can be any type that has certain behaviour.
///
///     Note: Traits are similar to a feature often called interfaces in other languages, although
///     with some differences.
///
/// Defining a Trait
///
/// A type's behaviour consists of the methods we can call on that type. Different types share the
/// same behaviour if we can call the same methods on all of those types. Trait definitions are a
/// way to group method signatures together to define a set of behaviours necessary to accomplish
/// some purpose.
///
/// For example, let's say we have multiple structs that hold various kinds and amounts of text: a
/// NewsArticle struct that holds a new story filed in a particular location and a SocialPost that
/// can have, at most, 280 characters along with metadata that indicates whether it was a new post,
/// or repost, or a reply to another post.
///
/// We want to make a media aggregator library crate named aggregator that can display summaries of
/// data that might be stored in a NewsArticle or SocialPost instance. To do this, we need a
/// summary from each type, and we'll request that summary by calling a summarize method on an
/// instance. Listing 10-12 shows the definition of a public Summary trait that expresses this
/// behaviour.
///
///     pub trait Summary {
///         fn summarize(&self) -> String;
///     }
///     Listing 10-12: A Summary trait that consists of the behaviour provided by a summarize method
/// Here, we declare a trait using the trait keyword and then the trait's name, which is Summary in
/// this case. We also declare this trait as pub so that crates depending on this crate can make
/// use of this trait too, as we'll see in a few examples. Inside the curly brackets, we declare
/// the method signatures that describes the behaviours of the types that implement this trait,
/// which in this case is fn summarize(&self) -> String.
///
