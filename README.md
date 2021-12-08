# SUS 
**Skj Udp1 Simulator**

Simple App simulating SKJ prog. task nr 3 PL <br>

# Features
- Detailed info about "what's going on"
- Random tasks based on a given seed

# Config
```json
{
  "tcp_address": "localhost:666",
  "udp_address": "localhost:999",
  "seed": 100,
  "init_flag": 0,
  "final_flag": 1
}
```

- `tcp_address` - address for TCP server to listen on
- `udp_address` - address for UDP server to listen on
- `seed` - number used for pseudo random generation
- `init_flag` - flag required by TCP server to initialize the connection
- `final_flag` - flag sent by UDP server if all tasks were completed

# Usage

### sim.exe
1. Make sure that `config.json` and `sim.exe` are in the same directory
2. Open `CMD` in the same directory where `sim.exe`
3. Enter `sim.exe` into the console window
4. Done

### src
1. Make sure that `config.json` is in the project's directory
2. Run with `cargo run --release`
3. Done

# Tasks
**[GCD]:** Greatest common divisor of `X` amount of numbers <br>
**[SUM]:** Sum of `X` amount of numbers <br>
**[XK]:** Find number `X` raised to power of `Y` not greater than value `Z` <br>
**[SD]:** Delete `X` from string `Y` <br>
**[SC]:** Multiply string `X` by 2 *(concatenation)* <br>
