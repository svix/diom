use std::io::{BufRead, IsTerminal, Write};

pub fn prompt(message: impl AsRef<str>) -> anyhow::Result<bool> {
    let message = message.as_ref();
    if !std::io::stdin().is_terminal() {
        anyhow::bail!("stdin is not a terminal");
    }
    let mut stdout = std::io::stdout().lock();
    let mut stdin = std::io::stdin().lock();
    loop {
        write!(stdout, "{message} (y/N)? ")?;
        stdout.flush()?;
        let mut buf = String::with_capacity(1);
        if stdin.read_line(&mut buf)? == 0 {
            // user hit ctrl-d
            return Ok(false);
        }
        writeln!(stdout)?;
        stdout.flush()?;
        let buf = buf.trim();
        if buf == "y" || buf == "Y" {
            return Ok(true);
        } else if buf == "n" || buf == "N" {
            return Ok(false);
        } else {
            eprintln!("Not sure what '{buf}' means");
        }
    }
}
