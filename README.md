A quickly written software to convert button push on a Joycon (Nintendo Switch 
controller) to a regular keypress in order to pass on the next slide on Libreoffice.

# Usage

Build with `cargo build -r`, run `target/release/joycon2click`.

Run Libreoffice (or anything using left and right) in full screen.

Once the Joycon is connected, the triggers (L, R, ZL, ZR) and buttons on the right (right arrow on the Dpad for left Joycon, A for the right Joycon) will
go to the next slide, and Y and Left arrow will go back to the previous slide by pushing Left and Right.

# Bugs

## Uinput and root account

The system requires write access to /dev/uinput, which requires root access.

The easiest is to use `chmod o+rw /dev/uinput`, but this is a blunt fix.

A more restricted fix would be: `setfacl -m u:$(id -un):rw /dev/uinput`, which restrict uinput
to the current user.

Be aware that opening uinput this way mean that any software can start injecting key press. For example, 
a malware could type commands when you are using a terminal, etc. 

A last potential fix is to start the software as root and use the option `-u someuser` to switch to a unprivilged user after /dev/uinput is opened. Make sure to switch to either the user on the console, or a user in the `input` group.

## Seccomp

By default, the program will limit itself using seccomp and BPF. This is still a experimental feature, and can be disabled with the `--disable_seccomp` argument if this cause issues.
For now, the filtering is not very granular and only allow a few syscalls, but there is plans to later restrict their arguments.

## Multiple joycon support

Only 1 joycon is supported (usually, the 1st one attached, but no garantee on that). Multiple joycons support is out of scope.

## Do not detect joycon disconnection

If the joycon is disconnected, nothing happen. If it reconnect, the software will stop, unless if `-b` is given on the commandline.

# Alternatives

A few people have wrote softwares similar to that one, I list them here for reference:

* https://github.com/HonbraDev/joyclicker-rs
