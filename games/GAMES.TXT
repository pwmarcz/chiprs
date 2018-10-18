                            ****************
                            * Chip-8 games *
                            ****************
Keys are:
1 2 3 C
4 5 6 D
7 8 9 E
A 0 B F

15PUZZLE: You have to move all the items and put them in increasing order,
          starting at the upper-left corner. Move the item you want by
          pressing his associated key (ex: to move item 3, press 3).
          WARNING: uses the original CHIP8 keyboard, so you may have several
          confusions.

AIRPLANE: You must drop packets from your airplane (key 8) and make sure
          they hit the ground without colliding with other planes.

BLINKY  : This game is a PACMAN clone. Your goal is to eat all the balls in
          the maze. There are some enemies, so be careful. Use 2 4 6 and 8
          to move.
          *NOTE: really use 3/6 to go up/down, 7/8 to go left/right, and
	  F to restart the game. HAS TO BE CHANGED.


BLITZ   : This game is a BOMBER clone. You are in a plane, and you must
          destroy the towers of a town. Your plane is flying left to right,
          and goes down. Use 5 to drop a bomb. The game ends when you crash
          yourself on a tower...

BREAKOUT: Same than BRIX, but has graphics looking like the game on the
          Atari 2600 console.

BRIX    : This game is an "arkanoid" precursor. You have 5 lives, and your
          goal is the destruction of all the brixs. Use 4 and 6 to move
          your paddle. The game ends when all the brixs are destroyed.

CAVE    : Type F to start, then use 2 4 6 8 to move through the CAVE without
          hitting the walls.

CONNECT4: This game is for two players. The goal is to align 4 coins in the
          game area. Each player's coins are colored. When you drop a coin,
          it is paced on the latest dropped coin in the same column, or at
          the bottom if the column is empty. Once the column is full, you
          cannot place any more coins in it. To select a column, use 4 and 6.
          To drop a coin, use 5. There is no winner detection yet. This will
          be soon avalaible (Hey! I don't spend my life on CHIP8 !).

FIGURES : Kind of Tetris with numbers. Use 4 and 6 to move, and 2 do do
          something I didn't get.

FILTER  : Catch everything that falls. Use 4 and 6 to move.

GUESS   : Think to a number between 1 and 63. CHIP8 shows you several boards
          and you have to tell if you see your number in them. Press 5 if so,
          or another key if not. CHIP8 gives you the number...

HIDDEN  : See HIDDEN.txt (use 2 4 6 8 to move, 5 to pick)

INVADERS: The well known game. Destroy the invaders with your ship. Shoot
          with 5, move with 4 and 6. Press 5 to begin a game.

LANDING : Try to flatten the field for landing (?). Key 9 drops a bomb.

KALEID  : A little program (not a game) to make funny graphics. Move around
          the screen with 2 4 6 8. To finish and make CHIP8 repeat your
          moves, press 0.

MAZE    : This little program draws random mazes.

MERLIN  : This is the SIMON game. The goal is to remember in which order the
          squares are lighted. The game begins by lighting 4 random squares,
          and then asks you to light the squares in the correct order.
          You win a level when you give the exact order, and each increasing
          level shows a additionnal square. The game ends when you light an
          incorrect square. Keys are 4 and 5 for the two upper squares, then
          7 and 8 for the two other ones.
	  *NOTE: 5->2 8->6 7->8

MISSILE : You must shoot the 8 targets on the screen using key 8. Your
	  shooter moves a little bit faster each time you shoot. You
	  have 12 missiles to shoot all the targets, and you win 5
	  points per target shot.

PADDLES : F=1P game, E=2Pgame 7/9=P1 left/right, 4/6=P2 left/right.

PONG(1P): 1-player pong, play against the computer. 1/4=up/down.

PONG (2): Here is the well known pong game. Two versions are available.
          The only difference is that PONG2 is mostly like the original
          game. Player 1 uses 1 and 4, player 2 uses C and D.

PUZZLE  : Same than PUZZLE2. Wait for randomization... Instead of moving the
          item by pressing his associated key, move it UP DOWN LEFT RIGHT
          with respectively 2 8 4 6. Up and Down are inverted as the game
          uses the original CHIP8 keyboard.
	  *NOTE: must exchange up and down keys.

ROCKET  : Follow the tunnel. B=start, 4/6=left/right.
	  *NOTE: B->5

SOCCER  : Pong-like. 1/4=P1 up/down, C/D=P2 up/down.

SPACEF  : Fly through the screen 3 times to win the level.
	  F=start, E=start level, 1/4=up/down
	  *NOTE: F->5 E->6
	  
SQUASH  : Exactely same than WALL, except that you have 5 balls, which are
          not to be lost...
	  *NOTE: 1->2 4->6

SYZYGY  : This game is a SNAFU, or TRON variant. You are a snake, and you
          are very hungry. So, you eat the "0" which appears on the screen.
          Each time you eat, your size and score increase.
          You can choose to play with or without a border by pressing
          F or E before playing. Use 7 8 3 6 to move respectively LEFT RIGHT
          UP DOWN. When finished, press B to see the score.
	  *NOTE: change E/F->2/8 3/6->2/8 7/8->4/6 B->5

TANK    : You are in a tank which has 25 bombs. Your goal is to hit 25 times
          a mobile target. The game ends when all your bombs are shot.
          If your tank hits the target, you lose 5 bombs. Use 2 4 6 and 8 to
          move. This game uses the original CHIP8 keyboard, so directions 2
          and 8 are swapped.
	  *NOTE: exchange 2/8

TETRIS  : Guess what this game is... I'm sure you don't need the rules. If
          you do, please ask your friends. Use 4 to rotate, 5 and 6 to move,
          1 to drop a piece.
	  *NOTE: 4->5 5->4 1->8

TICTAC  : A TIC-TAC-TOE game. Play with [1] to [9] keys. Each key corresponds
          to a square in the grid. The game never ends, so at any time, the
          winner is the one who has the best score.

TRON    : B=with borders. F=without borders. 0=start. Player 1: 1/4 3/C.
	  Player 2: 7/A 9/E.

UFO     : A precursor of INVADERS. You have 15 missiles to shoot on the two
          invaders. The big one moves on the left and gives you 5 points.
          The small one moves on the right at variant speeds. You can shoot
          them in three directions: left, up and right. Use 4 to shoot on the
          left, 5 to shoot up, 6 to shoot on the right. The game ends after
          having shot the 15 missiles.

VBRIX   : Like BRIX, but the brix are put vertically, and the pad also moves
          vertically. Start by pressing 7, and move using 4 and 1.
	  *NOTE: 7->5 1->2 4->6

VERS    : A TRON clone. Keys:
                                   Left Pl.   Right Pl.
                          UP          7          C
                          DOWN        A          D
                          LEFT        1          B
                          RIGHT       2          F

WALL    : One of these PONG variations. Move using 1 and 4. As said in the
          early seventies: "AVOID MISSING BALL FOR HIGH SCORE" !!!
	  *NOTE: 1->2 4->6

WIPEOFF : Another BRIX variant, but quite hard to play. Move with 4 and 6.
          Your score is shown when you lose all your lives.


** CHIP-8 test programs:

C8PIC   : Display the word CHIP8.

IBM     : Display an IBM logo.

ROCKET2 : Small animation. F=start.
	  *NOTE: F->5

TAPEWORM: Make a very long snake. F=start, 2 4 6 8=turn.
	  *NOTE: F->5

TIMEBOMB: Chronometer. 2/8: Set time. 5: Start.

X-MIRROR: Draws symmetric patterns using 2 4 6 8.

