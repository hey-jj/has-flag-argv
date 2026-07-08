# Changelog

## [0.1.1] - 2026-07-07

### Performance

- Explicitly prefixed flags can use less CPU time across long argument lists. (#15)
- Long flag names can begin the argument scan without walking the full flag name. (#16)
