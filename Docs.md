# Documentation
simple_tui a simple framework for building very simple and basic tui's. Which is very lightweight and fast ⚡⚡
## Table of Content
- [Installation](#install)
  - [Required crates](#crates)
- [Components](#comp)
  - [Userinput](#Userinput)
  - [Itemlist](#Itemlist)

## Installation  <a name="install"></a>
Only get the simple_tui.rs file and import your liked function. Easy and lightweight! ⚡
```rust 
mod simpletui;
use simpletui::the_function_you_want_to_import;
```
### Required crates <a name="crates"></a>
```toml
[dependencies]
colored = "2.1.0"
crossterm = "0.27.0"
```
## Components <a name="comp"></a>
```rust 
mod simpletui;
```
### Userinput <a name="Userinput"></a>
![grafik](https://github.com/OctoDino/simple_tui/assets/58665605/e1fad078-aca2-42d5-aa2d-65e9bbe75cad)

Creates a userinput, with a python like syntax. Returns the inputted value as `String`
#### Import
```rust
mod simpletui;
use simpletui::user_input;
```
#### Example
```rust
let input: String = user_input("How was your day?");
```
### Itemlist <a name="Itemlist"></a>
![grafik](https://github.com/OctoDino/simple_tui/assets/58665605/f159664e-5dc7-4f9e-b0eb-a8e32a0d2d65)

Creates from a given array a interactable list. Navigation with the arrow keys. Returns the selected array element as an `i32` type
#### Import
```rust
mod simpletui;
use simpletui::list_selection;
```

#### Example
```rust
//Creating a array
let list: Box<[&str]> = Box::from(["Option 1", "Option 2", "Option 3"]);

//Rendering the list in the console and printing the selected array element
//(e.g. returns 2 when "Option 3" is selected)
let selected_item = list_selection(list);
println!("{}", selected_item.to_string())
```
