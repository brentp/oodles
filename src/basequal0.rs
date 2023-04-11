use std::fs::File;
use std::io::{BufReader, BufWriter};

use noodles::bam;
use noodles::core::Position;
use noodles::sam::record::quality_scores::Score;

pub(crate) fn basequal0(
    input: String,
    output: String,
    left_read1: usize,
    right_read1: usize,
    left_read2: usize,
    right_read2: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_bam = File::open(input)?;
    let mut input_bam_reader = bam::Reader::new(BufReader::new(input_bam));

    let output_bam = File::create(output)?;
    let mut output_bam_writer = bam::Writer::new(BufWriter::new(output_bam));

    let header = input_bam_reader.read_header()?;
    // write the header to output writer
    output_bam_writer.write_header(&header)?;

    for result in input_bam_reader.records(&header) {
        let mut record = result?;
        let flags = record.flags();
        // get the base qualities from the record
        let base_qualities = record.quality_scores_mut();
        // get the flags from the record and check if it is a read1
        let (left, right) = if flags.is_first_segment() {
            (left_read1, right_read1)
        } else {
            (left_read2, right_read2)
        };

        let zero = Score::try_from(0)?;

        for i in 0..left {
            if i >= base_qualities.len() {
                break;
            }
            // set the ith base quality to 0. Position is 1-based
            let i = Position::new(i as usize + 1).ok_or(format!("invalid position {i}"))?;
            base_qualities[i] = zero;
        }
        // now do the same for the right end
        for i in 0..right {
            if i >= base_qualities.len() {
                break;
            }
            if base_qualities.len() - i == 0 {
                break;
            }
            let i = Position::new(base_qualities.len() - i - 1 + 1)
                .ok_or(format!("invalid position {i}"))?;
            base_qualities[i] = zero;
        }

        output_bam_writer.write_record(&header, &record)?;
    }

    Ok(())
}
