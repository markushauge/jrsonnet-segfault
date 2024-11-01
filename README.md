```
jrsonnet-segfault on  main [?] is 📦 v0.1.0 via 🦀 v1.80.0 on ☁️  (eu-west-1) took 11s
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/jrsonnet-segfault`
zsh: segmentation fault  cargo run
```

### LLDB

```
jrsonnet-segfault on  main [?] is 📦 v0.1.0 via 🦀 v1.80.0 on ☁️  (eu-west-1)
❯ lldb target/debug/jrsonnet-segfault
(lldb) target create "target/debug/jrsonnet-segfault"
Current executable set to '/Users/markus/Development/GitHub/markushauge/jrsonnet-segfault/target/debug/jrsonnet-segfault' (arm64).
(lldb) run
Process 55731 launched: '/Users/markus/Development/GitHub/markushauge/jrsonnet-segfault/target/debug/jrsonnet-segfault' (arm64)
Process 55731 stopped
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=1, address=0x8)
    frame #0: 0x0000000100193090 jrsonnet-segfault`_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop::h6a8898af8da3eef4 at mod.rs:394:14
Target 0: (jrsonnet-segfault) stopped.
(lldb) frame variable
(*const dyn jrsonnet_evaluator::obj::ObjectLike) val = {
  pointer = NULL
  vtable = 0x0000000000000000
}
```

### Miri

```
jrsonnet-segfault on  main is 📦 v0.1.0 via 🦀 v1.84.0-nightly on ☁️  (eu-west-1)
❯ MIRIFLAGS="-Zmiri-disable-stacked-borrows -Zmiri-disable-isolation" cargo miri run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/bin/cargo-miri runner target/miri/aarch64-apple-darwin/debug/jrsonnet-segfault`
error: Undefined Behavior: constructing invalid value: encountered a dangling reference (use-after-free)
   --> /Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:521:1
    |
521 | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (use-after-free)
    |
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: BACKTRACE:
    = note: inside `std::ptr::drop_in_place::<std::boxed::Box<dyn jrsonnet_evaluator::ObjectLike>> - shim(Some(std::boxed::Box<dyn jrsonnet_evaluator::ObjectLike>))` at /Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:521:1: 521:56
    = note: inside `std::ptr::drop_in_place::<jrsonnet_evaluator::gc::TraceBox<dyn jrsonnet_evaluator::ObjectLike>> - shim(Some(jrsonnet_evaluator::gc::TraceBox<dyn jrsonnet_evaluator::ObjectLike>))` at /Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:521:1: 521:56
    = note: inside `std::mem::ManuallyDrop::<jrsonnet_evaluator::gc::TraceBox<dyn jrsonnet_evaluator::ObjectLike>>::drop` at /Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:256:18: 256:53
    = note: inside `jrsonnet_gcmodule::cc::RawCcBox::<jrsonnet_evaluator::gc::TraceBox<dyn jrsonnet_evaluator::ObjectLike>, jrsonnet_gcmodule::collect::ObjectSpace>::drop_t` at /Users/markus/.cargo/registry/src/index.crates.io-6f17d22bba15001f/jrsonnet-gcmodule-0.3.7/src/cc.rs:337:22: 337:66
    = note: inside `<jrsonnet_gcmodule::cc::RawCc<jrsonnet_evaluator::gc::TraceBox<dyn jrsonnet_evaluator::ObjectLike>, jrsonnet_gcmodule::collect::ObjectSpace> as std::ops::Drop>::drop` at /Users/markus/.cargo/registry/src/index.crates.io-6f17d22bba15001f/jrsonnet-gcmodule-0.3.7/src/cc.rs:613:17: 613:31
    = note: inside `std::ptr::drop_in_place::<jrsonnet_gcmodule::cc::RawCc<jrsonnet_evaluator::gc::TraceBox<dyn jrsonnet_evaluator::ObjectLike>, jrsonnet_gcmodule::collect::ObjectSpace>> - shim(Some(jrsonnet_gcmodule::cc::RawCc<jrsonnet_evaluator::gc::TraceBox<dyn jrsonnet_evaluator::ObjectLike>, jrsonnet_gcmodule::collect::ObjectSpace>))` at /Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:521:1: 521:56
    = note: inside `std::ptr::drop_in_place::<jrsonnet_evaluator::ObjValue> - shim(Some(jrsonnet_evaluator::ObjValue))` at /Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:521:1: 521:56
    = note: inside `std::ptr::drop_in_place::<jrsonnet_evaluator::Val> - shim(Some(jrsonnet_evaluator::Val))` at /Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:521:1: 521:56
note: inside closure
   --> src/main.rs:14:51
    |
14  |         v.unwrap().manifest(&yaml_format).unwrap();
    |                                                   ^
    = note: inside closure at /Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:797:29: 797:36
    = note: inside closure at /Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:88:21: 88:35
    = note: inside `<std::ops::Range<usize> as std::iter::Iterator>::fold::<(), {closure@std::iter::adapters::map::map_fold<usize, std::result::Result<jrsonnet_evaluator::Val, jrsonnet_evaluator::Error>, (), {closure@jrsonnet_evaluator::val::ArrValue::iter::{closure#0}}, {closure@std::iter::Iterator::for_each::call<std::result::Result<jrsonnet_evaluator::Val, jrsonnet_evaluator::Error>, {closure@src/main.rs:13:43: 13:46}>::{closure#0}}>::{closure#0}}>` at /Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2546:21: 2546:32
    = note: inside `<std::iter::Map<std::ops::Range<usize>, {closure@jrsonnet_evaluator::val::ArrValue::iter::{closure#0}}> as std::iter::Iterator>::fold::<(), {closure@std::iter::Iterator::for_each::call<std::result::Result<jrsonnet_evaluator::Val, jrsonnet_evaluator::Error>, {closure@src/main.rs:13:43: 13:46}>::{closure#0}}>` at /Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:128:9: 128:50
    = note: inside `<std::iter::Map<std::ops::Range<usize>, {closure@jrsonnet_evaluator::val::ArrValue::iter::{closure#0}}> as std::iter::Iterator>::for_each::<{closure@src/main.rs:13:43: 13:46}>` at /Users/markus/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:800:9: 800:31
note: inside `main`
   --> src/main.rs:13:5
    |
13  | /     val.as_arr().unwrap().iter().for_each(|v| {
14  | |         v.unwrap().manifest(&yaml_format).unwrap();
15  | |     });
    | |______^

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error
```
