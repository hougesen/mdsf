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

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("bashate"),
    CommandType::Uv("bashate", "bashate"),
    CommandType::Pipx("bashate"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_bashate {
    #[test_with::executable(bashate || pipx || uv)]
    fn test_bashate_bash_1f0c485b85eb22b1() {
        let input = r#"# for loop examples

# pass
for i in $(seq 1 5); do
    echo $i
done

# pass
for (( i = 0 ; i < 5 ; i++ )); do
    echo $i
done

# fail E010
for i in $(seq 1 5); do
    echo $i
done

# fail E010
for (( i = 0 ; i < 5 ; i++ )); do
    echo $i
done

awk '{
    for (i = 1; i < 5; i++)
        print $i
}' < /dev/null
"#;

        let output = r#"# for loop examples

# pass
for i in $(seq 1 5); do
    echo $i
done

# pass
for (( i = 0 ; i < 5 ; i++ )); do
    echo $i
done

# fail E010
for i in $(seq 1 5); do
    echo $i
done

# fail E010
for (( i = 0 ; i < 5 ; i++ )); do
    echo $i
done

awk '{
    for (i = 1; i < 5; i++)
        print $i
}' < /dev/null
"#;

        let file_ext = crate::fttype::get_file_extension("bash");

        crate::tools::Tooling::Bashate.test_format_snippet(input, output, &file_ext);
    }
}
