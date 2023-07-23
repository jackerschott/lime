[![Rust-Build-Test](https://github.com/jackerschott/lime/actions/workflows/rust-build.yml/badge.svg)](https://github.com/jackerschott/lime/actions/workflows/rust-build.yml)
[![rust-clippy analyze](https://github.com/jackerschott/lime/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/jackerschott/lime/actions/workflows/rust-clippy.yml)

# lime -- Leightweight Image Editor
Non-destructive minimalistic image editor.

Edit images by simply writing a script, similar to LaTeX, but still using
interactive parts where it is necessary.
This will make it
- non-destructive, one can always change every script line
- efficient, one does not have to click through five menus just to
    apply a Gaussian blur as hundreth of times before
- minimalistic, one does not have to pay the performance costs of a heavy UI
- visual, one can recompile continously while editing the script to the result
- interactive, when using a brush or cutting out a path the process will still
    be performed interactively, while the result is cached

## Scripting
An example script would look like
```
base = create_layer()
base = apply_brush(base, 'darkred', 0.5)
base = gaussian_blur(base, 0.5, 0.5)

render(base)
```

## Features
Some basic features to implement are
- an image viewer with some interactive editing capabilities
- support for multiple layers
- support for masking
