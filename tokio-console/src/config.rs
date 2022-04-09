use crate::view::Palette;
use clap::{ArgGroup, Parser as Clap, ValueHint};
use serde::Deserialize;
use std::fs;
use std::ops::Not;
use std::process::Command;
use std::str::FromStr;
use std::time::Duration;
use tonic::transport::Uri;

#[derive(Clap, Debug)]
#[clap(
    name = clap::crate_name!(),
    author,
    about,
    version,
)]
#[deny(missing_docs)]
pub struct Config {
    /// The address of a console-enabled process to connect to.
    ///
    /// This may be an IP address and port, or a DNS name.
    #[clap(default_value = "http://127.0.0.1:6669", value_hint = ValueHint::Url)]
    pub(crate) target_addr: Uri,

    /// Log level filter for the console's internal diagnostics.
    ///
    /// The console will log to stderr if a log level filter is provided. Since
    /// the console application runs interactively, stderr should generally be
    /// redirected to a file to avoid interfering with the console's text output.
    #[clap(long = "log", env = "RUST_LOG", default_value = "off")]
    pub(crate) env_filter: tracing_subscriber::EnvFilter,

    #[clap(flatten)]
    pub(crate) view_options: ViewOptions,

    /// How long to continue displaying completed tasks and dropped resources
    /// after they have been closed.
    ///
    /// This accepts either a duration, parsed as a combination of time spans
    /// (such as `5days 2min 2s`), or `none` to disable removing completed tasks
    /// and dropped resources.
    ///
    /// Each time span is an integer number followed by a suffix. Supported suffixes are:
    ///
    /// * `nsec`, `ns` -- nanoseconds
    ///
    /// * `usec`, `us` -- microseconds
    ///
    /// * `msec`, `ms` -- milliseconds
    ///
    /// * `seconds`, `second`, `sec`, `s`
    ///
    /// * `minutes`, `minute`, `min`, `m`
    ///
    /// * `hours`, `hour`, `hr`, `h`
    ///
    /// * `days`, `day`, `d`
    ///
    /// * `weeks`, `week`, `w`
    ///
    /// * `months`, `month`, `M` -- defined as 30.44 days
    ///
    /// * `years`, `year`, `y` -- defined as 365.25 days
    #[clap(long = "retain-for", default_value = "6s")]
    retain_for: RetainFor,
}

#[derive(Debug)]
struct RetainFor(Option<Duration>);

#[derive(Clap, Debug, Clone)]
#[clap(group = ArgGroup::new("colors").conflicts_with("no-colors"))]
pub struct ViewOptions {
    /// Disable ANSI colors entirely.
    #[clap(name = "no-colors", long = "no-colors")]
    no_colors: Option<bool>,

    /// Overrides the terminal's default language.
    #[clap(long = "lang", env = "LANG")]
    lang: Option<String>,

    /// Explicitly use only ASCII characters.
    #[clap(long = "ascii-only")]
    ascii_only: Option<bool>,

    /// Overrides the value of the `COLORTERM` environment variable.
    ///
    /// If this is set to `24bit` or `truecolor`, 24-bit RGB color support will be enabled.
    #[clap(
        long = "colorterm",
        name = "truecolor",
        env = "COLORTERM",
        parse(from_str = parse_true_color),
        possible_values = &["24bit", "truecolor"],
    )]
    truecolor: Option<bool>,

    /// Explicitly set which color palette to use.
    #[clap(
        long,
        possible_values = &["8", "16", "256", "all", "off"],
        group = "colors",
        conflicts_with_all = &["no-colors", "truecolor"]
    )]
    palette: Option<Palette>,

    #[clap(flatten)]
    toggles: ColorToggles,
}

/// Toggles on and off color coding for individual UI elements.
#[derive(Clap, Debug, Copy, Clone)]
pub struct ColorToggles {
    /// Disable color-coding for duration units.
    #[clap(long = "no-duration-colors", group = "colors")]
    color_durations: Option<bool>,

