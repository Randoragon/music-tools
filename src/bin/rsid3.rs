use std::env::args;
use std::process::ExitCode;
use anyhow::{anyhow, Result};
use id3::{Tag, TagLike, Frame, Content};
use id3::frame::{Comment, Lyrics, ExtendedText, ExtendedLink};

/// Represents all options passed to the program on the command line.
#[derive(Debug)]
struct Cli {
    help: bool,
    list_frames: bool,
    delimiter: Option<String>,
    null_delimited: bool,
    get_frames: Vec<Frame>,
    set_frames: Vec<Frame>,
    files: Vec<String>,
}

impl Cli {
    /// Prints how to use the program.
    fn print_usage() {
        eprintln!("Usage:  rsid3 [OPTION] FILE...");
        eprintln!("");
        eprintln!("Reads or writes ID3v2 tags in mp3 files.");
        eprintln!("Supported standards: ID3v2.2, ID3v2.3, ID3v2.4.");
        eprintln!("");
        eprintln!("Options:");
        eprintln!("  -h, --help               Show this help and exit.");
        eprintln!("  -L, --list-frames        List all supported frames.");
        eprintln!("  -d SEP, --delimiter SEP  Separate multiple printed values with SEP.");
        eprintln!("  -0, --null-delimited     Separate multiple printed values with the null byte.");
        eprintln!("  --FRAME                  Print the value of FRAME.");
        eprintln!("  --FRAME DESC             Print the value of FRAME (TXXX, WXXX).");
        eprintln!("  --FRAME DESC LANG        Print the value of FRAME (COMM, USLT).");
        eprintln!("  --FRAME= TEXT            Set the value of FRAME.");
        eprintln!("  --FRAME= DESC TEXT       Set the value of FRAME (TXXX, WXXX).");
        eprintln!("  --FRAME= DESC LANG TEXT  Set the value of FRAME (COMM, USLT).");
        eprintln!("");
        eprintln!("If the value of LANG is irrelevant when printing a frame, 'first'");
        eprintln!("can be passed instead, in which case the first frame with a matching");
        eprintln!("DESC is printed.");
        eprintln!("If no print or set options are supplied, all frames are printed.");
        eprintln!("Any number of print and set options can be passed in any order.");
        eprintln!("Print options are always evaluated before set options. Both print");
        eprintln!("and set options are evaluated in the order in which they were passed.");
    }

