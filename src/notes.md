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
-   Modify Field - Stages, each stage adds new obstacles (Mutated Grounds)
-   Add hazards to the stage (Torture Chamber)
-   Add a thingy to your car (Pimp My Ride)
-   Add speed-boosts that temporary boost movement (Gotta go fast)
-   Movement forward is always pressed (Charlie's Wildcard)

-   Give the other player inverted controls (Mind Fuck) (KILLED FOR NOW MIGHT MOVE TO NEGATIVE EFFECTS)

Sepration of Modifications (draft)

-   Modifications that affect gameplay in a neutral / positive way
    should perhaps be seperated away from negative modifications

-   Maybe negative modifications (e.g. mind fuck) could appear as random injections into the game
    splash-effect

-   For this we'd need more negative effects, right now only mind fuck could work
    Other ideas:
-   Switch goals (goals switch place but players stay on the same side)
-   Crazy hazards (Hazard objects move around in random fashion)
-   Shuffle controls (shuffle the 5 keys, move / turret)

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

[x] MOD: ModifyField
[x] MOD: Turret
[x] Knockback
[x] Projectiles can kill other players
[x] Shots can be fired
[x] JUICE: countdown on round start + on goal
[x] MOD: Inverted
[x] BUG: ball initial velocity weird
[x] MOD: AddHazard (graphics?)
[x] JUICE: camrea shake on turret shot
[x] Turret: Make projectiles destroy on wall hit
[x] MOD: Speed-boosters
[x] MDD: Charlie's wildcard

[] GAME: On death countdown before can go again
[] TURRET: Only 3 shots per round

[ish] MOD: DecreaseDegrade (needs some more polish)
[] JUICE: boost effect
[] JUICE: camera shake on ball hit

[] Turret: Both players should get it
[] Projectile: Projectile on projectile should not be a thing

[] COLLIDER: Create segmented collider for modified player

[] MOD: ModifyCar (segmented collider & new sprite)

[] PLAYER: Movement 2 hard
[] PLAYER: Turn rate 2 fast

[] GRAPHICS: Hazard
[] GRAPHICS: Projectile
[] GRAPHICS: Level
[] GRAPHICS: player (a random animal)

[] GAME: Pause functionality

[] JUICE: blinking props on goal
[] TWEAK: sensor colliders for out-of-bounds instead of manual checking

[x] GAME: Intro cutscene thingy

On average from 2010 over 10 thousand people die every year from drunk driving accidents.

The worst part is that the victims also include people minding their own business on the street, that is objectively bad.

You crazy fucks are having beers and such then you put yourself in control of a metallic elephant with turbo engines. Jesus christ.

For the love of god play this game instead to simulate the adrenaline flow of driving a blazer.

I don't really mind saving lives you know, fucking hell it might even be my destiny, to save lives.

Anyway here's the game...
