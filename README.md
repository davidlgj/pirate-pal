## Super simple program so I can shut down my raspotify pirate audiop by pressing the a button


## Notes
four buttons, active low connected to BCM 5, 6, 16, and 24 (A, B, X, Y respectively)


Using cross to cross compile since arch has a glibc version that is later than raspbian and breaks
with the error `/lib/aarch64-linux-gnu/libc.so.6: version `GLIBC_2.39' not found`

So now I build with `cross build --target aarch64-unknown-linux-gnu`
