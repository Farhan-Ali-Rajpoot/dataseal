use colored::*;

/// Validate arguments for commands with aliases.
/// `cmd_names` = array of command names/aliases.
/// `parts` = user input split into command + args.
/// `expected` = number of arguments (excluding command itself)
pub fn validate_args(cmd_names: &[&str], parts: &[&str], expected: usize) -> bool {
    let input_cmd = parts[0];

    // Check if input command matches any alias
    let matched = cmd_names.iter().any(|&alias| alias == input_cmd);

    if !matched {
        println!(
            "{} {}\n{}",
            "Error:".red().bold(),
            format!("Unknown command '{}'", input_cmd).red(),
            "Try 'help' for a list of available commands.".yellow()
        );
        return false;
    }

    // Validate argument count
    if parts.len() - 1 != expected {
        println!("\n{}", "Invalid usage!".red().bold());
        println!("{}", "Here’s how to use this command:".bright_black());
        println!("{}", "──────────────────────────────".bright_black());

        match input_cmd {
            "database-info" | "di" => print_usage(&[
                "database-info",
                "di"
            ]),
            "decrypt-all-files" | "decallfiles" | "decaf" => print_usage(&[
                "decrypt-all-files",
                "decallfiles",
                "decaf"
            ]),
            "encrypt-all-files" | "encallfiles" | "encaf" => print_usage(&[
                "encrypt-all-files",
                "encallfiles",
                "encaf"
            ]),
            "decrypt-all-passwords" | "decallpass" | "decap" => print_usage(&[
                "decrypt-all-passwords",
                "decallpass",
                "decap",
            ]),
            "encrypt-all-passwords" | "encallpass" | "encap" => print_usage(&[
                "encrypt-all-passwords",
                "encallpass",
                "encap",
            ]),
            "cut-paste-file" | "cutpastefile" | "cpf" => print_usage(&[
                "cut-paste-file <filename",
                "cutpastefile <filname>",
                "cpf <filename",
            ]),
            "paste-file" | "pf" => print_usage(&[
                "paste-file <filename",
                "pf <filename",
            ]),
            "change-root-password" | "chgrootpass" | "crp" => print_usage(&[
                "change-root-password <oldpassword> <newpassword>",
                "chgrootpass <oldpassword> <newpassword>",
                "crp <oldpassword> <newpassword>",
            ]),
            "restore-all-passwords" | "resallpass" | "rap" => print_usage(&[
                "restore-all-passwords",
                "resallpass",
                "rap",
            ]),
            "empty-recycle-bin" | "emp-rec-bin" | "erb" => print_usage(&[
                "empty-recycle-bin",
                "emp-rec-bin",
                "erb",
            ]),
            "search-deleted-passwords" | "searchdelpass" | "sdp" => print_usage(&[
                "search-deleted-passwords <query>",
                "searchdelpass <query>",
                "sdp <query>",
            ]),
            "search-decrypted-passwords" | "searchdecpass" | "sdecp" => print_usage(&[
                "search-decrypted-passwords <query>",
                "searchdecpass <query>",
                "sdecp <query>",
            ]),
            "search-passwords" | "searchpass" | "sp" => print_usage(&[
                "search-passwords <query>",
                "searchpass <query>",
                "sp <query>",
            ]),
            "search-encrypted-passwords" | "searchencpass" | "sencp" => print_usage(&[
                "search-encrypted-passwords <query>",
                "searchencpass <query>",
                "sencp <query>",
            ]),
            "list-encrypted-passwords" | "lsencpass" | "lencp" => print_usage(&[
                "list-encrypted-passwords",
                "lsencpass",
                "lencp",
            ]),
            "list-deleted-passwords" | "lsdelpass" | "ldp" => print_usage(&[
                "list-deleted-passwords <query>",
                "lsdelpass <query>",
                "ldp <query>",
            ]),
            "list-decrypted-passwords" | "lsdecpass" | "ldecp" => print_usage(&[
                "list-decrypted-passwords",
                "lsdecpass",
                "ldecp",
            ]),
            "list-passwords" | "lspass" | "lp" => print_usage(&[
                "list-passwords",
                "lspass",
                "lp",
            ]),
            "restore-all-files" | "resallfiles" | "raf" => print_usage(&[
                "restore-all-files",
                "resallfiles",
                "raf",
            ]),
            "search-deleted-files" | "searchdelfiles" | "sdf" => print_usage(&[
                "search-deleted-files <query>",
                "searchdelfiles <query>",
                "sdf <query>",
            ]),
            "search-decrypted-files" | "searchdecfiles" | "sdecf" => print_usage(&[
                "search-decrypted-files <query>",
                "searchdecfiles <query>",
                "sdecf <query>",
            ]),
            "search-files" | "sf" => print_usage(&[
                "search-files <query>",
                "sf <query>",
            ]),
            "search-encrypted-files" | "searchencfiles" | "sencf" => print_usage(&[
                "search-encrypted-files <query>",
                "searchencfiles <query>",
                "sencf <query>",
            ]),
            "list-deleted-files" | "lsdelfiles" | "ldf" => print_usage(&[
                "list-deleted-files",
                "lsdelfiles",
                "ldf",
            ]),
            "list-encrypted-files" | "lsencfiles" | "lencf" => print_usage(&[
                "list-encrypted-files",
                "lsencfiles",
                "lencf",
            ]),
            "list-decrypted-files" | "lsdecfiles" | "ldecf" => print_usage(&[
                "list-decrypted-files",
                "lsdecfiles",
                "ldecf",
            ]),
            "list-files" | "lsfiles" | "lf" => print_usage(&[
                "list-files",
                "lsfiles",
                "lf",
            ]),
            // "restore-file" | "resfile" | "rf" => print_usage(&[
            //     "restore-file <name>",
            //     "resfile <name>",
            //     "rf <name>",
            // ]),
            "delete-all-files" | "delallfiles" | "daf" => print_usage(&[
                "delete-all-files",
                "delallfiles",
                "daf",
            ]),
            // "delete-file" | "delfile" | "df" => print_usage(&[
            //     "delete-file <name>",
            //     "delfile <name>",
            //     "df <name>",
            // ]),
            // "decrypt-file" | "decfile" | "decf" => print_usage(&[
            //     "decrypt-file <filename>",
            //     "decfile <filename>",
            //     "decf <filename>",
            // ]),
            // "encrypt-file" | "encfile" | "encf" => print_usage(&[
            //     "encrypt-file <filename>",
            //     "encfile <filename>",
            //     "encf <filename>",
            // ]),
            // "cut-add-file" | "cutaddfile" | "caf" => print_usage(&[
            //     "cut-add-file <name> <filename>",
            //     "cutaddfile <name> <filename>",
            //     "caf <name> <filename>",
            // ]),
            // "add-file" | "addfile" | "af" => print_usage(&[
            //     "add-file <name> <filename>",
            //     "addfile <name> <filename>",
            //     "af <name> <filename>",
            // ]),
            // "restore-password" | "respass" | "rp" => print_usage(&[
            //     "restore-password <name>",
            //     "respass <name>",
            //     "rp <name>",
            // ]),
            "delete-all-passwords" | "delallpass" | "dap" => print_usage(&[
                "delete-all-passwords",
                "delallpass",
                "dap",
            ]),
            // "delete-password" | "delpass" | "dp" => print_usage(&[
            //     "delete-password <name>",
            //     "delpass <name>",
            //     "dp <name>",
            // ]),
            // "decrypt-password" | "decpass" | "decp" => print_usage(&[
            //     "decrypt-password <name>",
            //     "decpass <name>",
            //     "decp <name>",
            // ]),
            // "encrypt-password" | "encpass" | "encp" => print_usage(&[
            //     "encrypt-password <name>",
            //     "encpass <name>",
            //     "encp <name>",
            // ]),
            // "change-password" | "chgpass" | "cp" => print_usage(&[
            //     "change-password <name> <newpassword>",
            //     "chgpass <name> <newpassword>",
            //     "cp <name> <newpassword>",
            // ]),
            // "add-password" | "addpass" | "ap" => print_usage(&[
            //     "add-password <name> <password>",
            //     "addpass <name> <password>",
            //     "ap <name> <password>",
            // ]),
            "cd" => print_usage(&["cd <path>"]),
            "ls" => print_usage(&["ls"]),
            "lsa" => print_usage(&["lsa"]),
            "pwd" => print_usage(&["pwd"]),
            "help" => print_usage(&["help"]),
            "clear" => print_usage(&["clear"]),
            _ => println!("{} {}", "Usage:".cyan(), input_cmd),
        }

        println!("{}", "──────────────────────────────".bright_black());
        return false;
    }

    true
}

/// Helper to print usage aliases nicely
pub fn print_usage(variants: &[&str]) {
    for (i, v) in variants.iter().enumerate() {
        if i > 0 {
            println!("   {} {}", "or".yellow(), v.cyan());
        } else {
            println!("   {}", v.cyan());
        }
    }
}
