//! Introduction videos for topics. Each is a YouTube video that plays in
//! YouTube's own embedded player - the app never copies the file, so the
//! creator keeps their views and adverts and embedding stays within YouTube's
//! terms. The first video on a topic is the one to watch before the lesson;
//! the rest cover the remaining spec points.
//!
//! Built-in lists, read off YouTube on 16 September 2026:
//! - Computer Science: Craig'n'Dave's OCR GCSE (J277) specification-order
//!   playlist, one video per spec bullet.
//! - Maths: Corbettmaths' numbered topic videos (GCSE index plus the Level 2
//!   Further Maths page for functions and calculus), chosen per 4MA1 topic.
//! - Sciences: mostly FreeScienceLessons (from the topic pages on their site),
//!   with Cognito, Mr Exham (Edexcel IGCSE Biology) and a few others for the
//!   IGCSE-only topics the AQA-shaped channels skip.
//! - Business: Two Teachers, Halima Teaches (AQA 8132) and Bizconsesh.
//! - Economics: Mr Lee's Cambridge IGCSE chapter series, with ThinkIGCSE
//!   and EconplusDal alongside.
//! - English: Mr Bruff (a quick-revision video per poem and a 2026 guide per
//!   Language question), Mr Salles, Easy as GCSE and First Rate Tutors.
//! - Further Maths: TLMaths' A-level series and Corbettmaths' Further Maths
//!   page, with 1st Class Maths and Bicen Maths for the longer walkthroughs.
//! - French (AQA 8652): Collins Revision's AQA GCSE theme videos first, then
//!   Learn French With Alexa (grammar explainers and GCSE speaking topics) and
//!   her AQA-specific GCSE French With Alexa channel for the listening,
//!   dictation, photo card, reading, translation and writing tasks; with
//!   No Waffle GCSE, I'm Stuck, GCSE Online Courses, The EverLearner, The
//!   perfect French with Dylane, The Ideal Teacher Language School and
//!   astarfrench alongside.
//! - Spanish (AQA 8692): Collins Revision's AQA GCSE theme videos with
//!   astarspanish's topic revision and speaking practice; grammar from The
//!   Language Tutor, Señor Jordan and others (Spanish with James, SACAPUNTAS
//!   SPANISH GCSE, Spanish With Qroo Paul, Real Fast Spanish, Lingo Learner);
//!   exam skills from astarspanish and MyGCSESpanishTutor.
//! - German (AQA 8662): The Ideal Teacher Language School's GCSE listening
//!   practice and Learn German's vocabulary lessons for the themes; grammar
//!   from German Lessons with Herr Ferguson, mugridge language, GCSE German
//!   Tutorials, YourGermanTeacher, Bausteine eins and Learn German with Anja;
//!   exam skills from Learn German with Herr Reid and The Ideal Teacher.

use serde::Serialize;

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    /// The 11-character YouTube video id.
    pub id: String,
    pub title: String,
    /// Who made it, shown next to the player so the credit is theirs.
    pub by: String,
}

const CND: &str = "Craig'n'Dave";
const CM: &str = "Corbettmaths";
const ASTBURY: &str = "Mr Astbury";
const FSL: &str = "FreeScienceLessons";
const COG: &str = "Cognito";
const EXHAM: &str = "Mr Exham Biology";
const TWOT: &str = "Two Teachers";
const BIZC: &str = "Bizconsesh";
const HALIMA: &str = "Halima Teaches";
const MRLEE: &str = "Mr Lee - Business Econ";
const EPD: &str = "EconplusDal";
const THINK: &str = "ThinkIGCSE";
const BRUFF: &str = "Mr Bruff";
const SALLES: &str = "Mr Salles Teaches English";
const EASY: &str = "Easy as GCSE";
const FRT: &str = "First Rate Tutors";
const TLM: &str = "TLMaths";
const FIRSTCLASS: &str = "1st Class Maths";
const BICEN: &str = "Bicen Maths";

const ALEXA: &str = "Learn French With Alexa";
const ANJA: &str = "Learn German with Anja";
const ASTARES: &str = "astarspanish";
const BAUSTEINE: &str = "Bausteine eins";
const COLLINS: &str = "Collins Revision";
const DYLANE: &str = "The perfect French with Dylane";
const EVERLEARNER: &str = "The EverLearner";
const FERGUSON: &str = "German Lessons with Herr Ferguson";
const GCSEGER: &str = "GCSE German Tutorials";
const GCSEOC: &str = "GCSE Online Courses";
const GCSE_ALEXA: &str = "GCSE French With Alexa";
const HERRREID: &str = "Learn German with Herr Reid";
const IDEAL: &str = "The Ideal Teacher Language School";
const IMSTUCK: &str = "I'm Stuck - GCSE and A-Level Revision";
const JORDAN: &str = "Señor Jordan";
const LANGTUTOR: &str = "The Language Tutor - Spanish";
const LEARNGERMAN: &str = "Learn German";
const MUGRIDGE: &str = "mugridge language";
const MYGCSEES: &str = "MyGCSESpanishTutor";
const NOWAFFLE: &str = "No Waffle GCSE";
const QROOPAUL: &str = "Spanish With Qroo Paul";
const SACAPUNTAS: &str = "SACAPUNTAS SPANISH GCSE";
const YGT: &str = "YourGermanTeacher";

/// (topic id, [(video id, title, creator)]) in the order they should be watched.
const T2U: &str = "tutor2u";
const KED: &str = "Keducate";
const MRB: &str = "Mr B";
const HAWKS: &str = "Geography Hawks";
const AUDEN: &str = "Audenshaw Geography";
const GCS: &str = "Geography Case Studies";

const CLOKE: &str = "MrClokeHistory";
const HISTTEACH: &str = "The History Teacher";
const CHSG: &str = "CHSG History";
const MADDEN: &str = "Miss Madden's Awesome History Channel";
const PEARSONUK: &str = "Pearson UK & International Schools";

const FINLAYSON: &str = "Mr Finlayson";
const HARRIS: &str = "Harris Federation Religious Studies";
const WISEREV: &str = "Wise Revise";
const BBCTEACH: &str = "BBC Bitesize for Teachers";

const PEC: &str = "The PE Classroom";
const MRMATT: &str = "Mr Matthews | PE Tutor & Life Coach";
const PEIN10: &str = "PE in 10";
const PLANETPE: &str = "Planet PE";

const FTT: &str = "The Food Tech Teacher";
const FFL: &str = "Food - a fact of life";
const ILLUM: &str = "Illuminate Publishing";
const FSA: &str = "FoodStandardsAgency";

const MGENIE: &str = "Maths Genie";

const LWAS: &str = "Learn with a Scientist";
const STUDYMIND: &str = "Study Mind";

const CSTUTOR: &str = "The Computer Science Tutor";

const RBIRD: &str = "Revision Bird";
const APRG: &str = "AP Revision Guide";
const PETEJ: &str = "Pete Jackson";
const HRS: &str = "History Revision Success";
const HTUTOR: &str = "TheHistoryTutor";

const VIDEOS: &[(&str, &[(&str, &str, &str)])] = &[
    // ---------- Computer Science (OCR GCSE J277) ----------
    ("cs:1.1.1", &[
        ("7Up7DIPkTzo", "The purpose of the CPU - the fetch-execute cycle", CND),
        ("hk9LPXzYeT0", "CPU components and their function", CND),
        ("KBmoqwVt4Qg", "Von Neumann architecture", CND),
    ]),
    ("cs:1.1.2", &[("pZs_jfoxNLA", "Characteristics of CPUs", CND)]),
    ("cs:1.1.3", &[("WR242RfnsIo", "Embedded systems", CND)]),
    ("cs:1.2.1", &[
        ("dhQOkkZXu5w", "The need for primary storage", CND),
        ("Q2pzT6oYPWg", "RAM and ROM", CND),
        ("M31SS70Od08", "Virtual memory", CND),
    ]),
    ("cs:1.2.2", &[
        ("FNwA-h_tfPo", "The need for secondary storage", CND),
        ("qIy_wgo03Oo", "Common types of storage", CND),
        ("xfDwcdap5LA", "Suitable storage devices", CND),
    ]),
    ("cs:1.2.3", &[
        ("jBXWZbHPLWI", "Units of data storage", CND),
        ("T5gIiz0VeyI", "Processing binary data", CND),
        ("KzgbVfnJ7I4", "Data capacity calculations", CND),
    ]),
    ("cs:1.2.4", &[
        ("pCIUh20mNlA", "Converting between denary and 8-bit binary", CND),
        ("3K_Nw_pDzUQ", "Adding two 8-bit binary integers", CND),
        ("nmbr7GxN6TA", "Converting between denary and 2-digit hexadecimal", CND),
        ("B_3W5C7ppE4", "Binary shifts", CND),
        ("9oYV4JvSsok", "Representing characters", CND),
        ("6EfxuAOKZKc", "Representing images", CND),
        ("Ed7AFAzB8PM", "Representing sound", CND),
    ]),
    ("cs:1.2.5", &[("kOFA8FPL5kE", "Compression", CND)]),
    ("cs:1.3.1", &[
        ("KeN3H8_Jhbc", "Types of networks", CND),
        ("E_9mlCpmnuk", "Performance of networks", CND),
        ("w_LyIDAh_bY", "Client-server, peer-to-peer", CND),
        ("0VZz19fBOUQ", "LAN hardware", CND),
        ("u0uPibV0JOw", "The internet", CND),
        ("PR51Iu3DC88", "Star and mesh networks", CND),
    ]),
    ("cs:1.3.2", &[
        ("MeKllP5f-R8", "Modes of connection", CND),
        ("pe6Wbfl9qt4", "Wireless encryption", CND),
        ("p9D0Ca3VNpM", "IP and MAC addressing", CND),
        ("_xwKBxDs7aY", "Standards", CND),
        ("ncGIs1Wnxn8", "Common protocols", CND),
        ("S6Kwx5ZJpxg", "The concept of layers", CND),
    ]),
    ("cs:1.4.1", &[
        ("4f05t8ppJfk", "Forms of attack", CND),
        ("jlvvek8n5g8", "Threats to networks", CND),
    ]),
    ("cs:1.4.2", &[("XJEjQN-CEDk", "Preventing vulnerabilities", CND)]),
    ("cs:1.5.1", &[
        ("tArQQD4SZ7Q", "The purpose of operating systems", CND),
        ("dX9zhaBLJ7w", "Operating systems 1", CND),
        ("feECP9Q21Ow", "Operating systems 2", CND),
    ]),
    ("cs:1.5.2", &[("PW_T3UtaMgw", "Utility system software", CND)]),
    ("cs:1.6.1", &[
        ("NSFjAaGeJfY", "Investigating technologies", CND),
        ("6zTOHgTT9qw", "Privacy issues", CND),
        ("fHOHOqIdhh8", "Cultural issues", CND),
        ("g91-xCNv8-E", "Environmental issues", CND),
        ("X_NKAJ9j2Qs", "Impacts of technology on society", CND),
        ("fT_jls9bsoI", "Legislation", CND),
        ("49IVvPiiGP4", "Open source vs proprietary", CND),
    ]),
    ("cs:2.1.1", &[
        ("wLJ1n47sGRI", "Abstraction", CND),
        ("zcLlXCzb4IQ", "Decomposition", CND),
        ("5EsSYVP_eMU", "Algorithmic thinking", CND),
    ]),
    ("cs:2.1.2", &[
        ("SIOleZLPMb4", "Inputs, processes and outputs", CND),
        ("F6f6W7S9Y6k", "Structure diagrams", CND),
        ("MFojJssyKLw", "Pseudocode and diagrams", CND),
        ("I0e74jfo1Es", "Identifying errors and suggesting fixes", CND),
        ("zjJTKUDCSVU", "Trace tables", CND),
    ]),
    ("cs:2.1.3", &[
        ("pKW-hwvD2-A", "Binary search", CND),
        ("Hr5cP7LOUkU", "Linear search", CND),
        ("aOZBMnTswL8", "Bubble sort", CND),
        ("Y8y7PnlE4Dg", "Merge sort", CND),
        ("jmdP4Y-x0Hc", "Insertion sort", CND),
    ]),
    ("cs:2.2.1", &[
        ("dpBe_TXFqZ8", "Variables, constants, inputs, outputs and assignments", CND),
        ("t0VphK9cWgE", "The three basic programming constructs", CND),
        ("qozjsKdyBzM", "Arithmetic and comparison operators", CND),
        ("IILJVSOg6Oo", "Boolean operators", CND),
    ]),
    ("cs:2.2.2", &[("oQAQNKomako", "Data types and casting", CND)]),
    ("cs:2.2.3", &[
        ("u2DxYA3fwYA", "Basic string manipulation", CND),
        ("DFxeKaRuFcU", "Basic file handling", CND),
        ("es-zBs43VAs", "Records to store data", CND),
        ("X7aJuMJpQxM", "SQL to search for data", CND),
        ("izvYtCaD9EE", "Arrays", CND),
        ("rm19TvcXSvk", "How to use sub programs", CND),
        ("bni4XVtEJp4", "Random number generation", CND),
    ]),
    ("cs:2.3.1", &[
        ("2IIF4Infdf4", "Defensive design considerations 1", CND),
        ("8I0il6GQbTo", "Defensive design considerations 2", CND),
        ("YwW_fBw1eCY", "Maintainability", CND),
    ]),
    ("cs:2.3.2", &[
        ("IgOXjw76d0g", "The purpose and types of testing", CND),
        ("upt_QTi0id8", "How to identify syntax and logic errors", CND),
        ("FbnEBkN_Nko", "Suitable test data", CND),
        ("pBz6YoFID0Y", "Making algorithms more robust", CND),
    ]),
    ("cs:2.4.1", &[
        ("jN9WtjyjXf4", "Simple logic diagrams", CND),
        ("U7dbx9fllLc", "Truth tables", CND),
        ("M7h8XBjp0-s", "Combining Boolean operators", CND),
        ("vPG5c-RJtog", "Applying logical operators in truth tables", CND),
    ]),
    ("cs:2.5.1", &[
        ("4qNj8zIQzWw", "Characteristics and purpose of different languages", CND),
        ("BYFXtpIUPpY", "The purpose of translators", CND),
        ("-uaeVcs2XFs", "Compilers and interpreters", CND),
    ]),
    ("cs:2.5.2", &[("QPMrXhx784Y", "The IDE", CND)]),
    // ---------- Maths (Edexcel IGCSE 4MA1) — Corbettmaths topic videos ----------
    ("maths:1.1", &[
        ("veILCLvH198", "Number: product of primes", CM),
        ("oK-EFDLeEqc", "Number: product of primes (LCM/HCF)", CM),
        ("gASAc6_kBL4", "Number: common factors/HCF", CM),
        ("DW_yEg6VGac", "Number: common multiples/LCM", CM),
        ("MUVQY6Oo5x8", "Number: prime numbers", CM),
    ]),
    ("maths:1.2", &[
        ("lalcQLW6MWE", "Fractions: addition diff denominators", CM),
        ("cs8lcT5GVwc", "Fractions: multiplication", CM),
        ("4n7AoTkLHDg", "Fractions: division", CM),
        ("hqxjP3lYHiQ", "Fractions: improper to mixed number", CM),
        ("qPDOmGq81MA", "Fractions: ordering", CM),
    ]),
    ("maths:1.3", &[
        ("KZbKYokJ3SQ", "Decimals: recurring decimals", CM),
        ("mTY6qa6GhQo", "Fractions to decimals", CM),
        ("ziCjq2YrsE4", "Decimals: division by decimals", CM),
        ("ysaSwbOr69o", "Decimals: ordering", CM),
    ]),
    ("maths:1.4a", &[
        ("To04tERCUPs", "Indices (numerical)", CM),
        ("ozuXy8_NZcg", "Algebra: indices", CM),
        ("_K9XYyv1bU0", "Indices: negative", CM),
        ("qYDClSo89eQ", "Indices: fractional", CM),
        ("oSfs9mGO_lU", "Number: square root", CM),
    ]),
    ("maths:1.4b", &[
        ("ndU_cCbPAm4", "Surds: intro, rules, simplifying", CM),
        ("KGEt3H06L4g", "Surds: addition/subtraction", CM),
        ("n5dlWeOZh0Q", "Surds: expanding brackets", CM),
        ("96SwZpRvhwY", "Surds: rationalising denominators", CM),
    ]),
    ("maths:1.5", &[
        ("-rpZ4qyjBWg", "Set notation", CM),
        ("xwK--rNDI9E", "Venn Diagrams", CM),
    ]),
    ("maths:1.6", &[
        ("uHYywMkaZC8", "Percentages: multipliers", CM),
        ("tUtgC7ZrsRc", "Percentages: increasing\\decreasing", CM),
        ("Q2gRAS08fE0", "Percentages: change", CM),
        ("aG5zkrCiQpM", "Percentages: reverse", CM),
        ("FBCs95Co_oU", "Percentages: compound interest", CM),
    ]),
    ("maths:1.7", &[
        ("z7UWth70guM", "Ratio: simplifying", CM),
        ("cflZnf9H5l4", "Ratio: sharing the total", CM),
        ("boFsvYAySo8", "Ratio: given one value", CM),
        ("SJhomYlGPZ8", "Ratio: given two ratios", CM),
        ("UaGpRNifYpA", "Ratio: difference between", CM),
        ("w_J_WQnAqjE", "Ratio: express as fractions or %", CM),
    ]),
    ("maths:1.8", &[
        ("GGHHuAKoIcw", "Rounding: significant figures", CM),
        ("l00tsmfk8oQ", "Number: estimation", CM),
        ("ebMrP74boHw", "Limits of accuracy", CM),
        ("LdczMmb7J-g", "Limits of accuracy: applying", CM),
        ("FQ8IFKNhphM", "Error Intervals", CM),
    ]),
    ("maths:1.9", &[
        ("cxGyZ3Yx9ow", "Standard form", CM),
        ("Z0IQoVt-Brc", "Standard form: addition", CM),
        ("q4iVOJHIC_E", "Standard form: multiplication", CM),
        ("lvF33FY_tXg", "Standard form: division", CM),
    ]),
    ("maths:1.10", &[
        ("IdgIbKnHWIQ", "Time: calculations", CM),
        ("1az6Gjb2wtk", "Units: Metric units (length)", CM),
        ("nv26rIqbc4g", "Units: converting areas", CM),
        ("3ZopQfmTISw", "Number: currency", CM),
        ("DCwhNKgQX5U", "Number: best buys", CM),
    ]),
    ("maths:1.11", &[("DNKqOTp7PDI", "Use of a calculator", CM)]),
    ("maths:2.1", &[
        ("i4ygOKZKdl4", "Algebra: notation", CM),
        ("zxJNJMDj2Ec", "Algebra: collecting like terms", CM),
        ("2QhPDNqMmZY", "Algebra: multiplying terms", CM),
        ("_DQ7vs5tdUQ", "Algebra: dividing terms", CM),
        ("ozuXy8_NZcg", "Algebra: indices", CM),
    ]),
    ("maths:2.2a", &[
        ("ZJA1qz4XovY", "Algebra: expanding brackets", CM),
        ("nUxCCVox-Zo", "Algebra: expanding two brackets", CM),
        ("_2NvkxBchm8", "Algebra: expanding three brackets", CM),
        ("UrOJrsRv9iI", "Factorisation", CM),
        ("X-djBcWVizM", "Factorisation: quadratics", CM),
        ("8lejlzdWV58", "Factorisation: quadratics harder", CM),
        ("IqN8Z1-nlsY", "Factorisation: difference of 2 squares", CM),
    ]),
    ("maths:2.2b", &[
        ("tlKN8NNNxdI", "Algebraic fractions: simplifying", CM),
        ("w3JewxYjiNs", "Algebraic fractions: addition", CM),
        ("93Y8hCoOAj0", "Algebraic fractions: multiplication", CM),
        ("c89f9IewdkI", "Algebraic fractions: division", CM),
        ("VS6apvTv2Rc", "Algebra: completing the square", CM),
        ("E08s12dSClo", "Completing the square with ax²", CM),
        ("pd9Q-e1JvtE", "Algebraic Proof", CM),
    ]),
    ("maths:2.3", &[
        ("ZkC2FX5TOJ8", "Algebra: substitution", CM),
        ("8U9u_itcs7k", "Algebra: changing the subject", CM),
        ("MKMSa-X6Rgw", "Algebra: changing the subject advanced", CM),
    ]),
    ("maths:2.4", &[
        ("30S7WxKcPwg", "Equations: solving", CM),
        ("85ZM3ZKqRhY", "Equations: letters both sides", CM),
        ("kaIqgpKV4Cc", "Equations: involving fractions", CM),
        ("Lz3VkLrDmhE", "Equations: forming", CM),
    ]),
    ("maths:2.5", &[
        ("kcOwC7uqJNE", "Proportion: direct", CM),
        ("uZ6l-loSdRs", "Proportion: inverse", CM),
        ("pXnyxf55nJU", "Proportion: Graphs", CM),
    ]),
    ("maths:2.6", &[
        ("phlus4x0UqM", "Simultaneous equations (elimination)", CM),
        ("FrECUQpaa80", "Simultaneous equations (substitution, both linear)", CM),
        ("20R39KaGwmc", "Simultaneous equations (graphical)", CM),
    ]),
    ("maths:2.7a", &[
        ("wJ_tLEwEEi8", "Quadratics: solving (factorising)", CM),
        ("MtHEk6Yy6N4", "Factorisation: splitting the middle", CM),
        ("3J0ccr74LcU", "Quadratics: formula", CM),
        ("VS6apvTv2Rc", "Algebra: completing the square", CM),
        ("NGILmqWOdSc", "Quadratic graphs: sketching using key points", CM),
    ]),
    ("maths:2.7b", &[
        ("-AQVy-MPdRU", "Quadratics: forming and solving", CM),
        ("ozP-vf99DK4", "Simultaneous equations: one linear, one quadratic", CM),
    ]),
    ("maths:2.8", &[
        ("OjgdLu0JaZo", "Inequalities", CM),
        ("-YcE_D78fGE", "Inequalities: solving (one sign)", CM),
        ("zTXiZ6SiiUs", "Inequalities: solving (two signs)", CM),
        ("8J_m-hMp8lY", "Inequalities: quadratic", CM),
        ("aexvnpH-jhI", "Inequalities: regions", CM),
    ]),
    ("maths:3.1a", &[
        ("qnVVTBAfNu4", "Sequences: nth term", CM),
        ("slLDExh-VXY", "Sequences: nth term for fractional sequences", CM),
        ("AL-joUBnEIw", "Quadratic nth term – Version 1", CM),
        ("vZl9L0c-Zkg", "Quadratic nth term – Version 2", CM),
    ]),
    ("maths:3.1b", &[
        ("Zolv3DQL5WI", "Proof of the sum of an arithmetic series", CM),
        ("P3-GQi1EjDQ", "Arithmetic sequences and series: Edexcel IGCSE exam questions", ASTBURY),
    ]),
    ("maths:3.2", &[
        ("akj9L0HaTY4", "Function Machines", CM),
        ("u1YQVzrgYDg", "Composite functions", CM),
        ("zpF9nbjResY", "Inverse functions", CM),
        ("6fDmLeS6Zzo", "Functions: domain and range", CM),
        ("6m4wFYD5vFw", "Functions: drawing", CM),
    ]),
    ("maths:3.3a", &[
        ("HdlnBX82jxI", "Linear graphs: y=mx+c", CM),
        ("YtHJP1rZ3pI", "Linear graphs: gradient of a line", CM),
        ("HxTkMsfWkME", "Linear graphs: gradient between points", CM),
        ("WdSXjD0bxI4", "Linear graphs: find equation of a line", CM),
        ("MJKPASvp0qY", "Linear graphs: equation through 2 points", CM),
        ("DWyaGrTA884", "Linear graphs: parallel lines", CM),
        ("PrwhdgnLK5k", "Linear graphs: perpendicular lines", CM),
    ]),
    ("maths:3.3b", &[
        ("LVhJzITdIH0", "Types of graph: cubics", CM),
        ("kTTTkMwXqrg", "Types of graph: reciprocal", CM),
        ("cp_nviIhgUw", "Types of graph: exponential", CM),
        ("OJEpC8pFxec", "Trigonometry: Sine graph", CM),
        ("XxQlu2jqLvc", "Trigonometry: Cosine graph", CM),
        ("d7SdnQXgr04", "Trigonometry: Tangent graph", CM),
    ]),
    ("maths:3.3c", &[
        ("eiRZATuHYg0", "Transformations of graphs", CM),
        ("SAG6B4Q1lwc", "Transformations of trigonometric graphs", CM),
        ("7C3f-sYMNCU", "Quadratics: solving graphically", CM),
        ("_ks2K1FuB80", "Quadratics: solving graphically advanced", CM),
    ]),
    ("maths:3.4", &[
        ("Kp6uup02CDc", "Calculus: introduction to differentiation", CM),
        ("yhNKawQHBIk", "Differentiation", CM),
        ("5K1LHvDNl8Y", "Differentiation after rearranging", CM),
        ("cEp7qD6vCSM", "Gradient of a curve", CM),
        ("N_FM6aON2z8", "Equation of a tangent to a curve", CM),
        ("9zYN1fUt2CM", "Second derivative", CM),
        ("8aPSaDNhJpk", "Stationary points", CM),
        ("9GkYv-vTEOU", "Application of differentiation", CM),
    ]),
    ("maths:4.1", &[
        ("dqg1DQCJa-E", "Angles: types of", CM),
        ("WmNfXG30opI", "Angles: straight line", CM),
        ("DriZsOZ-xXE", "Angles: vertically opposites", CM),
        ("QEsjIeSnEHU", "Angles: triangle", CM),
        ("mM-PU6hmkrg", "Angles: parallel lines", CM),
    ]),
    ("maths:4.2", &[
        ("gVo8ZrtlSp0", "Angles: polygons", CM),
        ("y9udtA3cPgU", "Angles: quadrilaterals", CM),
        ("D37lgOSY6w0", "2D shapes: quadrilaterals", CM),
        ("HA6gjFLm2Gk", "Congruent shapes", CM),
        ("IDW1ogTqox8", "Congruent triangles", CM),
    ]),
    ("maths:4.3", &[
        ("rNURPkUEvg0", "Symmetry: line", CM),
        ("YUCUcFBuFbU", "Symmetry: rotational", CM),
    ]),
    ("maths:4.4", &[
        ("o8DSb6D-0fw", "Speed, distance and time", CM),
        ("sv7zflLeduM", "Density", CM),
        ("fheOYg9TKQA", "Pressure", CM),
        ("8Wja7Ct_XvY", "Angles: bearings", CM),
        ("WXctwgaS2_s", "Angles: given bearings from two points", CM),
        ("vEiqRJ2LxFE", "Angles: back bearings", CM),
    ]),
    ("maths:4.5", &[
        ("1beKcgU9ogE", "Constructions: perpendicular bisector", CM),
        ("fBGOshZk94U", "Constructions: angle bisector", CM),
        ("EudHqBNyoWM", "Constructions: perpendicular: point on line", CM),
        ("o13HKzmYSUA", "Constructions: triangles SSS", CM),
        ("BWj041al8z8", "Constructions: loci part 1", CM),
        ("0OnH72_OFaE", "Constructions: loci part 2", CM),
        ("34CV0SnRoGg", "Constructions: loci part 3", CM),
        ("2PZ41oDEZ_Q", "Scales and maps", CM),
    ]),
    ("maths:4.6", &[
        ("vgMSLsos7Ew", "Circle theorems – theorems", CM),
        ("4I70hg61pS4", "Circle theorems – examples", CM),
        ("aNLwD4yyL0I", "Circle theorems proof: cyclic quadrilaterals", CM),
        ("U33XHR9faUE", "Circle theorems proof: alternate segment theorem", CM),
        ("y7-yT5qUtN0", "Circle theorems proof: angles at circumference\\centre", CM),
    ]),
    ("maths:4.7", &[("4YdhDXJWCZ8", "Geometric Proof", CM)]),
    ("maths:4.8a", &[
        ("iWLVTy_rGjs", "Pythagoras", CM),
        ("7mbN6HntdkE", "Pythagoras: rectangles/isosceles tri", CM),
        ("WWf3MnQ1SwU", "Trigonometry: introduction", CM),
        ("F_uTDZtRe0I", "Trigonometry missing sides", CM),
        ("I2MpcUZNQD0", "Trigonometry missing angles", CM),
        ("UjgOR07zOzY", "Trigonometry: Exact values", CM),
    ]),
    ("maths:4.8b", &[
        ("An_kU2n_3RY", "Trigonometry: sine rule (sides)", CM),
        ("ISxiacGy6oA", "Trigonometry: sine rule (angles)", CM),
        ("3H3u92WJAjw", "Trigonometry: cosine rule (sides)", CM),
        ("Q0quAR-kAZg", "Trigonometry: cosine rule (angles)", CM),
        ("eSFOMSxjMts", "Trigonometry: area of a triangle", CM),
        ("RHdFb2QhYCE", "Trigonometry: sine rule (ambiguous case)", CM),
    ]),
    ("maths:4.8c", &[
        ("Fk0Z-ArGMxE", "Pythagoras: 3D", CM),
        ("JJMZJFWMrpM", "Trigonometry: 3D", CM),
    ]),
    ("maths:4.9", &[
        ("6UCVcYnjBG4", "Perimeter", CM),
        ("Of8p1SfOcR0", "Area: circles", CM),
        ("jWX9KNToIcA", "Area: trapezium", CM),
        ("qiTmz3UtUiY", "Area: compound shapes", CM),
        ("XGhc_4ilUko", "Circles: arc length", CM),
        ("jmFw7xZNZ_I", "Area: sector", CM),
        ("fPDa9DUm25U", "Circles: segment area", CM),
    ]),
    ("maths:4.10", &[
        ("hi2QMbROemk", "Surface area: mixture", CM),
        ("rGc00WJaqV0", "Volume: prism", CM),
        ("ExALFZ3mP0Y", "Volume: cylinder", CM),
        ("X6cMvcxk1ig", "Volume: cone", CM),
        ("Tr1wXAWZuc8", "Volume: pyramid", CM),
        ("vFRuk6z49BE", "Volume: sphere", CM),
        ("i5OPtpu_4k8", "Volume: Frustum", CM),
        ("uFhUrWncKF8", "Surface area: sphere", CM),
        ("drGyc_JPYUc", "Surface area: cone", CM),
    ]),
    ("maths:4.11", &[
        ("L6DLoBMknoY", "Similar shapes: finding sides", CM),
        ("GlD88EpEJVo", "Similar shapes: area", CM),
        ("QMI90tONvzQ", "Similar shapes: volume", CM),
    ]),
    ("maths:5.1", &[
        ("h02d922Q5wk", "Vectors: Column", CM),
        ("xOdkldbusy0", "Vectors", CM),
    ]),
    ("maths:5.2", &[
        ("AE0w7QRjGqQ", "Reflections", CM),
        ("rgdRlbbWQgA", "Rotations", CM),
        ("qb4ElyditqY", "Translations: vector", CM),
        ("7362afSFdtw", "Enlargements", CM),
        ("22zNVcV_iKQ", "Enlargements: fractional scale factor", CM),
        ("YvpJN-h3bu0", "Enlargements: negative scale factor", CM),
    ]),
    ("maths:6.1a", &[
        ("U785Y-QI-K8", "Tables: two-way tables", CM),
        ("sdMT6iasnYQ", "Graphs: pie charts (draw)", CM),
        ("SxSewF7E1-0", "Graphs: pie charts (interpret)", CM),
        ("VUaOCgJTPjI", "Graphs: scatter graphs (draw)", CM),
        ("hlGrp8X3XyY", "Graphs: scatter graphs (correlation)", CM),
        ("cOgLCMQNVNU", "Graphs: frequency polygons (draw)", CM),
    ]),
    ("maths:6.1b", &[
        ("wGzp-wM90EU", "Graphs: histograms (draw)", CM),
        ("vhcikRsrdJo", "Graphs: histograms (interpret)", CM),
        ("VYa31tlstr0", "Graphs: histograms harder", CM),
    ]),
    ("maths:6.1c", &[
        ("FCnqp-Z9FC4", "Graphs: cumulative frequency (draw)", CM),
        ("DwM6MrGq1h4", "Graphs: cumulative frequency (reading)", CM),
        ("z41_PBqYuVg", "Graphs: box plots- draw\\interpret", CM),
        ("Q2OF86ZUYMs", "Graphs: box plots (compare)", CM),
    ]),
    ("maths:6.2", &[
        ("x8oPXIrLMc0", "Averages: mean", CM),
        ("zGbCFis_XpI", "Averages: mean (frequency table)", CM),
        ("7QReTFK2hD4", "Averages: mean (estimated)", CM),
        ("dZZu3sDVU5A", "Averages: median (grouped data)", CM),
        ("WYJBdAPyFmg", "Averages: Quartiles", CM),
        ("MkjfricztdA", "Averages: range", CM),
    ]),
    ("maths:6.3a", &[
        ("ur_hHjLrBNo", "Probability: basic", CM),
        ("Xqno7W0OUtE", "Probability: sample space", CM),
        ("y9T5ol65mSU", "Probability: not happening", CM),
        ("achhCbzYMdQ", "Probability: OR rule", CM),
        ("mUDqgCe-PAo", "Probability: independent events", CM),
        ("MS6lnCTgTSw", "Probability: relative frequency", CM),
    ]),
    ("maths:6.3b", &[
        ("PYEvSuz1Dxo", "Probability: tree diagrams", CM),
        ("xhFDlmQUAZo", "Probability: conditional", CM),
        ("E3rVM0OgyJE", "Frequency Trees", CM),
    ]),
    // ---------- Sciences (Edexcel IGCSE 4BI1 / 4CH1 / 4PH1) — FreeScienceLessons, Cognito, Mr Exham and others ----------
    ("bio:1a", &[
        ("aGDFNZApXXI", "Characteristics of Living Things (Organisms)", COG),
        ("Xzy4Ze93G3g", "Kingdoms of life: animals, plants, fungi, protoctists, bacteria and viruses", COG),
        ("S2O6sVcUtLU", "Classification", FSL),
        ("Yk3ooRgDVI4", "Eukaryotes and Prokaryotes", FSL),
    ]),
    ("bio:2a", &[
        ("MB6mE6weCS4", "Levels of Organisation  - Cells, Tissues, Organs and Organ Systems", COG),
        ("GuY0n7-zfds", "Animal Cells", FSL),
        ("EAoeI2gXBRg", "Plant Cells", FSL),
        ("UZwT-Jx8LzY", "Animal Cell Specialisation", FSL),
        ("yVd9Z3av1Ew", "Plant Cell Specialisation", FSL),
        ("Kh27eyjxvYM", "Stem Cells", FSL),
    ]),
    ("bio:2b", &[
        ("VLK2wANjQm0", "Digestive Enzymes", FSL),
        ("Rfvh4LIsEEM", "Effect of Temperature and pH on Enzymes", FSL),
        ("SqWTJWOBww4", "Required Practical 4: Food Tests", FSL),
        ("JyXXoevEWc8", "Required Practical 5: Effect of pH on Amylase", FSL),
    ]),
    ("bio:2c", &[
        ("C5pMigXBAgk", "Diffusion", FSL),
        ("qqe2NhQt8bY", "Osmosis", FSL),
        ("BXTi5tbnOr0", "Active Transport", FSL),
        ("DHGWH3NdAjc", "Surface Area to Volume Ratio", FSL),
        ("ef2Ts2AKhq8", "Required Practical 3: Effects of Osmosis on Plant Tissue", FSL),
    ]),
    ("bio:2d", &[
        ("rAJGnS_ktk4", "Photosynthesis", FSL),
        ("kx7AeCx_6xQ", "Limiting Factors", FSL),
        ("Q5rsuwMDCXY", "Uses of Glucose from Photosynthesis", FSL),
        ("c9aUWHleZ_k", "Structure of a Leaf | Plant Cell Organisation", COG),
        ("cBCKedXdFeE", "Required Practical 6: Photosynthesis", FSL),
    ]),
    ("bio:2e", &[
        ("4ui4oSHHnzA", "The Digestive System", FSL),
        ("VLK2wANjQm0", "Digestive Enzymes", FSL),
        ("5VW5-VXlWic", "Absorption in the Small Intestine", FSL),
        ("H6DrSG_KQjo", "Lifestyle and Disease", FSL),
    ]),
    ("bio:2f", &[
        ("ZKAaDbTP6Dc", "Respiration", FSL),
        ("xO2XlIMnLuM", "Exercise", FSL),
        ("7ZQQUi2DPMw", "Metabolism", FSL),
    ]),
    ("bio:2g", &[
        ("v0HDxCmJ6DY", "Gas Exchange in Flowering Plants", EXHAM),
        ("6zl6xaXTxxk", "Stomata & Guard Cells", "Launchpad Learning"),
        ("c9aUWHleZ_k", "Structure of a Leaf | Plant Cell Organisation", COG),
    ]),
    ("bio:2h", &[
        ("aPUPfzsqDgs", "Gas Exchange in the Lungs", FSL),
        ("Nn4ke02sW8Q", "The Lungs & Gas Exchange", COG),
        ("H6DrSG_KQjo", "Lifestyle and Disease", FSL),
    ]),
    ("bio:2i", &[
        ("2BR1zdMBhY4", "Plant Tissues", FSL),
        ("9yTDokLRZs0", "Transpiration", FSL),
        ("s06FvGH3QJo", "Transpiration & Translocation", COG),
    ]),
    ("bio:2j", &[
        ("nc_kbfjhiUo", "The Blood", FSL),
        ("wUm71FPuVCQ", "Pathogens", FSL),
        ("5X9MklLVhlw", "Non-Specific Defence Systems", FSL),
        ("HSrrPdJDqxM", "The Immune System", FSL),
        ("uPeZBhJYlnU", "Vaccination", FSL),
    ]),
    ("bio:2k", &[
        ("bpYaKM2hVFY", "The Heart and Circulation", FSL),
        ("Wx-MrhlOFMk", "Arteries, Veins and Capillaries", FSL),
        ("5wSfCZESRHU", "Cardiovascular Diseases", FSL),
    ]),
    ("bio:2l", &[
        ("DbLVB_EDnRs", "The Kidneys", FSL),
        ("kmRh_yRbAR4", "Maintaining the Body's Water Balance", FSL),
    ]),
    ("bio:2m", &[
        ("S45_3wWL-Xk", "Homeostasis", FSL),
        ("WoMPARSQPZw", "Thermoregulation", FSL),
        ("_Mts354VC7A", "Negative Feedback", FSL),
        ("_Bf5WKEMB5o", "Plant Hormones", FSL),
        ("fEo21LbnJJM", "Required Practical 8: Plant Responses", FSL),
        ("6boD9x0MMcs", "Uses of Plant Hormones", FSL),
    ]),
    ("bio:2n", &[
        ("oDS1hAqWp2M", "The Nervous System", FSL),
        ("Fm02i4vEi5Q", "Required Practical 7: Reaction Time", FSL),
        ("G_clJP1VGtk", "The Eye", FSL),
        ("QYHlHr_S5fg", "How the Eye Focuses", FSL),
    ]),
    ("bio:2o", &[
        ("c6olhi88KZs", "The Endocrine System", FSL),
        ("77oyUdNZ054", "Control of Blood Glucose Concentration", FSL),
        ("4CxNeiAICmc", "Hormones to Treat Infertility", FSL),
    ]),
    ("bio:3a", &[
        ("g2Y_IlEWXyE", "Plant Reproduction | Anatomy & Pollination", COG),
        ("h077JEQ8w6g", "Plant reproduction - Flower anatomy and pollination", EXHAM),
        ("GkzFimUJdD8", "Flower structure and insect pollination", "Science Sauce"),
        ("Fh9b6a-3DLQ", "Sexual and Asexual Reproduction", FSL),
    ]),
    ("bio:3b", &[
        ("Fh9b6a-3DLQ", "Sexual and Asexual Reproduction", FSL),
        ("w5SRMZlYR4w", "Meiosis and Fertilisation", FSL),
        ("iXswGsfeHJg", "The Menstrual Cycle", FSL),
    ]),
    ("bio:3c", &[
        ("TQ_iCf8mzMA", "DNA and the Genome", FSL),
        ("o4LHU79fB3s", "DNA Structure", FSL),
        ("1GgNNYZ47rk", "Protein Synthesis", FSL),
    ]),
    ("bio:3d", &[
        ("reVLRjZIh3c", "Alleles", FSL),
        ("Q4hSQJ0bl9g", "Cystic Fibrosis", FSL),
        ("oYEr8wIe5G0", "Polydactyly", FSL),
        ("wky7R3zYtTQ", "Family Trees", FSL),
        ("Lomr_t5Pdjs", "Inheritance of Sex", FSL),
        ("n3cXcDEveRc", "Mendel and Genetics", FSL),
    ]),
    ("bio:3e", &[
        ("I0VdEiPWkHs", "Cell division by Mitosis", FSL),
        ("w5SRMZlYR4w", "Meiosis and Fertilisation", FSL),
    ]),
    ("bio:3f", &[
        ("_LoPYfhTgeI", "Variation", FSL),
        ("RXmpVboM040", "Mutations", FSL),
        ("7RraYCKvTXc", "Evolution by Natural Selection", FSL),
        ("2waYa0ZwoXg", "Darwin and Natural Selection", FSL),
        ("L8XYxNqEJqI", "Evidence for Evolution: Resistant Bacteria", FSL),
    ]),
    ("bio:4a", &[
        ("ePsjdKoSA9g", "Competition and Interdependence", FSL),
        ("kIfMwZU8nk4", "Biotic and Abiotic Factors", FSL),
        ("KvK7EJimAH8", "Adaptations", FSL),
        ("2MW6nwf80XM", "Sampling Organisms", FSL),
        ("yLHz2Ea10Mg", "Required Practical 9: Sampling Organisms", FSL),
    ]),
    ("bio:4b", &[
        ("dRFQ8rZCK6Q", "Food Chains and Predator-Prey Cycles", FSL),
        ("AFC5LQ3KnvU", "Trophic Levels", FSL),
        ("sgh1OWm0oTQ", "Pyramids of Biomass", FSL),
    ]),
    ("bio:4c", &[
        ("cWj3u8voDSg", "The Carbon Cycle", FSL),
        ("vWZWPlFmua4", "The Nitrogen Cycle", FSL),
        ("UrP1E-yM7Cs", "Cycles Within Ecosystems - Nitrogen Cycle", EXHAM),
        ("6utMftGxuaI", "Decomposition", FSL),
    ]),
    ("bio:4d", &[
        ("7hu6vDP2a4Q", "Global Warming", FSL),
        ("K5vXnDGcOE4", "The Greenhouse Effect", FSL),
        ("pXCXXTgLoLE", "Pollution - Eutrophication", EXHAM),
        ("40o6Py7W1rs", "Deforestation & Land Use", COG),
        ("1Z405uGDZGo", "Waste Management", FSL),
    ]),
    ("bio:5a", &[
        ("MaWBxZQ8nHQ", "Food Production - Crop Plants and Greenhouses", EXHAM),
        ("7rmAZ-NYoBQ", "Food Production - Fertilisers", EXHAM),
        ("uCxj4Bs0E3A", "Food Production - Pest Control (Pesticides and Biological Control)", EXHAM),
        ("nrbJl3R4YJU", "Modern Farming Methods", FSL),
    ]),
    ("bio:5b", &[
        ("Ii-RkMwFSlQ", "Food Production - Biotechnology - Fermenters", EXHAM),
        ("ejyhTqAPVtI", "Food Production - Biotechnology - Yoghurt Production", EXHAM),
        ("hYlNIuiTm4k", "Food Production - Biotechnology - Bread", EXHAM),
        ("hcnDYP6tZs0", "Fermenters and yoghurt making", "Tom Dare"),
        ("jwnMfxDLYpY", "Food Production - Fish Farming", EXHAM),
        ("u59Eg1uNr5g", "Sustainable Fisheries", FSL),
    ]),
    ("bio:5c", &[("99nEQd2k6k4", "Selective Breeding", FSL)]),
    ("bio:5d", &[
        ("gu9T91GJXDo", "Genetic Engineering", FSL),
        ("4Wu86ACPTKY", "Genetic Engineering | GMO", COG),
        ("6C6lPfQbtek", "Role of Biotechnology", FSL),
    ]),
    ("bio:5e", &[
        ("uSY6m1gqtYc", "Cloning Plants - Micropropagation (tissue culture)", EXHAM),
        ("QekStThHD2M", "Cloning Plants", FSL),
        ("hNq-y2Kg5CE", "Cloning Animals", FSL),
    ]),
    ("chem:1a", &[
        ("CTwJEtjYffY", "The Three States of Matter", FSL),
        ("lxHMJaXOzP4", "What is diffusion?", COG),
    ]),
    ("chem:1b", &[
        ("7AZ2Z6_CQmA", "Solubility Curves", "FuseSchool - Global Education"),
        ("yYPVrK5ic5E", "Solubility Curves Explained", "Chemistry Simplified"),
    ]),
    ("chem:1c", &[
        ("nUzOXy9V-K0", "Elements, Compounds and Mixtures", FSL),
        ("r49wo5ficzg", "Filtration and Crystallisation", FSL),
        ("wXxFg7tdPjw", "Simple Distillation", FSL),
        ("XKOgDiFNkiA", "Fractional Distillation", FSL),
        ("dsKz9eF1Sc0", "Paper Chromatography", FSL),
        ("3oJxWwcnfJY", "Purity and Formulations", FSL),
    ]),
    ("chem:1d", &[
        ("cI2Shr8nns8", "The Nuclear Model of Atomic Structure", FSL),
        ("nyvVjJf7RAU", "Atomic Number and Mass Number", FSL),
        ("yOsMN89wIQc", "Relative Atomic Mass", FSL),
        ("NuUDkub7N9A", "Electron Energy Levels", FSL),
        ("IoldeyRWgz8", "Development of the Periodic Table", FSL),
        ("EReyx5QoUSs", "Group 0", FSL),
        ("7wguWXOZ8dc", "Metals", FSL),
    ]),
    ("chem:1e", &[
        ("Lsa1h7EeZ_M", "Interpreting a Chemical Formula", FSL),
        ("vxCyzR6uETs", "Balancing Chemical Equations", FSL),
        ("q49NwIrjaFw", "Relative Formula Mass", FSL),
        ("tV8Cv2x0SD0", "Formula of Ionic Compounds", FSL),
        ("K4pw_-U6Xpc", "Conservation of Mass", FSL),
    ]),
    ("chem:1f", &[
        ("-_-fNVmDwJk", "Calculating Moles of an Element", FSL),
        ("Md4BQL91U6w", "Calculating Moles of a Compound", FSL),
        ("kMak1TQ3YgU", "Calculating Mass of a Number of Moles", FSL),
        ("TV6n5MFH6IU", "Reacting Masses 1", FSL),
        ("5zOpoeN0dV0", "Reacting Masses 2", FSL),
        ("MuzOmFhiE8o", "Limiting reactant", FSL),
        ("9EV0Oq8g708", "Calculating Percentage Yield 1", FSL),
        ("A3ndfwX5lyI", "Calculating Percentage Yield 2", FSL),
    ]),
    ("chem:1g", &[
        ("u2V8b7M_caA", "Empirical Formula", FSL),
        ("JXHjWpo3Yxg", "Determining Empirical Formula from Reacting Masses", FSL),
        ("VaXQVoI3gtc", "Calculating Percentage by Mass", FSL),
    ]),
    ("chem:1h", &[
        ("XbDtmORzKO8", "Ionic Bonding 1: Ionic Bonding between Group 1 and Group 7", FSL),
        ("9HvMkqn6_Pc", "Ionic Bonding 2: Ionic Bonding between Group 2 and Group 6", FSL),
        ("V28_L3gteDo", "Charges on Ions", FSL),
        ("3hUwVYOue5s", "Properties of Ionic Compounds", FSL),
    ]),
    ("chem:1i", &[
        ("m5u4STdFlOE", "Covalent Bonding 1: Bonding in Hydrogen, Chlorine and Hydrogen chloride", FSL),
        ("4SNY8yK4gzw", "Covalent Bonding 2: Bonding in Water, Ammonia and Methane", FSL),
        ("xvYA7KBLipY", "Covalent Bonding 3: Bonding in Oxygen, Nitrogen and Carbon Dioxide", FSL),
        ("u_KR0UaZFkY", "Properties of Small Covalent Molecules", FSL),
    ]),
    ("chem:1j", &[
        ("gUNkLFf2WXU", "Diamond and Silicon Dioxide", FSL),
        ("iPoPeYHctPs", "Graphite", FSL),
        ("cjgODRJU79Y", "Graphene and Fullerenes", FSL),
    ]),
    ("chem:1k", &[("o56nWrsp-hI", "Metals and Alloys", FSL)]),
    ("chem:1l", &[
        ("AhTRiL6xjBA", "Introducing Electrolysis", FSL),
        ("YcyMElBEzAY", "Electrolysis of Aluminium Oxide", FSL),
        ("6WjC_Vi4roA", "Electrolysis of Aqueous Solutions 1", FSL),
        ("mL7mkqyLpSo", "Electrolysis of Aqueous Solutions 2", FSL),
        ("ukbtTTG1Kew", "Required Practical 3: Electrolysis", FSL),
        ("gnbuTl2ariI", "Oxidation and Reduction in Terms of Electrons", FSL),
    ]),
    ("chem:2a", &[
        ("Z9U0728-fPY", "Group 1 Part 1", FSL),
        ("PxdVHydF_U4", "Group 1 Part 2", FSL),
    ]),
    ("chem:2b", &[
        ("5l-5sKDudq8", "Group 7 Part 1 The Halogens", FSL),
        ("eBlWCl0wx4c", "Group 7 Part 2 Compounds of the Halogens", FSL),
        ("yxL0xvfBy3k", "Group 7 Part 3 Reactivity of the Halogens", FSL),
    ]),
    ("chem:2c", &[
        ("t1Z3GlNldLA", "The Atmosphere", FSL),
        ("Lk1V0buHEFs", "Reaction of Metals with Oxygen", FSL),
        ("8PM_tWNFbGY", "Combustion of Hydrocarbons", FSL),
        ("K5vXnDGcOE4", "The Greenhouse Effect", FSL),
    ]),
    ("chem:2d", &[
        ("MDQr5QFVGkk", "The Reactivity Series", FSL),
        ("ofw6oHSYGFI", "Acids Reacting with Metals", FSL),
        ("iA4mk3CTkmI", "Acids Reacting with Metals 2", FSL),
        ("q0CAfXV-YdY", "What is Corrosion and How to Stop it", COG),
        ("Gl1ctcnUnJ0", "Corrosion", FSL),
    ]),
    ("chem:2e", &[
        ("MXTSels6e2Y", "Extraction of Metals", FSL),
        ("YcyMElBEzAY", "Electrolysis of Aluminium Oxide", FSL),
        ("o56nWrsp-hI", "Metals and Alloys", FSL),
    ]),
    ("chem:2f", &[
        ("ZWZTDiwOWiI", "Acids and Alkalis", FSL),
        ("4pIHhXfGZlE", "Strong and Weak Acids", FSL),
    ]),
    ("chem:2g", &[
        ("QlSsle_jSQ8", "Three Reactions of Acids", FSL),
        ("ofw6oHSYGFI", "Acids Reacting with Metals", FSL),
        ("9GH95172Js8", "Required Practical 1: Making Soluble Salts", FSL),
        ("saRBT5oZfh8", "Required Practical 2: Carrying out a Titration", FSL),
        ("x8DLLCNMKAs", "Titration calculations 1", FSL),
    ]),
    ("chem:2h", &[
        ("Qf7mHlTi5rs", "Tests for Cations, Anions & Water", "IGCSE Science Revision"),
        ("jVkGKurtaiE", "Testing for Gases", FSL),
        ("Bd0A44Iv2OI", "Flame Tests", FSL),
        ("dBvpd9RhX8E", "Metal Hydroxide Precipitates", FSL),
        ("n1SiWOIJayI", "Identifying non-metal ions", FSL),
        ("YGdArTxQVq0", "Testing For Water", "FuseSchool - Global Education"),
    ]),
    ("chem:3a", &[
        ("4HS6D0hTzdg", "Exothermic and Endothermic Reactions", FSL),
        ("rdI7xEq4Ew8", "Required Practical 4: Temperature Changes", FSL),
        ("eExCBkp4jB4", "Bond Energy Calculations 1", FSL),
        ("PdValXAVUOc", "Bond Energy Calculations 2", FSL),
    ]),
    ("chem:3b", &[
        ("CLq7WzCmYrk", "Mean Rate of Reaction", FSL),
        ("u4Co4N-Jmbs", "Effect of Concentration on Rate", FSL),
        ("WojotwxPD6I", "Effect of Surface Area on Rate", FSL),
        ("G2TEfhwgq84", "Effect of Temperature on Rate", FSL),
        ("hel8fQjxcO8", "Catalysts", FSL),
        ("N5p06i9ilmo", "Required Practical 5: Rates of Reaction", FSL),
        ("6LV63WtuvJg", "Using Tangents to Determine Rate", FSL),
    ]),
    ("chem:3c", &[
        ("66qcNNJFy6E", "Reversible Reactions", FSL),
        ("utmV4Q0t6MI", "Concentration and Reversible Reactions", FSL),
        ("SlI5m0RQqik", "Temperature and reversible reactions", FSL),
        ("hngzmRrAXTE", "Pressure and Reversible Reactions", FSL),
    ]),
    ("chem:4a", &[
        ("2ATXC_weN7s", "Hydrocarbons - Alkanes & Homologous Series", COG),
        ("ZSAtCBvBDBE", "What are isomers?", "IGCSE World"),
        ("4EAh9E2KhOE", "Properties of Hydrocarbons", FSL),
    ]),
    ("chem:4b", &[
        ("CX2IYWggEBc", "Crude oil and Hydrocarbons", FSL),
        ("3I7yCkSXPos", "Fractional Distillation of Crude Oil", FSL),
    ]),
    ("chem:4c", &[
        ("8PM_tWNFbGY", "Combustion of Hydrocarbons", FSL),
        ("yLp6LOgPHmI", "Pollutants from Fuels", FSL),
        ("14XIKuOZ2Yw", "Fossil Fuels", FSL),
    ]),
    ("chem:4d", &[("7AWwjKbRa_o", "Cracking", FSL)]),
    ("chem:4e", &[
        ("2ATXC_weN7s", "Hydrocarbons - Alkanes & Homologous Series", COG),
        ("4EAh9E2KhOE", "Properties of Hydrocarbons", FSL),
    ]),
    ("chem:4f", &[
        ("CmANMHeeZgw", "Alkenes", FSL),
        ("3ZLpmNJu-e0", "Reactions of Alkenes 1", FSL),
        ("ZHjFAxS0ivI", "Chemistry \"Reactions of Alkenes 2", FSL),
    ]),
    ("chem:4g", &[
        ("uFZasZ-hs_A", "Alcohols", FSL),
        ("w3IBYDJ5XmM", "Reactions of Alcohols", FSL),
    ]),
    ("chem:4h", &[
        ("ketAGS1gkQM", "Carboxylic Acids", FSL),
        ("cYgRd4rXY6I", "Esters", COG),
        ("oIcICtyw-fA", "Alcohols, carboxylic acids and esters", "Science with Hazel"),
    ]),
    ("chem:4i", &[
        ("GhvevdJU_DM", "Addition Polymers", FSL),
        ("QBuSFPOtcJ4", "Condensation Polymers", FSL),
        ("Y0eDQ0dOpTM", "Bonding in Polymers", FSL),
    ]),
    ("phys:1a", &[
        ("DkCw2C-DkT0", "Distance-Time Graphs", FSL),
        ("VJefeYJL3uE", "Velocity-Time Graphs - How to Find Acceleration & Distance Travelled", COG),
        ("M_0FRIX8wIM", "Speed", FSL),
        ("09aDQcci_tQ", "Velocity", FSL),
    ]),
    ("phys:1b", &[
        ("r5iXzDCRMsE", "Acceleration", FSL),
        ("qpqWzTwnwUk", "Acceleration 2", FSL),
        ("M_0FRIX8wIM", "Speed", FSL),
    ]),
    ("phys:1c", &[
        ("P1lSWWUkMdQ", "Scalar and Vector Quantities", FSL),
        ("xxK8N23nx9M", "Contact and Non-contact Forces", FSL),
        ("PL8ATKipoB4", "Resultant Forces", FSL),
        ("PG8wV022Eu0", "Vector Diagrams", FSL),
    ]),
    ("phys:1d", &[
        ("_W3VbonFNcw", "Newton's First Law of Motion", FSL),
        ("SqdCCxv9YzI", "Newton's Second Law of Motion", FSL),
        ("wANmggaC9pY", "Newton's Third Law of Motion", FSL),
        ("W2aBVbcHr_k", "Gravity and Weight", FSL),
        ("VOMNGlasL-0", "Required Practical 7: Acceleration", FSL),
    ]),
    ("phys:1e", &[
        ("drMKdcMq3o0", "Vehicle Stopping Distance", FSL),
        ("AiXhR2eZxgo", "Force and Braking", FSL),
        ("aVy_gNVaCGg", "Forces Acting on a Skydiver", FSL),
        ("k3AYX5INJ_4", "Terminal Velocity - What Affects Air Resistance | Resultant Force & Acceleration", COG),
    ]),
    ("phys:1f", &[
        ("ACDbJ8rsQDo", "Forces and Elasticity", FSL),
        ("jQAt3e6Bz7U", "Required Practical 6: Stretching a Spring", FSL),
        ("Qw_9kX9PARc", "Elastic Potential Energy", FSL),
    ]),
    ("phys:1g", &[
        ("ZtQhlwPxE28", "Momentum", FSL),
        ("YEHcQD6Hij8", "Conservation of momentum", FSL),
        ("6yx0fQrK3fA", "Change in Momentum", FSL),
    ]),
    ("phys:1h", &[
        ("0RXm47J196Q", "Moments", FSL),
        ("RW2N-oPcM9g", "Balanced Moments", FSL),
        ("p7QS4cz-Avs", "How Moments Work - Spanners and Seesaws", COG),
        ("YTGql3Ilu9E", "Centre of gravity", "Pla Academy: IGCSE and A level buddy"),
    ]),
    ("phys:2a", &[
        ("cx9xLwa7Gco", "Resistance", FSL),
        ("2CA1mcYw3IQ", "Resistors", FSL),
        ("CEBfn4ndQWI", "Current in Series Circuits", FSL),
        ("YsZeZotYVag", "Required Practical 3: Resistance", FSL),
    ]),
    ("phys:2b", &[
        ("CEBfn4ndQWI", "Current in Series Circuits", FSL),
        ("JhBrAmQYr2g", "Current in Parallel Circuits", FSL),
        ("YAzyHRusOS0", "Potential Difference in Series Circuits", FSL),
        ("UM1jyQVdGD8", "Potential Difference in Parallel Circuits", FSL),
        ("vJRXozSVTI8", "Resistors in Series and Parallel", FSL),
    ]),
    ("phys:2c", &[
        ("WzSh6ykqn9I", "Resistance of a Filament Lamp", FSL),
        ("Tk_OltwtxZE", "Diodes and LEDs", FSL),
        ("bb7sRiLKCvg", "Light-Dependent Resistors", FSL),
        ("bjt4CrRL8yM", "Thermistors", FSL),
        ("A1SyKvdHoqY", "Required Practical 4: Current / PD Characteristics", FSL),
    ]),
    ("phys:2d", &[
        ("gj1tu8bTKjI", "Energy Transfer by Appliances", FSL),
        ("WLaUmNr4lho", "Calculating Energy Transferred by Appliances", FSL),
        ("LOyJdI41aCU", "Power of Components", FSL),
        ("MEvO2rQFIWk", "DC and AC Supply", FSL),
        ("fbu3o9wavHk", "Mains Electricity", FSL),
    ]),
    ("phys:2e", &[
        ("ts7WumFAaSg", "Charge in Circuits", FSL),
        ("WAMyh1zVtyU", "Calculating Energy Transfer by Components", FSL),
    ]),
    ("phys:2f", &[
        ("5obbfXg_MH4", "Static Electricity", FSL),
        ("rPbx_XrrKLQ", "Electric Fields", FSL),
    ]),
    ("phys:3a", &[
        ("0f5iYCNCnow", "Transverse and Longitudinal Waves", FSL),
        ("ITe6snlZBp8", "Properties of Waves", FSL),
        ("Aucu7YshyQ0", "The Wave Equation", FSL),
        ("3qCmEHRFRH8", "Properties of Waves 2", FSL),
        ("UNmv6H-f180", "Required Practical 8: Ripple Tank", FSL),
    ]),
    ("phys:3b", &[
        ("u5vkYjV1V1A", "Electromagnetic Waves", FSL),
        ("L0iivb-acqU", "Uses of EM waves", FSL),
    ]),
    ("phys:3c", &[
        ("8K6gOST8pZk", "Reflection of Waves", FSL),
        ("wO49W5lsP0s", "Refraction of Waves", FSL),
        ("2fN_jvf4fw8", "Required practical 9: Reflection and Refraction", FSL),
    ]),
    ("phys:3d", &[
        ("UUc44Vg5pCI", "Refraction of waves", COG),
        ("sxAlityiJWY", "Total internal reflection", "Save My Exams"),
        ("bIUbj3Bh2LI", "Total internal reflection", "Physics With Mr Drew"),
        ("55p9yBCoIyw", "Critical angle", "Physics With Mr Drew"),
    ]),
    ("phys:3e", &[
        ("N_07EkzEhVQ", "Sound Waves", FSL),
        ("s9wZkP64rAc", "Sound Waves and Hearing", COG),
        ("MzZmgk1Hjs8", "How to read an oscilloscope", "Physics Online"),
        ("YI3_wsPRi6Q", "Measuring the speed of sound with an oscilloscope", "Launchpad Learning"),
    ]),
    ("phys:4a", &[
        ("JGwcDCeYRYo", "Energy Stores, Transferring Energy & Work Done", COG),
        ("lbp01HgTBNc", "Energy Transfers: Pendulum", FSL),
        ("7ZlTNAUNFts", "Energy Transfers: Bungee Jumper", FSL),
    ]),
    ("phys:4b", &[
        ("NI5jaeBrIgQ", "Efficiency", FSL),
        ("L8vz1MQsuys", "The Sankey diagram", "Physics Online"),
        ("NC8ItrcR2Ak", "Sankey diagrams", "Ace Physics and Maths"),
    ]),
    ("phys:4c", &[
        ("rUnABMRPzvg", "Conduction, Convection & Radiation | How Heat Energy is Transferred", COG),
        ("je-qc7sxYzU", "How Radiation Affects Temperature", COG),
        ("GTdgI-0KckA", "Cooling of Buildings", FSL),
        ("lLH45loyPUA", "Required Practical 2: Thermal Insulators", FSL),
    ]),
    ("phys:4d", &[
        ("JHEmPZ-YnrU", "Work Done by a Force", FSL),
        ("EDT0DPhaaMY", "Calculating Power", FSL),
        ("-zy9eWzmGe4", "Kinetic Energy", FSL),
        ("63OTIdNb-TE", "Gravitational Potential Energy", FSL),
        ("PY80j_iNT9Y", "Work done and Energy Transfer", FSL),
    ]),
    ("phys:4e", &[
        ("1dJKvxhGEgA", "Energy from Fossil Fuels", FSL),
        ("ar3-Ps04AJI", "Nuclear Power", FSL),
        ("pqzvUur7QRw", "Renewable Sources of Energy", FSL),
        ("lA8USjDkcXk", "The UK Energy Mix", FSL),
    ]),
    ("phys:5a", &[
        ("-EZmXVOSa20", "Density", FSL),
        ("ScXOp8Zph28", "Required Practical 5: Density", FSL),
        ("P08-lYPy1hI", "Pressure in Fluids", FSL),
        ("SVB6CjbTIAI", "Floating or Sinking", FSL),
    ]),
    ("phys:5b", &[
        ("Hs5x0-IU2F4", "Specific Heat Capacity", FSL),
        ("HAPmwu7byGM", "Required Practical 1: Specific Heat Capacity", FSL),
        ("vJgBIvuLvgY", "Heating and Cooling Graphs", FSL),
        ("x7GZ2DXef84", "Specific Latent Heat", FSL),
        ("5WVT5NR0iLA", "Internal Energy", FSL),
    ]),
    ("phys:5c", &[
        ("zjkBMk5d3tM", "Particle Theory & States of Matter | Solids, Liquids & Gases", COG),
        ("hKO3DpgiISk", "Particle Motion in Gases", FSL),
        ("JVlWh4vofsk", "Absolute zero", "GCSE Physics Explained"),
    ]),
    ("phys:5d", &[
        ("RuoZqmNiMEo", "Pressure in Gases", FSL),
        ("NxD7L4B7fRE", "Pressure & Volume | pV = Constant Equation", COG),
        ("HbmZmxxempQ", "Gas laws: Boyle's, Charles' and the pressure law", "Save My Exams"),
        ("m19-8Vtewkw", "Work Done on a Gas", FSL),
    ]),
    ("phys:6a", &[
        ("sRyy7-jEu3Q", "Permanent and Induced Magnets", FSL),
        ("FodEDHaEY68", "Magnetic Fields", FSL),
        ("dMbWkodL12I", "Electromagnets", FSL),
        ("V1cTPQxN4K0", "Electromagnetic Devices", FSL),
    ]),
    ("phys:6b", &[
        ("GNLhSKZh-jM", "The Motor Effect", FSL),
        ("fiQ38p6vb8o", "The Electric Motor", FSL),
        ("1DqWMHyRhYg", "Loudspeakers and Headphones", FSL),
    ]),
    ("phys:6c", &[
        ("NjgqJahwsG0", "The Generator Effect", FSL),
        ("k1IivkRjd1U", "The Alternator and Dynamo", FSL),
        ("UsZsns63Km4", "The Microphone", FSL),
    ]),
    ("phys:6d", &[
        ("M9ytpIMB5d8", "Transformers", FSL),
        ("_16o6j6YlXY", "Transformer Calculations", FSL),
        ("iNvGiTn64fQ", "The National Grid", FSL),
    ]),
    ("phys:7a", &[
        ("dftq9xGXcf8", "Atomic Structure", FSL),
        ("k8cLFDa8zmY", "Atomic and Mass Numbers", FSL),
        ("0ASldDQmIOQ", "Alpha-Scattering and the Nuclear Model", FSL),
        ("F_Y1-JieCrg", "Radioactivity", FSL),
        ("nW0S1C6wVrg", "Properties of Alpha, Beta and Gamma Radiation", FSL),
    ]),
    ("phys:7b", &[
        ("xpSBhUpBXic", "Nuclear Equations", FSL),
        ("F_Y1-JieCrg", "Radioactivity", FSL),
    ]),
    ("phys:7c", &[
        ("wj9BzGFao8k", "Half Life", FSL),
        ("Z7394DMkfQs", "Background Radiation", FSL),
    ]),
    ("phys:7d", &[
        ("teGu0VAPlOo", "Irradiation and Contamination", FSL),
        ("YejvYYRjSUk", "Nuclear Radiation in Medicine", FSL),
    ]),
    ("phys:7e", &[
        ("onkW8BF5I3Q", "Nuclear Fission and Nuclear Fusion", FSL),
        ("ar3-Ps04AJI", "Nuclear Power", FSL),
    ]),
    ("phys:7f", &[("onkW8BF5I3Q", "Nuclear Fission and Nuclear Fusion", FSL)]),
    ("phys:8a", &[
        ("mndRVjMovQk", "The Solar System", FSL),
        ("okMA18ppu98", "Orbital Motion", FSL),
        ("W2aBVbcHr_k", "Gravity and Weight", FSL),
    ]),
    ("phys:8b", &[
        ("V0Y1JlVuin4", "Lifecycle of Stars", FSL),
        ("V69KZun35K8", "Life Cycle of Stars | How Stars are Formed & Destroyed", COG),
    ]),
    ("phys:8c", &[
        ("DaehJctk0Iw", "Hertzsprung-Russell diagrams", "Science with Hazel"),
        ("C90DOE87TYc", "Red-Shift", FSL),
        ("bWEtm-7cYzM", "What is Red Shift?", COG),
    ]),
    // ---------- Business (AQA GCSE 8132) — Two Teachers, Halima Teaches, Bizconsesh and others ----------
    ("bus:3.1.1", &[
        ("yPuPVWfWdK4", "The role of enterprise", "TakingTheBiz"),
        ("xzVJVdY-Dhk", "What is an entrepreneur?", BIZC),
        ("GiiOu8mTHus", "The Purpose Of Business & Enterprise", "BizzWizard"),
    ]),
    ("bus:3.1.2", &[
        ("BN2cQNNvg_4", "Types of Business Ownership : Sole Traders, Partnerships, LTD, PLC and Franchise", TWOT),
        ("-yvTvtN_9e4", "Business Ownership Structures", BIZC),
    ]),
    ("bus:3.1.3", &[
        ("OzWTEe4bna4", "Business Aims and Objectives Explained", TWOT),
        ("cPeUX5qmU3Y", "Why Business Aims & Objectives Change | Sainsbury's Examples", TWOT),
        ("nvsNq3ri7ks", "Setting aims and objectives", HALIMA),
    ]),
    ("bus:3.1.4", &[
        ("tZGol4xtY3g", "Stakeholders | What is a Stakeholder?", TWOT),
        ("FlKDvWasUCg", "Business stakeholders", HALIMA),
        ("36a0PtQ5sGs", "What are Stakeholders?", BIZC),
    ]),
    ("bus:3.1.5", &[
        ("eU2VMJ2d1ks", "Factors Influencing Business Location Explained", TWOT),
        ("xClseet9SMo", "Business Location Factors", BIZC),
    ]),
    ("bus:3.1.6", &[
        ("q0IiqfVyrjY", "Business planning", HALIMA),
        ("O0lXFwG5o3w", "Business Plans", BIZC),
        ("rUAsm4Szqw8", "Revenue, cost and profit", "Business Teacher T"),
    ]),
    ("bus:3.1.7", &[
        ("e0DWrnTZW3I", "Expanding a business", HALIMA),
        ("I3orIItbKW0", "Economies and diseconomies of scale", "The Secondary Scholar"),
        ("OBTeXJPeqTc", "Diseconomies of Scale", BIZC),
    ]),
    ("bus:3.2.1", &[
        ("SxaBwx682U8", "Technology in Business", HALIMA),
        ("8SLixwLnpTc", "Impact of using E-Commerce", BIZC),
        ("CXJHSmCaxVY", "Digital Communication and Stakeholders", BIZC),
    ]),
    ("bus:3.2.2", &[
        ("Ko-S6U7a6zk", "Ethical and environmental considerations", HALIMA),
        ("A9i8dwKC7TE", "Business Ethics | The Impact of Ethics on Business", TWOT),
        ("XrqPA_Pr0GY", "Environmental Considerations", BIZC),
    ]),
    ("bus:3.2.3", &[
        ("Og2HQ1Bv65s", "Economy and Business | How the Economic Climate Impacts Businesses Explained!", TWOT),
        ("KK4oNhhp2Ts", "Consumer Income", BIZC),
        ("5Upb3buctBM", "Impact of Unemployment", BIZC),
    ]),
    ("bus:3.2.4", &[
        ("-loRR8XBeDw", "Globalisation", BIZC),
        ("-Vk2kuji44M", "Exchange Rates", BIZC),
        ("D2G51WsQNn4", "Exchange Rate Impacts", BIZC),
    ]),
    ("bus:3.2.5", &[
        ("jZWzzqv6CHo", "The Impact of Legislation on Businesses | Legislation & Business", TWOT),
        ("gNpaWXTP7Jc", "Employment Law | The 4 Key Principles Explained", TWOT),
        ("crMRgS2LyV0", "Consumer Law", BIZC),
    ]),
    ("bus:3.2.6", &[
        ("SX1lu2ZJNYI", "Competitive Environment", HALIMA),
        ("ephvKaL2ZoU", "Risk and Uncertainty Explained", "tutor2u"),
        ("pzwwpurAHR0", "Competitive Environment", BIZC),
        ("uv7cUS67Fo4", "Impact of Competition", BIZC),
    ]),
    ("bus:3.3.1", &[
        ("m8Ou6fGTBcQ", "Production Process", HALIMA),
        ("Zlf-YsnDDYg", "Production processes", "Mastery Mind"),
        ("FMidebp7kaA", "Just in Time - JIT - Pros and Cons", BIZC),
    ]),
    ("bus:3.3.2", &[
        ("W6CdBnlHt8U", "What is Procurement? What is Logistics?", BIZC),
        ("1RTBzazmX70", "What is Supply Chain Management?", BIZC),
        ("XTE0rDy48zw", "Understanding Procurement & Logistics", "BizzWizard"),
    ]),
    ("bus:3.3.3", &[
        ("lf74Oc-D1zE", "Managing Quality : Quality Control & Quality Assurance", TWOT),
        ("tZ6g9UztPb8", "Quality Control (QC)", BIZC),
        ("P6FcEmQ2BF0", "Quality Assurance (QA)", BIZC),
        ("1WwcJUylPNg", "Benefits of Quality", BIZC),
    ]),
    ("bus:3.3.4", &[
        ("vcGCVH2g0dA", "Good customer service", HALIMA),
        ("DHukPz033Hc", "What is Customer Service?", BIZC),
        ("CYWOiH6MVH0", "Sales Process", BIZC),
        ("gCH3fUa2Heo", "The Sales Process Explained", "Business Teacher T"),
    ]),
    ("bus:3.4.1", &[
        ("_Y9jgBtmapw", "Models of Organisational Structure - Functional, Regional, Product & Matrix", BIZC),
        ("ZsJ6Rbg6SWU", "Centralised Structures vs. Decentralised Structures", BIZC),
        ("zvhjDlu8VIc", "Understanding Organisational Structures", "BizzWizard"),
    ]),
    ("bus:3.4.2", &[
        ("hHXlsJ2VQ70", "Recruitment and Selection | The Recruitment and Selection Process Explained", TWOT),
        ("xdhopNi5yIc", "Recruitment and selection", HALIMA),
        ("XEqGPjxgLqE", "Recruitment and Selection Process", BIZC),
    ]),
    ("bus:3.4.3", &[
        ("XtnH0nPRcxw", "Financial & Non-Financial Methods of Motivation", BIZC),
        ("nDT-kduw9VQ", "Methods of Financial & Non-Financial Motivation", "BizzWizard"),
    ]),
    ("bus:3.4.4", &[
        ("BuaJwz6Dtn0", "Training", HALIMA),
        ("ojmYJVLAzp4", "On The Job vs. Off The Job Training", BIZC),
        ("r4Db2Lqsl1o", "Induction Training", BIZC),
    ]),
    ("bus:3.5.1", &[
        ("_hw4K9lu_vQ", "Identifying and understanding customer needs", HALIMA),
        ("PLKBVCZjXUw", "Customer Needs", BIZC),
        ("ZzIfjvILvk4", "Customer Needs Explained", "Business Teacher T"),
    ]),
    ("bus:3.5.2", &[
        ("mt3rkutNtNI", "What is Segmentation?", BIZC),
        ("8i0yxc-P0j4", "Market Segmentation In Under 4 Minutes - With Examples!", "Business Teacher T"),
        ("LkVyZSfg6xE", "Market Segmentation Explained", "BizzWizard"),
    ]),
    ("bus:3.5.3", &[
        ("S_-bLwHwcoU", "Primary & Secondary Market Research", BIZC),
        ("sfNhIyFiLao", "What is Market Research?", "BizzWizard"),
        ("E4rCRsgvhKE", "Market Research: Primary vs Secondary Research", THINK),
    ]),
    ("bus:3.5.4", &[
        ("JC8lGW1T1bY", "Marketing Mix", BIZC),
        ("nd5KWzHMA5k", "Product Life Cycle", BIZC),
        ("qHsTLbEfKgg", "The Boston Matrix : Tesla Examples", TWOT),
        ("IpkibcN7sRw", "Boston Matrix", BIZC),
    ]),
    ("bus:3.6.1", &[
        ("DAZi6XcTZzE", "Sources of Business Finance : Bank Loans, Trade Credit, Share Capital, Overdrafts & More", TWOT),
        ("i760YLhlV0Q", "Internal Finance and External Finance", BIZC),
        ("epxzAvSJUkA", "Sources of finance: internal vs external", "Dean Hoss"),
    ]),
    ("bus:3.6.2", &[
        ("4SNWA_HbF6U", "Cash Flow Forecasting : How to Complete a Cash Flow Forecast Example", TWOT),
        ("UmJ9dOF4vHQ", "What is a Cash Flow Forecast?", BIZC),
        ("hif6NwAcxPI", "Why Cash Flow forecasting is useful?", BIZC),
    ]),
    ("bus:3.6.3", &[
        ("UB8aIchQ8j4", "Break-even analysis", "Business 101"),
        ("mMu2I2zBY2Q", "Average Rate of Return (ARR)", BIZC),
        ("qYsZcElRiX4", "Average Rate of Return (ARR)", BIZC),
        ("HOnN_Qj3z04", "Break-Even Formula - To Learn!", BIZC),
    ]),
    ("bus:3.6.4", &[
        ("DM7TqljUues", "Net profit and gross profit: formulas and margin calculations", TWOT),
        ("Zp7ku0kEbto", "Ratio analysis: gross profit margin and net profit margin", BIZC),
        ("4IGsYndj9ms", "How to calculate Gross Profit & Net Profit", BIZC),
    ]),
    // ---------- Economics (Cambridge IGCSE 0987) — Mr Lee's chapter series, ThinkIGCSE, EconplusDal and others ----------
    ("econ:1.1", &[
        ("Nvy1sEKrtYU", "The basic economic problem", MRLEE),
        ("f0MoZpGI5VQ", "The nature of the basic economic problem: scarcity, wants and resources", "Bahruz - IGCSE Business"),
        ("W9IjktFC9Tg", "The Economic Problem (Scarcity & Choice)", EPD),
    ]),
    ("econ:1.2", &[
        ("AQ0SBU-wbX0", "Factors of production: land, labour, capital and enterprise", "Bahruz - IGCSE Business"),
        ("jOWFasMbwSc", "Factors of Production", "Study with Milya"),
        ("Wljl3KKemDQ", "What are the factors of production?", BIZC),
    ]),
    ("econ:1.3", &[
        ("5JgpjOxUcxM", "Opportunity cost", "Mr Goff"),
        ("jkmhldTxHRg", "Opportunity Cost", "Study with Milya"),
        ("eUvgrKRhSBs", "Opportunity cost", "Sir Usman | Economics & Business"),
    ]),
    ("econ:1.4", &[
        ("QqrLaONBVDk", "Production Possibility Curves", "Study with Milya"),
        ("IzccVWouIxM", "Production Possibility Curves - PPCs / PPFs", EPD),
        ("Uc9VhlYC5eg", "Production Possibility Curve (PPC) - Chapter 4 - Economics IGCSE", "TaleWhale TV"),
    ]),
    ("econ:2.1", &[
        ("pvcHbtRErgk", "The role of markets in allocating resources", MRLEE),
        ("KF4dcCX3kK8", "The role of markets in allocating resources", "Bahruz - IGCSE Business"),
        ("6wjvoYdotnw", "Role of Markets in Allocating Resources", "Study with Milya"),
    ]),
    ("econ:2.2", &[
        ("kzPo5OU5QPI", "Demand & Shifts in the Demand Curve", MRLEE),
        ("aH_XC6EAzXE", "Demand and the Demand Curve", EPD),
        ("9jLlOPqHxLs", "Change in Demand vs. Change in Quantity Demanded", "Marginal Revolution University"),
    ]),
    ("econ:2.3", &[
        ("C1em1Rfl0hI", "Supply and shifts in the supply curve", MRLEE),
        ("3lUFSA-nY2s", "Supply: supply curves, market supply and shifts in supply", "Bahruz - IGCSE Business"),
        ("TvBHJERKto0", "What Shifts the Supply Curve?", "Marginal Revolution University"),
    ]),
    ("econ:2.4", &[
        ("u8EawjnkfpA", "Price determination", MRLEE),
        ("R1frAHwOEbU", "Price determination: market equilibrium and disequilibrium", "Bahruz - IGCSE Business"),
        ("BZqxagFuuHg", "Market Equilibrium & Disequilibrium", EPD),
    ]),
    ("econ:2.5", &[
        ("nZiNTQ_qQmA", "Price changes", MRLEE),
        ("pBsdr7riV88", "Price Changes", "Study with Milya"),
    ]),
    ("econ:2.6", &[
        ("BkQDHjpFW98", "Price elasticity of demand", MRLEE),
        ("d5OdfkmHwVk", "Price Elasticity of Demand", THINK),
        ("nOlOf_KEnrw", "Price Elasticity of Demand - PED", EPD),
    ]),
    ("econ:2.7", &[
        ("qZuQzd1GB9U", "Price elasticity of supply", MRLEE),
        ("3By-uf1cSTE", "Price Elasticity of Supply", "Study with Milya"),
        ("ICjglEvPL44", "Price Elasticity of Supply (PES)", EPD),
    ]),
    ("econ:2.8", &[
        ("OobdxjpOOJM", "The market economic system and market failure", MRLEE),
        ("ujJEe8spxec", "The market economic system: advantages and disadvantages", "Bahruz - IGCSE Business"),
        ("jqMo7tTx9T8", "Market Economic System: Advantages and Disadvantages", THINK),
    ]),
    ("econ:2.9", &[
        ("OobdxjpOOJM", "The market economic system and market failure", MRLEE),
        ("2HU2ZLRGyOM", "Types of Market Failure", EPD),
        ("wiHjVeX3DKc", "Merit and De-Merit Goods - Imperfect Information", EPD),
        ("fQy9mVR3I1o", "Public Goods", EPD),
        ("CytzEvsPKhY", "Merit goods, demerit goods and externalities", "EnhanceTuition"),
    ]),
    ("econ:2.10", &[
        ("GurGAR9Hliw", "The mixed economic system", MRLEE),
        ("78BP6XRywAM", "Government Intervention", MRLEE),
        ("9EseSEgLsvU", "What is a Mixed Economy?", "Mr. Sinn"),
    ]),
    ("econ:3.1", &[
        ("nCAcVqSqOxg", "Money and banking", MRLEE),
        ("h6hHbJMFyIA", "Money and banking: functions of money, commercial and central banks", "Bahruz - IGCSE Business"),
        ("cqGkm6qtRWg", "Central Banks and Commercial Banks Compared in One Minute", "One Minute Economics"),
    ]),
    ("econ:3.2", &[
        ("XMzawZNS2cQ", "Households", MRLEE),
        ("eef_YaBZGQQ", "Influences on Spending, Saving, and Borrowing", THINK),
        ("dCffj4OkRaQ", "Households: spending, saving and borrowing", "Bahruz - IGCSE Business"),
    ]),
    ("econ:3.3", &[
        ("47cwiNk6NiA", "The Labour Market", MRLEE),
        ("qsKQASmFCyk", "Workers: wages and the labour market", MRLEE),
        ("seg2W1j8iDg", "Wage Determination and Occupational Choices", THINK),
        ("-WyGlDmwLLE", "Labour Market Wage Determination", EPD),
    ]),
    ("econ:3.4", &[
        ("EjExuH1w6Rs", "Firms: size, growth and mergers", MRLEE),
        ("I3orIItbKW0", "Economies and diseconomies of scale", "The Secondary Scholar"),
        ("i56CWKKzcfc", "Types of integration, mergers and economies of scale", "Econ Insights 101"),
    ]),
    ("econ:3.5", &[
        ("lfZkoEPVOVY", "Labour-intensive vs capital-intensive production", THINK),
        ("IB6biD20iSQ", "Productivity & Division of Labour", MRLEE),
        ("qTL1h3GNrmg", "Capital vs Labour intensive production", "Econ Insights 101"),
    ]),
    ("econ:3.6", &[
        ("SgvqTDPIQ18", "Firms' costs, revenue and objectives", MRLEE),
        ("TcIj94BzSF8", "Fixed and Variable Costs (AFC, TFC, AVC)", EPD),
        ("AZr_038EMsU", "Objectives of Firms - Profit Max, Rev Max, Sales Max, Satisficing", EPD),
    ]),
    ("econ:3.7", &[
        ("3fvkiDeYgCw", "Types of markets", MRLEE),
        ("-eDVyBRxp0c", "Market Structures :Competitive vs Monopoly", THINK),
        ("CJJQL5i_Z3E", "Monopolies, Oligopolies and competitive markets", "Mr Goff"),
        ("UXC51iTDEJI", "Monopoly", EPD),
    ]),
    ("econ:4.1", &[
        ("Pe9iewI17Hc", "Government Macroeconomic intervention", MRLEE),
        ("JjkND_40MmA", "Government Macroeconomic Aims", THINK),
        ("OPV1BOs1ISI", "Macro Objectives of Government (Growth Unemployment, Inflation, Trade - TIGERS)", EPD),
    ]),
    ("econ:4.2", &[
        ("1BeUz0qT-Hg", "Fiscal policy", MRLEE),
        ("8pwldnyKiZg", "Fiscal policy and the budget", "Mr Goff"),
        ("NEcfy0HpewQ", "Fiscal Policy - Government Spending and Taxation", EPD),
    ]),
    ("econ:4.3", &[
        ("ixLDwUM2s6o", "Monetary policy", MRLEE),
        ("FdKEf1zfNwc", "Monetary policy", "Mr Goff"),
        ("uBaTPugw3M4", "Monetary Policy - Interest Rates, Money Supply & Exchange Rate", EPD),
        ("R8VBRCs2jTU", "How does raising interest rates control inflation?", "The Economist"),
    ]),
    ("econ:4.4", &[
        ("OZopG-lWm_w", "Supply Side Policy", MRLEE),
        ("lYKAMu3F9YE", "Supply-Side Policies and Their Impact", THINK),
        ("PvfdPfEd-gk", "Supply Side Policies (Interventionist and Market Based) - With Evaluation", EPD),
    ]),
    ("econ:4.5", &[
        ("lU3a3pk_9MY", "Economic Growth", MRLEE),
        ("SwaCg7Gwtzw", "What causes an economic recession?", "TED-Ed"),
        ("5vDHdxjtSTU", "Causes of Economic Growth (Short Run and Long Run)", EPD),
    ]),
    ("econ:4.6", &[
        ("TEzeuZUHb9g", "Employment and unemployment", MRLEE),
        ("AoKT6fNNdvc", "Unemployment: Causes, Consequences, and Reduction Policies", THINK),
        ("DWLv6JHa7YE", "Types and Causes of Unemployment (Cyclical, Structural, Frictional and more)", EPD),
        ("0UC6SydfKDA", "Consequences of unemployment", "Mr Goff"),
    ]),
    ("econ:4.7", &[
        ("47RAgSzcpMg", "Inflation", MRLEE),
        ("lxgAfp6mA_8", "Inflation: Causes, Consequences, and Control", THINK),
        ("nfZTP7MB5D4", "Deflation: Causes, Consequences, and Management", THINK),
        ("USj52Vlvd5M", "Cost-push Inflation and Demand-pull Inflation", "Jacob Clifford"),
        ("PX9XdZGsFXs", "Deflation - Causes and Consequences (Deflation can be Deadly!)", EPD),
    ]),
    ("econ:5.1", &[
        ("Qp5LEGLavKQ", "Economic development", MRLEE),
        ("l9pqS_0TTF4", "Living standards and income distribution", THINK),
        ("vUhDKkOejBo", "HDI and GDP per head: past paper questions", "Fundoo Tutor IGCSE Grade 9 and 10"),
        ("z-vdJKnC7FM", "Measures of Economic Growth & Living Standards - GDP, GDP/Capita, GNI, Green GDP", EPD),
    ]),
    ("econ:5.2", &[
        ("12M4Pn7bChw", "Poverty", MRLEE),
        ("rwF037tTm94", "Poverty: Definitions, Causes, and Policy Solutions", THINK),
        ("-xFqPVj06Us", "Absolute and relative poverty", "tutor2u"),
    ]),
    ("econ:5.3", &[
        ("F33bp3LCb0E", "Population", MRLEE),
        ("I5NPzQ4Hon4", "Population Growth: Factors, Variations, and Effects", THINK),
        ("IOa0V4iAprs", "Population pyramids explained", "LEARN OR DIE"),
    ]),
    ("econ:5.4", &[
        ("ZyKAkzm1wGo", "Differences in economic development", MRLEE),
        ("PbkGaQEd1vY", "Differences in economic development between countries", THINK),
        ("9tnw6jYqAk0", "Differences in economic development between countries", "Sir Usman | Economics & Business"),
    ]),
    ("econ:6.1", &[
        ("PYcc52nSOIQ", "Specialisation and free trade", MRLEE),
        ("glEoWRMI18s", "Free Trade - Benefits and Costs", EPD),
        ("NI9TLDIPVcs", "Specialization and Trade", "CrashCourse"),
    ]),
    ("econ:6.2", &[
        ("2Niumv8HDfQ", "Globalisation and trade restrictions", MRLEE),
        ("GczFPH_TbNs", "Globalisation, multinationals, free trade and protectionism", THINK),
        ("QO1fRsyhu44", "Types of Protectionism", EPD),
        ("TLmXIFIYZ64", "Globalisation", EPD),
    ]),
    ("econ:6.3", &[
        ("iI2X3s1RVlc", "Foreign exchange rates", MRLEE),
        ("j4IRdVqpdMc", "Foreign Exchange Rates: Definition, Fluctuations and Consequences", THINK),
        ("c7YC2PKab7M", "Exchange Rate Changes - Appreciations and Depreciations of a Floating Exchange Rate", EPD),
        ("MzTcvpXdfcs", "Impact of Exchange Rate Appreciations and Depreciations with Evaluation", EPD),
    ]),
    ("econ:6.4", &[
        ("dr0T4ey0xhM", "Current account", MRLEE),
        ("2pUMl0QzWXs", "The current account of the balance of payments", THINK),
        ("mvq6Fjzdjd8", "Current Account of the Balance of Payments", EPD),
        ("xZIPvpiPvqY", "The importance of the balance of payments on current account", "Mr Goff"),
    ]),
    // ---------- English Literature (AQA GCSE 8702) — Mr Bruff, Mr Salles, Easy as GCSE and others ----------
    ("englit:3.1.1a", &[
        ("4GSCWDa1qcE", "Shakespeare in seven minutes: Macbeth summary", EASY),
        ("pCsypkF5U_Y", "Macbeth Plot Summary in Under 4 Minutes", "Schooling Online"),
        ("goET70zn57s", "Macbeth: top set analysis (part 1)", BRUFF),
    ]),
    ("englit:3.1.1b", &[
        ("NmMAO82R8Cg", "Character Analysis: Macbeth", BRUFF),
        ("sSDcTyMAt0U", "Student grade 9 essay on Macbeth's character", SALLES),
        ("a5zPagitg5E", "How to get a grade 9 in Macbeth: the only 10 concepts you need", SALLES),
    ]),
    ("englit:3.1.1c", &[
        ("90iY1ku7flA", "Character Analysis: Lady Macbeth", BRUFF),
        ("KV2hlM2pkS8", "Lady Macbeth Character Analysis: English Literature Revision", EASY),
        ("JevkOJ2ajIQ", "Macbeth: the character of Lady Macbeth", "BBC Bitesize for Teachers"),
    ]),
    ("englit:3.1.1d", &[
        ("smK89SS_z8A", "Ambition in Macbeth | Theme Analysis", "Comics and Lit"),
        ("Ljrf1-UzAgM", "Kingship in Macbeth | Theme Analysis", "Comics and Lit"),
        ("O3v6SHRjZhM", "'Ambition' in Macbeth: Key Quotes & Analysis", "Dr Aidan"),
    ]),
    ("englit:3.1.1e", &[
        ("DDH4ooBU7TA", "Macbeth Themes Revision: The Supernatural, Fate vs Free Will, Appearance vs Reality", EASY),
        ("Fe4JOaR8UdQ", "Guilt in Macbeth | Theme Analysis", "Comics and Lit"),
        ("9RkMKSUYXHE", "'Fate and Free Will' in Macbeth: Key Quotes & Analysis", "Dr Aidan"),
        ("p0srFlBU7iU", "Macbeth Themes Revision: Ambition and Guilt", EASY),
    ]),
    ("englit:3.1.1f", &[
        ("SOCR0Ab1ABk", "Macbeth context to impress your examiner", "The Lightup Hub"),
        ("-S6sQtmbYhY", "Macbeth: contextual analysis", "Schooling Online"),
        ("r9EiT09JFWs", "James I And His Influence On Macbeth", "Pate Resources"),
        ("KvaZd3OmPNA", "Macbeth context: witchcraft in Shakespeare's time", "myShakespeare"),
    ]),
    ("englit:3.1.2a", &[
        ("de5NRZRZohE", "'A Christmas Carol' Plot Summary", BRUFF),
        ("sZB-G4882aM", "'A Christmas Carol': Structure", BRUFF),
        ("8fzPJUtstn4", "A Christmas Carol: 7 minute summary", EASY),
        ("e98F6whQUFM", "Dickens' A Christmas Carol: top set analysis", BRUFF),
    ]),
    ("englit:3.1.2b", &[
        ("F2kuQSbazUo", "Ebeneezer Scrooge: Character Analysis - 'A Christmas Carol'", BRUFF),
        ("-HXBa-8N8KI", "How does Scrooge transform in A Christmas Carol?", "Jen Chan"),
        ("kyJBrUxlNpg", "Ebenezer Scrooge character analysis", EASY),
        ("c2x9wiRRFQY", "A Christmas Carol: Analysis of Scrooge + Key Quotes", "Dr Aidan"),
    ]),
    ("englit:3.1.2c", &[
        ("YW6Qo3TB39o", "The 3 Ghosts: Character Analysis - 'A Christmas Carol'", BRUFF),
        ("A9TNVOWQxdY", "Jacob Marley: Character Analysis - 'A Christmas Carol'", BRUFF),
        ("XJJ7zSSOhYw", "The ghosts part 1: Marley and the Ghost of Christmas Past", EASY),
        ("p-TZzy30Mkc", "The ghosts part 2: Christmas Present and Christmas Yet To Come", EASY),
        ("15HiKFCMEyk", "'The Ghosts' in A Christmas Carol (Key Quotes & Analysis)", "Dr Aidan"),
    ]),
    ("englit:3.1.2d", &[
        ("WA-BTiru9RA", "A Christmas Carol Themes: Poverty and Social Injustice", "Beyond Revision"),
        ("hKjF0NFpMC8", "A Christmas Carol Themes: Transformation and Redemption", "Beyond Revision"),
        ("2gMw20RDgIM", "Poverty in A Christmas Carol: quotes, arguments and essay plan", "Revise with Mr Wood"),
        ("nWu8oR2H1Cg", "A Christmas Carol key themes: poverty", "The Ten Minute English Teacher"),
    ]),
    ("englit:3.1.2e", &[
        ("w7V4tXuhbk8", "'A Christmas Carol': Context", BRUFF),
        ("3xRonangfz0", "Dickens' A Christmas Carol in context", "ClickView"),
        ("NpQTrX6Zh98", "A Christmas Carol: context", "Tutoring with Gavin"),
        ("vL74fkFkHpM", "Grade 9 A Christmas Carol revision: context", "The Lightup Hub"),
    ]),
    ("englit:3.2.1a", &[
        ("Dc7-wKFR5y8", "An Inspector Calls: 7 minute summary", EASY),
        ("QJ_0VgEduXY", "'An Inspector Calls': Act 1 Summary", BRUFF),
        ("bcXMy84cr5g", "'An Inspector Calls': Act 2 Summary", BRUFF),
        ("v_m3SMNk-SA", "'An Inspector Calls': Act 3 Summary", BRUFF),
    ]),
    ("englit:3.2.1b", &[
        ("KvhiaECGjTY", "'An Inspector Calls': Mr Birling Character Analysis", BRUFF),
        ("NRhvVIINlyM", "'An Inspector Calls': Mrs Birling Character Analysis", BRUFF),
        ("0lfHDKhZ_aw", "'An Inspector Calls': Sheila Character Analysis", BRUFF),
        ("ryE3EnENnBI", "'An Inspector Calls': Eric Character Analysis", BRUFF),
        ("8TdZtpuDB_Q", "'An Inspector Calls': Gerald Animated Character Analysis", BRUFF),
    ]),
    ("englit:3.2.1c", &[
        ("FOeASYrxL1c", "'An Inspector Calls': Inspector Goole Character Analysis", BRUFF),
        ("_sMRnzualzQ", "Who is Inspector Goole? Animated character analysis", EASY),
    ]),
    ("englit:3.2.1d", &[
        ("Txqz_UiLXHc", "Everything you need to know on social responsibility", EASY),
        ("gUqKoJXIdPY", "Gender and power: theme analysis", EASY),
        ("Prp1-e3kAHE", "An Inspector Calls themes: responsibility overview", "Beyond Revision"),
        ("5jlZ0IXmSxA", "The theme of social responsibility in An Inspector Calls", "ClickView"),
    ]),
    ("englit:3.2.1e", &[
        ("CKAPm-BlfmE", "An Inspector Calls context: why is it set in 1912 and not 1945?", "GCSE English Explained"),
        ("Zi3iiR1tz6I", "An Inspector Calls: context, 1912 and 1945", "CENTURY Tech"),
        ("F1wlZpu1txo", "An Inspector Calls context: J. B. Priestley", "Comics and Lit"),
        ("3fXw8lWWtlA", "An Inspector Calls: context and background", "ClickView"),
    ]),
    ("englit:3.2.2a", &[
        ("PV_EeGJmWqA", "'Ozymandias' in 6.5 Minutes: Quick Revision", BRUFF),
        ("mqWZLy7hUOc", "Revise all of Ozymandias in one video", EASY),
        ("d_Egz2bDQ0o", "Percy Shelley's 'Ozymandias'", BRUFF),
    ]),
    ("englit:3.2.2b", &[
        ("McAbDpgtje0", "'London' in 6 Minutes: Quick Revision", BRUFF),
        ("cJ6NCtJdoqM", "Revise everything about London in one video", EASY),
        ("zHp8eVi27Nw", "William Blake: 'London'", BRUFF),
        ("6BERjLZzuOg", "William Blake's London: poetry between the lines", "BBC Bitesize for Teachers"),
    ]),
    ("englit:3.2.2c", &[
        ("5yVflZI3Mr0", "'Extract from The Prelude' in Under 6 Minutes: Quick Revision", BRUFF),
        ("Rq10axbFfg4", "The Prelude: summary and analysis", "Mr Everything English"),
        ("_iFFRzRajQw", "Grade 9 analysis of The Prelude", SALLES),
        ("EGn1Ilx_3o4", "Wordsworth's Prelude, boat-stealing: poetry between the lines", "BBC Bitesize for Teachers"),
    ]),
    ("englit:3.2.2d", &[
        ("itfGGpFIloc", "'My Last Duchess' in 6 Minutes: Quick Revision", BRUFF),
        ("P6WO9AKt4zI", "My Last Duchess: Power and Conflict poetry revision", "The Ten Minute English Teacher"),
        ("T9h_csKEwxg", "'My Last Duchess' by Robert Browning", BRUFF),
    ]),
    ("englit:3.2.2e", &[
        ("Zwgv-MdWDYU", "'The Charge of the Light Brigade' in 5 Minutes: Quick Revision", BRUFF),
        ("eminBimYdbg", "Everything you need to know about The Charge of the Light Brigade", EASY),
        ("OXVs8KydoNY", "Alfred Lord Tennyson's 'The Charge of the Light Brigade'", BRUFF),
    ]),
    ("englit:3.2.2f", &[
        ("FH_robM_-wg", "'Exposure' by Wilfred Owen in 5 Minutes: Quick Revision", BRUFF),
        ("6jivG8sWG0I", "Revise Exposure in one video", EASY),
        ("64FESmLvQEs", "Wilfred Owen: 'Exposure'", BRUFF),
    ]),
    ("englit:3.2.2g", &[
        ("gVgl_pLemfw", "'Storm on the Island' by Seamus Heaney in 5.5 Minutes: Quick Revision", BRUFF),
        ("iU558Fnafik", "Storm on the Island: complete revision guide", EASY),
        ("Sgsu_WgO9GY", "Seamus Heaney: 'Storm on the Island'", BRUFF),
    ]),
    ("englit:3.2.2h", &[
        ("4--ViHqsaNc", "'Bayonet Charge' by Ted Hughes in 5.5 Minutes: Quick Revision", BRUFF),
        ("i2NgWjDSk4g", "Revise Bayonet Charge in one video", EASY),
        ("6AMuwf9zzKM", "Ted Hughes: 'Bayonet Charge'", BRUFF),
    ]),
    ("englit:3.2.2i", &[
        ("R4fLrVnp2jk", "'Remains' by Simon Armitage in 5 Minutes: Quick Revision", BRUFF),
        ("RM3qpLPLF0o", "Revise Remains in one video", EASY),
        ("vmUCX-dSb9E", "Simon Armitage: 'Remains'", BRUFF),
    ]),
    ("englit:3.2.2j", &[
        ("xdD3je6OKAU", "'Poppies' in 4.5 Minutes: Quick Revision", BRUFF),
        ("Z0j3GHL1RE0", "Revise Poppies in one video", EASY),
        ("FEqSAT77SDQ", "'Poppies' by Jane Weir", BRUFF),
        ("cbH-6YYbxgE", "Grade 9 analysis: Poppies", SALLES),
    ]),
    ("englit:3.2.2k", &[
        ("MbJGwPjZ3ZM", "'War Photographer' in 5 Minutes: Quick Revision", BRUFF),
        ("4_5ALePPJUE", "Revise War Photographer in one video", EASY),
        ("HeZCQlUMQxI", "Carol Ann Duffy: 'War Photographer'", BRUFF),
    ]),
    ("englit:3.2.2l", &[
        ("BSC2cKcgkMk", "'Tissue' in 6 Minutes: Quick Revision", BRUFF),
        ("ZR9_yFJhZWU", "Tissue: analysis", SALLES),
        ("wVjZpi9lkcI", "Imtiaz Dharker: 'Tissue'", BRUFF),
    ]),
    ("englit:3.2.2m", &[
        ("wzx1J8ojGPE", "'The Emigree' in 5 Minutes: Quick Revision", BRUFF),
        ("9QpRshR-_10", "Revise The Émigrée in one video", EASY),
        ("RfIJ8iXLfLc", "'The Emigree', by Carol Rumens", BRUFF),
    ]),
    ("englit:3.2.2n", &[
        ("cnyv4bucAwA", "'Checking Out Me History' by John Agard in 6 Minutes: Quick Revision", BRUFF),
        ("Mj1bMk_E7GQ", "John Agard: 'Checking Out Me History'", BRUFF),
        ("-0nJ_IlMFac", "Comparing Checking Out Me History and The Émigrée", "MyEdSpace English"),
    ]),
    ("englit:3.2.2o", &[
        ("115XZNvCwlI", "'Kamikaze' in 4 Minutes: Quick Revision", BRUFF),
        ("CNSpDKNGkVg", "Revise Kamikaze in one video", EASY),
        ("9zwoe5twfd4", "'Kamikaze' by Beatrice Garland", BRUFF),
        ("hvof1FUlf4s", "Beatrice Garland reads 'Kamikaze'", BRUFF),
    ]),
    ("englit:3.2.2p", &[
        ("5mQGQhcaptM", "Which Power and Conflict poems compare best?", BRUFF),
        ("zreLmzXklaE", "Power and Conflict poetry: animated summary of all 15 poems", BRUFF),
        ("F28c6KOhgNw", "All 15 poems explained in one mindmap", FRT),
        ("urXta6o-7Xg", "3 key points on all 15 Power and Conflict poems", BRUFF),
    ]),
    ("englit:3.2.2q", &[
        ("M9ra8A0fTBU", "How to structure your Power and Conflict poetry analysis", BRUFF),
        ("HI9q90kLhDY", "Comparing poems: a step-by-step guide", FRT),
        ("u_1mjUY54oE", "Poetry comparison: 4 ways to get grade 9", SALLES),
        ("kUnAtrRFbes", "Grade 9 student essay: Bayonet Charge and Exposure", SALLES),
        ("vMBydl-0Kng", "How to organise your AQA anthology poetry comparison", BRUFF),
    ]),
    ("englit:3.2.3a", &[
        ("7Bari-Ggx5w", "Unseen poetry: how to get full marks in Paper 2 Section C", FRT),
        ("3yr7VSIgy9s", "How to analyse any unseen poem", "Jen Chan"),
        ("aQ7CTHb_An0", "Unseen poem: the perfect method", SALLES),
        ("iIWmthgysSM", "Ultimate guide to Literature Paper 2 Section C: unseen poetry", BRUFF),
    ]),
    ("englit:3.2.3b", &[
        ("Al5rdKkfBlY", "Unseen poem comparison sorted", SALLES),
        ("bBsUyLuBjyA", "Comparing the unseen poems: the way to secure 8/8", "Mr Everything English"),
        ("1YrfpNluI_U", "Unseen poetry comparison question", "Bossing English with Mr F"),
    ]),
    ("englit:3.3a", &[
        ("U4t509-raoE", "How are form and structure different?", "Jen Chan"),
        ("Md0l6uRl2c8", "How to analyse any quote in your English essay", FRT),
        ("6zXBiAuPQ_Y", "10 language and structure techniques you'll find in any exam", FRT),
    ]),
    ("englit:3.3b", &[
        ("tIhHjL9c9G4", "How to include context in literary analysis", "Jen Chan"),
        ("QGCjcAdm53A", "The best way to add context (AO3) to a Literature essay", "Mr Everything English"),
    ]),
    // ---------- English Language (AQA GCSE 8700) — Mr Bruff's 2026 guides, First Rate Tutors, Mr Salles ----------
    ("englang:1.1a", &[
        ("OlIwGb7bSOI", "Paper 1 Question 1: your guide", BRUFF),
        ("7J660l_NmwU", "Paper 1 rap", BRUFF),
    ]),
    ("englang:1.1b", &[
        ("L_dE68iUg-k", "Paper 1 Question 2", BRUFF),
        ("HblzTRxJ_-4", "Paper 1 Question 2 in detail: walking talking mock", BRUFF),
        ("ML9V4Sy3z_s", "How to get full marks on Paper 1 Question 2", "The Lightup Hub"),
        ("MtOCSrnioVo", "Paper 1 Question 2 (new for 2026)", "Comics and Lit"),
    ]),
    ("englang:1.1c", &[
        ("VNVB5InFrHQ", "Paper 1 Question 3", BRUFF),
        ("hM9Gv58zDac", "The easiest way to analyse structure in Paper 1 Question 3", BRUFF),
        ("cQ1XJKCnC0g", "How to find a structure technique in 30 seconds", FRT),
        ("zQgroyKOArg", "Paper 1 Question 3: how to get 8/8", "The Lightup Hub"),
    ]),
    ("englang:1.1d", &[
        ("y22Ciur-Ryo", "Paper 1 Question 4: your guide", BRUFF),
        ("4HMwe4kqkGQ", "How to answer Paper 1 Question 4 in 3 steps", FRT),
        ("Gs4Fc7wzVD8", "Paper 1 Question 4: how to get 20/20", "The Lightup Hub"),
    ]),
    ("englang:1.2a", &[
        ("M4IE_jSK6lg", "Descriptive writing: secrets of the new mark scheme", BRUFF),
        ("zP5bUlDKJK0", "The perfect descriptive writing piece in five steps", FRT),
        ("qLKbxVyUGJM", "Paper 1 Question 5: how to get 40/40", "The Lightup Hub"),
    ]),
    ("englang:1.2b", &[
        ("OalIJCsUMvY", "Paper 1 Question 5: writing a story opening", BRUFF),
        ("AkfkWVEy_QQ", "The perfect story opening in 5 steps", FRT),
        ("Qrc4X1nxipo", "11 fatal mistakes of story writing (and how to avoid them)", SALLES),
        ("pLc3ICEC2GY", "The perfect creative writing story in 5 steps", FRT),
    ]),
    ("englang:1.2c", &[
        ("tAalMSzrK0U", "Learning from the best: writing an effective story opening", BRUFF),
        ("H4-u20fCldE", "How to write an engaging story opening", "Miss Adams Teaches..."),
        ("ACSGA0hHWzU", "How to get 40/40 in Question 5 using this sentence", FRT),
        ("pdoaQ4wLzXI", "3 creative writing mistakes examiners hate", FRT),
    ]),
    ("englang:2.1a", &[
        ("yKZ_Tr2Y-CE", "Paper 2 Question 1", BRUFF),
        ("-ZN_X5sIcZI", "How easy is Paper 2 Question 1?", SALLES),
        ("5P2r2BgXSd4", "Paper 2 Question 1: get those easy marks", "Mr Everything English"),
    ]),
    ("englang:2.1b", &[
        ("Y51zxEf4QYQ", "Paper 2 Question 2: your guide", BRUFF),
        ("TvRzN8k45II", "Paper 2 Question 2: how to actually infer and compare", FRT),
        ("wh9KPBApsU0", "Paper 2 Question 2: how to get 8/8", "The Lightup Hub"),
        ("fb12tKlX0q0", "Paper 2 Question 2: how to get full marks", SALLES),
    ]),
    ("englang:2.1c", &[
        ("RUWxpg_EmeM", "Paper 2 Question 3", BRUFF),
        ("68lBeFZgJsI", "Paper 2 Question 3 (worked example)", BRUFF),
        ("prIBv9luZIw", "Paper 2 Question 3: what really gets the grades?", SALLES),
        ("btxhOONIr3Y", "Paper 2 Question 3: full mark answer and method", SALLES),
    ]),
    ("englang:2.1d", &[
        ("tNb3RdGEmYA", "Paper 2 Question 4: your guide", BRUFF),
        ("eWkSbes21Z8", "Paper 2 Question 4: compare writers' perspectives", "BBC Bitesize - GCSE Revision Support"),
        ("colgbQp1zaM", "Paper 2 Question 4: a full-mark comparison paragraph", FRT),
        ("4WW0vYfsnnM", "Paper 2 Question 4: the perfect method is easy", SALLES),
    ]),
    ("englang:2.2a", &[
        ("v0aAitntCvo", "Paper 2 Question 5", BRUFF),
        ("A3YlQFjm6K0", "How to answer any persuasive writing question", FRT),
        ("xFdQvdArGm4", "Persuasive writing techniques you must know", SALLES),
        ("Cmv9M8l_ftc", "Paper 2 Question 5: how to get 40/40", "The Lightup Hub"),
    ]),
    ("englang:2.2b", &[
        ("GauwoIz_nHI", "What should you include in a letter, article, leaflet, essay or speech?", BRUFF),
        ("60NkImwWrvc", "Writing an article", BRUFF),
        ("T7TM6qmRqus", "Writing a letter", BRUFF),
        ("EMmAriRCl20", "Writing a speech", BRUFF),
        ("bkc9rINcYAo", "The perfect article, letter or speech for Paper 2", FRT),
    ]),
    ("englang:2.2c", &[
        ("jJk2GOgRSrw", "How to answer any Paper 2 Question 5", FRT),
        ("wbvIrznF5mY", "Paper 2 Question 5: top-grade structure breakdown", "Literature Lens"),
        ("QkAU7mA-PE4", "How to plan top grade persuasive writing", SALLES),
        ("B1aONhs0SC4", "Paper 2 Question 5: argue and persuade", SALLES),
    ]),
    ("englang:3a", &[
        ("TuzEG6rUuqA", "How to get a Distinction in your Spoken Language endorsement", "Mr Davey"),
        ("2d7uajjU_JI", "Achieving a Distinction for Spoken Language", "SchofieldShakespeare"),
        ("fv-TrMqO53A", "Planning your spoken language presentation", "Mrs Stockill's English"),
        ("PjuxHC4zjWQ", "How to secure a Distinction in your GCSE English speech", "Dylan Boateng"),
    ]),
    ("englang:3b", &[
        ("TuzEG6rUuqA", "How to get a Distinction in your Spoken Language endorsement", "Mr Davey"),
        ("2d7uajjU_JI", "Achieving a Distinction for Spoken Language", "SchofieldShakespeare"),
        ("jNmwNB3xFr8", "Five tips to help you during your speaking exam", "British Council | LearnEnglish Teens"),
    ]),
    // ---------- Further Pure Maths (Edexcel IGCSE 4PM1) — TLMaths, Corbettmaths, 1st Class Maths, Bicen Maths ----------
    ("fpm:1a", &[
        ("F492MeO74fE", "Logarithms: Introducing Logarithms", TLM),
        ("M3TVZT05XOA", "Laws of Logarithms: Introducing the Laws of Logarithms", TLM),
        ("Cr5jHLDIbGY", "Laws of Logarithms: Using the Laws", TLM),
        ("Jt2j6ZJ5FCg", "Laws of Logarithms: Writing as a Single Logarithm", TLM),
    ]),
    ("fpm:1b", &[
        ("ndU_cCbPAm4", "Surds", CM),
        ("96SwZpRvhwY", "Rationalising denominators", CM),
        ("x25DsjbilsM", "Surds: Introducing Rationalising the Denominator Part 1", TLM),
        ("kjzIeojMfWk", "Surds: Introducing Rationalising the Denominator Part 2", TLM),
    ]),
    ("fpm:2", &[
        ("DUm6fQkZG3g", "The Discriminant", CM),
        ("kBSj35rik8w", "The sum and product of the roots of a quadratic", "The Organic Chemistry Tutor"),
        ("Ha_S-GLRVow", "Discriminants", "Save My Exams"),
    ]),
    ("fpm:3a", &[
        ("b88DwALjFdw", "The Factor Theorem and The Remainder Theorem", "Maths Genie"),
        ("A_S1YcVsO80", "Algebraic Long Division", CM),
        ("lyMwX8_QZIc", "Polynomials: Introducing the Factor Theorem", TLM),
        ("hPgL_bqa_fM", "The Factor Theorem", FIRSTCLASS),
    ]),
    ("fpm:3b", &[
        ("8J_m-hMp8lY", "Quadratic Inequalities", CM),
        ("Hpv9Y5S5rOw", "Inequalities: Examples of Solving Quadratic Inequalities", TLM),
        ("-YcE_D78fGE", "Solving inequalities", CM),
    ]),
    ("fpm:3c", &[
        ("aexvnpH-jhI", "Inequalities and Regions on Graphs", CM),
        ("ChrX2MNLQX8", "Inequality Regions", FIRSTCLASS),
        ("RBnnSmKxE4c", "Linear programming", "BareauMaths"),
    ]),
    ("fpm:4", &[
        ("pRu31H6qG4k", "Graphs: Sketching Quadratics, Cubics, Quartics and Quintics", TLM),
        ("fy45qX8cUwQ", "Graphing rational functions and their asymptotes", "Professor Dave Explains"),
        ("TNZe8MCzPZ0", "Reciprocal graphs", "Zeeshan Zamurred"),
        ("kTTTkMwXqrg", "Reciprocal graphs", CM),
    ]),
    ("fpm:5", &[
        ("g3E_A598BVs", "Sigma notation, arithmetic and geometric series", "Zeeshan Zamurred"),
        ("Zolv3DQL5WI", "Proof of the sum of an arithmetic series", CM),
        ("5VTmlDk4ed0", "Proof of the sum of a geometric series", CM),
        ("fGXBcn_9L4g", "Sum to infinity of a geometric series", "Zeeshan Zamurred"),
        ("5G7VKXarees", "Sequences and series in 23 minutes", BICEN),
    ]),
    ("fpm:6", &[
        ("T_IJcW07YDQ", "Binomial Expansion", FIRSTCLASS),
        ("rRs8s2UH8Mo", "Binomial expansion where n is a fraction or negative", "Mastering Maths"),
        ("JnjAr7PhwqI", "Binomial expansion with a negative power", "Maths at Home"),
    ]),
    ("fpm:7", &[
        ("EJ3dwJfmzpI", "Vectors in 21 minutes", BICEN),
        ("5S6j_K8x8hQ", "Position vectors", "Zeeshan Zamurred"),
        ("0QqAF1aRnes", "Vectors: solving geometric problems", "mathonify"),
        ("xOdkldbusy0", "Vectors", CM),
    ]),
    ("fpm:8", &[
        ("zSkRQ7mr6EA", "Straight lines: gradient, midpoint and distance between two points", "A Level Maths Tutor | John Armstrong"),
        ("tRTk9vG0nwM", "Gradient Formula", CM),
        ("fldJZL5JQAw", "Distance between two points formula", CM),
        ("LqEYBytlhek", "Midpoint of a Line", CM),
        ("RS_HSbad_eA", "Ratio in Coordinate Geometry", CM),
        ("ZJOezY0xfr0", "Equation of a Line Through a Point", CM),
    ]),
    ("fpm:9a", &[
        ("LLfOrUH86_E", "Chain, product and quotient rule explained in 14 minutes", "NeilDoesMaths"),
        ("BIu0m2DObAA", "Differentiation: Introducing the Chain Rule", TLM),
        ("eeXTgSniNiI", "Differentiation: Introducing the Product Rule", TLM),
        ("GoxqlIrWNAY", "Differentiation: Introducing the Quotient Rule", TLM),
        ("yhNKawQHBIk", "Differentiation", CM),
    ]),
    ("fpm:9b", &[
        ("8aPSaDNhJpk", "Stationary Points", CM),
        ("7D8f4ACZLmY", "Differentiation (Gradients, Tangents and Normals)", FIRSTCLASS),
        ("y9M6kmgSnw8", "Differentiation (Maxima and Minima)", FIRSTCLASS),
        ("N_FM6aON2z8", "Equation of a Tangent to a Curve", CM),
        ("S6yXq9vbVdU", "Equation of a Normal", CM),
    ]),
    ("fpm:9c", &[
        ("cmMQ8bHb65U", "How to find the area under a curve with integration", "Jack's Maths"),
        ("Ec1-qiCyP48", "Areas under curves", "Zeeshan Zamurred"),
        ("QLHJl2_aM5Q", "Volume of a solid of revolution by integration", "Professor Dave Explains"),
        ("zimC-9onLNo", "Integration: volumes of revolution", "HEGARTYMATHS"),
    ]),
    ("fpm:9d", &[
        ("pFeuGMMiZWw", "Position, velocity and acceleration using derivatives", "Patrick J"),
        ("JHFJC0vmfU0", "Differentiation: Introducing Connected Rates of Change", TLM),
        ("MKj3nNIu0vE", "Connected rates of change", "Maths Genie"),
        ("9GkYv-vTEOU", "Solving Problems using Differentiation", CM),
    ]),
    ("fpm:10a", &[
        ("nzGDeZS2FF0", "Trigonometry: Introducing Radians", TLM),
        ("pQTn-lgT_ko", "Radians in 18 minutes", BICEN),
        ("UjgOR07zOzY", "Exact trigonometric values", CM),
        ("vULd_UA2N_0", "Trigonometry: Using the Formula for Arc Length in Radians", TLM),
        ("TEq-2fSDWvk", "Trigonometry: Using the Formula for Area of a Sector in Radians", TLM),
    ]),
    ("fpm:10b", &[
        ("7xeLeDulY60", "The Sine Rule", FIRSTCLASS),
        ("3H3u92WJAjw", "Cosine rule", CM),
        ("Kvxoa97K1GQ", "Trigonometry: Using the Sine Rule", TLM),
        ("99CL-tNyNR0", "3D Trigonometry and Pythagoras", FIRSTCLASS),
    ]),
    ("fpm:10c", &[
        ("SifUQHn9f78", "Trigonometric Identities", FIRSTCLASS),
        ("9wBG-gs7qRk", "Trigonometric Identities", CM),
        ("R_8xC1W-DCQ", "Addition formulae", BICEN),
        ("kJwCXuxLj4E", "An Introduction to Solving Trigonometric Equations", CM),
        ("_McuKeG9DQI", "Solving Trigonometric Equations", FIRSTCLASS),
        ("cILaBqbmPX0", "Solving Trigonometric Equations 1", CM),
    ]),
    // ---------- Geography (AQA GCSE 8035) ----------
    ("geog:3.1.1.1", &[
        ("LQp_82E2fTs", "What is a Natural Hazard? | AQA GCSE Geography | Natural Hazards 1", T2U),
        ("9AAWepfiu2Q", "What is Hazard Risk & What Affects it?  AQA GCSE Geography | Natural Hazards 2", T2U),
        ("-O-pKr8CtMU", "Natural Hazards & Hazard Risk | AQA GCSE 9-1 Geography", KED),
    ]),
    ("geog:3.1.1.2a", &[
        ("-PQBgveU7q0", "Tectonic Plate Boundaries/Margins | AQA GCSE 9-1 Geography", KED),
        ("GoCkjr5WfyQ", "Destructive Plate Margins | AQA GCSE Geography | Tectonic Hazards 5", T2U),
        ("1RuhHqOdBnQ", "Constructive Plate Margins | AQA GCSE Geography | Tectonic Hazards 6", T2U),
    ]),
    ("geog:3.1.1.2b", &[
        ("enOkPyisuz4", "5) Comparing the effects of earthquakes - Chile & Nepal - Powered by @GeographyHawks", HAWKS),
        ("UHLTFaP-2ZE", "6) Comparing responses to earthquakes - Powered by @GeographyHawks", HAWKS),
        ("Nlp_O6W5kXk", "The Nepal and Chile earthquakes, case studies of contrasting wealth.", "Bonhill Geography"),
    ]),
    ("geog:3.1.1.2c", &[
        ("T6Botca1_FQ", "Why Live in Areas of Tectonic Risk? | AQA GCSE Geography | Tectonic Hazards 10", T2U),
        ("hK-dUnWv7Zg", "Risk Mitigation (MP3) for Tectonic Hazards | AQA GCSE Geography | Tectonic Hazards 11", T2U),
        ("dfq9Q6FPBME", "Why do people live near tectonic hazards? | AQA GCSE 9-1 Geography", KED),
    ]),
    ("geog:3.1.1.3a", &[
        ("jvfB4YFRC3w", "Global Atmospheric Circulation | QA GCSE Geography | Weather Hazards 1", T2U),
        ("0DIqel4KLW0", "How do Tropical Storms Form? | AQA GCSE Geography | Weather Hazards 4", T2U),
        ("5MuabnhLOdY", "How Might Climate Change Affect Tropical Storms? | AQA GCSE Geography | Weather Hazards 6", T2U),
    ]),
    ("geog:3.1.1.3b", &[
        ("AIZgRjERPgs", "Typhoon Haiyan 2013 (Tropical Storm Case Study) | AQA GCSE 9-1 Geography", KED),
        ("luG3Wh76ikk", "Tropical Storms & Typhoon Haiyan, 2013 - SUNDAY MORNING COFFEE - AQA GCSE 9-1 Geography 2021", MRB),
        ("rNPUPrHwuxA", "Reducing the effects of tropical storms | AQA GCSE 9-1 Geography", KED),
    ]),
    ("geog:3.1.1.3c", &[
        ("lFNHFC4ZBgI", "Extreme Weather in the UK | AQA GCSE Geography | Weather Hazards 11", T2U),
        ("rB4g0pE4Tw8", "Somerset Levels Flooding | AQA GCSE Geography | Weather Hazards 12", T2U),
        ("FH8E2Q3qfJg", "How Will Extreme Weather Affect the UK? | AQA GCSE Geography | Weather Hazards 15", T2U),
    ]),
    ("geog:3.1.1.4", &[
        ("MMSIp7l2vbU", "Climate Change - Evidence, Causes & Effects | AQA GCSE 9-1 Geography", KED),
        ("EglASOE5eBo", "Human Causes of Climate Change | AQA GCSE Geography | Climate Change 4", T2U),
        ("pasfzie_28Q", "Managing Climate Change - Mitigation & Adaptation | AQA GCSE 9-1 Geography", KED),
    ]),
    ("geog:3.1.2.1", &[
        ("t4Pbt0a7E_I", "What is an Ecosystem? | AQA GCSE Geography | Ecosystems 1", T2U),
        ("PlBTnEnhR44", "Small-scale Ecosystems | AQA GCSE Geography | Ecosystems 3", T2U),
        ("f2vt9mh7dfc", "Global Biomes | AQA GCSE Geography | Ecosystems 4", T2U),
    ]),
    ("geog:3.1.2.2a", &[
        ("EiBJ6fFDoAU", "What are Tropical Rainforests Like? | AQA GCSE Geography | Tropical Rainforests 1", T2U),
        ("AoI_ueDjC-s", "Adapting to the Tropical Rainforest | AQA GCSE Geography | Tropical Rainforests 2", T2U),
        ("15osu_Ga4-A", "The Tropical Rainforest's Interdependence - AQA GCSE Geography", "Mr Sheehan Geography"),
    ]),
    ("geog:3.1.2.2b", &[
        ("7lW_CxGYtSA", "Malaysian Rainforest (Rainforest Case Study) | AQA GCSE 9-1 Geography", KED),
        ("AItfFfU_8NA", "Paper 1 Section B - The Living World: Deforestation in Malaysia - Causes, Impacts and Management", AUDEN),
        ("teezEfPopYI", "Sustainable Management of Tropical Rainforests | AQA GCSE 9-1 Geography", KED),
    ]),
    ("geog:3.1.2.3a", &[
        ("umj-z3Q5154", "What are Hot Deserts Like? | AQA GCSE Geography | Hot Deserts 1", T2U),
        ("uGRvnUVkwec", "Adapting to Hot Deserts | AQA GCSE Geography | Hot Deserts 2", T2U),
        ("m0sc8-F2PmI", "How have plants and animals adapted to hot deserts?", "Internet Geography"),
    ]),
    ("geog:3.1.2.3b", &[
        ("0K-7Um7Wi_A", "GCSE, Hot Deserts  - The Sahara, Challenges to Developments", "Horizon Education"),
        ("uxeqjW0U9-w", "Sahel and Desertification AQA GCSE Geography", "mrcoolegeography"),
        ("4xls7K_xFBQ", "Why is Africa building a Great Green Wall? BBC News", "BBC News"),
    ]),
    ("geog:3.1.3.1", &[
        ("42mbHe0Epyg", "Physical Landscapes in the UK | AQA GCSE 9-1 Geography", KED),
        ("O_Rsbt_Yg9w", "UK relief, rivers and landscapes", "Rob Gamesby"),
        ("oOvm4c8O73E", "The UK's Physical Landscape: The Basics", "Simple Geography"),
    ]),
    ("geog:3.1.3.2a", &[
        ("JC47DJU4gWE", "What Affects Waves? | AQA GCSE Geography | Coastal Landscapes 1", T2U),
        ("l20SZC3O090", "Weathering and Mass Movement | AQA GCSE Geography | Coastal Landscapes 2", T2U),
        ("6z2N8Mtv_kw", "Erosion, Transportation and Deposition | AQA GCSE Geography | Coastal Landscapes 3", T2U),
    ]),
    ("geog:3.1.3.2b", &[
        ("JO1QeONbGCk", "COASTAL LANDFORMS at SWANAGE  | GCSE Geography Revision | 100 Day Exam Countdown 10.6", "Mrs B Geography"),
        ("gIUThLF7bIw", "Landforms of Erosion: Caves, Arches, Stacks and Stumps | AQA GCSE Geography Coastal Landscapes 5", T2U),
        ("MGHissPs180", "Landforms of Deposition: Spits | AQA GCSE Geography | Coastal Landscapes 9", T2U),
    ]),
    ("geog:3.1.3.2c", &[
        ("Z2CsQjliQq4", "Coastal Management - Hard & Soft Engineering, Managed Retreat | AQA GCSE 9-1 Geography", KED),
        ("goIBgpLn7yc", "Coastal Management/Engineering Strategies at Lyme Regis - OMG Revision – GCSE Geography 9-1", "OMG Revision"),
        ("x7jemyJujg8", "Medmery managed realignment scheme", "Environment Agency"),
    ]),
    ("geog:3.1.3.3a", &[
        ("67JGxLsi8oM", "The River's Long Profile | AQA GCSE Geography | River Landscapes 2", T2U),
        ("mTIQDg1bV2I", "The River's Cross Profile | AQA GCSE Geography | River Landscapes 3", T2U),
        ("5JBVCaDntzI", "Fluvial Erosion, Transportation and Deposition | AQA GCSE Geography | River Landscapes 1", T2U),
    ]),
    ("geog:3.1.3.3b", &[
        ("gDnGumb5NrY", "River Landforms - Waterfalls, Meanders, Oxbow Lakes, Levees & More | AQA GCSE 9-1 Geography", KED),
        ("nvPYWbIq8jE", "River Tees (River Landforms Case Study) | AQA GCSE 9-1 Geography", KED),
        ("Nb5HNFdjcLo", "River Tees Case Study – Upper to Lower Course Explained | AQA GCSE Geography", "Hums Mums"),
    ]),
    ("geog:3.1.3.3c", &[
        ("5ThPRongOzU", "Flood Hydrographs | AQA GCSE Geography | River Landscapes 9", T2U),
        ("udCezdUJoXc", "Flood Management: Soft & Hard Engineering | AQA GCSE 9-1 Geography", KED),
        ("c1ZpmItlvkQ", "BANBURY FLOOD ALLEVIATION SCHEME CASE STUDY - AQA GCSE 9-1 Geography 2020", MRB),
    ]),
    ("geog:3.2.1a", &[
        ("7lwllykzeKo", "What is Urbanisation? | AQA GCSE Geography | Urbanisation 1", T2U),
        ("llZ2-q_paoE", "Urbanisation: Natural Increase & Push/Pull Factors | AQA GCSE 9-1 Geography", KED),
        ("REcu0XG0iqc", "Megacities | AQA GCSE Geography | Urbanisation 2", T2U),
    ]),
    ("geog:3.2.1b", &[
        ("BHSo0mT9naY", "Rio de Janeiro Urbanisation Case Study - SUNDAY MORNING COFFEE - AQA GCSE 9-1 Geography 2021", MRB),
        ("lk0efjOZPHo", "Why is Rio de Janeiro Important? | AQA GCSE Geography | Rio de Janeiro (Brazil) City Study 1", T2U),
        ("H7Eh8jmS1pw", "Paper 2 Section A - Urban Issues and Challenges: The Favela Bairro Project", AUDEN),
    ]),
    ("geog:3.2.1c", &[
        ("eulPP0MthKI", "Paper 2 Section A - Urban Issues and Challenges: Opportunities and Challenges in Bristol", AUDEN),
        ("SeTfq_lMlYQ", "Bristol - AQA GCSE Geography Paper 2 Case Study", GCS),
        ("yFCUbhr4YEw", "Where do people live in the UK (Urban Change in the UK) – OMG Revision – GCSE Geography 9-1", "OMG Revision"),
    ]),
    ("geog:3.2.1d", &[
        ("NgvMGi9hAAo", "3 - Temple Quarter Regeneration Project - AQA GCSE GEOGRAPHY - BRISTOL CASE STUDY - Revision", MRB),
        ("7HS_TdDphjc", "Freiburg, Germany (Sustainable Urban Living Example) | AQA GCSE 9-1 Geography", KED),
        ("a_4U9KP6ZMw", "4) Sustainable urban transport strategies. Powered by @GeographyHawks", HAWKS),
    ]),
    ("geog:3.2.2a", &[
        ("LbQ4G0JD-D8", "Classifying Development | AQA GCSE Geography | Development Gap 1", T2U),
        ("7ckGMBrEVzw", "Measuring Development | AQA GCSE 9-1 Geography", KED),
        ("6YYIXoywGm8", "Demographic Transition Model | AQA GCSE Geography | Development Gap 5", T2U),
    ]),
    ("geog:3.2.2b", &[
        ("r7DOV2ZGzCM", "Uneven Development | AQA GCSE 9-1 Geography", KED),
        ("2BK3L7Ctmj4", "Reducing the development gap | GCSE GEOGRAPHY", "No Waffle GCSE"),
        ("73FvZ5-DlAk", "Tourism Reducing the Development Gap, Jamaica Case Study - AQA GCSE Geography", "Mr Sheehan Geography"),
    ]),
    ("geog:3.2.2c", &[
        ("TmirbTQRPrQ", "Introduction to Nigeria - a NEE - AQA GCSE Geography Unit 2B", HAWKS),
        ("S6RQMNR70tI", "NIGERIA CASE STUDY - NEE Example - SUNDAY MORNING COFFEE - AQA GCSE 9-1 Geography 2022", MRB),
        ("FV2eOG7PbMo", "Paper 2 Section B - The Changing Economic World: Shell in Nigeria Case Study", AUDEN),
    ]),
    ("geog:3.2.2d", &[
        ("9obg_rnGTJw", "The UK's Changing Employment Structure | AQA GCSE Geography | UK Economic Futures 1", T2U),
        ("5M1CLvWatbs", "UK Rural Population Change | AQA GCSE Geography | UK Economic Futures 6", T2U),
        ("_zgjuy0advs", "UK North South Divide | AQA GCSE Geography | UK Economic Futures 7", T2U),
    ]),
    ("geog:3.2.3.1", &[
        ("C-XH8hqMePQ", "Global Access to Resources | AQA GCSE Geography | Global Resources 1", T2U),
        ("o_ciFkYWTvo", "UK Food, Water and Energy", GCS),
        ("zobzSFrQyqM", "UK Energy Mix | AQA GCSE Geography | UK Overview 7", T2U),
    ]),
    ("geog:3.2.3.2a", &[
        ("bH13o72Wjlo", "Global Demand for Food | AQA GCSE Geography | Food 1", T2U),
        ("HfOSq8ILL_4", "Causes of Food Insecurity | AQA GCSE Geography | Food 2", T2U),
        ("yDccTm2Wv9U", "Impacts of Food Insecurity | AQA GCSE Geography | Food 3", T2U),
    ]),
    ("geog:3.2.3.2b", &[
        ("Zz7y7lEKFag", "Increasing Food Supply | AQA GCSE Geography | Food 4", T2U),
        ("irg6H2rpfwE", "Large-scale Agriculture Case Study: Indus River Basin | AQA GCSE Geography | Food 6", T2U),
        ("jGgrGxl9A1o", "Sustainable Farming Case Study: Makueni Food and Water Security | AQA GCSE Geography | Food 10", T2U),
    ]),
    ("geog:3.3.1", &[
        ("xR_Uyk6fpYo", "AQA Geography Paper 3 - Section A", "Geography Juice"),
        ("fotZoKsyYSs", "How to answer a 9 marker! - GCSE GEOGRAPHY 9 Markers - Exam technique", MRB),
        ("7fQJ1S6Ydis", "How to answer Geography Questions (Top tips for 4, 6 and 9 mark questions) AQA GCSE Geography", GCS),
    ]),
    ("geog:3.3.2", &[
        ("iTP12kbUvFE", "AQA Geography Paper 3 - Fieldwork Examples", "Geography Juice"),
        ("Gl6sPzd2gy8", "GCSE Geography | River Fieldwork | Bitesize | GCSE Revision", "BBC Bitesize - GCSE Revision Support"),
        ("a3SjbEpC-QM", "Unseen Fieldwork - AQA Geography GCSE Paper 3", "MrVisGeography"),
    ]),
    ("geog:3.4a", &[
        ("YMeVobilUxo", "Ordnance Survey Maps - GEOGRAPHY BASICS", MRB),
        ("Spi-7sT2Y5E", "4 & 6 Figure Grid References - GEOGRAPHY BASICS", MRB),
        ("4i_6eToM3X8", "Understanding contour lines with Steve Backshall and Ordnance Survey", "Ordnance Survey"),
    ]),
    ("geog:3.4b", &[
        ("_u0cZ-MXMoQ", "How to answer graph questions AQA GCSE Geography", "NDAGeography"),
        ("MczCAN9hSEI", "Calculating the Interquartile Range", "Pimlico Geography"),
        ("KnA9Xdv2I8g", "Percentage change in Geography", "lfata geography"),
    ]),
    // ---------- French (AQA GCSE 8652) ----------
    ("fre:3.1.1a", &[
        ("nQH0JWXA2v4", "Identity and relationships", COLLINS),
        ("LianvgQORh8", "Talking about family relationships", ALEXA),
        ("Eqraw7_JJRc", "Describing your best friend", ALEXA),
    ]),
    ("fre:3.1.1b", &[
        ("gLXBXgyh5UI", "Healthy living and lifestyle", COLLINS),
        ("E1Cuy4a5Ee4", "What you do to keep healthy", ALEXA),
        ("FHkGo7-rDAw", "Vocabulary for health problems", IDEAL),
    ]),
    ("fre:3.1.1c", &[
        ("h3y4ChZih3g", "Education and work", COLLINS),
        ("qN6qQABYtdg", "School vocabulary", ALEXA),
        ("ApOpvOmQU9U", "Jobs vocabulary", ALEXA),
    ]),
    ("fre:3.1.2a", &[
        ("xl2FWFNftv8", "Free-time activities", COLLINS),
        ("sqlbmwu4pJ8", "What you like doing in your spare time", ALEXA),
    ]),
    ("fre:3.1.2b", &[
        ("J-2vN0_Pt1c", "Customs, festivals and celebrations", COLLINS),
        ("jDBkqqhT3Bg", "Your favourite celebration and why", ALEXA),
    ]),
    ("fre:3.1.2c", &[
        ("WY9a9VP42E0", "Celebrity culture", COLLINS),
        ("UGtRUKD7P2k", "Listening practice on celebrity culture", IDEAL),
    ]),
    ("fre:3.1.3a", &[
        ("QJWNS5a4pbo", "Travel and tourism", COLLINS),
        ("jH3oZzf6z8k", "Your ideal holiday", ALEXA),
    ]),
    ("fre:3.1.3b", &[
        ("WShudSFcWhg", "Media and technology", COLLINS),
        ("Hf8PUjF3_1I", "The dangers of the internet", ALEXA),
        ("qxFXklF5qIA", "Phrases for social media", ALEXA),
    ]),
    ("fre:3.1.3c", &[
        ("wNfaKcWXZgQ", "The environment and where people live", COLLINS),
        ("-m-GCjXI84M", "Town or countryside?", ALEXA),
        ("psh5jpEufKo", "Environmental problems", EVERLEARNER),
    ]),
    ("fre:3.2.1a", &[
        ("CvkiPiW32hc", "Definite, indefinite and partitive articles", ALEXA),
        ("V7AjuIDn4oU", "Masculine or feminine? Noun gender", ALEXA),
        ("6edld_vN7VA", "The partitive: du, de la, des and de", ALEXA),
    ]),
    ("fre:3.2.1b", &[
        ("y8jiGE2uj_w", "Ce, cet, cette, ces", ALEXA),
        ("jXt-dAm6_-U", "Possessive adjectives: mon, ma, mes", ALEXA),
        ("2aFGlzmmVu4", "Direct object pronouns", ALEXA),
    ]),
    ("fre:3.2.1c", &[
        ("r7lt91QIQzY", "The present tense", NOWAFFLE),
        ("sgr9wgYAejs", "Simple negatives", ALEXA),
        ("o0tgXagvolU", "Asking questions with est-ce que", ALEXA),
    ]),
    ("fre:3.2.1d", &[
        ("I9owFJ1Z7fw", "How to form the perfect tense", IMSTUCK),
        ("R7CMGVsanu8", "Etre or avoir in the perfect tense", ALEXA),
        ("vmP4ISMLx-A", "The perfect tense, part 2", NOWAFFLE),
    ]),
    ("fre:3.2.1e", &[
        ("6HDCdU3yJtk", "Near future or simple future?", ALEXA),
        ("47vrMM7xIZo", "How to form the imperfect tense", IMSTUCK),
        ("tHb1vDbJxzk", "The imperative", GCSEOC),
    ]),
    ("fre:3.2.1f", &[
        ("3uWCHZ8NeQo", "Modal verbs", GCSEOC),
        ("DybxlyAQrxo", "Reflexive verbs", ALEXA),
        ("SDLhHuGZayU", "Falloir: il faut", ALEXA),
    ]),
    ("fre:3.2.1g", &[
        ("I1RUF472SFY", "Adjectives before or after the noun", ALEXA),
        ("7C-5_PllPuo", "Adjective agreement", EVERLEARNER),
        ("Mj15n078_XM", "Comparatives: plus, moins, aussi", ALEXA),
    ]),
    ("fre:3.2.1h", &[
        ("iEyvIzPKuIY", "French prepositions", ALEXA),
        ("yI_iWNJ3kzQ", "A, en, au, aux and chez", ALEXA),
        ("T17zerugsDI", "Dans, sous, sur, devant", ALEXA),
    ]),
    ("fre:3.2.2a", &[
        ("OHQPlvD1ypc", "The pronouns y and en", ALEXA),
        ("11KUkGmjZ30", "Emphatic pronouns: moi, toi, lui", DYLANE),
        ("GxxyR8P0pOI", "Relative pronouns qui and que", ALEXA),
    ]),
    ("fre:3.2.2b", &[
        ("GXu34g0DldQ", "How to form the future tense", IMSTUCK),
        ("QQLoyENxrnU", "The conditional", NOWAFFLE),
        ("JK5OMjjAc8A", "Imperfect or perfect?", ALEXA),
    ]),
    ("fre:3.2.2c", &[
        ("F2jUqNyXpaI", "Depuis with the present tense", GCSEOC),
        ("yAJv36ZGtYM", "Venir de and etre en train de", ALEXA),
        ("iGbdNzaqv2I", "The present participle: en + -ant", ALEXA),
    ]),
    ("fre:3.2.2d", &[
        ("uwIZHmbiMIE", "More negatives: ne plus, ne que, ne personne", DYLANE),
        ("n2g63PMCqiE", "The passive", GCSEOC),
        ("Ne431tdamsk", "Comparatives and superlatives", DYLANE),
    ]),
    ("fre:4.4a", &[
        ("Kd8QFUepOEE", "Top tips for the listening exam", GCSE_ALEXA),
        ("Idi2WO6mMus", "Listening Section A: positive or negative", GCSE_ALEXA),
    ]),
    ("fre:4.4b", &[
        ("H0psUcYBf90", "The dictation exercise", GCSE_ALEXA),
        ("0mC2zRtx8h8", "French vowel sounds", ALEXA),
    ]),
    ("fre:4.5", &[
        ("dDVgw3cd6jk", "The AQA speaking exam explained", "astarfrench"),
        ("vUFs3hp2vTY", "Full marks in the role-play", IDEAL),
        ("5MyAvoL79-U", "The photo card", GCSE_ALEXA),
    ]),
    ("fre:4.6", &[
        ("pOc6shgAm8s", "Top tips for the reading exam", GCSE_ALEXA),
        ("IbcM6Gyo8us", "Reading Section B: translation into English", GCSE_ALEXA),
    ]),
    ("fre:4.7", &[
        ("l5Yk23cCs4A", "Top tips for the writing exam", GCSE_ALEXA),
        ("A7bVJfRebig", "Writing Section B, Theme 1", GCSE_ALEXA),
        ("MqNFCjfrLlw", "How to get full marks in writing", IDEAL),
    ]),
    // ---------- Spanish (AQA GCSE 8692) ----------
    ("spa:3.1.1a", &[
        ("3ynFA-uR5dc", "Identity and relationships", COLLINS),
        ("xUq9hdPZ94M", "Family, friends and relationships revision", ASTARES),
        ("TtA_4FQDuAo", "Family and relationships vocabulary", MYGCSEES),
    ]),
    ("spa:3.1.1b", &[
        ("aCpSbHo3KEg", "Healthy living and lifestyle", COLLINS),
        ("YDzOqG8UL_o", "Healthy living and lifestyle revision", ASTARES),
    ]),
    ("spa:3.1.1c", &[
        ("HCGe7PE8OUo", "Education and work", COLLINS),
        ("Vn0O7Teek4A", "School and education revision", ASTARES),
    ]),
    ("spa:3.1.2a", &[
        ("LQW-D6ebBOU", "Free-time activities", COLLINS),
        ("uo8KIoA2kkA", "Free-time activities revision", ASTARES),
    ]),
    ("spa:3.1.2b", &[
        ("ECrAuoMIU54", "Customs, festivals and celebrations", COLLINS),
        ("9RWYJ2naqSs", "Customs and festivals revision", ASTARES),
    ]),
    ("spa:3.1.2c", &[
        ("_p_SqUYgsIU", "Celebrity culture", COLLINS),
        ("G6UOFKWP9o0", "Celebrity culture revision", ASTARES),
        ("uVvUGxtTfUs", "Speaking practice: celebrities", ASTARES),
    ]),
    ("spa:3.1.3a", &[
        ("x9-p4HOB34w", "Travel and tourism", COLLINS),
        ("zRKDguMysIQ", "Speaking practice: holidays", ASTARES),
    ]),
    ("spa:3.1.3b", &[
        ("u00GAQGTiBc", "Media and technology", COLLINS),
        ("KTyk27WJHpM", "Speaking practice: social media", ASTARES),
    ]),
    ("spa:3.1.3c", &[
        ("NXRHScAuelE", "The environment and where people live", COLLINS),
        ("EBHhPbLZiCk", "Speaking practice: what there is in your region", ASTARES),
    ]),
    ("spa:3.2.1a", &[
        ("TRMiMw4K5lo", "Noun gender", COLLINS),
        ("YeTIwDcKwZ4", "Definite and indefinite articles", LANGTUTOR),
        ("rUaX5OqTEzE", "Possessive adjectives: mi, tu, su", JORDAN),
    ]),
    ("spa:3.2.1b", &[
        ("PdFcezn9naY", "Pronouns", COLLINS),
        ("hVXSusr9nTg", "Direct object pronouns: lo, la, los, las", JORDAN),
        ("g4UzE8c2wik", "This, these, that and those", LANGTUTOR),
    ]),
    ("spa:3.2.1c", &[
        ("XWmVFzWXupk", "Regular -ar, -er and -ir verbs in the present", "Spanish with James"),
        ("-sv8B4oy_0w", "O to ue stem-changing verbs", JORDAN),
        ("BwSn383ghms", "Irregular yo forms: tengo, hago, salgo", JORDAN),
    ]),
    ("spa:3.2.1d", &[
        ("ZwTPwRMjLD8", "-ar verbs in the preterite", LANGTUTOR),
        ("bY_STs07NG4", "Past tenses", COLLINS),
        ("2vOuHl1wQsU", "Irregular preterite verbs", MYGCSEES),
    ]),
    ("spa:3.2.1e", &[
        ("ntY7ziEsxpI", "The present continuous", LANGTUTOR),
        ("95GJjXY2s88", "The present perfect: he jugado", SACAPUNTAS),
        ("AmnTX30VliE", "The imperfect tense", LANGTUTOR),
    ]),
    ("spa:3.2.1f", &[
        ("jxgOIGl219E", "The future or ir a + infinitive", JORDAN),
        ("nRaMf1Y1TCM", "The conditional", LANGTUTOR),
        ("C2UnO5khpi4", "Giving commands", LANGTUTOR),
    ]),
    ("spa:3.2.1g", &[
        ("SAfXpyZlz-I", "How to use gustar", LANGTUTOR),
        ("_uH_tosBLyo", "Reflexive verbs", LANGTUTOR),
        ("HCqsdkwpBAI", "Se puede and hay que + infinitive", "El Blog para aprender español"),
    ]),
    ("spa:3.2.1h", &[
        ("zV-XLyuyDyo", "Ser or estar?", "Learn Spanish with SpanishPod101.com"),
        ("U74ClJsbfb0", "Comparatives and superlatives", LANGTUTOR),
        ("RFpYe7hemVo", "The -ísimo ending", JORDAN),
    ]),
    ("spa:3.2.1i", &[
        ("hXkTwRWpyAU", "Por or para?", "Lingo Learner"),
        ("qY6DgSpSiR0", "The personal a", JORDAN),
        ("o88gkstA0ds", "Diminutives and augmentatives", LANGTUTOR),
    ]),
    ("spa:3.2.2a", &[
        ("OL86D_omkSQ", "Possessive pronouns", LANGTUTOR),
        ("4URFWAOaL64", "How to use lo que", "Real Fast Spanish"),
        ("quzRXk0oKp8", "Prepositional pronouns: conmigo, contigo", LANGTUTOR),
    ]),
    ("spa:3.2.2b", &[
        ("3nVHhqblh88", "Preterite or imperfect?", "The Spanish Dude"),
        ("G86u9YrJc9s", "Preterite stem-changers: e to i", JORDAN),
        ("U42loE1zhdw", "The future tense", LANGTUTOR),
    ]),
    ("spa:3.2.2c", &[
        ("pG_2m9_sTTY", "Introduction to the present subjunctive", JORDAN),
        ("-MZwa46X2C4", "The subjunctive in five minutes", "Breakthrough Spanish"),
        ("KB4WG7SXAVA", "Para que + subjunctive", QROOPAUL),
    ]),
    ("spa:3.2.2d", &[
        ("1BZalafcGNk", "Acabar de + infinitive", JORDAN),
        ("W62TVclkgG0", "Seguir + gerund", QROOPAUL),
        ("x1sh5raIbwo", "The passive voice", LANGTUTOR),
    ]),
    ("spa:3.2.3", &[
        ("hsLYD1Jyf3A", "Spanish letters and sounds", "Butterfly Spanish"),
        ("dvE_OCRHOhs", "Which syllable to stress, and accents", "Coffee Break Spanish"),
        ("DHfegU4_g9U", "The hardest sounds in Spanish", SACAPUNTAS),
    ]),
    ("spa:4.4", &[
        ("TeTu47OD1c8", "Last-minute tips for reading and listening", ASTARES),
        ("Q7QtY9VZ3rc", "Dictation practice: technology and social media", "We Teach MFL"),
        ("NKJbcx2LmKE", "Short listening practice", ASTARES),
    ]),
    ("spa:4.5", &[
        ("6kpipp7CDOI", "Full marks in the AQA speaking exam", ASTARES),
        ("JjvuQuX4xj0", "Full marks in the role-play", MYGCSEES),
        ("8KGB63Vjzag", "How to describe a photo", ASTARES),
    ]),
    ("spa:4.6", &[
        ("j5jTOGmbwqU", "Reading practice", ASTARES),
        ("VraN9bIec6c", "Vocabulary for reading and listening", ASTARES),
    ]),
    ("spa:4.7", &[
        ("JOM3_SpAzKE", "AQA Higher writing paper walkthrough", ASTARES),
        ("jqHlSUxYqvo", "Full marks in the translation into Spanish", ASTARES),
        ("mZHOiEs6wWk", "The 150-word task: model answer", MYGCSEES),
    ]),
    // ---------- German (AQA GCSE 8662) ----------
    ("ger:3.1.1a", &[
        ("3GQKN7LRjLA", "Listening practice: family", IDEAL),
        ("R1ZLm9E-9nw", "Family members and relatives", YGT),
        ("rmS00c5DsY4", "Describing people", "Spring German - Learn German with Chunks"),
    ]),
    ("ger:3.1.1b", &[
        ("G5hfTT98Oxc", "Listening practice: a healthy lifestyle", IDEAL),
        ("kYbg2rgq2W0", "Healthy food and drink vocabulary", IDEAL),
        ("505AzsYTrHc", "Listening practice: health and happiness", IDEAL),
    ]),
    ("ger:3.1.1c", &[
        ("95mtun_RIh0", "Listening practice: school", IDEAL),
        ("lvYIxKye_7s", "School vocabulary", LEARNGERMAN),
        ("Sa0whvtYau8", "Dream jobs and careers", LEARNGERMAN),
    ]),
    ("ger:3.1.2a", &[
        ("lYHKQnGrtLM", "Talking about your hobbies", YGT),
        ("dVb_VwVYehs", "Hobbies vocabulary", LEARNGERMAN),
        ("ueBmlrZbmwA", "Ordering in a restaurant", LEARNGERMAN),
    ]),
    ("ger:3.1.2b", &[
        ("wz2Ak8KJdWk", "Listening practice: celebrations and festivals", IDEAL),
        ("1E1CI3917ss", "Five German Christmas traditions", "DW History and Culture"),
    ]),
    ("ger:3.1.2c", &[
        ("UMWGkTusZhs", "Listening practice: celebrity culture", IDEAL),
        ("2WpIFWkDkGc", "Theme 2 conversation: model answer", HERRREID),
    ]),
    ("ger:3.1.3a", &[
        ("p8nJSKWnmxY", "Holidays and travel vocabulary", LEARNGERMAN),
        ("jT8bdI8BCMI", "Holiday vocabulary", FERGUSON),
        ("OL2K9_wJKMw", "Speaking practice: holidays", MUGRIDGE),
    ]),
    ("ger:3.1.3b", &[
        ("GG5HjY1Fe2M", "Computer and internet vocabulary", LEARNGERMAN),
        ("wa4oEGnjocs", "Mobile phone vocabulary", LEARNGERMAN),
        ("d9SS1y-HNmc", "Theme 3 conversation: model answer", HERRREID),
    ]),
    ("ger:3.1.3c", &[
        ("B3HJ5xnv75Q", "Listening practice: the environment", IDEAL),
        ("8etayvawQmg", "The environment and protecting it", LEARNGERMAN),
        ("RMH85sPUDHY", "Town or country? Pros and cons", LEARNGERMAN),
    ]),
    ("ger:3.2.1a", &[
        ("61_33WIZs9c", "Noun genders", FERGUSON),
        ("pb5CySTKP8w", "The German cases", BAUSTEINE),
        ("l4tinKah6GE", "The accusative case", FERGUSON),
    ]),
    ("ger:3.2.1b", &[
        ("MLgrCuKSMPE", "Possessive adjectives", FERGUSON),
        ("GHRHSR-5thc", "Accusative pronouns: mich, dich", YGT),
        ("gh8zSONIDoA", "Relative pronouns", BAUSTEINE),
    ]),
    ("ger:3.2.1c", &[
        ("k3zSbed5zZg", "The present tense", GCSEGER),
        ("x-9jPdkb_94", "Strong and stem-changing verbs", FERGUSON),
        ("HdDmddOPs5I", "Asking questions", FERGUSON),
    ]),
    ("ger:3.2.1d", &[
        ("jR4XeQxwGHQ", "Basic word order", MUGRIDGE),
        ("lpezJZoxTOs", "Separable verbs", MUGRIDGE),
        ("vfAWcUDScGM", "Nicht or kein?", MUGRIDGE),
    ]),
    ("ger:3.2.1e", &[
        ("9EozuuoKhcw", "The perfect tense", GCSEGER),
        ("X-Ry2ifoOIc", "Remembering the perfect tense", FERGUSON),
        ("WC6klvZ4pWc", "War and hatte", FERGUSON),
    ]),
    ("ger:3.2.1f", &[
        ("OdTjoC-m6rE", "The future tense", GCSEGER),
        ("N9L5X2Xf-Bs", "Modal verbs", MUGRIDGE),
        ("C7TPk1yDBH8", "Um ... zu", MUGRIDGE),
    ]),
    ("ger:3.2.1g", &[
        ("SXKD5bQl-zQ", "Adjective endings: the whole system", YGT),
        ("VoBwHY63_RQ", "Comparatives and superlatives", FERGUSON),
        ("c_40J7e1nrc", "Gern, lieber and am liebsten", MUGRIDGE),
    ]),
    ("ger:3.2.1h", &[
        ("OlRQT4V72LM", "Prepositions with the accusative and dative", YGT),
        ("2ERK1-rVgqw", "Dative prepositions", MUGRIDGE),
    ]),
    ("ger:3.2.2a", &[
        ("IsHeKV38SaY", "Weak nouns", FERGUSON),
        ("4BqDoomAERw", "The genitive case", FERGUSON),
        ("4F3oFBa3MuE", "Dative pronouns", YGT),
    ]),
    ("ger:3.2.2b", &[
        ("B54cqfJ6xG0", "The past tense", MUGRIDGE),
        ("J5FYjADg97c", "Modal verbs in the simple past", YGT),
        ("7Q0AjA_BVfs", "The imperative", FERGUSON),
    ]),
    ("ger:3.2.2c", &[
        ("5usFeazBaLY", "The conditional with würde", FERGUSON),
        ("T7BIl5KtHQk", "Hätte and wäre", FERGUSON),
        ("OHimNnDgbkQ", "How to use the passive", HERRREID),
    ]),
    ("ger:3.2.2d", &[
        ("AZecRi-Achc", "Coordination, inversion and subordination", FERGUSON),
        ("Dmv2BzXv_7U", "The nine two-way prepositions", ANJA),
        ("d5IoSISsYyE", "Da- and wo- compounds", FERGUSON),
    ]),
    ("ger:3.2.3", &[
        ("JGh9DR6bxpw", "Ei, ie, au and eu", ANJA),
        ("BoFEG5h7d-o", "How to pronounce umlauts", "Feli from Germany"),
        ("UFUqhT-rYzc", "Long and short vowels", FERGUSON),
    ]),
    ("ger:4.4", &[
        ("CYWQaYxSbws", "Getting a grade 9 in listening", HERRREID),
        ("BWDBr8LFWGU", "Higher dictation practice", IDEAL),
        ("jSZOsYQPzfw", "Listening exam tips", IDEAL),
    ]),
    ("ger:4.5", &[
        ("kxaTkx_DVJE", "The new-spec speaking exam", HERRREID),
        ("0TlPiDGWR-Y", "The read-aloud task", IDEAL),
        ("0VU_tfo_da0", "The speaking exam: all you need to know", HERRREID),
    ]),
    ("ger:4.6", &[("VebSZrHmsI4", "Reading German through cognates", "RobWords")]),
    ("ger:4.7", &[
        ("EUQut8achAQ", "Full marks in the 150-word task", IDEAL),
        ("wSzcwtWex5A", "Full marks in the 90-word task", IDEAL),
        ("ivMn0z6jVYI", "Translation into German", HERRREID),
    ]),
    // History (Pearson Edexcel GCSE History (1HI0))
    // ---------- History (Pearson Edexcel GCSE 1HI0: options 11, B4, P4, 31) ----------
    ("hist:11.1a", &[
        ("tzM7xtIkRnE", "GCSE History Rapid Revision: Medieval Causes of Disease", CLOKE),
        ("ar3ijNoZuf8", "GCSE History Rapid Revision: The Theory of the 4 Humours", CLOKE),
        ("4DYc3m0dc9k", "Edexcel GCSE History Medicine Through Time #2 - Rational Explanations for Disease (Medieval)", "Mr Richards"),
    ]),
    ("hist:11.1b", &[
        ("1FecDvmleMs", "GCSE History Rapid Revision: Medieval Treatments", CLOKE),
        ("jgLNgJPokcg", "GCSE History Rapid Revision: The Black Death", CLOKE),
        ("7IGK5ghBcyM", "Revision: Medieval hospitals", CHSG),
    ]),
    ("hist:11.2a", &[
        ("TJY4B6H_Fug", "GCSE History Rapid Revision: Renaissance Medicine Introduction", CLOKE),
        ("CbNNqHj-f5I", "GCSE History Rapid Revision: Andreas Vesalius", CLOKE),
        ("KIkIM34fVK8", "GCSE History Rapid Revision: Thomas Sydenham", CLOKE),
        ("g_8R4n4QzLY", "Revision: The Royal Society", CHSG),
    ]),
    ("hist:11.2b", &[
        ("y-xgbVy6olE", "GCSE History Rapid Revision: Renaissance Causes and Treatments of Disease", CLOKE),
        ("SHNLbSTRFV8", "Revision: William Harvey", CHSG),
        ("7r15ej0iN1k", "GCSE History Rapid Revision: The Great Plague, 1665", CLOKE),
    ]),
    ("hist:11.3a", &[
        ("oMNIQ_0_yCI", "GCSE History Rapid Revision: Louis Pasteur and Germ Theory", CLOKE),
        ("ZHW6kdeX3cI", "GCSE History Rapid Revision: Robert Koch", CLOKE),
        ("N62VqXEKnSk", "GCSE History Rapid Revision: Edward Jenner and Vaccination", CLOKE),
    ]),
    ("hist:11.3b", &[
        ("hbJ-09ZR9d4", "GCSE History Rapid Revision: 19th Century Advances in Surgery", CLOKE),
        ("_ZDUmZGJiXo", "GCSE History Rapid Revision: Florence Nightingale and 19th Century Hospitals", CLOKE),
        ("ZzhVLS-pXFM", "GCSE History Rapid Revision: 19th Century Public Health", CLOKE),
    ]),
    ("hist:11.4a", &[
        ("GPXI4F5aByE", "GCSE History Rapid Revision: 20th Century Lifestyle and Disease", CLOKE),
        ("REq1ywT0XfQ", "GCSE History Rapid Revision: Magic Bullets", CLOKE),
        ("RsXHqYSIbbY", "GCSE History Rapid Revision: Penicillin", CLOKE),
        ("hsDg9kquqHs", "GCSE History Rapid Revision: DNA", CLOKE),
    ]),
    ("hist:11.4b", &[
        ("UPfufJZzZzM", "The NHS and Treatment and Prevention in the twentieth century", MADDEN),
        ("nAGKsW9CwS0", "How has technology affected diagnosis and treatment?", MADDEN),
        ("gqWy1O2VMfo", "Lung cancer", MADDEN),
    ]),
    ("hist:11.5a", &[
        ("-dIOowtYEf4", "GCSE History Rapid Revision: Major Battles of WWI", CLOKE),
        ("QfAOhH-JYPo", "GCSE History Rapid Revision: Trench Warfare and WWI Medicine", CLOKE),
        ("w_Os19LsIs0", "GCSE History: WWI Medicine - Weapons and Wounds", CLOKE),
    ]),
    ("hist:11.5b", &[
        ("yCQA6MhskDo", "GCSE History Rapid Revision: The Evacuation Chain", CLOKE),
        ("SQuQEyz0Wto", "GCSE History Rapid Revision: Nursing and the RAMC in WWI", CLOKE),
        ("ZVELfv81Py0", "GCSE History Rapid Revision: WWI New Medical Techniques", CLOKE),
    ]),
    ("hist:11.5c", &[
        ("Ffvxk-zy4Dk", "GCSE History Exam Skills - Edexcel Paper 1 Q2a How Useful are the Sources? (8 Marks)", CLOKE),
        ("tf78GZ5pD6I", "Edexcel GCSE History Student Walkthrough - Paper 1 Q2a: Source utility", PEARSONUK),
        ("_pC-N5l4B0E", "Edexcel GCSE History Student Walkthrough - Paper 1 Q2b: Follow up an enquiry", PEARSONUK),
    ]),
    ("hist:B4.1a", &[
        ("sMiLxjCptAU", "GCSE History Rapid Revision: Elizabeth's Challenges at Home and Abroad", CLOKE),
        ("6QQiBA7fUUA", "Early Elizabethan England 1558-1588: The problem of Elizabeth's legitimacy", HISTTEACH),
        ("niPNzy6X6os", "GCSE History Rapid Revision: Elizabeth I - The Virgin Queen", CLOKE),
    ]),
    ("hist:B4.1b", &[
        ("J4luTCHc-tc", "GCSE History Rapid Revision: The Elizabethan Religious Settlement", CLOKE),
        ("-GbkZ_Y1AeQ", "Early Elizabethan England 1558-1588: The Religious settlement", HISTTEACH),
        ("P_SAHTOlNpg", "Early Elizabethan England: The difference between Catholics and Protestants", HISTTEACH),
    ]),
    ("hist:B4.1c", &[
        ("53mYP84AB6g", "GCSE History Rapid Revision: Challenges to the Religious Settlement", CLOKE),
        ("_tD3KvqCc8g", "Early Elizabethan England 1558-1588: Threats to Elizabeth's Religious Settlement", HISTTEACH),
        ("LIZtyIgtVio", "The Problem of Mary Queen of Scots: Early Elizabethan England", HISTTEACH),
    ]),
    ("hist:B4.2a", &[
        ("qLuPzcEON6s", "GCSE History Rapid Revision: Revolt of the Northern Earls, 1569", CLOKE),
        ("mhNxus0ixoA", "GCSE History Rapid Revision - Ridolfi, Throckmorton, Babington Plots (UPDATED)", CLOKE),
        ("jagJpQogoS8", "Early Elizabethan England 1558-1588: The Execution of Mary Queen of Scots", HISTTEACH),
    ]),
    ("hist:B4.2b", &[
        ("S78nvATXBf0", "GCSE History Rapid Revision: Elizabethan England - War with Spain", CLOKE),
        ("xPAKnqCOl_Q", "Early Elizabethan England: Spain and England - Commercial Rivalry", HISTTEACH),
        ("33zs4b3iyyw", "Early Elizabethan England: The Netherlands and Cadiz", HISTTEACH),
    ]),
    ("hist:B4.2c", &[
        ("q1etLovNOaY", "GCSE History Rapid Revision: Elizabethan England - The Spanish Armada", CLOKE),
        ("p5iryutlvrM", "Early Elizabethan England: Reasons and Plans for the Spanish Armada", HISTTEACH),
        ("l11MVbpQ-iQ", "The Spanish Armada: Early Elizabethan England", HISTTEACH),
    ]),
    ("hist:B4.3a", &[
        ("eKyCtKwXBMI", "GCSE History Rapid Revision: Education in Early Elizabethan England", CLOKE),
        ("lh5QMyinSNY", "GCSE History Rapid Revision: Elizabethan Sport, Leisure and Entertainment", CLOKE),
        ("l_XAxCQvWZQ", "Poverty: Causes and Changes - Early Elizabethan England", HISTTEACH),
    ]),
    ("hist:B4.3b", &[
        ("1qy1bJr3x9k", "GCSE History: The Elizabethan Age of Exploration", CLOKE),
        ("Cc-jLvP05Zk", "GCSE History Rapid Revision: Drake's Circumnavigation", CLOKE),
        ("pNmO9wEqxqU", "Sir Walter Raleigh & the Failure of Roanoke - Early Elizabethan England", HISTTEACH),
    ]),
    ("hist:P4.1a", &[
        ("hvQ8i_xym_A", "GCSE History Rapid Revision: Wartime Conferences", CLOKE),
        ("gl7BZUII91s", "Ideologies and Historic Differences: Superpower Relations and the Cold War Edexcel GCSE History", HISTTEACH),
        ("2dIHp54b6og", "GCSE History: Superpower Relations/Cold War- The Kennan and Novikov Telegrams", CLOKE),
    ]),
    ("hist:P4.1b", &[
        ("dgRR8DNKOcM", "GCSE History Rapid Revision: Cold War/Superpower Relation - Truman Doctrine and Marshall Plan", CLOKE),
        ("iaiPIM5Jmgo", "The Truman Doctrine, The Marshall Plan and Stalin's Response - Superpower Relations GCSE History", HISTTEACH),
        ("6XT17mR4_TM", "GCSE History Rapid Revision: Superpower Relations/The Cold War- The Berlin Crisis and Airlift", CLOKE),
    ]),
    ("hist:P4.1c", &[
        ("IGIq7yyW7wQ", "GCSE History Rapid Revision: Superpower Relations/Cold War- The Nuclear Arms Race", CLOKE),
        ("AiGNkwOXz-4", "The Arms Race, The Formation of NATO and the Warsaw Pact - Superpowers Edexcel GCSE History", HISTTEACH),
        ("42Py3a7IdSM", "GCSE History Rapid Revision: Superpower Relations/The Cold War- The Hungarian Uprising 1956", CLOKE),
    ]),
    ("hist:P4.2a", &[
        ("icjKb1BNiaA", "GCSE History Rapid Revision: Superpower Relations and the Cold War - The Berlin Ultimatum and Wall", CLOKE),
        ("y6l8bboOJBk", "The Berlin Ultimatum - Superpower Relations & the Cold War GCSE Edexcel History", HISTTEACH),
        ("E8SF3_TBFNA", "The Berlin Wall - Superpower Relations and the Cold War GCSE Edexcel 9-1", HISTTEACH),
    ]),
    ("hist:P4.2b", &[
        ("YHrdTBe5rKE", "GCSE History Rapid Revision: Superpower Relations and the Cold War- Cuban Revolution & Bay of Pigs", CLOKE),
        ("FVEDrterwVY", "GCSE History Rapid Revision: Superpower Relations and the Cold War - The Cuban Missile Crisis", CLOKE),
        ("o1XefXF4UUM", "13 Days on the Brink: The Cuban Missile Crisis – Superpower Relations & the Cold War GCSE Edexcel", HISTTEACH),
    ]),
    ("hist:P4.2c", &[
        ("ngjikj-7XBc", "GCSE History: Superpower Relations and The Cold War: The Prague Spring and Brezhnev Doctrine", CLOKE),
        ("tv-wwgFJijk", "Crushed Dreams: The Prague Spring (1968) – Superpower Relations & the Cold War GCSE Edexcel History", HISTTEACH),
        ("fzsFSyoX0-A", "Spotlight: Cold War crises in Hungary (1956) and Czechoslovakia (1968)", CHSG),
    ]),
    ("hist:P4.3a", &[
        ("ge1E17HFAD4", "GCSE History: Superpower Relations and the Cold War - Détent, SALT 1+2 and the Helsinki Accords", CLOKE),
        ("297oB9xFAjo", "From Conflict to Compromise: Détente – Superpower Relations & the Cold War GCSE Edexcel History", HISTTEACH),
        ("QtdIpIIvYZ8", "GCSE History Rapid Revision: Superpower Relations/Cold War - Soviet Invasion of Afghanistan", CLOKE),
    ]),
    ("hist:P4.3b", &[
        ("F0GTwVj_pQo", "The Second Cold War Explained | Superpower Relations & the Cold War | Edexcel History GCSE Revision", HISTTEACH),
        ("VwNS-DshCuQ", "GCSE Rapid Revision: Superpower Relations/Cold War - Gorbachev, Perestroika and Glasnost", CLOKE),
        ("TSWsW61NCHA", "GCSE Rapid Revision: Superpower Relations/Cold War - The End of the Soviet Union", CLOKE),
    ]),
    ("hist:31.1a", &[
        ("qA1RUmVVF2E", "GCSE History Rapid Revision: The German Revolution 1918-19", CLOKE),
        ("HvPYXUav-Z8", "GCSE History Rapid Revision: The Weimar Constitution", CLOKE),
        ("0Rl5so_KGr8", "Abdication and Armistice - Weimar and Nazi Germany GCSE", HISTTEACH),
    ]),
    ("hist:31.1b", &[
        ("u5stoytRj0Q", "The Treaty of Versailles and Dolchstoss - Weimar and Nazi Germany GCSE History", HISTTEACH),
        ("fvxlMEN2Agc", "GCSE History Rapid Revision: Uprisings Against the Weimar Republic 1918-19", CLOKE),
        ("z_-xakexp8E", "The Invasion of the Ruhr and Hyperinflation - Weimar and Nazi Germany GCSE", HISTTEACH),
    ]),
    ("hist:31.1c", &[
        ("cuTorPxBL5Y", "GCSE History Rapid Revision: Gustav Stresemann", CLOKE),
        ("z0iW8ChjE48", "Economic Recovery in the 1920s - Weimar and Nazi Germany Edexcel GCSE History", HISTTEACH),
        ("xvrS5EDtedk", "Stresemann's Foreign Policy - Weimar and Nazi Germany GCSE Edexcel", HISTTEACH),
    ]),
    ("hist:31.1d", &[
        ("YSINCa52cac", "GCSE History Rapid Revision: Germany in the 'The Golden Twenties'", CLOKE),
        ("-j52Dx5wUFk", "Changes for workers, women and to culture in the 1920s - Weimar and Nazi Germany GCSE Edexcel", HISTTEACH),
        ("Rv0Pz_YEk9M", "Cultural Changes in Weimar Germany, 1920s | GCSE History | Weimar & Nazi Germany", "A long, long time ago..."),
    ]),
    ("hist:31.2a", &[
        ("qKhN0lD3GUk", "GCSE Rapid Revision- Hitler's Early Life and Early Years of the Nazi Party", CLOKE),
        ("w1tZeS8CtB4", "Munich Putsch - causes, events and short-term consequences - Weimar and Nazi Germany", HISTTEACH),
        ("l8vn5BjqET4", "GCSE History Rapid Revision- Lean Years of the Nazi Party 1924-28", CLOKE),
    ]),
    ("hist:31.2b", &[
        ("bj6RIHbU7bs", "The Wall Street Crash and its impact on Germany - Weimar and Nazi Germany GCSE History", HISTTEACH),
        ("VxVTNbvQI60", "GCSE History Rapid Revision- Why Did People Support the Nazis?", CLOKE),
        ("35L3Ogldq4k", "GCSE History Rapid Revision- How did Hitler Become Chancellor?", CLOKE),
    ]),
    ("hist:31.3a", &[
        ("XALGoKldRAE", "GCSE History Rapid Revision- Building Dictatorship 1933-34", CLOKE),
        ("aqyltSaSjso", "From Chancellor to Fuhrer - Weimar and Nazi Germany GCSE Edexcel History", HISTTEACH),
        ("gZiF32_9wCY", "Overview: Creation of dictatorship 1933-34", CHSG),
    ]),
    ("hist:31.3b", &[
        ("RazvcEmXVpU", "GCSE History Rapid Revision- Control: Terror and the Police State", CLOKE),
        ("exmLJeF4sqM", "GCSE History Rapid Revision- Control: Propaganda", CLOKE),
        ("tbnQ1pCWg2c", "The Nazi Policies towards the Church and Resistance - Weimar and Nazi Germany GCSE Edexcel History", HISTTEACH),
    ]),
    ("hist:31.3c", &[
        ("vTyk-k8PfHg", "GCSE History Rapid Revision- Opposition and Resistance to the Nazis", CLOKE),
        ("SaIXbcwnyJ4", "Opposition and Resistance - Weimar and Nazi Germany Edexcel GCSE History", HISTTEACH),
        ("QiMn8iA1uGY", "Youth Opposition", MADDEN),
    ]),
    ("hist:31.4a", &[
        ("D2L4GQW-nr8", "GCSE History Rapid Revision: Women in Nazi Germany", CLOKE),
        ("HVX52SbDnAM", "GCSE History Rapid Revision: Children and Young People in Nazi Germany", CLOKE),
        ("rNlJUPhW3R0", "Policies towards the Young: Weimar and Nazi Germany Edexcel GCSE History", HISTTEACH),
    ]),
    ("hist:31.4b", &[
        ("iQNYNyl_fEw", "GCSE History Rapid Revision: Work and Employment", CLOKE),
        ("YQUHdGVkRc8", "Employment and Living Standards: Weimar and Nazi Germany Edexcel GCSE History", HISTTEACH),
        ("9xJGnv1iG7E", "GCSE History Rapid Revision: Strength Through Joy", CLOKE),
    ]),
    ("hist:31.4c", &[
        ("lCxAS5gy4I0", "GCSE History Rapid Revision: Nazi Persecution", CLOKE),
        ("_xlM-pIAgqc", "The Nazis Attitudes and Policies toward Minorities - Weimar and Nazi Germany GCSE Edexcel History", HISTTEACH),
        ("mQ1E6wO9jXQ", "What Was Kristallnacht in the Holocaust? | Holocaust Explainer", "United States Holocaust Memorial Museum"),
    ]),
    // ---------- GCSE Music (Eduqas C660QS), Component 3 Appraising ----------
    ("music:MEa", &[
        ("kMvm3hJ3v7o", "Eduqas GCSE Music: musical elements - melody", "HPA Music"),
        ("O7iC_194CIU", "GCSE MUSIC REVISION -  Describing a melody", "P Dillon"),
        ("-7aJjkPTTgU", "Musical Intervals | ABRSM | GCSE Music", "Music Learning Club"),
    ]),
    ("music:MEb", &[
        ("fB1GtVpWgoA", "Eduqas GCSE Music: musical elements - tonality", "HPA Music"),
        ("8OA2Y3mLRv4", "Eduqas GCSE Music: musical elements - harmony", "HPA Music"),
        ("9pxp7IDSpCQ", "GCSE MUSIC REVISION - TONALITY", "P Dillon"),
        ("DDYYjHis7p0", "Elements of Music 2- Harmony and Tonality - GCSE Music", "Music Learning Club"),
    ]),
    ("music:MEc", &[
        ("HyyZrEHNf2o", "Eduqas GCSE Music: musical elements - rhythm", "HPA Music"),
        ("o78-6xNV8So", "Eduqas GCSE Music: musical elements - tempo", "HPA Music"),
        ("oeer-e_xdWQ", "Elements of Music 4 - Tempo, Metre and Rhythm - GCSE Music", "Music Learning Club"),
        ("UqJxXH2voMI", "How to Tell if Music is in Simple Time or Compound Time - Music Theory", "Music Matters"),
    ]),
    ("music:MEd", &[
        ("mulSAumH_M8", "Elements of Music 6 - Sonority (Timbre) - GCSE Music", "Music Learning Club"),
        ("aMQ7pSq3ymM", "Eduqas GCSE Music: musical elements - dynamics", "HPA Music"),
        ("ZH3dTLwBZXM", "Elements of Music 5 - Dynamics and Articulation - GCSE Music", "Music Learning Club"),
        ("UaJBRbk8KGQ", "GCSE Concepts-  Test 1a 5a - Vocal and Instrumental Techniques", "INA Music"),
    ]),
    ("music:MC", &[
        ("1vTuZ0ls65k", "Eduqas GCSE Music Exam Paper Walk Through", "Miss McCall"),
        ("eTRUaOfeGx4", "Musical Elements Revision - GCSE Music Eduqas", "Miss McCall"),
        ("fn8W7Js47yI", "GCSE Music Exam How to answer a Listening Question", "Andrew Moxon"),
    ]),
    ("music:MLa", &[
        ("FMp26uf9gE8", "Elements of Music 8 - Notation - GCSE Music", "Music Learning Club"),
        ("-r8SjCso5Qo", "Let's Read Music 4 - Treble Clef Note Names", "JohnMcAllisterMusic"),
        ("0Sos_zBGo1k", "Let's Read Music 5 - Bass Clef Note Names", "JohnMcAllisterMusic"),
        ("LlNXEaO3CGY", "How to Group Notes and Rests in Simple Time Signatures | ABRSM Music Theory", "Serenity Music Tuition"),
    ]),
    ("music:MLb", &[
        ("xY9Q0R0G2jM", "Key Signatures - Everything You Need To Know in 6 minutes", "Brad Harrison Music"),
        ("G20foMzvczc", "Key Signatures Made Easy", "MusicTheoryAcademy"),
        ("-MdQspoF9wQ", "How to Work Out the Key of a Piece of Music - Music Theory", "Music Matters"),
    ]),
    ("music:MLc", &[
        ("iByJsZ9CnMA", "What are Primary Triads? | Music Theory | ABRSM Grade 4 | Video Lesson", "Liberty Park Music"),
        ("M2skX-SNIvA", "How the Roman Numeral System Works - Music Theory", "Michael New"),
        ("YBHY-0mmKkA", "How Chord Inversions Work - Music Theory", "Music Matters"),
    ]),
    ("music:MLd", &[
        ("uJXKNYecFWk", "GCSE Music Revision - Melodic Dictation", "Baines Music"),
        ("bsYPyb_WF38", "GCSE Music Revision - Rhythmic Dictation", "Baines Music"),
    ]),
    ("music:AoS1a", &[
        ("XPaTjBOj2hI", "GCSE Concepts  - 7a Recognising Baroque Classical & Romantic", "INA Music"),
        ("wXtYLSKXjgo", "OCR GCSE Music Virtual Textbook AoS 2 - 1. Baroque Features", "Flipping Fantastic"),
        ("81POr1RrcCc", "OCR GCSE Music Virtual Textbook AoS 2 - 3. Classical Features", "Flipping Fantastic"),
        ("7VJtknZcQk0", "OCR GCSE Music Virtual Textbook AoS 2 - 5. Romantic Features", "Flipping Fantastic"),
    ]),
    ("music:AoS1b", &[
        ("nQn_bd8o27s", "Form and Structure - Eduqas GCSE Music", "Miss McCall"),
        ("0C3M9vTzlcA", "Elements of Music 3 - Structure - GCSE Music", "Music Learning Club"),
        ("dXdjjmW_qIk", "Analysing the form of the Minuet and Trio - Analysing music (6/8)", "OpenLearn from The Open University"),
        ("JFkr8nCCvaw", "Theme and Variation Form", "Dave Conservatoire"),
    ]),
    ("music:AoS1c", &[
        ("YAwf3rn6z7k", "Melodic Devices - GCSE and Alevel Music", "Mr Luke's Music Room"),
        ("npkO85OJp7g", "Introduction to Sequence and Imitation in Music Theory", "Picardy"),
        ("PZ52ZBrNGCg", "What is an Ostinato? - Explanation with Examples", "MusicHelpGuy"),
    ]),
    ("music:AoS1d", &[
        ("CLqvmbXhXe0", "Cadences - GCSE Music", "Music Learning Club"),
        ("nq1yUQmo2Lg", "GCSE MUSIC REVISION - CADENCES!", "P Dillon"),
        ("3aRBWDHE4g8", "Cadences - The 4 types explained - Perfect, Plagal, Imperfect, Interrupted", "MusicTheoryAcademy"),
        ("C59w4uRTaNE", "Using Alberti Bass as a Compositional Technique - Music Composition", "Music Matters"),
    ]),
    ("music:AoS1e", &[
        ("710fqBgIgoc", "Bach - Badinerie (Eduqas GCSE Music)", "JHA Music"),
        ("PkpxRFBpH2c", "EDUQAS GCSE Music Bach Badinerie revision", "Langdon Academy Music Department"),
        ("-5yf67vQKfo", "GCSE Music | Bach's Badinerie - Analysis of Baroque Style and Structure | Bitesize | GCSE Revision", "BBC Bitesize - GCSE Revision Support"),
        ("qrMNlIKGHg8", "Eduqas GCSE Music: Bach Badinerie Practice Questions", "Miss McCall"),
    ]),
    ("music:AoS2a", &[
        ("_D8WdBgiBtM", "GCSE MUSIC REVISION - Texture", "P Dillon"),
        ("VXzIJbNLDe8", "Elements of Music 7 - Texture - GCSE Music", "Music Learning Club"),
        ("Y1x6tM4wE_U", "GCSE Music Revision - What is Texture?", "Baines Music"),
    ]),
    ("music:AoS2b", &[
        ("_KHjGruWLU8", "The string Quartet explained in less than 5 minutes", "Enjoy Classical Music"),
        ("bqJ_cYjwjOA", "GCSE Music | Chamber music revision", "RevisionBuddy"),
        ("fYvvp1WP5xo", "Trio Sonatas - Introduction for ABRSM Grade 8 Music Theory Candidates", "Victoria Williams (mymusictheory)"),
    ]),
    ("music:AoS2c", &[
        ("o9-0-3aeQbc", "History of Musical Theatre With Mr  Lawrence - Types of Musicals", "Brandon Lawrence"),
        ("6s5_tRFu22A", "What Makes a Song a Musical Theatre Song? (3 Brilliant Examples)", "Brett Boles"),
        ("bC9yL5YdfQQ", "What is a Musical?", "StageAgent"),
    ]),
    ("music:AoS2d", &[
        ("aBg_gQxAShM", "Jazz Fundamentals: What Are the Blues?", "Jazz at Lincoln Center's JAZZ ACADEMY"),
        ("fT4H2xEE9NM", "What Does a Rhythm Section Do in Jazz?", "Jazz at Lincoln Center's JAZZ ACADEMY"),
        ("31JgwfP15kw", "Jazz Fundamentals: What Is Swing?", "Jazz at Lincoln Center's JAZZ ACADEMY"),
    ]),
    ("music:AoS3a", &[
        ("6wUuXWaVyUU", "Film music revision video", "Manningtree Music Department"),
        ("yVVg-95K2nc", "OCR GCSE Music Virtual Textbook AoS 4 - 2. Film Music (Atmosphere)", "Flipping Fantastic"),
        ("as_FkQenP-8", "OCR GCSE Music Virtual Textbook AoS 4 - 4. Film Music (Tension)", "Flipping Fantastic"),
    ]),
    ("music:AoS3b", &[
        ("XacNZ5fRBuI", "OCR GCSE Music Virtual Textbook AoS 4 - 1. Film Music (Leitmotifs)", "Flipping Fantastic"),
        ("A5YejJX_Ccs", "60 Second Guide to Film Music - Leitmotifs", "The Musicologist"),
        ("itMJ-fUPXqE", "How to Transform a Leitmotif", "Sideways"),
    ]),
    ("music:AoS3c", &[
        ("dltdKYUvhhE", "Minimalism and music technology | Music - Howard Goodall's Story of Music", "BBC Bitesize for Teachers"),
        ("vOAwZrsxVnQ", "Minimalism Music Techniques", "musicmsrevision"),
    ]),
    ("music:AoS4a", &[
        ("oXifpcE7ewU", "Learn Popular Music Song Structure", "Mr Morley Music Education"),
        ("SDJwg1JoPtY", "Every type of Song Structure EXPLAINED", "David Bennett Music Theory"),
        ("AGV7Gmnpvv0", "AABA Song Form - Music Theory 101", "McGovern Drums"),
        ("nqA7_0Z1_PY", "12 Bar Blues Explained", "GuiTargetLessons"),
    ]),
    ("music:AoS4b", &[
        ("W2rSTMiXT3Q", "OCR GCSE Music Virtual Textbook AoS 5 - 1. Voices in Pop", "Flipping Fantastic"),
        ("NzVanMOoT3g", "OCR GCSE Music Virtual Textbook AoS 5 - 2. Instruments of Pop", "Flipping Fantastic"),
        ("wecCctwwQOQ", "Using loops and samples | Music - Dev's Music Technology", "BBC Bitesize for Teachers"),
        ("2Wt4OUgQrBk", "Using digital audio effects | Music - Dev's Music Technology", "BBC Bitesize for Teachers"),
    ]),
    ("music:AoS4c", &[
        ("0G-ye4xnqPE", "OCR GCSE Music Virtual Textbook AoS 3 - 2. Bhangra", "Flipping Fantastic"),
        ("7QhQTA8W73I", "Bhangra Music - Rhythms Of The World - OCR GCSE Music", "Music GCSE Revision"),
        ("VI1bKCjgkAY", "GCSE Concepts -  8a Recognising Popular Music, Fusion and Minimalism", "INA Music"),
        ("UKH3bwyDYaM", "What is Fusion Music and how can you define it?", "TOG Music Making"),
    ]),
    ("music:AoS4d", &[
        ("o00iXaSlflk", "Toto - Africa (Eduqas GCSE Music revision)", "JHA Music"),
        ("KBS1vxd06C4", "EDUQAS GCSE Music Toto Africa revision", "Langdon Academy Music Department"),
        ("DAOi9NLbamQ", "Eduqas GCSE Music: Toto Africa Chord Revision Video", "Miss McCall"),
        ("ViZQm1yo1PA", "Eduqas GCSE Music: Toto Africa Practice Questions", "Miss McCall"),
    ]),
    // Religious Studies (AQA GCSE Religious Studies A (8062))
    ("rs:3.1.2.1a", &[
        ("Sxwh9cY44Fk", "Nature of God (AQA GCSE Religious Studies - Christian Beliefs) REVISION", FINLAYSON),
        ("5-EYFQmc_jY", "02 Christian Beliefs The Oneness of God and the Trinity", HARRIS),
        ("r3lcJI2Wrmc", "Creation (AQA GCSE Religious Studies - Christian Beliefs) REVISION", FINLAYSON),
    ]),
    ("rs:3.1.2.1b", &[
        ("39vXQVefNaA", "Afterlife, Judgement & Salvation (AQA GCSE Religious Studies - Christian Beliefs) REVISION", FINLAYSON),
        ("pZ2IUyo5f6Q", "09 Christian Beliefs Different Christian Beliefs about the afterlife", HARRIS),
        ("fEVCQfNKrNQ", "10 Christian Beliefs Judgement", HARRIS),
    ]),
    ("rs:3.1.2.1c", &[
        ("tHAOzbH0Rmc", "Jesus Christ (AQA GCSE Religious Studies - Christian Beliefs) REVISION", FINLAYSON),
        ("b7lP1li1uv0", "06 Christian Beliefs Incarnation, Crucifixion, Resurrection and Ascension", HARRIS),
        ("OsXmmczMD9Y", "08 Christian Beliefs The role of Jesus in Salvation and atonement", HARRIS),
    ]),
    ("rs:3.1.2.2a", &[
        ("dLkRqfICBGs", "Worship & Prayer (AQA GCSE Religious Studies - Christian Practices) REVISION", FINLAYSON),
        ("x9W7r8gD_EI", "01 Christian Practices Prayer and Worship", HARRIS),
        ("68fsYJ7r0Rk", "Prayer. Christianity: Practices. AQA Religious Studies GCSE 8062.", WISEREV),
    ]),
    ("rs:3.1.2.2b", &[
        ("_5KtiDH18gM", "Sacraments (AQA GCSE Religious Studies - Christian Practices) REVISION", FINLAYSON),
        ("VwP-y8sr2vY", "Baptism. Christianity: Practices. AQA Religious Studies GCSE 8062.", WISEREV),
        ("LOclTTHVXuw", "Holy Communion | AQA Christianity", NOWAFFLE),
    ]),
    ("rs:3.1.2.2c", &[
        ("z3Z7c6Nw7eE", "Pilgrimages & Festivals (AQA GCSE Religious Studies - Christian Practices) REVISION", FINLAYSON),
        ("6boceruWNL4", "03 Christian Practices Pilgrimage", HARRIS),
        ("BYDlkbT04_k", "04 Christian Practices Festivals", HARRIS),
    ]),
    ("rs:3.1.2.2d", &[
        ("kPz8QNX8Vbk", "The Church (AQA GCSE Religious Studies - Christian Practices) REVISION", FINLAYSON),
        ("v2Rihd8AT4E", "05 Christian Practices Role of the Church in the local community", HARRIS),
        ("m7KBTcyzd1I", "07 Christian Practices Worldwide Church", HARRIS),
    ]),
    ("rs:3.1.5.1a", &[
        ("jr4m90x0aQI", "01 Islam Beliefs 6 Articles of Faith & Five Roots", HARRIS),
        ("dofjH2Y0-tQ", "02 Islam Beliefs Nature of Allah", HARRIS),
        ("xbQssfoRbGA", "Sunni and Shia | AQA Islam", NOWAFFLE),
    ]),
    ("rs:3.1.5.1b", &[
        ("8WxyWxIb-qU", "Angels | AQA Islam", NOWAFFLE),
        ("Zeb6tANtzJ0", "Predestination | AQA Islam", NOWAFFLE),
        ("PbfCIoHId00", "07 Islam Beliefs Life After Death", HARRIS),
    ]),
    ("rs:3.1.5.1c", &[
        ("dOBz-zJztoA", "04 Islam Beliefs Prophethood", HARRIS),
        ("plLvRRO3K-M", "Holy Books | AQA Islam", NOWAFFLE),
        ("6ftBkumNdBY", "05 Islam Beliefs Imamate", HARRIS),
    ]),
    ("rs:3.1.5.2a", &[
        ("VyElTrvo6_s", "01 Islam Practices 5 Pillars & 10 Obligatory Acts", HARRIS),
        ("xM7_NwTRe48", "02 Islam Practices Shahadah", HARRIS),
        ("nsYAkrUgRZw", "03 Islam Practices Salah", HARRIS),
    ]),
    ("rs:3.1.5.2b", &[
        ("1HgxEUwlf4A", "04 Islam Practices Sawm", HARRIS),
        ("jPeoWM4kqlQ", "05 Islam Practices Almsgiving", HARRIS),
        ("B0QuObF43kU", "Edexcel Religious Studies - Living the Muslim Life - 5 Zakah and Khums", "Miss Morris Manc"),
    ]),
    ("rs:3.1.5.2c", &[
        ("IwRUSqfnY2g", "06 Islam Practices Hajj", HARRIS),
        ("7WKSymvzSOw", "6. Hajj (pilgrimage)", "ColmersRS"),
        ("Ok7-mB62xeE", "What is Hajj? | Religious Studies - My Life, My Religion: Islam", BBCTEACH),
    ]),
    ("rs:3.1.5.2d", &[
        ("zYL5x4mlRpo", "07 Islam Practices Jihad", HARRIS),
        ("c7jIGfSGjDU", "08 Islam Practices Festivals", HARRIS),
        ("qFU9Cb0D6lo", "Ramadan and Eid-ul-Fitr | Religious Studies - My Life, My Religion: Islam", BBCTEACH),
    ]),
    ("rs:3.2.1.1a", &[
        ("dEa3TFxsGxY", "GCSE RS: Theme A.3 Sex before Marriage", NOWAFFLE),
        ("hJRoIDiaMSg", "GCSE RS: Theme A.5 Marriage", NOWAFFLE),
        ("B5fppTry9yo", "SUMMARY AQA Religious Studies A: Divorce and Remarriage", "Miss Appiah R2R (Road to RS)"),
    ]),
    ("rs:3.2.1.1b", &[
        ("3Jo8YZI_xTo", "GCSE RS: Theme A.7 Nature of Families", NOWAFFLE),
        ("EHBPzyDjRms", "GCSE RS: Theme A.8 Purpose of Families", NOWAFFLE),
        ("sNb7GdwVClo", "GCSE RS: Theme A.9 Gender Equality", NOWAFFLE),
    ]),
    ("rs:3.2.1.2a", &[
        ("wak1NXUqGVA", "GCSE RS: Theme B.1 Origins of the Universe", NOWAFFLE),
        ("ToxHJoGXRyM", "GCSE RS: Theme B.3 The Environment", NOWAFFLE),
        ("OuQjZhbqrHg", "GCSE RS: Theme B.4 Animals experimentation", NOWAFFLE),
    ]),
    ("rs:3.2.1.2b", &[
        ("P6v3bS3_58o", "The value of human life. AQA RS GCSE 8062 Thematic Studies, Theme B, Religion and Life", WISEREV),
        ("75JTvi2B1XQ", "GCSE RS: Theme B.6 Abortion", NOWAFFLE),
        ("gJ-tFWsZDxQ", "GCSE RS: Theme B.7 Euthanasia", NOWAFFLE),
    ]),
    ("rs:3.2.1.4a", &[
        ("lvfGKWnmJbc", "GCSE RS: Theme D.4 Just War", NOWAFFLE),
        ("qL-naLCa6gs", "GCSE RS: Theme D.5 Holy War", NOWAFFLE),
        ("WblLnovegek", "GCSE RS: Theme D.6 Pacifism", NOWAFFLE),
    ]),
    ("rs:3.2.1.4b", &[
        ("ZcCwBwrtHlU", "GCSE RS: Theme D.3 Weapons of Mass destruction", NOWAFFLE),
        ("mpbZEBWGd6Y", "Pacifism & Peacemaking. AQA RS 8062 Thematic Studies D, Religion, Peace and Conflict", WISEREV),
        ("KK6_iQ1vxSY", "GCSE RS: Theme D.7 Responses to Victims", NOWAFFLE),
    ]),
    ("rs:3.2.1.5a", &[
        ("pLx5ZtI7F4w", "GCSE RS: Theme E : Intentions", NOWAFFLE),
        ("Iam3C5fzaY4", "GCSE RS: Theme E.2 Reasons for Crime", NOWAFFLE),
        ("yjfI8u3915A", "GCSE RS: Theme E.4 Types of Crime", NOWAFFLE),
    ]),
    ("rs:3.2.1.5b", &[
        ("VK4QXIK6rDI", "GCSE RS: Theme E.5 Aims of Punishment", NOWAFFLE),
        ("_6POMEniCXY", "GCSE RS: Theme E.7 Prison/Community service", NOWAFFLE),
        ("qqo0vYvrSPU", "What are the rights and wrongs of the death penalty? | Religious Studies - Matters of Life and Death", BBCTEACH),
    ]),
    // AQA GCSE Drama 8261. Read off YouTube on 29 September 2026; every id,
    // title and channel checked against YouTube's oEmbed response. George Coles
    // teaches the AQA 8261 paper question by question; Pog Jam, Miss G Drama and
    // Drama Department cover the AQA set plays; the National Theatre, RSC Learning
    // and Frantic Assembly show professional practice. The two Around the World
    // in 80 Days videos summarise Verne's novel, whose plot Laura Eason's play follows.
    ("drama:3.1.1a", &[
        ("k_lm16sOEfo", "Theatre Roles - GCSE Drama AQA", "Adam Goodger"),
        ("JnQe6SkCGJg", "AQA GCSE Drama: Section A", "George Coles"),
        ("frgpWqnsSpk", "Yes Let's...learn about job roles in the theatre!", "Yes Let's"),
    ]),
    ("drama:3.1.1b", &[
        ("N5BhAeHWS8o", "Staging - Proscenium Arch // End On // Thrust Stage // Traverse // In The Round // Promenade Theatre", "Theatre Beard"),
        ("RnmphZQMR50", "Stage Directions Explained: Upstage, Downstage, Stage Left & Stage Right", "Curtains Up Acting Studio"),
        ("hUodZboyK2c", "Stage types for GCSE Drama w/b 20th April", "Miss Garred Drama"),
    ]),
    ("drama:3.1.1c", &[
        ("JV83l2jiWso", "Form, Genre, Structure and Style", "The Drama Classroom"),
        ("fwNt0uXk7Sk", "AQA GCSE Drama Key Terminology", "Kylie Sakura"),
    ]),
    ("drama:3.1.1d", &[
        ("D4jbLgu1pjU", "Objectives and obstacles: Acting techniques | Drama - Acting Around Words", "BBC Bitesize for Teachers"),
        ("EU0sDMDCnSI", "You NEED Subtext in your Acting", "Dan Tracy"),
        ("XFAnBChBh00", "Dramatic Elements - Dramatic Tension", "Carissa Shale"),
    ]),
    ("drama:3.1.1e", &[
        ("rB8Mt5c-guM", "AQA GCSE Drama Written Exam: Social Context", "DeeperRootsProject"),
        ("4SqZdMecH0o", "Elizabethan Theatre Explained | Shakespeare’s Stage & The Globe in Under 3 Minutes", "Theatre Bites"),
    ]),
    ("drama:3.1.1f", &[
        ("O5WxspCGbIg", "Vocal and Physical Acting Skills", "missdrurydrama"),
        ("iKPHJndmjhQ", "How to use your voice | Key vocabulary | GCSE Drama Component 3 | Edexcel 9-1", "Anna Starbuck-Ahmed"),
        ("Xln9A65G1Ag", "Applying Physical Skills to Drama Characters", "Open eLMS"),
    ]),
    ("drama:3.1.1g", &[
        ("YqA0F9IRFJc", "Drama Skills - Levels and Proxemics", "The Drama Coach - Lisa Southam"),
        ("RsCSDVjvWF0", "Daily Drama Briefing 1: Levels, Positioning, Use of Space.", "Remotely Funny Drama"),
    ]),
    ("drama:3.1.1h", &[
        ("nuwGZOIdGjk", "GCSE Set Design: The Purpose of Set", "Pog Jam"),
        ("CUtiZ7vCchc", "GCSE SET DESIGN TECHNIQUES", "Pog Jam"),
        ("ItPwLajPYoc", "Set Design Terminology", "The Drama Classroom"),
    ]),
    ("drama:3.1.1i", &[
        ("aLROmAKAUyo", "Costume, Wigs and Make-up | National Theatre", "National Theatre"),
        ("1MymZm4l8WU", "COSTUME DESIGN with Mr Turner - a tutorial in costume design (Drama at KS3 & KS4)", "FLHS DRAMA"),
        ("ZXFK9aKnqIs", "How To Make a Puppet in Theatre | With Handspring Theatre Company | National Theatre", "National Theatre"),
    ]),
    ("drama:3.1.1j", &[
        ("dTOSNle7umc", "GCSE DRAMA - Lighting: The Purpose of Lighting", "Pog Jam"),
        ("MGtX9P8gDI8", "Designing Sound for Theatre | National Theatre", "National Theatre"),
        ("rOLem6kMDGo", "DRAMA GCSE - Sound Design Revision Video Part 1", "Drama FBS"),
    ]),
    ("drama:3.1.2a", &[
        ("ikKG3j1wjKU", "AQA GCSE Drama: Section B (4-mark The Crucible)", "George Coles"),
        ("JSkx0FyVnCg", "AQA GCSE Drama: Section B (4-mark TIKTBT)", "George Coles"),
        ("KFE74mps9vw", "4 mark essay question  - Costume design", "Glenthorne Drama"),
    ]),
    ("drama:3.1.2b", &[
        ("8rrF4N2Z4cg", "AQA GCSE Drama: Section B (8-mark The Crucible)", "George Coles"),
        ("o6NqG_qKiRg", "AQA GCSE Drama: Section B (8-mark TIKTBT)", "George Coles"),
    ]),
    ("drama:3.1.2c", &[
        ("vrzIT56cugs", "AQA GCSE Drama: Section B (12-mark The Crucible)", "George Coles"),
        ("2cS4O4kx2sM", "AQA GCSE Drama: Section B (12-mark TIKTBT)", "George Coles"),
    ]),
    ("drama:3.1.2d", &[
        ("bayEbEbZmnU", "AQA GCSE Drama: Section B (20-mark TIKTBT)", "George Coles"),
        ("4TQwZhXsf2g", "AQA GCSE Drama - 20 Mark Question", "Miss Bell Drama"),
    ]),
    ("drama:3.1.2e", &[
        ("cixS7dbsD7g", "GCSE Drama Set Design - Correcting Common Mistakes and a quick look at Mrs Lyons' house.", "Pog Jam"),
        ("RWP9h7s3WTg", "Blood Brothers Lighting Questions 2 (Drama)", "Pog Jam"),
        ("SP7uaCgqUos", "This WORKS for Almost Any Design Question // Edquas WJEC Drama GCSE Exam", "Drama Dan"),
    ]),
    ("drama:3.1.2f", &[
        ("VqnS-HhY_YY", "Context of The Crucible - Arthur Miller", "Schooling Online"),
        ("Sa7l0601i_E", "Plot Summary of The Crucible by Arthur Miller in Under 10 Minutes", "Schooling Online"),
        ("tvNxesAUl1E", "How We Made It | The Olivier Rains for The Crucible | National Theatre at home", "National Theatre"),
    ]),
    ("drama:3.1.2g", &[
        ("G7aQryhoKxM", "Blood Brothers Summary (Animated) || 7 Minute Summary", "Easy as GCSE"),
        ("Ru5u96cJA1I", "Blood Brothers - Context", "Drama Talk with Mr Warner"),
        ("DDDz0BZwtw0", "GCSE Drama Blood Brothers - Narrator", "Pog Jam"),
        ("1thwA261Ee4", "Physical and Vocal Skills - Blood Brothers", "Drama Department"),
    ]),
    ("drama:3.1.2h", &[
        ("7rfGAyLkNy4", "Noughts and Crosses Play Synopsis | Drama/English", "StudyWithLndz"),
        ("ArDlbPrZ0CE", "GCSE Drama Revision - Section B- Noughts and Crosses", "Bellerive Drama"),
        ("5btXTSDzOWw", "AQA GCSE Drama  Component 1 - Section B Noughts and Crosses 9.1", "Miss G Drama"),
    ]),
    ("drama:3.1.2i", &[
        ("QB8wm1ip_is", "Around the World in 80 Days Video Summary", "GradeSaver"),
        ("Lbr8T-TBeuE", "AROUND THE WORLD IN EIGHTY DAYS : The Illustrated Summary of Jules Verne's Epic Race Against Time", "Storytime Illustrated"),
    ]),
    ("drama:3.1.2j", &[
        ("VkQXE1CB2n4", "Things I Know To Be True | Introduction for Students", "Apex Drama Tools"),
        ("9EHJ_BwcetY", "Things I Know To Be True: Digital Theatre + Show Teaser", "franticassembly"),
        ("OLi9IiCvb8o", "Things I Know To Be True: Geordie Brookman & Scott Graham", "franticassembly"),
    ]),
    ("drama:3.1.2k", &[
        ("9flK30EKIi4", "Romeo and Juliet Context Lesson - Shakespeare Today", "Schooling Online"),
        ("Ebop9PQS_ms", "Romeo And Juliet Summary || Shakespeare in 7 Minutes", "Easy as GCSE"),
        ("y6OjbkAbB9o", "Staging Romeo and Juliet | English Literature - Romeo and Juliet: Shakespeare Unlocked", "BBC Bitesize for Teachers"),
    ]),
    ("drama:3.1.2l", &[
        ("T8opucP3PRo", "A Taste of Honey - Shelagh Delaney and Joan Littlewood", "National Theatre"),
        ("idPajMkQBUM", "A Taste of Honey by Shelagh Delaney - plot summary and main themes in 5 minutes", "Story Summaries"),
        ("uPkyUANfK48", "A TASTE OF HONEY by SHELAGH DELANEY Explained | Kitchen Sink Realism | Summary | Analysis | Symbols", "TheCursedCulture"),
    ]),
    ("drama:3.1.2m", &[
        ("Vgj-kJWzF6E", "The Great Wave | National Theatre | Interview with Indhu Rubasingham and Francis Turnly", "WhatsOnStage"),
        ("IPDbalGesI0", "How We Made It | Using Video Projections in The Great Wave | National Theatre", "National Theatre"),
        ("JmsC2k3WyIU", "The Great Wave (National Theatre Collection 3 on Drama Online) | Clip", "Drama Online"),
    ]),
    ("drama:3.1.2n", &[
        ("m9YDcpWKDb4", "The Empress Context and Content", "RSC Learning"),
        ("xtOrWfAssr4", "The Empress Design", "RSC Learning"),
        ("4kqnr3dU4vA", "The Empress Movement", "RSC Learning"),
    ]),
    ("drama:3.1.3a", &[
        ("_YQkSHqfTd0", "AQA GCSE Drama: Section C (Live theatre review)", "George Coles"),
        ("PZZinJhmJ2c", "Theatre review?! | Exam series | GCSE Drama Component 3 | Edexcel 9-1", "Anna Starbuck-Ahmed"),
        ("AC_u6gjjLvg", "The Curious Incident of the Dog in the Night-Time: Design Challenge", "National Theatre"),
    ]),
    ("drama:3.1.3b", &[
        ("7b6ISSpTY-Y", "How do I analyse and evaluate? | GCSE Drama Component 3 | Edexcel 9-1", "Anna Starbuck-Ahmed"),
        ("10oC4lrwPrM", "2 GCSE Drama Live Production Characterisation/Acting Question", "Joni McAuliffe"),
        ("z7J6mnocp0s", "Ensemble acting | English Literature – The Curious Incident of the Dog in the Night-time", "BBC Bitesize for Teachers"),
    ]),
    ("drama:3.1.3c", &[
        ("63FKY3Ixd7I", "3  GCSE Drama Live Production Revision - Design Question", "Joni McAuliffe"),
        ("ZCZwSGApj6E", "Design Elements  | English Literature – The Curious Incident of the Dog in the Night-time", "BBC Bitesize for Teachers"),
        ("XTt8Bd5gN9c", "5 GCSE Drama Live Production Revision Write This Way", "Joni McAuliffe"),
    ]),
// AQA GCSE Physical Education 8582. Channel constants - add beside the others at the top of videos.rs:

// Entries - paste into VIDEOS after the last subject:
    // ---------- Physical Education (AQA 8582) - The EverLearner, The PE Classroom, Mr Matthews and PE in 10 (all AQA-labelled), with Planet PE, PE TUTOR, simplype and others where they fill a gap. Titles and channels exactly as YouTube oEmbed returned them on 29 September 2026 ----------
    ("pe:3.1.1.1a", &[
        ("ZnIfjLnxwqs", "AQA GCSE PE: Bones Of The Human Body | The Skeletal System | The Skeleton | Anatomy | Paper 1", MRMATT),
        ("j1QsLy8myZI", "AQA GCSE PE - Functions of the Skeleton", EVERLEARNER),
        ("qVDaYnMHgkU", "AQA GCSE PE Synovial Joints", PEC),
        ("DlwxIipAwJk", "AQA GCSE PE - Types of Freely Movable Joints", EVERLEARNER),
        ("-uZqPl16CgY", "AQA GCSE PE: Joint Movements", EVERLEARNER),
    ]),
    ("pe:3.1.1.1b", &[
        ("LSVKIj9xulY", "AQA GCSE PE: Muscle groups", EVERLEARNER),
        ("lZ5td1-TM4E", "AQA GCSE PE: Antagonistic pairs", EVERLEARNER),
        ("x1rBfg1vrAc", "AQA GCSE PE Revision - Types of Contractions", EVERLEARNER),
    ]),
    ("pe:3.1.1.2a", &[
        ("AkA2w9gH7QI", "AQA GCSE PE: The Pathway Of Air & Gaseous Exchange | The Lungs & Alveoli | The Respiratory System", MRMATT),
        ("RpZh3Edvdzw", "AQA GCSE PE - The Lungs & Gas Exchange", PEC),
        ("DsyxRR4C8Mk", "AQA GCSE PE: Structure and function of blood vessels", EVERLEARNER),
    ]),
    ("pe:3.1.1.2b", &[
        ("7xxRQJsuc5s", "AQA GCSE PE: Structure of the Heart", EVERLEARNER),
        ("tTSL_Bwjib0", "AQA GCSE PE: Cardiac Cycle", EVERLEARNER),
        ("CG0k1hw0e0k", "Cardiac output, stroke volume and heart rate- GCSE PE Paper 1", PLANETPE),
    ]),
    ("pe:3.1.1.2c", &[
        ("BACMHCejqhw", "AQA GCSE PE: Mechanics of Breathing", EVERLEARNER),
        ("-TnU6Kkn7SQ", "AQA GCSE PE: Spirometer Trace & Lung Volumes | Tidal Volume, Reserve Volumes & Residual Volume | AQA", MRMATT),
        ("VOpq_p9t-Qw", "AQA GCSE PE Revision - Interpreting a Spiromter Trace", EVERLEARNER),
    ]),
    ("pe:3.1.1.3", &[
        ("Jc73f_jxjWo", "AQA GCSE PE: Aerobic and anaerobic energy", EVERLEARNER),
        ("ez0gFmFoWvU", "AQA GCSE PE EPOC", PEC),
        ("Uuer25qlSI4", "GCSE PE- Recovery Methods (cool down, Diet, Ice Baths)", PLANETPE),
    ]),
    ("pe:3.1.1.4", &[
        ("zXQyjdTPk08", "AQA GCSE PE - Short-Term Effects of Exercise", PEC),
        ("5UeHb9zuvos", "AQA GCSE PE - Long-Term Effects of Exercise", PEC),
        ("PDhXGyvPQew", "Long Term Effects of Exercise AQA GCSE PE", EVERLEARNER),
    ]),
    ("pe:3.1.2.1", &[
        ("DEnuUfI3Ow0", "AQA GCSE PE: Levers", EVERLEARNER),
        ("3o4XMyAg2gQ", "AQA GCSE PE: First, Second and Third Class Lever Systems & Mechanical Advantage | Movement Analysis", MRMATT),
        ("MJtcEMT7G2c", "AQA GCSE PE: Movement Analysis", EVERLEARNER),
    ]),
    ("pe:3.1.2.2", &[
        ("rXWAd3VFThk", "AQA GCSE PE Planes & Axes", PEC),
        ("yu-U4AJticU", "Planes and Axes of Movement in Sport - GCSE PE", "The PE Tutor"),
    ]),
    ("pe:3.1.3.1", &[
        ("GiezIBTfl68", "AQA GCSE PE   Health and Fitness", EVERLEARNER),
        ("xq_ZQE13LvI", "GCSE PE- Health and Fitness", PLANETPE),
    ]),
    ("pe:3.1.3.2a", &[
        ("4fRMWdYHmvM", "AQA GCSE PE - The Components of Fitness", PEC),
        ("3ElyE8j03Sc", "AQA GCSE PE Revision - Components of Fitness", EVERLEARNER),
    ]),
    ("pe:3.1.3.2b", &[
        ("gXzhVvylYYs", "AQA Fitness Tests 9 Marker", PEC),
        ("YQf5U-bzcbs", "Fitness Tests GCSE PE", "simplype"),
        ("Zc0EqncnHQg", "Fitness Testing Limitations - GCSE PE", "The PE Tutor"),
    ]),
    ("pe:3.1.3.3a", &[
        ("U8b9x8tccCA", "Principles of Training, GCSE PE AQA, Paper 1", PEIN10),
        ("fQloluDDngc", "GCSE PE- Principles of Training using Dual Coding from @pegeekscorner", PLANETPE),
    ]),
    ("pe:3.1.3.3b", &[
        ("qNSh5TaXu9I", "AQA Types of Training 6 Marks", PEC),
        ("bHRQ09Y3pDw", "Circuit Training- GCSE PE AQA, Paper 1", PEIN10),
        ("GGrSLS81aYc", "Fartlek Training- GCSE PE AQA, Paper 1", PEIN10),
        ("CasEg9GR6ng", "Interval Training- GCSE PE AQA, Paper 1", PEIN10),
        ("fF7KK81hNN8", "Plyometric Training- GCSE PE AQA, Paper 1", PEIN10),
    ]),
    ("pe:3.1.3.4a", &[
        ("NpuXPgj7CGY", "AQA GCSE PE - Training Intensity", EVERLEARNER),
        ("DmjegRwCiXE", "AQA GCSE PE Revision Course: Training Zones & Thresholds Explained", "PE TUTOR"),
    ]),
    ("pe:3.1.3.4b", &[
        ("wLrC5PLRN1g", "AQA GCSE PE: Altitude training", EVERLEARNER),
        ("bchOAhUzskg", "GCSE PE- TRAINING SEASONS", PLANETPE),
        ("8w3eE7IFgJk", "Seasonal Aspects of Training GCSE PE", "simplype"),
    ]),
    ("pe:3.1.3.5", &[
        ("9FcW-KFy8Mk", "Warm-Ups and Cool-Downs, GCSE PE AQA- Paper 1", PEIN10),
        ("9nBeRK5Gj0o", "AQA GCSE PE: The Benefits Of Warming Up & Cooling Down Before & After Exercise | Injury Prevention", MRMATT),
    ]),
    ("pe:3.1.4.1", &[
        ("V9cchIesym8", "AQA GCSE PE - Quantitative & Qualitative Data", "High Tunstall PE"),
        ("JBM88l_lghA", "Use of Data GCSE PE", "simplype"),
    ]),
    ("pe:3.1.4.2", &[
        ("JBM88l_lghA", "Use of Data GCSE PE", "simplype"),
        ("TvDvTT4WC4A", "PE: How is Data Collected, Presented and Evaluated", "Access GCSEPod"),
    ]),
    ("pe:3.1.4.3", &[
        ("TvDvTT4WC4A", "PE: How is Data Collected, Presented and Evaluated", "Access GCSEPod"),
        ("JBM88l_lghA", "Use of Data GCSE PE", "simplype"),
    ]),
    ("pe:3.2.1.1", &[
        ("uH0Mvx2Tdok", "Skill Classification AQA GCSE PE", PEC),
        ("e6jtvH6DDlo", "AQA GCSE PE: Skill Classification | Basic, Complex, Open & Closed Skills | Paper 2", MRMATT),
    ]),
    ("pe:3.2.1.2", &[
        ("XYjhdDeFC9I", "Goal Setting AQA GCSE PE", PEC),
        ("SGuuHGNfCLk", "AQA GCSE PE - Smart Targets", EVERLEARNER),
    ]),
    ("pe:3.2.1.3", &[
        ("6Tzwij2banA", "AQA GCSE PE: The Basic Information Processing Model | AQA Paper 2", MRMATT),
        ("c8EPHoU6JtM", "Information Processing - AQA GCSE PE", PEC),
    ]),
    ("pe:3.2.1.4", &[
        ("84Kyb5F2-AY", "AQA GCSE PE: Guidance", EVERLEARNER),
        ("3KtsGP_1iZU", "Guidance AQA GCSE PE", PEC),
        ("RKnU5-YBr0k", "Feedback AQA GCSE PE", PEC),
        ("T2c9TmrBbAU", "Types of Feedback in Sport- GCSE PE AQA- Paper 2", PEIN10),
    ]),
    ("pe:3.2.1.5a", &[
        ("Q2vwBuR3Vwo", "GCSE PE  Paper 2- arousal inverted u Theory and How To Control It", PLANETPE),
        ("4DkIOrHGf5g", "AQA GCSE PE Revision Course: Inverted U Theory", "PE TUTOR"),
        ("3sryr6W73RU", "AQA GCSE PE Arousal & Motivation", PEC),
    ]),
    ("pe:3.2.1.5b", &[
        ("jDrdd9GH0Fs", "AQA GCSE PE - Aggression & Personality", PEC),
        ("Z-9e9eniWQQ", "AQA GCSE PE: Motivation", EVERLEARNER),
        ("3sryr6W73RU", "AQA GCSE PE Arousal & Motivation", PEC),
    ]),
    ("pe:3.2.2.1", &[
        ("7cAyWfEA5C4", "AQA GCSE PE: Engagement Patterns of Different Social Groups | Factors Affecting Sports Participation", MRMATT),
        ("5D19l6OeMoc", "Engagement Patterns AQA GCSE PE", PEC),
    ]),
    ("pe:3.2.2.2", &[
        ("oBrEsBlOxpk", "AQA GCSE PE: Commercialisation of Sport | Sport, Sponsorship & The Media | The Golden Triangle", MRMATT),
        ("Pkmof4Nbev4", "AQA GCSE PE: Positive and Negative Impact of Sponsorship and Media", EVERLEARNER),
        ("jkNKvLfCYoI", "AQA GCSE PE: Positive and Negative Impact of Technology", EVERLEARNER),
    ]),
    ("pe:3.2.2.3a", &[
        ("W_MRRZYiDtY", "AQA GCSE PE: Conduct of Performers", EVERLEARNER),
        ("pDQX_6d4Pr8", "AQA GCSE PE - Performance Enhancing Drugs", PEC),
        ("ni-DoxNsEzw", "AQA GCSE PE: Positive and Negative Impact of PEDs", EVERLEARNER),
        ("5jSIbrNDNl4", "What is blood doping? GCSE PE paper 2", PLANETPE),
    ]),
    ("pe:3.2.2.3b", &[
        ("4-cjlVhO2B8", "AQA GCSE PE: Hooliganism, Spectator Behaviour & Strategies Used To Combat Hooliganism in Sport", MRMATT),
        ("12bux6WBkC4", "AQA GCSE PE Spectator Behaviour", PEC),
    ]),
    ("pe:3.2.3.1", &[
        ("mCHFq1kMsjk", "AQA GCSE PE: Health and Wellbeing", EVERLEARNER),
        ("1ladU-3m-Kk", "AQA Health, Fitness & Well-Being", PEC),
    ]),
    ("pe:3.2.3.2", &[
        ("btlthLwJOzk", "AQA GCSE PE Sedentary Lifestyle/Somatotypes", PEC),
        ("2nhbw3ipm04", "Sedentary Lifestyle", PEC),
    ]),
    ("pe:3.2.3.3", &[
        ("y-MfbxuF4hI", "AQA GCSE PE Diet & Nutrition", PEC),
        ("A9_-oMb4sec", "AQA GCSE PE - Reasons for a Balanced Diet", EVERLEARNER),
        ("BrPLUNc9zE4", "AQA GCSE PE - Hydration", EVERLEARNER),
    ]),
    ("media:2a", &[
        ("MQmBCqT8DSE", "Key Concepts - Media Language", "GCSE Media Revision"),
        ("bow0Y9QUlBU", "Media Studies - Roland Barthes' Semiotic Theory - Simple Guide for Students And Teachers", "Mrs Fisher"),
        ("_vINP4yXsFI", "Media Studies - Propp's Character Theory - Simple Guide For Students & Teachers", "Mrs Fisher"),
        ("hNaDStRuPdI", "Media Studies - Steve Neale's Genre Theory - Simple Guide for Students & Teachers", "Mrs Fisher"),
    ]),
    ("media:2b", &[
        ("yJr0gO_-w_Q", "Stuart Hall's Representation Theory Explained! Media Studies revision", "The Media Insider"),
        ("HxK5CXfKSCI", "Media Studies - Stuart Hall's Representation Theory - Simple Guide For Students & Teachers", "Mrs Fisher"),
        ("LeXzLUpw8mg", "Media Studies - Laura Mulvey’s Male Gaze / Feminist theory - Simple Guide", "Mrs Fisher"),
        ("pyF2XVhWe0E", "Media Studies - Alvarado’s Theory Of Ethnicity & Racial Stereotypes - A Simple Guide", "Mrs Fisher"),
    ]),
    ("media:2c", &[
        ("iYipVkF3pMI", "Media Studies - Ownership", "Mrs Fisher"),
        ("T4_Qjm0yho8", "Media Studies - Vertical Integration - Key Words", "Mrs Fisher"),
        ("PsPQoQSPI1c", "An Overview of Media Regulation in the UK", "Coombe Media & Film Studies"),
        ("7lAZFkEUFKc", "Media Studies Concepts - Ofcom and U.K. Broadcast TV Regulation", "Mrs Fisher"),
    ]),
    ("media:2d", &[
        ("koYBPkgXrBU", "Target Audience Explained: Demographics vs Psychographics | GCSE Media Studies | Eduqas", "TheMediaShepherd"),
        ("_1pBBnnWbDQ", "Media Studies - Uses & Gratifications Theory - Simple Guide", "Mrs Fisher"),
        ("FcJEkjn7sJY", "Active and Passive Audience", "GCSE Media Revision"),
        ("U7RO60SkDbw", "Media Studies - Stuart Hall's Reception Theory - Simple Guide For Students & Teachers", "Mrs Fisher"),
        ("tTRk3Y6BnqA", "Media Studies - Gauntlett's Identity Theory - Simple Guide for Students and Teachers", "Mrs Fisher"),
    ]),
    ("media:2e", &[
        ("Fx8qLcYigAw", "Quality Street - Context", "GCSE Media Revision"),
        ("L0_i-tva77M", "GCSE Media Studies Context The Sun Newspaper [Eduqas]", "Dr G Khan"),
        ("YueiuY6NIZw", "0.1.7 - Contexts:  Political Contexts", "MrMediaStudies"),
    ]),
    ("media:2.1a", &[
        ("kntvnqBQktQ", "GCSE Media - Vogue Cover (July 2021) - Media Language & Representation", "Mrs Fisher"),
        ("5OzOZR72M3E", "GCSE Media - GQ (august 19 issue) - Media Language & Representation", "Mrs Fisher"),
        ("phvRY_J7yxw", "Deconstructing GQ GCSE Media", "Mr Dolman"),
    ]),
    ("media:2.1b", &[
        ("lRnTaDEWq3s", "GCSE - The Man With The Golden Gun - Media Language & Representation", "Mrs Fisher"),
        ("Pn60surD2wQ", "GCSE Media - No Time To Die poster - Media Language & Representation", "Mrs Fisher"),
        ("Dfk4xju42mw", "The Man with the Golden Gun - Annotations", "GCSE Media Revision"),
    ]),
    ("media:2.1c", &[
        ("sfYmJUENktU", "Newspaper Conventions: Tabloid vs Broadsheets", "Coombe Media & Film Studies"),
        ("SWExl3aNWfk", "GCSE Media - Guardian - Media Language & Representation", "Mrs Fisher"),
        ("FT1Dun9i9LU", "The Sun Newspaper: Complete Exam Guide | GCSE Media Studies | Eduqas", "TheMediaShepherd"),
        ("hXXsrabastk", "COMPONENT 1 SECTION A - THE SUN FRONT COVER - MEDIA STUDIES [EDUQAS]", "Ms P Harvey"),
    ]),
    ("media:2.1d", &[
        ("0om_gIXb0ck", "GCSE Media - Quality Street Advert -- Media Language -  A Guide for Students & Teachers", "Mrs Fisher"),
        ("nKI1BOyaKp8", "GCSE Media - Quality Street Advert -- Representation -  A Guide for Students & Teachers", "Mrs Fisher"),
        ("ngcAqFDPT1U", "GCSE Media  Deconstructing the NHS 111 Advert", "Mr Dolman"),
    ]),
    ("media:2.1e", &[
        ("1GmbuWAdLsE", "GCSE Media - Component 1 Exam Paper - What to Expect", "Mrs Fisher"),
        ("uYn94pjI4BU", "GCSE 5 mark Representation Social Context question -  Vogue Magazine", "Dr G Khan"),
        ("dz0R3AQL3dg", "GCSE Media Studies Component 1: Tips, Answers & Examples", "The Media Insider"),
    ]),
    ("media:2.1f", &[
        ("2DmuTskxGgU", "GCSE Media - The Sun - Industries", "Mrs Fisher"),
        ("eB2ny3yb-VA", "GCSE Media - The Sun - Audiences", "Mrs Fisher"),
        ("uUODfEkWR9Q", "The Sun Newspaper: Following the Money - Business Model Explained | GCSE Media Studies | Eduqas", "TheMediaShepherd"),
        ("t5MEEmVub_k", "The Sun Newspaper: Power, Politics & Influence | GCSE Media Studies | Eduqas", "TheMediaShepherd"),
    ]),
    ("media:2.1g", &[
        ("qwmVmhgR8pA", "Desert Island Discs  The Original Podcast GCSE Media", "Mr Dolman"),
        ("XKr8cZL-CLk", "The BBC Charter", "GCSE Media Revision"),
        ("xTxPETlbyPc", "Radio Industries & BBC Radio 1 Live Lounge", "Coombe Media & Film Studies"),
    ]),
    ("media:2.1h", &[
        ("6UimhBLR8_g", "GCSE Media - James Bond website - Industry", "Mrs Fisher"),
        ("Kr9kLouS0OM", "What even is Vertical Integration?", "Coombe Media & Film Studies"),
    ]),
    ("media:2.1i", &[
        ("P0ou3Sh-TFM", "GCSE Media - Fortnite - Industries & Audiences", "Mrs Fisher"),
        ("7F1W7-PlH9s", "GCSE Media - Fortnite Website - Industry & Audience", "Mrs Fisher"),
        ("70xcmxwlVKw", "Fortnite's Billion Dollar Secret: Games as a Service Explained | GCSE Media Studies | Eduqas", "TheMediaShepherd"),
    ]),
    ("media:2.2a", &[
        ("WC8hKs8AN9o", "GCSE Media - Component 2 - What To Expect", "Mrs Fisher"),
        ("nn53_SZnQpA", "Sitcom Genre", "GCSE Media Revision"),
        ("G93hiTpW3Tw", "Crime Drama - Comparisons", "GCSE Media Revision"),
        ("OrDwS1TSzE8", "GCSE Media - TV Sitcoms - Industry - Simple Guide for Students & Teachers", "Mrs Fisher"),
        ("UtDb_0nrA9w", "Media Studies Concepts - Regulation of Streaming Sites in the U.K.", "Mrs Fisher"),
    ]),
    ("media:2.2b", &[
        ("DMfIefdVq_g", "Trigger Point Series 2 | First Look | ITV", "ITV"),
        ("_Ks1qiTKp04", "The Making of “Trigger Point”", "TV Horizon"),
    ]),
    ("media:2.2c", &[
        ("tsymfcvb3Mw", "GCSE Media - The Sweeney - Industry", "Mrs Fisher"),
        ("rkwwykpGgb4", "The Sweeney - Key Concepts and Context", "GCSE Media Revision"),
    ]),
    ("media:2.2d", &[
        ("XVRykVTyLNw", "GCSE Media - Man Like Mobeen - Audience", "Mrs Fisher"),
        ("3aYqKZ9Z1c0", "GCSE Media - Man Like Mobeen - Industry", "Mrs Fisher"),
        ("AUpDW2lSucM", "Revising Television - Man Like Mobeen", "Octo Beard"),
    ]),
    ("media:2.2e", &[
        ("dG7yHMFUNls", "GCSE Media - Modern Family - Media Language", "Mrs Fisher"),
        ("DcXVkPOMBBg", "GCSE Media - Modern Family - Representation", "Mrs Fisher"),
        ("-PH-U1SNYgo", "GCSE Eduqas Media Studies - Sitcom Revision (Modern Family and Friends)", "Edward"),
    ]),
    ("media:2.2f", &[
        ("N7I_fAAPImc", "Friends - Set Episode", "GCSE Media Revision"),
        ("8f_JH84_Mrs", "GCSE Media - TV Sitcoms - Audiences", "Mrs Fisher"),
    ]),
    ("media:2.2g", &[
        ("Pel14H2QIDA", "Music Video - Questions", "GCSE Media Revision"),
        ("78VQeJw1lNE", "GCSE Eduqas Media Studies Revision - Music Videos (The Man and Intentions)", "Edward"),
    ]),
    ("media:2.2h", &[
        ("v2V03Asuh6Y", "Taylor Swift - Website Annotations", "GCSE Media Revision"),
        ("Eo3_q9KFY0U", "Media Studies - Henry Jenkins Fandom theory - A simple guide for students  teachers", "Mrs Fisher"),
        ("u8rFe2Z60Hg", "Media Studies - Clay Shirky's End Of Audience Theory - Simple Guide For Students & Teachers", "Mrs Fisher"),
    ]),
    ("media:2.2i", &[
        ("mexblZDOHw4", "GCSE Media - Good As Hell by Lizzo", "Mrs Fisher"),
        ("vuq-VAiW9kw", "Lizzo - Good As Hell (Official Music Video)", "Lizzo Music"),
    ]),
    ("media:2.2j", &[
        ("I7HgJJR-yZc", "GCSE Media - The Man by Taylor Swift", "Mrs Fisher"),
        ("AqAJLh9wuZ0", "Taylor Swift - The Man (Official Video)", "Taylor Swift"),
        ("aXbrwoIVkLU", "GCSE Media - Taylor Swift - Online Media", "Mrs Fisher"),
    ]),
    ("media:2.2k", &[
        ("dF9b8DXTFtI", "GCSE Media - Superheroes by Stormzy", "Mrs Fisher"),
        ("q-EW4-B11hw", "STORMZY - SUPERHEROES", "Stormzy"),
    ]),
    ("media:2.2l", &[
        ("KlLlJHDcyhQ", "GCSE Media - Intentions by Justin Bieber - Media Language, Representation & Audience", "Mrs Fisher"),
        ("3AyMjyHu1bA", "Justin Bieber - Intentions (Official Video (Short Version)) ft. Quavo", "JustinBieberVEVO"),
    ]),
    ("media:2.2m", &[
        ("ACONooRUGyc", "GCSE Media - Rio by Duran Duran - Media Language & Representation", "Mrs Fisher"),
        ("nTizYn3-QN0", "Duran Duran - Rio (Official Music Video)", "DuranDuranVEVO"),
        ("Sf2wVJlqn1A", "Rio - Case Study", "GCSE Media Revision"),
    ]),
    ("media:2.2n", &[
        ("r40hohDAmvo", "GCSE Media - TLC Waterfalls", "Mrs Fisher"),
        ("8WEtxJ4-sh4", "TLC - Waterfalls (Official HD Video)", "TLCVEVO"),
        ("0rWZLuciS6U", "TLC Waterfalls - Case Study", "GCSE Media Revision"),
    ]),
    // ---------- Design and Technology (AQA GCSE 8552) - Collins Revision's AQA series, DTtoolbox, Tech Revision with Mrs Swanepoel, MR Ridley and others ----------
    ("dt:3.1.1a", &[
        ("GTgS6ozqsRk", "Types of Manufacturing, Automation and Robotics (JIT, FMS) - GCSE DT", "DTtoolbox"),
        ("dlyw7C0HqHo", "Impact on Industry - AQA GCSE Design & Technology", "Collins Revision"),
        ("AgTZa0Nd9F8", "Impact on Production - AQA GCSE Design & Technology", "Collins Revision"),
        ("47Iz5mJ4t2E", "Enterprise, technology and the impact on people GCSE DT", "DTtoolbox"),
    ]),
    ("dt:3.1.1b", &[
        ("-urTbV6Ep9M", "Impact on Society and the Environment - AQA GCSE Design & Technology", "Collins Revision"),
        ("w2vEEtwT5_o", "Video 5 - Technology Push and Market Pull", "Tech Revision with Mrs Swanepoel"),
        ("0-B8BEqbuWs", "Planned Obsolescence", "DT Mr C"),
    ]),
    ("dt:3.1.2", &[
        ("thy3_w74fR0", "Energy generation and storage GCSE DT - The pros and cons of renewables and non-renewables", "DTtoolbox"),
        ("Yh_zChJ8CiE", "Energy Generation and Storage - AQA GCSE Design & Storage", "Collins Revision"),
        ("vUUOicGhmYs", "Mr Ridley's Quick Revision Power Generation  for GCSE D&T", "MR Ridley Design & Technology"),
    ]),
    ("dt:3.1.3", &[
        ("tZsciepY4RY", "New Materials - AQA GCSE Design & Technology", "Collins Revision"),
        ("ggQjmN3N92w", "Modern and Smart Materials GCSE DT", "DTtoolbox"),
        ("cWI_iYGnufU", "GCSE D&T Question walkthrough Composite Materials", "MR Ridley Design & Technology"),
    ]),
    ("dt:3.1.4", &[
        ("jEzH0CxEsKw", "Electronic Systems - AQA GCSE Design & Technology", "Collins Revision"),
        ("fMFC1SaHeHk", "Electronic systems GCSE DT", "DTtoolbox"),
        ("gQ1d08MjIZU", "GCSE - Electronics - Inputs and Outputs", "Tech Revision with Mrs Swanepoel"),
    ]),
    ("dt:3.1.5", &[
        ("H3Mve7ZVG8A", "Mechanisms in life and industry - cams, linkages, pulleys, gears GCSE DT", "DTtoolbox"),
        ("NKIY3ghQNOc", "Mechanical Systems: Principles of Lever - AQA GCSE Design & Technology", "Collins Revision"),
        ("0yx_VaPsBKw", "Mechanical Systems: Gears - AQA GCSE Design & Technology", "Collins Revision"),
        ("iJRKBsAnbZY", "GCSE - Mechanical Systems (Core)", "Tech Revision with Mrs Swanepoel"),
    ]),
    ("dt:3.1.6.1a", &[
        ("AbWDLLS6cZk", "Materials: Paper and Board - AQA GCSE Design Technology", "Collins Revision"),
        ("GaxXM7q3U70", "Materials: Timber - AQA GCSE Design & Technology", "Collins Revision"),
        ("55z8P_HnSzQ", "Paper, card and board GCSE DT", "DTtoolbox"),
        ("ez9Wap9G_OA", "Hardwoods and Softwoods GCSE DT", "DTtoolbox"),
        ("0kjTxNPskoA", "Manufactured boards GCSE DT", "DTtoolbox"),
    ]),
    ("dt:3.1.6.1b", &[
        ("3-wBZTEHBl4", "Materials: Metals - AQA GCSE Design & Technology", "Collins Revision"),
        ("QUbfWwYV-Oc", "Materials: Polymers - AQA GCSE Design & Technology", "Collins Revision"),
        ("kpf9WNOXrc0", "Materials: Textiles - AQA GCSE Design & Technology", "Collins Revision"),
        ("TG3O6F3YxwU", "Metals GCSE DT", "DTtoolbox"),
    ]),
    ("dt:3.1.6.2", &[
        ("dG6BTfS52HE", "Material properties GCSE DT", "DTtoolbox"),
        ("donjm2xq3U8", "Properties of Materials - AQA GCSE Design & Technology", "Collins Revision"),
        ("X9cfEOJjk_Y", "Material Properties Core", "Tech Revision with Mrs Swanepoel (V2)"),
    ]),
    ("dt:3.2.1", &[
        ("q8f5ajj06AY", "Selection of Materials - AQA GCSE Design & Technology", "Collins Revision"),
        ("VL918ge7dKg", "GCSE - Selecting Materials and Components", "Tech Revision with Mrs Swanepoel"),
    ]),
    ("dt:3.2.2", &[
        ("MIMc2qxIfyo", "Forces & Stresses GCSE DT", "DTtoolbox"),
        ("oVxNHNPGFNs", "GCSE - Forces", "Tech Revision with Mrs Swanepoel"),
    ]),
    ("dt:3.2.3", &[
        ("W2jhLxSBE-g", "Ecological, Environmental and Social Issues - AQA GCSE Design & Technology", "Collins Revision"),
        ("4KXmdFfZVcs", "Sustainability in Design GCSE DT", "DTtoolbox"),
        ("AlBmyysGLtk", "Mr Ridley's Quick Revision Sustainability and the 6 R's for GCSE D&T", "MR Ridley Design & Technology"),
    ]),
    ("dt:3.2.4", &[
        ("8KfM6o8AZT8", "GCSE - Production of Polymers", "Tech Revision with Mrs Swanepoel"),
        ("5d28UhFLz0s", "D&T Home Learning The Conversion of Timber", "MR Ridley Design & Technology"),
        ("OXQDsSctP1M", "How Paper Is Made", "PaperOne"),
    ]),
    ("dt:3.2.5a", &[
        ("_YoWpYS2UGo", "Working with Materials - AQA GCSE Design & Technology", "Collins Revision"),
        ("quVcegl1L_w", "Enhanced Materials", "Mr Everett's Design and Technology Workshop"),
    ]),
    ("dt:3.2.5b", &[
        ("Fcu268fdIK0", "Manufacturing Processes 1: Process Types and Processes used with Paper and Board - AQA GCSE Design &", "Collins Revision"),
        ("kPm3ZF__f-I", "Manufacturing Processes 2: Timber Based Materials - AQA GCSE Design & Technology", "Collins Revision"),
        ("tsA1IHpyia4", "Manufacturing Processes 3: Metals and Alloys - AQA GCSE Design & Technology", "Collins Revision"),
        ("Em_NE-pF1m8", "Manufacturing Processes 4: Polymers - AQA GCSE Design & Technology", "Collins Revision"),
        ("dYao1yGgQtA", "Manufacturing Processes 5: Textiles and Electronic Systems - AQA GCSE Design & Technology", "Collins Revision"),
    ]),
    ("dt:3.2.6", &[
        ("TY7Y_vOcLLI", "Mr Ridley’s Quick Revision Stock forms of materials", "MR Ridley Design & Technology"),
        ("mNQDdtRMaLk", "Mr Ridley's Quick Revision Standard components for GCSE D&T", "MR Ridley Design & Technology"),
        ("3OEl8o63xbs", "Video 12 - Standard Components", "Tech Revision with Mrs Swanepoel"),
    ]),
    ("dt:3.2.7", &[
        ("XVW8Yt7EfJg", "Scales of Manufacture - AQA GCSE Design & Technology", "Collins Revision"),
        ("GceGdHBegBs", "Scales of Production", "KS3-5 Design & Technology"),
        ("asz34DefC8Q", "GCSE Style Question Scales of Production", "MR Ridley Design & Technology"),
    ]),
    ("dt:3.2.8a", &[
        ("AA3R4hyenM4", "Measurement and Production Aids - AQA GCSE Design & Technology", "Collins Revision"),
        ("pRQvLRMkaow", "GCSE - Production Aids (Core)", "Tech Revision with Mrs Swanepoel"),
        ("cbV8Wbvmpjg", "Design and Technology (D&T) | KS3 | Vacuum forming | BBC Teach", "BBC Bitesize for Teachers"),
    ]),
    ("dt:3.2.8b", &[
        ("nz3KfBcyqKk", "GCSE - Quality Control", "Tech Revision with Mrs Swanepoel"),
        ("XcSIwcYU-rQ", "Commercial printing processes GCSE DT", "DTtoolbox"),
        ("rph6uOD6ytI", "GCSE - Commercial processes - Polymers", "Tech Revision with Mrs Swanepoel"),
    ]),
    ("dt:3.2.9", &[
        ("uB64p3HErVo", "Finishing Materials - AQA GCSE Design & Technology", "Collins Revision"),
        ("UdowcJaiiPI", "Mr Ridley's RMT Revision 005 Metal Processes and Finishes", "Mr Ridley RMT Revision"),
        ("PjOyzDmwWEw", "Design and Technology (D&T) | KS3 | Finishing wood | BBC Teach", "BBC Bitesize for Teachers"),
    ]),
    ("dt:3.3.1", &[
        ("3sAeD_AxkcU", "Research and Investigation - AQA GCSE Design & Technology", "Collins Revision"),
        ("lBTIFauPG4M", "Briefs and Specifications - AQA GCSE Design & Technology", "Collins Revision"),
        ("QsSlV4H5cDM", "A brief guide to Anthropometrics and Ergonomics", "Mr Wolsey DT"),
    ]),
    ("dt:3.3.2", &[
        ("wb1FiCigWnY", "Sustainability", "DT Mr C"),
        ("M2vdUmYF1Q0", "Sustainability and Biopolymers Exam Question walkthrough", "MR Ridley Design & Technology"),
        ("g8LC3PJ-7r4", "What is Fairtrade?", "Fairtrade Ireland"),
    ]),
    ("dt:3.3.3", &[
        ("jtGXpIAlh0w", "The Work of Others: Designers - AQA GCSE Design & Technology", "Collins Revision"),
        ("-t5w4aoQClI", "The Work of Others: Companies - AQA GCSE Design & Technology", "Collins Revision"),
        ("r4lzmUbYX4A", "How the work of designers has shaped our world GCSE DT", "DTtoolbox"),
        ("Yy9qtk3wzIw", "Design Companies - Apple and Dyson", "Tech Revision with Mrs Swanepoel"),
        ("xUQt0Wp2eKE", "Section C - Designers - Breuer and Starck", "Tech Revision with Mrs Swanepoel"),
    ]),
    ("dt:3.3.4", &[
        ("mhQ-Cx7dP0g", "Design Strategies - AQA GCSE Design & Technology", "Collins Revision"),
        ("toqKqcMd5mY", "Mr Bailey D&T Avoiding design fixation", "Meden School"),
        ("1hFx_Zz4FUw", "Video 13 - Iterative Design", "Tech Revision with Mrs Swanepoel"),
    ]),
    ("dt:3.3.5", &[
        ("valOxAgXUZY", "Communication of Ideas: 3D Sketching - AQA GCSE Design & Technology", "Collins Revision"),
        ("f3maOKCw4UQ", "Communication of Ideas: System and Schematic Drawings - AQA GCSE Design & Technology", "Collins Revision"),
        ("qlIXUnWgDrc", "Computer-Based Tools - AQA GCSE Design & Technology", "Collins Revision"),
        ("i4zof1MIhgI", "Mr Ridley's Quick revision Communication of Ideas for GCSE D&T", "MR Ridley Design & Technology"),
        ("Kz1FqLyH9WM", "AQA GCSE Design and Technology Exam paper 2024 3rd Angle Orthographic Question ", "MrChoDT"),
    ]),
    ("dt:3.3.6", &[
        ("7NrpE2n20P8", "Prototype Development - AQA GCSE Design & Technology", "Collins Revision"),
        ("fsn-I6FnBuc", "Exploring and Developing Ideas - AQA GCSE Design & Technology", "Collins Revision"),
    ]),
    ("dt:3.3.7", &[
        ("BwOYk6hLmkQ", "GCSE - Selecting Materials Video 2", "Tech Revision with Mrs Swanepoel"),
        ("1liY9QTp80c", "GCSE D&T Exam Question walkthrough, Materials, stock forms and offshore manufacture.", "MR Ridley Design & Technology"),
    ]),
    ("dt:3.3.8", &[
        ("kzqX0oUUY_U", "Engineering Tolerances Explained", "Nathan Nagele"),
        ("wVWyY9k22Gk", "Introduction to Tolerances - Part I: What is a Tolerance?", "GD&T Basics - Engineer Essentials"),
        ("1lZrJuYWqIA", "AQA GCSE Design and Technology Exam paper 2021 Quality control and Material processing Question", "MrChoDT"),
    ]),
    ("dt:3.3.9", &[
        ("W5KH8bxrzLM", "Tessellation 🧩 and Nesting - Using materials efficiently", "DTtoolbox"),
        ("htcGe5CbjMg", "Nesting , minimising waste and cutting efficiently", "M White"),
    ]),
    ("dt:3.3.10", &[
        ("0TyUSku0asQ", "Health and Safety for DT", "CJDT - Happy DTing!"),
        ("7LBv2UWOI4Y", "Mr Ridley's RMT Revison 007 Hand Tools", "Mr Ridley RMT Revision"),
        ("yuah4GQ2n4M", "GCSE D&T exam walkthrough Pt 7 Risk assessment", "MR Ridley Design & Technology"),
    ]),
    ("dt:3.3.11", &[
        ("zlFZjw-ync8", "Soldering Basics How to use a soldering iron", "MR Ridley Design & Technology"),
        ("fEqUW_ND2Lo", "Design and Technology (D&T) | KS3 | Laminating wood | BBC Teach", "BBC Bitesize for Teachers"),
        ("CP5E8P-KSV4", "Design and Technology (D&T) | KS3 | Finishing plastic | BBC Teach", "BBC Bitesize for Teachers"),
    ]),
    // ---------- Food Preparation and Nutrition (AQA GCSE 8585) — The Food Tech Teacher, Collins Revision and others ----------
    ("food:3.2.1.1", &[
        ("apysgPLH1ow", "GCSE Food - Protein", FTT),
        ("UUgD9ERiTIs", "Protein and Fat - AQA GCSE Food Preparation", COLLINS),
    ]),
    ("food:3.2.1.2", &[
        ("7NaP_AaO3Ds", "GCSE Fats", FTT),
        ("UUgD9ERiTIs", "Protein and Fat - AQA GCSE Food Preparation", COLLINS),
    ]),
    ("food:3.2.1.3", &[
        ("pE8eTFpgQRg", "Carbohydrates (GCSE Food)", FTT),
        ("1pp7iObepQI", "Carbohydrate - AQA GCSE Food Preparation", COLLINS),
    ]),
    ("food:3.2.2.1a", &[
        ("TD1tjVDRmQA", "Fat and Water Soluble Vitamins GCSE Food", FTT),
        ("B4xW7Es_eLo", "Vitamins - AQA GCSE Food Preparation", COLLINS),
    ]),
    ("food:3.2.2.1b", &[
        ("TD1tjVDRmQA", "Fat and Water Soluble Vitamins GCSE Food", FTT),
        ("B4xW7Es_eLo", "Vitamins - AQA GCSE Food Preparation", COLLINS),
        ("wHH39VJEh9E", "The science behind vitamins and minerals | Biology  – Gastro Lab", BBCTEACH),
    ]),
    ("food:3.2.2.2", &[
        ("RdTBZPL1UE0", "Minerals in food: calcium, iron, sodium, fluoride, iodine and phosphorus. GCSE Food", FTT),
        ("RWniiQYkFpk", "Minerals and Water - AQA GCSE Food Preparation", COLLINS),
    ]),
    ("food:3.2.2.3", &[
        ("b7s2Aqj72Q8", "Hydration | Design and Technology - Food Preparation and Nutrition", BBCTEACH),
        ("RWniiQYkFpk", "Minerals and Water - AQA GCSE Food Preparation", COLLINS),
        ("gficVLrGhS0", "The Eatwell Guide - Hydration", FFL),
    ]),
    ("food:3.2.3.1a", &[
        ("kQELdUX2HP8", "Healthy Eating & the Eatwell Guide", FTT),
        ("UIQ1Hyq9HG0", "Eight guidelines for healthy eating | Design Technology - Food Preparation and Nutrition", BBCTEACH),
        ("tA3p1aXmE18", "Making Informed Choices - AQA GCSE Food Preparation", COLLINS),
    ]),
    ("food:3.2.3.1b", &[
        ("tA3p1aXmE18", "Making Informed Choices - AQA GCSE Food Preparation", COLLINS),
        ("i7Q8e9gNig8", "Nutrition and Life Stages", "Home Economics with Mrs McErlean"),
        ("SKmKPKZi_0g", "The Gluten Free Diet - Coeliac UK", "Coeliac UK"),
    ]),
    ("food:3.2.3.2", &[
        ("3sC8e0FZ3Po", "Energy Needs (GCSE Food)", FTT),
        ("zLkWhIqaETE", "Energy Needs GCSE food", FTT),
    ]),
    ("food:3.2.3.3", &[
        ("h7F-nhRosOo", "How to carry a nutritional analysis using Explore Food", FTT),
    ]),
    ("food:3.2.3.4", &[
        ("vwM0Wc_9hKE", "Diet, Nutrition and Health - AQA GCSE Food Technology", COLLINS),
        ("fiFi-d0RwKo", "Healthier cooking | Design and Technology - Food Preparation and Nutrition", BBCTEACH),
    ]),
    ("food:3.3.1.1", &[
        ("r9ZrT5vtVv0", "Heat Transfer Methods (GCSE Food)", FTT),
        ("vg5k6t6uZwE", "Conduction animation - AQA GCSE Food Preparation and Nutrition", ILLUM),
        ("p6W53kHIXKc", "Cooking of Food, Heat Transfer and Selecting Appropriate Cooking Methods - AQA GCSE Food Preparation", COLLINS),
    ]),
    ("food:3.3.1.2", &[
        ("p6W53kHIXKc", "Cooking of Food, Heat Transfer and Selecting Appropriate Cooking Methods - AQA GCSE Food Preparation", COLLINS),
        ("fiFi-d0RwKo", "Healthier cooking | Design and Technology - Food Preparation and Nutrition", BBCTEACH),
    ]),
    ("food:3.3.2.1", &[
        ("C2ipBYy5BMI", "Proteins:  Functional & Chemical Properties of Food  (GCSE)", FTT),
        ("bJ7uXScRTWw", "Coagulation film -   AQA GCSE Food Preparation and Nutrition", ILLUM),
        ("IOUUab3fq2k", "Gluten and Baking (Food Science)", FTT),
        ("hCyYQgPLP0w", "Omelette (denaturation/coagulation)", FTT),
    ]),
    ("food:3.3.2.2", &[
        ("NS6yWwiCyEg", "Carbohydrates Functions and Properties of Food (GCSE)", FTT),
        ("f93XTxmg1ME", "Gelatinisation    GCSE Food", FTT),
        ("ze8y7IXlYsc", "Caramelisation  GCSE Food", FTT),
        ("xjTIocPYt0A", "Carbohydrates - AQA GCSE Food Preparation", COLLINS),
    ]),
    ("food:3.3.2.3", &[
        ("Q7NKrlvUfBs", "Fats & Oils: Functional and Chemical Properties of Food (GCSE)", FTT),
        ("TqpBtoqQ9qM", "Fats and Oils - AQA GCSE Food Preparation", COLLINS),
        ("vc8O8vGCzXk", "Emulsions and Food Science (Mayonnaise)", FTT),
    ]),
    ("food:3.3.2.4", &[
        ("ojNA099qYhs", "Proteins and Enzymic Browning - AQA GCSE Food Preparation", COLLINS),
        ("P_1qp8GKNTY", "How to use lemon juice to  prevent browning of fruit", FTT),
    ]),
    ("food:3.3.2.5", &[
        ("r8A5msR4oGc", "Raising Agents - AQA GCSE Food Preparation", COLLINS),
        ("hzbDh5org2E", "Chemical Raising Agents (GCSE)  Baking Powder and Bicarbonate of Soda", FTT),
        ("GwA1xU1XXrQ", "Science of bread making  GCSE Food", FTT),
    ]),
    ("food:3.4.1.1", &[
        ("lLxq8kr0mzA", "Microorganisms, Enzymes and Food Spoilage - AQA GCSE Food Preparation", COLLINS),
    ]),
    ("food:3.4.1.2", &[
        ("lLxq8kr0mzA", "Microorganisms, Enzymes and Food Spoilage - AQA GCSE Food Preparation", COLLINS),
        ("P_1qp8GKNTY", "How to use lemon juice to  prevent browning of fruit", FTT),
    ]),
    ("food:3.4.1.3", &[
        ("_kNOoFVIh04", "Microorganisms in Food Production - AQA GCSE Food Preparation", COLLINS),
        ("FAXrblgNgK4", "How bacteria and moulds are used in cheese making", FTT),
        ("uNy2-PHkFH8", "Yeast and Sugar Experiment using Balloons (Fermentation)", FTT),
    ]),
    ("food:3.4.1.4", &[
        ("J8D-Mjv17YI", "Bacterial Contamination - AQA GCSE Food Preparation", COLLINS),
        ("GYlp1_7XIw4", "FSA Explains: Campylobacter", FSA),
        ("7XT8dBmJdMo", "FSA Explains: Salmonella", FSA),
        ("mOXU7Yuhsds", "FSA Explains: Listeria", FSA),
    ]),
    ("food:3.4.2.1", &[
        ("MBuHjXI_oAQ", "Buying and Storing Food - AQA GCSE Food Preparation", COLLINS),
        ("CDIpDupYPiY", "Use by vs best before dates", FSA),
        ("flxmB8NKMzE", "Food Safety | Design and Technology - Food Preparation and Nutrition", BBCTEACH),
    ]),
    ("food:3.4.2.2", &[
        ("mBKXpn21PAo", "Preparing and Cooking Food - AQA GCSE Food Preparation", COLLINS),
        ("flxmB8NKMzE", "Food Safety | Design and Technology - Food Preparation and Nutrition", BBCTEACH),
    ]),
    ("food:3.5.1.1", &[
        ("duWLlUJhTo4", "Factors Affecting Food Choice - AQA GCSE Food Preparation", COLLINS),
    ]),
    ("food:3.5.1.2a", &[
        ("I00oPUYlaDQ", "Food Choices - AQA GCSE Food Preparation", COLLINS),
        ("I3gSqWiGqrY", "K is for Kosher | A to Z of Religion and Beliefs | BBC Teach", BBCTEACH),
    ]),
    ("food:3.5.1.2b", &[
        ("I00oPUYlaDQ", "Food Choices - AQA GCSE Food Preparation", COLLINS),
        ("fHo15_MxS4g", "FSA Explains: Food hypersensitivity", FSA),
    ]),
    ("food:3.5.1.3", &[
        ("Cqw7CqDBKVk", "Food Labelling - AQA GCSE Food Preparation", COLLINS),
        ("OZOIEYQ0axo", "Food labelling | Design and Technology - Food Preparation and Nutrition", BBCTEACH),
    ]),
    ("food:3.5.2", &[
        ("sT8zpJ_OLYo", "British and International Cuisines - AQA GCSE Food Preparation", COLLINS),
    ]),
    ("food:3.5.3", &[
        ("kXPGo9Lsydc", "Sensory Evaluation - AQA GCSE Food Preparation", COLLINS),
        ("vFkiKSsYi0k", "Sensory Analysis GCSE Food", FTT),
        ("zNchJla7G0E", "Sensory perception | Design and Technology - Food Preparation and Nutrition", BBCTEACH),
    ]),
    ("food:3.6.1.1", &[
        ("AWaSsfEUnjE", "Food Provenance and Production Methods - AQA GCSE Food Preparation", COLLINS),
        ("XPCBzcb49_M", "Sustainable fishing explained", "Marine Stewardship Council - Sustainable seafood"),
    ]),
    ("food:3.6.1.2", &[
        ("CipUcBhG1G4", "Food and the Environment - AQA GCSE Food Preparation", COLLINS),
        ("hhlmrlJN9uM", "What Are Food Miles - And Why Do They Matter? | BBC The Social", "BBC Scotland"),
    ]),
    ("food:3.6.1.3", &[
        ("PewgG5ercM8", "Sustainability of Food - AQA GCSE Food Preparation", COLLINS),
        ("_ACn3e4qnaM", "GCSE Biology Revision \"Food Security\" (Triple)", "Freesciencelessons"),
    ]),
    ("food:3.6.2.1", &[
        ("WFYlLJMTRpI", "Food Production - AQA GCSE Food Preparation", COLLINS),
        ("RPPWHSSIUdI", "Food Processing - AQA GCSE Food Preparation", COLLINS),
        ("ZJw3E_Vip7Q", "Milk: Pasteurisation, homogenisation, sterilisation, UHT (GCSE Food)", FTT),
        ("RwPzRMdMHOY", "From Wheat to Bread", FFL),
    ]),
    ("food:3.6.2.2", &[
        ("sApHxtpWB5E", "Fortification and Enrichment of food (GCSE)", FTT),
        ("JU51f737Obg", "FSA Explains: Food additives", FSA),
        ("EfuIg7VtCnI", "FSA Explains: Genetically Modified Food", FSA),
    ]),
    // Maths (Pearson Edexcel GCSE Mathematics (1MA1) Higher)
    ("maths_edx:N1-3", &[
        ("if8ZsZXhQJE", "Order of Operations - Corbettmaths", CM),
        ("mED76j4Agiw", "Addition and Subtraction involving Negatives - Corbettmaths", CM),
        ("70cAYYCJBuQ", "How to use BODMAS (Order of Operations)", COG),
    ]),
    ("maths_edx:N4-5", &[
        ("oK-EFDLeEqc", "LCM HCF using Product of Primes - Corbettmaths", CM),
        ("kHLwbPwvTtw", "HCF/LCM - GCSE Maths", FIRSTCLASS),
        ("3H6ET7P902Q", "Product Rule for Counting - Corbettmaths", CM),
    ]),
    ("maths_edx:N6-7", &[
        ("ozuXy8_NZcg", "Laws of Indices - Corbettmaths", CM),
        ("qYDClSo89eQ", "Fractional indices - Corbettmaths", CM),
        ("DvNYkbafpIY", "GCSE Maths - What to do when Powers are Fractions (Powers Part 6/6) (2026/27 exams)", COG),
    ]),
    ("maths_edx:N8", &[
        ("ndU_cCbPAm4", "Surds - Corbettmaths", CM),
        ("96SwZpRvhwY", "Rationalising denominators - Corbettmaths", CM),
        ("I_Mys8RNt30", "Calculating With Surds - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_edx:N9", &[
        ("cxGyZ3Yx9ow", "Standard Form - Corbettmaths", CM),
        ("H3ewmorcYjU", "What is Standard Form (also known as Scientific Notation)? (Part 1/4) (2026/27 exams)", COG),
        ("u3FZaXs3hDE", "How to Multiply and Divide in Standard Form (Part 3/4) (2026/27 exams)", COG),
    ]),
    ("maths_edx:N10-12", &[
        ("KZbKYokJ3SQ", "Recurring decimals to fractions - Corbettmaths", CM),
        ("RCnSGUpoKbE", "How to Convert Recurring Decimals to Fractions (Proportions Part 6/6) (2026/27 exams)", COG),
        ("Iq-6CjlEUW4", "Fractions decimals percentages - Corbettmaths", CM),
    ]),
    ("maths_edx:N13-16", &[
        ("ebMrP74boHw", "Lower and Upper Bounds - Corbettmaths", CM),
        ("FQ8IFKNhphM", "Error Intervals - Corbettmaths", CM),
        ("JTQ2Wh5E2js", "How to Estimate in Maths (2026/27 exams)", COG),
    ]),
    ("maths_edx:A1-3", &[
        ("QvxWrYtzrtM", "GCSE Maths - Expressions vs Equations (2026/27 exams)", COG),
        ("28DkE4vMN6o", "Substitution into Expressions - Corbettmaths", CM),
        ("l54us4Q7nNY", "Equating Coefficients - Corbettmaths", CM),
    ]),
    ("maths_edx:A4", &[
        ("X-djBcWVizM", "Factorising quadratics 1 - Corbettmaths", CM),
        ("nfLb8MPO99U", "GCSE Maths - Factorising Quadratics - Part 2 - (When the x² Coefficient is More Than 1)", COG),
        ("YtHMjuB9f_g", "Algebraic Fractions (Operations) - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_edx:A5-6", &[
        ("8U9u_itcs7k", "Changing the Subject - Corbettmaths", CM),
        ("pd9Q-e1JvtE", "Algebraic Proof - Corbettmaths", CM),
        ("5lcefrczJlE", "GCSE Maths - Rearranging Formulas Part 2 - When The Subject Appears Twice (2026/27 exams)", COG),
    ]),
    ("maths_edx:A7", &[
        ("u1YQVzrgYDg", "Composite Functions - Corbettmaths", CM),
        ("zpF9nbjResY", "Inverse Functions - Corbettmaths", CM),
        ("ZRQJGecu1fs", "Function Notation - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_edx:A8-10", &[
        ("nUk47WSiS30", "GCSE Maths - What on Earth is y = mx + c (2026/27 exams)", COG),
        ("YtHJP1rZ3pI", "Gradient of a Line - Corbettmaths", CM),
        ("PrwhdgnLK5k", "Perpendicular graphs - Corbettmaths", CM),
    ]),
    ("maths_edx:A11-12", &[
        ("7xE5pj9-n1Q", "Finding Turning Points using Completing the Square", CM),
        ("oUcjmGThMdc", "Exponential Graphs - Corbettmaths", CM),
        ("fNH5EWWtf7k", "Trigonometric Graphs - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_edx:A13", &[
        ("eiRZATuHYg0", "Transformations of Graphs - Corbettmaths", CM),
        ("F8YGp_j7YhM", "Transforming Graphs", MGENIE),
        ("ctVr9NpSiL4", "Transformations of Graphs - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_edx:A14-15", &[
        ("1AVtslXytRA", "Area Under Graph - Corbettmaths", CM),
        ("cEp7qD6vCSM", "Gradient of a Curve - Corbettmaths", CM),
        ("UsmhVCjfzYQ", "Speed Time Graphs - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_edx:A16", &[
        ("_DOBTxLmUTM", "Equation of a Circle - Corbettmaths", CM),
        ("NHrb8N9oAUY", "Equation of a Tangent to a Circle - Corbettmaths", CM),
        ("12NqSRpfTR4", "Equation of a Circle - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_edx:A17", &[
        ("30S7WxKcPwg", "Solving Equations - Corbettmaths", CM),
        ("85ZM3ZKqRhY", "Solving equations with letters on both sides - Corbettmaths", CM),
        ("9FuR91H8EVU", "GCSE Maths - How to Solve Algebraic Equations (Part 1 of 3) (2026/27 exams)", COG),
    ]),
    ("maths_edx:A18", &[
        ("wJ_tLEwEEi8", "Solving Quadratics using Factorisation - Corbettmaths", CM),
        ("3J0ccr74LcU", "Quadratic formula - Corbettmaths", CM),
        ("abBgTO8eW-c", "Solving Quadratic Equations by Completing the Square - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_edx:A19", &[
        ("phlus4x0UqM", "Simultaneous Equations elimination - Corbettmaths", CM),
        ("ozP-vf99DK4", "Simultaneous equations (linear and non-linear) - Corbettmaths", CM),
        ("eEYreNDTvKQ", "Non-Linear Simultaneous Equations - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_edx:A20", &[
        ("eWP15jyatIo", "Iteration - Corbettmaths", CM),
        ("WHZ2IiKcqeU", "Iteration - GCSE Higher Maths", FIRSTCLASS),
        ("FPMimYoO8kU", "Change of Sign - Corbettmaths", CM),
    ]),
    ("maths_edx:A21-22", &[
        ("Lz3VkLrDmhE", "Forming equations - Corbettmaths", CM),
        ("u-YNzmlZWeg", "GCSE Maths - Solving Algebraic Inequalities with 1 Inequality Sign (Inequalities Part 2)", COG),
        ("8J_m-hMp8lY", "Quadratic Inequalities - Corbettmaths", CM),
    ]),
    ("maths_edx:A23-25", &[
        ("qnVVTBAfNu4", "The nth Term - Corbettmaths", CM),
        ("AL-joUBnEIw", "Quadratic Sequences Version 1 - Corbettmaths", CM),
        ("871OBfK5o2M", "GCSE Maths - Types of Number Sequences - Arithmetic vs Geometric (2026/27 exams)", COG),
    ]),
    ("maths_edx:R1-2", &[
        ("1az6Gjb2wtk", "Converting Metric Units for Length", CM),
        ("2PZ41oDEZ_Q", "Maps and Scales - Corbettmaths", CM),
        ("6XpBX-7cDPE", "GCSE Maths - Using Scales on Maps and Scale Diagrams (2026/27 exams)", COG),
    ]),
    ("maths_edx:R3-8", &[
        ("UcPVAh4igpI", "GCSE Maths - What are Ratios & How to Simplify Them (Part 1) (2026/27 exams)", COG),
        ("cflZnf9H5l4", "Ratio sharing the total - Corbettmaths", CM),
        ("SJhomYlGPZ8", "Given Two Ratios - Corbettmaths", CM),
    ]),
    ("maths_edx:R9", &[
        ("tUtgC7ZrsRc", "Increasing Decreasing by a Percentage - Corbettmaths", CM),
        ("Q2gRAS08fE0", "Percentage Change - Corbettmaths", CM),
        ("dIeb2ryQ1ko", "GCSE Maths - Reverse Percentages - Calculating The Cost Before The Discount (2026/27 exams)", COG),
    ]),
    ("maths_edx:R10", &[
        ("AXkCfkVrjK8", "Unitary Method - Corbettmaths", CM),
        ("z9JlSDzSy3c", "GCSE Maths - What Does Directly Proportional Mean? (2026/27 exams)", COG),
        ("01l1a21qDaM", "Proportion and Time - Corbettmaths", CM),
    ]),
    ("maths_edx:R11", &[
        ("dHVK7IeLGT8", "Speed, Distance, Time - Corbettmaths", CM),
        ("sv7zflLeduM", "Density - Corbettmaths", CM),
        ("fheOYg9TKQA", "Pressure - Corbettmaths", CM),
    ]),
    ("maths_edx:R12", &[
        ("8-4KQw8unfY", "GCSE Maths - Similar Shapes (2026/27 exams)", COG),
        ("GlD88EpEJVo", "Similar Shapes: Areas", CM),
        ("0U1T_f5xb98", "Similar Areas and Volumes - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_edx:R13", &[
        ("kcOwC7uqJNE", "Direct Proportion - Corbettmaths", CM),
        ("uZ6l-loSdRs", "Inverse Proportion - Corbettmaths", CM),
        ("pXnyxf55nJU", "Proportionality Graphs - Corbettmaths", CM),
    ]),
    ("maths_edx:R14-15", &[
        ("rGf6_1E8x7k", "Average Rate of Change - Corbettmaths", CM),
        ("YfDhGn3NLQw", "Instantaneous Rate of Change - Corbettmaths", CM),
        ("zVSq5b3PPfY", "GCSE Maths - How to Find the Gradient of a Straight Line (2026/27 exams)", COG),
    ]),
    ("maths_edx:R16", &[
        ("FBCs95Co_oU", "Compound Interest - Corbettmaths", CM),
        ("gliYGWM7wfY", "Compound Interest and Depreciation", MGENIE),
        ("CRKlXB5g1gE", "Exponential Growth and Decay", MGENIE),
    ]),
    ("maths_edx:G1-2", &[
        ("1beKcgU9ogE", "Perpendicular Bisectors - Corbettmaths", CM),
        ("BWj041al8z8", "Loci part 1 - Corbettmaths", CM),
        ("3viWgGJmkFo", "Constructions - GCSE Maths", FIRSTCLASS),
    ]),
    ("maths_edx:G3-4", &[
        ("gVo8ZrtlSp0", "Angles in Polygons - Corbettmaths", CM),
        ("I5auyoXYoX0", "GCSE Maths - Alternate, Corresponding and Allied Angles - Parallel Lines Angle Rules (2026/27 exams)", COG),
        ("mlG56WCfobI", "GCSE Maths - Types of Quadrilateral (2026/27 exams)", COG),
    ]),
    ("maths_edx:G5-6", &[
        ("IDW1ogTqox8", "Congruent Triangles - Corbettmaths", CM),
        ("aK8i7LKZd9o", "GCSE Maths - Congruent Triangle Rules (2026/27 exams)", COG),
        ("4YdhDXJWCZ8", "Geometric Proof - Corbettmaths", CM),
    ]),
    ("maths_edx:G7-8", &[
        ("rgdRlbbWQgA", "Rotations - Corbettmaths", CM),
        ("u2EgwMYwibw", "Describing Enlargements - Corbettmaths", CM),
        ("0_hclyERGAw", "Enlargements (Negative Scale Factor) - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_edx:G9-10", &[
        ("vgMSLsos7Ew", "Circle Theorems - Corbettmaths", CM),
        ("dBIlCD_JF9Q", "Circle Theorems - GCSE Higher Maths", FIRSTCLASS),
        ("CmprPWcLtlk", "Circle Theorem Proofs - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_edx:G11-15", &[
        ("BnwoipoGWJ8", "Views and Elevations", CM),
        ("pm8i-thxvCo", "GCSE Maths - What are Bearings? (2026/27 exams)", COG),
        ("8Wja7Ct_XvY", "Bearings - Corbettmaths", CM),
    ]),
    ("maths_edx:G16-18", &[
        ("Wcv0f5PpTv0", "GCSE Maths - Area of a Sector and Length of an Arc of a Circle  (Circles Part 3) (2026/27 exams)", COG),
        ("X6cMvcxk1ig", "Volume of a Cone - Corbettmaths", CM),
        ("VwdMbDpMab4", "Surface Area of Prisms - Corbettmaths", CM),
    ]),
    ("maths_edx:G19", &[
        ("L6DLoBMknoY", "Similar Shapes - Missing Sides", CM),
        ("QMI90tONvzQ", "Similar Shapes: Volumes", CM),
        ("u1uOcrjQCh4", "Similar Triangles - GCSE Maths", FIRSTCLASS),
    ]),
    ("maths_edx:G20-21", &[
        ("iWLVTy_rGjs", "Pythagoras - Corbettmaths", CM),
        ("WFH_7n7hpHo", "Trigonometry | SOH CAH TOA | Sin, Cos, Tan", COG),
        ("UjgOR07zOzY", "Exact trigonometric values - Corbettmaths", CM),
    ]),
    ("maths_edx:G22-23", &[
        ("7xeLeDulY60", "The Sine Rule - GCSE Higher Maths", FIRSTCLASS),
        ("3H3u92WJAjw", "Cosine rule - Corbettmaths", CM),
        ("eSFOMSxjMts", "Area of any Triangle - Corbettmaths", CM),
    ]),
    ("maths_edx:G24-25", &[
        ("h02d922Q5wk", "Column Vectors - Corbettmaths", CM),
        ("xOdkldbusy0", "Vectors - Corbettmaths", CM),
        ("vkwFUig3dGY", "Vectors in Shapes", MGENIE),
    ]),
    ("maths_edx:P1-5", &[
        ("ur_hHjLrBNo", "Probability - Corbettmaths", CM),
        ("MS6lnCTgTSw", "Relative Frequency - Corbettmaths", CM),
        ("QDnDtxDEHWg", "Probability and Relative Frequency", MGENIE),
    ]),
    ("maths_edx:P6-8", &[
        ("Xqno7W0OUtE", "Sample Space Diagrams - Corbettmaths", CM),
        ("xwK--rNDI9E", "Venn Diagrams - Corbettmaths", CM),
        ("Z5BX-LbG7mI", "Probability Tree Diagrams - GCSE Maths", FIRSTCLASS),
    ]),
    ("maths_edx:P9", &[
        ("xhFDlmQUAZo", "Conditional Probability - Corbettmaths", CM),
        ("nIeMiayWVvw", "Conditional Probability - GCSE Higher Maths", FIRSTCLASS),
        ("34WnM69jaGs", "Conditional Probability", MGENIE),
    ]),
    ("maths_edx:S1", &[
        ("tBlVp3v3J5g", "Random Sampling - Corbettmaths", CM),
        ("n4lyeQAacOU", "Using Samples - Corbettmaths", CM),
        ("IgYrPnA14tM", "Capture Recapture - Corbettmaths", CM),
    ]),
    ("maths_edx:S2-4", &[
        ("wGzp-wM90EU", "Drawing Histograms - Corbettmaths", CM),
        ("PzBE82c1dfY", "Cumulative Frequency Diagrams - GCSE Higher Maths", FIRSTCLASS),
        ("z41_PBqYuVg", "Drawing and Reading Box Plots - Corbettmaths", CM),
    ]),
    ("maths_edx:S5-6", &[
        ("VUaOCgJTPjI", "Scatter Graphs - Corbettmaths", CM),
        ("hlGrp8X3XyY", "Scatter Graphs Correlation - Corbettmaths", CM),
        ("kVr15gM0tCE", "Comparing Distributions", MGENIE),
    ]),
    // Maths (AQA GCSE Mathematics (8300) Higher)
    ("maths_aqa:N1-3", &[
        ("if8ZsZXhQJE", "Order of Operations - Corbettmaths", CM),
        ("mED76j4Agiw", "Addition and Subtraction involving Negatives - Corbettmaths", CM),
        ("70cAYYCJBuQ", "How to use BODMAS (Order of Operations)", COG),
    ]),
    ("maths_aqa:N4-5", &[
        ("oK-EFDLeEqc", "LCM HCF using Product of Primes - Corbettmaths", CM),
        ("kHLwbPwvTtw", "HCF/LCM - GCSE Maths", FIRSTCLASS),
        ("3H6ET7P902Q", "Product Rule for Counting - Corbettmaths", CM),
    ]),
    ("maths_aqa:N6-7", &[
        ("ozuXy8_NZcg", "Laws of Indices - Corbettmaths", CM),
        ("qYDClSo89eQ", "Fractional indices - Corbettmaths", CM),
        ("DvNYkbafpIY", "GCSE Maths - What to do when Powers are Fractions (Powers Part 6/6) (2026/27 exams)", COG),
    ]),
    ("maths_aqa:N8", &[
        ("ndU_cCbPAm4", "Surds - Corbettmaths", CM),
        ("96SwZpRvhwY", "Rationalising denominators - Corbettmaths", CM),
        ("I_Mys8RNt30", "Calculating With Surds - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:N9", &[
        ("cxGyZ3Yx9ow", "Standard Form - Corbettmaths", CM),
        ("H3ewmorcYjU", "What is Standard Form (also known as Scientific Notation)? (Part 1/4) (2026/27 exams)", COG),
        ("u3FZaXs3hDE", "How to Multiply and Divide in Standard Form (Part 3/4) (2026/27 exams)", COG),
    ]),
    ("maths_aqa:N10-12", &[
        ("KZbKYokJ3SQ", "Recurring decimals to fractions - Corbettmaths", CM),
        ("RCnSGUpoKbE", "How to Convert Recurring Decimals to Fractions (Proportions Part 6/6) (2026/27 exams)", COG),
        ("Iq-6CjlEUW4", "Fractions decimals percentages - Corbettmaths", CM),
    ]),
    ("maths_aqa:N13-16", &[
        ("ebMrP74boHw", "Lower and Upper Bounds - Corbettmaths", CM),
        ("FQ8IFKNhphM", "Error Intervals - Corbettmaths", CM),
        ("JTQ2Wh5E2js", "How to Estimate in Maths (2026/27 exams)", COG),
    ]),
    ("maths_aqa:A1-3", &[
        ("QvxWrYtzrtM", "GCSE Maths - Expressions vs Equations (2026/27 exams)", COG),
        ("28DkE4vMN6o", "Substitution into Expressions - Corbettmaths", CM),
        ("l54us4Q7nNY", "Equating Coefficients - Corbettmaths", CM),
    ]),
    ("maths_aqa:A4", &[
        ("X-djBcWVizM", "Factorising quadratics 1 - Corbettmaths", CM),
        ("nfLb8MPO99U", "GCSE Maths - Factorising Quadratics - Part 2 - (When the x² Coefficient is More Than 1)", COG),
        ("YtHMjuB9f_g", "Algebraic Fractions (Operations) - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:A5-6", &[
        ("8U9u_itcs7k", "Changing the Subject - Corbettmaths", CM),
        ("pd9Q-e1JvtE", "Algebraic Proof - Corbettmaths", CM),
        ("5lcefrczJlE", "GCSE Maths - Rearranging Formulas Part 2 - When The Subject Appears Twice (2026/27 exams)", COG),
    ]),
    ("maths_aqa:A7", &[
        ("u1YQVzrgYDg", "Composite Functions - Corbettmaths", CM),
        ("zpF9nbjResY", "Inverse Functions - Corbettmaths", CM),
        ("ZRQJGecu1fs", "Function Notation - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:A8-10", &[
        ("nUk47WSiS30", "GCSE Maths - What on Earth is y = mx + c (2026/27 exams)", COG),
        ("YtHJP1rZ3pI", "Gradient of a Line - Corbettmaths", CM),
        ("PrwhdgnLK5k", "Perpendicular graphs - Corbettmaths", CM),
    ]),
    ("maths_aqa:A11-12", &[
        ("7xE5pj9-n1Q", "Finding Turning Points using Completing the Square", CM),
        ("oUcjmGThMdc", "Exponential Graphs - Corbettmaths", CM),
        ("fNH5EWWtf7k", "Trigonometric Graphs - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:A13", &[
        ("eiRZATuHYg0", "Transformations of Graphs - Corbettmaths", CM),
        ("F8YGp_j7YhM", "Transforming Graphs", MGENIE),
        ("ctVr9NpSiL4", "Transformations of Graphs - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:A14-15", &[
        ("1AVtslXytRA", "Area Under Graph - Corbettmaths", CM),
        ("cEp7qD6vCSM", "Gradient of a Curve - Corbettmaths", CM),
        ("UsmhVCjfzYQ", "Speed Time Graphs - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:A16", &[
        ("_DOBTxLmUTM", "Equation of a Circle - Corbettmaths", CM),
        ("NHrb8N9oAUY", "Equation of a Tangent to a Circle - Corbettmaths", CM),
        ("12NqSRpfTR4", "Equation of a Circle - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:A17", &[
        ("30S7WxKcPwg", "Solving Equations - Corbettmaths", CM),
        ("85ZM3ZKqRhY", "Solving equations with letters on both sides - Corbettmaths", CM),
        ("9FuR91H8EVU", "GCSE Maths - How to Solve Algebraic Equations (Part 1 of 3) (2026/27 exams)", COG),
    ]),
    ("maths_aqa:A18", &[
        ("wJ_tLEwEEi8", "Solving Quadratics using Factorisation - Corbettmaths", CM),
        ("3J0ccr74LcU", "Quadratic formula - Corbettmaths", CM),
        ("abBgTO8eW-c", "Solving Quadratic Equations by Completing the Square - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:A19", &[
        ("phlus4x0UqM", "Simultaneous Equations elimination - Corbettmaths", CM),
        ("ozP-vf99DK4", "Simultaneous equations (linear and non-linear) - Corbettmaths", CM),
        ("eEYreNDTvKQ", "Non-Linear Simultaneous Equations - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:A20", &[
        ("eWP15jyatIo", "Iteration - Corbettmaths", CM),
        ("WHZ2IiKcqeU", "Iteration - GCSE Higher Maths", FIRSTCLASS),
        ("FPMimYoO8kU", "Change of Sign - Corbettmaths", CM),
    ]),
    ("maths_aqa:A21-22", &[
        ("Lz3VkLrDmhE", "Forming equations - Corbettmaths", CM),
        ("u-YNzmlZWeg", "GCSE Maths - Solving Algebraic Inequalities with 1 Inequality Sign (Inequalities Part 2)", COG),
        ("8J_m-hMp8lY", "Quadratic Inequalities - Corbettmaths", CM),
    ]),
    ("maths_aqa:A23-25", &[
        ("qnVVTBAfNu4", "The nth Term - Corbettmaths", CM),
        ("AL-joUBnEIw", "Quadratic Sequences Version 1 - Corbettmaths", CM),
        ("871OBfK5o2M", "GCSE Maths - Types of Number Sequences - Arithmetic vs Geometric (2026/27 exams)", COG),
    ]),
    ("maths_aqa:R1-2", &[
        ("1az6Gjb2wtk", "Converting Metric Units for Length", CM),
        ("2PZ41oDEZ_Q", "Maps and Scales - Corbettmaths", CM),
        ("6XpBX-7cDPE", "GCSE Maths - Using Scales on Maps and Scale Diagrams (2026/27 exams)", COG),
    ]),
    ("maths_aqa:R3-8", &[
        ("UcPVAh4igpI", "GCSE Maths - What are Ratios & How to Simplify Them (Part 1) (2026/27 exams)", COG),
        ("cflZnf9H5l4", "Ratio sharing the total - Corbettmaths", CM),
        ("SJhomYlGPZ8", "Given Two Ratios - Corbettmaths", CM),
    ]),
    ("maths_aqa:R9", &[
        ("tUtgC7ZrsRc", "Increasing Decreasing by a Percentage - Corbettmaths", CM),
        ("Q2gRAS08fE0", "Percentage Change - Corbettmaths", CM),
        ("dIeb2ryQ1ko", "GCSE Maths - Reverse Percentages - Calculating The Cost Before The Discount (2026/27 exams)", COG),
    ]),
    ("maths_aqa:R10", &[
        ("AXkCfkVrjK8", "Unitary Method - Corbettmaths", CM),
        ("z9JlSDzSy3c", "GCSE Maths - What Does Directly Proportional Mean? (2026/27 exams)", COG),
        ("01l1a21qDaM", "Proportion and Time - Corbettmaths", CM),
    ]),
    ("maths_aqa:R11", &[
        ("dHVK7IeLGT8", "Speed, Distance, Time - Corbettmaths", CM),
        ("sv7zflLeduM", "Density - Corbettmaths", CM),
        ("fheOYg9TKQA", "Pressure - Corbettmaths", CM),
    ]),
    ("maths_aqa:R12", &[
        ("8-4KQw8unfY", "GCSE Maths - Similar Shapes (2026/27 exams)", COG),
        ("GlD88EpEJVo", "Similar Shapes: Areas", CM),
        ("0U1T_f5xb98", "Similar Areas and Volumes - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:R13", &[
        ("kcOwC7uqJNE", "Direct Proportion - Corbettmaths", CM),
        ("uZ6l-loSdRs", "Inverse Proportion - Corbettmaths", CM),
        ("pXnyxf55nJU", "Proportionality Graphs - Corbettmaths", CM),
    ]),
    ("maths_aqa:R14-15", &[
        ("rGf6_1E8x7k", "Average Rate of Change - Corbettmaths", CM),
        ("YfDhGn3NLQw", "Instantaneous Rate of Change - Corbettmaths", CM),
        ("zVSq5b3PPfY", "GCSE Maths - How to Find the Gradient of a Straight Line (2026/27 exams)", COG),
    ]),
    ("maths_aqa:R16", &[
        ("FBCs95Co_oU", "Compound Interest - Corbettmaths", CM),
        ("gliYGWM7wfY", "Compound Interest and Depreciation", MGENIE),
        ("CRKlXB5g1gE", "Exponential Growth and Decay", MGENIE),
    ]),
    ("maths_aqa:G1-2", &[
        ("1beKcgU9ogE", "Perpendicular Bisectors - Corbettmaths", CM),
        ("BWj041al8z8", "Loci part 1 - Corbettmaths", CM),
        ("3viWgGJmkFo", "Constructions - GCSE Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:G3-4", &[
        ("gVo8ZrtlSp0", "Angles in Polygons - Corbettmaths", CM),
        ("I5auyoXYoX0", "GCSE Maths - Alternate, Corresponding and Allied Angles - Parallel Lines Angle Rules (2026/27 exams)", COG),
        ("mlG56WCfobI", "GCSE Maths - Types of Quadrilateral (2026/27 exams)", COG),
    ]),
    ("maths_aqa:G5-6", &[
        ("IDW1ogTqox8", "Congruent Triangles - Corbettmaths", CM),
        ("aK8i7LKZd9o", "GCSE Maths - Congruent Triangle Rules (2026/27 exams)", COG),
        ("4YdhDXJWCZ8", "Geometric Proof - Corbettmaths", CM),
    ]),
    ("maths_aqa:G7-8", &[
        ("rgdRlbbWQgA", "Rotations - Corbettmaths", CM),
        ("u2EgwMYwibw", "Describing Enlargements - Corbettmaths", CM),
        ("0_hclyERGAw", "Enlargements (Negative Scale Factor) - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:G9-10", &[
        ("vgMSLsos7Ew", "Circle Theorems - Corbettmaths", CM),
        ("dBIlCD_JF9Q", "Circle Theorems - GCSE Higher Maths", FIRSTCLASS),
        ("CmprPWcLtlk", "Circle Theorem Proofs - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:G11-15", &[
        ("BnwoipoGWJ8", "Views and Elevations", CM),
        ("pm8i-thxvCo", "GCSE Maths - What are Bearings? (2026/27 exams)", COG),
        ("8Wja7Ct_XvY", "Bearings - Corbettmaths", CM),
    ]),
    ("maths_aqa:G16-18", &[
        ("Wcv0f5PpTv0", "GCSE Maths - Area of a Sector and Length of an Arc of a Circle  (Circles Part 3) (2026/27 exams)", COG),
        ("X6cMvcxk1ig", "Volume of a Cone - Corbettmaths", CM),
        ("VwdMbDpMab4", "Surface Area of Prisms - Corbettmaths", CM),
    ]),
    ("maths_aqa:G19", &[
        ("L6DLoBMknoY", "Similar Shapes - Missing Sides", CM),
        ("QMI90tONvzQ", "Similar Shapes: Volumes", CM),
        ("u1uOcrjQCh4", "Similar Triangles - GCSE Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:G20-21", &[
        ("iWLVTy_rGjs", "Pythagoras - Corbettmaths", CM),
        ("WFH_7n7hpHo", "Trigonometry | SOH CAH TOA | Sin, Cos, Tan", COG),
        ("UjgOR07zOzY", "Exact trigonometric values - Corbettmaths", CM),
    ]),
    ("maths_aqa:G22-23", &[
        ("7xeLeDulY60", "The Sine Rule - GCSE Higher Maths", FIRSTCLASS),
        ("3H3u92WJAjw", "Cosine rule - Corbettmaths", CM),
        ("eSFOMSxjMts", "Area of any Triangle - Corbettmaths", CM),
    ]),
    ("maths_aqa:G24-25", &[
        ("h02d922Q5wk", "Column Vectors - Corbettmaths", CM),
        ("xOdkldbusy0", "Vectors - Corbettmaths", CM),
        ("vkwFUig3dGY", "Vectors in Shapes", MGENIE),
    ]),
    ("maths_aqa:P1-5", &[
        ("ur_hHjLrBNo", "Probability - Corbettmaths", CM),
        ("MS6lnCTgTSw", "Relative Frequency - Corbettmaths", CM),
        ("QDnDtxDEHWg", "Probability and Relative Frequency", MGENIE),
    ]),
    ("maths_aqa:P6-8", &[
        ("Xqno7W0OUtE", "Sample Space Diagrams - Corbettmaths", CM),
        ("xwK--rNDI9E", "Venn Diagrams - Corbettmaths", CM),
        ("Z5BX-LbG7mI", "Probability Tree Diagrams - GCSE Maths", FIRSTCLASS),
    ]),
    ("maths_aqa:P9", &[
        ("xhFDlmQUAZo", "Conditional Probability - Corbettmaths", CM),
        ("nIeMiayWVvw", "Conditional Probability - GCSE Higher Maths", FIRSTCLASS),
        ("34WnM69jaGs", "Conditional Probability", MGENIE),
    ]),
    ("maths_aqa:S1", &[
        ("tBlVp3v3J5g", "Random Sampling - Corbettmaths", CM),
        ("n4lyeQAacOU", "Using Samples - Corbettmaths", CM),
        ("IgYrPnA14tM", "Capture Recapture - Corbettmaths", CM),
    ]),
    ("maths_aqa:S2-4", &[
        ("wGzp-wM90EU", "Drawing Histograms - Corbettmaths", CM),
        ("PzBE82c1dfY", "Cumulative Frequency Diagrams - GCSE Higher Maths", FIRSTCLASS),
        ("z41_PBqYuVg", "Drawing and Reading Box Plots - Corbettmaths", CM),
    ]),
    ("maths_aqa:S5-6", &[
        ("VUaOCgJTPjI", "Scatter Graphs - Corbettmaths", CM),
        ("hlGrp8X3XyY", "Scatter Graphs Correlation - Corbettmaths", CM),
        ("kVr15gM0tCE", "Comparing Distributions", MGENIE),
    ]),
    // Maths (OCR GCSE Mathematics (J560) Higher)
    ("maths_ocr:N1-3", &[
        ("if8ZsZXhQJE", "Order of Operations - Corbettmaths", CM),
        ("mED76j4Agiw", "Addition and Subtraction involving Negatives - Corbettmaths", CM),
        ("70cAYYCJBuQ", "How to use BODMAS (Order of Operations)", COG),
    ]),
    ("maths_ocr:N4-5", &[
        ("oK-EFDLeEqc", "LCM HCF using Product of Primes - Corbettmaths", CM),
        ("kHLwbPwvTtw", "HCF/LCM - GCSE Maths", FIRSTCLASS),
        ("3H6ET7P902Q", "Product Rule for Counting - Corbettmaths", CM),
    ]),
    ("maths_ocr:N6-7", &[
        ("ozuXy8_NZcg", "Laws of Indices - Corbettmaths", CM),
        ("qYDClSo89eQ", "Fractional indices - Corbettmaths", CM),
        ("DvNYkbafpIY", "GCSE Maths - What to do when Powers are Fractions (Powers Part 6/6) (2026/27 exams)", COG),
    ]),
    ("maths_ocr:N8", &[
        ("ndU_cCbPAm4", "Surds - Corbettmaths", CM),
        ("96SwZpRvhwY", "Rationalising denominators - Corbettmaths", CM),
        ("I_Mys8RNt30", "Calculating With Surds - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:N9", &[
        ("cxGyZ3Yx9ow", "Standard Form - Corbettmaths", CM),
        ("H3ewmorcYjU", "What is Standard Form (also known as Scientific Notation)? (Part 1/4) (2026/27 exams)", COG),
        ("u3FZaXs3hDE", "How to Multiply and Divide in Standard Form (Part 3/4) (2026/27 exams)", COG),
    ]),
    ("maths_ocr:N10-12", &[
        ("KZbKYokJ3SQ", "Recurring decimals to fractions - Corbettmaths", CM),
        ("RCnSGUpoKbE", "How to Convert Recurring Decimals to Fractions (Proportions Part 6/6) (2026/27 exams)", COG),
        ("Iq-6CjlEUW4", "Fractions decimals percentages - Corbettmaths", CM),
    ]),
    ("maths_ocr:N13-16", &[
        ("ebMrP74boHw", "Lower and Upper Bounds - Corbettmaths", CM),
        ("FQ8IFKNhphM", "Error Intervals - Corbettmaths", CM),
        ("JTQ2Wh5E2js", "How to Estimate in Maths (2026/27 exams)", COG),
    ]),
    ("maths_ocr:A1-3", &[
        ("QvxWrYtzrtM", "GCSE Maths - Expressions vs Equations (2026/27 exams)", COG),
        ("28DkE4vMN6o", "Substitution into Expressions - Corbettmaths", CM),
        ("l54us4Q7nNY", "Equating Coefficients - Corbettmaths", CM),
    ]),
    ("maths_ocr:A4", &[
        ("X-djBcWVizM", "Factorising quadratics 1 - Corbettmaths", CM),
        ("nfLb8MPO99U", "GCSE Maths - Factorising Quadratics - Part 2 - (When the x² Coefficient is More Than 1)", COG),
        ("YtHMjuB9f_g", "Algebraic Fractions (Operations) - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:A5-6", &[
        ("8U9u_itcs7k", "Changing the Subject - Corbettmaths", CM),
        ("pd9Q-e1JvtE", "Algebraic Proof - Corbettmaths", CM),
        ("5lcefrczJlE", "GCSE Maths - Rearranging Formulas Part 2 - When The Subject Appears Twice (2026/27 exams)", COG),
    ]),
    ("maths_ocr:A7", &[
        ("u1YQVzrgYDg", "Composite Functions - Corbettmaths", CM),
        ("zpF9nbjResY", "Inverse Functions - Corbettmaths", CM),
        ("ZRQJGecu1fs", "Function Notation - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:A8-10", &[
        ("nUk47WSiS30", "GCSE Maths - What on Earth is y = mx + c (2026/27 exams)", COG),
        ("YtHJP1rZ3pI", "Gradient of a Line - Corbettmaths", CM),
        ("PrwhdgnLK5k", "Perpendicular graphs - Corbettmaths", CM),
    ]),
    ("maths_ocr:A11-12", &[
        ("7xE5pj9-n1Q", "Finding Turning Points using Completing the Square", CM),
        ("oUcjmGThMdc", "Exponential Graphs - Corbettmaths", CM),
        ("fNH5EWWtf7k", "Trigonometric Graphs - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:A13", &[
        ("eiRZATuHYg0", "Transformations of Graphs - Corbettmaths", CM),
        ("F8YGp_j7YhM", "Transforming Graphs", MGENIE),
        ("ctVr9NpSiL4", "Transformations of Graphs - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:A14-15", &[
        ("1AVtslXytRA", "Area Under Graph - Corbettmaths", CM),
        ("cEp7qD6vCSM", "Gradient of a Curve - Corbettmaths", CM),
        ("UsmhVCjfzYQ", "Speed Time Graphs - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:A16", &[
        ("_DOBTxLmUTM", "Equation of a Circle - Corbettmaths", CM),
        ("NHrb8N9oAUY", "Equation of a Tangent to a Circle - Corbettmaths", CM),
        ("12NqSRpfTR4", "Equation of a Circle - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:A17", &[
        ("30S7WxKcPwg", "Solving Equations - Corbettmaths", CM),
        ("85ZM3ZKqRhY", "Solving equations with letters on both sides - Corbettmaths", CM),
        ("9FuR91H8EVU", "GCSE Maths - How to Solve Algebraic Equations (Part 1 of 3) (2026/27 exams)", COG),
    ]),
    ("maths_ocr:A18", &[
        ("wJ_tLEwEEi8", "Solving Quadratics using Factorisation - Corbettmaths", CM),
        ("3J0ccr74LcU", "Quadratic formula - Corbettmaths", CM),
        ("abBgTO8eW-c", "Solving Quadratic Equations by Completing the Square - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:A19", &[
        ("phlus4x0UqM", "Simultaneous Equations elimination - Corbettmaths", CM),
        ("ozP-vf99DK4", "Simultaneous equations (linear and non-linear) - Corbettmaths", CM),
        ("eEYreNDTvKQ", "Non-Linear Simultaneous Equations - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:A20", &[
        ("eWP15jyatIo", "Iteration - Corbettmaths", CM),
        ("WHZ2IiKcqeU", "Iteration - GCSE Higher Maths", FIRSTCLASS),
        ("FPMimYoO8kU", "Change of Sign - Corbettmaths", CM),
    ]),
    ("maths_ocr:A21-22", &[
        ("Lz3VkLrDmhE", "Forming equations - Corbettmaths", CM),
        ("u-YNzmlZWeg", "GCSE Maths - Solving Algebraic Inequalities with 1 Inequality Sign (Inequalities Part 2)", COG),
        ("8J_m-hMp8lY", "Quadratic Inequalities - Corbettmaths", CM),
    ]),
    ("maths_ocr:A23-25", &[
        ("qnVVTBAfNu4", "The nth Term - Corbettmaths", CM),
        ("AL-joUBnEIw", "Quadratic Sequences Version 1 - Corbettmaths", CM),
        ("871OBfK5o2M", "GCSE Maths - Types of Number Sequences - Arithmetic vs Geometric (2026/27 exams)", COG),
    ]),
    ("maths_ocr:R1-2", &[
        ("1az6Gjb2wtk", "Converting Metric Units for Length", CM),
        ("2PZ41oDEZ_Q", "Maps and Scales - Corbettmaths", CM),
        ("6XpBX-7cDPE", "GCSE Maths - Using Scales on Maps and Scale Diagrams (2026/27 exams)", COG),
    ]),
    ("maths_ocr:R3-8", &[
        ("UcPVAh4igpI", "GCSE Maths - What are Ratios & How to Simplify Them (Part 1) (2026/27 exams)", COG),
        ("cflZnf9H5l4", "Ratio sharing the total - Corbettmaths", CM),
        ("SJhomYlGPZ8", "Given Two Ratios - Corbettmaths", CM),
    ]),
    ("maths_ocr:R9", &[
        ("tUtgC7ZrsRc", "Increasing Decreasing by a Percentage - Corbettmaths", CM),
        ("Q2gRAS08fE0", "Percentage Change - Corbettmaths", CM),
        ("dIeb2ryQ1ko", "GCSE Maths - Reverse Percentages - Calculating The Cost Before The Discount (2026/27 exams)", COG),
    ]),
    ("maths_ocr:R10", &[
        ("AXkCfkVrjK8", "Unitary Method - Corbettmaths", CM),
        ("z9JlSDzSy3c", "GCSE Maths - What Does Directly Proportional Mean? (2026/27 exams)", COG),
        ("01l1a21qDaM", "Proportion and Time - Corbettmaths", CM),
    ]),
    ("maths_ocr:R11", &[
        ("dHVK7IeLGT8", "Speed, Distance, Time - Corbettmaths", CM),
        ("sv7zflLeduM", "Density - Corbettmaths", CM),
        ("fheOYg9TKQA", "Pressure - Corbettmaths", CM),
    ]),
    ("maths_ocr:R12", &[
        ("8-4KQw8unfY", "GCSE Maths - Similar Shapes (2026/27 exams)", COG),
        ("GlD88EpEJVo", "Similar Shapes: Areas", CM),
        ("0U1T_f5xb98", "Similar Areas and Volumes - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:R13", &[
        ("kcOwC7uqJNE", "Direct Proportion - Corbettmaths", CM),
        ("uZ6l-loSdRs", "Inverse Proportion - Corbettmaths", CM),
        ("pXnyxf55nJU", "Proportionality Graphs - Corbettmaths", CM),
    ]),
    ("maths_ocr:R14-15", &[
        ("rGf6_1E8x7k", "Average Rate of Change - Corbettmaths", CM),
        ("YfDhGn3NLQw", "Instantaneous Rate of Change - Corbettmaths", CM),
        ("zVSq5b3PPfY", "GCSE Maths - How to Find the Gradient of a Straight Line (2026/27 exams)", COG),
    ]),
    ("maths_ocr:R16", &[
        ("FBCs95Co_oU", "Compound Interest - Corbettmaths", CM),
        ("gliYGWM7wfY", "Compound Interest and Depreciation", MGENIE),
        ("CRKlXB5g1gE", "Exponential Growth and Decay", MGENIE),
    ]),
    ("maths_ocr:G1-2", &[
        ("1beKcgU9ogE", "Perpendicular Bisectors - Corbettmaths", CM),
        ("BWj041al8z8", "Loci part 1 - Corbettmaths", CM),
        ("3viWgGJmkFo", "Constructions - GCSE Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:G3-4", &[
        ("gVo8ZrtlSp0", "Angles in Polygons - Corbettmaths", CM),
        ("I5auyoXYoX0", "GCSE Maths - Alternate, Corresponding and Allied Angles - Parallel Lines Angle Rules (2026/27 exams)", COG),
        ("mlG56WCfobI", "GCSE Maths - Types of Quadrilateral (2026/27 exams)", COG),
    ]),
    ("maths_ocr:G5-6", &[
        ("IDW1ogTqox8", "Congruent Triangles - Corbettmaths", CM),
        ("aK8i7LKZd9o", "GCSE Maths - Congruent Triangle Rules (2026/27 exams)", COG),
        ("4YdhDXJWCZ8", "Geometric Proof - Corbettmaths", CM),
    ]),
    ("maths_ocr:G7-8", &[
        ("rgdRlbbWQgA", "Rotations - Corbettmaths", CM),
        ("u2EgwMYwibw", "Describing Enlargements - Corbettmaths", CM),
        ("0_hclyERGAw", "Enlargements (Negative Scale Factor) - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:G9-10", &[
        ("vgMSLsos7Ew", "Circle Theorems - Corbettmaths", CM),
        ("dBIlCD_JF9Q", "Circle Theorems - GCSE Higher Maths", FIRSTCLASS),
        ("CmprPWcLtlk", "Circle Theorem Proofs - GCSE Higher Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:G11-15", &[
        ("BnwoipoGWJ8", "Views and Elevations", CM),
        ("pm8i-thxvCo", "GCSE Maths - What are Bearings? (2026/27 exams)", COG),
        ("8Wja7Ct_XvY", "Bearings - Corbettmaths", CM),
    ]),
    ("maths_ocr:G16-18", &[
        ("Wcv0f5PpTv0", "GCSE Maths - Area of a Sector and Length of an Arc of a Circle  (Circles Part 3) (2026/27 exams)", COG),
        ("X6cMvcxk1ig", "Volume of a Cone - Corbettmaths", CM),
        ("VwdMbDpMab4", "Surface Area of Prisms - Corbettmaths", CM),
    ]),
    ("maths_ocr:G19", &[
        ("L6DLoBMknoY", "Similar Shapes - Missing Sides", CM),
        ("QMI90tONvzQ", "Similar Shapes: Volumes", CM),
        ("u1uOcrjQCh4", "Similar Triangles - GCSE Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:G20-21", &[
        ("iWLVTy_rGjs", "Pythagoras - Corbettmaths", CM),
        ("WFH_7n7hpHo", "Trigonometry | SOH CAH TOA | Sin, Cos, Tan", COG),
        ("UjgOR07zOzY", "Exact trigonometric values - Corbettmaths", CM),
    ]),
    ("maths_ocr:G22-23", &[
        ("7xeLeDulY60", "The Sine Rule - GCSE Higher Maths", FIRSTCLASS),
        ("3H3u92WJAjw", "Cosine rule - Corbettmaths", CM),
        ("eSFOMSxjMts", "Area of any Triangle - Corbettmaths", CM),
    ]),
    ("maths_ocr:G24-25", &[
        ("h02d922Q5wk", "Column Vectors - Corbettmaths", CM),
        ("xOdkldbusy0", "Vectors - Corbettmaths", CM),
        ("vkwFUig3dGY", "Vectors in Shapes", MGENIE),
    ]),
    ("maths_ocr:P1-5", &[
        ("ur_hHjLrBNo", "Probability - Corbettmaths", CM),
        ("MS6lnCTgTSw", "Relative Frequency - Corbettmaths", CM),
        ("QDnDtxDEHWg", "Probability and Relative Frequency", MGENIE),
    ]),
    ("maths_ocr:P6-8", &[
        ("Xqno7W0OUtE", "Sample Space Diagrams - Corbettmaths", CM),
        ("xwK--rNDI9E", "Venn Diagrams - Corbettmaths", CM),
        ("Z5BX-LbG7mI", "Probability Tree Diagrams - GCSE Maths", FIRSTCLASS),
    ]),
    ("maths_ocr:P9", &[
        ("xhFDlmQUAZo", "Conditional Probability - Corbettmaths", CM),
        ("nIeMiayWVvw", "Conditional Probability - GCSE Higher Maths", FIRSTCLASS),
        ("34WnM69jaGs", "Conditional Probability", MGENIE),
    ]),
    ("maths_ocr:S1", &[
        ("tBlVp3v3J5g", "Random Sampling - Corbettmaths", CM),
        ("n4lyeQAacOU", "Using Samples - Corbettmaths", CM),
        ("IgYrPnA14tM", "Capture Recapture - Corbettmaths", CM),
    ]),
    ("maths_ocr:S2-4", &[
        ("wGzp-wM90EU", "Drawing Histograms - Corbettmaths", CM),
        ("PzBE82c1dfY", "Cumulative Frequency Diagrams - GCSE Higher Maths", FIRSTCLASS),
        ("z41_PBqYuVg", "Drawing and Reading Box Plots - Corbettmaths", CM),
    ]),
    ("maths_ocr:S5-6", &[
        ("VUaOCgJTPjI", "Scatter Graphs - Corbettmaths", CM),
        ("hlGrp8X3XyY", "Scatter Graphs Correlation - Corbettmaths", CM),
        ("kVr15gM0tCE", "Comparing Distributions", MGENIE),
    ]),
    // WJEC Eduqas GCSE English Language C700QS. Found by YouTube search on
    // 30 September 2026; every id, title and channel checked against YouTube's
    // oEmbed response. Access GCSEPod has a tips video and an example-responses
    // video for each Eduqas question; TeachGCSEEnglish, GuigLit and Literature
    // Daydreams walk through the Eduqas papers and writing forms. Mr Bruff,
    // First Rate Tutors and Mr Salles cover general skills (AO6, techniques,
    // persuasion) that apply to every board.
    ("englang_edq:2.1a", &[
        ("hg6NNRybDxk", "How to MASTER GCSE English Language Fiction Reading (Paper 1A - EDUQAS)", "GuigLit"),
        ("-bI7DjvE_JM", "EDUQAS PAPER 1 EXAM WALKTHROUGH (All questions!) GCSE English Language", "TeachGCSEEnglish"),
        ("eQukZ25v1rQ", "THE FIVE TOP TIPS - EDUQAS GCSE ENGLISH LANGUAGE PAPER 1", "GuigLit"),
    ]),
    ("englang_edq:2.1b", &[
        ("YvbZo4C5GC8", "GCSEPod Eduqas English Language Component 1, Question 1, Tips for Success", "Access GCSEPod"),
        ("koEabjE5oHg", "GCSEPod Eduqas English Language Component 1, Question 1, Example Responses", "Access GCSEPod"),
        ("EHvLzOA-ZnU", "Eduqas English Language Paper 1 Question 1-2, English Language Revision, Eduqas English Component 1", "Literature Daydreams"),
    ]),
    ("englang_edq:2.1c", &[
        ("uW_Ru_yEg5A", "GCSEPod English Language Eduqas Component 1 Question 2 Tips for Success", "Access GCSEPod"),
        ("OafVU1vAYKs", "GCSEPod Eduqas English Language Component 1, Question 2, Example Responses", "Access GCSEPod"),
        ("et3zShu2GxM", "EDUQAS GCSE English Language Paper 1 IMPRESSIONS question (dog extract)", "TeachGCSEEnglish"),
    ]),
    ("englang_edq:2.1d", &[
        ("9me2bd5kV98", "GCSEPod Eduqas English Language Component 1, Question 3, Tips for Success", "Access GCSEPod"),
        ("YZ-IWVsn6Vo", "GCSEPod Eduqas English Language Component 1, Question 3, Example Responses", "Access GCSEPod"),
        ("D4djHePElqA", "EDUQAS GCSE English Language Paper 1 the 'language' question (Pat & Bruce)", "TeachGCSEEnglish"),
    ]),
    ("englang_edq:2.1e", &[
        ("vFjKkCo-Pkw", "GCSEPod Eduqas English Language Component 1, Question 4, Tips for Success", "Access GCSEPod"),
        ("-_MSBNfWHDA", "Eduqas English Language Paper 1 Question 3-4, English Language Revision, Eduqas English Component 1", "Literature Daydreams"),
        ("O-d-Zg4oRMs", "EDUQAS Paper 1 Reading Q1-Q5 walkthrough 2024 - GCSE English Language", "TeachGCSEEnglish"),
        ("6zXBiAuPQ_Y", "10 Language & Structure Techniques You'll Find In ANY GCSE English Language Exam (AO2 Marks)", "First Rate Tutors"),
    ]),
    ("englang_edq:2.1f", &[
        ("0NaQ8sy0FOA", "GCSEPod Eduqas English Language Component 1, Question 5, Tips for Success", "Access GCSEPod"),
        ("UdeJJhdVmOk", "GCSEPod Eduqas English Language Component 1, Question 5, Example Responses", "Access GCSEPod"),
        ("hTEuzusMTX0", "Eduqas English Language Paper 1 Question 5, English Language Revision, Eduqas English Component 1", "Literature Daydreams"),
    ]),
    ("englang_edq:2.1g", &[
        ("_beYf2WsLw0", "TOP 10 TIPS: EDUQAS SHORT STORY - GCSE ENGLISH LANGUAGE", "TeachGCSEEnglish"),
        ("h_UoZyhQEzA", "EDUQAS SHORT STORY Paper 1 video - GCSE English Language", "TeachGCSEEnglish"),
        ("lJRCpUsi2gw", "3 MINUTE MAGIC REVISION - Paper 1 WRITING (Short Story) EDUQAS GCSE English Language", "TeachGCSEEnglish"),
        ("uwaTu-3aapI", "Eduqas English Language Paper 1 Narrative Writing, Component 1 Story, Eduqas GCSE English", "Literature Daydreams"),
    ]),
    ("englang_edq:2.1h", &[
        ("6uo_1Y3fxQ8", "How to MASTER GCSE Fiction Writing (EDUQAS GCSE English Language)", "GuigLit"),
        ("8yU7Zwq4DVI", "SHORT STORY: TEN TOP TIPS! EDUQAS Section B Paper 1 exam - GCSE English Language", "TeachGCSEEnglish"),
        ("9P9Ymaz-yMk", "GCSE English Language | Characterisation & Narrative Voice | Glecta", "GLECTA KS2 11Plus KS3 GCSE A-Level Tutoring"),
    ]),
    ("englang_edq:2.1i", &[
        ("ECFDyuu0DKk", "A06: Semi Colons (the king of punctuation)", "Mr Bruff"),
        ("1jJunV2HtWw", "Varying Sentence Length: Look at This!", "Mr Bruff"),
        ("yqzOK6uJ9BE", "Varying sentence structure for GCSE English.", "GCSE English hints and tips"),
    ]),
    ("englang_edq:2.2a", &[
        ("qQFVQ7CK120", "How to MASTER GCSE English Language Non-Fiction Reading (EDUQAS)", "GuigLit"),
        ("IGGnAe6-IRU", "Eduqas English Language Component 2 Overview", "Literature Daydreams"),
        ("x8rSea19eoQ", "19th Century Texts | GCSE English Language", "ExamQA"),
        ("QgIjdGYsmz8", "Understanding 19th Century Writing", "Pass My English"),
    ]),
    ("englang_edq:2.2b", &[
        ("aZCu2PaOqqI", "GCSEPod English Language Eduqas Component 2 Questions 1 and 3 Tips for Success", "Access GCSEPod"),
        ("l-NrTEWo1v4", "Eduqas Language: Non-Fiction Reading Q1 and 2", "Miss Bird"),
        ("VvbfzUwkySw", "EDUQAS Paper 2 Reading Q1-Q6 walkthrough 2024 - GCSE English Language", "TeachGCSEEnglish"),
    ]),
    ("englang_edq:2.2c", &[
        ("BnzMwLJr6sk", "GCSEPod English Language Eduqas Component 2 Question 2 Tips for Success", "Access GCSEPod"),
        ("ogjpf32YGRs", "GCSEPod English Language Eduqas Component 2 Question 2 Example Responses", "Access GCSEPod"),
    ]),
    ("englang_edq:2.2d", &[
        ("KYxvTyTDmQ4", "GCSEPod English Language Eduqas Component 2 Question 4 Tips for Success", "Access GCSEPod"),
        ("0-dzjxfNkWo", "GCSEPod English Language Eduqas Component 2 Question 4 Example Responses", "Access GCSEPod"),
        ("hLh58X0yvIk", "EDUQAS PAPER 2 EXAM WALKTHROUGH (All questions!) GCSE English Language", "TeachGCSEEnglish"),
    ]),
    ("englang_edq:2.2e", &[
        ("--cGqbfnLFw", "GCSEPod English Language Eduqas Component 2 Question 5 Tips for Success", "Access GCSEPod"),
        ("ES9-Iosm0UU", "GCSEPod English Language Eduqas Component 2 Question 5 Example Responses", "Access GCSEPod"),
        ("ue4WltNoapA", "EDUQAS GCSE English Language Paper 2 Question 5 - 'COMPARISON' (Captain Scott / Ben Fogle exam)", "TeachGCSEEnglish"),
    ]),
    ("englang_edq:2.2f", &[
        ("vhUUT_73BZ8", "GCSEPod English Language Eduqas Component 2 Question 6 Tips for Success", "Access GCSEPod"),
        ("EK-DEU5dbq0", "GCSEPod English Language Eduqas Component 2 Question 6 Example Responses", "Access GCSEPod"),
        ("niBN2CFteWU", "EDUQAS GCSE English Language Paper 2 Question 6 - 'COMPARE' 10 marks (Captain Scott/Ben Fogle exam)", "TeachGCSEEnglish"),
        ("fX0Tpu9zP3I", "Eduqas English Language Comp 2 Q15-16, Eduqas Comparison Questions", "Literature Daydreams"),
    ]),
    ("englang_edq:2.2g", &[
        ("xvHuwPBVUlk", "LETTERS - Paper 2 writing exam (EDUQAS GCSE English Language)", "TeachGCSEEnglish"),
        ("MdoJVlSjjR0", "ARTICLES - Paper 2 writing exam (EDUQAS GCSE English Language)", "TeachGCSEEnglish"),
        ("1HDvxjD79sY", "REVIEWS - Paper 2 writing exam (EDUQAS GCSE English Language)", "TeachGCSEEnglish"),
        ("Ci3BKzcdtNU", "SPEECHES - Paper 2 writing exam (EDUQAS GCSE English Language)", "TeachGCSEEnglish"),
        ("N23gOW7f058", "GUIDES - EDUQAS PAPER 2 WRITING", "TeachGCSEEnglish"),
        ("t099uy29RpI", "Eduqas English Language Paper 2, Write a Report, English Language Revision, Eduqas English", "Literature Daydreams"),
    ]),
    ("englang_edq:2.2h", &[
        ("_a9cibqX9Y4", "Eduqas GCSE English Language Transactional Writing", "Easy Ed"),
        ("n8qiyil-6DE", "These 15 PERSUASIVE Techniques DOMINATE Q5", "Mr Salles Teaches English"),
        ("KnRQPw-hExU", "How to Get a GRADE 9: Eduqas Paper 2 Section B Writing", "TeachGCSEEnglish"),
    ]),
    ("englang_edq:2.2i", &[
        ("JZ01FWPmDG8", "2025 EDUQAS Paper 2 Writing exam walkthrough - GCSE English Language", "TeachGCSEEnglish"),
        ("IKnQtUfZF78", "2025 EDUQAS Paper 2 Writing Mega Revision!", "TeachGCSEEnglish"),
        ("a7zv5T6s_z8", "EDUQAS GCSE ENGLISH LANGUAGE PAPER 2 - LAST-MINUTE ADVICE", "GuigLit"),
    ]),
    // Chemistry (AQA GCSE Chemistry (8462) Higher)
    ("chem_aqa:4.1.1", &[
        ("nUzOXy9V-K0", "Elements, Compounds and Mixtures", FSL),
        ("9WNFwOqb_24", "The History of the Atom | Models & Theories", COG),
        ("kvTzdD1eGUQ", "Electronic Structure", COG),
    ]),
    ("chem_aqa:4.1.2", &[
        ("IoldeyRWgz8", "Development of the Periodic Table", FSL),
        ("TGZs93DzMDY", "Group 1 Elements - Alkali Metals | Properties | Reactivity", COG),
        ("uYubwlPRdCs", "Group 7 & Group 0 Elements - Halogens & Noble Gases | Properties | Trends", COG),
    ]),
    ("chem_aqa:4.1.3", &[
        ("A5RpVT1WA8E", "Transition Elements", FSL),
        ("XD_ZXoFK1bQ", "The transition elements | Properties and uses", "The Science Break"),
    ]),
    ("chem_aqa:4.2.1", &[
        ("MdU44WeiLps", "Ionic Bonding - Formation | Dot and Cross Diagrams", COG),
        ("m5u4STdFlOE", "Covalent Bonding 1: Bonding in Hydrogen, Chlorine and Hydrogen chloride", FSL),
        ("tRKkGRBndto", "Metallic Bonding - Properties of Metals | Alloys & Pure Metals", COG),
    ]),
    ("chem_aqa:4.2.2", &[
        ("CTwJEtjYffY", "The Three States of Matter", FSL),
        ("ybi6kQHKVws", "Ionic Compounds - Structure | Properties | Formula", COG),
        ("SxuldWdqFB0", "Types of Covalent Structures: Simple Molecular & Giant Covalent Structures", COG),
    ]),
    ("chem_aqa:4.2.3", &[
        ("gUNkLFf2WXU", "Diamond and Silicon Dioxide", FSL),
        ("RVV8pncaA7A", "Diamond & Graphite - Structure | Properties", COG),
        ("cjgODRJU79Y", "Graphene and Fullerenes", FSL),
    ]),
    ("chem_aqa:4.2.4", &[
        ("VxKHzt93h-0", "Nanoparticles", FSL),
        ("NtRgKuNiQ-Y", "Nanoparticles - Uses | Risks", COG),
    ]),
    ("chem_aqa:4.3.1", &[
        ("K4pw_-U6Xpc", "Conservation of Mass", FSL),
        ("-I2mwJH6MhA", "Relative Formula Mass | Mr & Percentage Mass Calculations", COG),
        ("sXeUIGW3nRY", "GCSE Working Scientifically \"Uncertainty\"", FSL),
    ]),
    ("chem_aqa:4.3.2", &[
        ("kBlmEfS_P00", "Moles & Mass - Avogadro's Constant | Formula for Moles, Mass & Mr", COG),
        ("4wTSLBBBMo0", "Using Moles to Balance Equations", FSL),
        ("LTnhZvyOvFU", "Limiting Reactants & Balanced Equations", COG),
    ]),
    ("chem_aqa:4.3.3", &[
        ("9EV0Oq8g708", "Calculating Percentage Yield 1", FSL),
        ("h1-Vj6eh-mM", "Atom Economy", FSL),
        ("eZCtGJwns0Q", "Atom Economy - Calculations | Why it's Important", COG),
    ]),
    ("chem_aqa:4.3.4", &[
        ("rluAIfmi7x8", "Concentration Calculations (grams per dm³)", COG),
        ("x8DLLCNMKAs", "Titration calculations 1", FSL),
        ("saRBT5oZfh8", "Required Practical 2: Carrying out a Titration", FSL),
    ]),
    ("chem_aqa:4.3.5", &[
        ("tYE-1nywlFs", "Using Gas Volumes", FSL),
        ("Xx9xrkbDLog", "Gas Calculations - Volume & Moles | Mass, Moles & Mr", COG),
        ("OzutTl0sYd0", "Using Gas Volumes 2", FSL),
    ]),
    ("chem_aqa:4.4.1", &[
        ("MDQr5QFVGkk", "The Reactivity Series", FSL),
        ("MXTSels6e2Y", "Extraction of Metals", FSL),
        ("QlG9Kt-9yTI", "Redox Reactions - Displacement Reactions | Ionic Equations | Half Equations", COG),
    ]),
    ("chem_aqa:4.4.2", &[
        ("QlSsle_jSQ8", "Three Reactions of Acids", FSL),
        ("FOgZgMBbnXM", "Strong Acids & Weak Acids | pH & Concentration of Hydrogen Ions", COG),
        ("9GH95172Js8", "Required Practical 1: Making Soluble Salts", FSL),
    ]),
    ("chem_aqa:4.4.3", &[
        ("AhTRiL6xjBA", "Introducing Electrolysis", FSL),
        ("me19WmGg8Dw", "Electrolysis Part 3/3 - Aqueous Solutions", COG),
        ("ukbtTTG1Kew", "Required Practical 3: Electrolysis", FSL),
    ]),
    ("chem_aqa:4.5.1", &[
        ("hNNvIsQLSV8", "Exothermic & Endothermic Reactions - Reaction Profiles | Activation Energy", COG),
        ("eExCBkp4jB4", "Bond Energy Calculations 1", FSL),
        ("rdI7xEq4Ew8", "Required Practical 4: Temperature Changes", FSL),
    ]),
    ("chem_aqa:4.5.2", &[
        ("riikUBLFBJs", "Cells and Batteries", FSL),
        ("iJgMuDzkdkI", "Fuel Cells", FSL),
        ("NAYCRPyd4xU", "Fuel Cells - Structure | How they Work | Half Equations | Pros & Cons", COG),
    ]),
    ("chem_aqa:4.6.1", &[
        ("JK7yPzO9POU", "Factors Affecting Rate of Reaction & Collision Theory", COG),
        ("aanZzrUPEDw", "Measuring Rates of Reaction from Graphs - Mean Rate & Rate at a Specific Time", COG),
        ("N5p06i9ilmo", "Required Practical 5: Rates of Reaction", FSL),
    ]),
    ("chem_aqa:4.6.2", &[
        ("66qcNNJFy6E", "Reversible Reactions", FSL),
        ("5rC6f_P_A48", "Reversible Reactions & Dynamic Equilibrium", COG),
        ("KZPO23kia_o", "Le Chatelier's Principle - Position of Equilibrium in Reversible Reactions", COG),
    ]),
    ("chem_aqa:4.7.1", &[
        ("CX2IYWggEBc", "Crude oil and Hydrocarbons", FSL),
        ("-qxeVFJh4e4", "Fractional Distillation - Crude Oil | Hydrocarbons", COG),
        ("7AWwjKbRa_o", "Cracking", FSL),
    ]),
    ("chem_aqa:4.7.2", &[
        ("83Is-rouV-U", "Addition Reactions of Alkenes", COG),
        ("uFZasZ-hs_A", "Alcohols", FSL),
        ("ketAGS1gkQM", "Carboxylic Acids", FSL),
    ]),
    ("chem_aqa:4.7.3", &[
        ("1ZUg6ZC3ltA", "Addition Polymers & Polymerisation", COG),
        ("QBuSFPOtcJ4", "Condensation Polymers", FSL),
        ("7o27fhO_nm0", "Naturally Occurring Polymers - Polypeptides, DNA, and Carbohydrates", COG),
    ]),
    ("chem_aqa:4.8.1", &[
        ("3oJxWwcnfJY", "Purity and Formulations", FSL),
        ("TdJ57SQ6GAQ", "Paper Chromatography", COG),
        ("P8i4QYncQxI", "Required Practical 6: Chromatography", FSL),
    ]),
    ("chem_aqa:4.8.2", &[
        ("jVkGKurtaiE", "Testing for Gases", FSL),
        ("QH-7X5STCO4", "Testing for Gases - Hydrogen/Chlorine/Oxygen/CO2", COG),
    ]),
    ("chem_aqa:4.8.3", &[
        ("Bd0A44Iv2OI", "Flame Tests", FSL),
        ("1BCc_RrrSSw", "Flame Emission Spectroscopy (Flame Photometry)", COG),
        ("4iZRs4XlJOE", "Required Practical 7: Identifying Ions", FSL),
    ]),
    ("chem_aqa:4.9.1", &[
        ("t1Z3GlNldLA", "The Atmosphere", FSL),
        ("DnFcyhsSVGg", "Evolution of the Atmosphere | How the Atmosphere Changed Over Time", COG),
        ("KMK8Bo6XdSc", "The Early Atmosphere", "Primrose Kitten Academy | GCSE & A-Level Revision"),
    ]),
    ("chem_aqa:4.9.2", &[
        ("K5vXnDGcOE4", "The Greenhouse Effect", FSL),
        ("5ZzDxYY48HQ", "Greenhouse Gases & Climate Change", COG),
        ("ilfqxt90ED4", "Carbon Footprint", FSL),
    ]),
    ("chem_aqa:4.9.3", &[
        ("yLp6LOgPHmI", "Pollutants from Fuels", FSL),
        ("H0Z_R6GLo9A", "Air Pollution - How Pollutants are Made | Why They're Harmful", COG),
    ]),
    ("chem_aqa:4.10.1", &[
        ("td8WR_FOQc0", "Potable Water - Treating Freshwater | Desalinating Seawater", COG),
        ("n7pYRQs20bI", "Waste Water Treatment", FSL),
        ("b5RVPauf4oM", "Alternative Methods of Extracting Metals", FSL),
    ]),
    ("chem_aqa:4.10.2", &[
        ("Znnhe4BJH14", "Life-Cycle Assessment", FSL),
        ("-i_Eug83uzo", "Lifecycle Assessments (LCAs) - Stages | Examples | Limitations", COG),
        ("tg8tntiqQYk", "Recycling", FSL),
    ]),
    ("chem_aqa:4.10.3", &[
        ("q0CAfXV-YdY", "What is Corrosion and How to Stop it", COG),
        ("QRDctnk7sF4", "Alloys", FSL),
        ("q7VAq8t2gj0", "Ceramics and Composites", FSL),
    ]),
    ("chem_aqa:4.10.4", &[
        ("HAkaD6-7fgQ", "The Haber Process", FSL),
        ("1_HoWz5Kxfk", "The Haber Process Explained", COG),
        ("rKzt9BvvEeQ", "NPK Fertilisers", FSL),
    ]),
    // Biology (AQA GCSE Biology (8461) Higher)
    ("bio_aqa:4.1.1", &[
        ("qHkUOlC8Nbo", "Cell Types and Cell Structure", COG),
        ("bSbZpKV-MHE", "Required Practical 1: Microscopes", FSL),
        ("r7_nj3AZX6w", "Required Practical 2: Culturing Microorganisms", FSL),
    ]),
    ("bio_aqa:4.1.2", &[
        ("I0VdEiPWkHs", "Cell division by Mitosis", FSL),
        ("UsovRxQAUZ4", "Mitosis | Chromosomes | Cell Division | Cytokinesis", COG),
        ("Kh27eyjxvYM", "Stem Cells", FSL),
    ]),
    ("bio_aqa:4.1.3", &[
        ("C5pMigXBAgk", "Diffusion", FSL),
        ("BXTi5tbnOr0", "Active Transport", FSL),
        ("ef2Ts2AKhq8", "Required Practical 3: Effects of Osmosis on Plant Tissue", FSL),
    ]),
    ("bio_aqa:4.2.1", &[
        ("MB6mE6weCS4", "Levels of Organisation - Cells, Tissues, Organs and Organ Systems", COG),
        ("o5ei68M69g4", "AQA GCSE Biology: Principles of Organisation", STUDYMIND),
    ]),
    ("bio_aqa:4.2.2a", &[
        ("VLK2wANjQm0", "Digestive Enzymes", FSL),
        ("SqWTJWOBww4", "Required Practical 4: Food Tests", FSL),
        ("JyXXoevEWc8", "Required Practical 5: Effect of pH on Amylase", FSL),
    ]),
    ("bio_aqa:4.2.2b", &[
        ("bpYaKM2hVFY", "The Heart and Circulation", FSL),
        ("Wx-MrhlOFMk", "Arteries, Veins and Capillaries", FSL),
        ("aPUPfzsqDgs", "Gas Exchange in the Lungs", FSL),
    ]),
    ("bio_aqa:4.2.2c", &[
        ("5wSfCZESRHU", "Cardiovascular Diseases", FSL),
        ("H6DrSG_KQjo", "Lifestyle and Disease", FSL),
        ("BLITE8uA4Tw", "Cancer", FSL),
    ]),
    ("bio_aqa:4.2.3", &[
        ("2BR1zdMBhY4", "Plant Tissues", FSL),
        ("9yTDokLRZs0", "Transpiration", FSL),
        ("s06FvGH3QJo", "Transpiration & Translocation", COG),
    ]),
    ("bio_aqa:4.3.1a", &[
        ("K5zFxfbmC1M", "Measles and HIV", FSL),
        ("Ri-kEX_JI8o", "Salmonella and Gonorrhoea", FSL),
        ("_DfTkFzQCG8", "Communicable Disease | Protist & Fungal Disease", COG),
    ]),
    ("bio_aqa:4.3.1b", &[
        ("k1oCiR9Y2zQ", "Immune System & Defences | Types of White Blood Cell", COG),
        ("uPeZBhJYlnU", "Vaccination", FSL),
        ("w3ykU52K-Hw", "Drug Development and Testing - Clinical Trials", COG),
    ]),
    ("bio_aqa:4.3.2", &[
        ("cGNvdB3qNw4", "Monoclonal Antibodies", FSL),
        ("Bgde7orTxD8", "Uses of Monoclonal Antibodies", FSL),
        ("XrUW54Ea598", "Monoclonal Antibodies", COG),
    ]),
    ("bio_aqa:4.3.3", &[
        ("VHnS8HUtu44", "Infectious Diseases in Plants", FSL),
        ("02Mx7s8gIig", "Plant diseases 2", FSL),
        ("zL8JOw8Qzns", "Plant Defence Responses", FSL),
    ]),
    ("bio_aqa:4.4.1", &[
        ("rAJGnS_ktk4", "Photosynthesis", FSL),
        ("kx7AeCx_6xQ", "Limiting Factors", FSL),
        ("cBCKedXdFeE", "Required Practical 6: Photosynthesis", FSL),
    ]),
    ("bio_aqa:4.4.2", &[
        ("ZKAaDbTP6Dc", "Respiration", FSL),
        ("xO2XlIMnLuM", "Exercise", FSL),
        ("7ZQQUi2DPMw", "Metabolism", FSL),
    ]),
    ("bio_aqa:4.5.1", &[
        ("S45_3wWL-Xk", "Homeostasis", FSL),
        ("zvDwNd_aKsk", "Homeostasis | Receptors, Coordination Centres, Effectors | Negative Feedback", COG),
    ]),
    ("bio_aqa:4.5.2", &[
        ("oDS1hAqWp2M", "The Nervous System", FSL),
        ("Fm02i4vEi5Q", "Required Practical 7: Reaction Time", FSL),
        ("G_clJP1VGtk", "The Eye", FSL),
    ]),
    ("bio_aqa:4.5.3a", &[
        ("c6olhi88KZs", "The Endocrine System", FSL),
        ("77oyUdNZ054", "Control of Blood Glucose Concentration", FSL),
        ("kmRh_yRbAR4", "Maintaining the Body's Water Balance", FSL),
    ]),
    ("bio_aqa:4.5.3b", &[
        ("iXswGsfeHJg", "The Menstrual Cycle", FSL),
        ("5rsdXadNj-E", "Contraception", FSL),
        ("_Mts354VC7A", "Negative Feedback", FSL),
    ]),
    ("bio_aqa:4.5.4", &[
        ("_Bf5WKEMB5o", "Plant Hormones", FSL),
        ("fEo21LbnJJM", "Required Practical 8: Plant Responses", FSL),
        ("6boD9x0MMcs", "Uses of Plant Hormones", FSL),
    ]),
    ("bio_aqa:4.6.1a", &[
        ("Fh9b6a-3DLQ", "Sexual and Asexual Reproduction", FSL),
        ("w5SRMZlYR4w", "Meiosis and Fertilisation", FSL),
        ("TQ_iCf8mzMA", "DNA and the Genome", FSL),
    ]),
    ("bio_aqa:4.6.1b", &[
        ("reVLRjZIh3c", "Alleles", FSL),
        ("3XhzsN8TpTI", "Inherited Disorders | Polydactyly | Cystic Fibrosis", COG),
        ("Lomr_t5Pdjs", "Inheritance of Sex", FSL),
    ]),
    ("bio_aqa:4.6.2", &[
        ("7RraYCKvTXc", "Evolution by Natural Selection", FSL),
        ("99nEQd2k6k4", "Selective Breeding", FSL),
        ("gu9T91GJXDo", "Genetic Engineering", FSL),
    ]),
    ("bio_aqa:4.6.3", &[
        ("2waYa0ZwoXg", "Darwin and Natural Selection", FSL),
        ("n3cXcDEveRc", "Mendel and Genetics", FSL),
        ("sqBZRQirPM4", "Speciation", FSL),
    ]),
    ("bio_aqa:4.6.4", &[
        ("S2O6sVcUtLU", "Classification", FSL),
        ("qRqa454ZIro", "Classification Systems | Linnaean System", COG),
    ]),
    ("bio_aqa:4.7.1", &[
        ("ePsjdKoSA9g", "Competition and Interdependence", FSL),
        ("kIfMwZU8nk4", "Biotic and Abiotic Factors", FSL),
        ("KvK7EJimAH8", "Adaptations", FSL),
    ]),
    ("bio_aqa:4.7.2", &[
        ("yLHz2Ea10Mg", "Required Practical 9: Sampling Organisms", FSL),
        ("wSNSGcOqb38", "Carbon Cycle & Water Cycle", COG),
        ("LeH5FUKSPzU", "Required Practical 10: Decay", FSL),
    ]),
    ("bio_aqa:4.7.3", &[
        ("I5UR9uMeWuQ", "Biodiversity", FSL),
        ("cickRHnPKvc", "How Humans Reduce Biodiversity", COG),
        ("d3m73r64N2I", "Maintaining Biodiversity", FSL),
    ]),
    ("bio_aqa:4.7.4", &[
        ("AFC5LQ3KnvU", "Trophic Levels", FSL),
        ("sgh1OWm0oTQ", "Pyramids of Biomass", FSL),
        ("WqjZFDsKQDo", "Transfer of Biomass | Energy Flow in Food Chains", LWAS),
    ]),
    ("bio_aqa:4.7.5", &[
        ("_ACn3e4qnaM", "Food Security", FSL),
        ("nrbJl3R4YJU", "Modern Farming Methods", FSL),
        ("u59Eg1uNr5g", "Sustainable Fisheries", FSL),
    ]),
    // Physics (AQA GCSE Physics (8463) Higher)
    ("phys_aqa:4.1.1", &[
        ("JGwcDCeYRYo", "Energy Stores, Transferring Energy & Work Done", COG),
        ("-zy9eWzmGe4", "Kinetic Energy", FSL),
        ("63OTIdNb-TE", "Gravitational Potential Energy", FSL),
    ]),
    ("phys_aqa:4.1.2", &[
        ("ROBkMqJQLr4", "Conservation of Energy | Open & Closed Systems", COG),
        ("NI5jaeBrIgQ", "Efficiency", FSL),
        ("lLH45loyPUA", "Required Practical 2: Thermal Insulators", FSL),
    ]),
    ("phys_aqa:4.1.3", &[
        ("N5mHKqcit9I", "Energy Resources - Renewables & Non-renewables | Uses | Sources of Electricity", COG),
        ("1dJKvxhGEgA", "Energy from Fossil Fuels", FSL),
        ("pqzvUur7QRw", "Renewable Sources of Energy", FSL),
    ]),
    ("phys_aqa:4.2.1", &[
        ("BbizKa6eywo", "Voltage, Current & Resistance | V = IR Equation | IV Graphs", COG),
        ("YsZeZotYVag", "Required Practical 3: Resistance", FSL),
        ("A1SyKvdHoqY", "Required Practical 4: Current / PD Characteristics", FSL),
    ]),
    ("phys_aqa:4.2.2", &[
        ("vJRXozSVTI8", "Resistors in Series and Parallel", FSL),
        ("Hk4JEB5DITw", "Series Circuits", COG),
        ("GIvE5Zlpea8", "Parallel Circuits", COG),
    ]),
    ("phys_aqa:4.2.3", &[
        ("fbu3o9wavHk", "Mains Electricity", FSL),
        ("tb8mW-FiexA", "Plugs & Wires - Earth Wire | Neutral Wire | Live Wire", COG),
        ("B1XiX3sNV-c", "Alternating Current & Direct Current | AC & DC", COG),
    ]),
    ("phys_aqa:4.2.4", &[
        ("LOyJdI41aCU", "Power of Components", FSL),
        ("WLaUmNr4lho", "Calculating Energy Transferred by Appliances", FSL),
        ("u3isbsyDtL0", "National Grid - How we Generate & Transmit Electricity | Transformers", COG),
    ]),
    ("phys_aqa:4.2.5", &[
        ("5obbfXg_MH4", "Static Electricity", FSL),
        ("d7_dMHwqpgM", "Static Electricity | Build-up of Charge on Insulating Materials", COG),
        ("_lP3Nd_JLLU", "Electric Fields - Field Lines | Ionisation of Air", COG),
    ]),
    ("phys_aqa:4.3.1", &[
        ("-EZmXVOSa20", "Density", FSL),
        ("ScXOp8Zph28", "Required Practical 5: Density", FSL),
        ("zjkBMk5d3tM", "Particle Theory & States of Matter | Solids, Liquids & Gases", COG),
    ]),
    ("phys_aqa:4.3.2", &[
        ("LUHGFLR2p8k", "Specific Heat Capacity | Internal Energy & Temperature", COG),
        ("VFwf1JldiJA", "Specific Latent Heat - Vaporisation & Fusion | Equation", COG),
        ("HAPmwu7byGM", "Required Practical 1: Specific Heat Capacity", FSL),
    ]),
    ("phys_aqa:4.3.3", &[
        ("eHizt31t1rs", "Factors Affecting Gas Pressure | How Gas Particles Create Pressure", COG),
        ("RuoZqmNiMEo", "Pressure in Gases", FSL),
        ("m19-8Vtewkw", "Work Done on a Gas", FSL),
    ]),
    ("phys_aqa:4.4.1", &[
        ("Q8y4x5EElm8", "Development of the model of the atom", COG),
        ("0ASldDQmIOQ", "Alpha-Scattering and the Nuclear Model", FSL),
        ("k8cLFDa8zmY", "Atomic and Mass Numbers", FSL),
    ]),
    ("phys_aqa:4.4.2", &[
        ("F_Y1-JieCrg", "Radioactivity", FSL),
        ("xpSBhUpBXic", "Nuclear Equations", FSL),
        ("wvgT52mwM3Y", "Radioactive Decay & Half-life | How to Calculate Activity & Half-life", COG),
    ]),
    ("phys_aqa:4.4.3", &[
        ("teGu0VAPlOo", "Irradiation and Contamination", FSL),
        ("Z7394DMkfQs", "Background Radiation", FSL),
        ("qlvKhMJibPY", "Using Radiation in Medicine - Radiotherapy | Medical Tracers", COG),
    ]),
    ("phys_aqa:4.4.4", &[
        ("onkW8BF5I3Q", "Nuclear Fission and Nuclear Fusion", FSL),
        ("JQEZaJX7NVQ", "Nuclear Fission - How it Works | Nuclear Reactors | Pros & Cons", COG),
        ("Elv4bsa7TIA", "Nuclear Fusion - How it Works | Pros & Cons", COG),
    ]),
    ("phys_aqa:4.5.1", &[
        ("SYU1TKQru1c", "Free Body Diagrams & How to Calculate Resultant Forces", COG),
        ("W2aBVbcHr_k", "Gravity and Weight", FSL),
        ("8RI2_gJy0L0", "Resolving Forces", FSL),
    ]),
    ("phys_aqa:4.5.2", &[
        ("JHEmPZ-YnrU", "Work Done by a Force", FSL),
        ("Pf5EHVxc4XI", "Power & Work Done - Equations | Calculations", COG),
        ("PY80j_iNT9Y", "Work done and Energy Transfer", FSL),
    ]),
    ("phys_aqa:4.5.3", &[
        ("ACDbJ8rsQDo", "Forces and Elasticity", FSL),
        ("abWi5y6bJ9k", "Elasticity, Spring Constant & Hooke's Law | Force Extension Graphs", COG),
        ("jQAt3e6Bz7U", "Required Practical 6: Stretching a Spring", FSL),
    ]),
    ("phys_aqa:4.5.4", &[
        ("0RXm47J196Q", "Moments", FSL),
        ("uwzhXL0vp1w", "Levers and Gears", FSL),
        ("pqbPcLTz-tE", "Moments - Seesaws & Gears", COG),
    ]),
    ("phys_aqa:4.5.5", &[
        ("P08-lYPy1hI", "Pressure in Fluids", FSL),
        ("OIpvOIb3GAg", "Liquid Pressure & Upthrust - Calculating Liquid Pressure | How it Varies With Depth", COG),
        ("s8C2RktZtbM", "Atmospheric Pressure", COG),
    ]),
    ("phys_aqa:4.5.6a", &[
        ("NZvMXcIztuU", "Speed, Velocity, Distance & Displacement | Scalar & Vector Quantities", COG),
        ("DkCw2C-DkT0", "Distance-Time Graphs", FSL),
        ("VJefeYJL3uE", "Velocity-Time Graphs - How to Find Acceleration & Distance Travelled", COG),
    ]),
    ("phys_aqa:4.5.6b", &[
        ("D8XOm4swxOU", "Newton's 1st & 2nd Laws | Circular Motion | Inertia", COG),
        ("VOMNGlasL-0", "Required Practical 7: Acceleration", FSL),
        ("aVy_gNVaCGg", "Forces Acting on a Skydiver", FSL),
    ]),
    ("phys_aqa:4.5.6c", &[
        ("drMKdcMq3o0", "Vehicle Stopping Distance", FSL),
        ("q0yjYZdTS3I", "Stopping Distances - Thinking Distance | Braking Distance", COG),
        ("AiXhR2eZxgo", "Force and Braking", FSL),
    ]),
    ("phys_aqa:4.5.7", &[
        ("ZtQhlwPxE28", "Momentum", FSL),
        ("mjeOuh0G-gw", "Momentum (Part 1 of 2) | Conservation of Momentum", COG),
        ("KjFA2Zvdn6I", "Momentum (Part 2 of 2) - Changes in Momentum", COG),
    ]),
    ("phys_aqa:4.6.1", &[
        ("1DFAy8MXkMA", "Longitudinal & Transverse Waves - Labelling & Calculating Wave Speed", COG),
        ("UNmv6H-f180", "Required Practical 8: Ripple Tank", FSL),
        ("2fN_jvf4fw8", "Required practical 9: Reflection and Refraction", FSL),
    ]),
    ("phys_aqa:4.6.2", &[
        ("zx0UIMA2-bA", "Electromagnetic Waves - Frequency | Wavelength | Where They Come From", COG),
        ("L0iivb-acqU", "Uses of EM waves", FSL),
        ("0O3NdG0uWM4", "How Lenses Work - Convex & Concave Lenses | Real & Virtual Images", COG),
    ]),
    ("phys_aqa:4.6.3", &[
        ("SvVQAg_oA2A", "Black Body Radiation", FSL),
        ("je-qc7sxYzU", "How Radiation Affects Temperature", COG),
        ("eE7OPL7pesA", "Required Practical 10: Infrared", FSL),
    ]),
    ("phys_aqa:4.7.1", &[
        ("FodEDHaEY68", "Magnetic Fields", FSL),
        ("bHLsHjRMAsU", "Permanent & Induced Magnets", COG),
        ("LTfP8yPAVFw", "Electromagnetism - Wires | Coils | Solenoids | Electromagnets", COG),
    ]),
    ("phys_aqa:4.7.2", &[
        ("GNLhSKZh-jM", "The Motor Effect", FSL),
        ("XWL8963-AuQ", "Motor Effect - How to Find Force Direction & Strength", COG),
        ("fiQ38p6vb8o", "The Electric Motor", FSL),
    ]),
    ("phys_aqa:4.7.3", &[
        ("NjgqJahwsG0", "The Generator Effect", FSL),
        ("UsZsns63Km4", "The Microphone", FSL),
        ("7RtBUEZbKmI", "How Transformers Work", COG),
    ]),
    ("phys_aqa:4.8.1", &[
        ("V69KZun35K8", "Life Cycle of Stars | How Stars are Formed & Destroyed", COG),
        ("mndRVjMovQk", "The Solar System", FSL),
        ("okMA18ppu98", "Orbital Motion", FSL),
    ]),
    ("phys_aqa:4.8.2", &[
        ("C90DOE87TYc", "Red-Shift", FSL),
        ("bWEtm-7cYzM", "What is Red Shift?", COG),
        ("QAa2O_8wBUQ", "What is Dark Matter and Dark Energy?", "Kurzgesagt – In a Nutshell"),
    ]),

    // ---------- English Literature (WJEC Eduqas C720QS) ----------
    ("englit_edq:C1Aa", &[
        ("4GSCWDa1qcE", "Shakespeare in Seven Minutes: Macbeth Summary", "Easy as GCSE"),
        ("L_P3Ub_jnAg", "GCSE English Literature | Key themes and characters in Macbeth | Bitesize | GCSE Revision", "BBC Bitesize - GCSE Revision Support"),
        ("DDH4ooBU7TA", "Macbeth Themes Revision: The Supernatural, Fate vs Free Will, Appearance vs Reality", "Easy as GCSE"),
    ]),
    ("englit_edq:C1Ab", &[
        ("gSNzOHF09oU", "Eduqas English Literature GCSE Macbeth Extract Question, Eduqas Literature Component 1", "Literature Daydreams"),
        ("_XCgmX7JaNo", "Macbeth Extract Question, Eduqas GCSE English Literature, GCSE English Literature Exam Tips", "Literature Daydreams"),
        ("rNVoF_gw2Pg", "Eduqas English Literature Component 1 - Macbeth extract analysis", "Mr Baugh"),
    ]),
    ("englit_edq:C1Ac", &[
        ("63TJccckY-8", "Macbeth Whole Play Question, Eduqas GCSE English Literature, GCSE Revision Tips", "Literature Daydreams"),
        ("XkSbI13FtFs", "118 Eduqas GCSE English Literature - exam technique (\"Macbeth\")", "Chalk in My Pockets"),
    ]),
    ("englit_edq:C1Ba", &[
        ("fmEsJXZ3Ulo", "116 The Schoolboy, William Blake", "Chalk in My Pockets"),
        ("-FRy0iltH8g", "'The Schoolboy' by William Blake GCSE English Analysis", "NHHS English Faculty Youtube"),
    ]),
    ("englit_edq:C1Bb", &[
        ("_4UFNa7vXJw", "115 I Wandered Lonely as a Cloud, William Wordsworth", "Chalk in My Pockets"),
        ("gqmEoMuFfxs", "'I Wandered Lonely as a Cloud' by William Wordsworth", "NHHS English Faculty Youtube"),
        ("hCU0cb6SpU0", "Eduqas Poetry Anthology 2027 - I Wandered as Lonely as a Cloud by William Wordsworth Revision Video", "Sophie"),
    ]),
    ("englit_edq:C1Bc", &[
        ("g7ZKivxo1HI", "125 'Cousin Kate', Christina Rossetti", "Chalk in My Pockets"),
        ("W6hY4pOof7w", "'Cousin Kate' by Christina Rossetti GCSE English Analysis", "NHHS English Faculty Youtube"),
        ("RPY8PTkFpAk", "Cousin Kate, by Christina Rossetti", "Mr Salles Teaches English"),
    ]),
    ("englit_edq:C1Bd", &[
        ("qu4PIz-wlk8", "119 Sonnet 29 - I think of thee!, Elizabeth Barrett Browning", "Chalk in My Pockets"),
        ("fq4PxFfiO7g", "'Sonnet 29' by Elizabeth Barrett Browning GCSE English Analysis", "NHHS English Faculty Youtube"),
        ("EpCV4ugLEog", "3 Things You Need to Know about 'Sonnet 29' by Elizabeth Barrett Browning", "Mr Bruff"),
    ]),
    ("englit_edq:C1Be", &[
        ("9JMxsYRa8bk", "111 Drummer Hodge, Thomas Hardy", "Chalk in My Pockets"),
        ("LyZcAR6xYxU", "'Drummer Hodge' by Thomas Hardy GCSE English Analysis", "NHHS English Faculty Youtube"),
    ]),
    ("englit_edq:C1Bf", &[
        ("X4IQnpUVsJA", "112 Disabled, Wilfred Owen", "Chalk in My Pockets"),
        ("jFe7jVhrSJU", "Analysis of 'Disabled' by Wilfred Owen", "Mrs Rumsey"),
    ]),
    ("englit_edq:C1Bg", &[
        ("xlkZ2imFid0", "117 I Shall Return, Claude McKay", "Chalk in My Pockets"),
        ("lQjfGgNvghU", "'I Shall Return' by Claude McKay GCSE English Analysis", "NHHS English Faculty Youtube"),
    ]),
    ("englit_edq:C1Bh", &[
        ("aLcsIABAVLs", "114 Decomposition, Zulfikar Ghose", "Chalk in My Pockets"),
        ("UXzjORqpTnA", "'Decomposition' by Zulfikar Ghose GCSE English Analysis", "NHHS English Faculty Youtube"),
    ]),
    ("englit_edq:C1Bi", &[
        ("MmIB0cWm3SQ", "127 'Catrin', Gillian Clarke", "Chalk in My Pockets"),
        ("W-9v1-YyvJQ", "'Catrin' by Gillian Clarke GCSE English Analysis", "NHHS English Faculty Youtube"),
    ]),
    ("englit_edq:C1Bj", &[
        ("W59iqkPZ0O4", "110 Blackberry Picking, Seamus Heaney", "Chalk in My Pockets"),
        ("dpAaaVqvCno", "'Blackberry Picking' by Seamus Heaney GCSE English Analysis", "NHHS English Faculty Youtube"),
    ]),
    ("englit_edq:C1Bk", &[
        ("AuhjiAluRtY", "120 Kamikaze, Beatrice Garland", "Chalk in My Pockets"),
        ("TD-F9d6DhJY", "'Kamikaze' by Beatrice Garland GCSE English Analysis", "NHHS English Faculty Youtube"),
        ("115XZNvCwlI", "'Kamikaze' in 4 Minutes: Quick Revision", "Mr Bruff"),
    ]),
    ("englit_edq:C1Bl", &[
        ("V-BBGGuALsA", "122 'War Photographer', Carol Ann Duffy", "Chalk in My Pockets"),
        ("vGZSyaeRD00", "'War Photographer' by Carol Ann Duffy GCSE English Analysis", "NHHS English Faculty Youtube"),
        ("MbJGwPjZ3ZM", "'War Photographer' in 5 Minutes: Quick Revision", "Mr Bruff"),
    ]),
    ("englit_edq:C1Bm", &[
        ("NLPG1DM82EY", "123 Dusting the Phone, Jackie Kay", "Chalk in My Pockets"),
        ("tBWP_7LcbiA", "‘Dusting the Phone’ by Jackie Kay GCSE English Analysis", "NHHS English Faculty Youtube"),
    ]),
    ("englit_edq:C1Bn", &[
        ("68tcjTxlzSs", "121 'Remains', Simon Armitage", "Chalk in My Pockets"),
        ("Ynoxz_BqXMQ", "'Remains' by Simon Armitage GCSE English Analysis", "NHHS English Faculty Youtube"),
        ("R4fLrVnp2jk", "'Remains' by Simon Armitage in 5 Minutes: Quick Revision", "Mr Bruff"),
    ]),
    ("englit_edq:C1Bo", &[
        ("dbGR_u-wrWQ", "126 'Origin Story', Eve L. Ewing", "Chalk in My Pockets"),
        ("7YLDngSBkfU", "'Origin Story' by Eve L Ewing GCSE English Analysis", "NHHS English Faculty Youtube"),
    ]),
    ("englit_edq:C1Bp", &[
        ("aH8M9kyAM0M", "Eduqas NEW GCSE Poetry Anthology - EVERY POEM ANALYSED", "GuigLit"),
    ]),
    ("englit_edq:C1Bq", &[
        ("JaONYCCgAZM", "128 Eduqas GCSE English Literature - exam technique (Poetry anthology)", "Chalk in My Pockets"),
        ("92352fy-rBA", "HOW TO COMPARE TWO POEMS OR TWO TEXTS IN A GCSE ESSAY: GRADE 9 MODEL ANSWER FRAMEWORK &  EXPLANATION", "First Rate Tutors"),
    ]),
    ("englit_edq:C2Aa", &[
        ("Dc7-wKFR5y8", "An Inspector Calls || 7 Minute Summary", "Easy as GCSE"),
        ("7Gzbj3Az598", "An Inspector Calls: Context, Themes & Quotes - Everything You Need to Know For The 2026 GCSE Exams", "First Rate Tutors"),
    ]),
    ("englit_edq:C2Ab", &[
        ("vhwiM-bFIz4", "Eduqas GCSE English Literature Revision: An Inspector Calls - How to Tackle the Exam", "Literature Daydreams"),
        ("JCapBWDO8h0", "113 Eduqas GCSE English Literature - exam technique (\"An Inspector Calls\")", "Chalk in My Pockets"),
    ]),
    ("englit_edq:C2Ba", &[
        ("8fzPJUtstn4", "A Christmas Carol || 7 Minute Summary", "Easy as GCSE"),
        ("_qzKxJr0kVM", "A Christmas Carol: Context, Themes & Quotes - Everything You Need to Know For The 2026 GCSE Exams", "First Rate Tutors"),
        ("v2SQ8WGl364", "Eduqas English Literature GCSE, Christmas Carol Revision", "Literature Daydreams"),
    ]),
    ("englit_edq:C2Bb", &[
        ("RXhqz5iZFxA", "A Christmas Carol: How to MASTER the Exam (EDUQAS)", "GuigLit"),
        ("BFw8vgNWEJU", "A Christmas Carol - LIVE GCSE Exam Walkthrough (EDUQAS)", "GuigLit"),
    ]),
    ("englit_edq:C2Ca", &[
        ("4bgRhm0rvuI", "Exam Success: How to Master GCSE Unseen Poetry (EDUQAS)", "GuigLit"),
        ("mjbyQrrdNlc", "GCSE UNSEEN POETRY - KEY ADVICE (AQA, EDUQAS & EDEXCEL)", "GuigLit"),
    ]),
    ("englit_edq:C2Cb", &[
        ("EUgrfFx8iG0", "124 Eduqas GCSE English Literature - exam technique (Comparing unseen poetry)", "Chalk in My Pockets"),
        ("r9d2yq2ErIg", "Top Tips for Tackling Unseen Poetry - GCSE English Literature Paper 2", "Miss Adams Teaches..."),
    ]),
    // ---------- Business (Pearson Edexcel GCSE 1BS0) - Sense Business Studies' code-numbered
    // Theme 1 series, Business As Usual's Edexcel topic recaps, Mr Cheeseman, Two Teachers,
    // Bizconsesh and others. Read off YouTube on 30 September 2026; every id, title and
    // channel checked against YouTube's oEmbed response. ----------
    ("bus_edx:1.1.1", &[
        ("Uwg9Fx24pjg", "1.1.1 The dynamic nature of business GCSE Business", "Sense Business Studies"),
        ("FTG_l2CA6P8", "Dynamic Nature of Business (Edexcel)", BIZC),
        ("OgyfbyWd6G8", "The Dynamic Nature of Business Explained", TWOT),
    ]),
    ("bus_edx:1.1.2", &[
        ("NyFjqnrTQso", "1.1.2 Risk and reward GCSE Business", "Sense Business Studies"),
        ("_O3fYhWX0ro", "Entrepreneur Risks & Rewards", BIZC),
    ]),
    ("bus_edx:1.1.3", &[
        ("jAKDU7eRju0", "1.1.3 The role of business enterprise GCSE Business", "Sense Business Studies"),
        ("C5RK_whu7iw", "Added Value | 5 ways a business can add value", TWOT),
        ("flPQfGLcDHE", "1.1 Enterprise and entrepreneurship in 11 minutes (Edexcel GCSE Business Recap)", "Business As Usual - GCSE and A Level Revision"),
    ]),
    ("bus_edx:1.2.1", &[
        ("_U8UKR2KuqU", "1.2.1 Customer needs GCSE Business Studies", "Sense Business Studies"),
        ("Bp9Cz3G5Rrw", "Edexcel GCSE Business - Understanding Customer Needs", "Business Boost with Mr Cheeseman"),
    ]),
    ("bus_edx:1.2.2", &[
        ("xsBNjQcbBX0", "1.2.2 Market research GCSE business studies", "Sense Business Studies"),
        ("NSavcu-cIs0", "Market Research | The Purpose of Market Research Explained | Lego, Gymshark & Apple Examples", TWOT),
        ("rjjNjS-a_dE", "1.2 Spotting an opportunity in 18 minutes (Edexcel GCSE Business Recap)", "Business As Usual - GCSE and A Level Revision"),
    ]),
    ("bus_edx:1.2.3", &[
        ("BrhZ2Dc8sF8", "1.2.3 Market segmentation GCSE Business Studies", "Sense Business Studies"),
        ("HbeVCqrBaIg", "Edexcel GCSE Business - Market Mapping", "Business Boost with Mr Cheeseman"),
        ("ZUOCx1iO8gI", "Market Segmentation | How Gymshark use Market Segmentation Explained.", TWOT),
    ]),
    ("bus_edx:1.2.4", &[
        ("-98NBv0xvPc", "1.2.4 The competitive environment GCSE Business Studies", "Sense Business Studies"),
        ("pzwwpurAHR0", "Competitive Environment", BIZC),
    ]),
    ("bus_edx:1.3.1", &[
        ("YbCx-e3O7pU", "1.3.1 Business aims and objectives GCSE Business Studies", "Sense Business Studies"),
        ("OzWTEe4bna4", "Business Aims and Objectives Explained", TWOT),
    ]),
    ("bus_edx:1.3.2a", &[
        ("yyTmZ3XUs6s", "1.3.2 Business revenues, costs and profits GCSE Business", "Sense Business Studies"),
        ("iNflEO5ar5A", "Fixed, Variable & Total Costs | Business Costs", TWOT),
        ("aCNV_RzoU5c", "Paper 1 & Theme 1 ALL Calculations! (Edexcel GCSE Business)", "Business As Usual - GCSE and A Level Revision"),
    ]),
    ("bus_edx:1.3.2b", &[
        ("7RocdGhTd6Q", "Break-Even Analysis | How to Calculate the Break-Even Point Explained.", TWOT),
        ("kG3SvTFhhE4", "GCSE Business Studies - Edexcel - Breakeven (Course Content)", "Revise Enterprise"),
        ("6m7f7LG1OpU", "Interpreting A Break Even Analysis - GCSE Business Studies Revision - OCR, Edexcel, AQA - Break-Even", "BizzWizard"),
    ]),
    ("bus_edx:1.3.3", &[
        ("rsTFEUsdrvY", "1.3.3 Cash and cash-flow GCSE Business Studies", "Sense Business Studies"),
        ("4SNWA_HbF6U", "Cash Flow Forecasting Explained | How to Complete a Cash Flow Forecast Example", TWOT),
        ("7nL4KSl6Fk8", "What is the difference between cash flow and profit?", BIZC),
    ]),
    ("bus_edx:1.3.4", &[
        ("a5KFtmouLw0", "1.3.4 Sources of business finance GCSE Business Studies", "Sense Business Studies"),
        ("DAZi6XcTZzE", "Sources of Business Finance Explained | Bank Loans, Trade Credit, Share Capital, Overdrafts & More", TWOT),
    ]),
    ("bus_edx:1.4.1", &[
        ("VsACCR4WYuc", "1.4.1 The options for start up and small businesses", "Sense Business Studies"),
        ("XlsZDWlJM1c", "Edexcel GCSE Business - Limited Liability", "Business Boost with Mr Cheeseman"),
        ("Yn28I9uGWUw", "Edexcel GCSE Business | Forms of Business: Franchising", "tutor2u"),
    ]),
    ("bus_edx:1.4.2", &[
        ("m03PqIVXkkk", "1.4.2 Business location GCSE Business Studies", "Sense Business Studies"),
        ("eU2VMJ2d1ks", "Factors Influencing Business Location Explained", TWOT),
    ]),
    ("bus_edx:1.4.3", &[
        ("KjGvqCQuQg4", "1.4.3 The marketing mix GCSE Business Studies - Part One", "Sense Business Studies"),
        ("fD6Z6MeNt9I", "1.4.3 The marketing mix GCSE Business Studies - Part Two", "Sense Business Studies"),
        ("N8kgSSbycUA", "Edexcel GCSE Business - Marketing MIx", "Business Boost with Mr Cheeseman"),
    ]),
    ("bus_edx:1.4.4", &[
        ("HLk2U-D2gXI", "1.4.4 Business plans GCSE Business Studies", "Sense Business Studies"),
        ("O0lXFwG5o3w", "Business Plans", BIZC),
    ]),
    ("bus_edx:1.5.1", &[
        ("E373s0azagg", "1.5.1 Business stakeholders GCSE business studies", "Sense Business Studies"),
        ("tZGol4xtY3g", "Stakeholders | What is a Stakeholder?", TWOT),
    ]),
    ("bus_edx:1.5.2", &[
        ("Pvfr7xfU808", "Technology and Business | Different Types of Technology Used by Business", TWOT),
        ("8pjlKDf8w2Y", "Payment Systems Explained  - GCSE Business Studies Revision - OCR, Edexcel, AQA - BizzWizard", "BizzWizard"),
        ("cQmgL1_l8hc", "1.5 Understanding external influences in 23 minutes (Edexcel GCSE Business Recap)", "Business As Usual - GCSE and A Level Revision"),
    ]),
    ("bus_edx:1.5.3", &[
        ("gNpaWXTP7Jc", "Employment Law | The 4 Key Principles Explained", TWOT),
        ("eaqQX5BU5L4", "Consumer Rights | What consumer rights do I have?", TWOT),
        ("jZWzzqv6CHo", "The Impact of Legislation on Businesses | Legislation & Business", TWOT),
    ]),
    ("bus_edx:1.5.4", &[
        ("Og2HQ1Bv65s", "Economy and Business | How the Economic Climate Impacts Businesses Explained!", TWOT),
        ("b2P6XwYypk4", "Impact of Interest Rate Changes - GCSE & A Level Business", BIZC),
        ("D2G51WsQNn4", "Exchange Rate Impacts", BIZC),
    ]),
    ("bus_edx:1.5.5", &[
        ("cQmgL1_l8hc", "1.5 Understanding external influences in 23 minutes (Edexcel GCSE Business Recap)", "Business As Usual - GCSE and A Level Revision"),
        ("NWgvdhlzloc", "Understanding External Influences on Businesses - GCSE Business Studies Revision- OCR, Edexcel, AQA", "BizzWizard"),
    ]),
    ("bus_edx:2.1.1", &[
        ("pPU2_FYNBRw", "2 1 Growing the Business in 24 minutes (Edexcel GCSE Business Recap)", "Business As Usual - GCSE and A Level Revision"),
        ("PuFtnYszmEM", "Organic Business Growth | How to Grow Internally", TWOT),
        ("EoA23SSwEGI", "Inorganic Business Growth | Mergers & Takeovers", TWOT),
    ]),
    ("bus_edx:2.1.2", &[
        ("cPeUX5qmU3Y", "Why Business Aims & Objectives Change | Sainsbury's Examples", TWOT),
        ("Y43vxczMqFM", "Changing Aims & Objectives", BIZC),
    ]),
    ("bus_edx:2.1.3", &[
        ("XwsbL4GF2RA", "What is Globalisation?", TWOT),
        ("H4tMzsxcJk0", "Barriers to International Trade | Tariffs & Trading Blocs", TWOT),
        ("-loRR8XBeDw", "Globalisation - GCSE Business & A Level Business", BIZC),
    ]),
    ("bus_edx:2.1.4", &[
        ("A9i8dwKC7TE", "Business Ethics | The Impact of Ethics on Business", TWOT),
        ("aS3_tauT_WE", "Business Ethics", BIZC),
        ("m2eGMnAskLI", "Businesses & The Environment - GCSE Business Studies - OCR, Edexcel, AQA - Environmental Impact", "BizzWizard"),
    ]),
    ("bus_edx:2.2.1", &[
        ("LRr9__P_5Zc", "The Design Mix Explained", TWOT),
        ("ob5KWs3I3aY", "Product Life Cycle Explained | Apple iPhone & Coca Cola Examples", TWOT),
        ("DJQsOWshYeA", "What is Product Differentiation | McDonald's Example", TWOT),
    ]),
    ("bus_edx:2.2.2", &[
        ("xTw7JUxWsYM", "Pricing Strategies Explained", TWOT),
        ("J0TV5pBbAm8", "2.2 Making Marketing Decisions in 23 minutes (Edexcel GCSE Business Recap)", "Business As Usual - GCSE and A Level Revision"),
    ]),
    ("bus_edx:2.2.3", &[
        ("XI2qK1UASxI", "The Promotional Mix Explained | McDonald's Examples", TWOT),
        ("uDgWxo2lNjM", "Promotional Methods", BIZC),
    ]),
    ("bus_edx:2.2.4", &[
        ("MpKKM0ElCZA", "Distribution Channels Explained", TWOT),
        ("C5KkwKxCuRU", "The Marketing Mix: Place - Digital Distribution - GCSE Business Studies Revision - OCR, Edexcel, AQA", "BizzWizard"),
    ]),
    ("bus_edx:2.2.5", &[
        ("J0TV5pBbAm8", "2.2 Making Marketing Decisions in 23 minutes (Edexcel GCSE Business Recap)", "Business As Usual - GCSE and A Level Revision"),
        ("tAwN1SlkrAg", "Impact of Competition on the Marketing Mix - GCSE Business & A Level Business", BIZC),
        ("_f65-t9vjuI", "Marketing Mix Analysis", BIZC),
    ]),
    ("bus_edx:2.3.1", &[
        ("D4fh1WD2Y0s", "2.3 Making Operations Decisions in 26 minutes (Edexcel GCSE Business Recap)", "Business As Usual - GCSE and A Level Revision"),
        ("ueRb14-kVEw", "How Businesses use Job Production to Meet Customer Needs and Increase Profit Margins", TWOT),
        ("cryNtuWJm3E", "Why Batch Production is used by Nike and Other Businesses", TWOT),
        ("Qw0hueAiUes", "Why Mass Production is used by Tesla and Apple", TWOT),
    ]),
    ("bus_edx:2.3.2", &[
        ("P-X1B0N6P7k", "Managing Stock Explained | Bar Gate Stock Graphs, JIT, Procurement & Logistics", TWOT),
        ("nh6QXGfvJuA", "GCSE Edexcel Business Studies - Theme 1.3 - Bar Gate Stock Graphs (Course Content)", "Revise Enterprise"),
        ("FMidebp7kaA", "Just in Time - JIT - Pros and Cons", BIZC),
    ]),
    ("bus_edx:2.3.3", &[
        ("lf74Oc-D1zE", "Managing Quality Explained | Quality Control & Quality Assurance", TWOT),
        ("oi1cARLWI5c", "Quality Control vs. Quality Assurance - QC vs. QA", BIZC),
    ]),
    ("bus_edx:2.3.4", &[
        ("-TA5oQv6uxc", "The Sales Process Explained | Apple Examples", TWOT),
        ("DHukPz033Hc", "What is Customer Service?", BIZC),
    ]),
    ("bus_edx:2.4.1", &[
        ("DM7TqljUues", "Net Profit and Gross Profit | Formulas, Margin Calculations and How to Interpret Figures Explained", TWOT),
        ("NeYLPo5hqTI", "Average Rate of Return Explained | How to Calculate the ARR Method of Investment Appraisal", TWOT),
        ("qkjj2R-niM8", "Paper 2 & Theme 2 ALL Calculations! (Edexcel GCSE Business)", "Business As Usual - GCSE and A Level Revision"),
    ]),
    ("bus_edx:2.4.2", &[
        ("jE2-eijnL_k", "Understanding Business Performance - GCSE Business", BIZC),
        ("120SqXOUHU8", "2.4 Making Finance Decisions in 15 minutes (Edexcel GCSE Business Recap)", "Business As Usual - GCSE and A Level Revision"),
    ]),
    ("bus_edx:2.5.1", &[
        ("LCAAivdxVTU", "Organisational Structures Explained", TWOT),
        ("w_V6MqPFLsI", "The Importance of Effective Communication in Business", TWOT),
        ("ZAS9KYJjXCg", "Different Ways of Working Explained | Employment Contracts & Work Arrangements", TWOT),
    ]),
    ("bus_edx:2.5.2", &[
        ("4aTvJT-qTYc", "Recruitment Documents Explained | Person Specification, Job Description, Application Form and CV", TWOT),
        ("FuKQf5JQIXg", "Internal VS External Recruitment", TWOT),
    ]),
    ("bus_edx:2.5.3", &[
        ("tdOIK1pqeKY", "Formal & Informal Training - GCSE Business", BIZC),
        ("0OGXd_LFcbQ", "2.5 Making Human Resources Decisions in 27 minutes (Edexcel GCSE Business Recap)", "Business As Usual - GCSE and A Level Revision"),
    ]),
    ("bus_edx:2.5.4", &[
        ("a-IiJT_2j84", "The Importance of Motivation in the Workplace | 3 Key Benefits Explained", TWOT),
        ("XtnH0nPRcxw", "Financial & Non-Financial Methods of Motivation", BIZC),
        ("YQ2uhzmqJDY", "Non-Financial Motivation | Top 10 Methods of Non-Financial Motivation for Employees", TWOT),
    ]),
// - Computer Science (AQA 8525): Craig'n'Dave's AQA GCSE (8525) series, with
//   The Computer Science Tutor's AQA units alongside.
    // ---------- Computer Science (AQA GCSE 8525) - Craig'n'Dave's AQA GCSE (8525) series, with The Computer Science Tutor ----------
    ("cs_aqa:3.1.1a", &[
        ("KAPi2FgjR3I", "33. AQA GCSE (8525) SLR7 - 3.1 What’s an algorithm", CND),
        ("ZpPgB6F9gOg", "34. AQA GCSE (8525) SLR7 - 3.1 Decomposition", CND),
        ("ByICvJvoOUk", "35. AQA GCSE (8525) SLR7 - 3.1 Abstraction", CND),
        ("U97x_OkoLPw", "37. AQA GCSE (8525) SLR7 - 3.1 How to produce algorithms", CND),
        ("b6sF3dluzEk", "GCSE Computer Science AQA Unit 1 Flowcharts", CSTUTOR),
    ]),
    ("cs_aqa:3.1.1b", &[
        ("4MrSVeOb1z4", "38. AQA GCSE (8525) SLR7 - 3.1 Algorithm inputs, processes & outputs", CND),
        ("9xQnIHONwG8", "39. AQA GCSE (8525) SLR7 - 3.1 Tracing algorithms", CND),
    ]),
    ("cs_aqa:3.1.2", &[
        ("9B6QsadamfU", "40. AQA GCSE (8525) SLR7 - 3.1 Algorithm efficiency", CND),
    ]),
    ("cs_aqa:3.1.3", &[
        ("0HXBo--PaAw", "41. AQA GCSE (8525) SLR7 - 3.1 Linear search", CND),
        ("Sa-4CObj8Ag", "42. AQA GCSE (8525) SLR7 - 3.1 Binary search", CND),
    ]),
    ("cs_aqa:3.1.4", &[
        ("KTWDjuGbuVY", "43. AQA GCSE (8525) SLR7 - 3.1 Merge sort", CND),
        ("vy_domkFPxw", "44. AQA GCSE (8525) SLR7 - 3.1 Bubble sort", CND),
    ]),
    ("cs_aqa:3.2.1", &[
        ("nDKCG1G-6CE", "45. AQA GCSE (8525) SLR8 - 3.2 Data types & casting", CND),
    ]),
    ("cs_aqa:3.2.2a", &[
        ("zDduIm2c6aA", "46. AQA GCSE (8525) SLR8 - 3.2 Variables & constants", CND),
        ("JIOmpA-UHk4", "49. AQA GCSE (8525) SLR8 - 3.2 The 3 basic programming constructs", CND),
        ("Od9VqugK0ak", "50. AQA GCSE (8525) SLR8 - 3.2 Meaningful identifiers", CND),
    ]),
    ("cs_aqa:3.2.2b", &[
        ("JIOmpA-UHk4", "49. AQA GCSE (8525) SLR8 - 3.2 The 3 basic programming constructs", CND),
        ("p-3ANzXJi0E", "GCSE Computer Science AQA  Programming Help   Totalling and Counting", CSTUTOR),
    ]),
    ("cs_aqa:3.2.3", &[
        ("NfBUZssd19M", "51. AQA GCSE (8525) SLR8 - 3.2 Arithmetic operators", CND),
        ("5WsgaY9AELY", "AQA GCSE Computer Science Exam - Operators - Arithmetic, Boolean and Relational", "lil anonymous"),
    ]),
    ("cs_aqa:3.2.4", &[
        ("5WsgaY9AELY", "AQA GCSE Computer Science Exam - Operators - Arithmetic, Boolean and Relational", "lil anonymous"),
    ]),
    ("cs_aqa:3.2.5", &[
        ("eRYOzp4yKQU", "52. AQA GCSE (8525) SLR8 - 3.2 Boolean operators", CND),
        ("5WsgaY9AELY", "AQA GCSE Computer Science Exam - Operators - Arithmetic, Boolean and Relational", "lil anonymous"),
    ]),
    ("cs_aqa:3.2.6", &[
        ("sb3pSoJO-uM", "53. AQA GCSE (8525) SLR9 - 3.2 Data structures & arrays", CND),
        ("X6wTroS-oqA", "54. AQA GCSE (8525) SLR9 - 3.2 Records", CND),
        ("83B5lVA8pGE", "GCSE Computer Science AQA  Programming Help  Arrays   Outputing and inputting", CSTUTOR),
    ]),
    ("cs_aqa:3.2.7", &[
        ("ZQJZETgIsf0", "47. AQA GCSE (8525) SLR8 - 3.2 User input & display output", CND),
    ]),
    ("cs_aqa:3.2.8", &[
        ("GaGgkoPBeHI", "55. AQA GCSE (8525) SLR9 - 3.2 String manipulation", CND),
    ]),
    ("cs_aqa:3.2.9", &[
        ("dywD5MN1EmI", "56. AQA GCSE (8525) SLR9 - 3.2 Random number generation", CND),
    ]),
    ("cs_aqa:3.2.10", &[
        ("aSU3s_Dfw9I", "48. AQA GCSE (8525) SLR8 - 3.2 Introduction to subroutines", CND),
        ("kXdb9UeJUm8", "57. AQA GCSE (8525) SLR9 - 3.2  How to use subroutines", CND),
        ("P4STQn_8_nc", "58. AQA GCSE (8525) SLR9 - 3.2 Local variables", CND),
        ("tb0tnfm3miY", "59. AQA GCSE (8525) SLR9 - 3.2 Structured programming", CND),
    ]),
    ("cs_aqa:3.2.11a", &[
        ("MW0TyBbDfXg", "60. AQA GCSE (8525) SLR10 - 3.2 Data validation", CND),
        ("PYhf90NpbQE", "61. AQA GCSE (8525) SLR10 - 3.2 Authentication routines", CND),
    ]),
    ("cs_aqa:3.2.11b", &[
        ("lc1Ewe2KpY8", "62. AQA GCSE (8525) SLR10 - 3.2 Test data", CND),
        ("baC_PXrfbaI", "63. AQA GCSE (8525) SLR10 - 3.2 Types of errors", CND),
        ("egEoe11v3hg", "64. AQA GCSE (8525) SLR10 - 3.2 Identifying syntax and logic errors", CND),
    ]),
    ("cs_aqa:3.3.1", &[
        ("2u9qxydMWUs", "72. AQA GCSE (8525) SLR13 - 3.3 Number bases", CND),
        ("lx07YDAso_o", "73. AQA GCSE (8525) SLR13 - 3.3 Bit patterns", CND),
        ("Q7zCoFcKLoI", "74. AQA GCSE (8525) SLR13 - 3.3 Why hexadecimal is used", CND),
    ]),
    ("cs_aqa:3.3.2", &[
        ("yE3frJ6WNJ0", "75. AQA GCSE (8525) SLR13 - 3.3 Converting between decimal & binary", CND),
        ("CNsol7wk6A0", "76. AQA GCSE (8525) SLR13 - 3.3 Converting between decimal & hex", CND),
        ("XN1xi5_YK3Y", "GCSE Computer Science AQA Data representation topic Hexadecimal", CSTUTOR),
    ]),
    ("cs_aqa:3.3.3", &[
        ("r7wXMEHA1eQ", "77. AQA GCSE (8525) SLR13 - 3.3 Units", CND),
        ("Zr6ouiksxoU", "GCSE Computer Science AQA Unit 3 units and binary", CSTUTOR),
        ("cFmvf4vyB_g", "Units and Numbers - AQA GCSE Computer Science", "KnowItAll Ninja"),
    ]),
    ("cs_aqa:3.3.4", &[
        ("8E7cGcqg1zc", "78. AQA GCSE (8525) SLR13 - 3.3 8-bit binary addition", CND),
        ("EL-OXz03Lhk", "79. AQA GCSE (8525) SLR13 - 3.3 Binary shifts", CND),
        ("3pRL4r-0Nrs", "GCSE Computer Science AQA unit 3   Binary Addition and Binary Shifts", CSTUTOR),
    ]),
    ("cs_aqa:3.3.5", &[
        ("XSMVSqNa8Mg", "80. AQA GCSE (8525) SLR13 - 3.3 Characters", CND),
        ("Mxve5zLXn7s", "GCSE Computer Science AQA Unit 3 Character Sets", CSTUTOR),
    ]),
    ("cs_aqa:3.3.6", &[
        ("arT8KLlbvLQ", "81. AQA GCSE (8525) SLR13 - 3.3 Images", CND),
        ("H-HlOtdemU0", "GCSE Computer Science AQA Unit 3 - Images", CSTUTOR),
    ]),
    ("cs_aqa:3.3.7", &[
        ("baTsGqkBR84", "82. AQA GCSE (8525) SLR13 - 3.3 Sound", CND),
        ("8sKqixHacdk", "GCSE Computer Science AQA Unit 3   Sound", CSTUTOR),
    ]),
    ("cs_aqa:3.3.8", &[
        ("pOhLOf6IgNQ", "83. AQA GCSE (8525) SLR13 - 3.3 Compression", CND),
        ("ZMqadiLkyWQ", "84. AQA GCSE (8525) SLR13 - 3.3 Compression Huffman coding", CND),
        ("UyOARO7UVdw", "85. AQA GCSE (8525) SLR13 - 3.3 Compression Run length encoding", CND),
        ("wuF8KGwQTkc", "GCSE Computer Science AQA Unit 3 Huffman Tree, Run length encoding, Lossy, Lossless Compression", CSTUTOR),
    ]),
    ("cs_aqa:3.4.1", &[
        ("M1sMFVbpxT4", "23. AQA GCSE (8525) SLR5 - 3.4 Hardware and software", CND),
    ]),
    ("cs_aqa:3.4.2", &[
        ("ElCMxOKNJUw", "66. AQA GCSE (8525) SLR11 - 3.4 Truth tables", CND),
        ("uE7HYTtA-pg", "65. AQA GCSE (8525) SLR11 - 3.4 Logic circuit diagrams", CND),
        ("jl0E3vmV1Co", "67. AQA GCSE (8525) SLR11 - 3.4 Boolean expressions", CND),
        ("wxqpdAxS_-s", "68. AQA GCSE (8525) SLR11 - 3.4 Applying logical operators in truth tables", CND),
    ]),
    ("cs_aqa:3.4.3", &[
        ("pYo9IebKQjw", "24. AQA GCSE (8525) SLR5 - 3.4 Systems software", CND),
        ("opwxkjPl7NU", "25. AQA GCSE (8525) SLR5 - 3.4 Operating systems 1", CND),
        ("qzVapJ_QI1Q", "26. AQA GCSE (8525) SLR5 - 3.4 Operating systems 2", CND),
        ("Z-V2_2F5a3k", "27. AQA GCSE (8525) SLR5 - 3.4 Utility system software", CND),
        ("Z8PnjbhF8M8", "GCSE Computer Science AQA unit 4 Operating systems and Utility Software", CSTUTOR),
    ]),
    ("cs_aqa:3.4.4", &[
        ("eNTafXN8j-g", "69. AQA GCSE (8525) SLR12 - 3.4 Characteristics and purposes of different languages", CND),
        ("obZDguA3_RE", "70. AQA GCSE (8525) SLR12 - 3.4 Purpose of translators", CND),
        ("-98kmXsK5BY", "71. AQA GCSE (8525) SLR12 - 3.4 Assemblers, compilers & interpreters", CND),
    ]),
    ("cs_aqa:3.4.5a", &[
        ("S17wGhjlnJ8", "1. AQA GCSE (8525) SLR1 - 3.4 CPU components", CND),
        ("mUxgOlnwoHo", "3. AQA GCSE (8525) SLR1 - 3.4 Fetch-execute cycle", CND),
        ("054pAzr0UKk", "4. AQA GCSE (8525) SLR1 - 3.4 Performance of CPUs", CND),
        ("Wk_Vkk0bvwM", "AQA Computer Science unit 4  topic 1 Systems architecture", CSTUTOR),
    ]),
    ("cs_aqa:3.4.5b", &[
        ("pwU3WJwQ-8Q", "6. AQA GCSE (8525) SLR2 - 3.4 Different types of memory", CND),
        ("gcGukgVt-4c", "7. AQA GCSE (8525) SLR2 - 3.4 RAM and ROM", CND),
        ("MQPPfiSdDwU", "8. AQA GCSE (8525) SLR2 - 3.4 The need for secondary storage", CND),
        ("98nTTU3NyJE", "10. AQA GCSE (8525) SLR2 - 3.4 Secondary storage", CND),
        ("0Z9NPPlXk_k", "9. AQA GCSE (8525) SLR2 - 3.4 Common types of storage", CND),
        ("b1tXebPtMGg", "11. AQA GCSE (8525) SLR2 - 3.4 Cloud storage", CND),
        ("tyL50MvqCds", "5. AQA GCSE (8525) SLR1 - 3.4 Embedded systems", CND),
        ("kJKMiiXphbU", "AQA GCSE Computer Science  unit 4 Topic 3 Memory", CSTUTOR),
        ("yT12A2chewQ", "GCSE Computer Science AQA unit 4 Computer systems Topic Secondary Storage", CSTUTOR),
    ]),
    ("cs_aqa:3.5a", &[
        ("DXCNsm4H8f8", "12. AQA GCSE (8525) SLR3 - 3.5 Types of networks", CND),
        ("JhoZN0QoDX4", "13. AQA GCSE (8525) SLR3 - 3.5 Modes of connection", CND),
        ("xQ742qLjgu0", "16. AQA GCSE (8525) SLR3 - 3.5 The need for & methods of network security", CND),
        ("ADMgjr3LMg8", "GCSE Computer Science AQA Unit LANs, PANs and Wans, Cloud storage, Wired and wireless networks", CSTUTOR),
    ]),
    ("cs_aqa:3.5b", &[
        ("d8aOblvUwPE", "15. AQA GCSE (8525) SLR3 - 3.5 Common network protocols", CND),
        ("q7D-XXywWYc", "17. AQA GCSE (8525) SLR3 - 3.5 The TCP-IP protocol model", CND),
        ("xVCfiaNqZ6I", "GCSE Computer Science AQA Networks Topic TCP/IP Protocol Stack and Protocols", CSTUTOR),
    ]),
    ("cs_aqa:3.6.1", &[
        ("LAi5e-61e18", "18. AQA GCSE (8525) SLR4 - 3.6 Cyber security & threats", CND),
        ("V5y5m21wrLs", "GCSE Computer Science AQA Unit 6 Cyber security", CSTUTOR),
    ]),
    ("cs_aqa:3.6.2", &[
        ("LAi5e-61e18", "18. AQA GCSE (8525) SLR4 - 3.6 Cyber security & threats", CND),
        ("g1v3BFOmMWI", "21. AQA GCSE (8525) SLR4 - 3.6 Penetration testing", CND),
        ("V5y5m21wrLs", "GCSE Computer Science AQA Unit 6 Cyber security", CSTUTOR),
    ]),
    ("cs_aqa:3.6.2.1", &[
        ("-3vrfMG70xA", "19. AQA GCSE (8525) SLR4 - 3.6 Social engineering threats", CND),
        ("V5y5m21wrLs", "GCSE Computer Science AQA Unit 6 Cyber security", CSTUTOR),
    ]),
    ("cs_aqa:3.6.2.2", &[
        ("7-bXzoDGozE", "20. AQA GCSE (8525) SLR4 - 3.6 Malware", CND),
        ("V5y5m21wrLs", "GCSE Computer Science AQA Unit 6 Cyber security", CSTUTOR),
    ]),
    ("cs_aqa:3.6.3", &[
        ("Na0ko0rnOnQ", "22. AQA GCSE (8525) SLR4 - 3.6 Security measures", CND),
        ("V5y5m21wrLs", "GCSE Computer Science AQA Unit 6 Cyber security", CSTUTOR),
    ]),
    ("cs_aqa:3.7.1", &[
        ("Q1b1mpl_oa0", "86. AQA GCSE (8525) SLR14 - 3.7 Relational databases introduction", CND),
        ("Hn3mv4dUjUo", "87. AQA GCSE (8525) SLR14 - 3.7 Relational databases concepts", CND),
    ]),
    ("cs_aqa:3.7.2", &[
        ("woK70tWDybw", "88. AQA GCSE (8525) SLR14 - 3.7 Use of SQL to search", CND),
        ("PL5eqMQp-aM", "89. AQA GCSE (8525) SLR14 - 3.7 Use of SQL to insert, update, delete", CND),
    ]),
    ("cs_aqa:3.8", &[
        ("KiZMFplVy2U", "32. AQA GCSE (8525) SLR6 - 3.8 Impacts & risks of digital technology", CND),
        ("4X3H-JTVYSg", "29. AQA GCSE (8525) SLR6 - 3.8 Privacy issues", CND),
        ("XkQRzYiDB3U", "31. AQA GCSE (8525) SLR6 - 3.8 Environmental impacts", CND),
        ("KEkSRgwHPRU", "28. GCSE AQA (8525) SLR6 - 3.8 Discussing technologies", CND),
        ("zvrg-MZPbDQ", "GCSE Computer Science AQA   Unit 8   Ethical, legal and environmental issues in technology", CSTUTOR),
    ]),
// AQA GCSE History 8145. Channel constants - add beside the others at the top
// of videos.rs (CLOKE, HISTTEACH and IMSTUCK already exist):

// Entries - paste into VIDEOS after the last subject:
    // ---------- History (AQA GCSE 8145: options 1AB, 1AD, 1BB, 1BC, 2AA, 2AB, 2BA, 2BC). Titles and channels exactly as YouTube oEmbed returned them on 30 September 2026 ----------
    ("hist_aqa:1AB.1a", &[
        ("YdCiwGg9MyI", "Episode 1-Kaiser Wilhelm II and the German Monarchy//AQA GCSE History: Germany Revision Series", RBIRD),
        ("Qo8LIfRtLSI", "AQA GCSE History Germany 1890-1945: Topic 1 - Kaiser Wilhelm II - 1890 1914", PETEJ),
        ("4sgngKUAOKs", "Kaiser Wilhelm and the difficulties of ruling Germany | GCSE History Revision, Germany 1890–1945", "History Core"),
        ("7W9nuvr8cek", "The Navy Laws | Democracy & Dictatorship | AQA GCSE History", "The Learning Academy Education"),
    ]),
    ("hist_aqa:1AB.1b", &[
        ("lH_ZM1qIQN4", "Episode 2-Impact of WW1 and the Treaty of Versailles//AQA GCSE History: Germany Revision Series", RBIRD),
        ("H7Im3cUrFDE", "Episode 3-The Weimar Republic and its unpopularity//AQA GCSE History: Germany Revision Series", RBIRD),
        ("IGTnk0UP5Gs", "Episode 5-Hyperinflation and the Recovery//AQA GCSE History: Germany Revision Series", RBIRD),
        ("dKEO1zoq6RM", "Episode 6-The Golden Years//AQA GCSE History: Germany Revision Series", RBIRD),
        ("Saa1UxNZIx8", "AQA History - Germany - Weimar Culture", "Mr Kidd - History"),
    ]),
    ("hist_aqa:1AB.2", &[
        ("2q_7_-RZ-ig", "Episode 7-The Great Depression//AQA GCSE History: Germany Revision Series", RBIRD),
        ("5DqycpJGM0s", "Episode 8-The Rise of the Nazis in the 1930's//AQA GCSE History: Germany Revision Series", RBIRD),
        ("u9RUtOfqZ4Q", "AQA GCSE History 1890-1945 - Topic 4: The impact of the Great Depression and rise of Nazis", PETEJ),
        ("35L3Ogldq4k", "GCSE History Rapid Revision- How did Hitler Become Chancellor?", CLOKE),
        ("XALGoKldRAE", "GCSE History Rapid Revision- Building Dictatorship 1933-34", CLOKE),
    ]),
    ("hist_aqa:1AB.3a", &[
        ("byasKZ4NylE", "Episode 12-Work and Home//AQA GCSE History: Germany Revision Series", RBIRD),
        ("_80O0amD9zw", "Episode 15-Germany's War Economy//AQA GCSE History: Germany Revision Series", RBIRD),
        ("5_klJqNiAi0", "Episode 17-The Impact of Total War//AQA GCSE History: Germany Revision Series", RBIRD),
    ]),
    ("hist_aqa:1AB.3b", &[
        ("D4ZtZetzpi8", "Episode 13-Young People in Nazi Germany//AQA GCSE History: Germany Revision Series", RBIRD),
        ("Y2sV1lbw-m4", "Episode 14-Nazi Racial Policy//AQA GCSE History: Germany Revision Series", RBIRD),
        ("wDiluWAjQ10", "Episode 11-The Nazis and the Church//AQA GCSE History: Germany Revision Series", RBIRD),
        ("8nTvz1Ev6nw", "Episode 18-The Holocaust//AQA GCSE History: Germany Revision Series", RBIRD),
        ("D2L4GQW-nr8", "GCSE History Rapid Revision: Women in Nazi Germany", CLOKE),
    ]),
    ("hist_aqa:1AB.3c", &[
        ("MET6q5kbz4s", "Episode 9-Nazi Propaganda//AQA GCSE History: Germany Revision Series", RBIRD),
        ("oJuY6zxqpBA", "Episode 10-Living in the Nazi Terror State//AQA GCSE History: Germany Revision Series", RBIRD),
        ("YCw_YVTTufg", "Episode 16-Opposition towards to Nazis//AQA GCSE History: Germany Revision Series", RBIRD),
        ("vTyk-k8PfHg", "GCSE History Rapid Revision-  Opposition and Resistance to the Nazis", CLOKE),
    ]),
    ("hist_aqa:1AD.1a", &[
        ("SU1lRX3ZFJw", "America 1920-1973: Complete Overview | Introduction & Timeline | AQA GCSE Revision Guide", IMSTUCK),
        ("Rh0Qeu5NKfc", "Why was there a boom in America 1920s? Why did people buy shares? GCSE AMERICA GRADE 9", HRS),
        ("632xI-wfD68", "Causes of The 1920s BOOM | WW1, Natural Resources, Government Policies | GCSE History | Part 1", IMSTUCK),
        ("THc37UbltH8", "How did life change for women in 1920s America? GCSE AMERICA | Get a Grade 9", HRS),
    ]),
    ("hist_aqa:1AD.1b", &[
        ("ZcFHNEraMGM", "Prohibition and Organised Crime in the 1920s- AQA GCSE History- America 1920-1973: Revision notes", HTUTOR),
        ("__mx-DkRtNw", "Organised Crime in America - who was Al Capone? GCSE AMERICA 1920-1973 | Get a Grade 9", HRS),
        ("XyohAdDPdXU", "Immigration to America in the 1920s | GCSE AMERICA 1920-1973 | Get a Grade 9", HRS),
        ("rtF09nhZE-Q", "GCSE AMERICA 1920-1973 | L12: The Red Scare", HRS),
        ("_rCaOQpFvbo", "GCSE History: Racism and the Ku Klux Klan", "ArkVega"),
    ]),
    ("hist_aqa:1AD.2a", &[
        ("xS1k5LZNDAo", "Why did Wall St Crash? | GCSE AMERICA 1920-1973 L13", HRS),
        ("uvOKARPUYTI", "What did Hoover do in the Great Depression? | GCSE AMERICA 1920-1973 L15", HRS),
        ("6McAQKarV1U", "What was the New Deal? | AMERICA 1920-1973 Get a Grade 9", HRS),
        ("Bm6LT-CrjpQ", "How effective was Roosevelt's New Deal? GCSE AMERICA 1920-1973 | Get a Grade 9!", HRS),
        ("yYUeghI7mB8", "GCSE History: Opposition to the New Deal", "ArkVega"),
    ]),
    ("hist_aqa:1AD.2b", &[
        ("ABGi6ecphMk", "How did World War Two change lives in America? GCSE AMERICA 1920-1973 L22", HRS),
        ("OKHJv2edKP4", "Impact of WWII on America- AQA GCSE History- America 1920-1973: Revision", HTUTOR),
        ("vuXvnxSRCXU", "GCSE History | How Did WWII Change America? | Bitesize | GCSE Revision", "BBC Bitesize - GCSE Revision Support"),
    ]),
    ("hist_aqa:1AD.3a", &[
        ("BuXf5CJweS8", "GCSE USA- McCarthyism", "St Bede’s History Department"),
        ("3Xkn_TdrANk", "American Prosperity 1950s #GCSE #AQA #Education", "Mr Keir History Teacher"),
        ("N35IugBYH04", "What is McCarthyism? And how did it happen? - Ellen Schrecker", "TED-Ed"),
    ]),
    ("hist_aqa:1AD.3b", &[
        ("iTLokw5omfQ", "GCSE History - Civil Rights: The Montgomery Bus Boycott 1955-56", CLOKE),
        ("B99LM69jfcM", "Civil Rights Movement - Different Ways Of Protesting - GCSE History", IMSTUCK),
        ("hUgGG1L7zt4", "GCSE History - Civil Rights: Malcolm X and Black Power", CLOKE),
        ("NU3Rw2XTdPc", "GCSE History Rapid Revision: The Civil Rights Act, 1964", CLOKE),
    ]),
    ("hist_aqa:1AD.3c", &[
        ("zhKqeYtp6rU", "Kennedy's 'New Frontier' and Johnson's 'Great Society'- AQA GCSE History Revision Talk Through", HTUTOR),
        ("rI5AC8B4j38", "11  JFK's 'New Frontier' and LBJ's 'Great Society'   Teacher Explanation   #HisTV", "HisTV"),
        ("1YvJJEFOYNY", "12  Feminism in the 1960s and early 1970s   Teacher Explanation   #HisTV", "HisTV"),
    ]),
    ("hist_aqa:1BB.1", &[
        ("L-xL62P4SoI", "AQA GCSE History Conflict and Tension 1918-1939 Part 1 - The Big Three", PETEJ),
        ("v2xTKTKwZJs", "AQA GCSE History Conflict and Tension 1918-1939 Part 2 - The Treaty of Versailles", PETEJ),
        ("KJS9xRCZkQ0", "GCSE History Rapid Revision:  Terms of the Treaty of Versailles", CLOKE),
        ("KwIEKR23WyE", "Germany's Humiliation: The Treaty of Versailles (AQA GCSE History)", "Academyk"),
    ]),
    ("hist_aqa:1BB.2a", &[
        ("VaLZKQH_Pwk", "League of Nations structure and success/failures in the 1920s- GCSE History talk through for AQA.", HTUTOR),
        ("U5j0hgj7bug", "League of Nations - Successes and Failures - GCSE History", IMSTUCK),
        ("3KNTcC8FDJ0", "AQA GCSE History Conflict and Tension 1918-1939 Part 5 - Diplomacy outside the League of Nations", PETEJ),
        ("q2eNpOtzWy0", "Locarno Pact - Undermining the League of Nations #Conflict&Tension #AQA #GCSE #Education", "Mr Keir History Teacher"),
    ]),
    ("hist_aqa:1BB.2b", &[
        ("gTEAG5534ow", "AQA GCSE History Conflict and Tension 1918-1939 Part 6 - The League of Nations in the 1930s", PETEJ),
        ("z8_EmBkCnHc", "The Manchurian Crisis | Conflict & Tension | GCSE History", "The Learning Academy Education"),
        ("mi6rnLEtAJ0", "The Abyssinian Crisis | Conflict & Tension | GCSE History", "The Learning Academy Education"),
        ("rjTrCFNQhEc", "AQA History - Conflict and Tension - What was the Abyssinian Crisis", "Mr Kidd - History"),
    ]),
    ("hist_aqa:1BB.3a", &[
        ("5MzjxQNW6Gk", "AQA GCSE History Conflict and Tension 1918-1939 - Part 7 - The Development of Tension 1933-1935", PETEJ),
        ("Vu7squNlJyk", "AQA History - Conflict and Tension - What actually caused WWII", "Mr Kidd - History"),
        ("nI1Kn_lMcNU", "2: GCSE History - Remilitarisation of the Rhineland & Anschluss", "Simon Hinds"),
        ("TNOea7SyI1c", "The Munich Agreement 1938 | GCSE History", "GCSEHistoryChannel"),
    ]),
    ("hist_aqa:1BB.3b", &[
        ("W2dkEL2n5C0", "AQA GCSE History Conflict and Tension 1918-1939 Part 9 - The outbreak of World War 2", PETEJ),
        ("enr95a7GjMc", "AQA History - Conflict and Tension - Nazi Soviet Pact", "Mr Kidd - History"),
        ("oOlOXIundc8", "Nazi-Soviet Pact - How It Started WW2 - GCSE History", IMSTUCK),
    ]),
    ("hist_aqa:1BC.1a", &[
        ("03Y4__g0o5A", "Episode 2- Yalta and Potsdam Conferences//AQA GCSE History: Cold War Revision Series", RBIRD),
        ("d7qv_MltlFY", "Yalta and Potsdam: Write an Account example answer- GCSE History 1945-1972 Cold War AQA", HTUTOR),
        ("bu17c_3OTHw", "The Cold War: The Potsdam Conference 1945 - Truman, Attlee and Stalin -  Episode 3", IMSTUCK),
    ]),
    ("hist_aqa:1BC.1b", &[
        ("9sUMAPBaV5U", "Episode 3- Iron Curtain, Marshall Aid, Berlin Blockade//AQA GCSE History: Cold War Revision Series", RBIRD),
        ("dgRR8DNKOcM", "GCSE History Rapid Revision: Cold War/Superpower Relation - Truman Doctrine and Marshall Plan", CLOKE),
        ("J1hxAhd-9hc", "The Berlin Blockade and Airlift - Superpower Relations and the Cold War", HISTTEACH),
        ("dx2IUWXBcoQ", "The Cold War: Cominform and Comecon - Episode 8", IMSTUCK),
    ]),
    ("hist_aqa:1BC.2", &[
        ("AHaUlKx9M78", "Episode 4- Tension in Asia (Korea, Vietnam and China) //AQA GCSE History: Cold War Revision Series", RBIRD),
        ("-OvV20kP7Go", "Episode 5 -The Arms Race//AQA GCSE History: Cold War Revision Series", RBIRD),
        ("LkDVarp_Vkc", "Episode 6 -The Space Race//AQA GCSE History: Cold War Revision Series", RBIRD),
        ("Hy-Ax0jEO74", "Episode 7- The Hungarian Uprising//AQA GCSE History: Cold War Revision Series", RBIRD),
        ("1ZpLnqtk04U", "Episode 8- Espionage and the U-2//AQA GCSE History: Cold War Revision Series", RBIRD),
    ]),
    ("hist_aqa:1BC.3a", &[
        ("bQDnyIvEB6w", "Episode 9- The Berlin Wall//AQA GCSE History: Cold War Revision Series", RBIRD),
        ("C1W_XCCI3bA", "Episode 10- The Cuban missile Crisis/AQA GCSE History: Cold War Revision Series", RBIRD),
        ("icjKb1BNiaA", "GCSE History Rapid Revision: Superpower Relations and the Cold War - The Berlin Ultimatum and Wall", CLOKE),
        ("FVEDrterwVY", "GCSE History Rapid Revision: Superpower Relations and the Cold War - The Cuban Missile Crisis", CLOKE),
    ]),
    ("hist_aqa:1BC.3b", &[
        ("ftgrvtl7a_0", "Episode 11- The Prague Spring/AQA GCSE History: Cold War Revision Series", RBIRD),
        ("Yj5b5GktlqQ", "Episode 12- Détente and Easing the Tension/AQA GCSE History: Cold War Revision Series", RBIRD),
        ("WNYVJjyoqWQ", "The Cold War: The Prague Spring 1968 and the Crisis in Czechoslovakia - Episode 40", IMSTUCK),
    ]),
    ("hist_aqa:2AA.1", &[
        ("1T_Me7fG534", "Episode 1-Medicine in the Middle Ages//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("Kvk7tPr1pzE", "Episode 2-Islamic Medicine//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("7CxWARoPkX4", "Episode 3-Treating Disease in the Middle Ages//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("2axv4sZd_4k", "Episode 5-Britain's Black Death//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("glJcvFgaGyU", "AQA GCSE History: Health & the People | Medieval Health Overview", "Mr Slone History"),
    ]),
    ("hist_aqa:2AA.2", &[
        ("d4tWNAmhsPs", "Episode 6 - The Renaissance//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("t0NCA2J833A", "Episode 7 - The Changes in Medical Treatment//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("gcejvnZMEeU", "Episode 8 - The Rise of Doctors and Surgery//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("3OcUuyuJXSg", "AQA Health & The People 5.  The Renaissance, Vesalius, Pare & Harvey", "Teacher Coupe"),
        ("3Y1eF3QPYH4", "Episode 10 - Edward Jenner//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("oXn_fQei-Ec", "GCSE History - Who was John Hunter?", "worcesterjonny"),
    ]),
    ("hist_aqa:2AA.3a", &[
        ("xEwVueybHUQ", "Episode 11 -Germ Theory and Fighting Germs//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("wqiCy8TQLxs", "Episode 12 -Anaesthetics//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("8Wpxu4sYIPk", "Episode 13 -Antiseptics//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("TyLer_NI1Rc", "GCSE History: Louis Pasteur & Robert Koch | Health and Medicine Revision", "The A-Level & GCSE History Tutor"),
        ("REq1ywT0XfQ", "GCSE History Rapid Revision: Magic Bullets", CLOKE),
    ]),
    ("hist_aqa:2AA.3b", &[
        ("f-leEUu0-v8", "Episode 14 -Public Health//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("ZzhVLS-pXFM", "GCSE History Rapid Revision: 19th Century Public Health", CLOKE),
        ("935_zSVbHkk", "GCSE History Rapid Revision: Dr. John Snow and the Broad Street Cholera Epidemic", CLOKE),
    ]),
    ("hist_aqa:2AA.4a", &[
        ("l35ciSSALt4", "Episode 16- Fleming and Penicillin//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("nLXauAMZSRI", "Episode 17- Modern Treatments//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("ldFnR-Ugwms", "Episode 15 -The Impact of the World Wars//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("RsXHqYSIbbY", "GCSE History Rapid Revision: Penicillin", CLOKE),
    ]),
    ("hist_aqa:2AA.4b", &[
        ("YgD2VBxsr8s", "Episode 18- The Liberal and Social Reforms//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("bu1pSysSI9s", "Episode 19- The National Health Service//AQA GCSE History: Medicine Revision Series", RBIRD),
        ("5DOJFNCjA8U", "GCSE History Rapid Revision: The Liberal Reforms 1906-1914", CLOKE),
        ("pb76WtsccAk", "GCSE History Rapid Revision: Origins of the NHS", CLOKE),
    ]),
    ("hist_aqa:2AB.1", &[
        ("i_gchqy3Jq0", "Episode 1- The Magna Carta//AQA GCSE History: Britain, Power and the People Revision Series", RBIRD),
        ("Tuc2Ntldlas", "Episode 2-Simon de Montfort//AQA GCSE History: Britain, Power and the People Revision Series", RBIRD),
        ("vQZpOZqhNIw", "Episode 3-Peasants' Revolt/AQA GCSE History: Britain, Power and the People Revision Series", RBIRD),
        ("oOAwX6s0vmA", "King John and the Magna Carta | GCSE Power and the People (1)", APRG),
        ("l8yJ_JQRuq8", "Simon de Montfort and the Provisions of Oxford | GCSE Power and the People (2)", APRG),
        ("5-jutI5TO9o", "The Peasants' Revolt of 1381 | GCSE Power and the People (3)", APRG),
    ]),
    ("hist_aqa:2AB.2", &[
        ("K75NOnqjBWw", "Henry VIII and the Pilgrimage of Grace | GCSE Power and the People (4)", APRG),
        ("_eHFaUK9l7o", "GCSE History Rapid Revision - The Pilgrimage of Grace", CLOKE),
        ("9_Z94eNSXkg", "Causes of the English Civil War | GCSE Power and the People (5)", APRG),
        ("RnVgA3f5bnc", "The English Civil War | GCSE Power and the People (6)", APRG),
        ("5JP6j3tCotc", "The American Revolution explained | GCSE Power and the People (7)", APRG),
    ]),
    ("hist_aqa:2AB.3", &[
        ("ZoQtZo3nfSA", "Who were the Chartists? | GCSE Power and the People (8)", APRG),
        ("bw2rUP8tTNk", "What was the Anti-Corn Law League? | GCSE Power and the People (9)", APRG),
        ("brFMRkynEo8", "19th Century Trade Unions | GCSE Power and the People (10)", APRG),
        ("7geGIkPy5q0", "What was the Great Reform Act 1832? | Parliamentary Archives", "History Hub"),
    ]),
    ("hist_aqa:2AB.4", &[
        ("P2lvHVSSLRg", "The Women's Suffrage Movement | GCSE Power and the People (12)", APRG),
        ("F-69nGRdk1o", "1926 General Strike | GCSE Power and the People (13)", APRG),
        ("wNdMk5B3DBs", "Minority Rights in Britain since WW2 | GCSE Power and the People (15)", APRG),
        ("lY9IVnfmCAw", "Spotlight On: Brixton Riots", "The National Archives UK"),
    ]),
    ("hist_aqa:2BA.1a", &[
        ("N451xrA5O7o", "GCSE History Rapid Revision: The Succession Crisis of 1066", CLOKE),
        ("8T6j6gO4UnM", "GCSE History Rapid Revision: The Battle of Stamford Bridge", CLOKE),
        ("5fhrbTPq4YY", "GCSE Rapid Revision: The Battle of Hastings", CLOKE),
        ("1woLduPy3sY", "GCSE History - Saxons and Normans: Why did William Win the Battle of Hastings?", CLOKE),
    ]),
    ("hist_aqa:2BA.1b", &[
        ("I-dtpRYCR8s", "GCSE History Rapid Revision: Early Rebellions 1068-1071", CLOKE),
        ("B_gwfiP0ztM", "GCSE History Rapid Revision: The Harrying of the North", CLOKE),
        ("ISYl4czhBqg", "GCSE History - Saxons and Normans: The Revolt of the Earls, 1075", CLOKE),
        ("9iqV_dztwpA", "GCSE History - Saxons and Normans: After Hastings - How did William Secure his Power?", CLOKE),
    ]),
    ("hist_aqa:2BA.2", &[
        ("wGwWcTCdb4Q", "Video 15: AQA GCSE History Paper 2: Norman England Part 1", "Joseph Burrows"),
        ("9RVLUJCfZuQ", "GCSE History Rapid Revision: The Feudal System", CLOKE),
        ("qtkgDDq469c", "GCSE History Rapid Revision: The Domesday Book", CLOKE),
        ("CFnMMO_-nFw", "GCSE History Rapid Revision: Crime and Punishment- Normans: Murdrum and Forest Laws", CLOKE),
        ("MJV8GR6x6CQ", "GCSE History Rapid Revision: The Norman Legal System", CLOKE),
    ]),
    ("hist_aqa:2BA.3", &[
        ("Y3ZtMWB4cD0", "Video 20: AQA GCSE History Paper 2: Norman England - Religion under the Normans", "Joseph Burrows"),
        ("EmqN02DuksY", "GCSE History Rapid Revision: The Norman Church", CLOKE),
        ("KDmeJ1Zucd8", "The Second Norman Conquest | Lanfranc's Reforms", "History Hub"),
    ]),
    ("hist_aqa:2BA.4", &[
        ("euZlouI0wpU", "What the White Tower tells us about religion and culture | Norman History | Schools and Teachers", "Historic Royal Palaces"),
        ("UoMMhzWbQkY", "A Medieval Masterpiece | William the Conqueror's Tower of London", "Historic Royal Palaces"),
        ("ii6AT3nsImg", "History Y10 GCSE The White Tower Design and Construction", "The Garibaldi School"),
    ]),
    ("hist_aqa:2BC.1", &[
        ("-bXEk7lCF5w", "Episode 1-An Introduction to Elizabeth I//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
        ("bCWLrJhtAEE", "Episode 2-The Elizabethan Court//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
        ("gXvb49LD96Q", "Episode 3-The Elizabethan Government//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
        ("vL6d-fIG4Hw", "Episode 4-  Elizabeth and Marriage//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
        ("TLKB7w_DLyY", "Essex's Rebellion 1601", PETEJ),
        ("1u1pgzappM4", "Episode 16-The end of Elizabeth's Reign//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
    ]),
    ("hist_aqa:2BC.2", &[
        ("QJr8olZefCU", "Episode 6- The Golden Age//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
        ("3lBQKRa4SHg", "Episode 7- Elizabethan Theatres//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
        ("miPxy3tUZMU", "Episode 5- Poverty in Elizabethan England//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
        ("hCMBkXCTMQ4", "AQA GCSE History Elizabethan England (1568-1603) - The Poor in Elizabethan England", PETEJ),
        ("b4oGHJWHHIw", "Episode 8-The Elizabethan Sailors//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
    ]),
    ("hist_aqa:2BC.3a", &[
        ("jAmrJf_Txeo", "Episode 12-The Catholic Threat//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
        ("mrMnlT_X8xM", "Episode 13-The Puritan Threat//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
        ("NnDm_5Zub3w", "Episode 11-The Northern Rebellion//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
        ("aQQMON3JHDM", "Episode 10-Mary, Queen of Scots//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
    ]),
    ("hist_aqa:2BC.3b", &[
        ("W24jpcrMYOY", "Episode 14-The War with Spain//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
        ("ikqmrJVwxF0", "Episode 15-The Spanish Armada//AQA GCSE History: Elizabeth I Revision Series", RBIRD),
        ("q1etLovNOaY", "GCSE History Rapid Revision: Elizabethan England - The Spanish Armada", CLOKE),
    ]),
    ("hist_aqa:2BC.4", &[
        ("lxpiSHcUAS8", "Kenilworth Castle GCSE Tour", "English Heritage"),
        ("IFuW1bkbDpw", "Kenilworth Castle | 10 Places That Made England with Dan Snow", "English Heritage"),
        ("VzeplGNrIhc", "Kenilworth Castle - physical features: 1266-1575", "Mr Loudon"),
    ]),
    // WJEC Eduqas GCSE Religious Studies C120QS (Route A, Islam). Read off
    // YouTube on 30 September 2026; every id, title and channel checked against
    // YouTube's oEmbed response. Mr Elliott, Head of Religious Studies and
    // Citizenship, Lytchett PRE and Peter Kernaghan teach the Eduqas Route A
    // papers; MrMcMillanREvis and RS Bites cover the shared ethics content;
    // AQA/Edexcel channels (Ben Wardle, No Waffle GCSE, Ms. Bee, Miss Morris
    // Manc) are used only where the Islamic content is the same; charities and
    // Humanists UK speak for themselves.
    ("rs_edq:2.1a", &[
        ("Re8FHRclTY4", "GCSE Religious Studies (Eduqas) - Theme 1: Issues of Relationships (explainer and revision video)", "Mr Elliott"),
        ("pdsKASnwN10", "Paper 1 - Unit 1: Issues of relationships (Eduqas: Route A - Paper 1)", "Head of Religious Studies and Citizenship"),
        ("Ld7KoRd_WIs", "GCSE RS Unit 3.3 - Marriage & Family Life in 5 Minutes | by MrMcMillanREvis", "MrMcMillanREvis"),
    ]),
    ("rs_edq:2.1b", &[
        ("7G8iu2KLi-0", "GCSE RS Unit 3.3 (Part 2 of 5) Divorce | by MrMcMillanREvis", "MrMcMillanREvis"),
        ("lV7t6Vr5xJo", "GCSE RS Unit 3.3 (Part 5 of 5) Contraception | by MrMcMillanREvis", "MrMcMillanREvis"),
        ("2yGZFdo0gE0", "Natural Law - GCSE RS Bites", "RS Bites"),
    ]),
    ("rs_edq:2.1c", &[
        ("5Mw_Zkjjdfs", "GCSE RS Unit 3.3 (Part 3 of 5) Homosexuality | by MrMcMillanREvis", "MrMcMillanREvis"),
        ("fk92bMEneAg", "Should women be priests?  | Religous Studies - Divine Women", "BBC Bitesize for Teachers"),
        ("-KRBIa3FNkc", "2.7-2.8 Christian teachings about the role of men and women: GCSE Religious Studies", "RS Teacher"),
    ]),
    ("rs_edq:2.1d", &[
        ("sw1pA5aEq1w", "1 Life and Death Issues Orgins of the Universe", "Peter Kernaghan"),
        ("xUqs9DcSIUc", "Paper 1 - Unit 2: Issues of Life and Death (Eduqas: Route A - Paper 1)", "Head of Religious Studies and Citizenship"),
        ("UEgfj6Bf4uc", "The Big Bang and Evolution - GCSE RS Bites", "RS Bites"),
    ]),
    ("rs_edq:2.1e", &[
        ("SF6Uzy0yaCQ", "2 Life and Death Issues Value of the world and animals", "Peter Kernaghan"),
        ("8BoitEO6QR8", "Why is environmental sustainability important to Christians - Eduqas GCSE Life and Death", "Lytchett PRE"),
        ("YzfU-eRe_Hs", "Peter Singer and Speciesism - GCSE RS Bites", "RS Bites"),
    ]),
    ("rs_edq:2.1f", &[
        ("2xDzi71TkQY", "4 Life and Death Issues Abortion and Euthanasia", "Peter Kernaghan"),
        ("VX7tArYx7iI", "3 Life and Death Issues Origin and Value of life", "Peter Kernaghan"),
        ("lVQ7kMP0JbA", "GCSE Religious Studies (Eduqas) - Theme 2: Issues of Life & Death (explainer and revision video)", "Mr Elliott"),
        ("p1PeICe4MUk", "'People should be allowed to choose to die with dignity.' | Andrew Copson | Assisted dying", "Humanists UK"),
    ]),
    ("rs_edq:2.1g", &[
        ("lP50M-wSrOA", "5 Life ad Death Issues   Life after Death  Funerals", "Peter Kernaghan"),
        ("MUbHa9Bl6FE", "Life after death | Islam", "No Waffle GCSE"),
        ("28FxOKDpobE", "Humanist, non-religious funerals", "Humanists UK"),
    ]),
    ("rs_edq:2.1h", &[
        ("uQwy7pIlyig", "GCSE Religious Studies (Eduqas) - Theme 3: Issues of Good & Evil (explainer and revision video)", "Mr Elliott"),
        ("6WzTxwCWYi0", "Paper 1 - Unit 3: Issues of Good and Evil (Eduqas: Route A - Paper 1)", "Head of Religious Studies and Citizenship"),
        ("FUDmovWczng", "John Hick & the Soul-Making Theodicy | The Problem of Evil", "Untangle Philosophy | Philosophy Made Simple"),
    ]),
    ("rs_edq:2.1i", &[
        ("YK8fFmKAiaA", "Capital Punishment - GCSE RS Bites", "RS Bites"),
        ("77rIfQF3r9c", "AQA Mr Lane Crime and Punishment : Islam Aims of Punishment", "Joe Lane (Mr Lane)"),
        ("yv-DIs71EQM", "AQA Mr Lane Crime and Punishment : Islam Capital Punishment", "Joe Lane (Mr Lane)"),
        ("qqo0vYvrSPU", "What are the rights and wrongs of the death penalty? | Religious Studies - Matters of Life and Death", "BBC Bitesize for Teachers"),
    ]),
    ("rs_edq:2.1j", &[
        ("tgQyBLWFDBI", "RE - A Mother's Forgiveness - BBC Learing", "bbclearning"),
        ("i-OhBopYz8Y", "Forgiveness in the Bible - GCSE RS Bites", "RS Bites"),
        ("pEE7fCWAHP0", "GCSE Religious Studies | Should religious people always forgive? | Bitesize | GCSE Revision", "BBC Bitesize - GCSE Revision Support"),
    ]),
    ("rs_edq:2.1k", &[
        ("jx7Sqzbqg0I", "GCSE Religious Studies (Eduqas) - Theme 4: Issues of Human Rights (explainer and revision video)", "Mr Elliott"),
        ("3lMakXOxrNw", "Paper 1 - Unit 4: Issues of Human Rights (Eduqas: Route A - Paper 1)", "Head of Religious Studies and Citizenship"),
        ("n3XDN6TAHoM", "E is for Extremism | A to Z of Religion and Beliefs | BBC Teach", "BBC Bitesize for Teachers"),
    ]),
    ("rs_edq:2.1l", &[
        ("Eh_HXrurrTA", "GCSE RS Unit 3.4 (Part 2 of 4) Discrimination & Prejudice | by MrMcMillanREvis", "MrMcMillanREvis"),
        ("ymc0ala25Fo", "Martin Luther King Jr. - Religion and Equality Revision", "Miss Richards"),
        ("ulCLciKTS7w", "Come to the Edge: Faith, Fear & Friendship", "Christian Muslim Forum"),
    ]),
    ("rs_edq:2.1m", &[
        ("NFy-pYDVRiQ", "GCSE RS Unit 3.2 (Part 5 of 5) World Poverty | by MrMcMillanREvis", "MrMcMillanREvis"),
        ("GT9JNpi0P44", "This is Christian Aid", "thisischristianaid"),
        ("Bhm0Ht95ceU", "40 years of Islamic Relief", "Islamic Relief Worldwide"),
    ]),
    ("rs_edq:2.2a", &[
        ("cuA4wL8XL_A", "1 Christian Beliefs and teachings   Nature of God", "Peter Kernaghan"),
        ("liCWoXqu2pI", "GCSE Religious Studies (Eduqas) - Christian Beliefs and Practices (explainer and revision video)", "Mr Elliott"),
        ("oxZ9-uHheoM", "The Trinity - GCSE RS Bites", "RS Bites"),
        ("e2fQNXvwWKg", "Describe Christian beliefs about suffering Eduqas GCSE RS", "Lytchett PRE"),
    ]),
    ("rs_edq:2.2b", &[
        ("67b-mb2mpKo", "2 Christian Beliefs and Teachings   Creation", "Peter Kernaghan"),
        ("Us0zsu4s0AY", "Eduqas GCSE RE Explain the different ways that Christians interpret the Genesis Creation accounts", "Lytchett PRE"),
    ]),
    ("rs_edq:2.2c", &[
        ("66k0ZbNomqA", "3 Christian Beliefs and teachings   Jesus Christ", "Peter Kernaghan"),
        ("cmGQYnFnNhY", "4 Christian Beliefs and Teachings Salvation", "Peter Kernaghan"),
        ("qmJQs_B4KHo", "Paper 2 - Christian Beliefs (Eduqas: Route A)", "Head of Religious Studies and Citizenship"),
    ]),
    ("rs_edq:2.2d", &[
        ("fwuX5ro2Z-w", "5 Christian Beliefs and Teachings Eschatology", "Peter Kernaghan"),
        ("iGk0rBBFsbo", "The afterlife heaven, hell and purgatory. Christianity. GCSE AQA Religious Studies", "Wise Revise"),
    ]),
    ("rs_edq:2.2e", &[
        ("MHW0EgKZ_uM", "GCSE RS Revision Comp 2 Christian Practices 1of5", "Peter Kernaghan"),
        ("E9KLsw6KCM8", "Paper 2 - Christian Practices (Eduqas: Route A)", "Head of Religious Studies and Citizenship"),
        ("8JpYuY1sF_I", "Non-Liturgical and Liturgical Worship EXPLAINED!", "Louis Revision"),
    ]),
    ("rs_edq:2.2f", &[
        ("OZ2dTz5wiPo", "The Sacraments - GCSE RS Bites", "RS Bites"),
        ("uT8EXzrstZA", "The Eucharist - GCSE RS Bites", "RS Bites"),
        ("uRymbCJmbYk", "What is a Christian Baptism? | Religious Studies - My Life, My Religion: Christianity", "BBC Bitesize for Teachers"),
    ]),
    ("rs_edq:2.2g", &[
        ("hddgmFy_-pY", "Educational Videos for Schools: Why Walsingham?", "The Anglican Shrine of Our Lady of Walsingham"),
        ("1njIvRW2RgI", "Reflections on a week in Taizé", "Diocese of Norwich (Church of England)"),
        ("wXk3wsnBia8", "3.5 Christian religious celebrations: GCSE Religious Studies", "RS Teacher"),
        ("CageBjJLDjA", "\"Easter is most important festival for Christians\"", "Lytchett PRE"),
    ]),
    ("rs_edq:2.2h", &[
        ("vnYtxRU5xIs", "Census: Christians in England & Wales falls below half for first time", "Sky News"),
        ("RwCFJYmWGig", "Church in the community | AQA Christianity", "No Waffle GCSE"),
        ("ZDfL67jrAGs", "Mission and Evangelism | AQA Christianity", "No Waffle GCSE"),
    ]),
    ("rs_edq:2.2i", &[
        ("UVlrCexLrYM", "Introduction to Tearfund", "Tearfund"),
        ("uZaAnpeClm0", "GCSE Christian Practices. Describe the work of the World Council of Churches", "Lytchett PRE"),
        ("-f0sgHwrd9Q", "Eduqas GCSE RS Explain the importance of the ecumenical movement for Christians.", "Lytchett PRE"),
        ("FG3p2iEdeGQ", "Persecution | AQA Christianity", "No Waffle GCSE"),
    ]),
    ("rs_edq:2.3a", &[
        ("2J9bofm3BUA", "GCSE Religious Studies (Eduqas) - Islamic Beliefs and Practices (explainer and revision video)", "Mr Elliott"),
        ("vsMbHRDJNOY", "Paper 3 - Islamic Beliefs: Sunni (Eduqas: Route A)", "Head of Religious Studies and Citizenship"),
        ("9XXe3RQn_TA", "ISLAM: TAWHID & THE NATURE OF GOD", "Ben Wardle"),
    ]),
    ("rs_edq:2.3b", &[
        ("M13QODyn-pU", "Paper 3 - Islamic Beliefs: Shi'a (Eduqas: Route A)", "Head of Religious Studies and Citizenship"),
        ("9ZH9S9LO84Q", "AQA GCSE Religious Studies Islam Six Articles of Faith and Five Roots of Usul Ad-Din Revision", "MsRutherfordRS"),
        ("rkakZtq2fKk", "1. Six articles of faith and five roots of Usul ad-Din", "ColmersRS"),
    ]),
    ("rs_edq:2.3c", &[
        ("hztR5PvcDgY", "GCSE ISLAM: RISALAH - PROPHETHOOD (ADAM, IBRAHIM, ISA & MUHAMMAD) | GCSE RELIGIOUS STUDIES AQA", "Ben Wardle"),
        ("-gxw0WWrpJo", "Prophethood: Adam| AQA Islam", "No Waffle GCSE"),
        ("Vap4TQi4Jjw", "Prophethood: Ibrahim | AQA Islam", "No Waffle GCSE"),
    ]),
    ("rs_edq:2.3d", &[
        ("4XSEphk7ovg", "ISLAM: ANGELS - NATURE & IMPORTANCE (GCSE RELIGIOUS STUDIES)", "Ben Wardle"),
        ("d77En82EcpI", "Basic Beliefs of Islam - Angels", "University of Nottingham"),
        ("8WxyWxIb-qU", "Angels | AQA Islam", "No Waffle GCSE"),
    ]),
    ("rs_edq:2.3e", &[
        ("DAb-ETJjtLg", "ISLAM: AKHIRAH - LIFE AFTER DEATH & PREDESTINATION (GCSE RELIGIOUS STUDIES)", "Ben Wardle"),
        ("HIOSi94tI5c", "Akhirah - Islam Beliefs and Teachings - AQA GCSE Religious Studies", "Ms. Bee Religious Studies"),
    ]),
    ("rs_edq:2.3f", &[
        ("UIAM3VQ27hY", "Paper 3 - Islamic Practices: (Eduqas: Route A)", "Head of Religious Studies and Citizenship"),
        ("H9U8T8x1AhQ", "The Five Pillars of Islam | Religions of the World", "BBC Bitesize for Teachers"),
        ("Ok7-mB62xeE", "What is Hajj? | Religious Studies - My Life, My Religion: Islam", "BBC Bitesize for Teachers"),
        ("PrqxCU6Y_7c", "GCSE Religious Studies (Eduqas): Walkthrough - Islam Paper (Route A)", "Mr Elliott"),
    ]),
    ("rs_edq:2.3g", &[
        ("VyElTrvo6_s", "01 Islam Practices   5 Pillars & 10 Obligatory Acts", "Harris Federation Religious Studies"),
        ("D6VLzBBRpG4", "Edexcel Religious Studies - Living the Muslim Life - 1 Ten Obligatory Acts", "Miss Morris Manc"),
    ]),
    ("rs_edq:2.3h", &[
        ("PrDAmMqfqh8", "ISLAM PRACTICES - Jihad", "Reigate_School_RE_Sociology"),
        ("825Nqua2tVo", "EXAM TECHNIQUE - 12 mark question: 'Greater jihad is more important than lesser jihad'.", "Reigate_School_RE_Sociology"),
    ]),
    ("rs_edq:2.3i", &[
        ("0hWJenSYrZQ", "AQA GCSE Religious Studies - Islamic Festivals", "Ms. Bee Religious Studies"),
        ("qFU9Cb0D6lo", "Ramadan and Eid-ul-Fitr | Religious Studies - My Life, My Religion: Islam", "BBC Bitesize for Teachers"),
        ("bT9gagG48Gc", "What is Ashura and how do Muslims observe it?", "Middle East Eye"),
        ("Gfl_fkUe5Ms", "Children's Series - What is Laylatul Qadr? - The Night of Decree - Mufti Menk", "Mufti Menk"),
    ]),
];

/// The built-in videos for a topic, first to watch first.
pub fn for_topic(topic_id: &str) -> Vec<Video> {
    VIDEOS.iter().find(|(id, _)| *id == topic_id)
        .map(|(_, vs)| vs.iter().map(|(id, title, by)| Video { id: id.to_string(), title: title.to_string(), by: by.to_string() }).collect())
        .unwrap_or_default()
}

/// The video id inside whatever someone pasted: a full watch link, a share
/// link, an embed link, or the bare id. None if it is not a YouTube video.
pub fn video_id(raw: &str) -> Option<String> {
    let t = raw.trim();
    let is_id = |s: &str| s.len() == 11 && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if is_id(t) { return Some(t.to_string()); }
    let rest = t.strip_prefix("https://").or_else(|| t.strip_prefix("http://"))?;
    let rest = rest.strip_prefix("www.").or_else(|| rest.strip_prefix("m.")).unwrap_or(rest);
    let candidate = if let Some(r) = rest.strip_prefix("youtu.be/") {
        r.split(['?', '&', '#']).next().unwrap_or("")
    } else if let Some(r) = rest.strip_prefix("youtube.com/").or_else(|| rest.strip_prefix("youtube-nocookie.com/")) {
        if let Some(q) = r.strip_prefix("watch?") {
            q.split('&').find_map(|kv| kv.strip_prefix("v=")).unwrap_or("")
        } else {
            // /embed/ID, /shorts/ID, /live/ID, /v/ID
            r.split('/').nth(1).unwrap_or("").split(['?', '&', '#']).next().unwrap_or("")
        }
    } else {
        return None;
    };
    is_id(candidate).then(|| candidate.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_cs_topic_has_an_intro_video() {
        for code in crate::course::spec_refs_for("cs") {
            assert!(!for_topic(&format!("cs:{code}")).is_empty(), "cs:{code} has no video");
        }
    }

    /// Subjects whose every topic has a video to start from.
    #[test]
    fn covered_subjects_have_a_video_on_every_topic() {
        for subj in ["maths", "fpm", "bio", "chem", "phys", "bus", "econ", "englit", "englang", "fre", "spa", "ger", "geog", "hist", "rs", "maths_edx", "maths_aqa", "maths_ocr", "chem_aqa", "bio_aqa", "phys_aqa"] {
            let def = crate::plan::SUBJECTS.iter().find(|d| d.id == subj).unwrap();
            for (code, _, _) in def.topics {
                assert!(!for_topic(&format!("{subj}:{code}")).is_empty(), "{subj}:{code} has no video");
            }
        }
    }

    #[test]
    fn video_topics_exist_in_the_plan() {
        for (topic, _) in VIDEOS {
            let (subj, code) = topic.split_once(':').unwrap();
            let def = crate::plan::SUBJECTS.iter().find(|d| d.id == subj).unwrap_or_else(|| panic!("{topic}: no subject {subj}"));
            assert!(def.topics.iter().any(|(c, _, _)| *c == code), "{topic}: no such topic in the plan");
        }
    }

    /// A video may rightly serve two topics (completing the square is both
    /// algebra and quadratics), but never twice on the same topic.
    #[test]
    fn ids_are_well_formed_and_unique_per_topic() {
        for (topic, vs) in VIDEOS {
            let mut seen = std::collections::HashSet::new();
            for (id, title, by) in *vs {
                assert_eq!(video_id(id).as_deref(), Some(*id), "{topic}: bad id {id}");
                assert!(seen.insert(*id), "{topic}: {id} listed twice");
                assert!(!title.trim().is_empty() && !by.trim().is_empty(), "{topic}: {id} has no title or creator");
            }
        }
    }

    #[test]
    fn parses_the_links_people_paste() {
        for raw in [
            "7Up7DIPkTzo",
            "https://www.youtube.com/watch?v=7Up7DIPkTzo",
            "https://www.youtube.com/watch?list=PLx&v=7Up7DIPkTzo&t=12s",
            "https://youtu.be/7Up7DIPkTzo?si=abc",
            "https://m.youtube.com/watch?v=7Up7DIPkTzo",
            "https://www.youtube-nocookie.com/embed/7Up7DIPkTzo?rel=0",
            "https://youtube.com/shorts/7Up7DIPkTzo",
            "  https://www.youtube.com/watch?v=7Up7DIPkTzo  ",
        ] {
            assert_eq!(video_id(raw).as_deref(), Some("7Up7DIPkTzo"), "{raw}");
        }
        for raw in ["", "https://vimeo.com/12345", "javascript:alert(1)", "https://www.youtube.com/", "not a link", "https://www.youtube.com/watch?v=short"] {
            assert_eq!(video_id(raw), None, "{raw}");
        }
    }
}
