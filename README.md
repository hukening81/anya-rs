# anya-rs

This is an experimental implementation of a chatbot that (hopefully) will support both NTQQ(NapCat) and Discord, as well as message forwarding between these two platforms and other functionalities.

## High Level Architecture
- Using Tokio to run websocket connection as tasks
- Using bevy ecs to make the whole system extendable
    - Using bevy's event module for event processing

 ```text
 du -h target/release/anya
 5.5M    target/release/anya
 ```