# Todo

## Set up
- [x] Set up a CI/CD structure on github with clippy as a mandatory step

## Backend 

- [ ] [IN_PROGRESS] Make the game playable as a CLI application
  - [x] Make it installable (`.install.sh` + instructions)
  - [x] Add clap to handle the CLI
    - [x] Set a debugging flag
  - [ ] [IN_PROGRESS] Make a `cheat-sheet` command
    - [x] Make a general sheat cheet to print
    - [x] Split by parts of the game (ex. trumps, game types, etc.)
    - [x] Make an enum for the options of cheat sheets, `CheatSheetOption`
    - [ ] Fill the strings with the proper cheat sheet
    - [ ] Add tests to `cheat_sheet.rs`
    - [ ] Add docs
  - [ ] [IN_PROGRESS] Set a state machine to handle the game state
  - [ ] Make a `new-game` command
    - [x] Enable `new-game` command and add descriptions
    - [x] Allow play command to have `player-name`, `suit-type` and `pack-size` argument options
    - [x] Create a new game from CLI commands
    - [x] Create the logic to add mock players to the game
    - [ ] Ensure this command updates a game state but does NOT depend on game state machine (state machine agnostic)
    - [ ] Add tests to `new_game_cli.rs`
    - [ ] Add docs
  - [ ] Make `join-game` command
    - [ ] Process `player_name` and `game_id` options to join the game
    - [ ] Add tests to `join_game_cli.rs`
  - [ ] Define a game state structure
  - [ ] Make the logic to deal cards
- [ ] Model the lib operations types
  - [x] Model `new_game_model`
  - [ ] Model `join_game_model`
  - [x] Model `player`
  - [x] Model `game_state`
- [ ] Model the [lambda operations](https://github.com/Rbatistab/dopplekopf-cdk/blob/main/docs/ARCHITECTURE_AND_DESIGN.md?plain=1#L68-L73)
  - [ ] Model `new_game_lambda_model`
  - [ ] Model `join_game_lambda_model`
- [x] Split the operations into crates
- [ ] Set a UUID generator
- [x] Split into a `Cards` crate library, agnostic to the game and a `doppelkopf-game` crate to handle the game (consider a rule engine crate)
- [ ] Create cards models, types and documentation
  - [ ] Model and document Card
  - [ ] Model and document Deck
- [ ] Make the game work (after the game is coded)
  - [ ] Check that the rules apply to the game (validate rules)
  - [ ] Create AI players
  - [ ] Simulate the game with mocked player actions
- [ ] Make the game playable

## Research

- [ ] 

## Reviews

- [ ] Review App's X component with X person
