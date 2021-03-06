use clap::{Arg, App};

mod fieldmatrix;
mod fumen;
mod piece;
mod field;
mod perm_gen;

#[cfg(test)]
mod tests;

fn main() {
    let matches = App::new("Tetris setup percent finder")
        .version("0.1")
        .author("Aitch: Alex Harrison")
        .arg(Arg::with_name("tetfu")
            .short("t")
            .long("tetfu")
            .help("The fumen from 'harddrop.com/fumen/'")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("no_softdrop")
            .short("S")
            .long("nosoftdrop")
            .help("put to not use softdrop when calculating"))
        .arg(Arg::with_name("no_hold")
            .short("H")
            .long("nohold")
            .help("put to not use hold when calculating"))
        .arg(Arg::with_name("no_rotation")
            .short("R")
            .long("norotation")
            .help("put to not use rotation/spins when calculating"))
        .get_matches();
        
                
    let (fumen_field, _comment) = fumen::decode(
        matches.value_of("tetfu").unwrap()
    );
    let (fumen_field, _) = field::discard_bottom(fumen_field);
    // todo set up to use softdrop

    let (base_field, color_field) = field::split_color(fumen_field);

    let options = field::PercentageOptions::new(
        !matches.is_present("no_hold"),
        !matches.is_present("no_rotation"),
        !matches.is_present("no_softdrop"),
    );

    match piece::color_field_to_pieces(color_field) {
        Ok(pieces) => {
            let impossibilities = piece::impossibilites(&pieces, &fumen_field);
            if !impossibilities.is_empty() {
                println!(
                    "error: {}",
                    piece::format_pieces(
                        &impossibilities,
                        "impossible to place pieces:\n"
                    )
                );
                return
            }

            let percent = field::find_percentage(
                base_field,
                pieces,
                options,
            );
            println!("{}%", percent);
        },
        Err(e) => println!("error: {}", e),
    };
}
