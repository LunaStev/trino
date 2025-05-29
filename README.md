# Trino - Ternary Virtual Machine Simulator

Trino is a CPU simulator based on ternary logic. While not actual hardware, it implements a virtual computer based on ternary logic.

## Usage

```bash
cargo run -- examples/sample.tri
```

## Example program

```tri
001012   # LOAD R0, 5
002001   # ADD R0, 1
999000   # HALT
```

## License 

MPL-2.0