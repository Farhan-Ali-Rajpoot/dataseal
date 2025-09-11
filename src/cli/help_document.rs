

pub fn help_document() -> String {
    let doc = r#"
        ==========================
                DataSeal CLI
        ==========================

        A secure command-line vault for managing encrypted files and credentials.

        USAGE:
            dataseal [OPTIONS]

        OPTIONS:
            -h, --help        Show this help message
            -c, --cli         Launch CLI interface
            -g, --gui         Launch GUI interface (default)

        COMMAND CATEGORIES:

          ┌───────────────────────┐
          │  SYSTEM COMMANDS      │
          └───────────────────────┘
            help              Show this help document
            exit, quit        Exit the CLI
            clear             Clear the screen

          ┌───────────────────────┐
          │  PATH COMMANDS        │
          └───────────────────────┘
            pwd               Print current working directory
            ls                List files and folders in current directory
            cd <dir>           Change current directory

          ┌───────────────────────┐
          │  FILE COMMANDS        │
          └───────────────────────┘
            add_file <name>     Encrypt and store a file in the database
            list_files           Show all stored files
            extract_file <name>  Decrypt and restore a stored file

          ┌────────────────────────────┐
          │  PASSWORD VAULT COMMANDS   │
          └────────────────────────────┘
            add_pass <label>     Store a new password entry
            list_pass            Show all saved password entries
            remove_pass <label>  Delete a stored password entry

        NOTES:
            Use commands exactly as shown.

    "#;

    doc.to_string()
}

pub fn unknown_command_message(cmd: &str) -> String {
    format!(
        "Unknown command: {}\n\tTry 'help' for a list of available commands.",
        cmd
    )
}