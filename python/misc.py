def sign (number) :
  if (number == 0) : return 0
  return number/abs(number)

def sizeAvailable (stack, size):
  return stack[size-1] != 0

def isLegalMove (stack, board, row, col, size):
  if not sizeAvailable(stack, size):
    return False
  if sign(board[row][col]) == 1 :
    return False
  if size <= abs(board[row][col]) :
    return False
  return True