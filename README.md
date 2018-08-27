# SpyParsey

A tool for parsing and querying a collection of SpyParty replays.

**This is still very much a work-in-progress!**

## Usage

Below is the current help text for the tool:

```
USAGE:
    spyparsey.exe [FLAGS] [OPTIONS]

FLAGS:
    -h, --help
            Prints help information

        --sniperwin
            Filters games that end in a sniper win

        --spywin
            Filters games that end in a spy win

    -V, --version
            Prints version information


OPTIONS:
        --completed-missions <MISSIONS>...
            Filters games that contain specific missions, ANY of which were completed

        --completed-missions-all <MISSIONS>...
            Filters games that contain specific missions, ALL of which were completed

        --maps <MAPS>...
            Filters based maps

        --modes <MODES>...
            Filters games that are a specific game mode

        --pair <NAMES>...
            Filters based on a pair of players who have played together

        --paths <PATHS>...
            Sets the list of paths to look for replays (can be directories or single replays)

        --players <NAMES>...
            Filters based on players' names (either spy or sniper). This uses OR matching, so if you use multiple player
            names it finds games with ANY of the players, not ALL of the players. If you want find the games where two
            players have played together, use the --pair option.
        --results <RESULTS>...
            Filters based on the result of the game [possible values: missionswin, spyshot, civilianshot, timeout,
            unfinished]
        --snipers <NAMES>...
            Filters based on snipers' names

        --spies <NAMES>...
            Filters based on spies' names
```

## Examples

- Find all replays from a particular player:
  
  `spyparsey --player plastikqs`
- Find all replays where a player shot a civilian on Ballroom or High-Rise:

  `spyparsey --sniper plastikqs --map ballroom highrise --result civilianshot`
- Find all replays where that were an "Any x/y" mode, where both bug and contact DA were completed:

  `spyparsey --mode any --completed-missions-all bug bb`
  
- Find out how many times you beat Dowsey on Balcony with bug, BB and seduce (hint, its 0):

  `spyparsey --spy plastikqs --sniper dowsey --map balcony --result missionswin --completed-missions-all bug bb seduce`

## Output

The current output is just a count of how many replays match the query, but many output formats will be added such as:

- `--count`
- `--summary`
- `--completed-missions`

## Notes

- Most of the filter options have aliases to their singular counterpart, but behave the same way i.e. `--spy` works the same as `--spies`.
- A lot of values are accepted for missions and modes e.g. `--modes a4/8` or `--completed-missions contactda`
