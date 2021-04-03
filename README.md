# RL-GUI (Final Year Computer Science Project)

A GUI written in Rust for my Digital Systems Project module at university. It uses an Elm-style MVVM architecture and
has support for some extremely basic widgets currently. It is intended to have an embedded scripting language such as Lua
for easy editing but that is currently under investigation.

![two buttons, two text areas, a text input and a text area copying the current value of the text input](./images/current_functionality.png)

## Current State

- Basic buttons implemented
- Basic text input implemented
- A html-like DSL implemented using proc-macro as an alternative to a full scripting language API in case that doesn't pan out
- Styling controls are inconsistent and very incomplete currently
- Flex layout in Columns and Rows has support only for top to bottom and left to right, but does allow for multiple
widgets to be displayed together.
- TextInput widget support needs to be correctly added to the DSL parser
- Keyboard events need to be given the status of control keys
- TextInput needs to respond to some common control key uses - jumping between words with ctrl left/right for example
  
## TODO
- Investigate more deeply into how a scripting language would be embedded into a GUI toolkit
- Consider API changes / a second API for scripting language implementation

## Embedded Scripting Spitballing
I'm unsure how embedded a scripting interface would work with the current MVVM interface. These are a few (completely 
un-researched) ideas on how it could work:
- The view method could fetch a Lua state that would be dynamically converted into RL-GUI Element types?
- - How would message handling work in this world? The messages are currently defined as a user defined Enum, but these
    would (probably?) not be directly available to pass between Lua and Rust. This may entail writing a second API.
- Create a second API for Lua use, which would have the state held in Lua? This seems like it is approaching a retained
  style API - if the entire GUI state is held in Lua and then processed into the backend, it would not be much extra work
  to have it manage when draw calls are issued.
- Would this be like a set of Lua functions that construct the GUI entirely in the Lua script? I'm finding it hard to find
  good examples of GUIs with Lua bindings