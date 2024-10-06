# `rofi-todoist`

Add tasks to [Todoist](https://todoist.com), using [Rofi](https://github.com/davatorium/rofi) for input.

## Installation

### Cargo

`cargo install --git https://github.com/Cretezy/rofi-todoist.git --locked`

### From source

```shell
git clone https://github.com/Cretezy/rofi-todoist.git
cd rofi-todoist
cargo build --release
./target/release/rofi-todoist
```

## Usage

Set the `TODOIST_API_TOKEN` environment variable or `~/.config/todoist` to your Todoist API token. This can be found under [Settings -> Integrations -> Developer](https://app.todoist.com/app/settings/integrations/developer).

Run the application. This will present a rofi prompt to input the task data. Entering nothing, pressing escape, or clicking off of rofi will cancel the application. The prompt supports the Todoist [Quick Add](https://get.todoist.help/hc/en-us/articles/115001745265) syntax, however doesn't handle autocomplete or formatting.

Press enter to commit the task details. Once the data has been sent to Todoist, the task name will be displayed as a notification. If there was an error, details of it will be displayed instead. A 403 error means your API token is invalid.
