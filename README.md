# cliutil
Cliutil is planned to be a collection of utilities for command line interfaces.

### Current State
'cliutil' is currently very alpha.  It is not recommended for use in production code.

#### Apis
- 'cliutil::constexpr' - Api for creating a compile-time static command line application.

#### Todos
- 'cliutil::constexpr'
    - implement help / version subcommands
    - implement proper error reporting
    - implement command, flag, and parameter name duplication checks

### Future Plans
#### Apis
- 'cliutil::runtime' - creates a cli app config at runtime. This will be useful for creating very dynamic cli apps.