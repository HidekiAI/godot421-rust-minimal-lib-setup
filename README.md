# godot421-rust-minimal-lib-setup

Sample project setup for Linux+Windows on Godot 4.2.1 with godot-rust with 2 libs (lib1 and lib2) deriving Node2D and NodeSprite2D, each calling '\_init()' and '\_ready()'

Original intentions were first to get at least 2 rust libraries to be interacting with Godot; mainly because I could not stand this dynamic variants of GDScript when I was implementing an 2D Array[Array[]] of homogeneous dimensions, and due to unstrict types, I'd sometimes place a row of (the array is row-ordered)ref instead of actual array that was constructed to be added because append() (or was it insert()) can append a new row-set as ref when all other rows are actualy Array[Object], and during runtime, it'll just (implicitly?) dynamic cast that ref of Array[Object] as Array[Object] in which, because it failed dynamic casting, returns null... good grief! Perhaps back when I was on C++/C# days, it was perfectly fine to think like that...

Simpler explanation of the above, if the array tells me that it's length()/size() is N, and I iterate though it N times, I EXPECT ALL N-elements in the array to be non-NULL/nullptr instances (nor should I have to dynamically cast each elements)!  But due to the flexibility of the language that allows variant types to be part of an array, I have to waste my time dynamically casting it and then testing whether it is null or not.

From a Rust (or even F#) programmer's point of view, that makes us cringe!  In any case, these days, I prefer compilers (more specifically, static-analyser in realtime as I type, such as rust-analyzer and Iodine (F#)) to catch the bugs for me during my "thinking" while implementing (coding), rather than after completion of implemenatations, guessing what typo I've made wrong during debugging/running/testing...

P/S: Obviously, on a memory-tight machines (I'd imagine people who chooses Godot really like small memory footprints), these memory-hogging static-analysers (i.e. Ionide on console vim eats up sometimes 1 or 2GB of my RAM!), it's is not too favored (though previously, I've felt that c-tags were actually slowing me down, but I don't use them anymore, so I don't know how much of an impact they actually have on my workflow).

Few (opinionated) notes:

- If you are attempting to implement 'lib.rs' to just do very simple things, don't bother, and use '.gd' GDScript instead. It's just a waste of time compiling and debugging/runtime-testing/verifying

## Technical Notes

- Platform: This boilerplate was only tested on Windows 11 due to my Debian desktop went south...
- Version: Godot v4.2.1 (though on .gdextension, you MUST set it to "4.2" (without the ".1" patch version) or else it'll complain that it cannot find your extension - bug on upstream?)
- Cargo: the github "master" version (see Cargo.toml) of [godot-rust (extension)][1]
- Each crates can only define ONE [entry-point][2] (i.e. in 'lib.rs'); but as long as multiple modules ('mod') you create are private to Godot, you can have as many 'mod' as you'd want
  - In C/C++ speaks, think of it as when you'd create a shared object (.so and/or .dll), you'd normally only have one single 'main()' as an entry point (well, you don't really need to define main() in shared object, but you get the point, just need to declare entry point).
- Crates which you've written that are shared between multiple libraries (i.e. lib1 and lib2) does not have to necessarily have GDExtension; Think of these to be just private library not exposed/shared to Godot
- The 'lib1' and 'lib2' are just examples of 2 libs that are interacting with Godot, and each has its own 'lib.rs' in which it's sole purpose is to expose entry-points for each crates, but will try to clearly write the examples in a way which each libs will have multiple modules that are not exposed to Godot, as well as a shared crate which does not have an entry-point exposed to Godot

Directory structures are:

```bash
    + lib1/
      + src/
        + lib.rs
        + my_module.rs
    + lib2/
      + src/
        + lib.rs
        + my_module.rs
    + shared_internal_lib/
      + src/
        + lib.rs
        + my_module.rs
    + build.sh
```

Note that 'build.sh' is a BASH script, so if trying to run it in Windows, make sure to open up MinGW BASH terminal (i.e. windows-git).

- [1]: https://github.com/godot-rust/gdext
- [2]: https://godot-rust.github.io/docs/gdext/master/godot/prelude/trait.ExtensionLibrary.html

## Goals

- clear examples of 2 libs in which each libs (crates) will exponse one-and-only-one GDExtension entry-point
- clear examples of 1 (private) library which 2 libs will call/utilize, in which it is private and no entry-point is exposed for GDExtension to reference
- bare minimum code and interfaces but as much commenting as possible

## Post-mortem/what I've learned so far

### It's a script, not an application

Do NOT forget that each rust source code, or at least, the main 'lib.rs' file in which the entry points are exposed, are SCRIPT (GDScript) pattern.  I have this hard-to-let-go habbit of implementing RAII-pattern in which I usually do:

```rust
impl MyStruct {
  pub fn new(...) -> Self{
    ...
  }
  ...
}
```

 But the issue with this, is that your script has its own Init() and Ready() functions, and if your script "HasA" MyStruct, only place you should be calling MyStruct::new() is in Init().  But then, if MyStruct "HasA" Godot INode derived object (i.e. MyNodeX:Node2D), you then get into this bind of wondering when to call Node2D::Init() and how...  In a nutshell, just remind yourself that in GDScript, you never (or rarely?) had to call it's children's Init() (i.e. TileMap has TileSet as it's internal children in which by the time you access the TileMap GDScript, the TileSets are all instantiated/concrete).  Overall, just keep constantly reminding yourself that "I'm writing a script".

### Minimal version support

If you are trying to implement a GDExtension, you MUST set the "version" field in the "*.gdextension" file(s) to "4.2" (without the ".1" patch version) or else it'll complain that it cannot find your extension - bug on upstream?

### Write kernels

Unless you're writing the shared library logic (i.e. singleton pattern with Autoload, etc), constantly remind self that you're writing a "script" for each callback actions and signals.  Which means you MUST (or WANT to) keep the logic as simple and quick as possible; i.e. Imagine 1000 enemies visible on the viewport, in which they call your Process() function that takes 1mSec each (per enemy).  That means 1000 mSec (1 second) per frame is spent causing almost impossible to play due to heinous framerate!  Simply put, these Scripts are similar to "kernel" (as in, GPGPU coding) logics, which you'd want to make sure to have it process as quickly as possible and bail/opt-out early.

### Public and Private

Any modules declared in 'mod.rs' should be private from Godot visibility, and only thing that should be exposed/public/visible to Godot is what you've declared as entry-points in 'lib.rs'.

### No need to add that kitchen sink

If you are trying to implement 'lib.rs' to just do very simple things, don't bother, and use '.gd' GDScript instead. It's just a waste of time compiling and debugging/runtime-testing/verifying.
