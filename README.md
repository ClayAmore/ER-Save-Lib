# Description
ER-Save-Lib is a library for reading and writing Elden Ring save files, compatible with PC and Playstation Save Wizard exported saves. This library is currently in alpha and is being developed alongside the new release of [ER-Save-Editor](https://github.com/ClayAmore/ER-Save-Editor).

# Usage
## SaveApi
### Example
```rust
use er_save_lib::Save;

fn main() {
   // PC
   let save_api = SaveApi::from_path("./test/ER0000.sl2").expect("Failed to read save file!");

match save_api {
   Ok(save_api) => {
      let character_index = 0;
      save_api.set_character_name(character_index, "New Name");
      save_api.write_to_path("new/path/file_name.sl2");
   },
   Err(err) => eprintln!("{err}"),
}


// Playstation
let save = SaveApi::from_path("./test/ps_save.txt").expect("Failed to read save file!");

match save_api {
   Ok(save_api) => {
      let character_name = save_api.character_name();
      println!("{}", character_name);
      let bytes = save.write_to_vec();
   },
      Err(err) => eprintln!("{err}"),
   }
}
```


## Save
### Example
```rust
use er_save_lib::Save;

fn main() {
   // PC
   let save = Save::from_path("./test/ER0000.sl2").expect("Failed to read save file!");

    match save {
        Ok(save) => {
          save.write_to_path("new/path/file_name.sl2");
        },
        Err(err) => eprintln!("{err}"),
    }


   // Playstation
   let save = Save::from_path("./test/ps_save.txt").expect("Failed to read save file!");

    match save {
        Ok(save) => {
          let bytes = save.write_to_vec();
        },
        Err(err) => eprintln!("{err}"),
    }
}
```

## Credits
<a href="https://github.com/vswarte/"><img src="https://github.com/user-attachments/assets/c79f4130-a990-4b50-8131-5fe938b7573f"/></a>
<a href="https://github.com/nordgaren/"><img src="https://github.com/ClayAmore/ER-Save-Editor/assets/131625063/710c9ee6-c3df-4665-be6b-d96bce1ebf46"/>
