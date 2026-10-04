//! Diagnostic output for the `bragi` binary (CR-102).
//!
//! Everything the CLI says about its own progress, and every event the core
//! library emits, is a `tracing` event written to **stderr**. Stdout is
//! reserved for command results (`bragi strip` without `-o`, `--show-configs`,
//! `--help`, `--version`).
//!
//! Verbosity:
//!
//! | flag       | filter                                   | shows                                   |
//! |------------|------------------------------------------|-----------------------------------------|
//! | `-q`       | `error`                                  | errors only                             |
//! | (none)     | `warn,bragi=info,bragi_io_core=warn`     | the CLI's own progress lines + warnings |
//! | `-v`       | `info`                                   | + the library's per-parse summary       |
//! | `-vv`      | `warn,bragi=debug`                       | + rule-by-rule and stage detail         |
//! | `-vvv`     | `warn,bragi=trace`                       | + per-item detail                       |
//!
//! `RUST_LOG`, when set, replaces the flag-derived filter entirely (standard
//! `EnvFilter` syntax, e.g. `RUST_LOG=bragi_io_core::rules=debug`).
//!
//! Two renderings: at the default and `-v` levels lines are plain prose
//! (info lines bare, `warning:` / `error:` prefixes otherwise), which is
//! what a person at a terminal wants. From `-vv` up, or whenever `RUST_LOG`
//! is set, the standard `tracing-subscriber` format is used — elapsed time,
//! level and target on every line — which is what a person debugging wants.

use std::fmt;
use std::io::IsTerminal;

use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::EnvFilter;

/// Filter directive per verbosity step. Directive targets match by prefix,
/// so `bragi` covers this binary (`bragi`), its library (`bragi_io`) and the
/// core library (`bragi_io_core`); the default narrows core back to `warn`.
/// `bragi_io_core::profile` carries the `--profile` timing table, which is
/// only emitted when profiling was asked for, so it is always let through.
fn directives(verbosity: i8) -> &'static str {
    match verbosity {
        i8::MIN..=-1 => "error",
        0 => "warn,bragi=info,bragi_io_core=warn,bragi_io_core::profile=info",
        1 => "info",
        2 => "warn,bragi=debug",
        _ => "warn,bragi=trace",
    }
}

/// Install the global subscriber. `verbose` is the `-v` count, `quiet` the
/// `-q` flag. Call once, before anything logs.
pub fn init(verbose: u8, quiet: bool) {
    let verbosity: i8 = if quiet {
        -1
    } else {
        verbose.min(3) as i8
    };

    let from_env = std::env::var("RUST_LOG")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .and_then(|v| EnvFilter::try_new(v).ok());
    let detailed = from_env.is_some() || verbosity >= 2;
    let filter = from_env.unwrap_or_else(|| EnvFilter::new(directives(verbosity)));

    let builder = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr);

    if detailed {
        builder
            .with_ansi(std::io::stderr().is_terminal())
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .init();
    } else {
        builder.with_ansi(false).event_format(PlainFormat).init();
    }
}

/// Plain rendering for the default and `-v` levels: the message and any
/// structured fields, with no timestamp or target. Info lines are bare;
/// warnings and errors get a `warning:` / `error:` prefix.
struct PlainFormat;

impl<S, N> FormatEvent<S, N> for PlainFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        match *event.metadata().level() {
            Level::ERROR => write!(writer, "error: ")?,
            Level::WARN => write!(writer, "warning: ")?,
            Level::INFO => {}
            level => write!(writer, "{level} {}: ", event.metadata().target())?,
        }
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

/// Report a fatal error on stderr and exit with `code`.
///
/// Written directly rather than through the subscriber so that the message
/// survives any filter (`-q`, or a `RUST_LOG` aimed at other crates):
/// callers that run the binary read stderr for the error text on a
/// non-zero exit.
pub fn fail(code: i32, message: impl fmt::Display) -> ! {
    eprintln!("error: {message}");
    std::process::exit(code)
}
