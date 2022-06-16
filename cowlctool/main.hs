import System.Environment ( getArgs )

-- ensure all syntax in the entirety of the file follows Cowl's specifications
deep_check :: [Char] -> Integer
deep_check input = do
    1

-- parse the areas within and outside of strings and container pairs (parentheses, curly, square)
shallow_parse :: [Char] -> [[Char]]
shallow_parse raw = sp raw

    -- string, collected parsed strings, buffer, all parsed strings
sp :: [Char] -> [[Char]] -> [Char] -> [[Char]]
sp [] coll buf = coll ++ [buf]
sp 





main :: IO ()
main = do
    args <- getArgs
    contents <- readFile "build"
    print contents
    print args