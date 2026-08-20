clear

cargo run --bin raptor -- --compile -O3 --link examples/libs/libraylib.a --link -lm --link -lX11 examples/snake-multiplayer/player1.rp & cargo run --bin raptor -- --compile -O3 --link examples/libs/libraylib.a --link -lm --link -lX11 examples/snake-multiplayer/player2.rp

./build/player1 &
sleep 0.5
./build/player2