    /// Prints the available frames.
    fn print_all_frames() {
        println!("Read-write frames:");
        println!("COMM	User comment (DESC, LANG, TEXT)");
        println!("TALB	Album");
        println!("TBPM	Beats per minute");
        println!("TCAT	iTunes podcast category");
        println!("TCMP	iTunes compilation flag");
        println!("TCOM	Composer");
        println!("TCON	Content type (genre)");
        println!("TCOP	Copyright");
        println!("TDAT	Date of recording (DDMM)");
        println!("TDEN	Encoding time");
        println!("TDES	iTunes podcast description");
        println!("TDLY	Audio delay (ms)");
        println!("TDOR	Original release time");
        println!("TDRC	Recording time");
        println!("TDRL	Release time");
        println!("TDTG	Tagging time");
        println!("TENC	Encoder");
        println!("TEXT	Lyricist");
        println!("TFLT	File type");
        println!("TGID	iTunes podcast identifier");
        println!("TIME	Time of recording (HHMM)");
        println!("TIPL	Involved people list");
        println!("TIT1	Content group description");
        println!("TIT2	Title");
        println!("TIT3	Subtitle/description refinement");
        println!("TKEY	Starting key");
        println!("TKWD	iTunes podcast keywords");
        println!("TLAN	Audio languages");
        println!("TLEN	Audio length (ms)");
        println!("TMCL	Musicians credits list");
        println!("TMED	Source media type");
        println!("TMOO	Mood");
        println!("TOAL	Original album");
        println!("TOFN	Original filename");
        println!("TOLY	Original lyricist");
        println!("TOPE	Original artist/performer");
        println!("TORY	Original release year");
        println!("TOWN	Owner/Licensee");
        println!("TPE1	Lead artist/performer/soloist/group");
        println!("TPE2	Band/Orchestra/Accompaniment");
        println!("TPE3	Conductor");
        println!("TPE4	Interpreter/Remixer/Modifier");
        println!("TPOS	Part of set");
        println!("TPRO	Produced");
        println!("TPUB	Publisher");
        println!("TRCK	Track number");
        println!("TRDA	Recording dates");
        println!("TRSN	Internet radio station name");
        println!("TRSO	Internet radio station owner");
        println!("TSIZ	Size of audio data (bytes)");
        println!("TSO2	iTunes album artist sort");
        println!("TSOA	Album sort order key");
        println!("TSOC	iTunes composer sort");
        println!("TSOP	Performer sort order key");
        println!("TSOT	Title sort order key");
        println!("TSRC	International Standard Recording Code (ISRC)");
        println!("TSSE	Encoder settings");
        println!("TSST	Set subtitle");
        println!("TXXX	User-defined text data (DESC, TEXT)");
        println!("TYER	Year of recording");
        println!("USLT	Unsynchronised lyrics/text transcription (DESC, LANG, TEXT)");
        println!("WCOM	Commercial information");
        println!("WCOP	Copyright information");
        println!("WFED	iTunes podcast feed");
        println!("WOAF	Official file information");
        println!("WOAR	Official artist/performer information");
        println!("WOAS	Official source information");
        println!("WORS	Official internet radio information");
        println!("WPAY	Payment information");
        println!("WPUB	Official publisher information");
        println!("WXXX	User-defined URL data (DESC, URL)");
        println!("");
        println!("Read-only frames (rudimentary support):");
        println!("AENC	Audio encryption");
        println!("APIC	Attached (or linked) picture");
        println!("ASPI	Audio seek point index");
        println!("CHAP	Chapter");
        println!("COMR	Commercial frame");
        println!("CTOC	Table of contents");
        println!("ENCR	Encryption method registration");
        println!("EQU2	Equalization 2");
        println!("ETCO	Event timing codes");
        println!("GEOB	General encapsulated object");
        println!("GRID	Group identification registration");
        println!("GRP1	iTunes grouping");
        println!("IPLS	Involved people list");
        println!("LINK	Linked information");
        println!("MCDI	Binary dump of CD's TOC");
        println!("MLLT	MPEG location lookup table");
        println!("MVIN	iTunes movement number/count");
        println!("MVNM	iTunes movement name");
        println!("OWNE	Ownership frame");
        println!("PCNT	Play counter");
        println!("PCST	iTunes podcast flag");
        println!("POPM	Popularimeter");
        println!("POSS	Position synchronisation frame");
        println!("PRIV	Private frame");
        println!("RBUF	Recommended buffer size");
        println!("RVA2	Relative volume adjustment 2");
        println!("RVAD	Relative volume adjustment");
        println!("RVRB	Reverb");
        println!("SEEK	Seek frame");
        println!("SIGN	Signature frame");
        println!("SYLT	Synchronised lyrics/text");
        println!("SYTC	Synchronised tempo codes");
        println!("UFID	Unique file identifier");
        println!("USER	Terms of use");
    }

