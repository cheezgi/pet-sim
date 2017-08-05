# pet-sim

A small CLI pet simulator in Rust and a continuation of the pet simulator that
was originally in Visual C++ and was converted to C++ and later Java.

`pet-sim` was a project that I started as a part of a programming camp that I
attended a year or so ago to learn C++. Everyone who attended the class had to
create a final project, wether it be a game or a utility.

Me being a human being unoriginal, I had overheard someone talking about their
gerbil simulator, and since the only cool thing I could do so far is download
stuff off the internet, I decided that that was a great idea. And here I am,
five years or so later, continuing this project.

## build

`pet-sim` has a few debugging features that are hidden behind a build flag.
In order to build with these features, run the following command:

```
cargo build --features debug
```

This enables several debugging commands. More information is in the help screen.

