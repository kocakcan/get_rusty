/***
 * Enums and Pattern Matching
 *
 * Enums allow you to define a type by enumerating its possible variant. An enum can encode meaning
 * along with data.
 *
 * Defining an Enum
 *
 * Where structs give you a way of grouping together related fields and data, like a Rectangle with
 * its width and height, enums give you a way of saying a value is one of a possible set of values.
 * For example, we may want to say that Rectangle is one of a set of possible shapes that also
 * includes Circle and Triangle. To do this, Rust allows us to encode these possibilities as an
 * enum.
 *
 * Let's look at a situation we might want to express in code and see why enums are useful and more
 * appropriate than structs in this case. Say we need to work with IP addresses. Currently, two
 * major standards are used for IP addresses: version four and version six. Because these are the
 * only possibilities for an IP address that our program will come across, we can enumerate all
 * possible variants, which is where enumeration gets its name.
 *
 * Any IP address can be either four or a verion six address, but not both at the same time. That
 * property of IP addresses makes the enum data structure appropriate because an enum value can
 * only be one of its variants. Both version four and version six addresses are still fundamentally
 * IP addresses, so they should be treated as the same type when the code is handling situations
 * that apply to any kind of IP address.
 *
 * We can express this concept in code by defining an IpAddrKind enumeration and listing the
 * possible kinds an IP address can be, V4 and V6. These are the variants of the enum:
 *
 *  enum IpAddrKind {
 *      V4,
 *      V6,
 *  }
 * IpAddrKind is now a custom data type that we can use elsewhere in our code.
 *
 * Enum Values
 *
 * We can create instances of each of the two variants of IpAddrKind like this:
 *
 *  let four = IpAddrKind::V4;
 *  let six = IpAddrKind::V6;
 * Note that the variants of the enum a re namespaced under its identifier, and we use a double
 * colon to separate the tow. This is useful because now both values IpAddrKind::V4 and
 * IpAddrKind::V6 are of the same type: IpAddrKind. We can then, for instance, define a function
 * that takes any IpAddrKind:
 *
 *  fn route(ip_kind: IpAddrKind) {}
 * And we can call this function with either variant:
 *
 *  route(IpAddrKind::V4);
 *  route(IpAddrKind::V6);
 * Using enums has even more advantages. Thinking more about our IP address type, at the moment we
 * don't have a way to store the actual IP address data; we only know what kind it is.
 *
 *  enum IpAddrKind {
 *      V4,
 *      V6,
 *  }
 *
 *  struct IpAddr {
 *      kind: IpAddrKind,
 *      address: String,
 *  }
 *
 *  let home = IpAddr {
 *      kind: IpAddrKind::V4,
 *      address: String::from("127.0.0.1"),
 *  };
 *
 *  let loopback = IpAddr {
 *      kind: IpAddrKind::V6,
*       address: String::from("::1"),
 *  };
* Here we've defined a struct IpAddr that has two fields: a kind field that is of type IpAddrKind
* (the enum we defined previously) and an address field of type String.
 */

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("I'm using {:?}", ip_kind);
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}
