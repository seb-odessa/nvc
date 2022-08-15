extern crate ncurses;

use ncurses::*;
use std::char;

fn main() {
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Prompt for a character. */
    addstr("Enter a character: ");

    loop {
        /* Wait for input. */
        let ch = getch();
        if ch == KEY_F(1) {
            /* Enable attributes and output message. */
            addstr("\nF1");
            addstr(" pressed");
        } else {
            /* Enable attributes and output message. */
            addstr("\nKey pressed: ");
            addstr(format!("{}\n", char::from_u32(ch as u32).expect("Invalid char")).as_ref());
        }

        /* Refresh, showing the previous message. */
        refresh();
    }

    /* Wait for one more character before exiting. */
    getch();
    endwin();
}
