fn main() {
	println!("Hello from Cargo");
}

/*
Cargo is Rust’s build system and package manager.
1) Get Cargo Version: cargo --version
2) Creating a project with Cargo
	a) cargo new projectName(i.e. hello_cargo)
		a.1) To move to this project folder, use cd projectName
		a.2) You’ll see that Cargo has generated two files and one directory for us: a Cargo.toml file and a src directory with a main.rs file inside.

3) To Build(This command creates an executable file) use: cargo build
	a) it will produce an executable file at target/debug/projectName(hello_cargo)
	b) To run, type: ./target/debug/projectName(hello_cargo)

4)To Run with Cargo use: cargo run
	b) it will internall call "./target/debug/projectName(hello_cargo)"

5) cargo check: This command quickly checks your code to make sure it compiles but doesn’t produce an executable:
	a) It is much faster than "cargo build" because it skips the step of producing an executable. If you’re continually checking
	   your work while writing the code, using will speed up the process! As such, many Rustaceans run periodically as they write their program to make sure it compiles.
	   Then they run "cargo build" when they’re ready to use the executable

6) To release a build use: cargo build --release
	a) When your project is finally ready for release, you can use to compile it with optimizations. This command will create an executable in 
	   target/release instead of target/debug

7) To see all available cargo option use:  cargo new --help 

*/