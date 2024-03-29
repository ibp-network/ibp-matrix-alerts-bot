# ibp-matrix-alerts-bot
Subscribe to Alerts published by ibp-monitor and delivery over matrix

## 🚧 Work In Progress

- [&check;] matrix authentication, load and process commands from public and private rooms
- [&check;] implement http server with shared state (cache and matrix)
- [&check;] load members from json config file
- [&check;] review matrix commands:
    - [&check;] !subscribe alerts MEMBER SEVERITY [MUTE_INTERVAL]
    - [&check;] !unsubscribe alerts MEMBER SEVERITY
    - [&check;] !maintenance MEMBER MODE
    - [&check;] !alerts
    - [&check;] !help
    - [ ] !stats alerts
    - [ ] !test alert
- [&check;] allow configuration of mute time interval
- [&check;] implement /alerts webhook
- [&check;] implement alert stats counters
- [&check;] define alert message template
- [&check;] protect endpoint with API-Key