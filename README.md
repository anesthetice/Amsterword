## Amsterword - a simple yet fun and suspensful multiplayer word game

### installation

* download and install the rust compiler (https://www.rust-lang.org/learn/get-started)
* clone this repository using git or simply download it as a zip
* open a terminal inside the project and enter 'cargo run'
* alternatively you could instead enter 'cargo build --release' and then run the executable found in ./target/release

### usage

* you can interact with the game's settings by modifying 'config.json'
* number_of_players: self-explanatory
* hint_system: the hint system works by potentially showing the unique word in red instead of the usual green, but there is also a chance that it will show a generic word as red
* setting hint_system to false instead displays all word in cyan
* hint_probability_anomaly: the probability [0.0, 1.0] that the unique word will be shown in red instead of green
* hint_probability_generic: the probability [0.0, 1.0] that a generic word will be shown in red instead of green
* lexicon: the words that can appear (at least 2 needed)

Lastly I highly recommend you add your own words to the lexicon