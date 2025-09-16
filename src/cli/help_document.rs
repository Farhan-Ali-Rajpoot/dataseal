

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
            cd <dir>          Change current directory

          ┌───────────────────────┐
          │  FILE COMMANDS        │
          └───────────────────────┘
            add_file <name> <filename>      Store file in database, move to that directory where it exists, use cd <path> command. <name> is the name by which file 
                                            will be stored in database ( don't forget it ). <filenmae> is the name by which it exists in directory or disk.
            list_files                      Show all stored files
            encrypt_file <name>             Decrypt encrypted file stored on database. <name> is the name by which file was stored on database.

          ┌────────────────────────────┐
          │  PASSWORD VAULT COMMANDS   │
          └────────────────────────────┘
            add_pass <name> <password>      Store a new password or string entry
            list_pass                       Show all saved password or strings entries
            remove_pass <name>              Delete a stored password or special string entry

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