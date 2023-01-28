# TODOs

> List of WIP items in markdown, cause I don't want to use github issues at this point

## WIP

- [ ] Advanced perft
  - [ ] Perft statistics

- [ ] Find way to store statistics modules
  - [ ] God-statistics module
    - Pass it everythere (perft, tt get / set)
    - Fall-back to 0-size and no-op for every operation

- [ ] Fix in-code TODOs
  - [ ] Zorbist refactoring
  - [ ] Statistics refactoring
  - [ ] Parallel perft refactoring

## Big parts

- [ ] Search
  - [ ] Randomization
  - [ ] Insufficient material

- [ ] UCI

- [ ] Bitboard move generator

- [ ] Evaluation
  - [ ] Lazy evaluation

- [ ] Lichess bot

- [ ] Configurable statistics module for different parts of engine (features)

- [ ] Performance
  - [ ] Less bloat
    - [ ] Disable std
    - [ ] Disable C lib (use syscalls)
  - [ ] Release score
    - [ ] Benchmarks
  - [ ] Likely/unlikely
  - [ ] https://nnethercote.github.io/perf-book/build-configuration.html