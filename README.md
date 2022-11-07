Simple repro to show how returning a Result<String, Error> can cause extreme memory usage and delay with the Jetbrains Rust-plugin. 

Each test case generates a large string and returns it either directly, through a custom struct or wrapped as Ok().
If you run the program directly, it completes very quickly as expected, printing out the instants before and after returning.
Starting it in the debugger, it still runs fast if you only break and continue in main.

Enabling breakpoint as specified in get_string_as_result(), and then stepping or continuing, it will start consuming excessive memory and taking many seconds of wall-time.

Expected result would be as it is when using VS Code, that it simply returns the data immediately without increasing the memory usage.
