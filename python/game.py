from gameState import GameState

class Game :

  def reset (self) :
    self.players[0].reset()
    self.players[1].reset()
    self.gameState.reset()

  def __init__ (self, players, gameDimension, winCallback, drawCallback):
    self.gameDimension = gameDimension
    self.players = players
    self.winCallback = winCallback
    self.drawCallback = drawCallback
    self.gameState = GameState(gameDimension)
    self.gameState.reset()

  def play (self):
    while True:
      row, col, size = self.players[self.gameState.playing].nextAction(self.gameState)
    
      if not self.gameState.move(row, col, size) :
        self.players[self.gameState.playing].illegalMoveReport(row, col, size)

      if self.gameState.isWon :
        return self.winCallback(self.gameState.playing)
      if self.gameState.isDrawed :
        return self.drawCallback()
