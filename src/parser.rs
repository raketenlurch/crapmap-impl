use std::num::NonZeroU8;

use anyhow::{anyhow, Ok};

#[derive(Debug)]
#[allow(unused)]
struct Crapmap {
    width: NonZeroU8,
    height: NonZeroU8,
    colors: Option<NonZeroU8>,
    color_table: Option<Vec<[u8; 3]>>,
    color_pixel: Vec<Vec<u8>>,
}

impl Crapmap {
    /*fn new() -> Self {
        Self {
            magic_bytes: (),
            version: (),
            width: (),
            height: (),
            colors: (),
            color_table: (),
            color_pixel: (),
        }
    }*/
}

impl TryFrom<&[u8]> for Crapmap {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (magic_bytes, rest) = value.split_at(4);

        if magic_bytes != [0x43, 0x52, 0x42, 0x50] {
            return Err(anyhow!("not a crapmap image"));
        }

        let version = rest.get(0).ok_or_else(|| anyhow!("missing version"))?;
        if *version != 0x01 {
            return Err(anyhow!("unsupported version"));
        }

        let width: NonZeroU8 =
            (*rest.get(1).ok_or_else(|| anyhow!("missing width"))?).try_into()?;

        let height: NonZeroU8 =
            (*rest.get(2).ok_or_else(|| anyhow!("missing height"))?).try_into()?;

        let (_, rest) = rest.split_at(3);

        let pixel_count: usize = width.get() as usize * height.get() as usize;
        let (color_spec, pixel_bytes) = rest.split_at(rest.len() - pixel_count);

        let colors: Option<NonZeroU8> = color_spec.get(0).map(|c| (*c).try_into()).transpose()?;
        let color_table = if let Some(colors) = colors {
            let mut tables = Vec::new();
            for table_num in 0..colors.get() {
                let byte_index = 1 + (table_num * 3) as usize;
                let table: [u8; 3] = color_spec
                    .get(byte_index..byte_index + 3)
                    .ok_or_else(|| anyhow!("missing color table"))?
                    .try_into()
                    .unwrap();

                tables.push(table);
            }
            Some(tables)
        } else {
            None
        };

        let mut pixels = Vec::new();
        for line in 0..height.get() {
            let byte_index = (line * width.get()) as usize;
            let row: Vec<u8> = pixel_bytes
                .get(byte_index..byte_index + width.get() as usize)
                .ok_or_else(|| anyhow!("missing pixels"))?
                .into();

            pixels.push(row);
        }

        Ok(Crapmap {
            width,
            height,
            colors,
            color_table,
            color_pixel: pixels,
        })
    }
}
