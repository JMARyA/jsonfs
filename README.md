# JsonFileS

This crate provides the ability to wrap structs in JSON Files which are automatically saved to disk when dropped and can be reloaded from disk later.

## Features
- Map Structs to JSON Files
- Transparently works like a smart pointer
- JSON Files don't need to have all struct fields when loaded (Changing structs is possible, missing fields use default values)
