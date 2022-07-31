from copy import deepcopy
from player import Player

"""
This player search minimax heuristically
"""
class HeuristicMiniMaxPlayer (Player) :

  def __init__ (self, dimension, depth, heuristic) :
    self.dimension = dimension
    self.searchDepth = depth
    # on heuristic,  win, lose is +-1, draw is 0
    self.heuristic = heuristic

  def bestMove (self, gameState, depth) :

    if gameState.isDrawed :
      return 0, None
    if gameState.isWon :
      return gameState.playerPerspective, None
    if depth == 0:
      return self.heuristic(gameState), None

    value = float('-inf')
    bestmove = None

    for move in gameState.allMoves:
      [row, col, size] = move
      new_state = deepcopy(gameState)
      new_state.move(row, col, size)
      newValue, _ = self.bestMove(new_state, depth-1)

      if newValue > value :
        bestmove = move
        value = newValue

    return value, bestmove


  def nextAction (self, gameState) :
    value, move = self.bestMove(gameState, self.searchDepth)
    [row, col, size] = move
    return row, col, size