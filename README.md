# Terra and Caelus

## Design

Procedural puzzles around manipulating plant growth via powers that transform the land or air.

### Puzzle Goals

- Get a certain plant to grow in 1 or more locations
- Remove a certain plant (or all plants) from 1 or more locations
- Reach a location

### Mechanics

- Player is localized with a top-down view of the world
- Player movement is limited
  - mostly to cells containing plant growth
  - unlockable powers may include teleportation, travel through water/air, etc.
- Limited available actions in each puzzle
- Unlock actions by completing an associated sub-goal
  - "temples" contain gifts of the gods - additional powers
- Different plants spread differently, and affect the terrain differently

## TODO

- [x] Create Menu
- [x] Load Hand Written Levels
- [x] Create & Control Player
- [x] Get Goals Working
- [ ] Create Powers
  - [ ] Fertilize a tile
  - [ ] Plant a seed
  - [ ] Teleport Player
  - [ ] Drain a tile
  - [ ] Start a fire
  - [ ] Wind - blow seeds across a gap
- [ ] Adjust Simulation Rules for Clarity
  Ground Cover
  - [ ] Grass - spreads to neighbouring fertile tiles soil, but not to drained tiles or sand or through other plants
  - [ ] Creepying Thyme - spreads to neighbouring fertile tiles & sand, but not through other plants
  Flowers
  - [ ] Peony - only spreads through very fertile soil, doesn't drain
  - [ ] Hibiscus - only spreads through seeds, but enhances soil
  Weeds
  - [ ] Crabgrass - grows well in under-fertilized land, spread well, clears if too fertilized
  - [ ] Dandelion - spreads by wind, grows quickly, cleared if drained
- [ ] Generate Levels*

## Assets

- Bevy Engine
- Archeologicaps Font by Manfred Klein - <https://www.1001freefonts.com/archeologicaps.font>
