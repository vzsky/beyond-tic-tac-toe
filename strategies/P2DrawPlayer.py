from misc import sizeAvailable, sign
from player import Player
import random

"""
This player will not lose when played second on (3, 3, 5, 1) 
and should be able to draw (3, 3, n, 1) for all n
This player can't play first.
This player plays (3, 3, 5, 1) and should be able to play (3, 3, n, m)
when n = 1, this player plays randomly
"""
class P2NeverLosePlayer (Player) :

  def nextAction (self, gameState) :

    for i in range(self.dimension[0]):
      for j in range(self.dimension[1]):
        if sign(gameState.myBoard[i][j]) == -1:
          opponent_size = abs(gameState.myBoard[i][j])
          for k in range(1, self.dimension[2]+1):
            if k > opponent_size and sizeAvailable(gameState.myStack, k):
              return i, j, k

    if not sizeAvailable(gameState.myStack, 1):
      raise Exception("We should have a small size marker!")
    moves = []
    for i in range(self.dimension[0]):
      for j in range(self.dimension[1]):
        if sign(gameState.myBoard[i][j]) == 0:
          moves.append((i, j, 1))
    
    [row, col, size] = random.choice(moves)
    return row, col, size
