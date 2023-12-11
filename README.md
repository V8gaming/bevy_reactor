# BEVY RBMK-1000

## About
This is a simulation of the RBMK-1000 nuclear reactor, which was used in the Chernobyl Nuclear Power Plant. The simulation is written in Rust using the Bevy game engine.

Information about the RBMK-1000 can be found from the report from the USSR STATE COMMITTEE ON THE UTILIZATION OF ATOMIC ENERGY, 25-29 August 1986 [here](https://inis.iaea.org/collection/NCLCollectionStore/_Public/18/001/18001971.pdf).

## Todo
### Reactor
#### Reactor Core
- [X] Add reactor core sprite grid
    - [X] Add reactor core to tab system
- [ ] Refactor reactor spawning

#### Reactor UI
- [X] Add reactor UI
    - [X] Add reactor UI to tab system
- [ ] Add functionality to reactor UI

### Tab System
- [X] Add tab system
- [X] Add tab moving
- [X] Add close button to tabs
    - [X] Add tab closing
- [X] Camera Texture Demo
- [X] Add tab Border
- [ ] Add tab creation 
- [ ] Tab snapping and not going off screen ( stop mouse from moving )
- [ ] Flex alignment for tabs
- [ ] Add tab scaling, hard because all objects have to be rerendered or image becomes distorted ( Settings to preserve aspect ratio, or enable scrolling of smaller than content?, Rect in Sprite Struct, require visibilty calculation)
    - [ ] Add tab maximizing ( if camera, set main, else scale tab to screen size)
- [ ] Add tab scrolling
- [ ] Change close button to box sprite with x in it

## Bugs
- [ ] Button bounds not aligned with sprite
- [ ] z offset when dropped not reset (create global z offset manager?, make it so grabbing something sets the z offset to the greatest z offset + 1)
- [ ] Grabbing more than one tab at a time causes issues, limit to one tab at a time

## License
This project is duel licensed under the MIT license and the Apache License (Version 2.0). See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for more details.

## References
Fira Code font: 
    Author: Nikita Prokopov
    License: SIL Open Font License 1.1
    Link: [Github](https://github.com/tonsky/FiraCode)

Close Icon: Close by Setyo Ari Wibowo from [Noun Project](https://thenounproject.com/browse/icons/term/close/) (CC BY 3.0)