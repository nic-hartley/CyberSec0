> title: Compiling Bare-Metal Rust for PinePhone
> category: hpp
> publish: 2020-10-30
---

> **Note**:
> The way I'm writing this is all relatively clean and linear.
> It was _not_ simple to figure out.
> I did my fursona proud and screamed endlessly for, like, a month and a half.

I got a pleasant surprise:
My serial cable showed up earlier than expected!
And now that I've added an extremely cursed 3.5mm-to-USB cable to my collection, I can play with UART.

Unfortunately, that's going to have to wait.
See, while I was waiting for the cable, I started cleaning up the code.
I turned this:

```rust
unsafe {
  core::ptr::write_volatile(0x01C20874 as *mut u32, 0x77711177);
  core::ptr::write_volatile(0x01C2087C as *mut u32, 0x001c0000);
}
```

into this:

```rust
PD_CFG_REG.index(@).write(0x77711177);
PD_DATA_REG.write(0x001c0000);
```

into this:

```rust
hw::led::init();
hw::led::set(hw::led::Color::White);
```

In doing so, though, the code no longer works on my hardware.
After a bit of digging, I've realized that the issue is by no means _impossible_ to solve, but it is a sign that things are going to start getting difficult.
There are two specific problems:

-   Rust, by default, is storing data in `.rodata`, even for constants that could fit entirely in registers.
    Then it loads out of that part of memory.
    Unfortunately, my extremely basic script just copies the `.text` section out, so it won't work as-is.
    It won't be impossible to fix, but it will mean a fairly major change to the build script.
-   Rust also expects the stack to be set up and usable.
    I'm not sure where the stack pointer is by default, but I'm pretty sure it's not pointing at valid memory.

Luckily, the solution here is fairly simple.
I'm going to need to insert a little bit of preamble Assembly to set the stack in the right spot, and building this image is going to get a little more complicated.
I've been planning out how to do this sort of thing for a while, but I was hoping I could get a _little_ further before I had to.
Oh well.

In short, I need a custom linker script -- to make sure everything gets put in the right place in the final binary, for my shell script to be able to yoink it out -- and a custom runtime object, to set things up and call `main`.
I can package both into a somewhat janky custom compile target.
It won't get merged into the mainline anytime soon, but it'll make this compilation quite a lot easier than any of the alternatives.
As a nice bonus, it'll make my build more repeatable, and (very nearly) eliminate the risk of an internal rustc change breaking things<sup>1</sup>.

See, up until now, I've just been yoinking the `.text` section and hoping that the entry point is the beginning of `.text`, because I'm replacing `_start`.
With a custom runtime, `hpprt.o`, written in Assembly, I can guarantee that _that_ thing's text section starts with the executable code I want.
The linker script can put the `hpprt.o`'s `.text` at the beginning of the output `.text` section, so that it _definitely_ starts with my executable code.
Then the runtime can call a main function, and the linker will automatically put things together.

The linker script will _also_ stuff all of the data that needs to be stored in the firmware into the `.text` section.
That keeps the Bash wrapper script nice and simple, since I can just keep pulling out `.text`, without having to worry about pulling other sections out of the binary and putting them in the right spot in the firmware, to get them loaded into the right spot in memory.

Finally, the linker script has a `.bss` section defined, but only so it can calculate where it starts and ends, as `__bss_start` and `__bss_end` respectively.
We'll get to what they're used for and why we need them in a bit.

Now, for the runtime object:
It could do a lot.
I could do most of the startup, load the rest of the kernel, switch to AArch64, and so on.
There are two issues with that:

1.  It'd be incredibly complex.
    I want to keep `hpprt.o` as simple as possible to keep it auditable.
    Implementing all of the startup code in Assembly would make auditing a lot harder.
2.  It wouldn't get me anything.
    Especially for right now, when I'm just exploring the peripherals, I wouldn't get anything out of a more complex `hpprt.o`.
    This point might change eventually, but the first won't.

Instead of anything complex, `hpprt.o` will do just four things:

