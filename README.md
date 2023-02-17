<br/>
<p align="center">
  <a href="https://github.com/Tilo-K/rusty-blitzcrank">
    <img src="https://cdn.matches.lol/latest/img/champion/Blitzcrank.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Rusty Blitzcrank</h3>

  <p align="center">
    A batterys included library for the Riot API
    <br/>
    <br/>
    <a href="https://github.com/Tilo-K/rusty-blitzcrank"><strong>Explore the docs »</strong></a>
    <br/>
    <br/>
    <a href="https://github.com/Tilo-K/rusty-blitzcrank">View Demo</a>
    .
    <a href="https://github.com/Tilo-K/rusty-blitzcrank/issues">Report Bug</a>
    .
    <a href="https://github.com/Tilo-K/rusty-blitzcrank/issues">Request Feature</a>
  </p>
</p>

![Contributors](https://img.shields.io/github/contributors/Tilo-K/rusty-blitzcrank?color=dark-green) ![Stargazers](https://img.shields.io/github/stars/Tilo-K/rusty-blitzcrank?style=social) ![Issues](https://img.shields.io/github/issues/Tilo-K/rusty-blitzcrank) ![License](https://img.shields.io/github/license/Tilo-K/rusty-blitzcrank) ![Build](https://img.shields.io/github/actions/workflow/status/Tilo-K/rusty-blitzcrank/rust.yml)

## Table Of Contents

* [Built With](#built-with)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
* [Usage](#usage)
* [Authors](#authors)
* [Acknowledgements](#acknowledgements)

## Built With

The entire library is written in Rust.

## Getting Started

You can simply use the library installing it with cargo.

### Prerequisites

You will need cargo installed

* cargo

### Installation

1. Get a free API Key at [https://developer.riotgames.com/](https://developer.riotgames.com/)

2. Install

```sh
   cargo add rusty-blitzcrank
```



## Usage

```RUST
use rusty_blitzcrank::*;
fn main() {
    let mut client = apiclient::new("RGAPI-fffffff-aaaaae-dddd-cccc-1234567890".to_owned());

    let reg = region::Region::from_str("KR").unwrap();
    let summoner = &client.get_summoner_by_name("Hide on Bush", &reg).unwrap();

    let big_reg = region::Region::from_str("ASIA").unwrap();

    let opts = Some(types::GetMatchIdsOpts {
        count: Some(5),
        start_time: None,
        end_time: None,
        queue: None,
        game_type: Some(types::GameType::RANKED),
        start: Some(0),
    });

    let match_ids = client
        .get_match_ids(&summoner.puuid, &big_reg, opts)
        .unwrap();

    let m = client.get_match(&match_ids[0], &big_reg).unwrap();
    dbg!(&m.metadata.participants);

    let timeline = client.get_match_timeline(&match_ids[0], &big_reg).unwrap();

    for frame in timeline.info.frames {
        dbg!(&frame.participant_frames.n1.minions_killed);
    }

    let top_mast = client
        .get_champion_mastery_top(&summoner.id, None, &reg)
        .unwrap();

    dbg!(top_mast);

    let score = client
        .get_champion_mastery_score(&summoner.id, &reg)
        .unwrap();
    dbg!(score);
}
```
