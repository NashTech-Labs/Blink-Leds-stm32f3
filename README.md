This template is going to provide you a way using which you can complete the led-roulette challenge(Chapter 5) 
of **discovery** book.

## What I am using here to complete the challenge.
1. Rust Programming Language.
2. A stm32f3 Discovery Board.
3. Few dependencies.

## Build the Project

1. To build this project you need to run the command.

`cargo build`

**Note**: Make sure the F3 is connected to your computer and run the following commands in a new terminal.

2. Run command (**Note**: run this command from the new terminal in same directory.)

`openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
   `

Leave this OpenOcd running.

3. Execute GDB 

First, we need to determine what version of gdb you have that is capable of debugging ARM binaries.

Run this command from the new terminal in same directory.

`gdb-multiarch -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/Blink-Leds-stm32f3`

### Failing Case
If you face any error like this-

`$ gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...
Remote debugging using :3333
warning: Architecture rejected target-supplied description
Truncated register 16 in remote 'g' packet
(gdb)`

Then try Command.

`arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/Blink-Leds-stm32f3
`

OR

`gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/Blink-Leds-stm32f3`

### Successful Case
Case 1:

`$ arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...
Remote debugging using :3333
cortex_m_rt::Reset () at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:497
497     pub unsafe extern "C" fn Reset() -> ! {
(gdb)`

Case 2: 

`~/embedded-discovery/src/05-led-roulette (master)
$ arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette
Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...
Remote debugging using :3333
0x00000000 in ?? ()
(gdb)`

## Run the Project
To blink the **Leds** 

Run command:
1. **(gdb) load**
2. **(gdb) continue**

#### You Leds of stm32f3 Board are blinking with 1 sec delay.