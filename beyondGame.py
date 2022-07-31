from competition import Competition
from strategies.randomPlayer import RandomPlayer
from strategies.P2DrawPlayer import P2NeverLosePlayer
from strategies.fastMinimax import FastMiniMaxPlayer

gameDimension = (3, 3, 5, 1) # 3x3 with 5 sizes 1 copy each size

random = RandomPlayer(gameDimension)
p2 = P2NeverLosePlayer(gameDimension)
compet = Competition([random, p2], gameDimension)
compet.run(500)
# expected sth like [0, 125, 375]
minimax = FastMiniMaxPlayer(gameDimension, 'beyondFastMnmx.ch', False)
