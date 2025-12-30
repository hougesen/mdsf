use crate::readme::pad_right;

pub fn generate_command_table(commands: Vec<crate::tools::GeneratedCommand>) -> String {
    let mut table: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();

    let name_header = "Name";

    let command_header = "Command";

    let mut name_row_length = name_header.len();
    let mut command_row_length = command_header.len();

    let command_count = commands.len();

    for c in commands {
        let name = format!("`{}`", c.serde_rename);

        name_row_length = name_row_length.max(name.len());

        let command = format!(
            "`{}{}{}`",
            c.binary,
            if c.args.is_empty() { "" } else { " " },
            c.args.join(" ")
        );

        command_row_length = command_row_length.max(command.len());

        let existing = table.insert(name, command);

        assert!(existing.is_none());
    }

    let mut lines = Vec::new();

    for (name, command) in table {
        lines.push(format!(
            "| {} | {} |",
            pad_right(name, name_row_length, ' '),
            pad_right(command, command_row_length, ' '),
        ));
    }

    lines.sort_unstable();

    lines.insert(
        0,
        format!(
            "| {} | {} |",
            pad_right("-".to_string(), name_row_length, '-'),
            pad_right("-".to_string(), command_row_length + 1, '-'),
        ),
    );

    lines.insert(
        0,
        format!(
            "| {} | {} |",
            pad_right(name_header.to_string(), name_row_length, ' '),
            pad_right(command_header.to_string(), command_row_length + 1, ' '),
        ),
    );

    lines.insert(0, String::new());

    lines.insert(
        0,
        format!("`mdsf` currently supports {command_count} commands. Feel free to open an issue/pull-request if your favorite tool/command is missing! 😃" ),
    );

    lines.join("\n")
}
