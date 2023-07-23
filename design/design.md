# Design
This document is not meant to be complete for now.

## Usage Example
```
model = io.open_layer('photos_raw/model_on_greenscreen.jpg')

// applying automatic color correction by specifiying points ('color anchors')
// with known color
anchors = patterns.color_anchors(model, [#!black, #808080, #!white])
model = correct_color_by_anchors(model, anchors)

// remove greenscreen and replace with transparent pixels
// use 'feather' edges with gradually decreasing opaqueness for 13.2 pixels
screen_ref_points = patterns.keying_reference_points(model)
is_model = trafos.key_out(model, screen_ref_points, feather_edge_size=13.2)

model = layer_combis.apply_mask(model, is_model)

bg = io.open_layer('photos_raw/background.jpg')
```

## Lime Modules
The first thing lime does is parsing the entire input script by the Parser
module.
Since parsing is cheap and we might need access to subsequent instructions when
executing instructions, which is expensive, this is a good way to do it.
The resulting instructions are then executed one-by-one with limes main loop.

### Script Parser
The script parser translates a plain text file into a datastructure.
This datastructure contains instructions that can be directly used by Lime.
These instructions include all standard programming instructions that are
helpful for us.
In particular, assignments, routine calls, control flow (if, loops), etc.
Furthermore there are the following kinds of built in routine calls:
- I/O, eg. reading an image, writing a rendered image
- transformations, eg. applying a filter, resizing the image,
    applying brush strokes, cutting along a path
- creating a transformation pattern, eg. drawing brush strokes, tracing a
    path for a cut, drawing repair tool strokes
- layer combinations, eg. superposing a semi-transparent image onto an opaque
    background, applying a mask

Transformation patterns are supposed to be created interactively with a live
preview.
Writing a rendered image is supposed to be used multiple times, for example to
see intermediate rendering results.

For scripting there exist the following built-in types:
- bool, integer, float, because duh
- layer (image or mask)
- strings, needed for eg. filenames
Typing is inferred during parsing.
There is no way to create new types (we are not writing a programming language).
All in all the idea is to only add features if we have to or if there is a major
convience reason for it, otherwise we keep the language as simple as possible.

### Interpreter
This one contains routines to interpret instructions given to lime and
essentially glues everything together.
Naturally this will make heavy use of the image and io modules.

### IO
Abstracts all io operations that can be performed with lime instructions.

## Editor Modules


## Shared Modules

### Image
The image module abstracts all operations that can be performed on any single or
multiple layers (images or masks).
This module does only this and does not know about scripting, i/o or the parsed
datastructure.
