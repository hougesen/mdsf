use std::{ffi::OsStr, io::Write};

use process_control::{ChildExt, Control};
use tempfile::NamedTempFile;
use which::which;

use crate::{
    config::MdsfConfig,
    error::MdsfError,
    fttype::get_file_extension,
    get_project_dir,
    runners::{CommandType, JavaScriptRuntime},
    terminal::{
        print_binary_not_in_path, print_error_formatting, print_formatter_info,
        print_formatter_time,
    },
    tools::Tooling,
    LineInfo,
};

#[inline]
fn setup_temp_dir() -> std::io::Result<()> {
    let dir = get_project_dir();

    std::fs::write(
        dir.join(".gitignore"),
        "Automatically created by mdsf.
.gitignore
caches
*
",
    )?;

    Ok(())
}

#[inline]
pub fn setup_snippet(code: &str, file_ext: &str) -> std::io::Result<NamedTempFile> {
    let mut b = tempfile::Builder::new();

    let dir = get_project_dir();

    b.rand_bytes(12).suffix(file_ext).prefix(
        // ktlint wants PascalCase file names
        if file_ext == get_file_extension("kotlin") {
            "MdsfFile"
        } else {
            "mdsf"
        },
    );

    if !dir.join(".gitignore").try_exists().unwrap_or_default() {
        setup_temp_dir()?;
    }

    let mut f = b.tempfile_in(dir)?;

    f.write_all(code.as_bytes())?;
    f.flush()?;

    Ok(f)
}

#[inline]
pub fn read_snippet(file_path: &std::path::Path) -> std::io::Result<String> {
    std::fs::read_to_string(file_path)
}

#[inline]
fn handle_post_execution(
    result: Result<std::process::Output, MdsfError>,
    snippet_path: &std::path::Path,
    is_stdin: bool,
) -> Result<(bool, Option<String>), MdsfError> {
    match result {
        Ok(output) => {
            if output.status.success() {
                if is_stdin {
                    let code = String::from_utf8_lossy(&output.stdout).to_string();

                    Ok((false, Some(code)))
                } else {
                    read_snippet(snippet_path)
                        .map(|code| (false, Some(code)))
                        .map_err(MdsfError::from)
                }
            } else {
                Err(MdsfError::FormatterError(
                    String::from_utf8_lossy(&output.stderr).to_string(),
                ))
            }
        }
        Err(MdsfError::Io(err)) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                Ok((true, None))
            } else {
                Err(MdsfError::from(err))
            }
        }
        Err(err) => Err(err),
    }
}

fn spawn_command(
    mut cmd: std::process::Command,
    timeout: u64,
    is_stdin: bool,
    snippet_path: &std::path::Path,
    debug_enabled: bool,
) -> Result<std::process::Output, MdsfError> {
    if debug_enabled {
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());
    }

    let spawned = if is_stdin {
        let file_content = std::fs::read(snippet_path)?;

        let mut handle = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let child_stdin = handle.stdin.as_mut().ok_or(MdsfError::StdinWriteError)?;

        child_stdin.write_all(&file_content)?;
        child_stdin.flush()?;

        handle
    } else {
        cmd.spawn()?
    };

    if timeout == 0 {
        spawned.wait_with_output().map_err(MdsfError::Io)
    } else {
        spawned
            .controlled_with_output()
            .time_limit(std::time::Duration::from_secs(timeout))
            .terminate_for_timeout()
            .wait()
            .map_err(MdsfError::Io)?
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::TimedOut, "Process timed out"))
            .map(process_control::Output::into_std_lossy)
            .map_err(MdsfError::Io)
    }
}

#[inline]
pub fn execute_command(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
    timeout: u64,
    is_stdin: bool,
    debug_enabled: bool,
) -> Result<(bool, Option<String>), MdsfError> {
    if cmd.get_current_dir().is_none() {
        let binary_name = cmd.get_program();

        if !binary_in_path(binary_name) {
            return Err(MdsfError::MissingBinary(
                binary_name.to_string_lossy().to_string(),
            ));
        }
    }

    let output = spawn_command(cmd, timeout, is_stdin, snippet_path, debug_enabled);

    handle_post_execution(output, snippet_path, is_stdin)
}

