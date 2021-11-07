# Tic-Tac-Toe

This is a Tic-Tac-Toe game.
At the beginning you have to enter the size of your grid. If you enter `3`, you will have 3 lines and 3 rows.

## Mode

There are two _modes_, a more development mode with __TRACE__ display, and a more play mode without __TRACE__ display.
To not display the __TRACE__, start the program in _release_ mode with the command:

```bash
cargo build --release
```

or

```bash
cargo run --release
```

## Documentation

To read the documentation of the project, run the command:

```bash
cargo doc --open
```

## Communication package

As you can see, the communication package is not implemented.
However, we put the communication package into this project. This is a synchronize communication. To compile this package run the command:

```bash
rustc src/communication/main.rs -o communication
```

Then by executing the `communication` executable produced, in two different terminals you will see the communication between a client and a server.

## Authors

- Pierre-Louis GAUTIER
- Damien FRISSANT
