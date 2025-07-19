use zed_extension_api as zed;

mod commands;

struct LeetCodeExtension;

impl zed::Extension for LeetCodeExtension {
    fn new() -> Self {
        LeetCodeExtension
    }

    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        match command.name.as_str() {
            "leetcode-login" => commands::handle_login(args),
            "leetcode-list" => commands::handle_list(args),
            "leetcode-show" => commands::handle_show(args),
            "leetcode-test" => commands::handle_test(args, worktree),
            "leetcode-submit" => commands::handle_submit(args, worktree),
            _ => Err(format!("Unknown command: {}", command.name)),
        }
    }
}

zed::register_extension!(LeetCodeExtension);
