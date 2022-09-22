//
// This file is part of flash-img-seeker
//
// Copyright (C) 2022 Paul-Erwan RIO <paulerwan.rio@proton.me>
//
//

pub mod error;

use error::*;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::path::Path;

const HEADER_SZ: usize = 16;

#[derive(Debug)]
pub struct ImgHashTable {
    offset: usize,
    hash: u64,
    header: [u8; HEADER_SZ],
    size: usize,
}

fn compute_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn compute_hash_by_block(f: &File, block_size: usize) -> Result<Vec<ImgHashTable>> {
    let flash_size = f.metadata()?.len() as usize;
    let mut reader = BufReader::new(f);

    reader.seek(SeekFrom::Start(0))?;

    let mut offset: usize = 0;
    let remaining = flash_size % block_size;
    let mut buffer = vec![0u8; block_size];
    let mut table = Vec::<ImgHashTable>::new();

    loop {
        let size_r = reader.read(&mut buffer)?;

        if size_r != block_size {
            if size_r == remaining {
                if remaining != 0 {
                    buffer.truncate(remaining);
                } else {
                    // EOF reached and all file processed
                    break;
                }
            } else {
                // Unexpected EOF
                return Err(Error::ReadErr);
            }
        }

        let hash = compute_hash(&buffer);
        let mut header = [0u8; HEADER_SZ];
        header.copy_from_slice(&buffer[0..HEADER_SZ]);

        let hash_elem = ImgHashTable {
            offset,
            hash,
            header,
            size: size_r,
        };
        table.push(hash_elem);

        offset += block_size;

        if size_r == remaining {
            break;
        }
    }

    Ok(table)
}

fn locate_image_in_table(
    flash_hash_table: &Vec<ImgHashTable>,
    image_hash_table: &Vec<ImgHashTable>,
) -> Vec<usize> {
    let mut found = Vec::<usize>::new();

    let image_len = image_hash_table.len();
    let end = flash_hash_table.len() - image_len + 1;

    for (i, flash_elem) in flash_hash_table[..end].iter().enumerate() {
        let flash_extract = &flash_hash_table[i..i + image_len];

        // the headers must match
        if flash_extract
            .iter()
            .zip(image_hash_table.iter())
            .all(|(x, y)| x.header == y.header)
        {
            // then, the hash must match
            if flash_extract
                .iter()
                .take(image_len - 1)
                .zip(image_hash_table.iter().take(image_len - 1))
                .all(|(x, y)| x.hash == y.hash)
            {
                // The last element's hash has not been checked on purpose
                // as if the image is not 'block_size'-aligned, the length of
                // the last element does not match a complete block length.
                // Also, between NAND or eMMC images, the 'padding' content
                // cannot be predicted, as it could be x00's or xff's.

                // TODO: make the check for the last block
                // We would either:
                // - need the original flash image data (to compute a new
                //   hash from the correct length, or check each value one
                //   by one)
                // - or compute two hashes for the last block of the bin image:
                //   one padded with x00's, and one padded with xff's.
                // The will work only if we never encounter the case of
                // non-padded images (non-aligned images) of images padded
                // with other values
                found.push(flash_elem.offset);
            }
        }
    }

    found
}

pub fn seek_image<P: AsRef<Path>>(
    flash_hash_table: &Vec<ImgHashTable>,
    image_path: P,
    block_size: usize,
) -> Result<()> {
    let bin_file = File::open(&image_path)?;

    let image_hash_table = compute_hash_by_block(&bin_file, block_size)?;

    let found = locate_image_in_table(flash_hash_table, &image_hash_table);

    dbg!(found);

    Ok(())
}

pub fn process_flash_img<P: AsRef<Path>>(
    flash_img_path: P,
    block_size: usize,
) -> Result<Vec<ImgHashTable>> {
    assert!(block_size > HEADER_SZ);

    let f = File::open(flash_img_path)?;

    let table = compute_hash_by_block(&f, block_size)?;

    Ok(table)
}
