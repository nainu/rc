use clap::{App, ArgMatches};

#[derive(Debug)]
pub struct Arguments {
    pub col_idx: u8,
    pub ignore_head_line: u8,
    pub join: bool,
    pub verbose: bool,
}

impl Arguments {
    pub fn from_cmdline() -> Arguments {
        let matches = Self::get_app().get_matches();
        Self::from(matches)
    }

    fn from(matches: ArgMatches) -> Arguments {
        Arguments {
            col_idx: matches
                .value_of("column-index")
                .map(Self::parse_str_to_u8)
                .unwrap_or(0),
            ignore_head_line: matches
                .value_of("i")
                .map(Self::parse_str_to_u8)
                .unwrap_or(0),
            join: matches.is_present("join"),
            verbose: matches.is_present("verbose"),
        }
    }

    fn parse_str_to_u8(x: &str) -> u8 {
        x.parse::<u8>().unwrap()
    }

    fn get_app<'a>() -> App<'a, 'a> {
        let usage = "
        -c, --column-index=[col_idx] 'Index number of columns'
        -i, [n] 'Number of head lines to ignore'
        -j, --join
        -v, --verbose, 'Prints verbosely'";
        App::new("rc.rs").version("0.1").args_from_usage(usage)
    }
}
