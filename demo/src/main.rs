use std::{fs::File, io::Write};

fn main() {
    let mut f = File::options().append(true).open("foo.txt").unwrap();
    // ("foo.txt").unwrap();
    f.write(b"some bytes").unwrap();
    let mut f2 = File::create_new("bar.txt").unwrap();
    f2.write_all(buf)
}
