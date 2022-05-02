Nice solution shared on leedcode. Working with linked list in rust is a bit of a pain because of the ownership system but I see that this solution uses tricks I learned from this [document](https://rust-unofficial.github.io/too-many-lists/second-option.html). The trick is to use the Option's type [take](https://doc.rust-lang.org/std/option/enum.Option.html#method.take) method.

edit1: Going to tackle this problem again or a similar one to get used to the 'Box' type in Rust.
