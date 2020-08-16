#![recursion_limit="456"]
#![allow(clippy::slow_vector_initialization)]
#[path = "../macros/pikachu_macro.rs"]
#[macro_use]
mod pikachu_macro;
/*
pikachu-lang    brainfuck
pika! pika!     >           increment the data pointer
pika. pika.     <           decrement the data pointer
pika. pika!     +           increment the byte at pointer
pika! pika.     -           decrement the byte at pointer
pikachu!        .           output the byte at pointer
pikachu?        ,           input of one byte into pointer
pika pi?        [           Jump forward past the matching ] if the byte at the pointer is zero.
pika pi!        ]           if pointer is nonzero, jump to matching pika

[0111 0000]    [112]    p
[0110 1001]    [105]    i
[0110 1011]    [107]    k
[0110 0001]    [97]     a
[0010 0000]    [32]    (space)
[0111 0000]    [112]    p
[0110 1001]    [105]    i
[0110 1011]    [107]    k
[0110 0001]    [97]     a
[0110 0011]    [99]     c
[0110 1000]    [104]    h
[0111 0101]    [117]    u
[0010 0001]    [33]     !
[0000 1000]    [10]    (line feed)
*/
fn main() {
    let _ = pikachu!(
        pika! pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika pi?     pika. pika.
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika! pika!  pika! pika.  pika pi!     pika. pika.
        pika. pika!  pika. pika!
        pikachu!
        pika! pika!
        pika! pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika pi?     pika. pika.  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika! pika!  pika! pika.  pika pi!     pika. pika.  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pikachu!
        pika! pika!
        pika! pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika pi?     pika. pika.
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika! pika!  pika! pika.  pika pi!     pika. pika.
        pika! pika.  pika! pika.  pika! pika.
        pikachu!
        pika! pika!
        pika! pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika pi?     pika. pika.  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika! pika!  pika! pika.  pika pi!     pika. pika.  pika! pika.
        pika! pika.  pika! pika.
        pikachu!
        pika! pika!
        pika! pika!  pika. pika! pika. pika! pika. pika! pika. pika! pika. pika!
        pika pi?
            pika! pika!
            pika. pika!
            pika pi?
                pika. pika.
                pika. pika!
                pika. pika!
                pika! pika!
                pika! pika.
            pika pi!
            pika. pika.
            pika. pika.
            pika. pika!
            pika. pika!
            pika. pika!
            pika. pika!
            pika. pika!
            pika. pika!
            pika! pika!
            pika! pika.
            pika! pika.
            pika! pika.
        pika pi!
        pika. pika.
        pika. pika! pika. pika!
        pikachu!
        pika! pika!


        pika! pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika pi?     pika. pika.
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika! pika!  pika! pika.  pika pi!     pika. pika.
        pika. pika!  pika. pika!
        pikachu!
        pika! pika!


        pika! pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika pi?     pika. pika.  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika! pika!  pika! pika.  pika pi!     pika. pika.  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pikachu!
        pika! pika!


        pika! pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika pi?     pika. pika.
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika! pika!  pika! pika.  pika pi!     pika. pika.
        pika! pika.  pika! pika.  pika! pika.
        pikachu!
        pika! pika!


        pika! pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika pi?     pika. pika.  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika! pika!  pika! pika.  pika pi!     pika. pika.  pika! pika.
        pika! pika.  pika! pika.
        pikachu!
        pika! pika!



        pika! pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika pi?     pika. pika.  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika! pika!  pika! pika.  pika pi!     pika. pika.  pika! pika.
        pikachu!
        pika! pika!


        pika! pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!   pika pi?     pika. pika.
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika! pika!  pika! pika.  pika pi!     pika. pika.
        pika. pika! pika. pika! pika. pika! pika. pika!
        pikachu!
        pika! pika!


        pika! pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika pi?     pika. pika.  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!  pika. pika!
        pika. pika!  pika. pika!  pika. pika!  pika. pika! pika! pika!  pika! pika.  pika pi!     pika. pika.  pika! pika.
        pika! pika. pika! pika.
        pikachu!
        pika! pika!




        pika! pika!  pika. pika! pika. pika! pika. pika! pika. pika! pika. pika!
        pika pi?
            pika! pika!
            pika. pika!
            pika pi?
                pika. pika.
                pika. pika!
                pika. pika!
                pika! pika!
                pika! pika.
            pika pi!
            pika. pika.
            pika. pika.
            pika. pika!
            pika. pika!
            pika. pika!
            pika. pika!
            pika. pika!
            pika. pika!
            pika! pika!
            pika! pika.
            pika! pika.
            pika! pika.
        pika pi!
        pika. pika.
        pika. pika! pika. pika! pika. pika!
        pikachu!
        pika! pika!

    );
}


