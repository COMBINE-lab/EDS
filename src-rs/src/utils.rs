use clap::ArgMatches;

#[derive(Clone, Debug, PartialEq)]
pub enum FileType {
    EDS,
    MTX,
    H5,
    CSV,
}

pub fn find_output_format(sub_m: &ArgMatches) -> FileType {
    let mut out_file_type: Option<FileType> = None;
    let mut found_file_types = 0;

    out_file_type = match sub_m.is_present("mtx") {
        true => {
            found_file_types += 1;
            Some(FileType::MTX)
        }
        false => out_file_type,
    };

    out_file_type = match sub_m.is_present("csv") {
        true => {
            found_file_types += 1;
            Some(FileType::CSV)
        }
        false => out_file_type,
    };

    out_file_type = match sub_m.is_present("h5") {
        true => {
            found_file_types += 1;
            Some(FileType::H5)
        }
        false => out_file_type,
    };

    out_file_type = match sub_m.is_present("eds") {
        true => {
            found_file_types += 1;
            Some(FileType::EDS)
        }
        false => out_file_type,
    };

    assert!(found_file_types == 1, "found unexpected not 1 file types");
    return out_file_type.expect("can't find output format type");
}

pub fn get_output_path(input_path: &str, otype: FileType) -> (FileType, String) {
    let mut itype: FileType = FileType::EDS;
    let mut opath = input_path.to_string();
    let mut offset: usize = opath.len();
    let mut found_file_types = 0;

    match opath.find(".eds") {
        Some(val) => {
            offset = val;
        }
        _ => (),
    };

    match opath.find(".mtx") {
        Some(val) => {
            offset = val;
            itype = FileType::MTX;
            found_file_types += 1;
        }
        _ => (),
    };

    match opath.find(".h5") {
        Some(val) => {
            offset = val;
            itype = FileType::H5;
            found_file_types += 1;
        }
        _ => (),
    };

    match opath.find(".csv") {
        Some(val) => {
            offset = val;
            itype = FileType::CSV;
            found_file_types += 1;
        }
        _ => (),
    };

    assert!(
        found_file_types == 1 || itype == FileType::EDS,
        " Can't find right input file type "
    );
    assert!(
        itype != otype,
        "Found same input and output file file format"
    );

    info!(" Found {:?} as input file type ", itype);
    info!(" Found {:?} as output file type ", otype);

    match otype {
        FileType::MTX => opath.replace_range(offset.., ".mtx.gz"),
        FileType::CSV => opath.replace_range(offset.., ".csv.gz"),
        FileType::H5 => opath.replace_range(offset.., ".h5"),
        //FileType::EDS => opath.replace_range(offset.., "eds.gz");
        FileType::EDS => unreachable!(),
    }

    (itype, opath)
}
