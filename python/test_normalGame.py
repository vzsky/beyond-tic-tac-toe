from python.competition import Competition
from strategies.heuristics import getBasicHeuristic
from strategies.randomPlayer import RandomPlayer
from strategies.minimax import MiniMaxPlayer
from strategies.fastMinimax import FastMiniMaxPlayer
from strategies.heuristicMinimax import HeuristicMiniMaxPlayer

normalTicTacToeGameDim = (3, 3, 1, 9)

heuristic = getBasicHeuristic(normalTicTacToeGameDim, 2, 1)

random = RandomPlayer(normalTicTacToeGameDim)
minimax1 = MiniMaxPlayer(normalTicTacToeGameDim, 'normalMiniMax.ch')
minimax2 = FastMiniMaxPlayer(normalTicTacToeGameDim, 'normalBetterMnmx.ch')

hmnmx = HeuristicMiniMaxPlayer(normalTicTacToeGameDim, 2, heuristic)

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