- Turn on an LED
- Zero out `.bss`
- Set the stack pointer
- Call `k_main`

The LED's purpose is simple:
Give the user instant feedback, as soon as the device is powered on, that the device is on.
As it progresses through the boot, it'll go through the colors, starting at red on first powerup.

Zeroing out `.bss` is a little bit more complex.
In short, `.bss` is the section where all of the _modifiable_ global variables go.
Rust expects the whole region to be zeroed by default.
Luckily, that's fairly simple.
Because `.bss` will always be all zeroes, I don't actually need to store its information.
I can just define a couple of symbols in my linker script, marking the start and end of `.bss`, then zero out that memory region.
Strictly speaking, I don't need it _yet_, but I expect I will before too long.

Setting the stack pointer is self-explanatory.
I just set that to the end of the RAM area, so the stack can grow down into open space.

Finally, I call `k_main`.
It's just a function, exactly the same as any other.
It serves as my entry point into Rust.
I called it `k_main` rather than just `main` because I expect to have a _proper_ main function; `k_main` is mostly going to be setting up the platform to be as the kernel expects it.
At the end, `k_main` will never return, either by powering off the device or looping infinitely.
We'll get to how I can guarantee that in a later blogpost, but for now, a TL;DR: [`!`][never-type].

If you look at the source, you may notice there's some code after the `k_main` call.
All it does is rapidly flash the LED, so that if things go _really_ wrong and `k_main` does return, there's some indication.

Compiling that against another tiny assembly file, where `k_main` just returns immediately, does produce the binary I want.
So I've got all the pieces assembled, and now I just need to make Cargo accept them!

There are basically four ways to do this.
In rough order of cleanliness:

1.  Use a build script which links in `hpprt.o` and adds the linker script.
2.  Write a custom target which incorporates the linker script and `hpprt.o`.
3.  Switch away from workspaces and configure things through `.cargo/config.toml`.
4.  Switch to building with `cargo rustc` and pass the relevant arguments through that.

Unfortunately, number 1 doesn't (yet!) work.
So number 2 it is!
It's not _quite_ as nice as I'd like.
It has some paths relative to the working directory of the build command, so it's pretty brittle, but since I wrap the build in a Bash script, and will eventually be wrapping it up in a custom Cargo subcommand, it's something I can fairly simply deal with.
For now.
Eventually, I'm hoping to figure out a cleaner solution, or that one gets added.

Now, naturally, things didn't just work out straightaway.
I spent a while re-laying-out the target spec format, because I want each target to have a customized spec for itself.
I also had to hunt down a few bugs in my code.
I mixed up `MOVW` and `MOVT` a few times in my Assembly, and some duplicated bitshifts accidentally zeroed things out.
Interestingly, the optimizer managed to detect that, and elided the bitshifts entirely.

I also needed to tweak the linker script a bit, to make room for the boot header.
See, the A64 boots from a (proprietary, only partly reverse-engineered) disk format.
It's more-or-less compatible with standard partitioning, but it does mean that more data than I thought is copied the disk.
In short, the boot header _and_ my code are both copied; I thought it was just my code.
That left all the pointers offset by a bunch, which broke everything Rust tried to generate.

Anyway, with all that, I _think_ I have all of my issues fixed.
I haven't bricked the device, and I've got somewhat complex code actually running.
From here, I can get to properly exploring the device!
That's where we'll pick up next time, in this slow and extremely irregular series of blogposts documenting my slow descent into the abyss.

Happy Halloween!

---

<small>1:
Given the level of hackery I'm doing right now, it's basically impossible to code against any sort of officially stabilized interface.
Technically, a rustc change that breaks things for me could happen whenever.
However, with the linker script and custom runtime object, I've eliminated a lot of the unknown territory that breakages could come from.
The target specs are also kept _incredibly_ stable, despite technically not being stabilized, so that's unlikely to change and break.
</small>

  [ODA]: https://onlinedisassembler.com/odaweb/
  [never-type]: https://doc.rust-lang.org/reference/types/never.html
