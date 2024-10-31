A quickly written software to convert button push on a Joycon (Nintendo Switch 
controller) to a regular keypress in order to pass on the next slide on Libreoffice.

# Usage

Build with `cargo build -r`, run `target/release/joycon2click`.

Run Libreoffice (or anything using left and right) in full screen.

Once the Joycon is connected, the triggers (L, R, ZL, ZR) and buttons on the right (right arrow on the Dpad for left Joycon, A for the right Joycon) will
go to the next slide, and Y and Left arrow will go back to the previous slide by pushing Left and Right.

# Bugs

The system requires write access to /dev/uinput, which requires root access.

The easiest is to use `chmod o+rw /dev/uinput`, but this is a blunt fix.

A more restricted fix would be: `setfacl -m u:$(id -un):rw /dev/uinput`, which restrict uinput
to the current user.

Be aware that opening uinput this way mean that any software can start injecting key press. For example, 
a malware could type commands when you are using a terminal, etc. 

A last potential fix is to start the software as root and use the option `-u someuser` to switch to a unprivilged user after /dev/uinput is opened.
