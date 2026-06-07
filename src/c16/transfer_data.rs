/// Transfer Data Between Threads with Message Passing
///
/// One increasingly popular approach to ensuring safe concurrency is message passing, where threads
/// or actors communicate by sending each other messages containing data.
///
/// To accomplish message-sending concurrency, Rust's standard library provides an implementation of
/// channels. A channel is a general programming concept by which data is sent from one thread to
/// another.
///
/// You can imagine a channel in programming as being like a directional channel of water, such as a
/// stream or a river. If you put something like a rubber duck into a river, it will travel
/// downstream to the end of the waterway.
///
/// A channel has two halves: a transmitter and a receiver. The transmitter half is the upstream
/// locaiton where you put the rubber duck into the river, and the receiver half is where the rubber
/// duck ends up downstream. One part of your code calls methods on the transmitter with the data
/// you want to send, and another part checks the receiving end for arriving messages. A channel is
/// said to be closed if either the transmitter or receiver half is dropped.
///
///     use std::sync::mpsc;
///
///     fn main() {
///         let (tx, rx) = mpsc::channel();
///     }
///     Listing 16-6: Creating a channel and assigning the two halves to tx and rx
/// We create a channel using the mpsc::channel function; mpsc stands for multiple producer, single
/// consumer. In short, the way Rust's standard library implements channels means a channel can have
/// multiple sending ends that produce values but only one receiving end that consumes those values.
/// Imagine multiple streams flowing together into one big river: Everything sent down any of the
/// streams will end up in one river at the end.
///
/// The mpsc::channel function returns a tuple, the first element of which is the sending end--the
/// transmitter--and the second element of which is the receiving end--the receiver. The
/// abbreviations tx and rx are traditionally used in many fields for transmitter and receiver,
/// respectively, so we name our variables as such to indicate each end.
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