#[inline]
pub fn format_snippet(
    config: &MdsfConfig,
    info: &LineInfo,
    code: &str,
    timeout: u64,
    debug_enabled: bool,
) -> String {
    let always_ran = config.languages.get("*");

    let language_formatters = config.languages.get(info.language).or_else(|| {
        if always_ran.is_none() {
            config.languages.get("_")
        } else {
            None
        }
    });

    if always_ran.is_some() || language_formatters.is_some() {
        let ext = config
            .custom_file_extensions
            .get(info.language)
            .map_or_else(
                || get_file_extension(info.language),
                std::borrow::ToOwned::to_owned,
            );

        if let Ok(snippet) = setup_snippet(code, &ext) {
            let snippet_path = snippet.path();

            if let Some(formatters) = always_ran {
                if let Ok(Some(formatted_code)) = formatters.format(
                    snippet_path,
                    info,
                    timeout,
                    debug_enabled,
                    config.javascript_runtime,
                ) {
                    if language_formatters.is_none() {
                        let mut f = formatted_code.trim().to_owned();

                        f.push('\n');

                        return f;
                    }
                }
            }

            if let Some(formatters) = language_formatters {
                if let Ok(Some(formatted_code)) = formatters.format(
                    snippet_path,
                    info,
                    timeout,
                    debug_enabled,
                    config.javascript_runtime,
                ) {
                    let mut f = formatted_code.trim().to_owned();

                    f.push('\n');

                    return f;
                }
            }
        }
    }

    code.to_owned()
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum MdsfFormatter<T>
where
    T: core::fmt::Display,
{
    Single(T),
    Multiple(Vec<MdsfFormatter<T>>),
}

#[inline]
pub fn binary_in_path(binary_name: &OsStr) -> bool {
    which(binary_name).is_ok()
}

impl core::fmt::Display for Tooling {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Default for MdsfFormatter<Tooling> {
    fn default() -> Self {
        Self::Multiple(Vec::new())
    }
}

impl MdsfFormatter<Tooling> {
    #[inline]
    pub fn format(
        &self,
        snippet_path: &std::path::Path,
        info: &LineInfo,
        timeout: u64,
        debug_enabled: bool,
        javascript_runtime: JavaScriptRuntime,
    ) -> Result<Option<String>, MdsfError> {
        Self::format_multiple(
            self,
            snippet_path,
            info,
            false,
            timeout,
            debug_enabled,
            javascript_runtime,
        )
        .map(|(_should_continue, output)| output)
    }

    #[inline]
    pub fn format_multiple(
        formatter: &Self,
        snippet_path: &std::path::Path,
        info: &LineInfo,
        nested: bool,
        timeout: u64,
        debug_enabled: bool,
        javascript_runtime: JavaScriptRuntime,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match formatter {
            Self::Single(f) => {
                let formatter_name: &str = f.as_ref();

                print_formatter_info(formatter_name, info);

                let time = std::time::Instant::now();

                let r = f.format_snippet(snippet_path, timeout, debug_enabled, javascript_runtime);

                print_formatter_time(formatter_name, info, time.elapsed());

                if let Err(e) = &r {
                    if let MdsfError::MissingBinary(binary) = e {
                        print_binary_not_in_path(
                            info.filename,
                            if formatter_name == binary {
                                formatter_name.to_string()
                            } else {
                                format!("{binary} ({formatter_name})")
                            }
                            .as_str(),
                        );

                        return Ok((false, None));
                    } else if let MdsfError::FormatterError(stderr) = e {
                        print_error_formatting(formatter_name, info, stderr);

                        return Ok((false, None));
                    }
                }

                r
            }

            Self::Multiple(formatters) => {
                let mut r = Ok((true, None));

                for f in formatters {
                    r = Self::format_multiple(
                        f,
                        snippet_path,
                        info,
                        true,
                        timeout,
                        debug_enabled,
                        javascript_runtime,
                    );

                    if r.as_ref()
                        .is_ok_and(|(should_continue, _code)| !should_continue)
                        && nested
                    {
                        break;
                    }
                }

                r
            }
        }
    }
}

#[inline]
pub fn run_tools(
    command_types: &[CommandType],
    file_path: &std::path::Path,
    set_args_fn: fn(std::process::Command, &std::path::Path) -> std::process::Command,
    timeout: u64,
    is_stdin: bool,
    debug_enabled: bool,
    javascript_runtime: JavaScriptRuntime,
) -> Result<(bool, Option<String>), MdsfError> {
    for (index, cmd) in command_types.iter().enumerate() {
        let cmd = set_args_fn(cmd.build(javascript_runtime), file_path);

        let execution_result = execute_command(cmd, file_path, timeout, is_stdin, debug_enabled);

        if index == command_types.len() - 1 {
            return execution_result;
        }

        if let Ok(r) = execution_result {
            if !r.0 {
                return Ok(r);
            }
        }
    }

    Ok((true, None))
}
