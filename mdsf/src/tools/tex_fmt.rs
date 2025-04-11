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
    fn test_tex_fmt_latex_ce9dd54b3a51a461() {
        let input = r#"\documentclass{article}

egin{document}

egin{itemize}
\item Lists with items
over multiple lines
\end{itemize}

egin{equation}
E = m c^2
\end{equation}

\end{document}"#;

        let output = r#"\documentclass{article}

egin{document}

egin{itemize}
  \item Lists with items
    over multiple lines
\end{itemize}

egin{equation}
  E = m c^2
\end{equation}

\end{document}
"#;

        let file_ext = crate::fttype::get_file_extension("latex");

        crate::tools::Tooling::TexFmt.test_format_snippet(input, output, &file_ext);
    }
}
