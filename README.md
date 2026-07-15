The typestate pattern is an API design pattern that encodes information about an object's run-time state in its compile-time type.

This repo is the best typestate implementation I have assembled from the sources below, as boilerplate-free as I can make it.

According to cliffle.com:
> an API using the typestate pattern will have:
>
> 1. Operations on an object (such as methods or functions) that are only available when the object is in certain states,
> 2. A way of encoding these states at the type level, such that attempts to use the operations in the wrong state fail to compile,
> 3. State transition operations (methods or functions) that change the type-level state of objects in addition to, or instead of, changing run-time dynamic state, such that the operations in the previous state are no longer possible.

## Sources

- <https://cliffle.com/blog/rust-typestate/>
- <https://zerotomastery.io/blog/rust-typestate-patterns/>
