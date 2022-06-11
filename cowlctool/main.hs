import System.Environment ( getArgs )

main :: IO ()
main = do
    args <- getArgs
    contents <- readFile "build"
    print contents
    print args