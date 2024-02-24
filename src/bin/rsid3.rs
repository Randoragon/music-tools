use std::env::args;
use std::collections::HashMap;
use std::process::ExitCode;
use anyhow::{anyhow, Result};

fn usage() {
    eprintln!("Usage:  rsid3 [OPTION] FILE...");
    eprintln!("");
    eprintln!("Reads or writes ID3v2 tags in mp3 files.");
    eprintln!("Supported standards: ID3v2.2, ID3v2.3, ID3v2.4.");
    eprintln!("");
    eprintln!("Options:");
    eprintln!("  -h, --help                Show this help and exit.");
    eprintln!("  -L, --list-frames         List all supported frames.");
    eprintln!("  --<FRAME>                 Print the value of a frame.");
    eprintln!("  --<FRAME> DESC            Print the value of a frame (TXXX, WXXX).");
    eprintln!("  --<FRAME> DESC LANG       Print the value of a frame (COMM, USLT).");
    eprintln!("  --<FRAME> DESC TEXT       Set the value of a frame (TXXX, WXXX).");
    eprintln!("  --<FRAME> DESC LANG TEXT  Set the value of a frame (COMM, USLT).");
}

struct Cli {
    help: bool,
    list_frames: bool,
    get_frames: Vec<Id3Frame>,
    set_frames: Vec<Id3Frame>,
}

enum Id3Frame {
    AENC(String), // Audio encryption
    APIC(String), // Attached (or linked) picture
    ASPI(String), // Audio seek point index
    CHAP(String), // Chapter
    COMM(String, String, String), // User comment (DESC, LANG, TEXT)
    COMR(String), // Commercial frame
    CTOC(String), // Table of contents
    ENCR(String), // Encryption method registration
    EQU2(String), // Equalization 2
    ETCO(String), // Event timing codes
    GEOB(String), // General encapsulated object
    GRID(String), // Group identification registration
    GRP1(String), // iTunes grouping
    IPLS(String), // Involved people list
    LINK(String), // Linked information
    MCDI(String), // Binary dump of CD's TOC
    MLLT(String), // MPEG location lookup table
    MVIN(String), // iTunes movement number/count
    MVNM(String), // iTunes movement name
    OWNE(String), // Ownership frame
    PCNT(String), // Play counter
    PCST(String), // iTunes podcast flag
    POPM(String), // Popularimeter
    POSS(String), // Position synchronisation frame
    PRIV(String), // Private frame
    RBUF(String), // Recommended buffer size
    RVA2(String), // Relative volume adjustment 2
    RVAD(String), // Relative volume adjustment
    RVRB(String), // Reverb
    SEEK(String), // Seek frame
    SIGN(String), // Signature frame
    SYLT(String), // Synchronised lyrics/text
    SYTC(String), // Synchronised tempo codes
    TALB(String), // Album
    TBPM(String), // Beats per minute
    TCAT(String), // iTunes podcast category
    TCMP(String), // iTunes compilation flag
    TCOM(String), // Composer
    TCON(String), // Content type (genre)
    TCOP(String), // Copyright
    TDAT(String), // Date of recording (DDMM)
    TDEN(String), // Encoding time
    TDES(String), // iTunes podcast description
    TDLY(String), // Audio delay (ms)
    TDOR(String), // Original release time
    TDRC(String), // Recording time
    TDRL(String), // Release time
    TDTG(String), // Tagging time
    TENC(String), // Encoder
    TEXT(String), // Lyricist
    TFLT(String), // File type
    TGID(String), // iTunes podcast identifier
    TIME(String), // Time of recording (HHMM)
    TIPL(String), // Involved people list
    TIT1(String), // Content group description
    TIT2(String), // Title
    TIT3(String), // Subtitle/description refinement
    TKEY(String), // Starting key
    TKWD(String), // iTunes podcast keywords
    TLAN(String), // Audio languages
    TLEN(String), // Audio length (ms)
    TMCL(String), // Musicians credits list
    TMED(String), // Source media type
    TMOO(String), // Mood
    TOAL(String), // Original album
    TOFN(String), // Original filename
    TOLY(String), // Original lyricist
    TOPE(String), // Original artist/performer
    TORY(String), // Original release year
    TOWN(String), // Owner/Licensee
    TPE1(String), // Lead artist/performer/soloist/group
    TPE2(String), // Band/Orchestra/Accompaniment
    TPE3(String), // Conductor
    TPE4(String), // Interpreter/Remixer/Modifier
    TPOS(String), // Part of set
    TPRO(String), // Produced
    TPUB(String), // Publisher
    TRCK(String), // Track number
    TRDA(String), // Recording dates
    TRSN(String), // Internet radio station name
    TRSO(String), // Internet radio station owner
    TSIZ(String), // Size of audio data (bytes)
    TSO2(String), // iTunes album artist sort
    TSOA(String), // Album sort order key
    TSOC(String), // iTunes composer sort
    TSOP(String), // Performer sort order key
    TSOT(String), // Title sort order key
    TSRC(String), // International Standard Recording Code (ISRC)
    TSSE(String), // Encoder settings
    TSST(String), // Set subtitle
    TXXX(String, String), // User-defined text data (DESC, TEXT)
    TYER(String), // Year of recording
    UFID(String), // Unique file identifier
    USER(String), // Terms of use
    USLT(String, String, String), // Unsynchronised lyrics/text transcription (DESC, LANG, TEXT)
    WCOM(String), // Commercial information
    WCOP(String), // Copyright information
    WFED(String), // iTunes podcast feed
    WOAF(String), // Official file information
    WOAR(String), // Official artist/performer information
    WOAS(String), // Official source information
    WORS(String), // Official internet radio information
    WPAY(String), // Payment information
    WPUB(String), // Official publisher information
    WXXX(String, String), // User-defined URL data (DESC, URL)
}

fn parse_args() -> Result<Cli> {
    let help = true;
    let list_frames = false;
    let get_frames = vec![];
    let set_frames = vec![];

    todo!();

    Ok(Cli {
        help,
        list_frames,
        get_frames,
        set_frames,
    })
}

fn main() -> ExitCode {
    let cli = match parse_args() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("rsid3: {e}");
            eprintln!("Try 'rsid3 --help'.");
            return ExitCode::FAILURE;
        }
    };

    todo!();

    ExitCode::SUCCESS
}
