use csv;

use CliResult;
use config::{Config, Delimiter};
use util;

static USAGE: &'static str = "
Formats CSV data with a custom delimiter or CRLF line endings.

Generally, all commands in xsv output CSV data in a default format, which is
the same as the default format for reading CSV data. This makes it easy to
pipe multiple xsv commands together. However, you may want the final result to
have a specific delimiter or record separator, and this is where 'xsv fmt' is
useful.

Usage:
    xsv fmt [options] [<input>]

fmt options:
    -t, --out-delimiter <arg>  The field delimiter for writing CSV data.
                               [default: ,]
    --crlf                     Use '\\r\\n' line endings in the output.
    --ascii                    Use ASCII field and record separators.
    --quote <arg>              The quote character to use. [default: \"]
    --quote-always             Put quotes around every value.
    --escape <arg>             The escape character to use. When not specified,
                               quotes are escaped by doubling them.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. [default: ,]
";

#[derive(RustcDecodable)]
struct Args {
    arg_input: Option<String>,
    flag_out_delimiter: Option<Delimiter>,
    flag_crlf: bool,
    flag_ascii: bool,
    flag_output: Option<String>,
    flag_delimiter: Option<Delimiter>,
    flag_quote: Delimiter,
    flag_quote_always: bool,
    flag_escape: Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = try!(util::get_args(USAGE, argv));

    let rconfig = Config::new(&args.arg_input)
                         .delimiter(args.flag_delimiter)
                         .no_headers(true);
    let wconfig = Config::new(&args.flag_output)
                         .delimiter(args.flag_out_delimiter)
                         .crlf(args.flag_crlf);
    let mut rdr = try!(rconfig.reader());
    let mut wtr = try!(wconfig.writer());

    if args.flag_ascii {
        wtr = wtr.delimiter(b'\x1f')
                 .record_terminator(csv::RecordTerminator::Any(b'\x1e'));
    }
    if args.flag_quote_always {
        wtr = wtr.quote_style(csv::QuoteStyle::Always);
    }
    if let Some(escape) = args.flag_escape {
        wtr = wtr.escape(escape.as_byte()).double_quote(false);
    }
    wtr = wtr.quote(args.flag_quote.as_byte());
    for r in rdr.byte_records() {
        try!(wtr.write(try!(r).into_iter()));
    }
    try!(wtr.flush());
    Ok(())
}
