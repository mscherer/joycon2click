A quick rust software to convert button push on a Joycon (Nintendo Switch 
controller) to a key, in order to pass on the next slide on Libreoffice.

# Usage

Build with `cargo build -r`, run `target/release/joycon2click`.

Once the joycon is connected, the triggers (L, R, ZL, ZR) and buttons on the right (right arrow on the Dpad for left Joycon, A for the right Joycon) will
go to next slide, and Y and Left arrow will go back to previous slides by pushing Left and Right.
