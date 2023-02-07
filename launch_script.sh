#!/bin/sh
running="true"
while [ "$running" = "true" ]
do
    if cargo run --release 
    then
        running="false"
    fi
    cargo run --release login-notify "Forth interpreter crashed, restarting."
done
