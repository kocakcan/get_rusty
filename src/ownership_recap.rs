/*
 * Ownership Recap
 *
 * Ownership versus Garbage Collection
 *
 * To put ownership into context, we should talk about garbage collection. Most programming
 * languages use a garbage collector to manager memory, such as in Python, JavaScript, Java, and
 * Go. A garbage collector works at runtime adjacent to a running program (a tracing collector, at
 * least). The collector scans through memory to find data that's no longer used -- that is, the
 * running program can no longer reach that data from a function-local variable. Then the collector
 * deallocates the unused memory for later use.
 *
 * The key benefit of a garbage collector is that it avoids undefined behaviour (such as using
 * freed memory), as can happen in C or C++. Garbage collection also avoids the need for a complex
 * type system to check for undefined behaviour, like in Rust. However, there are a few drawbacks
 * to garbage collection. One obvious drawback is performance, as garbage collection incurs either
 * frequent small overheads (for reference-counting, like in Python and Swift) or infrequent large
 * overheads (for tracing, like in all other GC'd languages).
 *
 * But another less obvious drawback is that garbage collection can be unpredictable. To illustrate
 * the point, say we are implementing a Document type that represents a mutable list of words.
 *
 * Consider two key questions about document.py:
 *
 * 1. When is the words array deallocated? This program has created three pointers to the same
 *    array. The variables words, d, and d2 all contain a pointer to the words array allocated on
 *    the heap. Therefore Python will only deallocate the words array when all three variables are
 *    out of scope. More generally, it's often difficult to predict where data will be
 *    garbage-collected just by reading the source code.
 * 2. What are the contents of the document d? Because d2 contains a pointer to the same words
 *    array as d, then d2.add_word("world") also mutates the document d. Therefore; In this
 *    example, the words in d are ["Hello", "world"]. This happens because d.get_words() returns a
 *    mutable reference to the words array in d. Pervasive, implicit mutable references can easily
 *    lead to unpredictable bugs when data structures can leak their internals. Here, it is
 *    probably not intended behaviour that a change to d2 can change d.
 *
 * This problem is not unique to Python -- you can encounter similar behaviour in C#, Java,
 * JavaScript, and so on. In fact, most programming languages actually have a concept of pointers.
 * It's just a question of how the language exposes pointers to the programmer. Garbage collection
 * makes it difficult to see which variable points to which data. For example, it wasn't obvious
 * that d.get_words() produced a pointer to data with d.
 *
 * By contrast, Rust's ownership model puts pointers front-and-center. We can see that by
 * translating the Document type into a Rust data structure.
 *
 * This Rust API differs from the Python API in a few key ways:
 *
 * - The function new_document consumes the ownership of the input vector words. That means the
 * Document owns the word vector. The word vector will be predictably deallocated when its owning
 * Document goes out of scope.
 * - The function add_word requires a mutable reference &mut Document to be able to mutate a
 * document. It also consumes ownership of the input word, meaning no one else can mutate the
 * individual words of the document.
 * - The function get_words returns an explicit immutable reference to string within the document.
 * The only way to create a new document from this word is to deep-copy its contents.
 *
 * Rust makes working with memory and pointers concepts explicit, which is fucking awesome. This
 * has the dual benefit of (1) improving runtime performance by avoiding garbage collection, and
 * (2) improving predictability by preventing accidental "leaks" of data.
 */
