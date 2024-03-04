use std::env::args;
use std::collections::HashMap;
use std::process::ExitCode;
use anyhow::{anyhow, Result};

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
    eprintln!("  --FRAME= DESC TEXT       Set the value of FRAME (TXXX, WXXX).");
    eprintln!("  --FRAME= DESC LANG TEXT  Set the value of FRAME (COMM, USLT).");
    eprintln!("");
    eprintln!("If no get or set options are supplied, all frames are printed.");
    eprintln!("Any number of get and set options can be passed in any order.");
    eprintln!("Get options are always evaluated before set options. Both get and");
    eprintln!("set options are evaluated in the order in which they were passed.");
}

/// Represents all options passed to the program on the command line.
#[derive(Debug)]
struct Cli {
    help: bool,
    list_frames: bool,
    delimiter: Option<String>,
    null_delimited: bool,
    get_frames: Vec<Id3Frame>,
    set_frames: Vec<Id3Frame>,
}

/// A single ID3v2 frame (+ content, if any).
#[derive(Debug)]
enum Id3Frame {
    AENC(Option<String>), // Audio encryption
    APIC(Option<String>), // Attached (or linked) picture
    ASPI(Option<String>), // Audio seek point index
    CHAP(Option<String>), // Chapter
    COMM(Option<String>, Option<String>, Option<String>), // User comment (DESC, LANG, TEXT)
    COMR(Option<String>), // Commercial frame
    CTOC(Option<String>), // Table of contents
    ENCR(Option<String>), // Encryption method registration
    EQU2(Option<String>), // Equalization 2
    ETCO(Option<String>), // Event timing codes
    GEOB(Option<String>), // General encapsulated object
    GRID(Option<String>), // Group identification registration
    GRP1(Option<String>), // iTunes grouping
    IPLS(Option<String>), // Involved people list
    LINK(Option<String>), // Linked information
    MCDI(Option<String>), // Binary dump of CD's TOC
    MLLT(Option<String>), // MPEG location lookup table
    MVIN(Option<String>), // iTunes movement number/count
    MVNM(Option<String>), // iTunes movement name
    OWNE(Option<String>), // Ownership frame
    PCNT(Option<String>), // Play counter
    PCST(Option<String>), // iTunes podcast flag
    POPM(Option<String>), // Popularimeter
    POSS(Option<String>), // Position synchronisation frame
    PRIV(Option<String>), // Private frame
    RBUF(Option<String>), // Recommended buffer size
    RVA2(Option<String>), // Relative volume adjustment 2
    RVAD(Option<String>), // Relative volume adjustment
    RVRB(Option<String>), // Reverb
    SEEK(Option<String>), // Seek frame
    SIGN(Option<String>), // Signature frame
    SYLT(Option<String>), // Synchronised lyrics/text
    SYTC(Option<String>), // Synchronised tempo codes
    TALB(Option<String>), // Album
    TBPM(Option<String>), // Beats per minute
    TCAT(Option<String>), // iTunes podcast category
    TCMP(Option<String>), // iTunes compilation flag
    TCOM(Option<String>), // Composer
    TCON(Option<String>), // Content type (genre)
    TCOP(Option<String>), // Copyright
    TDAT(Option<String>), // Date of recording (DDMM)
    TDEN(Option<String>), // Encoding time
    TDES(Option<String>), // iTunes podcast description
    TDLY(Option<String>), // Audio delay (ms)
    TDOR(Option<String>), // Original release time
    TDRC(Option<String>), // Recording time
    TDRL(Option<String>), // Release time
    TDTG(Option<String>), // Tagging time
    TENC(Option<String>), // Encoder
    TEXT(Option<String>), // Lyricist
    TFLT(Option<String>), // File type
    TGID(Option<String>), // iTunes podcast identifier
    TIME(Option<String>), // Time of recording (HHMM)
    TIPL(Option<String>), // Involved people list
    TIT1(Option<String>), // Content group description
    TIT2(Option<String>), // Title
    TIT3(Option<String>), // Subtitle/description refinement
    TKEY(Option<String>), // Starting key
    TKWD(Option<String>), // iTunes podcast keywords
    TLAN(Option<String>), // Audio languages
    TLEN(Option<String>), // Audio length (ms)
    TMCL(Option<String>), // Musicians credits list
    TMED(Option<String>), // Source media type
    TMOO(Option<String>), // Mood
    TOAL(Option<String>), // Original album
    TOFN(Option<String>), // Original filename
    TOLY(Option<String>), // Original lyricist
    TOPE(Option<String>), // Original artist/performer
    TORY(Option<String>), // Original release year
    TOWN(Option<String>), // Owner/Licensee
    TPE1(Option<String>), // Lead artist/performer/soloist/group
    TPE2(Option<String>), // Band/Orchestra/Accompaniment
    TPE3(Option<String>), // Conductor
    TPE4(Option<String>), // Interpreter/Remixer/Modifier
    TPOS(Option<String>), // Part of set
    TPRO(Option<String>), // Produced
    TPUB(Option<String>), // Publisher
    TRCK(Option<String>), // Track number
    TRDA(Option<String>), // Recording dates
    TRSN(Option<String>), // Internet radio station name
    TRSO(Option<String>), // Internet radio station owner
    TSIZ(Option<String>), // Size of audio data (bytes)
    TSO2(Option<String>), // iTunes album artist sort
    TSOA(Option<String>), // Album sort order key
    TSOC(Option<String>), // iTunes composer sort
    TSOP(Option<String>), // Performer sort order key
    TSOT(Option<String>), // Title sort order key
    TSRC(Option<String>), // International Standard Recording Code (ISRC)
    TSSE(Option<String>), // Encoder settings
    TSST(Option<String>), // Set subtitle
    TXXX(Option<String>, Option<String>), // User-defined text data (DESC, TEXT)
    TYER(Option<String>), // Year of recording
    UFID(Option<String>), // Unique file identifier
    USER(Option<String>), // Terms of use
    USLT(Option<String>, Option<String>, Option<String>), // Unsynchronised lyrics/text transcription (DESC, LANG, TEXT)
    WCOM(Option<String>), // Commercial information
    WCOP(Option<String>), // Copyright information
    WFED(Option<String>), // iTunes podcast feed
    WOAF(Option<String>), // Official file information
    WOAR(Option<String>), // Official artist/performer information
    WOAS(Option<String>), // Official source information
    WORS(Option<String>), // Official internet radio information
    WPAY(Option<String>), // Payment information
    WPUB(Option<String>), // Official publisher information
    WXXX(Option<String>, Option<String>), // User-defined URL data (DESC, URL)
}

