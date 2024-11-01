```
jrsonnet-segfault on  main [?] is 📦 v0.1.0 via 🦀 v1.80.0 on ☁️  (eu-west-1) took 11s
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/jrsonnet-segfault`
zsh: segmentation fault  cargo run

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
