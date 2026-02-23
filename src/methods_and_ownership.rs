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

    // rect_ref.set_width(2);  /* this is not ok as it is not mutable */
    rect_ref.set_width(2);

    println!("{:?}", *rect_ref);
}
