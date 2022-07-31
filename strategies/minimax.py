from copy import deepcopy
from player import Player
from gameState import GameState
import pickle

"""
This player search minimax and only play optimally
However, This player sucks and search overlapped board all the time. 
This minimax search 740k nodes while is actually enough to do under 20k for normal ttt
"""
class MiniMaxPlayer (Player) :

  class miniMaxNode :
    def __init__ (self, state):
      self.state = state
      self.children = {}
      self.value = None

    def bestMove (self) :
      for move in self.children :
        if self.children[move].value == self.value:
          return move
 
  def makeMiniMaxTree (self, node) :
    self.nodeCounter += 1
    if self.nodeCounter % 10000 == 0 :
      print(self.nodeCounter, "nodes")
      
    if (node.state.isWon) :
      node.value = node.state.playerPerspective
      return
    if (node.state.isDrawed) :
      node.value = 0
      return

    node.value = -10 if node.state.playing == 0 else 10

    for move in node.state.allMoves:
      [row, col, size] = move
      next_state = deepcopy(node.state)
      next_state.move(row, col, size)
      child = self.miniMaxNode(next_state)
      self.makeMiniMaxTree(child)
      node.children[(row, col, size)] = child

      if node.state.playing == 0 :
        node.value = max(node.value, child.value)
      else :
        node.value = min(node.value, child.value)

  def saveTree (self, name):
    with open('cache/'+name, 'wb') as file:
      pickle.dump(self.root, file)

  def loadTree (self, name):
    with open('cache/'+name, 'rb') as file:
      self.root = pickle.load(file)

  def reset (self) :
    self.now = self.root

  def __init__ (self, dimension, name, cache=True) :
    self.nodeCounter = 0
    self.dimension = dimension
    startState = GameState(self.dimension)

    if (cache) : 
      self.loadTree(name)
    else : 
      self.root = self.miniMaxNode(startState)
      self.makeMiniMaxTree(self.root)
      self.saveTree(name)

    self.now = self.root

  def nextAction (self, gameState) :
    lastMove = gameState.lastMove
    if lastMove != None:
      self.now = self.now.children[lastMove]
    best = self.now.bestMove()
    self.now = self.now.children[best]
    return best