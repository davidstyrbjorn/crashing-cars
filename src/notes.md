States
MENU
GAME
MODIFICATION

MENU:

-   Simple menu, wait for player to start game

GAME:

-   Main loop, two players battle it out to win, timer is there to emphasize urgency
-   Runs a generic system waiting for the win resource to bet set to true

MODIFICATION:

-   Display current score between players
-   Scroll through selections of mods to the game
-   Mod changes how the round is won

Modifications:

-   Goal Keepers (Outsourcing)
-   Increase Speed (Furiously Fast)
-   Decrase Degrade Factor (Snappy Hamster)
-   Shootable Projectiles (Lock n' Load)
-   Give the other player inverted controls (Mind Fuck)
-   Modify Field - Stages, each stage adds new obstacles (Mutated Grounds)
-   Add hazards to the stage (Torture Chamber)
-   Add a thingy to your car (Pimp My Ride)

TODO:
[x] CODE: systems into seperate files
[x] GAME: ball with physics
[x] GAME: ball can score
[x] GAME: player needs more control
[x] CODE: score tracking
[x] GAME: concave colliders on the corners of the map
[x] GAME: tank controls for player? try it out see how it feels
[x] GAME: menu to game transition
[x] GAME: tweak ball rigidbody values
[x] GAME: transition to modification state
[x] CODE: modification representation
[x] GAME: transition to game after modification
[x] MODIFICATION: ui
MODIFICATIONS
[x] MOD: GoalKeeper
[x] MOD: IncreaseSpeed
[ish] MOD: DecreaseDegrade (needs some more polish)

[x] MOD: ModifyField
[] MOD: Turret
[] Knockback
[] Projectiles can kill other players
[x] Shots can be fired

[] MOD: Inverted
[] MOD: AddHazard
[] MOD: ModifyCar

[] BUG: ball initial velocity weird
[] TWEAK: sensor colliders for oob instead of manual checking
[] JUICE: camera shake on ball hit
