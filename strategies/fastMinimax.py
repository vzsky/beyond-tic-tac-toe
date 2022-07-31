from copy import deepcopy
from player import Player
from gameState import GameState
import pickle

"""
This player search minimax and only play optimally
This search only 18k nodes for normal game
This player can store and load cache so it's not neccessary 
loading up minimax tree everytime
This player doesn't know symmetry so symmetric boards are treated differently 
This would need to search 2 billions nodes for a (3, 3, 5, 1)
and definitely more than 26 millions which took hours
This player cares about states, so don't compete with its own object
"""
class FastMiniMaxPlayer (Player) :

  stateMap = {}

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
    if self.nodeCounter % 50000 == 0 :
      print(self.nodeCounter, "nodes")
      
    if (node.state.id in self.stateMap):
      return self.stateMap[node.state.id][0]
    if (node.state.isWon) :
      node.value = node.state.playerPerspective
      self.stateMap[node.state.id] = [node]
      return node
    if (node.state.isDrawed) :
      node.value = 0
      self.stateMap[node.state.id] = [node]
      return node

    node.value = -10 if node.state.playing == 0 else 10

    for move in node.state.allMoves:
      [row, col, size] = move
      next_state = deepcopy(node.state)
      next_state.move(row, col, size)
      next_node = self.miniMaxNode(next_state)
      child = self.makeMiniMaxTree(next_node)
      node.children[(row, col, size)] = child

      if node.state.playing == 0 :
        node.value = max(node.value, child.value)
      else :
        node.value = min(node.value, child.value)
    
    self.stateMap[node.state.id] = [node]
    return node

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