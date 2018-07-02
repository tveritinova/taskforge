// Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use super::error::{Error, ErrorKind, Result};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use config::Config;
use taskhero::tasks::Task;

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("new")
        .about("Create a new todo")
        .alias("n")
        .arg(
            Arg::with_name("body")
            .short("b")
            .help(
                "Body of the task, use this for storing a long form text explanation of a task.")
            .takes_value(true)
            .value_name("BODY"),
            )
        .arg(
            Arg::with_name("context")
            .short("c")
            .help(
                "Task context, used for keeping unrelated tasks separate. Common examples are 'work', 'home', etc.")
            .default_value("default")
            .takes_value(true)
            .value_name("CONTEXT"),
            )
        .arg(
            Arg::with_name("priority")
            .short("p")
            .takes_value(true)
            .value_name("PRIORITY")
            .validator(|s| s.parse::<i64>().map(|_| ()).map_err(|e| format!("{}", e))),
            )
        .arg(
            Arg::with_name("title")
            .multiple(true)
            .help("Title of the task")
            )
        .setting(AppSettings::TrailingVarArg)
}

pub fn new(config: &mut Config, args: &ArgMatches) -> Result<()> {
    let title = match args.values_of("title") {
        Some(words) => words
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" "),
        None => {
            return Err(Error::new(
                ErrorKind::InvalidArg("not enough values for title".to_string()),
                "Invalid arguments",
            ))
        }
    };

    let mut task = Task::new(&title);

    if let Some(context) = args.value_of("context") {
        task = task.with_context(context);
    }

    if let Some(priority) = args.value_of("priority") {
        // Validator already validates that parse will work so we can safely unwrap here.
        let p_int = priority.parse::<i64>().unwrap();
        task = task.with_priority(p_int);
    }

    if let Some(body) = args.value_of("body") {
        task = task.with_body(body);
    }

    config.state.add(task);

    config.save().map_err(|e| Error::from(e))
}
