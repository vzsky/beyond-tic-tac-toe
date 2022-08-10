from player import Player
import random

"""This player just plays randomly"""
class RandomPlayer (Player) :

  def nextAction (self, gameState) :
    moves = gameState.allMoves
    [row, col, size] = random.choice(moves)
    return row, col, size