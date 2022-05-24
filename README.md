# debugging

1. Run cargo test --lib --all 
2. Copy the path of the generated binary from which the test is run. 
```
umbreon-mbp:rsforge jsteurer$ cargo test --lib --all 
   Compiling rsforge v0.1.0 (/Users/jsteurer/Code/rust_workbench/rsforge)
    Finished test [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests (target/debug/deps/rsforge-4846333a86fd638c) // @@@ <- test binary is here 
```
3. Execute `lldb -- ./target/debug/deps/rsforge-4846333a86fd638c graph::test::find_shortest_path`
4. Set a breakpint `breakpoint set --file graph.rs --line 554`
5. Run to the breakpoint `run`
6. Go to the next step with `step` or skip to the next function with `next`
7. View a variable with `print <name>`

TBD: how do I use this? https://github.com/rust-lang/rust/pull/72357