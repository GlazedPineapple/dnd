use args::CLIArgs;
use roll::{Dice, Roll, Advantage};
use structopt::StructOpt;
use anyhow::anyhow;

mod args;
mod roll;

fn main() -> anyhow::Result<()> {
    let args = CLIArgs::from_args();

    let roll = args.roll.unwrap_or_else(|| {
        eprintln!("No dice were provided, defaulting to 1d20");

        Roll {
            dice: vec![Dice {
                count: 1,
                faces: 20,
            }],
            modifier: None,
        }
    });

    let advantage = match (args.advantage, args.disadvantage) {
        (false, false) => None,
        (false, true) => Some(Advantage::Disadvantage),
        (true, false) => Some(Advantage::Advantage),
        (true, true) => return Err(anyhow!("You cannot have both advantage and disadvantage on one roll")),
    };

    dbg!(advantage, roll);

    let result = roll::roll(roll, advantage);
    println!("Result: {}", result);

    Ok(())
}
