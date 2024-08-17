# TODO

**Main design goal**: make something that will entertain people
I'll drop this here and sort it later:  https://app.diagrams.net/#HRbatistab%2Fdoppelkopf-rs%2Fmain%2Fdocs%2FDoppelkopfDiagrams.drawio#%7B%22pageId%22%3A%22oIAwmTRgkffAONk_1_cy%22%7D

## High level stuff

- [ ] Determine who the user it and what it wants (ex. to be entertained)
- [ ] Define the presentation of the game
- [ ] Determine how to track the state of the game in terms of the actions and the score
- [ ] Define how to store the game

## Planning and design

- [ ] Model the game structures
  - [ ] Whiteboard the game, laying out the whole structure 
  - [ ] Find all the nouns
  - [ ] Determine what each noun can do and wether it's valid or not
- [ ] Deterine a game structure that keeps track of the game and show the score (remember it doesn't take actions)
- [ ] Create a diagram in draw.io (?)

# BACKLOG

- [x] Set up a CI/CD structure on github with clippy as a mandatory step
- [x] Split into a `Cards` crate library, agnostic to the game and a `doppelkopf-game` crate to handle the game (consider a rule engine crate)
- [ ] Explain the game on `README.md` (IN PROGRESS)
  - [ ] [Adding color seems cool!](https://clemensjarnach.github.io/02-articles/2023-05-02-article.html)
- [ ] Listen to Zelda's final summary of the game on the recording
- [ ] Define a design and architecture
- [ ] Code the modules from the nouns
- [ ] Make the game work
  - [ ] Code the game
  - [ ] Check that the rules apply to the game (validate rules)
  - [ ] Create AI players
  - [ ] Simulate the game with mocked player actions
- [ ] Make the game playable

