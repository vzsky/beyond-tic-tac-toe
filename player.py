from misc import isLegalMove

class Player :

  def reset (self):
    pass

  def __init__ (self, gameDimension):
    self.dimension = gameDimension

  def illegalMoveReport(self, row, col, size):
    raise Exception("ILLEGAL MOVE")

  def nextAction (self, gameState):
    pass