    /// Construct a Cli object representing passed command-line arguments.
    fn parse_args() -> Result<Self> {
        let args: Vec<String> = args().collect();
        let mut help = false;
        let mut list_frames = false;
        let mut delimiter: Option<String> = None;
        let mut null_delimited = false;
        let mut get_frames = vec![];
        let mut set_frames = vec![];
        let mut i = 1;
        while i < args.len() {
            let arg = args[i].as_str();
            match arg {
                "-h" | "--help" => { help = true; },
                "-L" | "--list-frames" => { list_frames = true; },
                "-d" | "--delimiter" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --delimiter"));
                    }
                    delimiter = Some(args[i + 1].clone());
                    i += 1;
                },
                str if str.starts_with("-d") => {
                    delimiter = Some(((args[i])[2..]).to_string());
                },
                "-0" | "--null-delimited" => { null_delimited = true; },
                "--" => { i += 1; break; },

                "--COMM" => {
                    if i + 2 >= args.len() {
                        return Err(anyhow!("2 arguments expected after --COMM"));
                    }
                    let comment = Comment {
                        description: args[i + 1].clone(),
                        lang: args[i + 2].clone(),
                        text: "".to_string(),
                    };
                    get_frames.push(Frame::with_content("COMM", Content::Comment(comment)));
                    i += 2;
                }
                "--USLT" => {
                    if i + 2 >= args.len() {
                        return Err(anyhow!("2 arguments expected after --USLT"));
                    }
                    let lyrics = Lyrics {
                        description: args[i + 1].clone(),
                        lang: args[i + 2].clone(),
                        text: "".to_string(),
                    };
                    get_frames.push(Frame::with_content("USLT", Content::Lyrics(lyrics)));
                    i += 2;
                },

                "--TXXX" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TXXX"));
                    }
                    let extended_text = ExtendedText {
                        value: "".to_string(),
                        description: args[i + 1].clone(),
                    };
                    get_frames.push(Frame::with_content("TXXX", Content::ExtendedText(extended_text)));
                    i += 1;
                },
                "--WXXX" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --WXXX"));
                    }
                    let extended_link = ExtendedLink {
                        link: "".to_string(),
                        description: args[i + 1].clone(),
                    };
                    get_frames.push(Frame::with_content("WXXX", Content::ExtendedLink(extended_link)));
                    i += 1;
                },

                // All parameterless getters
                str if Cli::is_getter_arg(str) => {
                    get_frames.push(Frame::text(&str[2..], ""));
                },

                "--COMM=" => {
                    if i + 3 >= args.len() {
                        return Err(anyhow!("3 arguments expected after --COMM="));
                    }
                    let comment = Comment {
                        description: args[i + 1].clone(),
                        lang: args[i + 2].clone(),
                        text: args[i + 3].clone(),
                    };
                    set_frames.push(Frame::with_content("COMM", Content::Comment(comment)));
                    i += 3;
                }
                "--USLT=" => {
                    if i + 3 >= args.len() {
                        return Err(anyhow!("3 arguments expected after --USLT="));
                    }
                    let lyrics = Lyrics {
                        description: args[i + 1].clone(),
                        lang: args[i + 2].clone(),
                        text: args[i + 3].clone(),
                    };
                    set_frames.push(Frame::with_content("USLT", Content::Lyrics(lyrics)));
                    i += 3;
                }

                "--TXXX=" => {
                    if i + 2 >= args.len() {
                        return Err(anyhow!("2 arguments expected after --TXXX="));
                    }
                    let extended_text = ExtendedText {
                        description: args[i + 1].clone(),
                        value: args[i + 2].clone(),
                    };
                    set_frames.push(Frame::with_content("TXXX", Content::ExtendedText(extended_text)));
                    i += 2;
                },
                "--WXXX=" => {
                    if i + 2 >= args.len() {
                        return Err(anyhow!("2 arguments expected after --WXXX="));
                    }
                    let extended_link = ExtendedLink {
                        description: args[i + 1].clone(),
                        link: args[i + 2].clone(),
                    };
                    set_frames.push(Frame::with_content("WXXX", Content::ExtendedLink(extended_link)));
                    i += 2;
                },

                // All parameterless setters
                str if Cli::is_setter_arg(str) => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after {str}"));
                    }
                    let text = args[i + 1].clone();
                    set_frames.push(Frame::text(&str[2..(str.len() - 1)], text));
                    i += 1;
                },

                str => {
                    if str.starts_with("-") {
                        return Err(anyhow!("Unknown option: '{arg}'"));
                    }
                    break;
                }
            };
            i += 1;
        }

        let files = (i..args.len())
            .map(|x| args[x].clone())
            .collect();

        Ok(Cli {
            help,
            list_frames,
            get_frames,
            delimiter,
            null_delimited,
            set_frames,
            files,
        })
    }

    /// Checks if a command-line argument is a getter argument.
    fn is_getter_arg(arg: &str) -> bool {
        arg.starts_with("--") && (&arg[2..]).chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    }

    /// Checks if a command-line argument is a setter argument.
    fn is_setter_arg(arg: &str) -> bool {
        arg.starts_with("--") && arg.ends_with("=") && match &arg[2..(arg.len() - 1)] {
            "COMM" | "TALB" | "TBPM" | "TCAT" | "TCMP" | "TCOM" | "TCON" | "TCOP" |
            "TDAT" | "TDEN" | "TDES" | "TDLY" | "TDOR" | "TDRC" | "TDRL" | "TDTG" |
            "TENC" | "TEXT" | "TFLT" | "TGID" | "TIME" | "TIPL" | "TIT1" | "TIT2" |
            "TIT3" | "TKEY" | "TKWD" | "TLAN" | "TLEN" | "TMCL" | "TMED" | "TMOO" |
            "TOAL" | "TOFN" | "TOLY" | "TOPE" | "TORY" | "TOWN" | "TPE1" | "TPE2" |
            "TPE3" | "TPE4" | "TPOS" | "TPRO" | "TPUB" | "TRCK" | "TRDA" | "TRSN" |
            "TRSO" | "TSIZ" | "TSO2" | "TSOA" | "TSOC" | "TSOP" | "TSOT" | "TSRC" |
            "TSSE" | "TSST" | "TXXX" | "TYER" | "USLT" | "WCOM" | "WCOP" | "WFED" |
            "WOAF" | "WOAR" | "WOAS" | "WORS" | "WPAY" | "WPUB" | "WXXX" => true,
            _ => false,
        }
    }

    /// Checks if a command-line argument is a delete argument.
    fn is_delete_arg(arg: &str) -> bool {
        arg.len() > 3 && arg.starts_with("--") && arg.ends_with("-")
        && (&arg[2..(arg.len() - 1)]).chars() .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    }
}

