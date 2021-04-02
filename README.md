# RL-GUI (Final Year Computer Science Project)

A GUI written in Rust for my Digital Systems Project module at university. It uses an Elm-style MVVM architecture and
has support for some extremely basic widgets currently. It is intended to have an embedded scripting language such as Lua
for easy editing but that is currently under investigation.

## Current State

- Basic buttons implemented
- Basic text input implemented
- A html-like DSL implemented using proc-macro as an alternative to a full scripting language API in case that doesn't pan out
- Styling controls are inconsistent and very incomplete currently
- Flex layout in Columns and Rows has support only for top to bottom and left to right, but does allow for multiple
widgets to be displayed together.
- TextInput widget support needs to be correctly added to the DSL parser
  
## TODO
- Investigate more deeply into how a scripting language would be embedded into a GUI toolkit
- Consider API changes / a second API for scripting language implementation