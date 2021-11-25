TARGET=armv7-unknown-linux-musleabihf
PI=pi@wordclock.local
cargo build --target=$TARGET && scp ../target/$TARGET/debug/led-time $PI: && ssh $PI ./led-time
