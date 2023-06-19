use std::fs::File;
use std::path::Path;
use symphonia::core::errors::Result;
use symphonia::core::formats::{FormatOptions, Track};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, StandardTagKey, Tag, Value, Visual};
use symphonia::core::probe::{Hint, ProbeResult};
#[derive(Debug)]
pub struct SongMetadata {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    track_no: Option<u8>,
    disc_no: Option<u8>,
    genre: Option<String>,
    date: Option<String>,
    comment: Option<String>,
    language: Option<String>,
    codec: Option<String>,
    encoder: Option<String>,
    bits_per_sample: Option<u32>,
    duration: Option<u64>,
    sample_rate: Option<u32>,
}
lazy_static! {
    static ref SUPPORTED_EXT_ARRAY: Vec<&'static str> = vec![
        "iso",
        "ape",
        "flac",
        "flac",
        "aif",
        "aiff",
        "wav",
        "m4a",
        "aac",
        "mp2",
        "mp3",
        "ogg",
        "wma",
        "opus",
        "tak"
    ];
    // ISO,APE,FLAC,AIF,AIFF,WAV,M4A,AAC,MP2,MP3,OGG,WMA,OPUS,TAK
}
pub fn probe(source: File, path: &str) -> Result<SongMetadata> {
    let source = Box::new(source);
    let mut hint = Hint::new();
    let path = Path::new(path);
    if let Some(extension) = path.extension() {
        if let Some(extension_str) = extension.to_str() {
            if !SUPPORTED_EXT_ARRAY.contains(&extension_str) {
                return Err(symphonia::core::errors::Error::Unsupported("unknown ext"));
            }
            hint.with_extension(extension_str);
        }
    }
    let mss = MediaSourceStream::new(source, Default::default());
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    match symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts) {
        Ok(mut probed) => Ok(get_format(&mut probed)),
        Err(err) => Err(err),
    }
}
fn get_format(probed: &mut ProbeResult) -> SongMetadata {
    let mut metadata = SongMetadata {
        title: None,
        artist: None,
        album: None,
        track_no: None,
        disc_no: None,
        genre: None,
        date: None,
        comment: None,
        language: None,
        codec: None,
        encoder: None,
        bits_per_sample: None,
        duration: None,
        sample_rate: None,
    };
    get_tracks(probed.format.tracks(), &mut metadata);
    // Prefer metadata that's provided in the container format, over other tags found during the
    // probe operation.
    if let Some(metadata_rev) = probed.format.metadata().current() {
        get_tags(metadata_rev.tags(), &mut metadata);
        get_visuals(metadata_rev.visuals(), &mut metadata);
    } else if let Some(metadata_rev) = probed.metadata.get().as_ref().and_then(|m| m.current()) {
        get_tags(metadata_rev.tags(), &mut metadata);
        get_visuals(metadata_rev.visuals(), &mut metadata);
    }
    metadata
}

fn get_tracks(tracks: &[Track], metadata: &mut SongMetadata) {
    if !tracks.is_empty() {
        for (idx, track) in tracks.iter().enumerate() {
            let params = &track.codec_params;

            if let Some(codec) = symphonia::default::get_codecs().get_codec(params.codec) {
                metadata.codec = Some(codec.short_name.to_string());
            }
            metadata.sample_rate = params.sample_rate;
            if let Some(n_frames) = params.n_frames {
                if let Some(tb) = params.time_base {
                    metadata.duration = Some(tb.calc_time(n_frames).seconds)
                }
            }
            metadata.bits_per_sample = params.bits_per_sample;
        }
    }
}

fn get_tags(tags: &[Tag], metadata: &mut SongMetadata) {
    if !tags.is_empty() {
        // Print tags with a standard tag key first, these are the most common tags.
        for tag in tags.iter() {
            if let Some(std_key) = tag.std_key {
                match std_key {
                    StandardTagKey::TrackTitle => metadata.title = Some(tag.value.to_string()),
                    StandardTagKey::Artist => metadata.artist = Some(tag.value.to_string()),
                    StandardTagKey::Album => metadata.album = Some(tag.value.to_string()),
                    StandardTagKey::Language => metadata.language = Some(tag.value.to_string()),
                    StandardTagKey::Genre => metadata.genre = Some(tag.value.to_string()),
                    StandardTagKey::Date => metadata.date = Some(tag.value.to_string()),
                    StandardTagKey::DiscNumber => {
                        if let Ok(no) = tag.value.to_string().parse() {
                            metadata.disc_no = Some(no);
                        }
                    }
                    StandardTagKey::TrackNumber => {
                        if let Ok(no) = tag.value.to_string().parse() {
                            metadata.track_no = Some(no);
                        }
                    }
                    StandardTagKey::Comment => metadata.comment = Some(tag.value.to_string()),
                    _ => {}
                }
            }
        }
    }
}

fn get_visuals(visuals: &[Visual], metadata: &mut SongMetadata) {
    // if !visuals.is_empty() {
    //
    //         println!("|");
    //         println!("| // Visuals //");
    //
    //         for (idx, visual) in visuals.iter().enumerate() {
    //
    //             if let Some(usage) = visual.usage {
    //                 println!("|     [{:0>2}] Usage:      {:?}", idx + 1, usage);
    //                 println!("|          Media Type: {}", visual.media_type);
    //             } else {
    //                 println!("|     [{:0>2}] Media Type: {}", idx + 1, visual.media_type);
    //             }
    //             if let Some(dimensions) = visual.dimensions {
    //                 println!(
    //                     "|          Dimensions: {} px x {} px",
    //                     dimensions.width, dimensions.height
    //                 );
    //             }
    //             if let Some(bpp) = visual.bits_per_pixel {
    //                 println!("|          Bits/Pixel: {}", bpp);
    //             }
    //             if let Some(ColorMode::Indexed(colors)) = visual.color_mode {
    //                 println!("|          Palette:    {} colors", colors);
    //             }
    //             println!("|          Size:       {} bytes", visual.data.len());
    //
    //             // Print out tags similar to how regular tags are printed.
    //             if !visual.tags.is_empty() {
    //                 println!("|          Tags:");
    //             }
    //
    //             for (tidx, tag) in visual.tags.iter().enumerate() {
    //                 if let Some(std_key) = tag.std_key {
    //                     println!(
    //                         "{}",
    //                         print_tag_item(tidx + 1, &format!("{:?}", std_key), &tag.value, 21)
    //                     );
    //                 } else {
    //                     println!("{}", print_tag_item(tidx + 1, &tag.key, &tag.value, 21));
    //                 }
    //             }
    //         }
    // }
}
