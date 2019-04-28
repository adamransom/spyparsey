# SpyParsey

A tool for parsing and querying a collection of SpyParty replays.

## Installation

- [Download the latest version](https://github.com/adamransom/spyparsey/releases/tag/v1.0)
- Put it anywhere you like
- Run it from the command line

## Usage

Below is the current help text for the tool:

```
USAGE:
    spyparsey.exe [FLAGS] [OPTIONS]

FLAGS:
        --count
            Outputs a count of the matched replays

        --countdown
            Filters games that end with the 10 second mission win countdown

        --csv
            Outputs matched replays in a verbose CSV format

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

### All Possible Filter Values
<details>
  <summary>--maps</summary>
  <ul>
      <li><code>aquarium</code></li>
      <li><code>balcony</code></li>
      <li><code>ballroom</code></li>
      <li><code>courtyard</code></li>
      <li><code>"crowded pub"</code> or <code>crowdedpub</code></li>
      <li><code>"double modern"</code> or <code>doublemodern</code></li>
      <li><code>gallery</code></li>
      <li><code>"high rise"</code>, <code>high-rise</code> or <code>highrise</code></li>
      <li><code>modern</code></li>
      <li><code>moderne</code></li>
      <li><code>"old balcony"</code> or <code>oldbalcony</code></li>
      <li><code>"old ballroom"</code> or <code>oldballroom</code></li>
      <li><code>"old courtyard 1"</code>, <code>oldcourtyard1</code> or <code>cy1</code></li>
      <li><code>"old courtyard 2"</code>, <code>oldcourtyard2</code> or <code>cy2</code></li>
      <li><code>"old gallery"</code> or <code>oldgallery</code></li>
      <li><code>"old veranda"</code> or <code>oldveranda</code></li>
      <li><code>panopticon</code> or <code>panop</code></li>
      <li><code>pub</code></li>
      <li><code>teien</code></li>
      <li><code>terrace</code></li>
      <li><code>veranda</code></li>
  </ul>
</details>

<details>
  <summary>--completed-missions / --completed-missions-all</summary>
  <ul>
      <li><code>"bug ambassador"</code>, <code>bugambassador</code> or <code>bug</code></li>
      <li><code>"contact double agent"</code>, <code>contactdoubleagent</code>, <code>contactda</code>, <code>contact</code> or <code>bb</code></li>
      <li><code>"fingerprint ambassador"</code>, <code>fingerprintambassador</code>, <code>fingerprint</code> or <code>fp</code></li>
      <li><code>"inspect statues"</code>, <code>inspectstatues</code> or <code>inspect</code></li>
      <li><code>"purloin guest list"</code>, <code>purloinguestlist</code> or <code>purloin</code></li>
      <li><code>"seduce target"</code>, <code>seducetarget</code> or <code>seduce</code></li>
      <li><code>"swap statue"</code>, <code>swapstatue</code> or <code>swap</code></li>
      <li><code>"transfer microfilm"</code>, <code>transfermicrofilm</code>, <code>transfermf</code> or <code>mf</code></li>
  </ul>
</details>

<details>
  <summary>--modes</summary>
  <ul>
      <li><code>any</code> or <code>a</code></li>
      <li><code>pick</code> or <code>p</code></li>
      <li><code>known</code> or <code>k</code></li>
      <li><code>"any 7 of 8"</code>, <code>"any 7/8"</code>, <code>a7/8</code></li>
      <li><code>"pick 7 of 8"</code>, <code>"pick 7/8"</code>, <code>p7/8</code></li>
      <li><code>"known 4 of 4"</code>, <code>"known 4/4"</code>, <code>"known 4"</code>, <code>k4</code></li>
  </ul>
</details>

<details>
  <summary>--result</summary>
  <ul>
      <li><code>missionswin</code></li>
      <li><code>spyshot</code></li>
      <li><code>civilianshot</code></li>
      <li><code>timeout</code></li>
      <li><code>unfinished</code></li>
  </ul>
</details>

## Examples

- Find all replays from a particular player:

  `spyparsey --player plastikqs`
- Find all replays where a player shot a civilian on Ballroom or High-Rise:

  `spyparsey --sniper plastikqs --map ballroom highrise --result civilianshot`
- Find all replays where that were an "Any x/y" mode, where both bug and contact DA were completed:

  `spyparsey --mode any --completed-missions-all bug bb`

- Find out how many times you beat Dowsey on Balcony with bug, BB and seduce:

  `spyparsey --spy plastikqs --sniper dowsey --map balcony --result missionswin --completed-missions-all bug bb seduce`

## Output

There are currently 3 modes of output. Explicit outputs are the following:

- `--count`

  Just show the number of replays that matched the filters.
- `--show-paths`

  Show the absolute paths of the replays that matched the filters. This can be piped into another command, maybe used to sort replays into various folders.
  
- `--csv`

  Outputs all the matched replays in a rather verbose CSV format. It includes almost every piece of data you can get from the header. I'm sure someone can think of clever ways to use this...

However, the default mode if you specify neither of the above is to output a few stats. What exactly is output depends on the filters (i.e. if you filter based on map, you won't get the "Maps Played" section). Below is an example of all the stats possible:

```
Total Replays:
    2625
Player Stats:
    plastikqs: 1461W 1149L (55.7%)
Maps Played:
    Courtyard: 592 (22.6%)
    Ballroom: 389 (14.8%)
    Pub: 301 (11.5%)
    Library: 270 (10.3%)
    High-Rise: 251 (9.6%)
    Moderne: 188 (7.2%)
    Gallery: 141 (5.4%)
    Balcony: 134 (5.1%)
    Terrace: 132 (5.0%)
    Veranda: 119 (4.5%)
    Aquarium: 72 (2.7%)
    Teien: 32 (1.2%)
    Panopticon: 2 (0.1%)
    Old Balcony: 2 (0.1%)
Missions Completed:
    Contact Double Agent: 1369 (53.4%)
    Purloin Guest List: 1023 (45.1%)
    Inspect Statues: 1016 (47.0%)
    Bug Ambassador: 1000 (39.7%)
    Seduce Target: 774 (30.2%)
    Swap Statue: 692 (31.9%)
    Transfer Microfilm: 285 (23.4%)
    Fingerprint Ambassador: 227 (10.2%)
Completed Mission Sets:
    Bug, BB, Inspect, Seduce: 63 (8.0%)
    BB, Inspect, Seduce, Purloin: 44 (5.6%)
    BB, Swap, Inspect, Purloin: 33 (4.2%)
    BB, Swap, Inspect, Seduce: 33 (4.2%)
    Bug, BB: 30 (3.8%)
    Bug, BB, Swap, Inspect: 29 (3.7%)
    Bug, BB, Swap, Seduce: 28 (3.5%)
    BB, Inspect, Seduce, Fingerprint: 27 (3.4%)
    Bug, BB, Seduce: 26 (3.3%)
    Bug, BB, Inspect, Purloin: 25 (3.2%)
Clock:
    Average Duration: 2m25s
    Clock Usage: 73.1%
Modes Played:
    Any: 2499 (95.2%)
    Known: 122 (4.6%)
    Pick: 4 (0.2%)
Results:
    Spy Shot: 1443 (55.0%)
    Civilian Shot: 553 (21.1%)
    Missions Win: 518 (19.7%)
    Spy Timeout: 96 (3.7%)
    Unfinished: 15 (0.6%)
```

These stats become quite useful/interesting when paired with specific filters. For example, you could find out which missions I tend to complete when I win as a spy on High-Rise a3/5:

```
$ spyparsey --spy plastikqs --map highrise --mode a3/5 --result missionswin
Total Replays:
    28
Player Stats:
    plastikqs: 28W 0L (100.0%)
Missions Completed:
    Contact Double Agent: 19 (79.2%)
    Seduce Target: 14 (63.6%)
    Purloin Guest List: 12 (66.7%)
    Transfer Microfilm: 11 (68.8%)
    Bug Ambassador: 10 (52.6%)
    Inspect Statues: 9 (50.0%)
    Swap Statue: 7 (53.8%)
    Fingerprint Ambassador: 4 (40.0%)
Completed Mission Sets:
    Bug, BB, Transfer MF: 3 (11.5%)
    BB, Seduce, Purloin: 3 (11.5%)
    Bug, BB, Purloin: 2 (7.7%)
    Swap, Inspect, Purloin: 2 (7.7%)
    BB, Inspect, Seduce: 2 (7.7%)
    Transfer MF, Seduce, Purloin: 2 (7.7%)
    BB, Transfer MF, Seduce: 2 (7.7%)
    BB, Transfer MF, Purloin: 1 (3.8%)
    BB, Inspect, Fingerprint: 1 (3.8%)
    Bug, Swap, Inspect: 1 (3.8%)
Clock:
    Average Duration: 2m39s
    Clock Usage: 90.4%
```

Seems like I enjoy going for those hard tells!

## Notes

- Most of the filter options have aliases to their singular counterpart, but behave the same way i.e. `--spy` works the same as `--spies`.
- I haven't optimised or done much performance-wise, but it's basically limited by disk read. When running over 20,000 replays it takes about 20 seconds to run the first time (regardless of filters) and then subsequent runs take less than a second or two (again, regardless of filters changing).
- SpyParty replays and folders have really long names, especially when players start creating their own organisation. If the entire path of a replay exceeds 260 characters, neither SpyParty nor spyparsey will be able to read these and will be missing in the output. You can use the `-vv` flag to see warnings about files that could not be read.
