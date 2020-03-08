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
            .default_value("v115@9fwhIewhIewhIewhSfAgH")
            .help("The fumen from 'harddrop.com/fumen/'")
            .required(true)
            .takes_value(true));
                
    let (fumen_field, _comment) = fumen::decode(
        matches.get_matches().value_of("tetfu").unwrap()
    );

    let (base_field, color_field) = field::split_color(fumen_field);

    let pieces = piece::color_field_to_pieces(color_field).unwrap();
    
    println!("{}%",
        field::find_percentage(
            base_field,
            pieces,
            field::PercentageOptions::new(),
        ).unwrap()
    );
}
