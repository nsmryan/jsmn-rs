# jmsn-rs
The jsmn-rs crate provide a rustic interface to an absolute gem of a C library called
jsmn. The jsmn library is a JSON parser which, in the words of its author, is
fast, portable, and simple. It does no memory allocation, and does not build of a tree
of nodes with pointers- instead it fills out an array of structures, and children
are indicated by indices into the array.


The thing I love most about this library is its simplicity- it contains exactly
2 enums, 2 structs, and 2 functions, with a total of 1 header file and 1 .c file.


The rustic wrapping includes using bindgen to generate raw bindings, available from
the raw module, and then wrapping those up in standard rust naming conventions.
This means using slices instead of pointers and lengths, and returning a Result
from jsmn\_parse instead of a number that can be an error code or count.


# Features
This library provides two features, "parent-links" and "strict" which can be set
when adding jsmn-rs as a dependancy in Cargo.toml. These add the pre-processor
directives JSMN\_PARENT\_LINKS and JSMN\_STRICT, respectively, when compiling jsmn and
generating the jsmn bindings.


I've used this library without the strict setting as a quick way to read configuration
files, and I've seen some benchmarks which show that the parent-links can increase
performance enormously.

# TODO
Ideally it would be tested on 64 and 32 bit machines, as I'm not sure
whether the use of "int" in the C source will cause problems here.
I tried to use isize and usize to make this okay, but for some casts it
might still be a problem.

There are also very few tests and very little example code on how to use the library.

