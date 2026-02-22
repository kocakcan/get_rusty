/*
* Method Syntax
*
* Methods are similar to functions: we declare them with the fn keyword and a name, they can have
* parameters and a return value, and they contain some code that's run when the method is called
* from somewhere else. Unlike functions, methods are defined within the context of a struct (or an
* enum or a trait object), and their first paramater is always self, which represents the instance
* of the struct the method is being called on.
*
* Defining Methods
*
* Let's change the area function that has a Rectangle instance as a parameter and instead an area
* method defined on the Rectangle struct, as shown below:
*
*   #[derive(Debug)]
*   struct Rectangle {
*       width: u32,
*       height: u32,
*   }
*
*   impl Rectangle {
*       fn area(&self) -> u32 {
*           self.width * self.height
*       }
*   }
*
*   fn main() {
*       let rect1 = Rectangle {
*           width: 30,
*           height: 50,
*       };
*
*       println!(
*           "The area of the rectangle is {} square pixels.",
*           rect1.area()
*       );
*   }
* To define the function within the context of Rectangle, we start an impl (implementation) block
* for Rectangle. Everything withing this impl block will be associated with the Rectangle type.
* Then we move the area function within the impl curly brackets and change the first (and in this
* case, only) parameter to be self in the signature and everywhere within the body. In main, where
* we called the area function and passed rect1 as an argument, we can instead use method syntax to
* call the area method on our Rectangle instance. The method syntax goes after an instance: we add
* a dot followed by the method name, parantheses, and any arguments.
*
* In the signature for area, we &self instead of rectangle: Rectangle. The &self is actually short
* for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block
* is for. Methods must have a parameter named self of type Self for their first parameter, so Rust
* lets you abbreviate this with only the name self in the first parameter spot. Note that we still
* need to use the & in front of the self shorthand to indicate that this method borrows the Self
* instance, just as we did in rectangle: &Rectangle. Methods can take ownership of self, borrow
* self immutably, as we've done here, or borrow self mutably, just as they can any other parameter.
*
* We chose &self here for the same reason we used &Rectangle in the function version: we don't want
* to take ownership, and we just want to read the data in the struct, not write to it. If we wanted
* to change the instance that we've called the method on as pair of what the method does, we'd use
* &mut self as the first parameter. Having a method that takes ownership of the instance by using
* just as the first parameter is rare; this technique is usually used when the method
* transforms self into something else and you want to prevent the caller from using the original
* instance after the transformation.
*
* The main reason for using methods instead of functions, in addition to providing method syntax
* and not having to repeat the type of self in every method's signature, is for organization. We've
* put all the things we can do with an instance of a type in one impl block rather than making
* future users of our code search for capabilities of Rectangle in various places in the library we
* provide.
*
* Note that we can choose to give a method the same name as one of the struct's fields. For
* example, we can define a method on Rectangle that is also named width:
*
*   impl Rectangle {
*       fn width(&self) -> bool {
*           self.width > 0
*       }
*   }
*
*   fn main() {
*       let rect1 = Rectangle {
*           width: 30,
*           height: 50,
*       };
*
*       if rect1.width() {
*           println!("The rectangle has a nonzero width; it is {}", rect1.width);
*       }
*   }
* Here, we're choosing to make the width method return true if the value in the instace's width
* field is greater than 0 and false if the value is 0: we can use a field within method of the same
* name for any purpose. In main, when we follow rect1.width with parantheses, Rust knows we mean
* the method width. When we don't use parantheses, Rust knows we mean the field width.
*
* Often, but not always, when we give a method the same name as a field we want it to only return
* the value in the field and do nothing else. Methods like this are called getters, and Rust does
* not implement them automatically for struct fields as some other languages do. Getters are useful
* because you can make the field private but the method public, and thus enable read-only access to
* that field as part of the type's public API.
*
* Methods with More Parameters
*
* Let's practice using methods by implementing a second method on the Rectangle struct. This time
* we want an instance of Rectangle to take another instance of Rectangle and return true if the
* second Rectangle can fit completely within self (the first Rectangle); otherwise; it should
* return false. That is, once we've defined the can_hold method.
*
*   fn main() {
*       let rect1 = Rectangle {
*           width: 30,
*           height: 50,
*       };
*       let rect2 = Rectangle {
*           width: 10,
*           height: 40,
*       };
*       let rect3 = Rectangle {
*           width: 60,
*           height: 45
*       };
*
*       println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
*       println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
*   }
* We know we want to define a method, so it will be within the impl Rectangle block. The method
* name will be can_hold, and it will take an immutable borrow of another Rectangle as a parameter.
* We can tell what the type of paramter will be by looking at the code that calls the method:
* rect1.can_hold(&rect2) passes in &rect2, which is an immutable borrow to rect2 (rather than
* write, which would mean we'd need a mutable borrow), and we want main to retain ownership of
* rect2 so we can use it again after calling the can_hold method. The return value of can_hold will
* be a Boolean, and the implementation will check whether the width and height of self are greater
* than the width and height of the other Rectangle respectively.
*
*   impl Rectangle {
*       fn area(&self) -> u32 {
*           self.width * self.height
*       }
*
*       fn can_hold(&self, other: &Rectangle) -> bool {
*           self.width > other.width && self.height && other.height
*       }
*   }
* When we run this code with the main function we'll get our desired output. Methods can take
* multiple parameters that we add to the signature after the self parameter, and those parameters
* work just like parameters in functions.
*
* Associated Functions
*
* All functions defined within an impl block are called associated functions because they're
* associated with the type named after the impl. We can define associated functions as functions
* that don't have self as their first parameter (and thus are not methods) because they don't need
* an instance of the type to work with. We've already used one function like this: the String::from
* function that's defined on the String type.
*
* Associated function that aren't methods are often used for constructors that will return a new
* instance of the struct. These are often called new, but new isn't a special name and isn't built
* into the language. For example, we could choose to provide an associated function named square
* that would have one dimension parameter and use that as both width and height, thus making it
* easier to create a square Rectangle rather than having to specify the same value twice:
*
*   impl Rectangle {
*       fn square(size: u32) -> Self {
*           Self {
*               width: size,
*               height: size,
*           }
*       }
*   }
* The Self keywords in the return type and in the body of the function are aliases for the type
* that appears after the impl keyword, which in this case is Rectangle.
*
* To call this associated function, we use the :: syntax with the struct name; let sq =
* Rectangle::square(3); is an example. This function is namespaced by the struct: the :: syntax is
* used for both associated functions and namespaces created by modules.
*
* Multiple impl Blocks
*
* Each struct is allowed to have multiple impl blocks.
*
*   impl Rectangle {
*       fn area(&self) -> u32 {
*           self.width * self.height
*       }
*   }
*
*   impl Rectangle {
*       fn can_hold(&self, other: &Rectangle) -> bool {
*           self.width > other.width && self.height > other.height
*       }
*   }
* There's no reason to separate these methods into multiple impl blocks here, but this is valid
* syntax. They are useful in other situations.
*
* Method Calls are Syntactic Sugar for Function Calls
*
* Using the concepts we've discussed so far, we can now see how method calls are syntactic sugar
* for function calls. For example:
*
*   impl Rectangle {
*       fn area(&self) -> u32 {
*           self.width * self.height
*       }
*
*       fn set_width(&mut self, width: u32) {
*           self.width = width;
*       }
*   }
* And let's say we have a rectangle r. Then the method calls r.area() and r.set_width(2) are
* equivalent to this:
*
*   let mut r = Rectangle {
*       width: 1,
*       height: 2
*   };
*   let area1 = r.area();
*   let area2 = Rectangle::area(&r);
*   assert_eq!(area1, area2);
*
*   r.set_width(2);
*   Rectangle::set_width(&mut r, 2);
* The method call r.area() becomes Rectangle::area(&r). The function name is the associated
* function Rectangle::area. The function argument is the &self parameter. Rust automatically
* inserts the borrowing operator &.
*
* The method call r.set_width(2) similarly becomes Rectangle::set_width(&mut r, 2). This method
* expects &mut self, so the first argument is a mutable borrow &mut r. The second argument is
* exactly the same, the number 2.
*
* As described earlier, Rust will insert as many references and dereferences as needed to make the
* types match up for the self parameter. For example, here are two equivalent calls to area for a
* mutable reference to a boxed rectangle:
*
*   let r = &mut Box::new(Rectangle {
*       width: 1,
*       height: 2,
*   });
*   let area1 = r.area().
*   let area2 = Rectangle::area(&**r);
*   assert_eq!(area1, area2);
* Rust will add tow dereferences (once for the mutable reference, once for the box) and then one
* immutable borrow because area expects &Rectangle. Note that this is also a situation where a
* mutable reference is "downgraded" into a shared reference. Conversely, you would not be allowed
* to call set_width on a value of type &Rectangle or &Box<Rectangle>.
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(self: &Self) -> u32 {
    //     self.width * self.height
    // }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // These both calls are equivalent
    // fn square(size: u32) -> Rectangle {
    //     Rectangle {
    //         width: size,
    //         height: size,
    //     }
    // }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let mut square = Rectangle::square(5);

    // Rectangle::set_width(&mut square, 10);   /* these both calls are equivalent */
    square.set_width(10);

    println!("The area of rect1 is {}", rect1.area());
    println!("The area of square is {}", square.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
