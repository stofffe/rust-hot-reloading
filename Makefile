compile_game:
	cargo build
	cp target/debug/libhot_reload.dylib libgame.dylib

compile_game_mac:
	cargo build
	install_name_tool -id '' target/debug/libhot_reload.dylib
	mv target/debug/libhot_reload.dylib .

run:
	make compile_game_mac
	cargo run

