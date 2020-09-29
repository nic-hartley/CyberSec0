> title: Exploring, Part 1: Firmware
> category: hpp
> publish: 2020-09-29
---

My PinePhone just arrived!
After a bit of struggle getting SSH working -- I accidentally disabled it in the postmarketOS installer, but luckily it wasn't hard to get back -- I'm successfully in the device.
This project, of course, is gonna require a lot of research.
It's also gonna require a lot of design work, which I might talk about later.
For now, I've got something more viscerally fun:
Device exploration!

(Spoiler alert: It starts bad and gets worse.)

The first thing I need to do is examine how the hardware actually works.
That's gonna mean at the very least poking through the memory, and maybe poking _at_ the memory, fiddling with MMIO.
Unfortunately, I can't actually get that to work in postmarketOS.
Hitting `/dev/mem` keeps giving me a `Bad address`, even though I'm hitting addresses that should be valid.
`/dev/mem` also has the usual one-meg limit on it.
All this tells me that, unfortunately, I'm not actually gonna get much exploration done through postmarketOS.

So, let's get to work on this exploration, then.
It's gonna be ugly.
I'm gonna have to write firmware.
That means it's gonna be a lot slower than I'd hoped.

Step 1 is a simple one:
See if I can compile Rust to the right platform.
Turns out, that's pretty easy!
All you need to do is `rustup target install aarch64-unknown-none`.
SCP that to the phone, run it, and it works!

Cool!
Uh.
Okay, so now things get complicated.
First, I want to draw your attention to [the HeartPinePhone git repo][git repo].
There are four subprojects there, but the important one _for now_ is `explorer`.
It's... well, custom firmware, sort of, but that's a very grandiose way to put it.
It's really just whatever random code I want to run on the device, for whatever random exploration I'm trying to do.

To get it onto my device, I'm first gonna flash [Jumpdrive] onto an SD card.
That'll let me easily flash the internal memory with my test firmware.
So let's start exploring!

Step 1 is just blinking the LED.
For the sake of education, I'm gonna explain my 
A quick look at [the PinePhone schematic][schematic] tells me which pins on the CPU connect to the LED:

![LED2's hookups are listed: PD18 for red, PD19 for green, PD20 for blue.][schematic-led]

A quick cross-reference between [a different part of the schematic][schematic-cpu] and the [relevant part of the user manual][manual-pins] leaves me satisfied that PD18 through PD20 are the right pins.
The manual makes it pretty clear what I (think I) need to do:

- Set PD18 through PD20 to Output by modifying PD_CFG2_REG
- Set the bits for PD18 through PD20 high in PD_DATA_REG
- Loop infinitely (which I'm gonna change soon)

That's pretty simple:

```rust
unsafe {
  *(0x1c20874 as *mut u32) = 0x77711177;
  *(0x1c2087c as *mut u32) = 0x001c0000;
  loop { }
}
```

And now we have the challenge of getting that Rust code to compile to raw bytecode.
[Someone][virkkunen] in the [Rust Community discord server][rust-community] pointed me to the [Rust RPi OS][rpi-os] tutorials, which will at least help me convert my Rust ELF binary to bytecode that will work on its own.
For now, I can just make a quick `#![no_std]` wrapper -- you can see it [here][repo-then], as well as the hacky shell script that pulls the actual code out and stuffs it into the right image format.

Hunting _down_ that image format was a struggle.
I'm not gonna document it here because it was mostly several days of trying a bunch of things, failing, asking around, not getting answers, and then finally being told by [megi] I'd had the image format right but needed to compile to A32 instead of A64.
Oops.

For now, I'm using a Bash script and a tiny third-party tool to massage it into the firmware format.
I'm gonna be cleaning that up into a separate Rust tool eventually, but for now, I just need to be able to compile to firmware.

With this, I've got a decent start on my explorer.
While I was at it, I got vibration working too, since it's the exact same code but for a different pin in the port.

So, that's enough effort for one blog post.
I know, not very satisfying, but I've been dragging myself through this for two weeks, and I want to post something!
I've got a pretty solid roadmap for my next steps, though:

1.  **Power**.
    I want to be able to actually shut off the phone, and it'll give me an excuse dig a bit deeper into how the communication stuff works.
    Plus, I'll need this to use the screen, and probably a few other components as well.
2.   **UART**.
    I don't have the headphone jack serial cable I want yet, which is why power is ahead of this, but once I do, I'll be able to communicate back and forth with the phone.
3.  **64-bit mode**.
    After I have UART, I want things to get fancy.
    I want a loader which, at least for now, will just be getting the CPU into AArch64 mode.
    Eventually it'll handle a bunch more, but for now, switching to 64-bit is all I want.
4.  **USB**.
    This is going to be necessary eventually, for a lot of things.
    For now, I'm just using it as an excuse to start digging into porting Linux driver and kernel code.
    I'm going to have to learn how eventually; I may as well start with something tangentially useful.
    Specifically, I'm gonna try to implement USB as a block device.
5.  **CPU control**.
    Clocking CPUs up higher, turning on more than one, and so on.
    Maybe even splitting processing among cores -- UART on CPU0, USB on CPU1, etc.

From there, it gets a bit fuzzier.
I could poke at WiFi, or look at the timers.
I'm going to try to avoid planning _too_ much.
I will, however, eventually start maintaining an unordered list of components to make.

For now, I hope you enjoyed this long, meandering, pointless and anticlimactic blogpost about my suffering!

  [git repo]: https://github.com/nic-hartley/HeartPinePhone
  [Jumpdrive]: https://github.com/dreemurrs-embedded/Jumpdrive
  [schematic]: http://files.pine64.org/doc/PinePhone/PinePhone%20v1.2a%20Released%20Schematic.pdf "This is the v1.2a version, which is the one I have. If you have a different model, you should grab that version's schematic. I don't really expect it to change, though."
  [manual]: http://files.pine64.org/doc/datasheet/pine64/Allwinner_A64_User_Manual_V1.0.pdf
  [schematic-led]: /post-assets/hpp-exploration-1/schematic-led.png
  [schematic-cpu]: /post-assets/hpp-exploration-1/schematic-cpu.png
  [manual-pins]: /post-assets/hpp-exploration-1/manual-pins.png
  [virkkunen]: https://virkkunen.net/
  [rust-community]: https://discord.gg/aVESxV8
  [rpi-os]: https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials
  [repo-then]: https://github.com/nic-hartley/HeartPinePhone/tree/33462eea4e794c9b9b0a499605dd283c6b28f485
