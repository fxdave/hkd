# Hot key daemon (WIP)

This is a simple daemon written in **Rust**.
Configurable via editing the `main.rs`, so it allows you more than a simple markdown file would, and it is fast.
At the same time, you get a good IDE support.

However, it is a bit slower to configure the hotkeys this way, so I suggest you to try `sxhkd` first if you don't have any special needs like stateful hotkeys, or somehow that wouldn't be fast enough.

I put every goodies into the `tools` module that will come handy for desktops.

# Checklist

[x] adding user defined state
[x] better way to define sequences
[ ] make possible to grab pointer
[ ] some language bridge? - Maybe I could allow shell languages, with some own sytanx for key bindings that will be parsed before the execution.