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
 */
