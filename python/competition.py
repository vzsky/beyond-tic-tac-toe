from game import Game

class Competition:
  def onWinReport (self, winner):
    self.stats[winner] += 1

  def onDrawReport (self):
    self.stats[2] += 1

  def __init__ (self, players, gameDimension):
    self.stats = [0, 0, 0] # P1, P2, Draw
    self.players = players
    self.game = Game(self.players, gameDimension, self.onWinReport, self.onDrawReport)

  def run (self, loop):
    for _ in range(loop):
      self.game.play()
      self.game.reset()
    print(self.stats)