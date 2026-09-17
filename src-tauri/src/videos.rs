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

/// (topic id, [(video id, title, creator)]) in the order they should be watched.
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
        for subj in ["maths", "fpm", "bio", "chem", "phys", "bus", "econ", "englit", "englang"] {
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
