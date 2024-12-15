compile_game:
	cd game && cargo build
	cp target/debug/libgame.dylib .

name:
	make compile_game
	install_name_tool -id '' libgame.dylib

inspect_game:
	nm libgame.dylib

run:
	make compile_game
	cargo run

