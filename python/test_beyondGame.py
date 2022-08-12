from competition import Competition
from strategies.heuristics import getBasicHeuristic
from strategies.randomPlayer import RandomPlayer
from strategies.P2DrawPlayer import P2NeverLosePlayer
from strategies.heuristicMinimax import HeuristicMiniMaxPlayer

gameDimension = (3, 3, 5, 1) # 3x3 with 5 sizes 1 copy each size

heuristic = getBasicHeuristic(gameDimension, 2, 1)

random = RandomPlayer(gameDimension)
p2 = P2NeverLosePlayer(gameDimension)
hmnmx = HeuristicMiniMaxPlayer(gameDimension, 2, heuristic)

# compet = Competition([random, random], gameDimension)
# compet.run(500)
# # expected sth like [185, 83, 232]

# compet = Competition([random, p2], gameDimension)
# compet.run(500)
# # expected sth like [0, 125, 375]

# compet = Competition([hmnmx, p2], gameDimension)
# compet.run(10)
# # expected [0, 0, 10] deterministic

compet = Competition([hmnmx, random], gameDimension)
compet.run(500)
# expected sth like [392, 15, 93]

compet = Competition([random, hmnmx], gameDimension)
compet.run(500)
# expected sth like [244, 94, 162]
# a bad second player