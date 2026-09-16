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

    #[test]
    fn every_maths_topic_has_an_intro_video() {
        let def = crate::plan::SUBJECTS.iter().find(|d| d.id == "maths").unwrap();
        for (code, _, _) in def.topics {
            assert!(!for_topic(&format!("maths:{code}")).is_empty(), "maths:{code} has no video");
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
