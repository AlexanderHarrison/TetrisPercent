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
        .arg(Arg::with_name("softdrop")
            .short("s")
            .long("softdrop")
            .help("put to use softdrop");
                
    let (fumen_field, _comment) = fumen::decode(
        matches.get_matches().value_of("tetfu").unwrap()
    );

    // todo set up to use softdrop

    let (base_field, color_field) = field::split_color(fumen_field);

    match piece::color_field_to_pieces(color_field) {
        Ok(pieces) => {
            println!("{}%",
            field::find_percentage(
                base_field,
                pieces,
                field::PercentageOptions::new(),
            ).unwrap())
        },
        Err(e) => {
            println!("error: {}", e);
        },
    };
}
