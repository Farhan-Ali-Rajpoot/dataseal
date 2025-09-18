use std::io::{self, Write};
use std::env::{current_dir};
use std::path::PathBuf;
use crate::db::Database;
use colored::*;

use super::{
    help_document::{help_document, unknown_command_message},
    commands::{fs_commands, pass_commands, file_commands, auth_commands},
    validate_args::validate_args,
};

pub fn start(master_password: &str) {
    let mut db: Database = match Database::new(master_password) {
        Some(d) => d,
        None => {
            println!("Failed to initialize database. Exiting...");
            return;
        }
    };
    // Clear Screen
    fs_commands::clear();

    // db.add_file("nexa-env", "../Github/Nexa/.env");
    let mut current_directory: PathBuf = current_dir().unwrap();
    let initial_dir = current_directory.clone();
    println!("DataSeal CLI ready. Type 'help' for commands.");

    loop {
        print!(
            "{}{}{}",
            "ds:".bright_green(),
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
                // Auth 
                "change-root-password" | "chgrootpass" | "crp" => {
                    if validate_args(&["change-root-password","chgrootpass","crp"], &parts, 2) {
                        if auth_commands::change_root_password(&mut db, &parts) {
                            fs_commands::clear();
                            println!("✅ Master password changed.");
                        }
                    }
                }
                // Recycle Bin
                "empty-recycle-bin" | "emp-rec-bin" | "erb" => {
                    if validate_args(&["empty-recycle-bin","emp-rec-bin","erb"], &parts, 0) {
                        file_commands::empty_recycle_bin_verbose(&mut db);
                    }
                },
                // File Commands
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
                    if validate_args(&["restore-file","resfile", "rf"], &parts, 1) {
                        file_commands::restore_file(&mut db, &parts);
                    }
                },
                "delete-all-files" | "delallfiles" | "daf" => {
                    if validate_args(&["delete-all-files","delallfiles","daf"], &parts, 0) {
                        file_commands::delete_all_files(&mut db);
                    }
                },
                "delete-file" | "delfile" | "df" => {
                    if validate_args(&["delete-file","delfile", "df"], &parts, 1) {
                        file_commands::delete_file(&mut db, &parts);
                    }
                },
                "decrypt-file" | "decfile" | "decf"=> {
                    if validate_args(&["decrypt-file","decfile", "decf"], &parts, 1) {
                        file_commands::decrypt_file(&mut db, &parts, &initial_dir);
                    }
                },
                "encrypt-file" | "encfile" | "encf" => {
                    if validate_args(&["encrypt-file","encfile", "encf"], &parts, 1) {
                        file_commands::encrypt_file(&mut db, &parts, &initial_dir);
                    }
                },
                "add-file" | "addfile" | "af" => {
                    if validate_args(&["add-file","addfile", "af"], &parts, 2) {
                        if file_commands::add_file(&mut db, &parts, &current_directory, &initial_dir) {
                            if let Ok(dir) = std::env::current_dir() {
                                current_directory = dir;
                            }
                        }
                    }
                },
                // Password Commands
                "restore-all-password" | "resallpass" | "rap" => {
                    if validate_args(&["restore-all-password","resallpass","rap"], &parts, 0) {
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
                    if validate_args(&["restore-password", "respass", "rp"], &parts, 1) {
                        pass_commands::restore_password(&mut db, &parts);
                    }
                },
                "delete-all-passwords" | "delallpass" | "dap" => {
                    if validate_args(&["delete-all-passwords", "delallpass", "dap"], &parts, 0) {
                        pass_commands::delete_all_passwords(&mut db);
                    }
                },
                "delete-password" | "delpass" | "dp" => {
                    if validate_args(&["delete-password", "delpass", "dp"], &parts, 1) {
                        pass_commands::delete_password(&mut db, &parts);
                    }
                },
                "decrypt-password" | "decpass" | "decp" => {
                    if validate_args(&["decrypted-password","decpass", "decp"], &parts, 1) {
                        pass_commands::decrypt_password(&mut db, &parts);
                    }
                },
                "encrypt-password" | "encpass" | "encp"=> {
                    if validate_args(&["encrypt-password","encpass", "encp"], &parts, 1) {
                        pass_commands::encrypt_password(&mut db, &parts);
                    }
                },
                "change-password" | "chgpass" | "cp" => {
                    if validate_args(&["change-password", "chgpass", "cp"], &parts, 2) {
                        pass_commands::change_password(&mut db, &parts);
                    }
                },
                "add-password" | "addpass" | "ap" => {
                    if validate_args(&["addpass", "add-password", "ap"], &parts, 2) {
                        pass_commands::add_password(&mut db, &parts);
                    }
                },
                // Default Commands
                "help" => {
                    if validate_args(&["help"], &parts, 0) {
                        println!("{}", help_document());
                    }
                },
                "ls" => {
                    if validate_args(&["ls"], &parts, 0) {
                        println!("{}", fs_commands::ls());
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