    /// Disable color-coding for terminated tasks.
    #[clap(long = "no-terminated-colors", group = "colors")]
    color_terminated: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigFile {
    charset: Option<CharsetConfig>,
    colors: Option<ColorsConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CharsetConfig {
    lang: String,
    ascii_only: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColorsConfig {
    enabled: bool,
    truecolor: bool,
    palette: Palette,
    enable: ColorsEnable,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColorsEnable {
    durations: bool,
    terminated: bool,
}

// === impl Config ===

impl Config {
    pub fn from_config() -> Self {
        let base_view_options = ConfigFile::from_config().map(|config| config.into_view_options());
        let mut config = Self::parse();
        let view_options = match base_view_options {
            None => config.view_options,
            Some(mut base) => {
                base.merge_with(config.view_options);
                base
            }
        };
        config.view_options = view_options;
        config
    }

    pub fn trace_init(&mut self) -> color_eyre::Result<()> {
        let filter = std::mem::take(&mut self.env_filter);
        use tracing_subscriber::prelude::*;

        // If we're on a Linux distro with journald, try logging to the system
        // journal so we don't interfere with text output.
        #[cfg(all(feature = "tracing-journald", target_os = "linux"))]
        let (journald, should_fmt) = {
            let journald = tracing_journald::layer().ok();
            let should_fmt = journald.is_none();
            (journald, should_fmt)
        };

        #[cfg(not(all(feature = "tracing-journald", target_os = "linux")))]
        let should_fmt = true;

        // Otherwise, log to stderr and rely on the user redirecting output.
        let fmt = if should_fmt {
            Some(
                tracing_subscriber::fmt::layer()
                    .with_writer(std::io::stderr)
                    .with_ansi(atty::is(atty::Stream::Stderr)),
            )
        } else {
            None
        };

        let registry = tracing_subscriber::registry().with(fmt).with(filter);

        #[cfg(all(feature = "tracing-journald", target_os = "linux"))]
        let registry = registry.with(journald);

        registry.try_init()?;

        Ok(())
    }

    pub(crate) fn retain_for(&self) -> Option<Duration> {
        self.retain_for.0
    }
}

// === impl ViewOptions ===

impl ViewOptions {
    pub fn is_utf8(&self) -> bool {
        let lang = self.lang.clone().unwrap_or_default();
        let ascii_only = self.ascii_only.unwrap_or(true);
        lang.ends_with("UTF-8") && !ascii_only
    }

    /// Determines the color palette to use.
    ///
    /// The color palette is determined based on the following (in order):
    /// - Any palette explicitly set via the command-line options
    /// - The terminal's advertised support for true colors via the `COLORTERM`
    ///   env var.
    /// - Checking the `terminfo` database via `tput`
    pub(crate) fn determine_palette(&self) -> Palette {
        // Did the user explicitly disable colors?
        if self.no_colors.unwrap_or(true) {
            tracing::debug!("colors explicitly disabled by `--no-colors`");
            return Palette::NoColors;
        }

        // Did the user explicitly select a palette?
        if let Some(palette) = self.palette {
            tracing::debug!(?palette, "colors selected via `--palette`");
            return palette;
        }

        // Does the terminal advertise truecolor support via the COLORTERM env var?
        if self.truecolor.unwrap_or(false) {
            tracing::debug!("millions of colors enabled via `COLORTERM=truecolor`");
            return Palette::All;
        }

        // Okay, try to use `tput` to ask the terminfo database how many colors
        // are supported...
        let tput = Command::new("tput").arg("colors").output();
        tracing::debug!(?tput, "checking `tput colors`");
        if let Ok(output) = tput {
            let stdout = String::from_utf8(output.stdout);
            tracing::debug!(?stdout, "`tput colors` succeeded");
            return stdout
                .map_err(|err| tracing::warn!(%err, "`tput colors` stdout was not utf-8 (this shouldn't happen)"))
                .and_then(|s| {
                    let palette = s.parse::<Palette>();
                    tracing::debug!(?palette, "parsed `tput colors`");
                    palette.map_err(|_| tracing::warn!(palette = ?s, "invalid color palette from `tput colors`"))
                })
                .unwrap_or_default();
        }

        Palette::NoColors
    }

    pub(crate) fn toggles(&self) -> ColorToggles {
        self.toggles
    }

    fn merge_with(&mut self, command_line: ViewOptions) {
        if command_line.no_colors.is_some() {
            self.no_colors = command_line.no_colors;
        }

        if command_line.lang.is_some() {
            self.lang = command_line.lang;
        }

        if command_line.ascii_only.is_some() {
            self.ascii_only = command_line.ascii_only;
        }

        if command_line.truecolor.is_some() {
            self.truecolor = command_line.truecolor;
        }

        if command_line.palette.is_some() {
            self.palette = command_line.palette;
        }

        if command_line.toggles.color_durations.is_some() {
            self.toggles.color_durations = command_line.toggles.color_durations;
        }

        if command_line.toggles.color_terminated.is_some() {
            self.toggles.color_terminated = command_line.toggles.color_terminated;
        }
    }
}

fn parse_true_color(s: &str) -> bool {
    let s = s.trim();
    s.eq_ignore_ascii_case("truecolor") || s.eq_ignore_ascii_case("24bit")
}

impl FromStr for RetainFor {
    type Err = humantime::DurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("none") => Ok(RetainFor(None)),
            _ => s
                .parse::<humantime::Duration>()
                .map(|duration| RetainFor(Some(duration.into()))),
        }
    }
}

// === impl ColorToggles ===

impl ColorToggles {
    pub fn color_durations(&self) -> bool {
        self.color_durations.map(std::ops::Not::not).unwrap_or(true)
    }

    pub fn color_terminated(&self) -> bool {
        self.color_durations.map(std::ops::Not::not).unwrap_or(true)
    }
}

// === impl ColorToggles ===

impl ConfigFile {
    fn from_config() -> Option<Self> {
        let mut base = dirs::config_dir();
        if let Some(path) = base.as_mut() {
            path.push("tokio-console/console.toml");
        }
        let base = base.and_then(|path| fs::read_to_string(path).ok());
        let base_file: Option<ConfigFile> = base.and_then(|raw| toml::from_str(&raw).ok());

        let current = fs::read_to_string("./console.toml").ok();
        let current_file: Option<ConfigFile> = current.and_then(|raw| toml::from_str(&raw).ok());
        merge_config_file(base_file, current_file)
    }

    fn into_view_options(self) -> ViewOptions {
        ViewOptions {
            no_colors: self.colors.as_ref().map(|config| Not::not(config.enabled)),
            lang: self.charset.as_ref().map(|config| config.lang.to_string()),
            ascii_only: self.charset.as_ref().map(|config| config.ascii_only),
            truecolor: self.colors.as_ref().map(|config| config.truecolor),
            palette: self.colors.as_ref().map(|config| config.palette),
            toggles: ColorToggles {
                color_durations: self
                    .colors
                    .as_ref()
                    .map(|config| Not::not(config.enable.durations)),
                color_terminated: self.colors.map(|config| Not::not(config.enable.terminated)),
            },
        }
    }
}

fn merge_config_file(before: Option<ConfigFile>, after: Option<ConfigFile>) -> Option<ConfigFile> {
    match (before, after) {
        (None, None) => None,
        (before @ Some(_), None) => before,
        (None, after @ Some(_)) => after,
        (Some(mut before), Some(after)) => {
            let ConfigFile { charset, colors } = after;
            if let Some(charset) = charset {
                before.charset = Some(charset)
            }
            if let Some(colors) = colors {
                before.colors = Some(colors)
            }
            Some(before)
        }
    }
}
