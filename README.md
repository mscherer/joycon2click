A quick rust software to convert button push on a Joycon (Nintendo Switch 
controller) to a key, in order to pass on the next slide on Libreoffice.

# Usage

Build with `cargo build -r`, run `target/release/joycon2click`.

Once the joycon is connected, the triggers (L, R, ZL, ZR) and buttons on the right (right arrow on the Dpad for left Joycon, A for the right Joycon) will
go to next slide, and Y and Left arrow will go back to previous slides by pushing Left and Right.

# Bugs

For now, the system requires write access to /dev/uinput, and so root access.

The easiest is to use `chmod o+rw /dev/uinput`, but this is not very granular.

A more suitable command might be: `setfacl -m u:$( id -un):rw /dev/uinput`.

Be aware that opening uinput mean that any software can start injecting key press. For example, 
a malware could type commands when you are using a terminal, etc. 

The software can be started with -u someuser to switch to that user after opening /dev/uinput, so just
need to be started as root.
