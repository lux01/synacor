module Main where

import Data.Foldable (forM_)

-- | The different directions that one can move through the grid
data Direction
  = North
  | East
  | South
  | West
  deriving (Show, Enum, Eq)

-- | A step is made of two parts, to ensure that we always have an operator
-- | and a value per step.
data Step = Step
  { first :: Direction
  , second :: Direction
  } deriving (Show, Eq)

-- | A path through the grid is a sequence of steps. Note that the path should
-- | be read right to left.
type Path = [Step]

-- | A position in the grid.
type Posn = (Int, Int)

-- | An orb value.
type Orb = Int

-- | A state of the player: a position in the grid, and an orb value.
type State = (Posn, Orb)

-- | Moves the position based on a direction.
move :: Direction -> Posn -> Posn
move North (x, y) = (x, y + 1)
move East (x, y) = (x + 1, y)
move South (x, y) = (x, y - 1)
move West (x, y) = (x - 1, y)


-- These next two functions could be cleaned up into an array, but
-- an auxiliary type would be needed to make it type safe.

-- | Gets the operator on the floor at the given position.
getOperator :: Posn -> (Orb -> Orb -> Orb)
getOperator (1, 0) = (-)
getOperator (3, 0) = (*)
getOperator (0, 1) = (+)
getOperator (2, 1) = (-)
getOperator (1, 2) = (*)
getOperator (3, 2) = (*)
getOperator (0, 3) = (*)
getOperator (2, 3) = (-)
getOperator pos =
  error $ "Attempted to read operator at position " ++ (show pos)

-- | Gets the value on the floor at the given position.
getValue :: Posn -> Orb
getValue (0, 0) = 22
getValue (2, 0) = 9
getValue (1, 1) = 4
getValue (3, 1) = 18
getValue (0, 2) = 4
getValue (2, 2) = 11
getValue (1, 3) = 8
getValue (3, 3) = 1
getValue pos = error $ "Attempted to read value at position " ++ (show pos)

-- | Given a complete path, computes the final state.
computeState :: Path -> State
computeState = foldr computeState' ((0, 0), 22)
  where
    computeState' step (pos, orb) =
      let d1 = first step
          d2 = second step
          pos' = move d1 pos
          op = getOperator pos'
          pos'' = move d2 pos'
          val = getValue pos''
      in (pos'', op orb val)

-- | Checks whether the given position is valid. Note that this function
-- | will report that the initial position is invalid, this is because it is
-- | not valid to move back to the initial position during movement.
isPosnValid :: Posn -> Bool
isPosnValid (x, y) =
  x >= 0 && x < 4 && y >= 0 && y < 4 && not (x == 0 && y == 0)

-- | Checks whether the given state is valid, i.e. is within the grid and
-- | has an orb with positive mass.
isStateValid :: State -> Bool
isStateValid (pos, o) = o > 0 && isPosnValid pos

-- | Computes all possible evolutions of the given paths
-- | and verifies that the intermediary and final position are valid.
evolvePaths :: [(Path, Posn)] -> [(Path, Posn)]
evolvePaths paths =
  [ (step : steps, pos'')
  | d2 <- [North, East, South, West]
  , d1 <- [North, East, South, West]
  , (steps, pos) <- paths
  , let pos' = move d1 pos
  , isPosnValid pos'
  , let pos'' = move d2 pos'
  , isPosnValid pos''
  , let step = Step {first = d1, second = d2}
  ]

-- | Computes all paths of length 2n that have valid final positions.
pathsOfLength2 :: Int -> [(Path, Posn)]
pathsOfLength2 n = head $ drop n $ iterate evolvePaths [([], (0, 0))]

-- | Computes the states for a set of paths.
computeStates :: [(Path, Posn)] -> [(Path, State)]
computeStates = map (\(path, _) -> (path, computeState path))

-- | The end state
endState :: State
endState = ((3, 3), 3)

-- | Checks if the end state is in the given list
hasEndState :: [(Path, State)] -> Bool
hasEndState = any (\(_, state) -> state == endState)

-- | Gets the end state path from the list.
getEndState :: [(Path, State)] -> Path
getEndState = fst . head . filter (\(_, state) -> state == endState)

-- | Main wrapper
main :: IO ()
main = do
  main' 1

-- | Computes all possible paths of length 2n and checks if the end
-- | state is within them, ending if it is and printing the path
-- | in a human readable form to the console.
main' :: Int -> IO ()
main' n = do
  putStrLn $ "Trying length " ++ (show $ 2 * n) ++ " paths."
  let states = computeStates $ pathsOfLength2 n
  if hasEndState states
    then do
      putStrLn "Solution found! Path is: "
      let path = getEndState states
      forM_ (reverse path) $ \step -> do
        putStrLn $ "   " ++ (show $ first step)
        putStrLn $ "   " ++ (show $ second step)
    else main' (n + 1)
