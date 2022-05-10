# EECS 767 Project

Authors: Jace Kline, Ishrak Hayet, Manoj Thangavel

## Setup

1. Install Cargo, the Rust build system.
2. Navigate to [src/backend](src/backend/).
3. Run `cargo build` to build the binary and dependencies.
4. Run `cp target/debug/backend ../../fs_search` to copy the built executable to the root of this project directory under the name `fs_search`.
5. Navigate back to the root of the project directory where the executable is located.

## Running the Program

1. Run `./fs_search SCRAPE_ROOT INDEX_FILE` to run the program.

    * The `SCRAPE_ROOT` argument is the a path to the root directory that the program should scrape and index.
    * The `INDEX_FILE` argument is a path to a file location that the program should use to persist the index. When re-run, supply this argument again to load the previously stored index.
    * Example: `./fs_search ./test-documents ./index.bson`

2. In a web browser, navigate to [http://localhost:8000/ui/](http://localhost:8000/ui/) to view the UI page.

## References

* [Text documents source](http://textfiles.com/stories/)
