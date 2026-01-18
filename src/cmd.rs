use std::env;

#[derive(Debug)]
pub enum Command {
    Init,
    Add { key: String },
    Get { key: String },
    List,
    Remove { key: String },
    ChangePassword,
}

pub fn parse_command() -> Result<Command, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(help_text());
    }

    match args[1].as_str() {
        "init" => Ok(Command::Init),

        "add" if args.len() == 3 => Ok(Command::Add {
            key: args[2].clone(),
        }),

        "get" if args.len() == 3 => Ok(Command::Get {
            key: args[2].clone(),
        }),

        "list" => Ok(Command::List),

        "remove" if args.len() == 3 => Ok(Command::Remove {
            key: args[2].clone(),
        }),

        "change-password" => Ok(Command::ChangePassword),

        "help" | "--help" | "-h" => Err(help_text()),

        _ => Err(help_text()),
    }
}

fn help_text() -> String {
    [
        "Usage:",
        "  vault init",
        "  vault add <key>",
        "  vault get <key>",
        "  vault list",
        "  vault remove <key>",
        "  vault change-password",
    ]
    .join("\n")
}
