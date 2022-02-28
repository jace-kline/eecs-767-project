import System.Posix (FileStatus, getSymbolicLinkStatus, fileAccess, fileExist, isRegularFile, isDirectory)
import System.Directory (listDirectory, readable, getPermissions, getDirectoryContents)
import System.Directory.Internal.Prelude (getArgs)

data FileType = Reg | Dir | Skip deriving (Eq, Show)

main :: IO ()
main = do
    args <- getArgs
    case args of
        (path:xs) -> do
            exists <- fileExist path
            permissions <- getPermissions path
            files <- listDirectory path
            status <- getSymbolicLinkStatus path
            filetype <- getFileType path
            print exists
            print $ readable permissions
            print $ ft status
            print filetype
            print files
            recurses <- filesFromRoot path
            print recurses
            filesFromRoot "/home/jacekline/dev/eecs-767/eecs-767-project/scrape.cpp" >>= print
        [] -> error "Provide directory path as argument"


-- main :: IO ()
-- main = do
--     args <- getArgs
--     case args of
--         (path:xs) -> do
--             files <- filesFromRoot path
--             print files
--             -- mapM_ putStrLn files
--         [] -> error "Provide directory path as argument"


concatIOList :: [IO a] -> IO [a]
concatIOList iolist = case iolist of
    (x:xs) -> do
        x' <- x
        xs' <- concatIOList xs
        return $ x':xs'
    [] -> return []

filesFromRoot :: FilePath -> IO [FilePath]
filesFromRoot path = do
    exists <- fileExist path
    if not exists then return []
        else do
            permissions <- getPermissions path
            if not (readable permissions) then return []
            else do
                filetype <- getFileType path
                case filetype of
                    Skip -> return []
                    Reg -> return [path]
                    Dir -> do
                        files <- listDirectory path
                        recurses <- mapM filesFromRoot files
                        return $ concat recurses


getFileType :: FilePath -> IO FileType
getFileType path = getSymbolicLinkStatus path >>= \status -> return (ft status) 


ft :: FileStatus -> FileType
ft status | isRegularFile status = Reg
          | isDirectory status = Dir
          | otherwise = Skip

    
-- readablePath :: FilePath -> IO [FilePath]
-- readablePath path = fileAccess path True False False