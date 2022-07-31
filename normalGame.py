from .. import competition
from strategies.randomPlayer import RandomPlayer
from strategies.minimax import MiniMaxPlayer
from strategies.fastMinimax import FastMiniMaxPlayer

normalTicTacToeGameDim = (3, 3, 1, 9)

random = RandomPlayer(normalTicTacToeGameDim)
minimax1 = MiniMaxPlayer(normalTicTacToeGameDim, 'normalMiniMax.ch')
minimax2 = FastMiniMaxPlayer(normalTicTacToeGameDim, 'normalBetterMnmx.ch')
compet = competition.Competition([random, minimax1], normalTicTacToeGameDim)
compet.run(500)
# expected sth like [0, 425, 75]
compet = competition.Competition([minimax1, random], normalTicTacToeGameDim)
compet.run(500)
# expected sth like [480, 0, 20]
compet = competition.Competition([minimax1, minimax2], normalTicTacToeGameDim)
compet.run(500)
# expected [0, 0, 500]