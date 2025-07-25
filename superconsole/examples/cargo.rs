/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use std::thread::sleep;
use std::time::Duration;

use superconsole::Component;
use superconsole::Line;
use superconsole::Lines;
use superconsole::Span;
use superconsole::SuperConsole;
use superconsole::style::Stylize;

const CRATES: &str = include_str!("cargo/crates.txt");
const WIDTH: usize = "=======>                  ".len() - 1;

#[derive(Debug)]
struct LoadingBar<'a> {
    crates: &'a [&'a str],
    iteration: usize,
}

impl Component for LoadingBar<'_> {
    fn draw_unchecked(
        &self,

        _dimensions: superconsole::Dimensions,
        mode: superconsole::DrawMode,
    ) -> anyhow::Result<superconsole::Lines> {
        let res = match mode {
            superconsole::DrawMode::Normal => {
                const BUILDING: &str = "   Building ";
                let iteration = self.iteration;
                let percentage = iteration as f64 / self.crates.len() as f64;
                let amount = (percentage * WIDTH as f64).ceil() as usize;

                let building = Span::new_styled(BUILDING.to_owned().cyan().bold())?;
                let loading_bar = format!(
                    "[{test:=>bar_amt$}{test2:padding_amt$}] {}/{}: ...",
                    iteration,
                    self.crates.len(),
                    test = ">",
                    test2 = "",
                    bar_amt = amount,
                    padding_amt = WIDTH - amount,
                );
                let loading = Span::new_unstyled(loading_bar)?;
                Line::from_iter([building, loading])
            }
            superconsole::DrawMode::Final => {
                const FINISHED: &str = "   Finished ";
                let finished = Span::new_styled(FINISHED.to_owned().green().bold())?;
                const COMPLETION: &str = "dev [unoptimized + debuginfo] target(s) in 14.45s";
                Line::from_iter([finished, Span::new_unstyled(COMPLETION)?])
            }
        };

        Ok(Lines(vec![res]))
    }
}

fn main() {
    let crates: Vec<_> = CRATES
        .lines()
        .map(|line| line.trim().split_once(char::is_whitespace).unwrap().1)
        .collect();
    let count = crates.len();

    let mut superconsole = SuperConsole::new().unwrap();

    for (i, c) in crates.iter().enumerate() {
        let building = Span::new_styled("  Compiling ".to_owned().green().bold()).unwrap();
        superconsole.emit(Lines(vec![Line::from_iter([
            building,
            Span::new_unstyled(c).unwrap(),
        ])]));
        superconsole
            .render(&LoadingBar {
                crates: &crates,
                iteration: i,
            })
            .unwrap();
        sleep(Duration::from_secs_f64(0.2));
    }

    superconsole
        .finalize(&LoadingBar {
            crates: &crates,
            iteration: count,
        })
        .unwrap();
}
