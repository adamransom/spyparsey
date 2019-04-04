# SpyParsey

A tool for parsing and querying a collection of SpyParty replays.

**This is still very much a work-in-progress!**

## Usage

Below is the current help text for the tool:

```
USAGE:
    spyparsey.exe [FLAGS] [OPTIONS]

FLAGS:
        --count
            Outputs a count of the matched replays

    -h, --help
            Prints help information

        --show-paths
            Outputs a list of the paths of matched replays

        --sniperwin
            Filters games that end in a sniper win

        --spywin
            Filters games that end in a spy win

    -V, --version
            Prints version information

    -v
            Sets the verbosity level for logging


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

There are currently 3 modes of output. Explicit outputs are the following:

- `--count`
  
  Just show the number of replays that matched the filters.
- `--show-paths`

  Show the absolute paths of the replays that matched the filters. This can be piped into another command, maybe used to sort replays into various folders.
  
However, the default mode if you specify neither of the above is to output a few stats. What exactly is output depends on the filters (i.e. if you filter based on map, you won't get the "Maps Played" section). Below is an example of all the stats possible:

```
Total Replays:
    2326
Player Stats:
    plastikqs: 1314W 999L (56.5%)
Maps Played:
    Courtyard: 553 (23.8%)
    Ballroom: 348 (15.0%)
    Library: 263 (11.3%)
    Pub: 263 (11.3%)
    HighRise: 230 (9.9%)
    Moderne: 179 (7.7%)
    Balcony: 128 (5.5%)
    Gallery: 126 (5.4%)
    Terrace: 117 (5.0%)
    Veranda: 115 (4.9%)
Missions Completed:
    Contact Double Agent: 1197 (21.0%)
    Inspect Statues: 928 (16.3%)
    Purloin Guest List: 925 (16.3%)
    Bug Ambassador: 895 (15.7%)
    Seduce Target: 673 (11.8%)
    Swap Statue: 610 (10.7%)
    Transfer Microfilm: 266 (4.7%)
    Fingerprint Ambassador: 195 (3.4%)
Completed Mission Sets:
    Bug, BB, Inspect, Seduce: 56 (8.0%)
    BB, Inspect, Seduce, Purloin: 43 (6.2%)
    Bug, BB, Swap, Inspect: 28 (4.0%)
    BB, Swap, Inspect, Purloin: 28 (4.0%)
    Bug, BB: 27 (3.9%)
    BB, Swap, Inspect, Seduce: 26 (3.7%)
    BB, Inspect, Seduce, Fingerprint: 25 (3.6%)
    Bug, BB, Seduce: 24 (3.4%)
    Bug, BB, Inspect, Purloin: 23 (3.3%)
    Bug, BB, Swap, Seduce: 22 (3.2%)
Modes Played:
    Any: 2217 (95.3%)
    Known: 105 (4.5%)
    Pick: 4 (0.2%)
Results:
    Spy Shot: 1275 (54.8%)
    Civilian Shot: 501 (21.5%)
    Missions Win: 453 (19.5%)
    Spy Timeout: 84 (3.6%)
    Unfinished: 13 (0.6%)
```

These stats become quite useful/interesting when paired with specific filters. For example, you could find out which missions I tend to complete when I win as a spy on High-Rise a3/5:

```
$ spyparsey --spy plastikqs --map highrise --mode a3/5 --result missionswin
Total Replays:
    27
Player Stats:
    plastikqs: 27W 0L (100.0%)
Completed Mission Sets:
    Bug, BB, Transfer MF: 3 (12.0%)
    BB, Transfer MF, Seduce: 2 (8.0%)
    Swap, Inspect, Purloin: 2 (8.0%)
    BB, Inspect, Seduce: 2 (8.0%)
    BB, Seduce, Purloin: 2 (8.0%)
    Bug, BB, Purloin: 2 (8.0%)
    Transfer MF, Seduce, Purloin: 2 (8.0%)
    Inspect, Seduce, Fingerprint: 1 (4.0%)
    Bug, Swap, Inspect: 1 (4.0%)
    BB, Purloin, Fingerprint: 1 (4.0%)
```

Seems like I enjoy going for those hard tells!

## Notes

- Most of the filter options have aliases to their singular counterpart, but behave the same way i.e. `--spy` works the same as `--spies`.
- A lot of values are accepted for missions and modes e.g. `--modes a4/8` or `--completed-missions contactda`
- I haven't optimised or done much performance-wise, but it's basically limited by disk read. When running over 20,000 replays it takes about 20 seconds to run the first time (regardless of filters) and then subsequent runs take less than a second or two (again, regardless of filters changing).
