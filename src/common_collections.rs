/*
 * Common Collections
 *
 * Rust's standard library includes a number of very useful data structures called collections.
 * Most other data types represent one specific value, but collections can contain multiple values.
 * Unlike the built-in array and tuple types, the data that these collections point to is stored on
 * the heap, which means the amount of data does not need to be known at compile time and can grow
 * or shrink as the program runs. Each kind of collection has different capabilities and costs, and
 * choosing an appropriate one for your current situation is a skill you'll develop over time.
 *
 * Storing Lists of Values with Vectors
 *
 * Vec<T> is also known as vector. Vectors allow you to store more than one value in a single data
 * structure that pulls all the values next to each other in memory. Vectors can only stores values
 * of the same type. They are useful when you have a list of items, such as the lines of text in a
 * file or the prices of items in a shopping cart.
 *
 * Creating a New Vector
 *
 * To create a new empty vector, we call the Vec::new function, as shown below:
 *
 *  let v: Vec<i32> = Vec::new();
 * Note that we added a type annotation here. Because we aren't inserting any values into this
 * vector, Rust doesn't know what kind of elements we intend to store. This is an important point.
 * Vectors are implemented using generics and Vec<T> type provided by the standard library can hold
 * any type. When we create a vector to hold a specific type, we can specify the type within angle
 * brackets.
 *
 * More often, you'll create Vec<T> with initial values and Rust will infer the type of value you
 * want to store, so you rarely need to do this type annotation. Rust conveniently provides the
 * vec! macro, which will create a new vector that holds the values you give it. The code below
 * creates a new Vec<i32> that holds the values 1, 2, and 3. The integer type is i32 because that's
 * the default integer type.
 *
 *  let v = vec![1, 2, 3];
 * Because we've given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the
 * type annotation isn't necessary.
 *
 * Updating a Vector
 *
 * To create a vector and then add elements to it, we can use the push method, as shown below:
 *
 *  let mut v = Vec::new();
 *
 *  v.push(5);
 *  v.push(6);
 *  v.push(7);
 *  v.push(8);
 * As with any variable, if we want to be able to change its value, we need to make it mutable
 * using the mut keyword. The numbers we place inside are all of type i32, and Rust infers this
 * from the data, so we don't need the Vec<i32> annotation.
 *
 * Reading Elements of Vectors
 *
 * There are two ways to reference a value stored in a vector: via indexing or by using the get
 * method. In the following examples, we've annotated the types of the values that are returned
 * from these functions for extra clarity.
 *
 *  let v = vec![1, 2, 3, 4, 5];
 *
 *  let third: &i32 = &v[2];
 *  println!("The third element is {third}");
 *
 *  let third: Option<&i32> = v.get(2);
 *  match third {
 *      Some(third) => println!("The third element is {third}"),
 *      None => println!("There is no third element"),
 *  }
 * Note a few details here. We use the index value of 2 to get the third element because vectors
 * are indexed by number, starting at zero. Using & and [] gives us a reference to the element at
 * the index value. When we use the get method with the index passed as an argument, we get an
 * Option<&T> that we can use with match.
 *
 * Rust provides these two ways to reference an element so you can choose how the program behaves
 * when you try to use an index value outside the range of existing elements. As an example, let's
 * see what happens when we have a vector of five elements and then we try to access an element at
 * index 100 with each technique, as shown below.
 *
 *  let v = vec![1, 2, 3, 4, 5];
 *
 *  let does_not_exist = &v[100];
 *  let does_not_exist = v.get(100);
 * When we run this code, the first [] method will cause the program to panic because it references
 * a nonexistent element. This method is best used when you want your program to crash if there's
 * an attempt to access an element past the end of the vector.
 *
 * When the get method is passed an index that is outside the vector, it returns None without
 * panicking. You would use this method if accessing an element beyond the range of the vector may
 * happen occasionally under normal circumstances. Your code will then have logic to handle having
 * either Some(&element) or None. For example, the index could be coming from a person entering a
 * number. If they accidentally enter a number that's too large and the program gets a None value,
 * you could tell the user how many items are in the current vector and give them another chance to
 * enter a valid value. That would be more user-friendly than crashing the program due to a typo!
 *
 * When the program has a valid reference, the borrow checker enforces the ownership and borrowing
 * rules to ensure this reference and any other references to the contents of the vector remain
 * valid. Recall the rule that states you can't have mutable and immutable references in the same
 * scope. That rules applies to the code below, where we hold an immutable reference to the first
 * element in a vector and try to add an element to the end. This program won't work if we also try
 * to refer to that element later in the function.
 *
 *  let mut v = vec![1, 2, 3, 4, 5];
 *
 *  let first = &v[0];
 *
 *  v.push(6);
 *
 *  println!("The first element is: {first}");
 *
 * Compiling this code will result in this error:
 *
 *  cannot borrow `v` as mutable because it is also borrowed as immutable
 * The code above might look like it should work: why should a reference to the first element care
 * about changes at the end of the vector? This error is due to the way vectors work: because
 * vectors put the values next to each other in memory, adding a new element onto the end of the
 * vector might require allocating new memory and copying the old elements to the new space, if
 * there isn't enough room to put all the elements next to each other where the vector is currently
 * stored. In that case, the reference to the first element would be pointing to deallocated
 * memory. The borrowing rules prevents programs from ending up in that situation.
 *
 * Iterating over the Values in a Vector
 *
 * To access each element in a vector in turn, we would iterate through all of the elements rather
 * than use indices to access one at a time. The code below shows how to use for loop to get
 * immutable references to each element in a vector of i32 values and print them.
 *
 *  let v = vec![100, 32, 57];
 *  for i in &v {
 *      println!("{i}");
 *  }
 * To read the number that n_ref refers to, we have to use the * dereference operator to get to the
 * value in n_ref before we can add 1 to it.
 *
 * We can also iterate over mutable references to each element in a mutable vector in order to make
 * changes to all the elements. The for loop below will add 50 to each element.
 *
 *  let mut v = vec![100, 32, 57];
 *  for i in &mut v {
 *      *i += 50;
 *  }
 * To change the value that the mutable reference refers to, we again use the * dereference
 * operator to get to the value in n_ref before we can use the += operator.
 *
 * Safely Using Iterators
 *
 * Iterators contain a pointer to data within the vector. We can see how iterators work by
 * desugaring a for-loop into the corresponding method calls of Vec::iter and Iterator::next:
 *
 *  let mut v: Vec<i32>             = vec![1, 2];
 *  let mut iter: Iter<'_, i32>     = v.iter();             -> L1
 *  let n1: &i32                    = iter.next().unwrap(); -> L2
 *  let n2: &i32                    = iter.next().unwrap(); -> L3
 *  let end: Option<&i32>           = iter.next();          -> L4
 * Observe that the iterator iter is a pointer that moves through each element of the vector. The
 * next method advances the iterator and returns an optional reference to the previous element,
 * either Some (which we unwrap) or None at the end of the vector.
 *
 * This detail is relevant to safely using vectors. For example, say we wanted to duplicate a
 * vector in place, such as [1, 2] becoming [1, 2, 1, 2]. A naive implementation might look like
 * this, annotated with the permissions inferred by the compiler:
 *
 *  fn dup_in_place(v: &mut Vec<i32>) {
 *      for n_ref in v.iter() {
 *          -> *v       | R (loses W due to mutable reference)
 *          -> v        | R (loses O after v.iter())
 *          -> n_ref    | RO
 *          -> *n_ref   | R
 *
 *          v.push(*n_ref);     -> push() requires RW (v doesn't have W)
 *      }
 *  }
 * Notice that v.iter() removes the W permission from *v. Therefore the v.push(..) operation is
 * missing the expected W permission. The Rust compiler will reject this program with a
 * corresponding error message:
 *
 *  cannot borrow `*v` as mutable because it is also borrowed as immutable
 * The safety issue beneath this error is reading deallocated memory. As soon as v.push(1) happens,
 * the vector will reallocate its contents and invalidate the iterator's pointer. So to use
 * iterators safely, Rust does not allow you to add or remove elements from the vector during iteration.
 *
 * One way to iterate over a vector without using a pointer is with a range, like we used for
 * string slices. For example, the range 0..v.len() is an iterator over all indices of a vector v,
 * as shown below:
 *
 *  let mut v: Vec<i32>         = vec![1, 2];
 *  let mut iter: Range<usize>  = 0..v.len();
 *  let i1: usize               = iter.next().unwrap();
 *  let n1: &i32                =&v[i1];
 *
 * Using an Enum to Store Multiple Types
 *
 * Vectors can only store values that are of the same type. This can be inconvenient; there are
 * definitely use cases for needing to store a list of items of different type. Fortunately, the
 * variants of an enum are defined under the same enum type, so when we need one type to represent
 * elements of different types, we can define and use an enum!
 *
 * For example, say we want to get values from a row in a spreadsheet in which some of the columns
 * in the row contain integers, some floating-point numbers, and some strings. We can define an
 * enum whose variants will hold the different value types, and all the enum variants will be
 * considered the same type: that of the enum. Then we can create a vector to hold that enum and
 * so, ultimately, hold different types.
 *
 *  enum SpreadsheetCell {
 *      Int(i32),
 *      Float(f64),
 *      Text(String),
 *  }
 *
 *  let row = vec![
 *      SpreadsheetCell::Int(3),
 *      SpreadsheetCell::Text(String::from("blue")),
 *      SpreadsheetCell::Float(10,12),
 *  ];
 * Rust needs to know what types will be in the vector at the compile time so it knows exactly how
 * much memory on the heap will be needed to store each element. We must also be explicit about
 * what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be
 * a chance that one or more of the types would cause errors with the operations performed on the
 * elements of the vector. Using an enum plus a match expression means that Rust will ensure at
 * compile time that every possible case is handled.
 *
 * If you don't know the exhaustive set of types a program will get at runtime to store in a
 * vector, the enum technique won't work. Instead, you can use a trait object.
 *
 * Dropping a Vector Drops Its Elements
 *
 * Like any other struct, a vector is freed when it goes out of scope.
 *
 *  {
 *      let v = vec![1, 2, 3, 4];
 *
 *      --do stuff with v--
 *  } // <- v goes out of scope and is freed here
 * When the vector gets dropped, all of its contents are also dropped, meaning the integers it
 * holds will be cleaned up. The borrow checker ensures that any references to contents of a vector
 * are only used while the vector itself is valid.
 */
