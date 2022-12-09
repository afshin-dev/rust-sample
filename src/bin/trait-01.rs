trait StrWriter {
     fn write(&self,s: &str );
}

struct Logger ;
struct Reporter ;

impl StrWriter for Logger {
     fn write(&self,s: &str ) {
        println!("log: {}", s);
    }
}

impl StrWriter for Reporter {
    fn write(&self,s: &str ) {
        println!("report: {}", s);
    }
}

fn write_str(s: &str, str_writer: impl StrWriter) {
    str_writer.write(s);
}

fn main() {
    let logger = Logger {} ;
    let reporter = Reporter {} ;
    write_str("trait is", logger);
    write_str("for deduplication of codeing", reporter);
}