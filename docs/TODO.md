# Todo

## Set up
- [x] Set up a CI/CD structure on github with clippy as a mandatory step

## Backend 

- [ ] [IN_PROGRESS] Make the game playable as a CLI application
  - [ ] Make it installable (`.install.sh` + instructions)
  - [x] Add clap to handle the CLI
    - [x] Set a debugging flag
  - [ ] [IN_PROGRESS] Make a `cheat-sheet` command
    - [x] Make a general sheat cheet to print
    - [x] Split by parts of the game (ex. trumps, game types, etc.)
    - [x] Make an enum for the options of cheat sheets, `CheatSheetOption`
    - [ ] Fill the strings with the proper cheat sheet
  - [ ] Make a `start-game` command
    - [x] Enable `start-game` command and add descriptions
    - [x] Allow play command to have `player-name`, `suit-type` and `pack-size` argument options
    - [x] Create a new game from CLI commands
    - [x] Create the logic to add mock players to the game
  - [ ] Make `join-game` command
    - [ ] Process `player_name` and `game_id` options to join the game
  - [ ] Define a game state structure
  - [ ] Make the logic to deal cards
- [ ] Model the [operations](https://github.com/Rbatistab/dopplekopf-cdk/blob/main/docs/ARCHITECTURE_AND_DESIGN.md?plain=1#L68-L73)
- [x] Split the operations into crates
- [ ] Set a UUID generator
- [x] Split into a `Cards` crate library, agnostic to the game and a `doppelkopf-game` crate to handle the game (consider a rule engine crate)
- [ ] Make the game work
  - [ ] Code the game
  - [ ] Check that the rules apply to the game (validate rules)
  - [ ] Create AI players
  - [ ] Simulate the game with mocked player actions
- [ ] Make the game playable

## Research

- [ ] 

## Reviews

- [ ] Review App's X component with X person
