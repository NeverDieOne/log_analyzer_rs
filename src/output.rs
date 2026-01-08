use std::io::Write;

pub enum Output {
    Line(String),
    Begin,
    End,
}

pub struct OutputWriter<W: Write> {
    writer: W,
}

impl<W: Write> OutputWriter<W> {
    pub fn new(writer: W) -> Self {
        OutputWriter { writer }
    }

    pub fn write(&mut self, output: &Output) -> std::io::Result<()> {
        match output {
            Output::Line(line) => {
                self.writer.write_all(line.as_bytes())
            }
            Output::Begin => self.writer.write_all(b"[\n"),
            Output::End => self.writer.write_all(b"\n]\n"),
        }
    }
}
