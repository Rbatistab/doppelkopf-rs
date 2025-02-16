# Todo

## Set up
- [x] Set up a CI/CD structure on github with clippy as a mandatory step

---

## Utils
- [ ] Create a data representation of the trumps
  - [ ] Represent normal game trumps
- [x] Split into a `Cards` crate library, agnostic to the game or it's logic
  - [ ] [IN_PROGRESS] Model the card-related types
    - [ ] [IN_PROGRESS] Model and document a `Cards` type
      - [x] Model and document Suits
        - [x]  Add `to_str` representation
      - [x] Model and document Ranks
        - [x]  Add `to_str` representation
      - [x] Create a `from` to create a new card
      - [x] Create a builder design pattern as `CardBuilder`
      - [ ] Add card color
      - [ ] Creat a `to_str` representation of a card (a pretty one)
    - [ ] [ON HOLD] Model and document a `Decks` type
      - [ ] Model and document a `SuitType` enum
      - [ ] Model and document a `PackSize` enum
      - [ ] Make the logic to deal cards
- [ ] Extract any util that is not necessary for the user from the utils mod
  - [ ] cli_commands should move to the cli_logic -> Move
  - [ ] cheat_sheet_strings should be known only to the cheat_sheet_logic -> Move
  - [ ] text_style can stay there to give the user some 
- [ ] Create a rule engine(?)

---
## Lib
- [ ] Model the lib operations types (these are the entry point to the logic from the clients)
  - [x] Model  and document a `CheatSheetInput` type (`cheat_sheet_model`)
  - [x] Model  and document a `CheatSheetOutput` type  (`cheat_sheet_model`)
  - [x] Model  and document a `NewGameArgsInput` type  (`new_game_model`)
  - [ ] Model  and document a `NewGameArgsOutput` type  (`new_game_model`)
  - [ ] Model  and document a `JoinGameArgsInput` type  (`join_game_model`)
  - [ ] Model  and document a `JoinGameArgsOutput` type  (`join_game_model`)
  - [x] Model  and document a `Player` type (`player`)
  - [x] Model  and document a `GameState` type (`player`)
  - [x] Create and document a `CheatSheetOption` enum (`cheat_sheet_model`)
- Add the core logic
  - [ ] [IN_PROGRESS] Add logic and document for `cheat_sheet_logic`
    - [x] Handle inputs and outputs with the proper model struct
    - [x] Make a general cheat sheet to print (overview)
    - [ ] Create constants for the cheat (text) to display as a cheat sheet
      - [x] Split by parts of the game (ex. trumps, game types, etc.)
      - [ ] Fill the strings with the proper cheat sheet
    - [x] Add tests and docs
  - [ ] Add logic and document for `new_game_logic`
    - [ ] (I'm a new game, what do I do?)
  - [ ] Add logic and document for `join_game_logic`
    - [ ] (I'm joining a new game, what do I do?)
  - [ ] Add logic and document for a `game_state_machine` to implement a state design pattern
- [ ] Set a state machine to handle the game state
- [ ] Set a UUID generator

---

## Web game (AWS)

***Remember the Web game is a client for the lib and here is the backend for it***

- [ ] Model the [lambda operations](https://github.com/Rbatistab/dopplekopf-cdk/blob/main/docs/ARCHITECTURE_AND_DESIGN.md?plain=1#L68-L73) on smithy
  - [ ] Set up a smithy model for the lambda contracts
  - [ ] Model  and document a `CheatSheetLambdaInput` type (`cheat_sheet_lambda_model`)
  - [ ] Model  and document a `CheatSheetLambdaOutput` type  (`cheat_sheet_lambda_model`)
  - [ ] Model  and document a `NewGameArgsLambdaInput` type  (`new_game_lambda_model`)
  - [ ] Model  and document a `NewGameArgsLambdaOutput` type  (`new_game_lambda_model`)
  - [ ] Model  and document a `JoinGameArgsLambdaInput` type  (`join_game_lambda_model`)
  - [ ] Model  and document a `JoinGameArgsLambdaOutput` type  (`join_game_lambda_model`)
  - [ ] ...
- [x] Split the operations into crates
- [ ] Should actually merge into a single operations crate(?)

---

## CLI game

***Remember the CLI game is a client for the lib***

- [ ] [IN_PROGRESS] Make the game playable as a CLI application
  - [x] Make it installable (`.install.sh` + instructions)
  - [x] Add clap to handle the CLI
    - [x] Set a debugging flag
  - [ ] Build cli commands to play the game
    - [x] Create command to get a new cheat sheet
      - [x] Update Clap commands for `cheat-sheet` option
      - [x] Make `cheat_sheet_cli` command that consumes `cheat_sheet_logic` from the lib
      - [ ] Update readme docs
    - [ ] Create command to start a new game
      - [x] Update Clap commands for `new-game` option
      - [ ] Make a `new_game_cli` command that consumes `new_game_logic` from the lib (***Ensure this command updates a game state but does NOT depend on game state machine (state machine agnostic)***)
        - [x] Enable `new_game_cli` command and add descriptions
        - [x] Allow play command to have `player-name`, `suit-type` and `pack-size` argument options
        - [x] Create the logic to add mock players to the game -> Somewhere else
        - [ ] ...
        - [ ] Update readme docs
    - [x] Create command to join a new game
      - [x] Update Clap commands for `join-game` option
      - [ ] Make a `join_game_cli` command that consumes `join_game_logic` from the lib
      - ...

---

## Research

- [ ] Dive deep on smithy and rust, [example](https://github.com/david-perez/smithy-rs-lambda-cdk/tree/master)

---

## Reviews

- [ ] Review App's X component with X person
