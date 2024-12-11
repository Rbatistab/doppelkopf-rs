# Todo

## Set up
- [x] Set up a CI/CD structure on github with clippy as a mandatory step

## Backend 

- [ ] Model the [operations](https://github.com/Rbatistab/dopplekopf-cdk/blob/main/docs/ARCHITECTURE_AND_DESIGN.md?plain=1#L68-L73)
- [x] Split the operations into crates
- [ ] Turn project into a set of Lambdas
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
