use polars::io::{csv::CsvReader, csv::CsvWriter, SerReader, SerWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut df = CsvReader::from_path("webp_results.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    let columns_to_exclude = ["grade1", "mistake1"];

    for col_name in columns_to_exclude.iter() {
        let _ = df.drop_in_place(col_name)?;
    }

    let columns_to_rename = [("grade2", "grade"), ("mistake2", "mistake")];

    for (old_name, new_name) in columns_to_rename.iter() {
        let _ = df.rename(old_name, new_name)?;
    }

    println!("{:?}", df);

    let mut file = std::fs::File::create("results.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();

    Ok(())
}
