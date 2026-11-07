#!/usr/bin/env bash
clear

./bootstrap/raptor -O3 --link examples/libs/libraylib.a --link -lX11 examples/snake-multiplayer/player1.rp & ./bootstrap/raptor -O3 --link examples/libs/libraylib.a --link -lX11 examples/snake-multiplayer/player2.rp

./build/player1 &
sleep 0.5
./build/player2
