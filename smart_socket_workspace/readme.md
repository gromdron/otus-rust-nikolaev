# Smart Socket Basic implementation

Some basic functions:
- Connect to delayed smart socket
- Toggle power state
- Get report from smart socket

## Components
- STP - custom string transfer protocol library above TCP.
- Smart socket server - server application, that accepts incoming connections and provide command execution.
- Smart socket client - thin wrapper library around STP. Provides smart socket functions.
- Smart socket TUI - smart socket application with terminal interface. Used smart socket client library.


## Exmaple usage

1. Run smart socket server
2. Run smart socket tui