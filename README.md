# Why?
I don't want to manually check if OptiFine has recently updated. So I created this over-engineered program, that even caches de old OptiFine version just to say to you if it has changed.

# Over-engineering
* Using [tokio](https://tokio.rs/) to do network and file system operations.
* Using [dirs](https://crates.io/crates/dirs) to write the caching files to the appropiate directory.
* Using [reqwest](https://crates.io/crates/reqwest) to fetch data.
* Using [select](https://crates.io/crates/select) to parse HTML.

I could have done this in a Python script with like 15 lines, but I want to save my precious 150ms runtime overhead and get a 3 minutes compile time overhead.
