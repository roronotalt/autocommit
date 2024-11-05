use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
    SlashCommandOutputSection,
};

struct autocommit;

impl zed::Extension for autocommit {
    fn new() -> Self {
        autocommit
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "autocommit" => Ok(vec![]),
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }

    fn run_slash_command(
        &self,
        command: zed_extension_api::SlashCommand,
        args: Vec<String>,
        worktree: Option<&zed_extension_api::Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "autocommit" => {
                if args.is_empty() {
                    return Err("nothing to echo".to_string());
                }

                let text = args.join(" ");

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "Echo".to_string(),
                    }],
                    text,
                })
            }
            command => Err(format!("unknown command {}", command)),
        }
    }
}

zed::register_extension!(autocommit);
