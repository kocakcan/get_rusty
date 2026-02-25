/*
* Methods and Ownership
*
* Methods muts be called on structs that have the necessary permissions. As a running example, we
* will use these three methods that take &self, &mut self, and self, respectively.
*
*   impl Rectangle {
*       fn area(&self) -> u32 {
*           self.width * self.height
*       }
*
*       fn set_width(&mut self, width: u32) {
*           self.width = width;
*       }
*
*       fn max(self, other: Rectangle) -> Rectangle {
*           Rectangle {
*               width: self.width.max(other.width),
*               height: self.height.max(other.height),
*           }
*       }
*   }
*
* Reads and Writes with &self and &mut self
*
* If we make an owned rectangle with let rect = Rectangle { ... }, then rect has R and O
* permissions. With those permissions, it is permissible to call the area and max methods:
*
*   let rect = Rectangle {
*       width: 0,
*       height: 0,
*   };
*
*   -> rect         | RO
*   -> rect.height  | RO
*   -> rect.width   | RO
*
*   println!("{}", rect.area());            -> area() requires R permission
*
*   let other_rect = Rectangle { width: 1, height: 1 };
*
*   -> other_rect           | RO
*   -> other_rect.height    | RO
*   -> other_rect.width     | RO
*
*   let max_rect = rect.max(other_rect);    -> max() requires RO permissions
*
*   -> rect                 | No permissions (moved)
*   -> rect.width           | No permissions (moved)
*   -> rect.height          | No permissions (moved)
*   -> other_rect           | No permissions (moved)
*   -> other_rect.height    | No permissions (moved)
*   -> other_rect.width     | No permissions (moved)
*
* However, if we try to call set_width, we are missing the W permission:
*
*   let rect = Rectangle {
*       width: 0,
*       height: 0,
*   };
*
*   rect.set_width(0);                      -> set_width() requires RW permissions
*
* Rust will reject this program with the corresponding error:
*
*   cannot borrow `rect` as mutable, as it is not declared as mutable
*
* We will get a similar error if we try to call set_width on an immutable reference to a Rectangle,
* even if the underlying rectangle is mutable:
*
*   -> Added the mut keyword to the let-binding
*   let mut rect = Rectangle {
*       width: 0,
*       height: 0,
*   };
*
*   -> rect         | RWO
*   -> rect.width   | RWO
*   -> rect.height  | RWO
*
*   rect.set_width(1);                      -> this is now ok
*
*   let rect_ref = &rect;                   -> immutable borrowing requires R
*
*   -> *rect_ref            | R
*   -> rect                 | No permissions (borrowed)
*   -> rect.width           | No permissions (borrowed)
*   -> rect.height          | No permissions (borrowed)
*   -> rect_ref             | RO
*   -> (*rect_ref).width    | R
*   -> (*rect_ref).height   | R
*
*   rect_ref.set_width(2);                  -> but this is still not ok
*                                           -> as it is an immutable reference 😉
*
* Moves with self
*
* Calling a method that expects self will move the input struct (unless the struct implements
* Copy). For example, we cannot use a Rectangle after passing it to max:
*
*   let rect = Rectangle {
*       width: 0,
*       height: 0,
*   };
*
*   -> rect         | RO
*   -> rect.width   | RO
*   -> rect.height  | RO
*
*   let other_rect = Rectangle {
*       width: 1,
*       height: 1,
*   };
*
*   > other_rect        | RO
*   > other_rect.width  | RO
*   > other_rect.height | RO
*
*   let max_rect = rect.max(other_rect);    -> max requires RO permissions and moves rect
*
*   -> rect                 | No permissions (moved by max_rect)
*   -> rect.width           | No permissions (moved by max_rect)
*   -> rect.height          | No permissions (moved by max_rect)
*   -> other_rect           | No permissions (moved by max_rect)
*   -> other_rect.width     | No permissions (moved by max_rect)
*   -> other_rect.height    | No permissions (moved by max_rect)
* Once we call rect.max(..), we move rect and so lose all permissions on it. Trying to compile this
* program would give us the following error:
*
*   borrow of moved value: `rect`
* A similar situation arises if we try to call a self method on a reference. For instance, say we
* tried to make a method set_to_max that assigns self to the output of self.max(..):
*
*   impl Rectangle {
*       fn set_to_max(&mut self, other: Rectangle) {
*           -> self             | R0
*           -> *self            | RW
*           -> (*self).width    | RW
*           -> (*self).height   | RW
*           -> other            | RO
*           -> other.width      | RO
*           -> other.height     | RO
*
*           *self = self.max(other);    -> dereferencing + assigning requires RW
*                                       -> max moves its parameter but self doesn't have O
*                                       permission
*       }
*   }
* Then we can see that self is missing O permissions in the operation self.max(..). Rust therefore
* rejects this program with the following error:
*
*   cannot move out of `*self` which is behind a mutable reference
*
* Good Moves vs Bad Moves
*
* For the case of Rectangle, it actually is safe to move out of *self, even though Rust doesn't let
* you do it. For example, if we simulate a program that calls the rejected set_to_max, you can see
* how nothing unsafe occurs:
*
*   impl Rectangle {
*       fn set_to_max(&mut self, other: Rectangle) {
*           let max = self.max(other);
*           *self = max;
*       }
*   }
*
*   fn main() {
*       let mut rect = Rectangle { width: 0, height: 1 };
*       let other_rect = Rectangle { width: 1, height: 0 };
*       rect.set_to_max(other_rect);
*   }
*
* The reason it's safe to move out of *self is because Rectangle does not own any heap data. In
* fact, we can actually get Rust to compile set_to_max by simply adding #[derive(Copy, Clone)] to
* the definition of Rectangle:
*
*   #[derive(Copy, Clone)]
*   struct Rectangle {
*       width: u32,
*       height: u32,
*   }
*
*   impl Rectangle {
*       fn set_to_max(&mut self, other: Rectangle) {
*           -> *self            | RW
*           -> self             | RO
*           -> (*self).width    | RW
*           -> (*self).height   | RW
*           -> other            | RO
*           -> other.width      | RO
*           -> other.heigth     | RO
*
*           *self = self.max(other);     -> dereferencing + assigning requires RW
*       }
*   }
* Notice that unlike before, self.max(other) no longer requires the O permission on *self or other.
* Remember that self.max(other) desugars to Rectangle::max(*self, other). The dereference *self
* does not require ownership over *self i Rectangle is copyable.
*
* You might wonder: why doesn't Rust automatically derive Copy for Rectangle? Rust does not
* auto-derive Copy for stability across API changes. Imagine that the author of the Rectangle type
* decided to add a name: String field. Then all client code that relies on Rectangle being Copy
* would suddenly get rejected by the compiler. To avoid that issue, API authors must explicitly add
* #[derive(Copy, Clone)] to indicate that they expect their struct to always be Copy.
*
* To better understand the issue, let's run a simulation. Say we added name: String to Rectangle.
* What would happen if Rust allowed set_to_max to compile?
*
*   struct Rectangle {
*       width: u32,
*       height: u32,
*       name: String,
*   }
*
*   impl Rectangle {
*       fn set_to_max(&mut self, other: Rectangle) {
*          let max = self.max(other);   -> L2
*          drop(*self); -> this is usually implicit, but added here for clarity
*
*           *self = max;
*       }
*   }
*
*   fn main() {
*       let mut r1 = Rectangle {
*           width: 9,
*           height: 9,
*           name: String::from("r1")
*       };
*       let r2 = Rectangle {
*           width: 16,
*           height: 16,
*           name: String::from("r2")
*       };
*       r1.set_to_max(r2);
*   }
*
* In this program, we call set_to_max with two rectangles r1 and r2. self is a mutable reference to
* r1 and other is a move of r2. After calling self.max(other), the max method consumes ownership of
* both rectangles. When max returns, Rust deallocates both strings "r1" and "r2" in the heap.
* Notice the problem: at the location L2, *self is supposed to be readable and writeable. However
* (*self).name (actually r1.name) has been deallocated.
*
* Therefore when we do *self = max, we encounter undefined behaviour. Then we overwrite *self, Rust
* will implicitly drop the data previously in *self. To make that behaviour explicit, we have added
* drop(*self). After calling drop(*self), Rust attempts to free (*self).name a second time. That
* action is a double-free, which is undefined behaviour.
*
* So remember: when you see an error like "cannot move out of *self", that's usually because you're
* trying to call a self method on a reference like &self or &mut self. Rust is protecting you from
* a double-free.
*
* Summary
*
* Structs let you create custom types that are meaningful for your domain. By using structs, you
* can keep associated pieces of data connected to each other and name each piece to make your code
* clear. In impl blocks, you can define functions that are associated with your type, and methods
* are a kind of associated function that let you specify the behaviour that instances of your
* structs have.
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };

    // rect.set_width(1);
    Rectangle::set_width(&mut rect, 1);

    let rect_ref = &mut rect;

    rect_ref.set_width(2);

    println!("{:?}", *rect_ref);
}
