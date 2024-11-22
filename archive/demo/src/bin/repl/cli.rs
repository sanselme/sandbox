// SPDX-License-Identifier: GPL-3.0

use clap::Command;

// strip out usage
const PARSER_TEMPLATE: &str = "\
    {all-args}\
";

// strip out name/version
const APPLET_TEMPLATE: &str = "\
    {about-with-newline}\n\
    {usage-heading}\n    {usage}\n\
    \n\
    {all-args} {after-help}\
";

pub(crate) fn cli() -> Command {
    Command::new("repl")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("APPLET")
        .subcommand_help_heading("APPLETS")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("ping")
                .about("Get a response")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("Quit the REPL")
                .help_template(APPLET_TEMPLATE),
        )
}
