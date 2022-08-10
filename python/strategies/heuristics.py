def maxBoardValue (gameDimension) :
  stack = [list(range(1, gameDimension[2]+1))] * gameDimension[3]
  boardSize = gameDimension[1]*gameDimension[2]
  board = sorted([item for sublist in stack for item in sublist])[0:(boardSize)]
  return sum(board)

def maxStackValue (gameDimension) :
  stack = [list(range(1, gameDimension[2]+1))] * gameDimension[3]
  return sum([item for sublist in stack for item in sublist])

def getBasicHeuristic (dim, alpha, beta) :
  divider = (alpha*maxBoardValue(dim) + beta*maxStackValue(dim)) # this is a loose upper bound
  def heuristic (gameState) :
    boardValue = gameState.board.sum()
    stackValue = 0
    for i in range(gameState.gameDimension[2]):
      stackValue += i*gameState.stacks[0][i]
      stackValue -= i*gameState.stacks[1][i]
    return (alpha*boardValue + beta*stackValue)/divider
  return heuristic