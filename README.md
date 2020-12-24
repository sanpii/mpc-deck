```
[Unit]
Description=mpc-deck

[Service]
ExecStart=/home/git/public_repositories/mpc-deck/current/target/release/oxfeed-api /dev/input/by-id/usb-Arduino_LLC_Arduino_Leonardo_HIDJB-event-if02
Restart=on-failure
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
```
