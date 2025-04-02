use termimad::inline as md;

struct HelpMessage {
    all: &'static str,
    general: &'static str,
    install: &'static str,
    uninstall: &'static str,
    update: &'static str,
    search: &'static str,
    detail: &'static str,
    system: &'static str,
}

impl HelpMessage {
    const fn new() -> Self {
        Self {
            all: include_str!("help/all.md"),
            general: include_str!("help/general.txt"),
            install: include_str!("help/install.txt"),
            uninstall: include_str!("help/uninstall.txt"),
            update: include_str!("help/update.txt"),
            search: include_str!("help/search.txt"),
            detail: include_str!("help/detail.txt"),
            system: include_str!("help/system.txt")
        }
    }
}

pub fn show_help_msg(help_type: &str) {
    const HELP_MESSAGES: HelpMessage = HelpMessage::new();
    match help_type {
        "" => show_help(HELP_MESSAGES.general),
        "system" => show_help(HELP_MESSAGES.system),
        "all" => show_help(HELP_MESSAGES.all),
        "install" => show_help(HELP_MESSAGES.install),
        "uninstall" => show_help(HELP_MESSAGES.uninstall),
        "update" => show_help(HELP_MESSAGES.update),
        "search" => show_help(HELP_MESSAGES.search),
        "detail" => show_help(HELP_MESSAGES.detail),
        _ => println!("Help type '{}' not found.", help_type),
    }
}
fn show_help(text_data: &str) {
    const COMMAND_NAME: &'static str = env!("CARGO_PKG_NAME");
    let text = text_data.replace("{command_name}", COMMAND_NAME);
    let text = format!("{}",md(&text));
    println!("{}", text);
}
