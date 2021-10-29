# Rust-integrity-playground
It's written in Rust!

I hope my Rust doesn't get rusty...

## TODO:
Figure out how to deal with PRs pushing too many docker containers.

* [x] Every PR created and every new commit pushed to an existing PR will produce a new Docker container. => many containers.
  * [x] If you push 10 commits to an existing PR only the top one will produce a Docker container, so that is not a concern.
* Should we remove old containers when the new one gets build for a PR?
  * Should we remove the container if a PR gets closed?
* Do we want to store them in the artifacts of a build? If so, how do we deal with PR deployments?
* How do we deal with older containers on `main`? Leave? Purge? ...