/// Convenience wrapper for getting any simple text content.
fn get_content_text<'a>(frame: &'a Frame) -> Result<&'a str> {
    match frame.content().text() {
        Some(x) => Ok(x),
        None => Err(anyhow!("Frame claims to be {} with T but has no text content: {frame:?}", frame.id())),
    }
}

/// Convenience wrapper for getting any link content.
fn get_content_link<'a>(frame: &'a Frame) -> Result<&'a str> {
    match frame.content().link() {
        Some(x) => Ok(x),
        None => Err(anyhow!("Frame claims to be {} with T but has no link content: {frame:?}", frame.id())),
    }
}

/// Convenience wrapper for getting TXXX content.
fn get_content_txxx<'a>(frame: &'a Frame) -> Result<&'a ExtendedText> {
    match frame.content().extended_text() {
        Some(x) => Ok(x),
        None => Err(anyhow!("Frame claims to be TXXX but has no extended text content: {frame:?}")),
    }
}

/// Convenience wrapper for getting WXXX content.
fn get_content_wxxx<'a>(frame: &'a Frame) -> Result<&'a ExtendedLink> {
    match frame.content().extended_link() {
        Some(x) => Ok(x),
        None => Err(anyhow!("Frame claims to be WXXX but has no extended link content: {frame:?}")),
    }
}

/// Convenience wrapper for getting COMM content.
fn get_content_comm<'a>(frame: &'a Frame) -> Result<&'a Comment> {
    match frame.content().comment() {
        Some(x) => Ok(x),
        None => Err(anyhow!("Frame claims to be COMM but has no comment content: {frame:?}")),
    }
}

/// Convenience wrapper for getting USLT content.
fn get_content_uslt<'a>(frame: &'a Frame) -> Result<&'a Lyrics> {
    match frame.content().lyrics() {
        Some(x) => Ok(x),
        None => Err(anyhow!("Frame claims to be USLT but has no lyrics content: {frame:?}")),
    }
}

