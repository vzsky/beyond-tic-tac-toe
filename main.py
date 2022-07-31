from torch import normal
from competition import Competition
from strategies.randomPlayer import RandomPlayer
from strategies.P2DrawPlayer import P2NeverLosePlayer
from strategies.minimax import MiniMaxPlayer
from strategies.betterMinimax import BetterMiniMaxPlayer

gameDimension = (3, 3, 5, 1) # 3x3 with 5 sizes 1 copy each size

# random = RandomPlayer(gameDimension)
# p2 = P2NeverLosePlayer(gameDimension)
# compet = Competition([random, p2], gameDimension)
# compet.run(500)
# # expected sth like [0, 125, 375]

normalTicTacToeGameDim = (3, 3, 1, 9)

# random = RandomPlayer(normalTicTacToeGameDim)
# minimax1 = MiniMaxPlayer(normalTicTacToeGameDim, 'normalMiniMax.ch')
# minimax2 = MiniMaxPlayer(normalTicTacToeGameDim, 'normalMiniMax.ch')
# compet = Competition([random, minimax1], normalTicTacToeGameDim)
# compet.run(500)
# # expected sth like [0, 425, 75]
# compet = Competition([minimax1, random], normalTicTacToeGameDim)
# compet.run(500)
# # expected sth like [480, 0, 20]
# compet = Competition([minimax1, minimax2], normalTicTacToeGameDim)
# compet.run(500)
# # expected [0, 0, 500]

random = RandomPlayer(normalTicTacToeGameDim)
)
compet = Competition([random, minimax], normalTicTacToeGameDim)
compet.run(500)