import numpy as np
from misc import isLegalMove, sign, sizeAvailable

class GameState :

  @property
  def id (self):
    stack0 = tuple(self.stacks[0])
    stack1 = tuple(self.stacks[1])
    return ((stack0, stack1), tuple(self.board.tobytes()), self.playing)

  def reset (self):
    self.stacks = [None, None]
    self.stacks[0] = [self.gameDimension[3]]*self.gameDimension[2]
    self.stacks[1] = [self.gameDimension[3]]*self.gameDimension[2]
    self.board = np.zeros((self.gameDimension[0], self.gameDimension[1]))
    self.playing = 0
    self.lastMove = None

  def __init__ (self, gameDimension):
    self.gameDimension = gameDimension
    self.reset()

  @property
  def playerPerspective(self):
    return 1 if self.playing == 0 else -1

  @property
  def myBoard (self):
    return self.board * self.playerPerspective

  def isLegalMove (self, row, col, size):
    return isLegalMove(self.stacks[self.playing], self.myBoard, row, col, size)

  @property
  def myStack (self):
    return self.stacks[self.playing]

  @property
  def allMoves (self):
      moves = []
      for i in range(self.gameDimension[0]):
        for j in range(self.gameDimension[1]):
          for k in range(1, self.gameDimension[2]+1):
            if self.isLegalMove(i, j, k):
              moves.append((i, j, k))
      return moves

  @property
  def isDrawed (self):
    if len(self.allMoves) == 0:
      return True
    return False

  @property
  def isWon (self):
    player_perspective = self.playerPerspective
    # check horizontally
    for i in range(self.gameDimension[0]):
      status = True
      for j in range(self.gameDimension[1]):
        if sign(self.board[i][j]) != player_perspective:
          status = False
      if status == True : return True
      
    # check horizontally
    for j in range(self.gameDimension[1]):
      status = True
      for i in range(self.gameDimension[0]):
        if sign(self.board[i][j]) != player_perspective:
          status = False
      if status == True : return True
    
    # check diagonally 
    if self.gameDimension[0] != self.gameDimension[1] : return False
    status = True
    for i in range(self.gameDimension[0]) :
      if sign(self.board[i][i]) != player_perspective:
        status = False
    if status == True: return True

    status = True
    for i in range(self.gameDimension[0]) :
      if sign(self.board[i][self.gameDimension[0]-i-1]) != player_perspective:
        status = False
    if status == True: return True

  def move (self, row, col, size):
    if not self.isLegalMove (row, col, size) : return False
    self.lastMove = (row, col, size)
    self.board[row][col] = self.playerPerspective * size
    self.stacks[self.playing][size-1] -= 1
    self.playing = self.playing ^ 1
    return True

  def log_board (self):
    for i in range(self.gameDimension[0]):
      for j in range(self.gameDimension[1]):
        print(self.board[i][j], end=' ')
      print()
    print()

