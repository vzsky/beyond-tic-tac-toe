from competition import Competition
from strategies.randomPlayer import RandomPlayer
from strategies.minimax import MiniMaxPlayer
from strategies.fastMinimax import FastMiniMaxPlayer
from strategies.heuristicMinimax import HeuristicMiniMaxPlayer

normalTicTacToeGameDim = (3, 3, 1, 9)

random = RandomPlayer(normalTicTacToeGameDim)
minimax1 = MiniMaxPlayer(normalTicTacToeGameDim, 'normalMiniMax.ch')
minimax2 = FastMiniMaxPlayer(normalTicTacToeGameDim, 'normalBetterMnmx.ch')

def hueristic (gameState) :
  boardValue = gameState.board.sum()
  stackValue = 0
  for i in range(gameState.gameDimension[2]):
    stackValue += i*gameState.stacks[0][i]
    stackValue -= i*gameState.stacks[1][i]
  return (2*boardValue + stackValue)/100

hmnmx = HeuristicMiniMaxPlayer(normalTicTacToeGameDim, 2, hueristic)

# compet = Competition([random, minimax1], normalTicTacToeGameDim)
# compet.run(500)
# # expected sth like [0, 425, 75]

# compet = Competition([minimax1, random], normalTicTacToeGameDim)
# compet.run(500)
# # expected sth like [480, 0, 20]

# compet = Competition([minimax1, minimax2], normalTicTacToeGameDim)
# compet.run(500)
# # expected [0, 0, 500]

# compet = Competition([random, random], normalTicTacToeGameDim)
# compet.run(500)
# # expect sth like [156, 165, 179]

# compet = Competition([hmnmx, random], normalTicTacToeGameDim)
# compet.run(500)
# # expect sth like [429, 49, 22]
# # therefore this heuristic works well first player

# compet = Competition([random, hmnmx], normalTicTacToeGameDim)
# compet.run(500)
# # expect sth like [200, 138, 162]