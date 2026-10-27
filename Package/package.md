### Packages
it consists of crates.

### Crates
there are two type of crates: lib crate and binary crate.

1. Binary Crate: it is rust file, which can be executable.
    Ex: main.rs 
2. Library Crate: it consists of methods, classes and more, it doesn't contain a main function

in this example, i got an example package

```
  package
      |
      +-> src
           |
           +--> main.rs
           |
           +--> math.rs
           |
           +--> util
                  |
                  +--> mod.rs
                  |
                  +--> math.rs
```

in these example,
main.rs is binary crate, both math.rs are library crate.

we can access modules directly by writing

```rs
  mod file_name;
```

1. mod is like namespace, and has extra features.
2. though you cannot put folder name in mod, unless you got mod.rs, to select the file path for compiler, i actually idk how it works.

in util/mod.rs
```rs
  pub mod math;
```

now you can use util as module too

in main.rs
```rs
  mod util;

  use util::math; // it works
```