impl Id3Frame {
    /// Prints the available frames.
    fn print_all() {
        println!("AENC	Audio encryption");
        println!("APIC	Attached (or linked) picture");
        println!("ASPI	Audio seek point index");
        println!("CHAP	Chapter");
        println!("COMM	User comment (DESC, LANG, TEXT)");
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
        println!("UFID	Unique file identifier");
        println!("USER	Terms of use");
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
    }
}

impl Cli {
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
            println!("arg#{i}: {arg}");
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
                "--" => break,

                "--AENC" => { get_frames.push(Id3Frame::AENC(None)); },
                "--APIC" => { get_frames.push(Id3Frame::APIC(None)); },
                "--ASPI" => { get_frames.push(Id3Frame::ASPI(None)); },
                "--CHAP" => { get_frames.push(Id3Frame::CHAP(None)); },
                "--COMR" => { get_frames.push(Id3Frame::COMR(None)); },
                "--CTOC" => { get_frames.push(Id3Frame::CTOC(None)); },
                "--ENCR" => { get_frames.push(Id3Frame::ENCR(None)); },
                "--EQU2" => { get_frames.push(Id3Frame::EQU2(None)); },
                "--ETCO" => { get_frames.push(Id3Frame::ETCO(None)); },
                "--GEOB" => { get_frames.push(Id3Frame::GEOB(None)); },
                "--GRID" => { get_frames.push(Id3Frame::GRID(None)); },
                "--GRP1" => { get_frames.push(Id3Frame::GRP1(None)); },
                "--IPLS" => { get_frames.push(Id3Frame::IPLS(None)); },
                "--LINK" => { get_frames.push(Id3Frame::LINK(None)); },
                "--MCDI" => { get_frames.push(Id3Frame::MCDI(None)); },
                "--MLLT" => { get_frames.push(Id3Frame::MLLT(None)); },
                "--MVIN" => { get_frames.push(Id3Frame::MVIN(None)); },
                "--MVNM" => { get_frames.push(Id3Frame::MVNM(None)); },
                "--OWNE" => { get_frames.push(Id3Frame::OWNE(None)); },
                "--PCNT" => { get_frames.push(Id3Frame::PCNT(None)); },
                "--PCST" => { get_frames.push(Id3Frame::PCST(None)); },
                "--POPM" => { get_frames.push(Id3Frame::POPM(None)); },
                "--POSS" => { get_frames.push(Id3Frame::POSS(None)); },
                "--PRIV" => { get_frames.push(Id3Frame::PRIV(None)); },
                "--RBUF" => { get_frames.push(Id3Frame::RBUF(None)); },
                "--RVA2" => { get_frames.push(Id3Frame::RVA2(None)); },
                "--RVAD" => { get_frames.push(Id3Frame::RVAD(None)); },
                "--RVRB" => { get_frames.push(Id3Frame::RVRB(None)); },
                "--SEEK" => { get_frames.push(Id3Frame::SEEK(None)); },
                "--SIGN" => { get_frames.push(Id3Frame::SIGN(None)); },
                "--SYLT" => { get_frames.push(Id3Frame::SYLT(None)); },
                "--SYTC" => { get_frames.push(Id3Frame::SYTC(None)); },
                "--TALB" => { get_frames.push(Id3Frame::TALB(None)); },
                "--TBPM" => { get_frames.push(Id3Frame::TBPM(None)); },
                "--TCAT" => { get_frames.push(Id3Frame::TCAT(None)); },
                "--TCMP" => { get_frames.push(Id3Frame::TCMP(None)); },
                "--TCOM" => { get_frames.push(Id3Frame::TCOM(None)); },
                "--TCON" => { get_frames.push(Id3Frame::TCON(None)); },
                "--TCOP" => { get_frames.push(Id3Frame::TCOP(None)); },
                "--TDAT" => { get_frames.push(Id3Frame::TDAT(None)); },
                "--TDEN" => { get_frames.push(Id3Frame::TDEN(None)); },
                "--TDES" => { get_frames.push(Id3Frame::TDES(None)); },
                "--TDLY" => { get_frames.push(Id3Frame::TDLY(None)); },
                "--TDOR" => { get_frames.push(Id3Frame::TDOR(None)); },
                "--TDRC" => { get_frames.push(Id3Frame::TDRC(None)); },
                "--TDRL" => { get_frames.push(Id3Frame::TDRL(None)); },
                "--TDTG" => { get_frames.push(Id3Frame::TDTG(None)); },
                "--TENC" => { get_frames.push(Id3Frame::TENC(None)); },
                "--TEXT" => { get_frames.push(Id3Frame::TEXT(None)); },
                "--TFLT" => { get_frames.push(Id3Frame::TFLT(None)); },
                "--TGID" => { get_frames.push(Id3Frame::TGID(None)); },
                "--TIME" => { get_frames.push(Id3Frame::TIME(None)); },
                "--TIPL" => { get_frames.push(Id3Frame::TIPL(None)); },
                "--TIT1" => { get_frames.push(Id3Frame::TIT1(None)); },
                "--TIT2" => { get_frames.push(Id3Frame::TIT2(None)); },
                "--TIT3" => { get_frames.push(Id3Frame::TIT3(None)); },
                "--TKEY" => { get_frames.push(Id3Frame::TKEY(None)); },
                "--TKWD" => { get_frames.push(Id3Frame::TKWD(None)); },
                "--TLAN" => { get_frames.push(Id3Frame::TLAN(None)); },
                "--TLEN" => { get_frames.push(Id3Frame::TLEN(None)); },
                "--TMCL" => { get_frames.push(Id3Frame::TMCL(None)); },
                "--TMED" => { get_frames.push(Id3Frame::TMED(None)); },
                "--TMOO" => { get_frames.push(Id3Frame::TMOO(None)); },
                "--TOAL" => { get_frames.push(Id3Frame::TOAL(None)); },
                "--TOFN" => { get_frames.push(Id3Frame::TOFN(None)); },
                "--TOLY" => { get_frames.push(Id3Frame::TOLY(None)); },
                "--TOPE" => { get_frames.push(Id3Frame::TOPE(None)); },
                "--TORY" => { get_frames.push(Id3Frame::TORY(None)); },
                "--TOWN" => { get_frames.push(Id3Frame::TOWN(None)); },
                "--TPE1" => { get_frames.push(Id3Frame::TPE1(None)); },
                "--TPE2" => { get_frames.push(Id3Frame::TPE2(None)); },
                "--TPE3" => { get_frames.push(Id3Frame::TPE3(None)); },
                "--TPE4" => { get_frames.push(Id3Frame::TPE4(None)); },
                "--TPOS" => { get_frames.push(Id3Frame::TPOS(None)); },
                "--TPRO" => { get_frames.push(Id3Frame::TPRO(None)); },
                "--TPUB" => { get_frames.push(Id3Frame::TPUB(None)); },
                "--TRCK" => { get_frames.push(Id3Frame::TRCK(None)); },
                "--TRDA" => { get_frames.push(Id3Frame::TRDA(None)); },
                "--TRSN" => { get_frames.push(Id3Frame::TRSN(None)); },
                "--TRSO" => { get_frames.push(Id3Frame::TRSO(None)); },
                "--TSIZ" => { get_frames.push(Id3Frame::TSIZ(None)); },
                "--TSO2" => { get_frames.push(Id3Frame::TSO2(None)); },
                "--TSOA" => { get_frames.push(Id3Frame::TSOA(None)); },
                "--TSOC" => { get_frames.push(Id3Frame::TSOC(None)); },
                "--TSOP" => { get_frames.push(Id3Frame::TSOP(None)); },
                "--TSOT" => { get_frames.push(Id3Frame::TSOT(None)); },
                "--TSRC" => { get_frames.push(Id3Frame::TSRC(None)); },
                "--TSSE" => { get_frames.push(Id3Frame::TSSE(None)); },
                "--TSST" => { get_frames.push(Id3Frame::TSST(None)); },
                "--TYER" => { get_frames.push(Id3Frame::TYER(None)); },
                "--UFID" => { get_frames.push(Id3Frame::UFID(None)); },
                "--USER" => { get_frames.push(Id3Frame::USER(None)); },
                "--WCOM" => { get_frames.push(Id3Frame::WCOM(None)); },
                "--WCOP" => { get_frames.push(Id3Frame::WCOP(None)); },
                "--WFED" => { get_frames.push(Id3Frame::WFED(None)); },
                "--WOAF" => { get_frames.push(Id3Frame::WOAF(None)); },
                "--WOAR" => { get_frames.push(Id3Frame::WOAR(None)); },
                "--WOAS" => { get_frames.push(Id3Frame::WOAS(None)); },
                "--WORS" => { get_frames.push(Id3Frame::WORS(None)); },
                "--WPAY" => { get_frames.push(Id3Frame::WPAY(None)); },
                "--WPUB" => { get_frames.push(Id3Frame::WPUB(None)); },
                "--COMM" => {
                    if i + 2 >= args.len() {
                        return Err(anyhow!("2 arguments expected after --COMM"));
                    }
                    let desc = Some(args[i + 1].clone());
                    let lang = Some(args[i + 2].clone());
                    get_frames.push(Id3Frame::COMM(desc, lang, None));
                    i += 2;
                }
                "--USLT" => {
                    if i + 2 >= args.len() {
                        return Err(anyhow!("2 arguments expected after --USLT"));
                    }
                    let desc = Some(args[i + 1].clone());
                    let lang = Some(args[i + 2].clone());
                    get_frames.push(Id3Frame::USLT(desc, lang, None));
                    i += 2;
                },
                "--TXXX" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TXXX"));
                    }
                    let desc = Some(args[i + 1].clone());
                    get_frames.push(Id3Frame::TXXX(desc, None));
                    i += 1;
                },
                "--WXXX" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --WXXX"));
                    }
                    let desc = Some(args[i + 1].clone());
                    get_frames.push(Id3Frame::WXXX(desc, None));
                    i += 1;
                },

                "--AENC=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --AENC="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::AENC(text));
                },
                "--APIC=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --AENC="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::APIC(text));
                },
                "--ASPI=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --ASPI="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::ASPI(text));
                },
                "--CHAP=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --CHAP="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::CHAP(text));
                },
                "--COMR=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --COMR="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::COMR(text));
                },
                "--CTOC=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --CTOC="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::CTOC(text));
                },
                "--ENCR=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --ENCR="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::ENCR(text));
                },
                "--EQU2=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --EQU2="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::EQU2(text));
                },
                "--ETCO=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --ETCO="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::ETCO(text));
                },
                "--GEOB=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --GEOB="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::GEOB(text));
                },
                "--GRID=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --GRID="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::GRID(text));
                },
                "--GRP1=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --GRP1="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::GRP1(text));
                },
                "--IPLS=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --IPLS="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::IPLS(text));
                },
                "--LINK=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --LINK="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::LINK(text));
                },
                "--MCDI=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --MCDI="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::MCDI(text));
                },
                "--MLLT=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --MLLT="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::MLLT(text));
                },
                "--MVIN=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --MVIN="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::MVIN(text));
                },
                "--MVNM=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --MVNM="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::MVNM(text));
                },
                "--OWNE=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --OWNE="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::OWNE(text));
                },
                "--PCNT=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --PCNT="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::PCNT(text));
                },
                "--PCST=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --PCST="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::PCST(text));
                },
                "--POPM=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --POPM="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::POPM(text));
                },
                "--POSS=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --POSS="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::POSS(text));
                },
                "--PRIV=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --PRIV="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::PRIV(text));
                },
                "--RBUF=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --RBUF="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::RBUF(text));
                },
                "--RVA2=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --RVA2="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::RVA2(text));
                },
                "--RVAD=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --RVAD="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::RVAD(text));
                },
                "--RVRB=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --RVRB="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::RVRB(text));
                },
                "--SEEK=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --SEEK="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::SEEK(text));
                },
                "--SIGN=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --SIGN="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::SIGN(text));
                },
                "--SYLT=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --SYLT="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::SYLT(text));
                },
                "--SYTC=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --SYTC="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::SYTC(text));
                },
                "--TALB=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TALB="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TALB(text));
                },
                "--TBPM=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TBPM="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TBPM(text));
                },
                "--TCAT=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TCAT="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TCAT(text));
                },
                "--TCMP=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TCMP="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TCMP(text));
                },
                "--TCOM=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TCOM="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TCOM(text));
                },
                "--TCON=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TCON="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TCON(text));
                },
                "--TCOP=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TCOP="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TCOP(text));
                },
                "--TDAT=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TDAT="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TDAT(text));
                },
                "--TDEN=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TDEN="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TDEN(text));
                },
                "--TDES=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TDES="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TDES(text));
                },
                "--TDLY=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TDLY="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TDLY(text));
                },
                "--TDOR=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TDOR="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TDOR(text));
                },
                "--TDRC=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TDRC="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TDRC(text));
                },
                "--TDRL=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TDRL="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TDRL(text));
                },
                "--TDTG=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TDTG="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TDTG(text));
                },
                "--TENC=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TENC="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TENC(text));
                },
                "--TEXT=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TEXT="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TEXT(text));
                },
                "--TFLT=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TFLT="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TFLT(text));
                },
                "--TGID=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TGID="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TGID(text));
                },
                "--TIME=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TIME="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TIME(text));
                },
                "--TIPL=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TIPL="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TIPL(text));
                },
                "--TIT1=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TIT1="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TIT1(text));
                },
                "--TIT2=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TIT2="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TIT2(text));
                },
                "--TIT3=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TIT3="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TIT3(text));
                },
                "--TKEY=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TKEY="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TKEY(text));
                },
                "--TKWD=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TKWD="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TKWD(text));
                },
                "--TLAN=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TLAN="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TLAN(text));
                },
                "--TLEN=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TLEN="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TLEN(text));
                },
                "--TMCL=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TMCL="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TMCL(text));
                },
                "--TMED=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TMED="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TMED(text));
                },
                "--TMOO=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TMOO="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TMOO(text));
                },
                "--TOAL=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TOAL="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TOAL(text));
                },
                "--TOFN=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TOFN="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TOFN(text));
                },
                "--TOLY=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TOLY="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TOLY(text));
                },
                "--TOPE=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TOPE="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TOPE(text));
                },
                "--TORY=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TORY="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TORY(text));
                },
                "--TOWN=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TOWN="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TOWN(text));
                },
                "--TPE1=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TPE1="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TPE1(text));
                },
                "--TPE2=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TPE2="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TPE2(text));
                },
                "--TPE3=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TPE3="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TPE3(text));
                },
                "--TPE4=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TPE4="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TPE4(text));
                },
                "--TPOS=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TPOS="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TPOS(text));
                },
                "--TPRO=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TPRO="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TPRO(text));
                },
                "--TPUB=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TPUB="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TPUB(text));
                },
                "--TRCK=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TRCK="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TRCK(text));
                },
                "--TRDA=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TRDA="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TRDA(text));
                },
                "--TRSN=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TRSN="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TRSN(text));
                },
                "--TRSO=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TRSO="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TRSO(text));
                },
                "--TSIZ=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TSIZ="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TSIZ(text));
                },
                "--TSO2=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TSO2="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TSO2(text));
                },
                "--TSOA=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TSOA="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TSOA(text));
                },
                "--TSOC=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TSOC="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TSOC(text));
                },
                "--TSOP=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TSOP="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TSOP(text));
                },
                "--TSOT=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TSOT="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TSOT(text));
                },
                "--TSRC=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TSRC="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TSRC(text));
                },
                "--TSSE=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TSSE="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TSSE(text));
                },
                "--TSST=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TSST="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TSST(text));
                },
                "--TYER=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --TYER="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::TYER(text));
                },
                "--UFID=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --UFID="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::UFID(text));
                },
                "--USER=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --USER="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::USER(text));
                },
                "--WCOM=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --WCOM="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::WCOM(text));
                },
                "--WCOP=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --WCOP="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::WCOP(text));
                },
                "--WFED=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --WFED="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::WFED(text));
                },
                "--WOAF=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --WOAF="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::WOAF(text));
                },
                "--WOAR=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --WOAR="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::WOAR(text));
                },
                "--WOAS=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --WOAS="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::WOAS(text));
                },
                "--WORS=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --WORS="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::WORS(text));
                },
                "--WPAY=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --WPAY="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::WPAY(text));
                },
                "--WPUB=" => {
                    if i + 1 >= args.len() {
                        return Err(anyhow!("1 argument expected after --WPUB="));
                    }
                    let text = Some(args[i + 1].clone());
                    set_frames.push(Id3Frame::WPUB(text));
                },
                "--COMM=" => {
                    if i + 3 >= args.len() {
                        return Err(anyhow!("3 arguments expected after --COMM="));
                    }
                    let desc = Some(args[i + 1].clone());
                    let lang = Some(args[i + 2].clone());
                    let text = Some(args[i + 3].clone());
                    set_frames.push(Id3Frame::COMM(desc, lang, text));
                    i += 3;
                }
                "--USLT=" => {
                    if i + 3 >= args.len() {
                        return Err(anyhow!("3 arguments expected after --USLT="));
                    }
                    let desc = Some(args[i + 1].clone());
                    let lang = Some(args[i + 2].clone());
                    let text = Some(args[i + 3].clone());
                    set_frames.push(Id3Frame::USLT(desc, lang, text));
                    i += 3;
                },
                "--TXXX=" => {
                    if i + 2 >= args.len() {
                        return Err(anyhow!("2 argument expected after --TXXX="));
                    }
                    let desc = Some(args[i + 1].clone());
                    let text = Some(args[i + 2].clone());
                    set_frames.push(Id3Frame::TXXX(desc, text));
                    i += 2;
                },
                "--WXXX=" => {
                    if i + 2 >= args.len() {
                        return Err(anyhow!("2 argument expected after --WXXX="));
                    }
                    let desc = Some(args[i + 1].clone());
                    let text = Some(args[i + 2].clone());
                    set_frames.push(Id3Frame::WXXX(desc, text));
                    i += 2;
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

        Ok(Cli {
            help,
            list_frames,
            get_frames,
            delimiter,
            null_delimited,
            set_frames,
        })
    }
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
        print_usage();
        return ExitCode::SUCCESS;
    }

    if cli.list_frames {
        Id3Frame::print_all();
        return ExitCode::SUCCESS;
    }

    println!("{cli:#?}");

    ExitCode::SUCCESS
}
