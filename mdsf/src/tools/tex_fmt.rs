///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("tex-fmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_tex_fmt {
    #[test_with::executable(tex-fmt)]
    fn test_tex_fmt_latex_1249f3d7d4b15b30() {
        let input = r#"\documentclass{article}

\begin{document}

\begin{itemize}
\item Lists with items
over multiple lines
\end{itemize}

\begin{equation}
E = m c^2
\end{equation}

\end{document}"#;

        let output = r#"\documentclass{article}

\begin{document}

\begin{itemize}
  \item Lists with items
    over multiple lines
\end{itemize}

\begin{equation}
  E = m c^2
\end{equation}

\end{document}
"#;

        let file_ext = crate::fttype::get_file_extension("latex");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::TexFmt
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}
