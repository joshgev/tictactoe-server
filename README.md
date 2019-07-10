# tictactoe-server
A server for tic tac toe.

## Usage

There are only two API calls.  The first, `POST /games` is used to create a game and the second, `POST /games/<game-id>`, is used 
to make a move.  

## Creating a game

To create a game, `POST` to `/games` a JSON object with the following structure:

```javascript
{
  "player": {
    "name": "Joshua",
    "piece": "X"
  }
}
```

Note that the `piece` parameter can be either "X" or "O".  Note further that "X" always makes the first move.

The response to this request is a monolithic Game Object that contains all relevant information about the game that has been created.  See below for details.


## Making a move

To make a move, `POST` to `/games/<game-id>` a JSON object with the following structure:

```javascript
{
  "row": 1,
  "column": 2
}
```

The game-id comes from the Game Object received when the game was created.

Both parameters are 0-indexed.  Note that if you try to put a piece in an invalid spot (either because it is out of bounds 
or because it already has a piece in it), you get the following response:

```javascript
{
  "message": "Not a valid spot"
}
```

Otherwise, you will get an updated Game Object.


## Game Objects

A Game Object looks like this:

```javascript
{
  "id": "b3c89720-e5f4-403e-b46e-4de7beabf04f",
  "human_player": {
    "name": "Josh",
    "piece": "O"
  },
  "computer_player": {
    "name": "Computer",
    "piece": "X"
  },
  "state": "Ongoing",
  "board": {
    "data": [
      [
        null,
        null,
        null
      ],
      [
        null,
        "X",
        null
      ],
      [
        null,
        null,
        null
      ]
    ]
  }
}
```

When the game is still ongoing, the `state` element will have value `Ongoing`.  When the game concludes in a draw, `state` 
will be `Draw`.  If the game is won by piece X, `state` will be

```javascript
{
  "Won": "X"
}
```

Obviously, if O is the winner, then we `state` becomes

```javascript
{
  "Won": "O"
}
```
