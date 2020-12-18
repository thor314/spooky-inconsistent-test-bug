What's going on here:

I have a pair of strructs that wrap NEAR `UnorderedMap`s.

I have a set of tests that I've written for these structs, and they fail
inconsistantly. See the tests for details.

Try: run `cargo test` 10 times. Observe that the `test_split_owners` and
`test_royalties` tests fail roughly 7 times out of 10. Sometimes they both pass.
If they consistantly both pass, uncomment the line underneath the comment
"SPOOKY"
