use clap:: {
    Parser,
    ValueEnum,
};

#[derive(Parser, Debug)]
#[command(version, about)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct WeatherArgs {
    // #[command(subcommand)]
    // pub format: LocationFormat,
    /// Location of the forecast
    pub location: String,
    /// Duration of the forecast
    #[arg(short, long, default_value = "now")]
    pub duration: Option<DurationType>, 

    /// Output format of the forecast
    #[arg(short, long, default_value = "compact")]
    pub output_mode: Option<OutputMode>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum DurationType {
    Now,
    Today,
    Tomorrow,
    Week,
}

#[derive(Debug)]
pub enum LocationInfo {
    Amenity(String),
    Street(String),
    City(String),
    Country(String),
    State(String),
    PostalCode(String),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutputMode {
    Compact,
    Detailed,
    Complete,
}


