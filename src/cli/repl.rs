use super::{
    structs::{Database,DatabaseArguments},
    help_document::{help_document, unknown_command_message},
    commands::{fs_commands, pass_commands, file_commands, auth_commands, system_commands, nested_db, folder_commands},
    validate_args::{validate_args,print_usage},
    colored::*,
    std::{
        path::{PathBuf},
        env::{current_dir},
        io::{self, Write},
    },
};

pub fn start(args: &mut DatabaseArguments) {
    // Clear Screen
    fs_commands::clear();

    let mut db: Database = match Database::with_dir(args) {
        Some(d) => d,
        None => {
            println!("Failed to initialize database. Exiting...");
            return;
        }
    };

    let mut current_db_location = db.config.db_info.name.clone();
    
    let mut current_directory: PathBuf = current_dir().unwrap();
    let initial_dir = current_directory.clone();
    println!("DataSeal CLI ready. Type 'help' for commands.");

    loop {
        print!(
            "{}: {}{}",
            current_db_location.bright_green(),
            current_directory.display().to_string().blue(),
            ">".bright_green()
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        let commands: Vec<&str> = input.split("&&").map(|s| s.trim()).collect();

        for cmd in commands {
            if cmd.is_empty() {
                continue;
            }

            let parts: Vec<&str> = cmd.split_whitespace().collect();

            match parts[0] {
                // Folder
                "search-directories" | "searchdir" | "sdir" => {
                    if validate_args(&["search-directories","searchdir","sdir"], &parts, 1) {
                        folder_commands::search_folders(&db, &parts);
                    }
                },
                "search-deleted-directories" | "searchdeldir" | "sddir" => {
                    if validate_args(&["search-deleted-directories","searchdeldir","sddir"], &parts, 1) {
                        folder_commands::search_deleted_folders(&db, &parts);
                    }
                },
                "search-encrypted-directories" | "searchencdir" | "sencdir" => {
                    if validate_args(&["search-encrypted-directories","searchencdir","sencdir"], &parts, 1) {
                        folder_commands::search_encrypted_folders(&db, &parts);
                    }
                },
                "search-decrypted-directories" | "searchdecdir" | "sdecdir" => {
                    if validate_args(&["search-decrypted-directories","searchdecdir","sdecdir"], &parts, 1) {
                        folder_commands::search_decrypted_folders(&db, &parts);
                    }
                },
                "list-deleted-directories" | "lsdeldir" | "lddir" => {
                    if validate_args(&["list-deleted_directories","lsdeldir","lddir"], &parts, 0) {
                        folder_commands::list_deleted_folders(&db);
                    }
                },
                "list-directories" | "lsdir" | "ldir" => {
                    if validate_args(&["list-directories","lsdir","ldir"], &parts, 0) {
                        folder_commands::list_folders(&db);
                    }
                },
                "list-encrypted-directories" | "lsencdir" | "lencdir" => {
                    if validate_args(&["list-encrypted-directories","lsencdir","lencdir"], &parts, 0) {
                        folder_commands::list_encrypted_folders(&db);
                    }
                },
                "list-decrypted-directories" | "lsdecdir" | "ldecdir" => {
                    if validate_args(&["list-decrypted-directories","lsdecdir","ldecdir"], &parts, 0) {
                        folder_commands::list_decrypted_folders(&db);
                    }
                },
                "delete-all-directories" | "dadir" => {
                    if validate_args(&["delete-all-directory" , "dadir"], &parts, 0) {
                        let force_flag = parts.iter().any(|&p| p == "-f" || p == "--force");
                        if force_flag {
                            db.delete_all_folders_with_progress(true);
                        }else {
                            db.delete_all_folders_with_progress(false);
                        }
                    }
                },
                "delete-directory" | "ddir" => {
                    if validate_args(&["delete-directory" , "ddir"], &parts, 1) {
                        let force_flag = parts.iter().any(|&p| p == "-f" || p == "--force");
                        if !force_flag {
                            folder_commands::delete_folder(&mut db, &parts);
                        }else {
                            folder_commands::delete_folder_force(&mut db, &parts);
                        }
                    }
                },
                "create-directory" | "cdir" => {
                    if parts.len() == 1 {
                        println!("\n{}", "Invalid usage!".red().bold());
                        println!("{}", "Here’s how to use this command:".bright_black());
                        println!("{}", "──────────────────────────────".bright_black());
                        print_usage(&[
                            "create-directory <directory-name>",
                            "cdir <directory-name>"
                        ]);
                    }else {
                        folder_commands::create_folder(&mut db, &parts);
                    }
                },
                // Nested DB
                "logout-nested-database" | "logoutnesteddb" | "logondb" => {
                    if validate_args(&["logout-nested-database","logoutnesteddb","logondb"], &parts, 0) {
                        if !db.config.is_nested {
                            println!("Already in root database. (root)");
                            continue;
                        }
                        nested_db::logout_nested_database(&mut db, &args, &initial_dir, &mut current_db_location);
                    }
                },
                "login-nested-database" | "loginnesteddb" | "logndb" => {
                    if validate_args(&["login-nested-database","loginnesteddb","logndb"], &parts, 1) {
                        if db.config.is_nested {
                            println!("Already in nested database. (nested)")
                        }
                        nested_db::login_nested_database(&mut db, &parts, &initial_dir, &mut current_db_location);
                    }
                },
                "delete-nested-database" | "delnesteddb" | "dndb" => {
                    if validate_args(&["delete-nested-database","delnesteddb","dndb"], &parts, 2) {
                        nested_db::delete_nested_database(&mut db, &parts);
                    }
                },
                "list-nested-databases" | "listnesteddbs" | "lndb" => {
                    if validate_args(&["list-nested-databases","listnesteddbs","lndb"], &parts, 0) {
                        nested_db::list_nested_databases(&db, &parts);
                    }
                },
                "create-nested-database" | "createnesteddb" | "cndb" => {
                    if validate_args(&["create-nested-database","createnesteddb","cndb"], &parts, 0) {
                        let force_flag = parts.iter().any(|&p| p == "-f" || p == "--force");

                        if db.config.is_nested && !force_flag {
                            println!("❌ Cannot create nested database inside nested database");
                            println!("💡 Use {} to force creation", "'cndb -f'".yellow());
                            continue;
                        }

                        if db.config.is_nested && force_flag {
                            println!("{}", "⚠️  Creating nested database inside nested database (forced)".yellow());
                        }

                        nested_db::create_nested_database(&mut db, &parts);
                    }
                },
                //System 
                "database-info" | "di" => {
                    if validate_args(&["storage-info","di"], &parts, 0) {
                        system_commands::show_database_info(&mut db);
                    }
                },
                // Auth 
                "change-root-password" | "chgrootpass" | "crp" => {
                    if validate_args(&["change-root-password","chgrootpass","crp"], &parts, 2) {
                        if auth_commands::change_root_password(&mut db, &parts) {
                            fs_commands::clear();
                            // println!("✅ Master password changed.");
                        }
                    }
                },
                // Recycle Bin
                "empty-recycle-bin" | "emp-rec-bin" | "erb" => {
                    if validate_args(&["empty-recycle-bin","emp-rec-bin","erb"], &parts, 0) {
                        file_commands::empty_recycle_bin_verbose(&mut db);
                    }
                },
                // File Commands
                "decrypt-all-files" | "decallfiles" | "decaf" => {
                    if validate_args(&["decrypt-all-files","decallfiles","decaf"], &parts, 0) {
                        file_commands::decrypt_all_files(&mut db, &initial_dir);
                    }
                },
                "encrypt-all-files" | "encallfiles" | "encaf" => {
                    if validate_args(&["encrypt-all-files","encallfiles","encaf"], &parts, 0) {
                        file_commands::encrypt_all_files(&mut db, &initial_dir);
                    }
                },
                "paste-file" | "pf" => {
                    if validate_args(&["paste-file", "pf"], &parts, 1) {
                        file_commands::paste_file(&mut db, &parts, &current_directory, &initial_dir);
                    }
                },
                "cut-paste-file" | "cutpastefile" | "cpf" => {
                    if validate_args(&["cut-paste-file","cutpastefile","cpf"], &parts, 1) {
                        file_commands::cut_paste_file(&mut db, &parts, &current_directory, &initial_dir);
                    }
                },
                "restore-all-files" | "resallfiles" | "raf" => {
                    if validate_args(&["restore-all-files","resallfiles","raf"], &parts, 0) {
                        file_commands::restore_all_files(&mut db);
                    }
                },
                "search-deleted-files" | "searchdelfiles" | "sdf" => {
                    if validate_args(&["search-deleted-files","searchdelfiles","sdf"], &parts, 1) {
                        file_commands::search_deleted_files(&mut db, &parts);
                    }
                },
                "search-decrypted-files" | "searchdecfiles" | "sdecf" => {
                    if validate_args(&["search-decrypted-files","searchdecfiles","sf", "sdecf"], &parts, 1) {
                        file_commands::search_decrypted_files(&mut db,&parts);
                    }
                },
                "search-files" | "sf"  => {
                    if validate_args(&["search-files","sf"], &parts, 1) {
                        file_commands::search_all_files(&mut db,&parts);
                    }
                },
                "search-encrypted-files" | "searchencfiles" | "sencf" => {
                    if validate_args(&["search-encrypted-files","searchencfiles","sencf"], &parts, 1) {
                        file_commands::search_encrypted_files(&mut db, &parts);
                    }
                },
                "list-deleted-files" | "lsdelfiles" | "ldf" => {
                    if validate_args(&["list-deleted-files","lsdelfiles","ldf"], &parts, 0) {
                        file_commands::list_deleted_files(&mut db);
                    }
                },
                "list-encrypted-files" | "lsencfiles" | "lencf" => {
                    if validate_args(&["list-encrypted-files", "lsencfiles", "lencf"], &parts, 0) {
                        file_commands::list_encrypted_files(&mut db);
                    }
                },
                "list-decrypted-files" | "lsdecfiles" | "ldecf"  => {
                    if validate_args(&["list-decrypted-files","lsdecfiles","ldf"], &parts, 0) {
                        file_commands::list_decrypted_files(&mut db);
                    }
                },
                "list-files" | "lsfiles" | "lf" => {
                    if validate_args(&["list-files","lsfiles","lf"], &parts, 0) {
                        file_commands::list_all_files(&mut db);
                    }
                },
                "restore-file" | "resfile" | "rf" => {
                    if parts.len() - 1 == 0 {
                        print_usage(&["restore-file <filename> <filename> ... ", "resfile <filename> <filename> ...", "rf <filename> <filename> ..."]);
                        return;
                    } 
                    file_commands::restore_file(&mut db, &parts);
                },
                "delete-all-files" | "delallfiles" | "daf" => {
                    if validate_args(&["delete-all-files","delallfiles","daf"], &parts, 0) {
                        file_commands::delete_all_files(&mut db);
                    }
                },
                "delete-file" | "delfile" | "df" => {
                    if parts.len() - 1 == 0 {
                        print_usage(&["delete-file <filename> <filename> ... ", "delfile <filename> <filename> ...", "df <filename> <filename> ..."]);
                        return;
                    } 
                    file_commands::delete_file(&mut db, &parts);
                },
                "decrypt-file" | "decfile" | "decf"=> {
                    if parts.len() - 1 == 0 {
                        print_usage(&["decrypt-file <filename> <filename> ... ", "decfile <filename> <filename> ...", "decf <filename> <filename> ..."]);
                        return
                    } 
                    file_commands::decrypt_file(&mut db, &parts, &initial_dir);
                },
                "encrypt-file" | "encfile" | "encf" => {
                    if parts.len() - 1 == 0 {
                        print_usage(&["encrypt-file <filename> <filename> ... ", "encfile <filename> <filename> ...", "encf <filename> <filename> ..."]);
                        return
                    } 
                    file_commands::encrypt_file(&mut db, &parts, &initial_dir);
                },
                "cut-add-file" | "cutaddfile" | "caf" => {
                    if parts.len() - 1 == 0 || parts.len() - 1 == 1 {
                        print_usage(&["cut-add-file <name> <filepath> <name> <filepath> ...","cutaddfile <name> <filepath> <name> <filepath> ...",
                        "caf <name> <filepath> <name> <filepath> ..."])
                    }
                    if file_commands::cut_add_file(&mut db, &parts, &current_directory, &initial_dir) {
                        if let Ok(dir) = std::env::current_dir() {
                            current_directory = dir;
                        }
                    }
                },
                "add-file" | "addfile" | "af" => {
                    if parts.len() - 1 == 0 || parts.len() - 1 == 1 {
                        print_usage(&["add-file <name> <filepath> <name> <filepath> ...","addfile <name> <filepath> <name> <filepath> ...",
                        "af <name> <filepath> <name> <filepath> ..."])
                    } 
                    if file_commands::add_file(&mut db, &parts, &current_directory, &initial_dir) {
                        if let Ok(dir) = std::env::current_dir() {
                            current_directory = dir;
                        }
                    }
                },
                // Password Commands
                "decrypt-all-passwords" | "decallpass" | "decap" => {
                    if validate_args(&["decrypt-all-passwords","decallpass","decap"], &parts, 0) {
                        pass_commands::decrypt_all_passwords(&mut db);
                    }
                },
                "encrypt-all-passwords" | "encallpass" | "encap" => {
                    if validate_args(&["encrypt-all-passwords","encallpass","encap"], &parts, 0) {
                        pass_commands::encrypt_all_passwords(&mut db);
                    }
                },
                "restore-all-passwords" | "resallpass" | "rap" => {
                    if validate_args(&["restore-all-passwords","resallpass","rap"], &parts, 0) {
                        pass_commands::restore_all_passwords(&mut db);
                    }
                }
                "search-deleted-passwords" | "searchdelpass" | "sdp" => {
                    if validate_args(&["search-deleted-passwords","searchdelpass","sdp"], &parts, 1) {
                        pass_commands::search_deleted_passwords(&mut db, &parts);
                    }
                },
                "search-decrypted-passwords" | "searchdecpass" | "sdecp" => {
                    if validate_args(&["search-decrypted-passwords","searchdecpass","sdecp"], &parts, 1) {
                        pass_commands::search_decrypted_passwords(&mut db, &parts);
                    }
                },
                "search-passwords" | "searchpass" | "sp" => {
                    if validate_args(&["search-passwords","searchpass","sp"], &parts, 1) {
                        pass_commands::search_all_passwords(&mut db, &parts);
                    }
                },
                "search-encrypted-passwords" | "searchencpass" | "sencp" => {
                    if validate_args(&["search-encrypted-passwords","searchencpass","sencp"], &parts, 1) {
                        pass_commands::search_encrypted_passwords(&mut db, &parts);
                    }
                },
                "list-deleted-passwords" | "lsdelpass" | "ldp" => {
                    if validate_args(&["list-deleted-passwords","listdelpass","ldp"], &parts, 0) {
                        pass_commands::list_deleted_passwords(&mut db);
                    }
                },
                "list-encrypted-passwords" | "lsencpass" | "lencp" => {
                    if validate_args(&["list-encrypted-passwords","listencpass","lencp"], &parts, 0) {
                        pass_commands::list_encrypted_passwords(&mut db);
                    }
                },
                "list-decrypted-passwords" | "lsdecpass" | "ldecp" => {
                    if validate_args(&["list-decrypted-passwords","list-passwords","lspass","lp","ldecp", "lsdecpass"], &parts, 0) {
                        pass_commands::list_decrypted_passwords(&mut db);
                    }
                },
                "list-passwords" | "lspass" | "lp"  => {
                    if validate_args(&["list-decrypted-passwords","list-passwords","lspass","lp","ldecp", "lsdecpass"], &parts, 0) {
                        pass_commands::list_all_passwords(&mut db);
                    }
                },
                "restore-password" | "respass" | "rp" => {
                    if parts.len() - 1 == 0 {
                        print_usage(&["restore-password <name> <name> ...", "respass <name> <name> ...", "rp <name> <name> ..."]);
                        return;
                    }
                    pass_commands::restore_password(&mut db, &parts);
                },
                "delete-all-passwords" | "delallpass" | "dap" => {
                    if validate_args(&["delete-all-passwords", "delallpass", "dap"], &parts, 0) {
                        pass_commands::delete_all_passwords(&mut db);
                    }
                },
                "delete-password" | "delpass" | "dp" => {
                    if parts.len() - 1 == 0 {
                        print_usage(&["delete-password <name> <name> ...", "delpass <name> <name> ...", "dp <name> <name> ..."]);
                        return
                    }
                    pass_commands::delete_password(&mut db, &parts);
                },
                "decrypt-password" | "decpass" | "decp" => {
                    if parts.len() - 1 == 0 {
                        print_usage(&["decrypt-password <name> <name> ...", "decpass <name> <name> ...", "decp <name> <name> ..."]);
                        return;
                    }
                    pass_commands::decrypt_password(&mut db, &parts);
                },
                "encrypt-password" | "encpass" | "encp"=> {
                    if parts.len() - 1 == 0 {
                        print_usage(&["encrypt-password <name> <name> ...", "encpass <name> <name> ...", "encp <name> <name> ..."])
                    }
                    pass_commands::encrypt_password(&mut db, &parts);
                },
                "change-password" | "chgpass" | "cp" => {
                    if parts.len() == 0 || parts.len() == 1 {
                        print_usage(&["change-password <name> <newpassword> <name> <newpass> ...", "chgpass <name> <newpassword> <name> <newpass> ...",
                        "cp <name> <newpassword> <name> <newpass> ..."])
                    }
                    pass_commands::change_password(&mut db, &parts);
                },
                "add-password" | "addpass" | "ap" => {
                    if parts.len() - 1 == 0 || parts.len() - 1 == 1 {
                        print_usage(&["add-password <name> <password> <name> <password> ...", "addpass <name> <password> <name> <password> ...",
                        "ap <name> <password> <name> <password> ..."])
                    } 
                    pass_commands::add_password(&mut db, &parts);
                },
                // Default Commands
                "help" => {
                    if validate_args(&["help"], &parts, 0) {
                        println!("{}", help_document());
                    }
                },
                "ls" => {
                    if validate_args(&["ls"], &parts, 0) {
                        fs_commands::ls();
                    }
                },
                "lsa" => {
                    if validate_args(&["lsa"], &parts, 0) {
                        fs_commands::lsa();
                    }
                },
                "pwd" => {
                    if validate_args(&["pwd"], &parts, 0) {
                        println!("{}", fs_commands::pwd());
                    }
                },
                "cd" => {
                    if validate_args(&["cd"], &parts, 1) {
                        let arg = parts[1];
                        if fs_commands::cd(arg) {
                            if let Ok(dir) = std::env::current_dir() {
                                current_directory = dir;
                            }
                        }
                    }
                },
                "clear" => {
                    if validate_args(&["clear"], &parts, 0) {
                        fs_commands::clear();
                    }
                },
                "version" | "--version" | "-v" => {
                    println!("Dataseal {}", env!("CARGO_PKG_VERSION"));
                }
                "exit" | "quit" => return,
                _ => println!("{}", unknown_command_message(cmd)),
            }       
        }
    }
}



pub fn push_path(path: &mut String, name: &str) {
    if let Some(pos) = path.find('/') {
        // Already has two segments -> replace second
        if path[pos + 1..].contains('/') {
            // truncate after first segment
            if let Some(first_slash) = path.find('/') {
                path.truncate(first_slash);
            }
        }
        // truncate everything after first slash
        if let Some(first_slash) = path.find('/') {
            path.truncate(first_slash);
        }
        path.push('/');
        path.push_str(name);
    } else {
        // Only root exists -> just append
        path.push('/');
        path.push_str(name);
    }
}
pub fn pop_path(path: &mut String) {
    if let Some(pos) = path.find('/') {
        // keep only root part
        path.truncate(pos);
    }
}