/// Get text contents from a tag, based on a frame query.
fn print_text_from_tag<'a>(tag: &'a Tag, frame: &Frame) -> Result<()> {
    match frame.id() {
        "TXXX" => {
            let desc_query = &get_content_txxx(frame)?.description;

            for txxx in tag.frames().filter(|&f| f.id() == "TXXX") {
                let extended_text = match get_content_txxx(txxx) {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("rsid3: {e}");
                        continue;
                    },
                };
                if extended_text.description == *desc_query {
                    println!("{}", extended_text.value);
                    return Ok(());
                }
            }
            return Err(anyhow!("TXXX frame with description '{desc_query}' not found"));
        },
        "WXXX" => {
            let desc_query = &get_content_wxxx(frame)?.description;
            for wxxx in tag.frames().filter(|&f| f.id() == "WXXX") {
                let extended_link = match get_content_wxxx(wxxx) {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("rsid3: {e}");
                        continue;
                    },
                };
                if extended_link.description == *desc_query {
                    println!("{}", extended_link.link);
                    return Ok(());
                }
            }
            return Err(anyhow!("WXXX frame with description '{desc_query}' not found"));
        },
        "COMM" => {
            let comment_query = get_content_comm(frame)?;
            let (desc_query, lang_query) = (&comment_query.description, &comment_query.lang);
            for comm in tag.frames().filter(|&f| f.id() == "COMM") {
                let comment = match get_content_comm(comm) {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("rsid3: {e}");
                        continue;
                    },
                };
                if comment.description == *desc_query && (comment.lang == *lang_query || *lang_query == "first") {
                    println!("{}", comment.text);
                    return Ok(());
                }
            }
            return Err(anyhow!("COMM frame with description '{desc_query}' and language '{lang_query}' not found"));
        },
        "USLT" => {
            let lyrics_query = get_content_uslt(frame)?;
            let (desc_query, lang_query) = (&lyrics_query.description, &lyrics_query.lang);
            for uslt in tag.frames().filter(|&f| f.id() == "USLT") {
                let lyrics = match get_content_uslt(uslt) {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("rsid3: {e}");
                        continue;
                    },
                };
                if lyrics.description == *desc_query && (lyrics.lang == *lang_query || *lang_query == "first") {
                    println!("{}", lyrics.text);
                    return Ok(());
                }
            }
            return Err(anyhow!("USLT frame with description '{desc_query}' and language '{lang_query}' not found"));
        },
        x if x.starts_with("T") => {
            let text_frame = match tag.get(x) {
                Some(frame) => frame,
                None => return Err(anyhow!("Frame not found: {x}")),
            };
            println!("{}", get_content_text(text_frame)?);
            return Ok(());
        },
        x if x.starts_with("W") => {
            let link_frame = match tag.get(x) {
                Some(frame) => frame,
                None => return Err(anyhow!("Frame not found: {x}")),
            };
            println!("{}", get_content_link(link_frame)?);
            return Ok(());
        },
        x => {
            let frame = match tag.get(x) {
                Some(frame) => frame,
                None => return Err(anyhow!("Frame not found: {x}")),
            };
            println!("{}", frame.content());
            return Ok(());
        },
    }
}

/// Prints frames from a file, with a custom delimiter.
fn print_file_frames(fpath: &str, frames: &Vec<Frame>, delimiter: &str) -> Result<()> {
    let tag = match Tag::read_from_path(fpath) {
        Ok(tag) => tag,
        Err(e) => return Err(anyhow!("Failed to read tags from file '{fpath}': {e}")),
    };

    let mut is_first = true;
    for frame in frames {
        match is_first {
            true => is_first = false,
            false => print!("{delimiter}"),
        }
        if let Err(e) = print_text_from_tag(&tag, frame) {
            eprintln!("rsid3: {e}");
        }
    }

    Ok(())
}

