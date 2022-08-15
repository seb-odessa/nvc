use std::io;
use std::io::BufRead;
use std::io::Write;
use std::thread;
use std::time;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use lib::HashMapNvc;
use lib::NextValidCharacter;

fn main() -> io::Result<()> {
    let mut nvc = HashMapNvc::new();
    println!("NVC building start.");
    for line in read_lines("./data/names.txt")? {
        nvc.parse(&line);
    }
    println!("NVC building complete.");

    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
    let mut path = String::new();

    loop {
        let chars = nvc.next(&path);

        let msg = chars
            .into_iter()
            .flat_map(|c| [c, ' '])
            // .take(path.len() * 2 - 1)
            .collect::<String>();

        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            msg
        )
        .unwrap();

        write!(stdout, "{}>{path}", termion::cursor::Left(1)).unwrap();

        stdout.lock().flush().unwrap();

        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Esc => break,
                termion::event::Key::Backspace => {
                    if !path.is_empty() {
                        path.truncate(path.len() - 1);
                    }
                }
                termion::event::Key::Char(ch) => {
                    path.push(ch);
                }
                _ => {}
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////
fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let mut lines = Vec::new();
    let file = std::fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            lines.push(line);
        }
    }
    return Ok(lines);
}
