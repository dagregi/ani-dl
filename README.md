# ani-dl

This CLI application, built with Rust and the Clap library, allows users to crawl and scrape animethemes.moe to download the openings and endings of anime. The downloaded files are saved on the user's computer for offline viewing.

## Features

* Crawl and scrape animethemes.moe
* Search for your favourite anime themes
* Download and save openings and endings of anime

## Installation

1. Clone the repository
2. Build the project using Cargo
3. Run the executable file

## Usage

```
ani-dl --search "THEME"
# select the theme to download/multi-selection is available

ani-dl --all --search "THEME"
# to download all results
```

## Todo

* Use fzf for selection
* Add the option to download videos

## Support

For any issues or feedback, please open an issue on GitHub.

Happy downloading!