/// Pretty-prints a single frame.
fn print_frame_pretty(frame: &Frame) -> Result<()> {
    match frame.id() {
        "TXXX" => {
            let extended_text = get_content_txxx(frame)?;
            println!("{}[{}]: {}", frame.id(), extended_text.description, extended_text.value);
        },
        "WXXX" => {
            let extended_link = get_content_wxxx(frame)?;
            println!("{}[{}]: {}", frame.id(), extended_link.description, extended_link.link);
        },
        "COMM" => {
            let comment = get_content_comm(frame)?;
            println!("{}[{}]({}): {}", frame.id(), comment.description, comment.lang, comment.text);
        },
        "USLT" => {
            let lyrics = get_content_uslt(frame)?;
            println!("{}[{}][{}]: {}", frame.id(), lyrics.description, lyrics.lang, lyrics.text);
        },
        str if str.starts_with("T") => {
            println!("{}: {}", frame.id(), get_content_text(frame)?);
        },
        str if str.starts_with("W") => {
            println!("{}: {}", frame.id(), get_content_link(frame)?);
        },
        _ => {
            println!("{}: {}", frame.id(), frame.content());
        },
    }
    Ok(())
}

/// Pretty-prints all supported frames stored in the file.
fn print_all_file_frames_pretty(fpath: &str) -> Result<()> {
    let tag = match Tag::read_from_path(fpath) {
        Ok(tag) => tag,
        Err(e) => return Err(anyhow!("Failed to read tags from file '{fpath}': {e}")),
    };

    let n_frames = tag.frames().count();
    println!("\n{}: {}, {} frame{}:", fpath, tag.version(), n_frames,
        if n_frames == 1 { "" } else { "s" });
    for frame in tag.frames() {
        print_frame_pretty(frame)?;
    }

    Ok(())
}

// Writes frames into a file. Previous values are overwritten, if any.
fn set_file_frames(fpath: &str, frames: Vec<Frame>) -> Result<()> {
    let mut tag = match Tag::read_from_path(fpath) {
        Ok(tag) => tag,
        Err(e) => return Err(anyhow!("Failed to read tags from file '{fpath}': {e}")),
    };

    let mut was_modified = false;
    for frame in frames {
        match frame.id() {
            x if x.starts_with("T") || x.starts_with("W") || x == "COMM" || x == "USLT" => {
                let _ = tag.add_frame(frame);
                was_modified = true;
            },
            _ => return Err(anyhow!("Writing to {frame} is not supported")),
        }
    }

    if was_modified {
        if let Err(e) = tag.write_to_path(fpath, tag.version()) {
            return Err(anyhow!("Failed to write tags to '{fpath}': {e}"));
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    let cli = match Cli::parse_args() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("rsid3: {e}, try 'rsid3 --help'");
            return ExitCode::FAILURE;
        }
    };

    if cli.help {
        Cli::print_usage();
        return ExitCode::SUCCESS;
    }

    if cli.list_frames {
        Cli::print_all_frames();
        return ExitCode::SUCCESS;
    }

    // Define the delimiter
    if cli.delimiter.is_some() && cli.null_delimited {
        eprintln!("rsid3: --delimiter and --null-delimited options are mutually exclusive");
        return ExitCode::FAILURE;
    }
    let delimiter = if cli.null_delimited {
        '\0'.to_string()
    } else {
        cli.delimiter.clone().unwrap_or('\n'.to_string())
    };

    // Handle all get options
    for fpath in &cli.files {
        if let Err(e) = print_file_frames(fpath, &cli.get_frames, &delimiter) {
            eprintln!("rsid3: {e}");
            return ExitCode::FAILURE;
        }
    }

    // Handle all set options
    for fpath in &cli.files {
        if let Err(e) = set_file_frames(fpath, cli.set_frames.to_owned()) {
            eprintln!("rsid3: {e}");
            return ExitCode::FAILURE;
        }
    }

    // Print all frames if no options supplied
    if cli.get_frames.is_empty() && cli.set_frames.is_empty() {
        for fpath in &cli.files {
            if let Err(e) = print_all_file_frames_pretty(fpath) {
                eprintln!("rsid3: {e}");
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
