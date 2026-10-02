//! The written course: what you must be able to do for each topic, and the
//! marks people drop on it.
//!
//! Keyed by topic id (`subject:speccode`) so it can be filled in subject by
//! subject without disturbing the topic lists in `plan.rs`. Anything missing
//! here simply falls back to the generic session steps. Everything is copied
//! into `PlanConfig` on first run and is editable from the Plan tab afterwards.
//!
//! Maths is written from the Edexcel International GCSE Mathematics A (4MA1)
//! specification, Issue 2, November 2017 — Higher Tier, which assumes all of
//! the Foundation Tier content as well. Every spec reference 1.1 to 6.3 is
//! covered; `plan::tests::every_maths_spec_reference_is_covered` proves it.

/// The official subject-content references for each specification, so the app
/// can show you its coverage rather than ask you to take it on trust. A subject
/// with no list here has not been checked against its spec yet, and the app
/// says so rather than implying the topic list is complete.
///
/// Only the reference codes are reproduced — the specifications themselves are
/// Pearson's copyright, so the app links to them rather than copying them.
pub const SPEC_REFS: &[(&str, &[&str])] = &[
    // Mathematics A (4MA1), Issue 2, November 2017 — Higher Tier.
    ("maths", &[
        "1.1", "1.2", "1.3", "1.4", "1.5", "1.6", "1.7", "1.8", "1.9", "1.10", "1.11",
        "2.1", "2.2", "2.3", "2.4", "2.5", "2.6", "2.7", "2.8",
        "3.1", "3.2", "3.3", "3.4",
        "4.1", "4.2", "4.3", "4.4", "4.5", "4.6", "4.7", "4.8", "4.9", "4.10", "4.11",
        "5.1", "5.2",
        "6.1", "6.2", "6.3",
    ]),
    // Further Pure Mathematics (4PM1), Issue 1 — single tier.
    ("fpm", &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]),
    // Business - AQA GCSE 8132, NOT Edexcel IGCSE 4BS1. Confirmed by the user,
    // who sits paper 8132/1. Six sections, 29 subsections, read from the
    // specification PDF (version 1.0, 19 August 2016).
    // Sections 3.1 and 3.2 are assessed on BOTH papers.
    ("bus", &[
        "3.1.1", "3.1.2", "3.1.3", "3.1.4", "3.1.5", "3.1.6", "3.1.7",
        "3.2.1", "3.2.2", "3.2.3", "3.2.4", "3.2.5", "3.2.6",
        "3.3.1", "3.3.2", "3.3.3", "3.3.4",
        "3.4.1", "3.4.2", "3.4.3", "3.4.4",
        "3.5.1", "3.5.2", "3.5.3", "3.5.4",
        "3.6.1", "3.6.2", "3.6.3", "3.6.4",
    ]),
    // Economics - Cambridge IGCSE (9-1) 0987, NOT Edexcel IGCSE 4EC1. Read from
    // the syllabus for exams in 2027, 2028 and 2029, which is the one that
    // applies to a 2028 sitting. Six sections, 31 subsections.
    ("econ", &[
        "1.1", "1.2", "1.3", "1.4",
        "2.1", "2.2", "2.3", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "2.10",
        "3.1", "3.2", "3.3", "3.4", "3.5", "3.6", "3.7",
        "4.1", "4.2", "4.3", "4.4", "4.5", "4.6", "4.7",
        "5.1", "5.2", "5.3", "5.4",
        "6.1", "6.2", "6.3", "6.4",
    ]),
    // Computer Science - OCR GCSE J277, NOT Edexcel IGCSE 4CP0. The user
    // corrected the board on 15 September 2026. Read from the specification,
    // version 3.1 (May 2026): 26 numbered sub-topics across 11 sections.
    ("cs", &[
        "1.1.1", "1.1.2", "1.1.3",
        "1.2.1", "1.2.2", "1.2.3", "1.2.4", "1.2.5",
        "1.3.1", "1.3.2",
        "1.4.1", "1.4.2",
        "1.5.1", "1.5.2",
        "1.6.1",
        "2.1.1", "2.1.2", "2.1.3",
        "2.2.1", "2.2.2", "2.2.3",
        "2.3.1", "2.3.2",
        "2.4.1",
        "2.5.1", "2.5.2",
    ]),
    // English Literature - AQA GCSE 8702, not Edexcel. The Power and Conflict
    // cluster is an AQA anthology, which is what identifies the board.
    ("englit", &["3.1.1", "3.1.2", "3.2.1", "3.2.2", "3.2.3", "3.3"]),
    // English Language - AQA GCSE 8700: Paper 1 sections A and B, Paper 2
    // sections A and B, and the Spoken Language endorsement.
    ("englang", &["1.1", "1.2", "2.1", "2.2", "3"]),
    // Biology (4BI1). Statements suffixed B are separate-science only - content
    // Double Award students skip and Triple Science students must cover.
    ("bio", &["1", "2", "3", "4", "5"]),
    // Chemistry (4CH1). Statements suffixed C are separate-science only.
    ("chem", &["1", "2", "3", "4"]),
    // Physics (4PH1). Statements suffixed P are separate-science only.
    ("phys", &["1", "2", "3", "4", "5", "6", "7", "8"]),
    // AQA GCSE French 8652, Spanish 8692 and German 8662 (first exams June 2026).
    // 3.1.1-3.1.3 are the three themes, 3.2.1 and 3.2.2 the Foundation and
    // Higher grammar, 3.2.3 the sound-symbol list (numbered in Spanish and
    // German; French lists it unnumbered, so the dictation topic 4.4b carries
    // it), and 4.4-4.7 the four papers.
    ("fre", &["3.1.1", "3.1.2", "3.1.3", "3.2.1", "3.2.2", "4.4", "4.5", "4.6", "4.7"]),
    ("spa", &["3.1.1", "3.1.2", "3.1.3", "3.2.1", "3.2.2", "3.2.3", "4.4", "4.5", "4.6", "4.7"]),
    ("ger", &["3.1.1", "3.1.2", "3.1.3", "3.2.1", "3.2.2", "3.2.3", "4.4", "4.5", "4.6", "4.7"]),
    // AQA GCSE Geography (8035). Options built: hot deserts, coasts and rivers, food; cold environments, glacial landscapes, water and energy are not.
    ("geog", &["3.1.1.1", "3.1.1.2", "3.1.1.3", "3.1.1.4", "3.1.2.1", "3.1.2.2", "3.1.2.3", "3.1.3.1", "3.1.3.2", "3.1.3.3", "3.2.1", "3.2.2", "3.2.3.1", "3.2.3.2", "3.3.1", "3.3.2", "3.4"]),
    // Pearson Edexcel GCSE History (1HI0). Options built: 11 (Medicine and the Western Front), B4, P4 and 31; the other options are not.
    ("hist", &["11.1", "11.2", "11.3", "11.4", "11.5", "B4.1", "B4.2", "B4.3", "P4.1", "P4.2", "P4.3", "31.1", "31.2", "31.3", "31.4"]),
    // Music - WJEC Eduqas GCSE (9-1) C660QS, specification "Version 4 October 2019"
    // (PDF eduqas-gcse-music-spec-from-2016-e-050225.pdf, posted 05/02/25), read
    // 29 September 2026. Only Component 3 Appraising (section 2.3) is examined in
    // writing. The spec has no numbered statements, so the references are its own
    // labels: ME Musical Elements, MC Musical Contexts, ML Musical Language and
    // Areas of study 1-4. Set works for 2027 and 2028: Bach, Badinerie (AoS1) and
    // Toto, Africa (AoS4). Appendix C (list of musical terms) is covered across ME.
    ("music", &["ME", "MC", "ML", "AoS1", "AoS2", "AoS3", "AoS4"]),
    // AQA GCSE Religious Studies A (8062). Route built: Christianity and Islam with Themes A, B, D and E; the other religions and themes are not.
    ("rs", &["3.1.2.1", "3.1.2.2", "3.1.5.1", "3.1.5.2", "3.2.1.1", "3.2.1.2", "3.2.1.4", "3.2.1.5"]),
    // AQA GCSE Drama 8261, specification version 1.8 (June 2026), read on
    // 29 September 2026. Only 3.1 Understanding drama is examined in writing
    // (Component 1); 3.2 Devising drama and 3.3 Texts in practice are practical
    // and not taught in the app. 3.1.2 is split into the four Section B question
    // types (a-e) and the nine set plays (f-n).
    ("drama", &["3.1.1", "3.1.2", "3.1.3"]),
    // Physical Education - AQA GCSE 8582, read from the specification PDF
    // version 1.7 (11 November 2025) on 29 September 2026. Written-exam content
    // only: 3.1 is Paper 1, 3.2 is Paper 2, and 3.1.4 Use of data is assessed on
    // both. 25 four-level references; 3.1.1.1, 3.1.1.2, 3.1.3.2, 3.1.3.3,
    // 3.1.3.4, 3.2.1.5 and 3.2.2.3 are split into lettered topics. The NEA
    // (section 4.4) is not examined and not listed.
    ("pe", &[
        "3.1.1.1", "3.1.1.2", "3.1.1.3", "3.1.1.4",
        "3.1.2.1", "3.1.2.2",
        "3.1.3.1", "3.1.3.2", "3.1.3.3", "3.1.3.4", "3.1.3.5",
        "3.1.4.1", "3.1.4.2", "3.1.4.3",
        "3.2.1.1", "3.2.1.2", "3.2.1.3", "3.2.1.4", "3.2.1.5",
        "3.2.2.1", "3.2.2.2", "3.2.2.3",
        "3.2.3.1", "3.2.3.2", "3.2.3.3",
    ]),
    // Media Studies - WJEC Eduqas GCSE (C680QS), specification version 10,
    // September 2025, read 29 September 2026. Section 2 is the theoretical
    // framework and the contexts of media, 2.1 Component 1 (Exploring the
    // Media) and 2.2 Component 2 (Understanding Media Forms and Products).
    // Component 3 (2.3) is non-exam assessment and is not taught in the app.
    ("media", &["2", "2.1", "2.2"]),
    // Design and Technology - AQA GCSE 8552. Read from the specification PDF,
    // version 1.2 (6 June 2022), on 29 September 2026. 3.1 core (3.1.6 is
    // numbered 3.1.6.1 material categories and 3.1.6.2 material properties),
    // 3.2 specialist (each taught through at least one material category or
    // system; Section B lets students choose theirs, so all six are covered),
    // 3.3 designing and making. The NEA (50%) is not taught in the app.
    ("dt", &[
        "3.1.1", "3.1.2", "3.1.3", "3.1.4", "3.1.5", "3.1.6.1", "3.1.6.2",
        "3.2.1", "3.2.2", "3.2.3", "3.2.4", "3.2.5", "3.2.6", "3.2.7", "3.2.8", "3.2.9",
        "3.3.1", "3.3.2", "3.3.3", "3.3.4", "3.3.5", "3.3.6", "3.3.7", "3.3.8", "3.3.9", "3.3.10", "3.3.11",
    ]),
    // Food Preparation and Nutrition - AQA GCSE 8585, read from the
    // specification PDF version 1.1, 21 January 2019 (read 29 September 2026).
    // The lowest-level numbered references in sections 3.2-3.6; 3.2.2.1,
    // 3.2.3.1 and 3.5.1.2 are split a/b. Section 3.1 (the twelve skill groups)
    // has no numbered content of its own and is folded into these topics;
    // 3.7 is assessed only through the NEA.
    ("food", &[
        "3.2.1.1", "3.2.1.2", "3.2.1.3",
        "3.2.2.1", "3.2.2.2", "3.2.2.3",
        "3.2.3.1", "3.2.3.2", "3.2.3.3", "3.2.3.4",
        "3.3.1.1", "3.3.1.2",
        "3.3.2.1", "3.3.2.2", "3.3.2.3", "3.3.2.4", "3.3.2.5",
        "3.4.1.1", "3.4.1.2", "3.4.1.3", "3.4.1.4",
        "3.4.2.1", "3.4.2.2",
        "3.5.1.1", "3.5.1.2", "3.5.1.3",
        "3.5.2", "3.5.3",
        "3.6.1.1", "3.6.1.2", "3.6.1.3",
        "3.6.2.1", "3.6.2.2",
    ]),
    // Pearson Edexcel GCSE Mathematics (1MA1) Higher. The DfE subject content references, Higher tier; the lessons are shared with AQA 8300.
    ("maths_edx", &["N1", "N2", "N3", "N4", "N5", "N6", "N7", "N8", "N9", "N10", "N11", "N12", "N13", "N14", "N15", "N16", "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "A9", "A10", "A11", "A12", "A13", "A14", "A15", "A16", "A17", "A18", "A19", "A20", "A21", "A22", "A23", "A24", "A25", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15", "R16", "G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8", "G9", "G10", "G11", "G12", "G13", "G14", "G15", "G16", "G17", "G18", "G19", "G20", "G21", "G22", "G23", "G24", "G25", "P1", "P2", "P3", "P4", "P5", "P6", "P7", "P8", "P9", "S1", "S2", "S3", "S4", "S5", "S6"]),
    // AQA GCSE Mathematics (8300) Higher. The DfE subject content references, Higher tier; the lessons are shared with Edexcel 1MA1.
    ("maths_aqa", &["N1", "N2", "N3", "N4", "N5", "N6", "N7", "N8", "N9", "N10", "N11", "N12", "N13", "N14", "N15", "N16", "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "A9", "A10", "A11", "A12", "A13", "A14", "A15", "A16", "A17", "A18", "A19", "A20", "A21", "A22", "A23", "A24", "A25", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15", "R16", "G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8", "G9", "G10", "G11", "G12", "G13", "G14", "G15", "G16", "G17", "G18", "G19", "G20", "G21", "G22", "G23", "G24", "G25", "P1", "P2", "P3", "P4", "P5", "P6", "P7", "P8", "P9", "S1", "S2", "S3", "S4", "S5", "S6"]),
    // OCR GCSE Mathematics (J560) Higher. OCR numbers this content 1.01 to 12.03; the topics use the DfE references it is written from, shared with Edexcel 1MA1 and AQA 8300.
    ("maths_ocr", &["N1", "N2", "N3", "N4", "N5", "N6", "N7", "N8", "N9", "N10", "N11", "N12", "N13", "N14", "N15", "N16", "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "A9", "A10", "A11", "A12", "A13", "A14", "A15", "A16", "A17", "A18", "A19", "A20", "A21", "A22", "A23", "A24", "A25", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15", "R16", "G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8", "G9", "G10", "G11", "G12", "G13", "G14", "G15", "G16", "G17", "G18", "G19", "G20", "G21", "G22", "G23", "G24", "G25", "P1", "P2", "P3", "P4", "P5", "P6", "P7", "P8", "P9", "S1", "S2", "S3", "S4", "S5", "S6"]),
    // AQA GCSE Biology (8461) Higher. Every subsection, Higher tier; lessons flag what Combined Science Trilogy leaves out.
    ("bio_aqa", &["4.1.1", "4.1.2", "4.1.3", "4.2.1", "4.2.2", "4.2.3", "4.3.1", "4.3.2", "4.3.3", "4.4.1", "4.4.2", "4.5.1", "4.5.2", "4.5.3", "4.5.4", "4.6.1", "4.6.2", "4.6.3", "4.6.4", "4.7.1", "4.7.2", "4.7.3", "4.7.4", "4.7.5"]),
    // English Language - WJEC Eduqas GCSE C700QS, specification "Version 3
    // January 2019" (eduqas-gcse-english-language-from-2015-e.pdf), read on
    // 30 September 2026. Section 2 Subject content: 2.1 Component 1 and 2.2
    // Component 2 are the two written exams; 2.3 Component 3 Spoken Language
    // is an unweighted endorsement and is not taught. Each reference is split
    // by letter into the paper's Section A questions and Section B writing.
    ("englang_edq", &["2.1", "2.2"]),
    // AQA GCSE Chemistry (8462) Higher. Every subsection, Higher tier; lessons flag what Combined Science Trilogy leaves out.
    ("chem_aqa", &["4.1.1", "4.1.2", "4.1.3", "4.2.1", "4.2.2", "4.2.3", "4.2.4", "4.3.1", "4.3.2", "4.3.3", "4.3.4", "4.3.5", "4.4.1", "4.4.2", "4.4.3", "4.5.1", "4.5.2", "4.6.1", "4.6.2", "4.7.1", "4.7.2", "4.7.3", "4.8.1", "4.8.2", "4.8.3", "4.9.1", "4.9.2", "4.9.3", "4.10.1", "4.10.2", "4.10.3", "4.10.4"]),
    // AQA GCSE Physics (8463) Higher. Every subsection, Higher tier; lessons flag what Combined Science Trilogy leaves out.
    ("phys_aqa", &["4.1.1", "4.1.2", "4.1.3", "4.2.1", "4.2.2", "4.2.3", "4.2.4", "4.2.5", "4.3.1", "4.3.2", "4.3.3", "4.4.1", "4.4.2", "4.4.3", "4.4.4", "4.5.1", "4.5.2", "4.5.3", "4.5.4", "4.5.5", "4.5.6", "4.5.7", "4.6.1", "4.6.2", "4.6.3", "4.7.1", "4.7.2", "4.7.3", "4.8.1", "4.8.2"]),
    // Economics - AQA GCSE 8136 (second board beside Cambridge 0987 "econ").
    // Read from the specification PDF, version 1.0 (21 July 2016; still the
    // current version on AQA's specification page), on 30 September 2026.
    // 39 four-level references: 3.1 (content 1-6) is Paper 1, 3.2 (content
    // 7-11) is Paper 2. No NEA. No reference is split.
    ("econ_aqa", &[
        "3.1.1.1", "3.1.1.2", "3.1.1.3",
        "3.1.2.1", "3.1.2.2", "3.1.2.3",
        "3.1.3.1", "3.1.3.2", "3.1.3.3", "3.1.3.4", "3.1.3.5", "3.1.3.6",
        "3.1.4.1", "3.1.4.2", "3.1.4.3",
        "3.1.5.1", "3.1.5.2", "3.1.5.3", "3.1.5.4",
        "3.1.6.1", "3.1.6.2",
        "3.2.1.1", "3.2.1.2",
        "3.2.2.1", "3.2.2.2", "3.2.2.3", "3.2.2.4", "3.2.2.5", "3.2.2.6",
        "3.2.3.1", "3.2.3.2", "3.2.3.3", "3.2.3.4",
        "3.2.4.1", "3.2.4.2", "3.2.4.3", "3.2.4.4",
        "3.2.5.1", "3.2.5.2",
    ]),
    // English Literature - WJEC Eduqas GCSE (9-1) C720QS, specification Version 4
    // (August 2024; "HT 12.08.2024"), read on 30 September 2026. The spec has no
    // numbered statements, so the references are its component sections: C1A
    // Shakespeare, C1B Poetry 1789 to the present day (the anthology for
    // assessment from 2027), C2A Post-1914 prose/drama, C2B 19th-century prose,
    // C2C Unseen poetry. Texts built: Macbeth, An Inspector Calls, A Christmas
    // Carol (the most-taught choices); the other set texts are not.
    ("englit_edq", &["C1A", "C1B", "C2A", "C2B", "C2C"]),
    // Business - Pearson Edexcel GCSE (9-1) 1BS0, specification Issue 2 (July
    // 2022), read on 30 September 2026. Theme 1 (Paper 1) has five topics and
    // 21 subsections, Theme 2 (Paper 2) five topics and 19 subsections. The app
    // splits 1.3.2 into 1.3.2a and 1.3.2b.
    ("bus_edx", &[
        "1.1.1", "1.1.2", "1.1.3",
        "1.2.1", "1.2.2", "1.2.3", "1.2.4",
        "1.3.1", "1.3.2", "1.3.3", "1.3.4",
        "1.4.1", "1.4.2", "1.4.3", "1.4.4",
        "1.5.1", "1.5.2", "1.5.3", "1.5.4", "1.5.5",
        "2.1.1", "2.1.2", "2.1.3", "2.1.4",
        "2.2.1", "2.2.2", "2.2.3", "2.2.4", "2.2.5",
        "2.3.1", "2.3.2", "2.3.3", "2.3.4",
        "2.4.1", "2.4.2",
        "2.5.1", "2.5.2", "2.5.3", "2.5.4",
    ]),
    // Computer Science - AQA GCSE 8525, the updated specification for first
    // teaching in September 2025, first exams June 2027 (spec dated 16 June
    // 2025; read from AQA's online specification pages on 30 September 2026).
    // The lowest-level numbered references in 3.1-3.8: 3.5 and 3.8 have no
    // sub-numbers. 3.1.1, 3.2.2, 3.2.11, 3.4.5 and 3.5 are split by letter.
    ("cs_aqa", &[
        "3.1.1", "3.1.2", "3.1.3", "3.1.4",
        "3.2.1", "3.2.2", "3.2.3", "3.2.4", "3.2.5", "3.2.6", "3.2.7", "3.2.8", "3.2.9", "3.2.10", "3.2.11",
        "3.3.1", "3.3.2", "3.3.3", "3.3.4", "3.3.5", "3.3.6", "3.3.7", "3.3.8",
        "3.4.1", "3.4.2", "3.4.3", "3.4.4", "3.4.5",
        "3.5",
        "3.6.1", "3.6.2", "3.6.2.1", "3.6.2.2", "3.6.3",
        "3.7.1", "3.7.2",
        "3.8",
    ]),
    // History - AQA GCSE 8145, read from the specification PDF version 1.3
    // (24 September 2019) on 30 September 2026. AQA numbers nothing below the
    // option, so each reference is paper + option code + Part: 1AB.1 is Paper 1
    // Section A option AB (Germany) Part one. Only the eight options covered
    // in the app are listed (the two most-taken in each section); the other
    // eight options (1AA, 1AC, 1BA, 1BD, 1BE, 2AC, 2BB, 2BD) are not covered.
    ("hist_aqa", &[
        "1AB.1", "1AB.2", "1AB.3",
        "1AD.1", "1AD.2", "1AD.3",
        "1BB.1", "1BB.2", "1BB.3",
        "1BC.1", "1BC.2", "1BC.3",
        "2AA.1", "2AA.2", "2AA.3", "2AA.4",
        "2AB.1", "2AB.2", "2AB.3", "2AB.4",
        "2BA.1", "2BA.2", "2BA.3", "2BA.4",
        "2BC.1", "2BC.2", "2BC.3", "2BC.4",
    ]),
    // WJEC Eduqas GCSE (9-1) Religious Studies C120QS, specification version 5
    // (September 2026), read on 30 September 2026. Route A only: 2.1 Component 1
    // (four themes, Christian and Islamic perspectives plus non-religious views
    // in Theme 2), 2.2 Component 2 (Christianity), 2.3 Component 3 option 3
    // (Islam). The spec has no numbered statements below these sections, so
    // topics are letter-suffixed splits of them.
    ("rs_edq", &["2.1", "2.2", "2.3"]),
    // Geography B - Pearson Edexcel GCSE (9-1) 1GB0, specification Issue 4,
    // September 2025 (gcse-2016-l12-geography-b-spec.pdf), read 30 September
    // 2026. The 53 numbered key ideas 1.1-9.6, plus Topic 6 (Geographical
    // investigations), whose four fieldwork contexts have no numbers and are
    // split 6a-6d. Integrated skills are numbered within each key idea and are
    // taught in that key idea's topic; the Paper 3 Section D decision is taught
    // in 9.6.
    ("geog_edxb", &[
        "1.1", "1.2", "1.3", "1.4", "1.5", "1.6", "1.7", "1.8", "1.9",
        "2.1", "2.2", "2.3", "2.4", "2.5", "2.6", "2.7",
        "3.1", "3.2", "3.3", "3.4", "3.5", "3.6", "3.7",
        "4.1", "4.2", "4.3", "4.4", "4.5", "4.6", "4.7", "4.8",
        "5.1", "5.2", "5.3", "5.4", "5.5", "5.6", "5.7", "5.8",
        "6",
        "7.1", "7.2",
        "8.1", "8.2", "8.3", "8.4", "8.5", "8.6",
        "9.1", "9.2", "9.3", "9.4", "9.5", "9.6",
    ]),
    // AQA GCSE Combined Science: Trilogy (8464) Higher. Every Trilogy subsection, Higher tier; each topic opens the matching separate-science lesson, which marks what Trilogy leaves out as Separate science only.
    ("combsci_aqa", &["4.1.1", "4.1.2", "4.1.3", "4.2.1", "4.2.2", "4.2.3", "4.3.1", "4.4.1", "4.4.2", "4.5.1", "4.5.2", "4.5.3", "4.6.1", "4.6.2", "4.6.3", "4.6.4", "4.7.1", "4.7.2", "4.7.3", "5.1.1", "5.1.2", "5.2.1", "5.2.2", "5.2.3", "5.3.1", "5.3.2", "5.4.1", "5.4.2", "5.4.3", "5.5.1", "5.6.1", "5.6.2", "5.7.1", "5.8.1", "5.8.2", "5.9.1", "5.9.2", "5.9.3", "5.10.1", "5.10.2", "6.1.1", "6.1.2", "6.1.3", "6.2.1", "6.2.2", "6.2.3", "6.2.4", "6.3.1", "6.3.2", "6.3.3", "6.4.1", "6.4.2", "6.5.1", "6.5.2", "6.5.3", "6.5.4", "6.5.5", "6.6.1", "6.6.2", "6.7.1", "6.7.2"]),
    // Economics - OCR GCSE (9-1) J205 (second board beside Cambridge 0987
    // "econ" and AQA 8136 "econ_aqa"). Read from the specification PDF,
    // version 2.0 (June 2026; branding and admin changes only, content
    // unchanged since first teaching in 2017), on 2 October 2026. 22 topic
    // numbers: 1-2 are J205/01, 3-4 are J205/02. No NEA. 2.1, 2.2, 2.3, 2.6,
    // 2.8, 3.5 and 4.4 are split by letter.
    ("econ_ocr", &[
        "1.1", "1.2",
        "2.1", "2.2", "2.3", "2.4", "2.5", "2.6", "2.7", "2.8",
        "3.1", "3.2", "3.3", "3.4", "3.5", "3.6", "3.7", "3.8",
        "4.1", "4.2", "4.3", "4.4",
    ]),
    // French - Pearson Edexcel GCSE 1FR1 (second board beside AQA 8652 "fre").
    // Read from the specification, Issue 2 (May 2025; first teaching 2024,
    // first exams June 2026; the current issue on Pearson's qualification
    // page), on 2 October 2026. The spec numbers none of its content, so these
    // references follow its own order:
    // T1-T6 the six thematic contexts as listed on page 7 (My personal world;
    // Lifestyle and wellbeing; My neighbourhood; Media and technology;
    // Studying and my future; Travel and tourism);
    // G1-G8 the eight sections of Appendix 2: Grammar (nouns, pronouns and
    // determiners; verbs; verbs: tenses; adjectives; adverbs; prepositions;
    // derivational morphology; sound-symbol correspondences);
    // P1-P4 Pearson's paper numbers (Speaking, Listening, Reading, Writing).
    // G1-G3 are split with letters; topic G4-5 covers G4 and G5.
    ("fre_edx", &[
        "T1", "T2", "T3", "T4", "T5", "T6",
        "G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8",
        "P1", "P2", "P3", "P4",
    ]),
    // German - Pearson Edexcel GCSE (9-1) 1GN1 (second board beside AQA 8662
    // "ger"). Read from the specification PDF, Issue 2 (May 2025;
    // gq000025-gcse-german-specification-2024-issue-2.pdf), on 2 October 2026.
    // Pearson numbers nothing, so the references are labels in the spec's own
    // order: T1-T6 the six thematic contexts; G1-G7 the sections of Appendix 2
    // Grammar (Nouns, pronouns and determiners; Verbs and tenses; Adjectives
    // and Adverbs, taught together as G3-4; Prepositions; Derivational
    // morphology; Sound-symbol correspondences); P1-P4 the four papers.
    // Higher tier. G1 and G2 are split by letter.
    ("ger_edx", &[
        "T1", "T2", "T3", "T4", "T5", "T6",
        "G1", "G2", "G3-4", "G5", "G6", "G7",
        "P1", "P2", "P3", "P4",
    ]),
    // Spanish - Pearson Edexcel GCSE 1SP1 (2024 specification), Higher tier.
    // Read from the specification, Issue 2 (May 2025), on 2 October 2026.
    // Edexcel does not number its content, so the references are the app's own:
    // T1-T6 the six thematic contexts in the spec's order; G1a-G8 Appendix 2
    // (Grammar) in order - nouns, pronouns and determiners; verbs; tenses;
    // adjectives and adverbs; prepositions; derivational morphology;
    // sound-symbol correspondences and stress; P1-P4 one per paper.
    ("spa_edx", &[
        "T1", "T2", "T3", "T4", "T5", "T6",
        "G1a", "G1b", "G1c", "G2a", "G2b", "G2c",
        "G3a", "G3b", "G3c", "G3d", "G3e",
        "G4-5", "G6", "G7", "G8",
        "P1", "P2", "P3", "P4",
    ]),
];

/// The official reference list for a subject, or empty if it has not been
/// checked against its specification yet.
pub fn spec_refs_for(subject_id: &str) -> &'static [&'static str] {
    SPEC_REFS.iter().find(|(id, _)| *id == subject_id).map(|(_, r)| *r).unwrap_or(&[])
}

/// Whether a topic code covers a spec reference. A topic covers it if the code
/// is that reference, or that reference plus a letter suffix — 4.8 is split
/// into 4.8a, 4.8b and 4.8c. The suffix must be letters, so "1.10" is never
/// mistaken for part of "1.1".
///
/// A topic may also span a run of references written as a range: "N1-3"
/// covers N1, N2 and N3 (the DfE GCSE Maths strands N, A, R, G, P, S number
/// their statements this way), optionally with a letter suffix ("A18-19b").
pub fn covers(code: &str, spec_ref: &str) -> bool {
    code == spec_ref
        || (code.starts_with(spec_ref)
            && !code[spec_ref.len()..].is_empty()
            && code[spec_ref.len()..].chars().all(|c| c.is_ascii_alphabetic()))
        || covers_range(code, spec_ref)
}

fn covers_range(code: &str, spec_ref: &str) -> bool {
    let Some((from, to)) = code.split_once('-') else { return false };
    let prefix: String = from.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
    let (Ok(lo), Ok(hi)) = (
        from[prefix.len()..].parse::<u32>(),
        to.trim_end_matches(|c: char| c.is_ascii_alphabetic()).parse::<u32>(),
    ) else { return false };
    if prefix.is_empty() || !spec_ref.starts_with(&prefix) { return false; }
    spec_ref[prefix.len()..].parse::<u32>().is_ok_and(|k| (lo..=hi).contains(&k))
}

/// (topic id, objectives, the mark people drop)
pub const TOPIC_DETAIL: &[(&str, &[&str], &str)] = &[
    // ---------- 1 Numbers and the number system ----------
    ("maths:1.1", &[
        "Use the four rules with negative numbers, and apply the correct order of operations",
        "Write any integer as a product of prime factors in index form",
        "Find the HCF and LCM of two or more numbers from their prime factorisations",
        "Solve worded problems that hide an HCF or an LCM, such as repeating events or cutting into equal pieces",
    ], "A worded LCM question rarely uses the word LCM. If two things repeat on different cycles and you need when they next coincide, that is an LCM."),

    ("maths:1.2", &[
        "Add, subtract, multiply and divide fractions and mixed numbers without a calculator",
        "Simplify a fraction to lowest terms, and order a list of fractions using a common denominator",
        "Express one number as a fraction of another, and find a fraction of a quantity",
        "Convert between fractions, decimals and percentages fluently",
    ], "Turn mixed numbers into improper fractions before multiplying or dividing. Multiplying the whole parts separately is the classic wrong answer."),

    ("maths:1.3", &[
        "Order decimals and convert between decimals, fractions and percentages",
        "Recognise which fractions give terminating decimals and which give recurring ones",
        "Convert a recurring decimal to a fraction using the 10x / 100x subtraction method",
        "Handle recurring decimals where the repeating part does not start immediately, such as 0.4166…",
    ], "The method marks are in the subtraction line. Write out 100x − 10x explicitly, then solve — an answer alone scores a fraction of the marks."),

    ("maths:1.4a", &[
        "Apply the index laws for multiplication, division and a power of a power",
        "Evaluate expressions with negative and fractional indices, such as 16^(−3/4) and 8^(2/3)",
        "Move between root form and index form confidently",
        "Solve equations with the unknown in the index by writing both sides to the same base",
    ], "A negative index means reciprocal, not a negative answer. 2^(−3) is 1/8, never −8."),

    ("maths:1.4b", &[
        "Simplify a surd by extracting square factors, e.g. √32 = 4√2",
        "Add, subtract and multiply surds and simplify the result",
        "Expand brackets containing surds and give the answer in the form a + b√c",
        "Rationalise a denominator, including two-term denominators using the conjugate",
    ], "For a denominator like 2 − √3, multiply top and bottom by 2 + √3. Multiplying by the same expression instead leaves a surd behind."),

    ("maths:1.5", &[
        "Use ∈, ⊆, ∪, ∩, ξ and A′ correctly, including sets defined algebraically",
        "Use n(A) for the number of elements in a set",
        "Complete a two- or three-set Venn diagram from worded information, starting at the centre",
        "Read counts and probabilities off a Venn diagram",
    ], "Fill the intersection of all the sets first and work outwards. Starting at the edges makes every other region wrong."),

    ("maths:1.6", &[
        "Use a multiplier for any percentage increase or decrease",
        "Calculate compound interest and depreciation over several years",
        "Work backwards from a changed amount to the original using reverse percentage",
        "Handle repeated change with a different rate each year, and find the overall percentage change",
    ], "Reverse percentage means dividing by the multiplier. If a price is £60 after a 20% rise, the original is 60 ÷ 1.2 = £50, not 60 minus 20%."),

    ("maths:1.7", &[
        "Simplify a ratio and write it in the form 1:n",
        "Divide a quantity in a two- or three-part ratio",
        "Solve problems where only the difference between two shares is given",
        "Use ratio with maps and scale drawings, and solve direct proportion problems",
    ], "When the question gives the difference between shares, divide by the difference in ratio parts, not by the total."),

    ("maths:1.8", &[
        "Round to a given number of significant figures or decimal places",
        "Write the upper and lower bounds of a value rounded to a given accuracy",
        "Combine bounds correctly for +, −, × and ÷ — for subtraction and division, pair max with min",
        "Estimate a calculation by rounding each value to 1 significant figure",
    ], "The lower bound of a division needs the smallest numerator over the largest denominator. Getting that pairing the wrong way round is the standard dropped mark."),

    ("maths:1.9", &[
        "Convert to and from standard form for both large and small numbers",
        "Multiply and divide in standard form, fixing the mantissa so that 1 ≤ a < 10",
        "Add and subtract in standard form by writing both numbers to the same power first",
        "Solve problems set in context using standard form",
    ], "34 × 10⁵ is not in standard form and loses the final mark. Adjust it to 3.4 × 10⁶."),

    ("maths:1.10", &[
        "Calculate with standard units of mass, length, area, volume and capacity, converting where needed",
        "Work with time on both the 12-hour and 24-hour clock, including timetables and intervals",
        "Solve money problems, including converting between currencies with a given exchange rate",
        "Answer everyday problems — best buys, bills, wages — showing the working",
    ], "An exchange rate is quoted one way round. Decide whether you multiply or divide before you touch the calculator."),

    ("maths:1.11", &[
        "Use the fraction, power, root and bracket keys to evaluate an expression in one go",
        "Use the memory and answer keys instead of writing down rounded intermediate values",
        "Switch between exact and decimal answers, and give the form the question asks for",
        "Check an answer is sensible with a quick mental estimate",
    ], "Rounding partway through and retyping the rounded number loses accuracy marks. Keep the full value in the calculator and round only at the end."),

    // ---------- 2 Equations, formulae and identities ----------
    ("maths:2.1", &[
        "Use index notation with positive, negative, zero and fractional powers",
        "Apply the index laws to algebraic terms, e.g. simplify 6x³ ÷ 2x⁻¹",
        "Tell the difference between an expression, an equation, a formula and an identity",
    ], "(3x)² is 9x², not 3x². The power applies to everything inside the bracket, coefficient included."),

    ("maths:2.2a", &[
        "Expand and simplify the product of two or three linear brackets",
        "Factorise fully by taking out the highest common factor",
        "Factorise quadratics where the coefficient of x² is not 1",
        "Recognise and use the difference of two squares",
    ], "Factorise fully. Taking 2 out of 8xy + 12y² when you could take out 4y earns only part of the mark."),

    ("maths:2.2b", &[
        "Simplify an algebraic fraction by factorising the top and the bottom before cancelling",
        "Add, subtract, multiply and divide algebraic fractions, giving a single fraction",
        "Complete the square to write a quadratic as a(x + b)² + c",
        "Construct an algebraic proof, such as showing the sum of three consecutive integers is a multiple of 3",
    ], "You can only cancel a factor of the whole numerator with a factor of the whole denominator. Cancelling a term inside a sum is the most common error on the paper."),

    ("maths:2.3", &[
        "Substitute negative and fractional values into a formula accurately",
        "Derive a formula from a worded or diagrammatic description",
        "Change the subject of a formula, including when the new subject appears twice",
        "Rearrange formulae involving powers, roots and fractions",
    ], "When the subject appears twice, gather those terms on one side and factorise. Moving them one at a time never finishes."),

    ("maths:2.4", &[
        "Solve linear equations with brackets, fractions and the unknown on both sides",
        "Clear fractions by multiplying every term by the lowest common denominator",
        "Form an equation from a worded or geometric situation and solve it",
    ], "Multiply every term by the denominator, including the ones that are already whole numbers."),

    ("maths:2.5", &[
        "Set up y = kx, y = kx², y = k/x, y = k/x² or y = k√x from a worded statement",
        "Find k from one pair of values, then use the complete equation",
        "Say what happens to y when x is doubled or tripled",
        "Match each type of proportion to the shape of its graph",
    ], "Find k and write the full equation before answering. Scaling by a ratio works for direct proportion but gives the wrong answer for squares and inverses."),

    ("maths:2.6", &[
        "Solve a pair of linear simultaneous equations by elimination",
        "Solve them by substitution, and pick whichever is quicker for the numbers given",
        "Interpret the solution as the point where two lines cross",
        "Form a pair of simultaneous equations from a worded problem",
    ], "When you eliminate by adding or subtracting, watch the signs across the whole second equation. A sign slip here costs every mark that follows."),

    ("maths:2.7a", &[
        "Solve a quadratic by factorising, including when the coefficient of x² is not 1",
        "Solve by the quadratic formula, giving answers to the accuracy asked for",
        "Solve by completing the square, and use it to find the turning point",
        "Recognise which method the question is steering you towards",
    ], "\"Give your answers to 2 decimal places\" means use the formula. \"Solve exactly\" or \"in surd form\" means factorise or complete the square."),

    ("maths:2.7b", &[
        "Form a quadratic equation from a worded or geometric context and solve it",
        "Reject solutions that make no sense in context, and say why",
        "Solve one linear and one quadratic equation simultaneously by substitution",
        "Give both solution pairs, matching each x with its own y",
    ], "If the answer is a length, an age or a time, reject the negative root and state that you are rejecting it. The mark is for the rejection."),

    ("maths:2.8", &[
        "Solve linear inequalities, reversing the sign when multiplying or dividing by a negative",
        "Represent a solution set on a number line with open and closed circles",
        "Solve a quadratic inequality by sketching the parabola, e.g. x² > 25 or x² + 3x + 2 > 0",
        "Shade a region defined by several linear inequalities, using dashed and solid lines correctly",
    ], "For a quadratic inequality, sketch it. Deciding whether the answer is inside or outside the roots by instinct is where most of these marks go."),

    // ---------- 3 Sequences, functions and graphs ----------
    ("maths:3.1a", &[
        "Generate terms from a term-to-term or a position-to-term rule",
        "Find the nth term of a linear sequence",
        "Find the nth term of a quadratic sequence using second differences",
        "Decide whether a given number appears in a sequence by solving",
    ], "For a quadratic sequence the coefficient of n² is half the second difference, not the second difference itself."),

    ("maths:3.1b", &[
        "Identify the first term a and the common difference d, including from two given terms",
        "Use nth term = a + (n − 1)d to find any term of an arithmetic sequence",
        "Use the sum formula to find the total of the first n terms of an arithmetic series",
        "Work backwards: find n from a given sum, or find a and d from two given terms",
    ], "The nth term formula uses (n − 1)d, not nd. Dropping the −1 shifts every answer by one common difference."),

    ("maths:3.2", &[
        "Use f(x) and f: x ↦ notation, and evaluate f(a) for any value",
        "State the domain and range, and which values must be excluded from a domain",
        "Find and simplify a composite function fg(x), doing g first",
        "Find an inverse function by rearranging y = f(x)",
    ], "fg(x) means do g first, then f. Working left to right is the single most common error on this topic."),

    ("maths:3.3a", &[
        "Find the gradient and the equation of a straight line through two given points",
        "Use y = mx + c to read off the gradient and intercept, and to write a line's equation",
        "Find the midpoint of a line segment from the coordinates of its ends",
        "Find the equation of a line parallel or perpendicular to a given line through a given point",
    ], "Perpendicular gradients multiply to −1, so you need the negative reciprocal. Just flipping the sign is not enough."),

    ("maths:3.3b", &[
        "Complete a table of values and plot a cubic or reciprocal graph accurately",
        "Recognise quadratic, cubic, reciprocal, exponential and trigonometric graphs on sight",
        "Sketch y = sin x, y = cos x and y = tan x for angles of any size in degrees",
        "Identify the asymptotes of a reciprocal graph",
    ], "Reciprocal graphs never touch the axes and come in two separate branches. Joining them through the origin is wrong."),

    ("maths:3.3c", &[
        "Apply and describe the transformations f(x) + a, f(x + a), af(x) and f(ax)",
        "Write the transformed function algebraically",
        "Estimate the gradient of a curve at a point by drawing a tangent",
        "Use the intersection of two graphs to solve an equation, and state which equation it solves",
    ], "f(x + a) shifts the graph left by a. Inside-the-bracket transformations go the opposite way to the sign."),

    ("maths:3.4", &[
        "Differentiate expressions made of integer powers of x",
        "Find the gradient at a point, and the equation of the tangent or normal there",
        "Find stationary points and decide whether each is a maximum or a minimum",
        "Differentiate displacement to velocity and again to acceleration, and solve kinematics problems",
    ], "Deciding maximum or minimum needs a justification — the shape of the graph or the sign either side. Labelling it without saying why scores nothing."),

    // ---------- 4 Geometry and trigonometry ----------
    ("maths:4.1", &[
        "Use angles at a point, angles on a straight line and vertically opposite angles",
        "Use alternate, corresponding and allied angles on parallel lines, naming each",
        "Use the angle sum and the exterior angle property of a triangle",
        "Use the angle properties of isosceles and equilateral triangles",
    ], "Name the reason properly — \"alternate angles are equal\", not \"Z angles\". The examiner wants the standard term."),

    ("maths:4.2", &[
        "Calculate the interior and exterior angles of regular and irregular polygons",
        "Use the angle sum of a quadrilateral and of an n-sided polygon",
        "Use the side, angle and diagonal properties of the special quadrilaterals",
        "Recognise congruent shapes and explain why they are congruent",
    ], "Exterior angles of any polygon add to 360°. Reaching for the interior formula when one division would do wastes time and invites slips."),

    ("maths:4.3", &[
        "Identify every line of symmetry in a two-dimensional shape",
        "State the order of rotational symmetry of a shape",
        "Use symmetry properties to name a quadrilateral from a description",
    ], "Every shape has rotational symmetry of at least order 1. Writing \"none\" instead of \"order 1\" is marked wrong."),

    ("maths:4.4", &[
        "Use speed = distance ÷ time, density = mass ÷ volume and pressure = force ÷ area in any direction",
        "Convert compound units, e.g. m/s to km/h and g/cm³ to kg/m³",
        "Work with three-figure bearings, including back bearings",
        "Calculate time intervals using the 12-hour and 24-hour clock",
    ], "A bearing is always three figures, measured clockwise from north. 45° must be written 045°."),

    ("maths:4.5", &[
        "Construct a perpendicular bisector and an angle bisector with compasses",
        "Construct a triangle from given sides and angles",
        "Solve a problem using a scale drawing",
        "Identify and shade a locus satisfying one or more conditions at once",
    ], "Leave your construction arcs on the page. Rubbing them out loses the method marks even when the drawing is perfect."),

    ("maths:4.6", &[
        "Use tangent–radius perpendicularity, and that two tangents from a point are equal",
        "Use the fact that a perpendicular from the centre bisects a chord",
        "Apply the angle at the centre, angle in a semicircle, same segment, cyclic quadrilateral and alternate segment theorems",
        "Use the intersecting chord properties, both internal and external",
    ], "Name the theorem at every step. \"Angles in the same segment are equal\" scores; an unexplained number does not."),

    ("maths:4.7", &[
        "Give the standard geometrical reason for every step of an angle calculation",
        "Build a chain of reasoning across several steps to reach the required angle",
        "Use the right vocabulary: alternate, corresponding, allied, subtended, cyclic",
    ], "In a \"give reasons\" question the reasons carry most of the marks. A correct final answer with no reasons can score almost nothing."),

    ("maths:4.8a", &[
        "Use Pythagoras to find any side of a right-angled triangle",
        "Use sine, cosine and tangent to find a missing side or angle",
        "Choose the right ratio by labelling opposite, adjacent and hypotenuse first",
        "Solve problems involving angles of elevation and depression",
    ], "To find an angle you need the inverse function. Forgetting sin⁻¹ and reading off sin instead gives a plausible-looking wrong answer."),

    ("maths:4.8b", &[
        "Use the sine rule to find a missing side or angle",
        "Use the cosine rule with three sides, or with two sides and the included angle",
        "Use ½ab sin C to find the area of any triangle",
        "Work with obtuse angles, including the ambiguous case of the sine rule",
    ], "Cosine rule when you have two sides and the angle between them, or all three sides. Sine rule otherwise. Choosing wrong costs the whole question."),

    ("maths:4.8c", &[
        "Find the length of a diagonal in a cuboid by using Pythagoras twice",
        "Find the angle between a line and a plane",
        "Find angles and lengths inside pyramids and prisms",
        "Redraw the right-angled triangle you are using as a separate 2D sketch before calculating",
    ], "Work in 2D. Draw out the single triangle you are actually using and label it — reasoning inside the 3D picture is where this falls apart."),

    ("maths:4.9", &[
        "Find the perimeter and area of compound shapes made from rectangles and triangles",
        "Find the area of parallelograms and trapezia",
        "Find the circumference and area of circles and semicircles",
        "Find arc length and sector area, and a segment area by subtracting a triangle from a sector",
    ], "The perimeter of a sector includes the two radii as well as the arc. Giving just the arc length is the classic error."),

    ("maths:4.10", &[
        "Find the volume of prisms, cylinders, pyramids, cones and spheres",
        "Find the surface area of cylinders, cones and spheres",
        "Work with composite solids and with frustums",
        "Convert between units of volume and capacity, e.g. cm³ to m³ and 1 litre = 1000 cm³",
    ], "Cone surface area uses the slant height, not the vertical height. Find the slant with Pythagoras first."),

    ("maths:4.11", &[
        "Prove two shapes are similar and find missing lengths with a scale factor",
        "Apply the area scale factor k² and the volume scale factor k³",
        "Work backwards from a ratio of areas or volumes to the ratio of lengths",
        "Solve problems with similar triangles sitting inside a larger figure",
    ], "If lengths are in ratio 2:3 then areas are 4:9 and volumes 8:27. Using the length ratio for area or volume is the standard trap."),

    // ---------- 5 Vectors and transformation geometry ----------
    ("maths:5.1", &[
        "Add, subtract and scale vectors in column notation and in a/b notation",
        "Find the magnitude (modulus) of a vector",
        "Express a journey across a shape as a resultant of the given vectors",
        "Use vectors to prove points are collinear, or that two lines are parallel",
    ], "To prove collinear, show one vector is a scalar multiple of the other and say they share a common point. The shared point is a mark in itself."),

    ("maths:5.2", &[
        "Reflect a shape in a given mirror line, including y = x and y = −x",
        "Rotate a shape about a given centre through a given angle and direction",
        "Translate a shape using a column vector",
        "Enlarge by a given scale factor about a centre, including negative and fractional factors, and describe any transformation fully",
    ], "A description must be complete. A rotation needs angle, direction and centre — miss one and the mark is gone."),

    // ---------- 6 Statistics and probability ----------
    ("maths:6.1a", &[
        "Draw and read bar charts, pie charts and pictograms",
        "Complete and interpret a two-way table",
        "Interpret a statistical diagram and say what it shows in context",
    ], "A pie chart angle is the frequency divided by the total, times 360. Using a percentage straight as degrees is wrong."),

    ("maths:6.1b", &[
        "Calculate frequency density and draw a histogram with unequal class widths",
        "Find a frequency from a histogram by working out an area",
        "Estimate how many values fall in part of a class by taking the matching part of the bar",
        "Complete a partly-drawn histogram from a frequency table",
    ], "Bar height is frequency density, not frequency. Reading heights as frequencies is the single biggest loss of marks on this topic."),

    ("maths:6.1c", &[
        "Build a cumulative frequency table and plot the points at the upper class boundaries",
        "Draw a smooth cumulative frequency curve",
        "Read off the median, the lower and upper quartiles, and the interquartile range",
        "Use the curve to estimate how many values fall above or below a given figure",
    ], "Plot cumulative frequency at the upper boundary of each class, not the midpoint. Plotting at midpoints skews every reading you take."),

    ("maths:6.2", &[
        "Find the mean, median, mode and range of a discrete data set",
        "Estimate the mean of grouped data using class midpoints",
        "Identify the modal class and the class containing the median",
        "Find the interquartile range, and compare two distributions using an average and a measure of spread",
    ], "Comparing two sets needs both an average and a spread, written as a sentence in context. Quoting the means alone loses half the marks."),

    ("maths:6.3a", &[
        "List the outcomes of one or two events systematically, using a sample space diagram",
        "Use P(not A) = 1 − P(A)",
        "Use the addition rule for mutually exclusive events",
        "Calculate expected frequency over a number of trials, and estimate probability from collected data",
    ], "Expected frequency means probability × number of trials. Giving the probability itself as the answer is a common slip."),

    ("maths:6.3b", &[
        "Draw a tree diagram for two or three events and label every branch",
        "Multiply along branches and add across the outcomes that satisfy the question",
        "Handle without-replacement problems, where the denominator changes",
        "Calculate conditional probability from a tree diagram, a Venn diagram or a table",
    ], "Without replacement means the second denominator drops by one, and often the numerator too. Reusing the same fractions is the most common error in the whole topic."),

    // ---------- Further Pure Mathematics (4PM1) ----------
    ("fpm:1a", &[
        "Recognise the shapes of the graphs of aˣ and log_b x",
        "Use the log laws: log(xy) = log x + log y, log(x/y) = log x − log y, log xᵏ = k log x",
        "Use log_a a = 1 and log_a 1 = 0",
        "Solve equations of the form aˣ = b, changing base where it helps",
    ], "log x + log y is log(xy), not log(x + y). Adding the arguments instead of multiplying them is the standard slip."),

    ("fpm:1b", &[
        "Apply the index laws with negative and fractional powers",
        "Simplify surds by extracting square factors, e.g. √48 = 4√3",
        "Add, subtract and multiply surds",
        "Rationalise a denominator, including the form 1/(2 − √3)",
    ], "√48 is 4√3, not 16√3. Take the square root of the factor you pull out, do not carry it across whole."),

    ("fpm:2", &[
        "Factorise a quadratic expression and complete the square",
        "Use the discriminant to say whether the roots are equal, real and distinct, or not real",
        "Use α + β = −b/a and αβ = c/a",
        "Form a new quadratic whose roots are expressions in α and β",
    ], "For an equation with roots α² and β², build the new sum and product out of α + β and αβ. Trying to find α and β themselves is the long way to a wrong answer."),

    ("fpm:3a", &[
        "Divide a polynomial by (x ± a) or (ax ± b), stating the quotient and the remainder",
        "Use the factor theorem: if f(a) = 0 then (x − a) is a factor",
        "Use the remainder theorem to find a remainder without dividing",
        "Factorise and solve a cubic, given one factor or by finding a rational root",
    ], "For a divisor like (2x − 3) the remainder theorem needs f(3/2), not f(3). Set the divisor to zero and solve it first."),

    ("fpm:3b", &[
        "Solve linear inequalities, reversing the sign when multiplying or dividing by a negative",
        "Rearrange a quadratic inequality so that one side is zero, then factorise",
        "Sketch the parabola to decide which region satisfies the inequality",
        "Give the answer in the right form, including double-ended inequalities",
    ], "Move everything to one side before factorising. Comparing two quadratics term by term does not work."),

    ("fpm:3c", &[
        "Draw the boundary lines for a set of linear inequalities in two variables",
        "Shade the feasible region, using dashed and solid lines correctly",
        "Identify integer points inside the region",
        "Find the maximum or minimum of an objective function at a vertex of the region",
    ], "The optimum of a linear objective always sits at a corner of the feasible region. Test the vertices rather than hunting about inside it."),

    ("fpm:4", &[
        "Sketch a polynomial graph from its factorised form, showing where it crosses the axes",
        "Sketch a rational function with a linear denominator",
        "Find asymptotes parallel to the coordinate axes",
        "Solve an equation by drawing two graphs and reading off the intersections",
    ], "A rational function has a vertical asymptote where the denominator is zero. Sketching a curve straight through that point gives the game away."),

    ("fpm:5", &[
        "Use sigma notation to write and to expand a series",
        "Use the nth term and the sum to n terms of an arithmetic series",
        "Use the nth term and the sum to n terms of a geometric series",
        "Use the sum to infinity of a convergent geometric series, and state the condition |r| < 1",
    ], "A sum to infinity only exists when |r| < 1. Quoting that condition is usually worth a mark on its own."),

    ("fpm:6", &[
        "Expand (1 + x)ⁿ for positive integer n using binomial coefficients",
        "Expand (a + bx)ⁿ by taking out a factor first",
        "Expand (1 + x)ⁿ for rational n as far as a given term",
        "State the validity condition, adjusted for the bracket you actually have",
    ], "For (1 + 3x)ⁿ the expansion is valid when |3x| < 1, so |x| < ⅓. Writing |x| < 1 without adjusting loses the mark."),

    ("fpm:7", &[
        "Add, subtract and scale vectors, and work in i and j components",
        "Find the magnitude of a vector and the unit vector in its direction",
        "Use position vectors and the fact that AB = OB − OA",
        "Find the point dividing AB in the ratio m:n, and prove collinearity or that lines are parallel",
    ], "If λ₁a + μ₁b = λ₂a + μ₂b and a and b are not parallel, you may equate the coefficients. Nearly every vector proof turns on that step."),

    ("fpm:8", &[
        "Find the distance between two points",
        "Find the coordinates of the point dividing a line in a given ratio",
        "Write the equation of a line as y = mx + c, y − y₁ = m(x − x₁) or ax + by = c",
        "Use m₁m₂ = −1 for perpendicular lines and equal gradients for parallel lines",
    ], "The ratio formula is weighted the opposite way round to instinct: dividing AB in m:n gives (nx₁ + mx₂)/(m + n). Check which weight goes with which end."),

    ("fpm:9a", &[
        "Differentiate powers of x, including negative and fractional powers",
        "Differentiate sin ax, cos ax and e^(ax)",
        "Use the product rule and the quotient rule",
        "Use the chain rule for a function of a function",
    ], "The chain rule multiplies by the derivative of the inside. Differentiating sin 3x as cos 3x, without the 3, is the standard error."),

    ("fpm:9b", &[
        "Find stationary points by solving dy/dx = 0",
        "Justify whether each stationary point is a maximum or a minimum",
        "Find the equation of the tangent to a curve at a given point",
        "Find the equation of the normal, using the negative reciprocal gradient",
    ], "Justifying maxima and minima is explicitly expected here. Use the second derivative or the sign either side, and say which you used."),

    ("fpm:9c", &[
        "Integrate powers of x (except 1/x), sin ax, cos ax and e^(ax)",
        "Evaluate a definite integral and use it to find the area under a curve",
        "Find the area between two curves by subtracting",
        "Find a volume of revolution about the x-axis or the y-axis",
    ], "A volume of revolution integrates y², not y. Forgetting to square before integrating is the classic loss of every mark in the question."),

    ("fpm:9d", &[
        "Differentiate displacement to velocity, and velocity to acceleration",
        "Integrate acceleration to velocity and velocity to displacement, finding the constants from given conditions",
        "Use the chain rule to link connected rates of change",
        "Use a small-change approximation with dy/dx",
    ], "Integrating always produces a constant. Kinematics questions hand you a condition to find it — dropping it makes every later answer wrong."),

    ("fpm:10a", &[
        "Convert between degrees and radians",
        "Use s = rθ for arc length and A = ½r²θ for sector area",
        "Know the exact values of sin, cos and tan for 30°, 45° and 60° and their radian equivalents",
        "Sketch the sine, cosine and tangent graphs for angles of any size",
    ], "s = rθ and A = ½r²θ need θ in radians. Feeding degrees in is the single most common error on this topic."),

    ("fpm:10b", &[
        "Use the sine rule and the cosine rule in any triangle",
        "Use ½ab sin C for the area of a triangle",
        "Solve problems in three dimensions, including the angle between a line and a plane",
        "Find the angle between two planes",
    ], "The cosine rule is on the formula sheet, but the sine rule and ½ab sin C are expected to be known. Learn those two."),

    ("fpm:10c", &[
        "Use cos²θ + sin²θ = 1 and tan θ = sin θ / cos θ to simplify and prove identities",
        "Use the addition formulae for sin(A ± B), cos(A ± B) and tan(A ± B)",
        "Solve trigonometric equations over a given interval",
        "Use the graph or the CAST diagram to find every solution in range",
    ], "Give every solution in the interval, not just the one the calculator returns. Sketch the graph and read off each crossing."),

    // ---------- Business (AQA GCSE 8132) ----------
    ("bus:3.1.1", &[
        "Say what a business is and why people start one, and separate goods from services and needs from wants",
        "Name the four factors of production and define opportunity cost",
        "Define the primary, secondary and tertiary sectors and place a business in the right one",
        "Describe what an entrepreneur is, what they are like and what they are trying to get out of it",
        "Explain why the business environment never stays still",
    ], "Capital as a factor of production means equipment and machinery, not money. The everyday meaning loses the mark."),

    ("bus:3.1.2", &[
        "Compare sole trader, partnership, private limited company, public limited company and not-for-profit",
        "Weigh each on control, finance available, liability and what happens to the profits",
        "Explain limited liability and say which structures have it",
        "Recommend a structure for a business, and justify it against that business's situation",
    ], "Limited liability protects the owner's personal money, not the business's. You are not asked how incorporation works legally - only what each structure means for the owner."),

    ("bus:3.1.3", &[
        "Name the main objectives: survival, profit maximisation, growth, market share, customer satisfaction, social and ethical aims, shareholder value",
        "Explain what objectives are actually for in running a business",
        "Explain why objectives differ with size, competition and type of business",
        "Explain how objectives change as a business grows, and how success can be judged by more than profit",
    ], "A not-for-profit judged on profit is being judged wrongly. Match the measure of success to the objective the business actually set."),

    ("bus:3.1.4", &[
        "Say what a stakeholder is and name the main ones: owners, employees, customers, the local community and suppliers",
        "State what each stakeholder wants from the business",
        "Explain how business activity affects each group",
        "Explain how stakeholders influence a business, and why their aims conflict",
    ], "The marks are in the conflict. Higher pay for workers versus higher dividends for owners is the same money going two ways - say so."),

    ("bus:3.1.5", &[
        "Explain the factors behind a location decision: nearness to the market, raw materials, labour, competition and cost",
        "Judge which factor matters most for a particular business",
        "Explain why a business might relocate, and what that costs it",
    ], "A factory and an online retailer weigh these completely differently. Answer about this business, not about location in general."),

    ("bus:3.1.6", &[
        "Explain why a business writes a plan: starting up, raising finance, setting objectives, organising the functions",
        "Describe the main sections of a business plan",
        "Weigh the benefits of planning against its drawbacks",
        "Distinguish fixed, variable and total costs, and revenue, profit and loss",
    ], "You will not be asked to write a business plan - only to explain why one exists and what is in it."),

    ("bus:3.1.7", &[
        "Compare organic growth - franchising, new stores, e-commerce, outsourcing - with external growth by merger or takeover",
        "Weigh the advantages and disadvantages of each route",
        "Explain purchasing and technical economies of scale",
        "Explain diseconomies of scale: poor communication, coordination problems and falling motivation",
        "Calculate and interpret average unit cost",
    ], "Economies of scale are about average unit cost falling, not total cost. Total cost rises with output - quoting it proves nothing."),

    ("bus:3.2.1", &[
        "Explain how e-commerce lets a business reach a wider market",
        "Explain how digital communication changes the way a business deals with its stakeholders",
        "Give real examples of the technology being used",
        "Weigh the benefits of adopting technology against its costs",
    ], "Have actual examples ready. The specification expects you to name real digital technology, not talk about 'the internet' in the abstract."),

    ("bus:3.2.2", &[
        "Explain what it means for a business to behave ethically, and give examples",
        "Weigh the benefits and drawbacks of ethical behaviour",
        "Explain environmental responsibility: congestion, recycling, waste, noise and air pollution, global warming, scarce resources",
        "Discuss the trade-off between profit and behaving ethically or sustainably",
    ], "The trade-off is the answer. Ethical and green choices usually cost money now and pay back in reputation later - say which way it falls for this business."),

    ("bus:3.2.3", &[
        "Explain how a change in interest rates affects a business that has borrowed",
        "Explain how interest rates change what consumers and other businesses spend",
        "Explain how the level of employment affects a business",
        "Explain how consumer spending shifts between products as incomes rise and fall",
    ], "You are not asked why interest rates change - only what happens to businesses when they do. Follow the effect through to this business."),

    ("bus:3.2.4", &[
        "Explain globalisation and its benefits and drawbacks for a business",
        "Explain how a business competes internationally through design, quality and price",
        "Explain how exchange rate movements affect the sales and profit of importers and exporters",
    ], "You will NOT be asked to calculate an exchange rate conversion for AQA Business - only to explain the effect of a change. Work out whether the business imports or exports first."),

    ("bus:3.2.5", &[
        "Outline employment law: the national minimum and living wage, and the Equality Act 2010",
        "Outline health and safety law: the Health and Safety at Work Act 1974",
        "Outline consumer law, including trade descriptions",
        "Explain what the legislation costs a business, and what happens if it fails to comply",
    ], "Only brief knowledge of each law is needed. The marks are for the EFFECT on the business - cost, training, recruitment - not for reciting the Act."),

    ("bus:3.2.6", &[
        "Define a market and explain what competition means",
        "Analyse how competition affects a business, and identify when a business faces little or none",
        "Explain the risks every business faces and why uncertainty cannot be removed",
        "Explain why entrepreneurs take the risk anyway, and how businesses reduce it",
    ], "Risk and uncertainty are not the same as failure. A good answer says what the business does to manage the risk it cannot avoid."),

    ("bus:3.3.1", &[
        "Compare job production with flow production and say when each suits a business",
        "Explain lean production and how it removes waste",
        "Explain just-in-time and how it makes production more efficient",
    ], "AQA needs job and flow only - batch production is not on this specification. Do not waste answer space on it."),

    ("bus:3.3.2", &[
        "Compare just-in-time with just-in-case stock management for a given business",
        "Weigh lower stock costs against more frequent deliveries and lost bulk discounts",
        "Explain how price, quality and reliability affect the choice of supplier",
        "Explain procurement, logistics and what effective supply chain management achieves",
    ], "You will not be asked to draw or read a stock control chart. The marks are in the trade-off: holding buffer stock costs money but running out costs sales."),

    ("bus:3.3.3", &[
        "Explain what customers expect of quality in goods and in services",
        "Explain how a business spots and measures quality problems, and what they cost",
        "Explain total quality management and its advantages",
        "Weigh the costs of maintaining quality against the benefits - sales, reputation, price, and avoiding recalls",
    ], "Quality problems get worse as a business grows, especially where it franchises or outsources. That link to growth is a standard question."),

    ("bus:3.3.4", &[
        "Describe the sales process and what customer engagement means",
        "Explain post-sales service: training, help lines and servicing",
        "Explain the benefits of good service: satisfaction, loyalty, higher spend and profit",
        "Explain the damage done by poor service, including word of mouth and lost revenue",
        "Explain how websites, e-commerce and social media have changed customer service",
    ], "This topic is unique to AQA among the common Business specifications. It is easy marks if you have revised it and a blank page if you have not."),

    ("bus:3.4.1", &[
        "Use the terms span of control, chain of command, delayering and delegation",
        "Explain why businesses have a structure, and describe the main job roles in it",
        "Compare tall and flat structures and how each is managed",
        "Explain centralisation and decentralisation",
        "Explain how the structure changes the way communication flows",
    ], "A wide span of control means fewer layers and faster communication but less supervision of each worker. Both halves earn the mark."),

    ("bus:3.4.2", &[
        "Compare internal and external recruitment and the benefits and drawbacks of each",
        "Describe the stages of recruitment: job analysis, job description, person specification and selection",
        "Explain what an effective recruitment process gives the business",
        "Compare full-time, part-time, job share and zero hour contracts",
    ], "A job description covers the job; a person specification covers the person. Zero hour contracts are explicitly on this specification - know their two sides."),

    ("bus:3.4.3", &[
        "Explain what a business gains from a motivated workforce: retention and productivity",
        "Explain financial methods: salary, wage, commission and profit sharing",
        "Explain non-financial methods: management style, training, more responsibility and fringe benefits",
    ], "AQA states plainly that motivational THEORIES are not examined - no Maslow, no Herzberg, no Taylor. Write about methods and their effects instead."),

    ("bus:3.4.4", &[
        "Explain the benefits of training: productivity, coping with new technology, motivation, retention, quality and service",
        "Describe induction, on-the-job and off-the-job training",
        "Explain what induction training in particular achieves",
        "Weigh on-the-job against off-the-job training and recommend one for a given business",
    ], "The recommendation is the mark. Say which method suits THIS business and why, rather than listing the advantages of both."),

    ("bus:3.5.1", &[
        "Explain why identifying and satisfying customer needs matters",
        "Link it to selling more, choosing the right marketing mix, avoiding costly mistakes and staying competitive",
        "Explain how a business finds out what its customers actually need",
    ], "Short topic, easy marks. The chain is: know the customer, get the mix right, avoid wasting money on the wrong product."),

    ("bus:3.5.2", &[
        "Explain segmentation by gender, age, location and income",
        "Explain how and why a business uses segmentation to target customers",
        "Explain the risk of targeting a segment badly, or of aiming at too broad a market",
    ], "Name the segment and then the marketing decision that follows from it. A list of ways to segment, with no decision attached, scores little."),

    ("bus:3.5.3", &[
        "Explain why businesses research: spotting opportunities, understanding customers and competitors",
        "Compare primary and secondary methods: questionnaires, surveys, interviews, focus groups, internet and printed press",
        "Distinguish qualitative from quantitative research and pick the right method for a business",
        "Read and manipulate data from tables and charts, including market share",
    ], "You are expected to handle the data, not just describe the method. Practise pulling figures off a table and saying what they mean."),

    ("bus:3.5.4", &[
        "Compare pricing methods: skimming, penetration, competitive, loss leader and cost-plus",
        "Explain what influences a pricing decision - costs, the market, competition, the life cycle - and that demand usually falls as price rises",
        "Explain product design, image, USP and brand image, and the risks of new products",
        "Use the product life cycle and the extension strategies at maturity",
        "Use the Boston matrix to explain how a business balances its product portfolio",
        "Compare promotional methods and explain what decides the promotional mix",
        "Compare distribution channels including retailers, telesales, e-commerce and m-commerce",
        "Explain how the four elements have to work together, and how the mix changes over time",
    ], "The Boston matrix and loss leader pricing are on AQA and are missed by anyone revising from a different board's notes. Skimming starts high, penetration starts low."),

    ("bus:3.6.1", &[
        "Name the internal and external sources: family and friends, retained profit, share issue, loan or mortgage, selling assets, overdraft, trade credit, hire purchase, government grants",
        "Weigh the advantages and disadvantages of each in a given situation",
        "Judge which source suits a new business and which suits an established one",
    ], "Match the source to the need and to the age of the business. A start-up cannot issue shares to the public; a long-term asset should not be bought on an overdraft."),

    ("bus:3.6.2", &[
        "Explain the difference between cash and profit, and why a profitable business can still fail",
        "Explain the consequences of cash-flow problems and the value of positive cash flow",
        "Complete and interpret parts of a cash-flow forecast: inflows, outflows, net cash flow, opening and closing balance",
        "Evaluate the fixes: rescheduling payments, an overdraft, cutting outflows, raising inflows, new finance",
    ], "You are not expected to build a whole forecast, only to fill in and read parts of one. Closing balance is opening balance plus net cash flow - carry it across."),

    ("bus:3.6.3", &[
        "Distinguish fixed, variable and total costs, and revenue, profit and loss",
        "Calculate the average rate of return on an investment project",
        "Explain break-even output and read a break-even chart",
        "Identify the break-even point and margin of safety from a chart",
        "Evaluate how useful break-even analysis actually is",
    ], "AQA does NOT ask you to draw a break-even chart or use the break-even formula - but it DOES ask for average rate of return, which most other boards leave out. Learn ARR."),

    ("bus:3.6.4", &[
        "Explain why financial statements matter for judging performance and making decisions",
        "Identify the main parts of an income statement and a statement of financial position",
        "Distinguish assets from liabilities, and explain why the statement is a snapshot of one day",
        "Calculate gross profit margin and net profit margin",
        "Judge performance against last year, against competitors, and from different stakeholders' points of view",
    ], "For AQA you need the two profit margins - not ROCE and not the current ratio. And no formulae are given in the exam, so recall them."),

    // ---------- Economics (Cambridge IGCSE 9-1, 0987) ----------
    ("econ:1.1", &[
        "Define the basic economic problem: finite resources against infinite wants",
        "Explain scarcity, and give examples for consumers, workers, producers and governments",
        "State the three questions every economy must answer: what, how, and for whom to produce",
        "Distinguish an economic good from a free good",
    ], "Scarcity is not shortage. A good is scarce because wants exceed what resources can supply, even when the shelves are full."),

    ("econ:1.2", &[
        "Define land, labour, capital and enterprise",
        "State the reward to each: rent, wages, interest and profit",
        "Explain what causes the quantity of each factor to change",
        "Explain what causes the quality of each factor to change",
    ], "The rewards are examinable in their own right and are easy marks. Capital means machinery and equipment, not money."),

    ("econ:1.3", &[
        "Define opportunity cost and give examples in different contexts",
        "Apply opportunity cost to a consumer's decision, a worker's, a firm's and a government's",
        "Show opportunity cost on a production possibility curve",
    ], "Opportunity cost is the NEXT BEST alternative given up - one thing, not the whole list of things you did not choose."),

    ("econ:1.4", &[
        "Define a production possibility curve and draw one",
        "Explain what points under, on and beyond the curve each mean",
        "Explain what a movement along the curve shows about opportunity cost",
        "Explain what causes the curve to shift, and what a shift means for growth",
    ], "Label both axes and mark your points. A point inside means unemployed or inefficiently used resources - not that the economy cannot produce more."),

    ("econ:2.1", &[
        "Define a market and give examples",
        "Explain the roles of buyers and sellers in a market",
        "Explain how a market allocates resources without anyone planning it",
    ], "Short topic. A market does not need a physical place - it is any arrangement bringing buyers and sellers together."),

    ("econ:2.2", &[
        "Define demand and link individual demand to market demand",
        "Draw and read a demand diagram",
        "Explain what causes an extension or contraction along the demand curve",
        "Explain what causes the demand curve to shift, and draw it",
    ], "Cambridge uses extension and contraction for movements ALONG the curve, and increase and decrease for SHIFTS. Use their words."),

    ("econ:2.3", &[
        "Define supply and link individual supply to market supply",
        "Draw and read a supply diagram",
        "Explain what causes an extension or contraction along the supply curve",
        "Explain what causes the supply curve to shift, and draw it",
    ], "A change in the price of the good itself never shifts the curve. Only a non-price factor does - cost, technology, tax, weather, number of firms."),

    ("econ:2.4", &[
        "Explain how the price mechanism answers what, how and for whom to produce",
        "Define market equilibrium and find it from a schedule and from a diagram",
        "Define market disequilibrium and find it from a schedule and from a diagram",
        "Identify shortages where demand exceeds supply, and surpluses where supply exceeds demand",
    ], "You must be able to work from a table of numbers as well as a graph. Cambridge asks for both - practise reading equilibrium off a schedule."),

    ("econ:2.5", &[
        "Explain how changes in demand and supply cause price to change",
        "Explain the effect of a price change on sales",
        "Use demand and supply diagrams to show the effect of changed market conditions",
    ], "Say which curve shifts, which way, and then read off the new price AND the new quantity. Half-answers give the price and forget the quantity."),

    ("econ:2.6", &[
        "Define price elasticity of demand and calculate it using the formula",
        "Interpret the value: perfectly inelastic, inelastic, unitary, elastic, perfectly elastic",
        "Draw demand curves showing different elasticities",
        "Explain what determines whether demand is elastic or inelastic",
        "Explain and calculate the effect of a price change on consumer spending and firms' revenue",
        "Explain what PED means for consumers, workers, firms and government",
    ], "If demand is inelastic, raising price raises revenue; if elastic, it lowers it. Test it with numbers if you are unsure - it is worth a lot of marks."),

    ("econ:2.7", &[
        "Define price elasticity of supply and calculate it using the formula",
        "Interpret the value across the full range from perfectly inelastic to perfectly elastic",
        "Draw supply curves showing different elasticities",
        "Explain what determines whether supply is elastic or inelastic",
    ], "Time is the main determinant. Supply is nearly always more elastic in the long run, when a firm can change all its factors."),

    ("econ:2.8", &[
        "Define the market economic system",
        "Give the arguments for it: efficiency, choice, innovation and the profit incentive",
        "Give the arguments against it: inequality, market failure and under-provision",
    ], "Keep this separate from the mixed economy topic. Cambridge examines them as two distinct sections and wants the pure case here."),

    ("econ:2.9", &[
        "Define market failure",
        "Define public goods, merit goods, demerit goods, private, external and social benefits and costs, and monopoly",
        "Explain the causes of market failure, including abuse of monopoly power",
        "Explain the consequences: over-consumption of demerit goods, under-consumption of merit goods, non-provision of public goods, restricted supply under monopoly",
    ], "Diagrams are NOT required for market failure on this syllabus - the definitions are. Learn the nine terms precisely; they carry the marks."),

    ("econ:2.10", &[
        "Define the mixed economic system and give the arguments for and against it",
        "Explain, draw and evaluate maximum and minimum prices, indirect taxation and subsidies",
        "Explain and evaluate regulation, privatisation, nationalisation and direct provision",
        "Explain and evaluate quotas, for example on extracting natural resources",
    ], "Here diagrams ARE required - for maximum and minimum prices, indirect taxes and subsidies. That is the opposite of the market failure section."),

    ("econ:3.1", &[
        "Describe the forms, functions and characteristics of money",
        "Explain the role and importance of a central bank",
        "Explain the role and importance of commercial banks",
    ], "This whole topic is absent from most other boards' Economics courses. If you are revising from non-Cambridge material you will simply never meet it."),

    ("econ:3.2", &[
        "Explain how income affects household spending, saving and borrowing",
        "Explain how the rate of interest affects each of the three",
        "Explain how confidence, age and culture affect each of the three",
    ], "The question usually names one influence and one behaviour. Answer about that pairing rather than writing everything you know about households."),

    ("econ:3.3", &[
        "Explain the wage and non-wage factors behind a choice of occupation",
        "Explain how the demand for and supply of labour set the wage, and draw the diagram",
        "Explain the effect of trade unions and of a national minimum wage, with a diagram",
        "Explain why wages differ: skills, sector, bargaining strength, discrimination and government policy",
        "Explain the causes and consequences of occupational and geographical mobility of labour",
        "Define division of labour and give its advantages and disadvantages",
    ], "A minimum wage only bites if it is set ABOVE the equilibrium wage. Draw it above and label the excess supply of labour."),

    ("econ:3.4", &[
        "Classify firms by sector and by private or public ownership",
        "Give the advantages and disadvantages of small and of large firms",
        "Define horizontal, vertical and conglomerate mergers, with examples and their pros and cons",
        "Explain internal and external economies and diseconomies of scale",
        "Draw and interpret an average total cost diagram showing both",
    ], "The three merger types are examinable by name. Vertical merges along the supply chain, horizontal at the same stage, conglomerate across unrelated markets."),

    ("econ:3.5", &[
        "Explain what determines a firm's demand for factors of production",
        "Compare labour-intensive and capital-intensive production and give reasons for each",
        "Distinguish production from productivity",
        "Explain what influences production and productivity, and the effect of investment",
    ], "Production is total output; productivity is output per unit of input. More workers producing more is not a productivity gain."),

    ("econ:3.6", &[
        "Define total, average total, fixed, average fixed, variable and average variable cost",
        "Calculate all six, and draw diagrams showing how output changes them",
        "Define and calculate total revenue and average revenue",
        "Explain how sales influence revenue",
        "Explain the objectives of firms: survival, social welfare, profit maximisation and growth",
    ], "Six cost measures, all calculable and all examinable. AFC always falls as output rises - fixed cost spread over more units."),

    ("econ:3.7", &[
        "Describe the characteristics of a competitive market and its advantages and disadvantages",
        "Explain the effect of many firms on price, quality, choice and profit",
        "Describe the characteristics of a monopoly and its advantages and disadvantages",
        "Explain the effect of a single firm on price, quality, choice and profit",
    ], "Diagrams and perfect-competition theory are explicitly NOT required. Do not draw the A-level style diagrams - describe the effects instead."),

    ("econ:4.1", &[
        "Name the aims: economic growth, full employment, stable prices, balance of payments stability, redistribution of income, environmental sustainability",
        "Explain why a government chooses particular aims and how it judges success",
        "Explain the conflict between full employment and stable prices",
        "Explain the conflicts between growth and sustainability, and between full employment and the balance of payments",
    ], "Six aims, not four. Redistribution of income and environmental sustainability are on this syllabus and are regularly forgotten."),

    ("econ:4.2", &[
        "Define the government budget, and a budget deficit and surplus, and calculate their size",
        "Explain the main areas of government spending and their effects",
        "Explain the reasons for taxation, from raising revenue to discouraging demerit goods",
        "Classify taxes as progressive, regressive or proportional, and as direct or indirect",
        "Explain the impact of tax on consumers, workers, firms, government and the economy",
        "Define fiscal policy and explain how changes in tax and spending serve the government's aims",
    ], "Progressive, regressive and proportional are examinable by definition and by example. A regressive tax takes a larger share from a lower income."),

    ("econ:4.3", &[
        "Define money supply and monetary policy",
        "Explain the measures: changing the interest rate, the money supply and the exchange rate",
        "Explain how monetary policy helps a government meet its macroeconomic aims",
    ], "Cambridge counts the foreign exchange rate as a monetary policy instrument. Most other courses do not - do not leave it out."),

    ("econ:4.4", &[
        "Define supply-side policy",
        "Explain the measures: education and training, infrastructure, labour market reform, lower direct taxes, deregulation, incentives, privatisation",
        "Explain how supply-side policy helps a government meet its aims",
    ], "Supply-side policy raises the economy's capacity, so it can raise growth without adding inflation. That is why it is slow and expensive."),

    ("econ:4.5", &[
        "Define economic growth and explain how real GDP measures it",
        "Explain the causes of growth: more total demand, more resources, or better resources",
        "Give the advantages and disadvantages of growth",
        "Define recession, explain its causes, and its consequences for consumers, workers, firms and government",
        "Evaluate the policies available to promote growth",
    ], "Real GDP is adjusted for inflation. A rise in money GDP with higher prices may be no real growth at all."),

    ("econ:4.6", &[
        "Define employment, unemployment and full employment",
        "Explain how unemployment is measured by the labour force survey, and use the unemployment rate formula",
        "Explain frictional, structural, cyclical and seasonal unemployment",
        "Explain the consequences for the individual, firms, government and the economy",
        "Evaluate the policies available to reduce unemployment",
    ], "The unemployment rate formula is examinable. Match the policy to the TYPE - retraining fixes structural unemployment, not cyclical."),

    ("econ:4.7", &[
        "Define inflation and deflation",
        "Explain how inflation is measured using the Consumer Prices Index",
        "Explain demand-pull and cost-push causes",
        "Explain how inflation affects savers, lenders and borrowers, and the wider economy",
        "Evaluate the policies available to control inflation",
    ], "Deflation is on this syllabus, and falling inflation is not deflation. Falling inflation still means prices are rising, just more slowly."),

    ("econ:5.1", &[
        "Explain real GDP per head as an indicator of living standards",
        "Explain the Human Development Index and name its components",
        "Give the advantages and disadvantages of each indicator",
        "Explain why living standards and income distribution differ within and between countries",
    ], "Know the HDI's components - income, education and life expectancy. This entire section is missing from most non-Cambridge courses."),

    ("econ:5.2", &[
        "Distinguish absolute from relative poverty",
        "Explain the causes: unemployment, low wages, illness, age and environmental factors",
        "Explain policies to reduce poverty and redistribute income: growth, education, healthcare, benefits, progressive tax and a minimum wage",
    ], "Absolute poverty is being unable to afford basic needs; relative poverty is being far below the average in your own country. A rich country still has relative poverty."),

    ("econ:5.3", &[
        "Define birth rate, death rate, net migration, immigration and emigration",
        "Explain how and why these vary between countries",
        "Explain the concept of an optimum population",
        "Explain the effects of changes in population size, age structure and gender distribution",
    ], "An ageing population is the standard question: a higher dependency ratio, more spending on pensions and healthcare, a smaller workforce."),

    ("econ:5.4", &[
        "Explain how countries differ in income, productivity and population growth",
        "Explain how they differ in the size of their primary, secondary and tertiary sectors",
        "Explain how they differ in saving and investment, education, healthcare and natural resources",
        "Explain the consequences of these differences",
    ], "Link the causes to each other rather than listing them. Low income means low saving, which means low investment, which keeps productivity low."),

    ("econ:6.1", &[
        "Define specialisation by country and explain what it is based on",
        "Give the advantages and disadvantages of specialisation",
        "Define free trade and give its advantages and disadvantages",
    ], "Cambridge asks for specialisation based on best resource allocation and low-cost production - NOT for comparative advantage, which is not on this syllabus."),

    ("econ:6.2", &[
        "Define globalisation and explain its causes: trade restrictions, transport and communication costs, and the movement of multinationals",
        "Explain the effects on trade, competition, the environment, migration, income distribution and development",
        "Explain the advantages and disadvantages of multinationals to host and home countries",
        "Describe tariffs, import quotas, subsidies and embargoes",
        "Explain the reasons for trade restrictions, from protecting infant industries to restricting demerit goods",
        "Explain the consequences of restrictions for the home country and its trading partners",
    ], "Embargoes are on this syllabus and are often forgotten. Multinationals must be judged from BOTH the host and the home country's side."),

    ("econ:6.3", &[
        "Define the foreign exchange rate",
        "Explain why currencies are bought and sold: trade, speculation, government intervention, profit and dividends, remittances and investment",
        "Define a floating exchange rate, appreciation and depreciation",
        "Explain how demand and supply set the equilibrium rate, and draw it",
        "Explain the causes of fluctuations: changes in exports and imports, interest rates and speculation",
        "Explain the effect of a change on the prices of and demand for exports and imports",
    ], "Work out the direction first, then the consequence. A stronger currency makes exports dearer abroad and imports cheaper at home."),

    ("econ:6.4", &[
        "Name the components of the current account: trade in goods, trade in services, primary income and secondary income",
        "Calculate a deficit or surplus on the current account and on each component",
        "Explain the causes of a current account deficit and surplus",
        "Explain the consequences for GDP, employment, inflation and the exchange rate",
        "Evaluate the policies available to achieve balance of payments stability",
    ], "Four components, and the calculation is examinable. Primary income is interest, profit and dividends; secondary income is transfers such as aid and remittances."),

    // ---------- Computer Science (OCR GCSE J277) ----------
    ("cs:1.1.1", &[
        "Describe what the CPU does and walk through the fetch-execute cycle",
        "State the job of the ALU, the control unit, cache and registers",
        "Explain the von Neumann architecture and what the MAR, MDR, program counter and accumulator each hold",
        "Distinguish a register that holds an address from one that holds data",
    ], "The MAR holds an address; the MDR holds data. Swapping them is the single most common CPU-question error."),

    ("cs:1.1.2", &[
        "Explain how clock speed affects performance",
        "Explain how cache size affects performance",
        "Explain how the number of cores affects performance, and why doubling cores does not double speed",
    ], "More cores only help if the software can split the work. Say that, or the answer reads as if you think cores are just faster."),

    ("cs:1.1.3", &[
        "Describe what an embedded system is and its characteristics",
        "Give examples of embedded systems and say what makes each one embedded",
        "Explain why an embedded system is built the way it is: dedicated, small, cheap, low-power",
    ], "An embedded system is a computer inside a device that is not a general-purpose computer. A laptop is not one; a washing machine's controller is."),

    ("cs:1.2.1", &[
        "Explain why a computer needs primary storage",
        "Compare RAM and ROM, and state the purpose of each",
        "Explain virtual memory: what it is, when it is used, and why it slows things down",
        "Explain the role of cache",
    ], "ROM holds the start-up instructions and does not change; RAM holds what is running now and empties when the power goes. Volatility is the distinction examiners want."),

    ("cs:1.2.2", &[
        "Explain why secondary storage is needed",
        "Describe optical, magnetic and solid-state storage and how each works",
        "Choose a suitable device for a given use and justify it",
        "Compare devices on capacity, speed, portability, durability, reliability and cost",
    ], "Justify the choice against the scenario, not in general. A phone needs solid state for durability and size; an archive needs capacity per pound."),

    ("cs:1.2.3", &[
        "Know the units from bit to petabyte, and that OCR uses 1,000 not 1,024",
        "Explain why data must be in binary for a computer to process it",
        "Calculate file sizes and the capacity a set of files needs",
    ], "OCR defines a kilobyte as 1,000 bytes. Use 1,024 and every capacity calculation is marked wrong."),

    ("cs:1.2.4", &[
        "Convert denary to binary and back, up to 8 bits, and add two 8-bit binary numbers including overflow",
        "Convert denary and binary to and from two-digit hexadecimal",
        "Perform binary shifts and explain their effect",
        "Explain character sets, ASCII and Unicode, and how bits per character limits the set",
        "Explain how an image is stored as pixels with colour depth and resolution, and the effect on quality and file size",
        "Explain metadata",
        "Explain how sound is sampled, and the effect of sample rate, duration and bit depth on quality and file size",
    ], "A left shift multiplies by two per place, a right shift divides. Say what happens to bits that fall off the end - that is where the mark is."),

    ("cs:1.2.5", &[
        "Explain why files are compressed",
        "Compare lossy and lossless compression",
        "Choose a compression type for a given file and justify it",
    ], "Lossy loses data permanently and is fine for photos and music; lossless keeps everything and is required for text and programs."),

    ("cs:1.3.1", &[
        "Compare LANs and WANs",
        "Explain what affects network performance",
        "Explain the roles of computers in client-server and peer-to-peer networks",
        "Name the hardware needed to build a LAN: access points, routers, switches, NICs, transmission media",
        "Explain the Internet as a network of networks, and the roles of DNS, hosting, the cloud, and web servers and clients",
        "Compare star and mesh topologies",
    ], "A switch sends data to the one device it is for; a hub sends it to everything. If you cannot say what a switch does differently, you cannot get the mark."),

    ("cs:1.3.2", &[
        "Compare wired and wireless connections",
        "Explain why data is encrypted on a network",
        "Explain IP addressing and MAC addressing and how they differ",
        "Explain why standards matter",
        "Describe what TCP/IP, HTTP, HTTPS, FTP, POP, IMAP and SMTP each do",
        "Explain the concept of layers and why protocols are layered",
    ], "A MAC address is fixed to the hardware; an IP address can change with the network. Learn what each of the seven protocols is for, one line each."),

    ("cs:1.4.1", &[
        "Describe malware and its forms",
        "Describe social engineering, including phishing, and why people are the weak point",
        "Describe brute-force attacks, denial of service, data interception and theft",
        "Explain the concept of SQL injection",
    ], "Name the attack, then say what it does and what it is after. Naming it alone is one mark; the explanation is the rest."),

    ("cs:1.4.2", &[
        "Explain penetration testing",
        "Explain how anti-malware software, firewalls, user access levels, passwords, encryption and physical security each prevent attacks",
        "Match a prevention method to the threat it addresses",
    ], "Match the defence to the attack. A firewall does nothing against phishing; user training does. Mismatched pairs score nothing."),

    ("cs:1.5.1", &[
        "Explain the purpose of an operating system",
        "Describe the user interface, and memory management and multitasking",
        "Describe peripheral management and drivers",
        "Describe user management and file management",
    ], "Five functions, each a separate mark. Learn them as a list you can reel off and then say what each one does."),

    ("cs:1.5.2", &[
        "Explain the purpose of utility software",
        "Describe encryption software, defragmentation and data compression as utilities",
        "Explain why defragmentation speeds up a magnetic disk and does nothing for a solid-state one",
    ], "Defragmenting an SSD is pointless and wears it out. The question that asks which drive benefits is testing exactly that."),

    ("cs:1.6.1", &[
        "Discuss the ethical, legal, cultural, environmental and privacy impacts of digital technology",
        "Describe the Data Protection Act 2018, the Computer Misuse Act 1990 and the Copyright, Designs and Patents Act 1988",
        "Compare open-source and proprietary software licences",
        "Build a balanced argument on a technology issue and reach a conclusion",
    ], "Match the law to the situation: hacking is Computer Misuse, holding customer data is Data Protection, copying software is Copyright. Naming the wrong Act scores nothing."),

    ("cs:2.1.1", &[
        "Explain abstraction and give an example",
        "Explain decomposition and give an example",
        "Explain algorithmic thinking",
    ], "Abstraction removes detail that does not matter; decomposition splits the problem into parts. Do not describe one when asked for the other."),

    ("cs:2.1.2", &[
        "Identify the inputs, processes and outputs of a problem",
        "Draw and read structure diagrams",
        "Create, interpret, correct, complete and refine algorithms in pseudocode, flowcharts and the Reference Language",
        "Identify common errors in algorithms",
        "Complete a trace table for a given algorithm",
    ], "A trace table records the value of every variable at every step. Missing a column or skipping a step loses the whole table's marks."),

    ("cs:2.1.3", &[
        "Carry out a binary search and a linear search by hand, and say when each applies",
        "Carry out bubble sort, merge sort and insertion sort by hand",
        "Show each pass of a sort on given data",
        "Identify a sort or search from a description or a trace",
    ], "Binary search needs sorted data. Say so. And show every pass of a sort, not just the end result - the marks are in the passes."),

    ("cs:2.2.1", &[
        "Use variables, constants, operators, inputs, outputs and assignments",
        "Use sequence, selection and iteration, including count- and condition-controlled loops",
        "Use the arithmetic operators, including MOD and DIV",
        "Use AND, OR and NOT",
    ], "MOD gives the remainder, DIV the whole-number quotient. 17 MOD 5 is 2; 17 DIV 5 is 3. These are examined directly."),

    ("cs:2.2.2", &[
        "Use integer, real, Boolean, character and string data types",
        "Choose the right data type for a value",
        "Cast between types and say when it is needed",
    ], "Input arrives as a string. If you are going to do arithmetic on it, cast it first - forgetting is the classic Paper 2 error."),

    ("cs:2.2.3", &[
        "Manipulate strings: length, substrings, case, concatenation",
        "Open, read, write and close a text file",
        "Use records to store data",
        "Write SQL to search for data with SELECT, FROM and WHERE",
        "Use one- and two-dimensional arrays to solve problems",
        "Write and call functions and procedures with parameters and return values",
        "Generate random numbers",
    ], "A function returns a value; a procedure does not. Use the right word and the right structure - it is examined in the Reference Language."),

    ("cs:2.3.1", &[
        "Explain defensive design: anticipating misuse and authentication",
        "Write input validation and explain what it checks",
        "Make a program maintainable with sub programs, naming conventions, indentation and comments",
    ], "Validation checks the input is sensible; authentication checks who the user is. They are different questions and often confused."),

    ("cs:2.3.2", &[
        "Explain why programs are tested",
        "Compare iterative testing during development with final testing",
        "Identify syntax errors and logic errors in given code",
        "Choose normal, boundary, invalid and erroneous test data and say what each is for",
        "Refine an algorithm in response to test results",
    ], "A syntax error stops the program running; a logic error runs and gives the wrong answer. Boundary data sits exactly on the edge of what is allowed."),

    ("cs:2.4.1", &[
        "Draw and read simple logic diagrams using AND, OR and NOT",
        "Complete truth tables",
        "Combine operators into more complex diagrams and expressions",
        "Apply logical operators in truth tables to solve problems",
    ], "Work a truth table column by column, one gate at a time. Trying to do the whole expression in your head is how the last column goes wrong."),

    ("cs:2.5.1", &[
        "Compare high-level and low-level languages and say what each is for",
        "Explain what a translator does and why one is needed",
        "Compare a compiler with an interpreter",
    ], "A compiler translates the whole program once and reports all errors; an interpreter translates line by line and stops at the first. Give both halves."),

    ("cs:2.5.2", &[
        "Name the common tools in an IDE: editor, error diagnostics, run-time environment, translator",
        "Explain what each tool is for",
        "Explain how an IDE helps a programmer write and debug code",
    ], "Say what the tool does for the programmer, not just its name. An editor with syntax highlighting catches typos as you type - that is the mark."),

    // ---------- English Literature (AQA GCSE 8702) ----------
    ("englit:3.1.1a", &[
        "Track the plot act by act, from the witches to Macbeth's death",
        "Explain how each scene moves the action on, rather than retelling it",
        "Know the turning points: the prophecy, Duncan's murder, Banquo's ghost",
        "Quote from any point in the play without the text in front of you",
    ], "Paper 1 gives you an extract but expects the whole play. Roughly half the marks come from outside the extract."),

    ("englit:3.1.1b", &[
        "Trace Macbeth from loyal soldier to tyrant, with a quotation at each stage",
        "Explain how Shakespeare presents his conscience and his self-deception",
        "Argue whether he is a victim of fate or fully responsible",
        "Use terms like soliloquy and hamartia only where they earn their place",
    ], "Write about Shakespeare's presentation, not about Macbeth as a real man. \"Shakespeare shows\" scores; \"Macbeth felt\" does not."),

    ("englit:3.1.1c", &[
        "Explain Lady Macbeth's part in the murder and how she manipulates him",
        "Track her decline from Act 1 to the sleepwalking scene",
        "Discuss how she subverts Jacobean expectations of women",
        "Compare her language early and late in the play",
    ], "Do not settle for \"evil\". The best answers hold both readings at once — driving force and eventual casualty."),

    ("englit:3.1.1d", &[
        "Explain ambition as the play's central force, and where it is condemned",
        "Contrast Duncan, Macbeth and Malcolm as rulers",
        "Explain the divine right of kings and why regicide horrified the audience",
        "Choose quotations that serve more than one theme",
    ], "A theme answer needs the writer's purpose. Say what Shakespeare warns his audience about, not merely that ambition appears."),

    ("englit:3.1.1e", &[
        "Explain how guilt is made physical: blood, hands, sleep",
        "Discuss whether the witches cause events or only reveal them",
        "Explain the supernatural as both atmosphere and moral disorder",
        "Link the disorder in nature to the murder of a king",
    ], "The witches never tell Macbeth to kill anyone. An answer that says they did has given away the question of responsibility."),

    ("englit:3.1.1f", &[
        "Explain the Jacobean context: James I, the Gunpowder Plot, witchcraft",
        "Analyse the imagery patterns of blood, darkness, clothing and disease",
        "Discuss stagecraft: soliloquy, dramatic irony, the banquet scene",
        "Comment on verse and prose, and on where the rhythm breaks",
    ], "Context earns marks only when it explains the writing. A paragraph of history with no link to the text scores nothing."),

    ("englit:3.1.2a", &[
        "Summarise each of the five staves and what changes in it",
        "Explain the circular structure and why Dickens chose it",
        "Explain how the ghost-story form carries a moral argument",
        "Quote from every stave, not only the first",
    ], "It is a novella in staves, not chapters — Dickens is writing a carol in prose. Noticing that is an easy mark."),

    ("englit:3.1.2b", &[
        "Track Scrooge's change stave by stave, with evidence for each step",
        "Explain how Dickens makes the early Scrooge repellent",
        "Discuss whether the transformation is convincing or too sudden",
        "Explain Scrooge as an allegory aimed at the wealthy reader",
    ], "The change is gradual. Answers that jump from mean to generous miss every mark for development."),

    ("englit:3.1.2c", &[
        "Explain Marley's function as warning and frame",
        "Explain what each spirit shows, and why in that order",
        "Discuss Ignorance and Want, and why they appear under the second spirit",
        "Analyse how each ghost is described and what the description implies",
    ], "Ignorance and Want are the most political moment in the book. Leaving them out of a social-responsibility answer wastes the best evidence."),

    ("englit:3.1.2d", &[
        "Explain Dickens's attack on attitudes to the poor",
        "Discuss the Cratchits, and Tiny Tim as a device rather than a person",
        "Explain redemption as the book's argument, not just its ending",
        "Connect family, generosity and community across the staves",
    ], "Dickens wants the reader to act, not only to feel sorry. Say what he is asking of his audience."),

    ("englit:3.1.2e", &[
        "Explain the 1843 context: the Poor Law, workhouses, child labour, Malthus",
        "Discuss the narrative voice and its direct address to the reader",
        "Analyse contrast, symbolism and pathetic fallacy",
        "Explain why he wrote it as a cheap book anyone could buy",
    ], "\"Victorian times were hard\" is not context. Name the specific thing and tie it to words on the page."),

    ("englit:3.2.1a", &[
        "Summarise the three acts and where each one ends",
        "Explain the real-time, single-set construction",
        "Explain how each character's involvement is revealed in turn",
        "Discuss the final phone call and what the cyclical ending does",
    ], "Paper 2 is closed book with no extract. Learn a handful of short quotations per character rather than long speeches."),

    ("englit:3.2.1b", &[
        "Contrast Arthur and Sybil Birling with Sheila and Eric",
        "Explain Gerald's position between the two generations",
        "Track who accepts responsibility and who does not",
        "Explain how each character stands for an attitude",
    ], "The generational split is the point. Treating the Birlings as one group loses Priestley's whole argument."),

    ("englit:3.2.1c", &[
        "Explain the Inspector's method: one line of enquiry at a time",
        "Analyse his final speech and its warning",
        "Discuss what he might be — policeman, conscience, socialist voice, something stranger",
        "Explain how he controls the pace and the stage",
    ], "Whether the Inspector is real is deliberately unresolved. Argue a reading and support it; do not try to settle it."),

    ("englit:3.2.1d", &[
        "Explain collective responsibility as the play's central claim",
        "Discuss class, and how the Birlings treat Eva Smith",
        "Discuss gender: Sheila, Sybil, and Eva's position as a working woman",
        "Explain the older and younger characters as Priestley's pessimism and hope",
    ], "Eva Smith never appears on stage. That silence is a choice worth writing about, not an oversight."),

    ("englit:3.2.1e", &[
        "Explain why it is set in 1912 but written in 1945",
        "Discuss the dramatic irony of Birling on the Titanic and on war",
        "Explain the play as a modern morality play",
        "Analyse stagecraft: lighting, entrances, the doorbell, the single set",
    ], "The 1912 setting lets the audience know Birling is wrong before he stops speaking. Naming that irony beats naming the date."),

    ("englit:3.2.2a", &[
        "Explain how Shelley presents power as temporary",
        "Analyse the sonnet form and the layers of narration",
        "Explain the irony of the inscription set against the ruin",
        "Learn two quotations you can deploy under pressure",
    ], "The sculptor's work outlasts the king — art survives power. That reading lifts an answer above \"power fades\"."),

    ("englit:3.2.2b", &[
        "Explain Blake's presentation of institutional control and suffering",
        "Analyse the repetition, the rhythm and the dramatic monologue voice",
        "Explain \"mind-forged manacles\" as the poem's central image",
        "Place it against Blake's anger at church, state and commerce",
    ], "The speaker is walking and observing. Tracking that journey through the stanzas gives you structure marks cheaply."),

    ("englit:3.2.2c", &[
        "Explain the shift from confidence to fear as the mountain rises",
        "Analyse the blank verse and the first-person retrospective voice",
        "Explain nature's power over human beings",
        "Discuss the lasting effect on the speaker's mind",
    ], "It is an extract from a much longer autobiographical poem. Say so — the fragment's context is worth a mark."),

    ("englit:3.2.2d", &[
        "Explain the Duke's control over the portrait, the visitor and the story",
        "Analyse the dramatic monologue and the rhyming couplets he cannot contain",
        "Explain what the poem reveals that the Duke does not intend",
        "Discuss possessiveness, status and the objectification of the Duchess",
    ], "The Duke condemns himself. The whole poem depends on the gap between what he says and what we understand."),

    ("englit:3.2.2e", &[
        "Explain the presentation of duty, obedience and futile sacrifice",
        "Analyse the dactylic rhythm and how it drives the poem",
        "Explain the repetition of \"six hundred\" and its shifting effect",
        "Discuss Tennyson's position as Poet Laureate writing about a blunder",
    ], "It both honours the soldiers and admits a mistake was made. Answers that pick only one side flatten the poem."),

    ("englit:3.2.2f", &[
        "Explain how the weather, not the enemy, becomes the antagonist",
        "Analyse the refrain \"But nothing happens\" and the circular structure",
        "Explain the half-rhyme and its unsettling effect",
        "Discuss Owen's purpose in writing against the glory of war",
    ], "Almost nothing happens on purpose. The boredom and cold are the point — do not hunt for action that is not there."),

    ("englit:3.2.2g", &[
        "Explain the shift from confidence to fear as the storm arrives",
        "Analyse the military and violent imagery used for nature",
        "Explain the effect of the blank verse and the colloquial opening",
        "Discuss possible readings about Northern Ireland",
    ], "\"Storm on the Island\" begins with \"Stormont\" if you read the first eight letters. Worth a sentence, not a paragraph."),

    ("englit:3.2.2h", &[
        "Explain the in-medias-res opening and its disorientation",
        "Analyse the presentation of instinct overriding thought",
        "Explain how time slows and stretches in the middle stanza",
        "Discuss patriotism reduced to a single desperate movement",
    ], "The soldier is never named and barely thinks. That anonymity is Hughes's method, not a gap to fill in."),

    ("englit:3.2.2i", &[
        "Explain the two halves: the event, and living with the event",
        "Analyse the colloquial voice and the monologue form",
        "Explain the presentation of guilt and post-traumatic stress",
        "Discuss the ambiguity of \"probably armed, possibly not\"",
    ], "The killing takes a few lines; the aftermath takes the rest. That imbalance is the poem's argument about trauma."),

    ("englit:3.2.2j", &[
        "Explain the mother's grief and the domestic detail carrying it",
        "Analyse the shifting time frame and its dreamlike movement",
        "Explain the textile and wounding imagery running through",
        "Discuss what is left deliberately unsaid about the son",
    ], "We are never told the son has died. The poem works by implication — stating it as fact misses the technique."),

    ("englit:3.2.2k", &[
        "Explain the photographer's detachment and its cost",
        "Analyse the religious imagery of the darkroom",
        "Explain the contrast between the war zone and \"Rural England\"",
        "Discuss the reader's indifference as the poem's real target",
    ], "The final stanza turns on the reader. An answer that stops at the photographer's suffering misses the accusation."),

    ("englit:3.2.2l", &[
        "Explain paper as a metaphor for the fragility of human power",
        "Analyse the free verse, the short stanzas and the lack of closure",
        "Explain the shift from documents to buildings to skin",
        "Discuss the presentation of transience and light",
    ], "The poem never fully resolves, and the form refuses to. Do not force a neat conclusion onto it."),

    ("englit:3.2.2m", &[
        "Explain the tension between memory and present reality",
        "Analyse the repeated \"sunlight\" and what it comes to mean",
        "Explain the ambiguity of the unnamed city and the unnamed threat",
        "Discuss identity, exile and belonging",
    ], "The country may no longer exist as she remembers it. That gap between memory and fact is the poem's subject."),

    ("englit:3.2.2n", &[
        "Explain the contrast between imposed history and reclaimed history",
        "Analyse the phonetic spelling and non-standard form as a political act",
        "Explain the different typography and rhythm of the two strands",
        "Discuss identity, education and cultural erasure",
    ], "The spelling is a deliberate assertion of voice, not an error. Saying so is central to any decent answer."),

    ("englit:3.2.2o", &[
        "Explain the pilot's journey and his return, told at second hand",
        "Analyse the narrative distance created by the daughter's voice",
        "Explain the sea imagery and the pull of memory and family",
        "Discuss shame, honour and the cost of turning back",
    ], "He is punished for surviving. The conflict is cultural and domestic, not military — that is what makes it unusual."),

    ("englit:3.2.2p", &[
        "Group the poems by power of humans, power of nature, and the effects of conflict",
        "Group them by identity and memory, and by loss and grief",
        "For any named poem, name two you could sensibly pair it with",
        "Know which poems share form — monologue, sonnet, free verse",
    ], "The exam names one poem and you choose the other. Pre-planning your pairs saves several minutes of panic."),

    ("englit:3.2.2q", &[
        "Structure a comparison by idea, not poem by poem",
        "Use comparative connectives so the comparison is explicit",
        "Compare methods — form, structure, imagery — not just content",
        "Weight both poems roughly equally",
    ], "Writing all about poem one then all about poem two caps your marks. Compare inside each paragraph."),

    ("englit:3.2.3a", &[
        "Read for the central idea before analysing anything",
        "Analyse language, form and structure in an unfamiliar poem",
        "Build a reading you can support, rather than hunting for devices",
        "Work to a time limit so the second question is not rushed",
    ], "Spotting a device scores nothing on its own. Say what the effect is and how it serves the poem's idea."),

    ("englit:3.2.3b", &[
        "Find a genuine point of comparison between two unseen poems",
        "Compare methods rather than summarising each in turn",
        "Handle the shorter mark allocation with a tighter answer",
        "Leave enough time — this question comes last and is often rushed",
    ], "The second unseen question is worth far fewer marks than the first. Spend time in proportion, not in panic."),

    ("englit:3.3a", &[
        "Write about form: sonnet, monologue, free verse, and why it matters",
        "Write about structure: shifts, volta, cyclical endings, stanza shape",
        "Analyse a single word closely rather than listing devices",
        "Embed short quotations inside your own sentences",
    ], "Feature-spotting is the commonest way to lose AO2. Always answer \"so what?\" after naming anything."),

    ("englit:3.3b", &[
        "Use context to illuminate a specific moment in the text",
        "Bring in the writer's purpose and intended audience",
        "Discuss how different readers might respond, then and now",
        "Keep context to a sentence woven into the argument",
    ], "Context is worth the fewest marks of the three main objectives. A history paragraph costs you time you needed for analysis."),

    // ---------- English Language (AQA GCSE 8700) ----------
    ("englang:1.1a", &[
        "List four things from a stated section of the text",
        "Stay inside the lines the question specifies",
        "Keep each answer short and literal",
        "Finish in about five minutes and move on",
    ], "Four marks, four points, five minutes. Straying outside the given lines scores zero however good the answer."),

    ("englang:1.1b", &[
        "Analyse words, phrases, language features and sentence forms",
        "Explain effects on the reader rather than naming techniques",
        "Zoom in on individual words within a quotation",
        "Use subject terminology accurately where it helps",
    ], "\"The writer uses a simile\" is worth nothing by itself. The mark is in what the simile makes the reader feel or see."),

    ("englang:1.1c", &[
        "Write about the whole extract: beginning, shifts, ending",
        "Explain where the focus narrows or widens, and why",
        "Comment on the order of information and what it withholds",
        "Avoid drifting back into language analysis",
    ], "This is the question most people answer wrongly, by writing about language again. Structure means what comes when."),

    ("englang:1.1d", &[
        "Take a clear position on the statement in the question",
        "Evaluate the writer's methods, not just the events",
        "Support with a range of evidence across the second half of the text",
        "Sustain a critical argument rather than describing",
    ], "It is worth 20 marks — the most on the paper. Plan it, and give it the time the mark allocation deserves."),

    ("englang:1.2a", &[
        "Describe a scene using varied senses and precise detail",
        "Use extended imagery rather than a scatter of unrelated devices",
        "Vary sentence length deliberately for pace",
        "Structure a description with a clear shape, not a list",
    ], "A description still needs shape — a movement, a shift, a return. Wandering loses the structure marks."),

    ("englang:1.2b", &[
        "Plan a narrative that fits the time available: one moment, done well",
        "Establish a voice and hold it",
        "Use dialogue sparingly and for a reason",
        "Resolve or land the ending deliberately",
    ], "An over-ambitious plot is the classic failure. A small moment written precisely beats an epic left unfinished."),

    ("englang:1.2c", &[
        "Open in a way that earns the reader's attention immediately",
        "End with control rather than a sudden stop or a dream",
        "Punctuate accurately, including for effect",
        "Vary vocabulary and sentence structure, and proofread",
    ], "Sixteen of the forty marks are for technical accuracy. Five minutes of proofreading is worth more than another paragraph."),

    ("englang:2.1a", &[
        "Work quickly through the true or false statements",
        "Check each against the stated section only",
        "Shade exactly the number of boxes asked for",
        "Spend no more than five minutes",
    ], "Shading more boxes than asked loses marks. Count them before you move on."),

    ("englang:2.1b", &[
        "Summarise differences or similarities between the two texts",
        "Infer rather than merely quote and describe",
        "Use linking language to keep both texts in view",
        "Stay focused on the specific focus of the question",
    ], "This is summary and inference, not language analysis. Writing about methods here wastes the marks entirely."),

    ("englang:2.1c", &[
        "Analyse how language is used in one named non-fiction text",
        "Cover words, phrases, features and sentence forms",
        "Explain the effect on the reader's view of the subject",
        "Keep to the text the question names",
    ], "Non-fiction language questions reward rhetorical devices with a purpose — persuading, dismissing, appealing."),

    ("englang:2.1d", &[
        "Compare the writers' attitudes, not just their subjects",
        "Compare the methods each uses to convey that attitude",
        "Sustain comparison throughout with comparative connectives",
        "Support with well-chosen evidence from both texts",
    ], "Sixteen marks and the hardest question on the paper. Compare viewpoints and methods together, not one then the other."),

    ("englang:2.2a", &[
        "Take a clear line and hold it throughout",
        "Use rhetorical methods deliberately: anecdote, statistic, triple, direct address",
        "Anticipate and answer a counter-argument",
        "Match tone to the audience the question gives you",
    ], "A one-sided rant scores less than an argument that concedes something. Acknowledging the other side is a mark of control."),

    ("englang:2.2b", &[
        "Adapt to the form named: letter, article, speech, essay",
        "Use the conventions of that form without wasting time on layout",
        "Adjust register for the stated audience",
        "Keep purpose in view in every paragraph",
    ], "Elaborate letter headings and addresses earn nothing. Signal the form quickly and spend the time on the writing."),

    ("englang:2.2c", &[
        "Plan a shape: opening, development, counter, conclusion",
        "Use paragraphing and discourse markers to guide the reader",
        "Write accurately under time pressure",
        "Leave five minutes to check",
    ], "Same as Paper 1: sixteen of forty marks are technical accuracy. Accuracy is the cheapest improvement available to you."),

    ("englang:3a", &[
        "Choose a subject you can speak about for several minutes",
        "Structure the talk with a clear opening and close",
        "Use notes without reading from them",
        "Use standard English and vary tone and pace",
    ], "It is separately endorsed and does not affect your grade, but it is compulsory — not doing it breaches the specification."),

    ("englang:3b", &[
        "Listen to the question and answer what was asked",
        "Develop answers beyond a single sentence",
        "Handle a challenge without becoming defensive",
        "Use appropriate register throughout",
    ], "The questions are part of the assessment, not an afterthought. Prepare for the obvious ones."),

    // ---------- Biology (4BI1) ----------
    ("bio:1a", &[
        "List the characteristics all living organisms share",
        "Describe the common features of plants, animals, fungi and protoctists",
        "Describe the features of prokaryotes such as bacteria",
        "Explain what a pathogen is, and give examples across the groups",
    ], "\"Movement\" and \"nutrition\" mean something specific in biology. Learn the eight characteristics as a list you can reel off."),

    ("bio:2a", &[
        "Describe the levels of organisation from organelle to organism",
        "Name the cell structures and state the function of each",
        "Compare plant and animal cells",
        "Explain cell differentiation and the arguments around stem cells",
    ], "Cell wall and cell membrane are not the same thing, and plants have both. Mixing them up is the standard slip."),

    ("bio:2b", &[
        "Identify the elements in carbohydrates, proteins and lipids, and their building blocks",
        "Carry out and interpret the food tests for glucose, starch, protein and fat",
        "Explain enzymes as biological catalysts and the active site",
        "Explain how temperature and pH affect enzyme activity",
    ], "Enzymes denature; they are not \"killed\". Using the right word is worth a mark in most enzyme questions."),

    ("bio:2c", &[
        "Define diffusion, osmosis and active transport, and say how they differ",
        "Explain which of the three needs energy and why",
        "Explain how surface area, distance, concentration gradient and temperature affect rate",
        "Investigate osmosis in living and non-living systems",
    ], "Osmosis is water moving down its own gradient through a partially permeable membrane. Saying \"water moves to where there is less water\" loses the mark."),

    ("bio:2d", &[
        "State the word and balanced symbol equations for photosynthesis",
        "Explain how light intensity, carbon dioxide and temperature affect the rate",
        "Explain how the leaf is adapted for photosynthesis",
        "Explain why plants need magnesium and nitrate ions",
    ], "Limiting factors questions want you to say which factor is limiting at which part of the graph, not just that the rate rises."),

    ("bio:2e", &[
        "Describe a balanced diet and the function of each nutrient group",
        "Describe the alimentary canal and the function of each part",
        "Explain the role of digestive enzymes and of bile",
        "Explain how the small intestine is adapted for absorption",
    ], "Bile is not an enzyme. It emulsifies fat and neutralises stomach acid — saying it digests fat is wrong."),

    ("bio:2f", &[
        "State the word and symbol equations for aerobic respiration",
        "State the word equations for anaerobic respiration in plants and animals",
        "Explain the role of ATP",
        "Compare aerobic and anaerobic respiration by product and by energy released",
    ], "Respiration is not breathing. Every year candidates lose marks by describing ventilation when asked about respiration."),

    ("bio:2g", &[
        "Explain gas exchange in a leaf and the role of stomata",
        "Explain why net gas exchange differs between day and night",
        "Explain how the leaf is adapted for gas exchange",
        "Investigate the effect of light on net gas exchange",
    ], "Plants respire day and night. The common error is saying they only respire in the dark."),

    ("bio:2h", &[
        "Describe the structure of the thorax and the mechanism of ventilation",
        "Explain the role of the intercostal muscles and diaphragm",
        "Explain how alveoli are adapted for gas exchange",
        "Explain the biological consequences of smoking",
    ], "Air moves because of pressure changes, not because muscles \"pull air in\". Describe the pressure change to get the mark."),

    ("bio:2i", &[
        "Describe the role of xylem and of phloem",
        "Explain how water is absorbed by root hair cells",
        "Define transpiration and explain what affects its rate",
        "Investigate environmental factors affecting water uptake",
    ], "Xylem carries water up only; phloem carries dissolved sugars both ways. Getting the direction wrong undoes the answer."),

    ("bio:2j", &[
        "Describe the components of blood and the role of each",
        "Explain how red blood cells are adapted for oxygen transport",
        "Explain the immune response and how vaccination produces immunity",
        "Explain the role of platelets in clotting",
    ], "Antibodies are made by white blood cells and are specific to one antigen. \"White blood cells eat germs\" is not enough at this level."),

    ("bio:2k", &[
        "Describe the structure of the heart and how it pumps blood",
        "Explain how heart rate changes during exercise and why",
        "Compare arteries, veins and capillaries and relate structure to function",
        "Describe the general layout of the circulatory system, and risk factors for heart disease",
    ], "Humans have a double circulation — blood passes through the heart twice per circuit. Drawing a single loop loses marks."),

    ("bio:2l", &[
        "Name the excretory products of the lungs, kidneys and skin",
        "Describe the urinary system and the structure of a nephron",
        "Explain ultrafiltration and selective reabsorption",
        "Explain the role of ADH in regulating water content",
    ], "Filtration is not selective; reabsorption is. Reversing those two is the classic kidney error."),

    ("bio:2m", &[
        "Define homeostasis and explain why it matters",
        "Explain how the skin controls body temperature",
        "Describe geotropic and phototropic responses in plants",
        "Explain the role of auxin in the phototropic response",
    ], "Auxin accumulates on the shaded side and makes those cells elongate, so the shoot bends towards the light. Say the mechanism, not just the outcome."),

    ("bio:2n", &[
        "Explain how the central nervous system coordinates a response",
        "Describe the reflex arc from receptor to effector",
        "Explain the role of neurotransmitters at a synapse",
        "Describe the structure of the eye and how it focuses on near and distant objects",
    ], "In accommodation, the ciliary muscles contract for near objects and the lens becomes fatter. The muscle-and-ligament logic is what examiners look for."),

    ("bio:2o", &[
        "Name the main hormones, their sources and their effects",
        "Compare nervous and hormonal communication",
        "Explain the role of adrenaline",
        "Explain how insulin and glucagon regulate blood glucose",
    ], "Hormones are slower, longer-lasting and travel in the blood; nerves are fast and short-lived. That comparison is asked almost every year."),

    ("bio:3a", &[
        "Compare sexual and asexual reproduction",
        "Describe insect-pollinated and wind-pollinated flowers",
        "Explain fertilisation and the growth of the pollen tube",
        "Investigate the conditions needed for germination",
    ], "Pollination and fertilisation are different events. Pollination moves pollen; fertilisation is the fusion of nuclei."),

    ("bio:3b", &[
        "Describe the male and female reproductive systems",
        "Explain the roles of oestrogen and progesterone in the menstrual cycle",
        "Explain the roles of FSH and LH",
        "Describe the placenta and how the embryo is protected",
    ], "Learn the menstrual cycle as four hormones acting in sequence. Naming hormones without their timing scores little."),

    ("bio:3c", &[
        "Explain what the genome is and what a gene is",
        "Describe DNA as a double helix of two strands with complementary bases",
        "Compare DNA and RNA",
        "Describe transcription and translation",
    ], "Protein synthesis has two stages in two places — transcription in the nucleus, translation at the ribosome. Say where as well as what."),

    ("bio:3d", &[
        "Use the terms allele, dominant, recessive, homozygous, heterozygous, genotype, phenotype",
        "Complete monohybrid crosses and predict the ratios",
        "Interpret family pedigree diagrams",
        "Explain codominance and how sex is determined",
    ], "Always give the parents' genotypes and a labelled Punnett square. A ratio with no working is worth almost nothing."),

    ("bio:3e", &[
        "Explain how mitosis produces genetically identical diploid cells",
        "Explain where mitosis occurs: growth, repair, asexual reproduction",
        "Explain how meiosis produces haploid, genetically varied gametes",
        "Know the human diploid and haploid numbers",
    ], "Meiosis halves the chromosome number and produces variation; mitosis does neither. Confusing them is the commonest genetics error."),

    ("bio:3f", &[
        "Explain the causes of variation, genetic and environmental",
        "Explain mutation as a rare random change, and what increases its rate",
        "Explain how a DNA change can alter a protein",
        "Explain Darwin's theory of natural selection, and antibiotic resistance as an example",
    ], "Organisms do not adapt in order to survive. Variation exists first, then selection acts on it — phrasing it the other way loses the mark."),

    ("bio:4a", &[
        "Define population, community, habitat and ecosystem",
        "Explain how abiotic and biotic factors affect populations",
        "Investigate population size using quadrats and transects",
        "Explain what biodiversity means",
    ], "Quadrat questions want the calculation: mean per quadrat, scaled to the whole area. Show the scaling."),

    ("bio:4b", &[
        "Name the trophic levels and use food chains, webs and pyramids",
        "Explain the transfer of substances and energy along a food chain",
        "Explain why only about 10% of energy passes to the next level",
        "Predict the effect of removing an organism from a food web",
    ], "Energy is lost as heat, in movement, and in waste — name the routes rather than just saying energy is lost."),

    ("bio:4c", &[
        "Describe the stages of the carbon cycle",
        "Describe the stages of the nitrogen cycle and the bacteria involved",
        "Explain the role of decomposers in both",
        "Explain why these cycles matter to ecosystems",
    ], "The nitrogen cycle needs four named groups of bacteria doing four different jobs. Learn them by job, not just by name."),

    ("bio:4d", &[
        "Explain how human activity increases greenhouse gases",
        "Explain the biological consequences of global warming",
        "Explain the effects of water and air pollution, including eutrophication",
        "Explain the effects of deforestation",
    ], "Eutrophication is a sequence: fertiliser, algal bloom, light blocked, plants die, bacteria multiply, oxygen falls, fish die. Marks come from the order."),

    ("bio:5a", &[
        "Explain how glasshouses and polythene tunnels increase yield",
        "Explain the effects of light, carbon dioxide and temperature on crop yield",
        "Explain how fertiliser increases yield",
        "Discuss the reasons for and against pest control",
    ], "This section rewards linking back to photosynthesis. An answer that never mentions limiting factors is missing the biology."),

    ("bio:5b", &[
        "Explain the role of yeast in bread and in alcohol production",
        "Investigate anaerobic respiration by yeast",
        "Explain the role of Lactobacillus in yoghurt production",
        "Explain the use of an industrial fermenter and the conditions it controls",
    ], "Fermenter questions want the reason for each condition — why that temperature, why sterile, why stirred. Listing conditions is not enough."),

    ("bio:5c", &[
        "Explain how selective breeding produces desired characteristics",
        "Give examples in plants and in animals",
        "Explain the process over successive generations",
        "Discuss the drawbacks, including reduced variation",
    ], "Selective breeding is not genetic modification. No genes are transferred — humans just choose who breeds."),

    ("bio:5d", &[
        "Explain how restriction enzymes and ligase are used to cut and join DNA",
        "Explain how plasmids and viruses act as vectors",
        "Explain how human insulin is produced by genetically modified bacteria",
        "Discuss genetically modified plants and what transgenic means",
    ], "Sticky ends produced by the same restriction enzyme are what let the gene join the plasmid. That detail is where the marks are."),

    ("bio:5e", &[
        "Describe micropropagation and what it is used for",
        "Describe the stages in producing cloned mammals",
        "Explain how cloned transgenic animals are produced",
        "Discuss the advantages and drawbacks of cloning",
    ], "Micropropagation clones plants from tissue; nuclear transfer clones animals. They are different processes — do not blur them."),

    // ---------- Chemistry (4CH1) ----------
    ("chem:1a", &[
        "Describe the arrangement, movement and energy of particles in each state",
        "Explain melting, boiling, freezing, condensing and sublimation in particle terms",
        "Explain diffusion and what affects its rate",
        "Interpret experimental results as evidence for the particle model",
    ], "Say what the particles are doing, not just what the substance looks like. Every state-change mark is in the particle description."),

    ("chem:1b", &[
        "Define solubility in g per 100 g of solvent",
        "Plot and read a solubility curve",
        "Work out what mass crystallises when a solution cools",
        "Investigate the solubility of a solid at different temperatures",
    ], "Solubility is per 100 g of water. Forgetting to scale to the actual mass of water is the standard calculation error."),

    ("chem:1c", &[
        "Classify substances as elements, compounds or mixtures",
        "Explain why a pure substance has a sharp melting point",
        "Describe filtration, crystallisation, simple and fractional distillation, chromatography",
        "Calculate and interpret Rf values",
    ], "Pure in chemistry means one substance only, not \"natural\" or \"clean\". Exam questions exploit the everyday meaning."),

    ("chem:1d", &[
        "Describe the atom in terms of protons, neutrons and electrons",
        "Use atomic number and mass number, and calculate relative atomic mass from isotopes",
        "Deduce electronic configurations for the first 20 elements",
        "Explain why elements in a group react similarly, and why noble gases are unreactive",
    ], "Relative atomic mass from isotopes is a weighted mean, not a simple average. Multiply each mass by its abundance."),

    ("chem:1e", &[
        "Write word equations and balanced symbol equations with state symbols",
        "Calculate relative formula mass, including for hydrated compounds",
        "Balance equations reliably, including ionic ones",
        "Deduce a formula from the charges on the ions",
    ], "You may change coefficients but never subscripts. Rewriting H₂O as H₂O₂ to balance is the classic destructive error."),

    ("chem:1f", &[
        "Use the mole as the unit for amount of substance",
        "Convert between mass, moles and relative formula mass",
        "Calculate reacting masses from a balanced equation",
        "Calculate percentage yield and identify the limiting reactant",
    ], "Work in moles, not grams, when using the equation ratio. Applying the ratio directly to masses gives the wrong answer every time."),

    ("chem:1g", &[
        "Define empirical and molecular formula",
        "Calculate an empirical formula from masses or percentages",
        "Deduce a molecular formula from the empirical formula and relative formula mass",
        "Determine the formula of a metal oxide experimentally",
    ], "Divide by relative atomic mass, then by the smallest result. Skipping the second division is where most people stop too early."),

    ("chem:1h", &[
        "Explain how ions form by electron loss or gain, and know the common charges",
        "Draw dot-and-cross diagrams for ionic compounds",
        "Explain ionic bonding as electrostatic attraction between oppositely charged ions",
        "Explain why ionic compounds have high melting points and conduct only when molten or dissolved",
    ], "Ionic compounds conduct when molten or in solution because the ions are then free to move. \"The electrons move\" is wrong."),

    ("chem:1i", &[
        "Explain a covalent bond as a shared pair of electrons",
        "Draw dot-and-cross diagrams for simple molecules",
        "Explain why simple molecular substances have low melting and boiling points",
        "Explain the trend in boiling point down a homologous series",
    ], "Melting a simple molecular substance breaks the weak forces between molecules, not the covalent bonds. Say which you mean."),

    ("chem:1j", &[
        "Explain why giant covalent structures have very high melting points",
        "Explain the structure and properties of diamond and graphite",
        "Explain the structure of silicon dioxide",
        "Explain why graphite conducts and diamond does not",
    ], "Graphite conducts because each carbon bonds to only three others, leaving one delocalised electron. That detail is the mark."),

    ("chem:1k", &[
        "Describe a metallic lattice as positive ions in a sea of delocalised electrons",
        "Explain metallic bonding as attraction between ions and delocalised electrons",
        "Explain conductivity, malleability and high melting point from the structure",
        "Explain why covalent compounds do not conduct",
    ], "Malleability comes from layers of ions sliding while the bonding stays intact. Saying metals are \"soft\" misses the mechanism."),

    ("chem:1l", &[
        "Explain electrolysis and identify anode, cathode, anion and cation",
        "Predict the products of electrolysis for molten compounds and solutions",
        "Write ionic half-equations for the reactions at each electrode",
        "Investigate the electrolysis of aqueous solutions",
    ], "Oxidation happens at the anode, reduction at the cathode. Half-equations must balance charge as well as atoms."),

    ("chem:2a", &[
        "Describe the reactions of lithium, sodium and potassium with water",
        "Explain the similarities in their reactions from electronic configuration",
        "Explain the trend in reactivity down the group",
        "Predict the properties of other Group 1 elements",
    ], "Reactivity increases down Group 1 because the outer electron is further from the nucleus and more easily lost. Give the reason, not just the trend."),

    ("chem:2b", &[
        "Know the colours and states of the halogens at room temperature",
        "Describe displacement reactions between halogens and halide solutions",
        "Explain the trend in reactivity down Group 7",
        "Predict the properties of other halogens",
    ], "Group 7 reactivity decreases down the group — the opposite of Group 1. Mixing up the directions is very common."),

    ("chem:2c", &[
        "Know the approximate percentages of gases in clean air",
        "Determine the percentage of oxygen in air experimentally",
        "Describe the combustion of elements in oxygen",
        "Describe the formation of carbon dioxide and explain the greenhouse effect",
    ], "Air is about 78% nitrogen and 21% oxygen. Quoting these the wrong way round undermines the whole answer."),

    ("chem:2d", &[
        "Arrange metals in a reactivity series from their reactions with water and acid",
        "Use displacement reactions to place a metal in the series",
        "Know the conditions needed for iron to rust",
        "Explain methods of rust prevention, including sacrificial protection",
    ], "Rusting needs both water and oxygen. Answers naming only one lose the mark straight away."),

    ("chem:2e", &[
        "Explain why the extraction method depends on position in the reactivity series",
        "Describe the extraction of a metal by reduction and by electrolysis",
        "Explain the uses of aluminium, copper, iron and steel from their properties",
        "Explain what an alloy is and why alloys are harder than pure metals",
    ], "Alloys are harder because different-sized atoms disrupt the regular layers, so they cannot slide. That sentence is the mark."),

    ("chem:2f", &[
        "Describe the colours of litmus, phenolphthalein and methyl orange in acid and alkali",
        "Use the pH scale and universal indicator",
        "Explain acids as sources of H⁺ ions and alkalis as sources of OH⁻ ions",
        "Explain acids and bases as proton donors and acceptors",
    ], "An alkali is a soluble base. Using the two words interchangeably costs marks in definition questions."),

    ("chem:2g", &[
        "Describe the reactions of acids with metals, metal oxides, hydroxides and carbonates",
        "Use the general rules for predicting the solubility of salts",
        "Describe how to prepare a pure, dry sample of a soluble salt",
        "Describe how to prepare an insoluble salt by precipitation",
    ], "Salt preparation questions want the full method: excess solid, filter, evaporate, crystallise, dry. Missing a step loses a mark each."),

    ("chem:2h", &[
        "Describe the tests for hydrogen, oxygen, carbon dioxide, ammonia and chlorine",
        "Carry out a flame test and know the colours produced",
        "Describe the tests for the common cations and anions",
        "Describe the chemical and physical tests for water",
    ], "Give the test and the positive result. \"Use limewater\" without \"turns milky\" is only half an answer."),

    ("chem:3a", &[
        "Distinguish exothermic and endothermic reactions by temperature change",
        "Calculate heat energy change and molar enthalpy change from calorimetry",
        "Draw and interpret energy level diagrams",
        "Use bond energies to calculate the enthalpy change of a reaction",
    ], "Bond breaking is endothermic, bond making exothermic. Getting these round the wrong way inverts every calculation."),

    ("chem:3b", &[
        "Describe experiments to investigate rate of reaction",
        "Explain the effects of surface area, concentration, temperature and pressure",
        "Explain the effect of a catalyst in terms of activation energy",
        "Draw and interpret reaction profile diagrams",
    ], "Explain rate through collision frequency and energy. \"The particles move faster\" alone is not a full explanation."),

    ("chem:3c", &[
        "Explain what a reversible reaction is and give examples",
        "Explain dynamic equilibrium in a closed system",
        "Predict the effect of changing temperature, pressure or concentration",
        "Explain why a catalyst does not affect the position of equilibrium",
    ], "A catalyst speeds up both directions equally, so equilibrium is reached sooner but the position does not move."),

    ("chem:4a", &[
        "Define hydrocarbon, homologous series, functional group and isomer",
        "Represent organic molecules by empirical, molecular, general, structural and displayed formulae",
        "Name compounds using the standard rules",
        "Write the possible structural isomers for a given formula",
    ], "Isomers must have the same molecular formula but a genuinely different structure. Redrawing the same molecule bent differently is not an isomer."),

    ("chem:4b", &[
        "Explain what crude oil is and how fractional distillation separates it",
        "Name the main fractions and their uses",
        "Explain the trend in colour, boiling point and viscosity down the column",
        "Relate the trends to molecule size",
    ], "Separation depends on boiling point, which depends on chain length. Say the mechanism, not just that the fractions come off at different heights."),

    ("chem:4c", &[
        "Define a fuel and describe complete and incomplete combustion",
        "Explain why carbon monoxide is poisonous",
        "Explain how oxides of nitrogen form in car engines",
        "Explain how sulfur dioxide and nitrogen oxides cause acid rain",
    ], "Incomplete combustion gives carbon monoxide and carbon, not just less energy. Name the products."),

    ("chem:4d", &[
        "Describe how long-chain alkanes are cracked",
        "Explain why cracking is necessary",
        "Write balanced equations for cracking reactions",
        "Explain the economic reason: supply and demand of fractions",
    ], "Cracking equations must balance — check the carbons and hydrogens on both sides before moving on."),

    ("chem:4e", &[
        "Know the general formula for alkanes and explain why they are saturated",
        "Draw structural and displayed formulae for the first four alkanes",
        "Describe the reaction of alkanes with halogens in the presence of light",
        "Explain substitution as the reaction type",
    ], "Alkanes are relatively unreactive. Their halogen reaction needs ultraviolet light — leaving that condition out loses the mark."),

    ("chem:4f", &[
        "Know the general formula for alkenes and explain why they are unsaturated",
        "Draw structural and displayed formulae for the first three alkenes",
        "Describe the reactions of alkenes with bromine, hydrogen and steam",
        "Use bromine water to distinguish alkanes from alkenes",
    ], "Bromine water goes from orange to colourless with an alkene — not \"clear\", which it already is. That word costs marks."),

    ("chem:4g", &[
        "Know the alcohol functional group and general formula",
        "Draw structural and displayed formulae for the first four alcohols",
        "Describe the oxidation of ethanol by air and by oxidising agents",
        "Compare fermentation with hydration of ethene as manufacturing routes",
    ], "Fermentation and hydration questions want a comparison — rate, purity, cost, renewability — not a description of one route."),

    ("chem:4h", &[
        "Know the carboxylic acid functional group and draw the first four",
        "Describe the reactions of carboxylic acids with metals, bases and carbonates",
        "Explain esterification and name the ester formed",
        "Prepare a sample of an ester and describe its properties",
    ], "Naming esters trips people up: the alcohol gives the first part, the acid the second. Ethanol plus ethanoic acid gives ethyl ethanoate."),

    ("chem:4i", &[
        "Explain addition polymerisation from alkene monomers",
        "Draw the repeat unit from a monomer, and deduce the monomer from a repeat unit",
        "Explain condensation polymerisation and the loss of a small molecule",
        "Explain the problems of disposing of addition polymers",
    ], "Repeat units need the trailing bonds through the brackets and an n outside. Drawing a molecule instead of a repeat unit loses the marks."),

    // ---------- Physics (4PH1) ----------
    ("phys:1a", &[
        "Use the standard units for distance, time, speed, force and mass",
        "Plot and interpret distance-time graphs, reading speed from the gradient",
        "Plot and interpret velocity-time graphs, reading acceleration from the gradient",
        "Find distance travelled from the area under a velocity-time graph",
    ], "Gradient of a distance-time graph is speed; gradient of a velocity-time graph is acceleration. Confusing the two graphs is the commonest error in this topic."),

    ("phys:1b", &[
        "Use average speed = distance ÷ time",
        "Use acceleration = change in velocity ÷ time",
        "Use v² = u² + 2as and rearrange it confidently",
        "Investigate the motion of everyday objects experimentally",
    ], "Deceleration is negative acceleration. Dropping the minus sign turns a correct method into a wrong answer."),

    ("phys:1c", &[
        "Distinguish vector and scalar quantities and give examples",
        "Explain that force is a vector with magnitude and direction",
        "Calculate the resultant of forces acting along the same line",
        "Identify the different types of force in a situation",
    ], "Balanced forces mean constant velocity, not necessarily rest. An object can move steadily with zero resultant force."),

    ("phys:1d", &[
        "Use force = mass × acceleration",
        "Use weight = mass × gravitational field strength, and distinguish mass from weight",
        "Explain Newton's third law as equal and opposite forces on different objects",
        "Explain friction and its effects",
    ], "Mass is in kilograms and does not change; weight is a force in newtons and does. Swapping them is guaranteed to lose marks."),

    ("phys:1e", &[
        "Explain stopping distance as thinking distance plus braking distance",
        "Describe the factors affecting each part",
        "Describe the forces on a falling object and explain terminal velocity",
        "Explain why terminal velocity is reached",
    ], "At terminal velocity the resultant force is zero, so acceleration is zero — but the object is still moving fast. Saying it stops is wrong."),

    ("phys:1f", &[
        "Investigate how extension varies with applied force",
        "Explain the initial linear region of a force-extension graph",
        "Describe elastic behaviour and the limit of proportionality",
        "Use force = spring constant × extension",
    ], "Use the extension, not the total length of the spring. Forgetting to subtract the original length is the standard slip."),

    ("phys:1g", &[
        "Use momentum = mass × velocity",
        "Use conservation of momentum to solve collision problems",
        "Use the relationship between force, change in momentum and time",
        "Explain safety features such as crumple zones in momentum terms",
    ], "Momentum is a vector — give opposite directions opposite signs before adding, or collision answers come out wrong."),

    ("phys:1h", &[
        "Use moment = force × perpendicular distance from the pivot",
        "Apply the principle of moments to a balanced beam",
        "Explain that weight acts through the centre of gravity",
        "Explain how upward forces on a beam relate to its stability",
    ], "The distance must be perpendicular to the force. Using the length along a tilted beam gives the wrong moment."),

    ("phys:2a", &[
        "Use the units for current, voltage, resistance, charge and power",
        "Use voltage = current × resistance",
        "Explain why current in a resistor causes heating",
        "Explain how insulation and fuses protect a circuit",
    ], "Rearranging V = IR under pressure is where marks go. Practise all three forms until it is automatic."),

    ("phys:2b", &[
        "Explain why a series or parallel circuit is used in a given situation",
        "Explain how current behaves in series and in parallel circuits",
        "Explain how voltage divides in series and is the same across parallel branches",
        "Calculate currents, voltages and resistances in both types of circuit",
    ], "In parallel the voltage is the same across each branch and the current splits. In series it is the other way round."),

    ("phys:2c", &[
        "Describe how current varies with voltage for a resistor, a filament lamp and a diode",
        "Describe the effect of temperature on a thermistor",
        "Describe the effect of light on an LDR",
        "Explain the uses of lamps and LEDs as indicators",
    ], "A filament lamp's I–V graph curves because resistance rises with temperature. Drawing it straight loses the physics."),

    ("phys:2d", &[
        "Use energy transferred = current × voltage × time",
        "Use power = current × voltage, and calculate running costs",
        "Explain the difference between mains AC and battery DC supply",
        "Explain the earth wire, fuses and circuit breakers",
    ], "The fuse rating must be just above the normal operating current. Choosing one below it means the appliance never works."),

    ("phys:2e", &[
        "Explain current as the rate of flow of charge",
        "Use charge = current × time",
        "Explain that current in solid metals is a flow of electrons",
        "Explain why current is conserved at a junction",
    ], "Electrons flow from negative to positive; conventional current is drawn the other way. Know which one the question means."),

    ("phys:2f", &[
        "Identify materials that are electrical conductors or insulators",
        "Explain charging by friction in terms of electron transfer",
        "Explain attraction and repulsion between charges",
        "Explain the dangers and the uses of static electricity",
    ], "Only electrons move — protons never do. Answers describing positive charge moving onto an object are wrong."),

    ("phys:3a", &[
        "Explain the difference between longitudinal and transverse waves",
        "Define amplitude, wavefront, frequency, wavelength and period",
        "Use wave speed = frequency × wavelength and frequency = 1 ÷ period",
        "Explain reflection, refraction and diffraction for all waves",
    ], "Waves transfer energy without transferring matter. That sentence is worth a mark on its own and is often forgotten."),

    ("phys:3b", &[
        "Know the order of the electromagnetic spectrum by wavelength and frequency",
        "Explain uses of each part of the spectrum",
        "Explain the detrimental effects of each part",
        "Know that all electromagnetic waves travel at the same speed in a vacuum",
    ], "Learn the order in one direction and stick to it. Reversing radio and gamma inverts every wavelength answer."),

    ("phys:3c", &[
        "Use the law of reflection with angles measured from the normal",
        "Draw ray diagrams for reflection in a plane mirror",
        "Explain refraction as a change in speed at a boundary",
        "Draw ray diagrams showing refraction through a block",
    ], "All angles are measured from the normal, never from the surface. Measuring from the surface makes every value wrong."),

    ("phys:3d", &[
        "Use refractive index = sin i ÷ sin r",
        "Investigate refraction experimentally",
        "Explain total internal reflection and the critical angle",
        "Explain the use of total internal reflection in optical fibres and prisms",
    ], "Total internal reflection needs light going from a denser to a less dense medium, above the critical angle. Both conditions are needed."),

    ("phys:3e", &[
        "Explain that sound waves are longitudinal and need a medium",
        "Know the frequency range for human hearing",
        "Investigate the speed of sound",
        "Explain how pitch and loudness relate to frequency and amplitude on an oscilloscope",
    ], "Pitch is frequency and loudness is amplitude. Swapping them is the single most common sound error."),

    ("phys:4a", &[
        "Describe energy transfers between stores",
        "Use the principle of conservation of energy",
        "Describe everyday energy transfers with correct terminology",
        "Explain why energy is never created or destroyed",
    ], "Energy is not \"used up\" or \"lost\" — it is transferred, usually to the surroundings as heat. The wording is assessed."),

    ("phys:4b", &[
        "Use efficiency = useful energy out ÷ total energy in",
        "Express efficiency as a decimal and as a percentage",
        "Draw and interpret Sankey diagrams",
        "Explain how to reduce wasted energy in a system",
    ], "Efficiency can never exceed 100%. Getting a figure above that means the useful and total values are the wrong way round."),

    ("phys:4c", &[
        "Explain thermal transfer by conduction in solids",
        "Explain convection in fluids and link it to density changes",
        "Explain thermal radiation and what affects emission and absorption",
        "Explain methods of reducing heat loss from a building",
    ], "Conduction needs particles in contact, so it does not happen in a vacuum — but radiation does. Match the mechanism to the situation."),

    ("phys:4d", &[
        "Use work done = force × distance moved in the direction of the force",
        "Use kinetic energy = ½ × mass × speed²",
        "Use gravitational potential energy = mass × g × height",
        "Use power = work done ÷ time, and apply conservation of energy to transfers",
    ], "Kinetic energy squares the speed, so doubling speed quadruples the energy. Forgetting to square is the standard mistake."),

    ("phys:4e", &[
        "Describe the energy transfers in the main generating methods",
        "Compare renewable and non-renewable resources",
        "Describe the advantages and disadvantages of each resource",
        "Explain the role of different resources in meeting demand",
    ], "\"Renewable\" is not the same as \"no environmental impact\". The best answers weigh reliability, cost and impact together."),

    ("phys:5a", &[
        "Use density = mass ÷ volume",
        "Investigate density for regular solids, irregular solids and liquids",
        "Use pressure = force ÷ area",
        "Explain how pressure varies with depth in a liquid, and use p = hρg",
    ], "Volume must be in consistent units before you divide. Mixing cm³ and m³ is where density answers go wrong by a factor of a million."),

    ("phys:5b", &[
        "Explain why heating a system raises temperature or changes state",
        "Describe what happens to particles during a change of state",
        "Use energy = mass × specific heat capacity × temperature change",
        "Obtain and interpret a temperature-time graph, and investigate specific heat capacity",
    ], "During a change of state the temperature stays constant while energy is still supplied. The flat part of the graph is the mark."),

    ("phys:5c", &[
        "Explain gas pressure in terms of molecular collisions",
        "Explain why there is an absolute zero of temperature",
        "Convert between the Celsius and Kelvin scales",
        "Explain why raising temperature raises the average kinetic energy of molecules",
    ], "Kelvin is Celsius plus 273. Gas law calculations must use Kelvin or they are simply wrong."),

    ("phys:5d", &[
        "Explain how pressure changes with volume at constant temperature",
        "Use p₁V₁ = p₂V₂",
        "Explain how pressure changes with temperature at constant volume",
        "Use the pressure-temperature relationship with temperature in Kelvin",
    ], "Convert to Kelvin before doing anything else. Using Celsius in a gas law is the single biggest source of lost marks here."),

    ("phys:6a", &[
        "Describe attraction and repulsion between magnetic poles",
        "Describe the properties of magnetically hard and soft materials",
        "Draw magnetic field patterns and explain induced magnetism",
        "Investigate the magnetic field of a bar magnet and of a current-carrying wire",
    ], "Field lines run from north to south outside the magnet and never cross. Both details are marked."),

    ("phys:6b", &[
        "Explain why a force acts on a current-carrying conductor in a magnetic field",
        "Use the left-hand rule to predict the direction of the force",
        "Describe how the force varies with current and field strength",
        "Explain the operation of a simple motor",
    ], "Left hand for motors. Using the right hand here is a very easy way to get every direction backwards."),

    ("phys:6c", &[
        "Explain that a voltage is induced when a conductor cuts field lines",
        "Describe the factors affecting the size of the induced voltage",
        "Describe the generation of electricity by a rotating coil",
        "Explain the difference between a motor and a generator",
    ], "Induction needs relative movement or a changing field. A stationary magnet in a stationary coil induces nothing."),

    ("phys:6d", &[
        "Describe the structure of a transformer",
        "Explain the use of step-up and step-down transformers in the National Grid",
        "Use the turns-ratio relationship between voltage and number of turns",
        "Use the relationship for an ideal transformer, VpIp = VsIs",
    ], "High voltage means low current, so less energy is lost as heat in the cables. That reasoning is the point of the whole topic."),

    ("phys:7a", &[
        "Describe the atom in terms of protons, neutrons and electrons",
        "Use atomic number and mass number, and explain what an isotope is",
        "Describe the nature of alpha, beta and gamma radiation",
        "Investigate the penetrating power of each type",
    ], "Alpha is stopped by paper, beta by aluminium, gamma reduced by lead. Learn the three absorbers as a set."),

    ("phys:7b", &[
        "Describe the effect of alpha and beta decay on atomic and mass number",
        "Balance nuclear equations",
        "Explain why nuclei decay",
        "Explain ionisation and why alpha is the most ionising",
    ], "Mass number and atomic number must each balance across a nuclear equation. Check both before moving on."),

    ("phys:7c", &[
        "Explain activity and the becquerel",
        "Define half-life",
        "Use half-life to calculate remaining activity or elapsed time",
        "Explain background radiation and its sources, and how radiation is detected",
    ], "Half-life questions need the number of halvings, not a subtraction. Halve repeatedly and count."),

    ("phys:7d", &[
        "Describe uses of radioactivity in medicine and industry",
        "Explain the difference between contamination and irradiation",
        "Describe the dangers of ionising radiation",
        "Explain how sources are chosen for a given use",
    ], "Contamination is having the source on or in you; irradiation is being exposed to it. The distinction is examined directly."),

    ("phys:7e", &[
        "Explain how a U-235 nucleus undergoes fission",
        "Explain how a chain reaction is set up and controlled",
        "Describe the role of the moderator, control rods and shielding",
        "Explain the products of fission and the problem of nuclear waste",
    ], "Moderator slows neutrons, control rods absorb them. Swapping the two roles is the classic reactor mistake."),

    ("phys:7f", &[
        "Describe fusion as the joining of light nuclei to form a heavier one",
        "Explain that fusion is the energy source of stars",
        "Explain why fusion does not happen easily on Earth",
        "Compare fusion with fission",
    ], "Fusion needs enormous temperature and pressure to overcome electrostatic repulsion. That is why it is hard, and it is the mark."),

    ("phys:8a", &[
        "Explain gravitational field strength and why it differs between bodies",
        "Explain how gravitational force provides the centripetal force for an orbit",
        "Describe the differences between the orbits of comets, moons and planets",
        "Use the relationship between orbital speed, radius and period",
    ], "Gravity is what keeps something in orbit, not what it is escaping. Orbit means continuously falling around the body."),

    ("phys:8b", &[
        "Explain how stars are classified by colour and surface temperature",
        "Describe the life cycle of a star of similar mass to the Sun",
        "Describe the life cycle of a star with much greater mass",
        "Explain the role of fusion at each stage",
    ], "The two life cycles diverge after the red giant stage. Learn them as two branches from one starting point."),

    ("phys:8c", &[
        "Draw and interpret the main components of a Hertzsprung–Russell diagram",
        "Explain how the brightness of a star depends on temperature and size",
        "Explain red shift and what it tells us about the universe",
        "Explain the evidence for the Big Bang, including microwave background radiation",
    ], "Red shift shows galaxies moving away, and more distant ones moving faster. That second half is what supports expansion."),

    // ---------- Geography (AQA GCSE Geography (8035)) ----------
    ("geog:3.1.1.1", &[
        "Define a natural hazard and say when a hazard becomes a disaster",
        "Sort hazards into tectonic, atmospheric, hydrological and geomorphological types with an example of each",
        "Explain hazard risk using hazard, vulnerability and capacity to cope",
        "Explain how urbanisation, poverty, farming and climate change each raise hazard risk, with a named place for each",
        "Judge which factor matters most, using a contrast such as Haiti and Chile in 2010",
    ], "Listing urbanisation, poverty, farming and climate change without saying why each one raises risk stays in Level 1. Every factor needs a mechanism and a named place."),

    ("geog:3.1.1.2a", &[
        "Describe the Earth's layers and the differences between oceanic and continental crust",
        "Explain plate tectonics theory, including convection currents and slab pull",
        "Describe the global distribution of earthquakes and volcanoes from a map, including exceptions such as Hawaii",
        "Explain the processes at constructive, destructive, collision and conservative margins that cause earthquakes and eruptions",
        "Explain why there are no volcanoes at conservative and collision margins",
    ], "Earthquake answers that just say the plates rub together lose marks. You need the chain: friction locks the plates, pressure builds, then it is released suddenly."),

    ("geog:3.1.1.2b", &[
        "Separate primary and secondary effects, and immediate and long-term responses, of an earthquake",
        "Recall the key facts of the Chile earthquake of 2010: causes, effects and responses",
        "Recall the key facts of the Nepal earthquake of 2015: causes, effects and responses",
        "Compare the two and explain how wealth shaped the differences",
        "Weigh wealth against other factors such as magnitude, remoteness and time of day",
    ], "People put the Chile tsunami or the Everest avalanches under primary effects. Both are secondary. Vague case-study facts also cap answers, so learn a small set of accurate numbers."),

    ("geog:3.1.1.2c", &[
        "Explain why people keep living near volcanoes and fault lines, with named places such as Etna and Iceland",
        "Describe how volcanoes are monitored and explain why earthquakes cannot be predicted",
        "Explain how earthquake-resistant buildings and other protection reduce risk",
        "Explain how hazard mapping, drills and evacuation planning reduce risk",
        "Evaluate which management approach works best for which hazard, and how wealth changes that",
    ], "Monitoring and prediction get mixed up. Monitoring collects the warning signs and prediction uses them to forecast. Every point should end with how it saves lives or property."),

    ("geog:3.1.1.3a", &[
        "Draw and label the three-cell model with the pressure belts and surface winds",
        "Describe the global distribution of tropical storms and name them by ocean",
        "Explain how tropical storms relate to the ITCZ, the trade winds and the Coriolis effect",
        "Explain the conditions and sequence of a tropical storm's formation, including latent heat",
        "Describe the structure of a tropical storm from the eye to the rain bands",
        "Explain how climate change might affect tropical storms' distribution, frequency and intensity",
    ], "Formation answers often leave out where the energy comes from: latent heat released when water vapour condenses. Many also put the strongest winds in the eye when they are in the eyewall."),

    ("geog:3.1.1.3b", &[
        "Separate the primary effects of a tropical storm from its secondary effects, and label each one",
        "Describe the effects of Typhoon Haiyan (Philippines, November 2013) with figures: 6,300 dead, a 5 m surge in Tacloban, 4.1 million displaced",
        "Separate immediate responses from long-term responses to Haiyan, and say who carried each out",
        "Explain how monitoring, prediction, protection and planning each reduce the effects of tropical storms",
        "Judge how effective the responses to a tropical storm were, and why",
    ], "Disease, homelessness and rising food prices are secondary effects, not primary ones. Put them under the wrong heading and they earn nothing, however well you explain them."),

    ("geog:3.1.1.3c", &[
        "Name the main UK weather hazards and give the weather pattern and a real example for each",
        "Explain the physical and human causes of the Somerset Levels floods of 2013-14",
        "Describe the social, economic and environmental impacts of the floods, with figures",
        "Explain how the management strategies (dredging, raised roads, pumping, the Bridgwater barrier) reduce the flood risk",
        "Use records and data as evidence that UK weather is becoming more extreme",
    ], "Answers mix up social, economic and environmental impacts. \"Homes were flooded\" earns nothing on a question about environmental impacts, so check the heading before you write."),

    ("geog:3.1.1.4", &[
        "Describe the evidence for climate change since the start of the Quaternary: ice cores, sediments, pollen, tree rings, historical and temperature records",
        "Explain how orbital changes, volcanic activity and solar output change the climate",
        "Explain how fossil fuels, agriculture and deforestation enhance the greenhouse effect",
        "Outline the effects of climate change on people and on the environment",
        "Tell mitigation apart from adaptation, give real examples of each and weigh them up",
    ], "Students mix up mitigation and adaptation. Mitigation reduces the causes (wind farms, carbon capture, the Paris Agreement), while adaptation responds to the effects (sea walls, drought-resistant crops)."),

    ("geog:3.1.2.1", &[
        "Define an ecosystem and sort its parts into biotic and abiotic",
        "Use a UK pond to name producers, consumers and decomposers, and draw a food chain and a food web",
        "Explain how nutrients are cycled between plants, animals, dead matter and the soil or water",
        "Explain the chain of effects when one part changes, e.g. eutrophication after fertiliser runoff",
        "Describe where the world's main biomes are found and what each one is like",
    ], "When one part of an ecosystem changes, students jump straight from cause to result (\"fertiliser gets in, so fish die\"). Every link in the chain is a mark, so write each step: algal bloom, light blocked, plants die, decomposers use up oxygen."),

    ("geog:3.1.2.2a", &[
        "Describe the climate, soils and vegetation layers of a tropical rainforest, quoting figures from a climate graph",
        "Explain how climate, water, soils, plants, animals and people in the rainforest depend on each other",
        "Explain how plants such as buttress roots, drip tips, lianas and epiphytes are adapted to the conditions",
        "Explain how animals such as the sloth, spider monkey and orangutan are adapted",
        "Explain why rainforest biodiversity is so high and why losing it matters",
    ], "Answers name an adaptation and stop there. Link the feature to the condition it copes with and say how that helps: drip tips shed heavy rain, so algae cannot grow on the leaf."),

    ("geog:3.1.2.2b", &[
        "Describe changing rates of deforestation, globally and in Brazil and Malaysia, using figures",
        "Explain the causes of deforestation in Malaysia with an example for each: palm oil, logging, roads, mining, the Bakun Dam, FELDA settlement, population growth",
        "Explain the impacts of deforestation: economic development, soil erosion and the contribution to climate change",
        "Explain why rainforests are valuable to people and the environment",
        "Evaluate selective logging, conservation, ecotourism, hardwood agreements and debt-for-nature swaps using real examples",
    ], "The case study is too vague: \"trees are cut down for farming\" stays in Level 1. Name the Malaysian detail, e.g. the second-largest palm oil producer, 14.4% of forest lost from 2000 to 2012, and the Bakun Dam flooding about 700 km²."),

    ("geog:3.1.2.3a", &[
        "Describe where hot deserts are and explain why they form around 30° north and south",
        "Describe the physical characteristics of hot deserts: climate, soils, landscape and vegetation",
        "Explain how climate, water, soils, plants, animals and people in a hot desert depend on each other",
        "Explain how named plants and animals are adapted to heat and drought",
        "Explain why hot desert biodiversity is low and vulnerable, with examples such as the addax",
    ], "A camel's hump stores fat, not water. Saying water loses the mark, and every adaptation needs the 'so that' explaining why it helps in the desert."),

    ("geog:3.1.2.3b", &[
        "Use the Sahara to explain development opportunities from mineral extraction, energy, farming and tourism",
        "Explain how extreme temperatures, water supply and inaccessibility make developing the Sahara difficult",
        "Define desertification and locate the Sahel on the fringe of the Sahara",
        "Explain the six causes of desertification and how they feed each other into soil erosion",
        "Evaluate strategies to reduce desertification: water and soil management, tree planting and appropriate technology",
    ], "Case-study answers without named places and facts (Noor Ouarzazate, Hassi Messaoud, magic stones in Burkina Faso) cannot reach the top level. Desertification is land degradation on the desert fringe, not the desert moving."),

    ("geog:3.1.3.1", &[
        "Locate the UK's major upland areas on a map, including the Grampians, Lake District, Pennines and Snowdonia",
        "Locate the major lowland areas, including the Fens, East Anglia and the London Basin",
        "Locate the major river systems, including the Severn, Thames, Trent, Tees and Tay",
        "Describe the upland–lowland pattern using the Tees–Exe line and name its exceptions",
        "Explain the pattern using rock type, glaciation and relief rainfall",
    ], "Describe means the pattern with compass directions and named examples, not an explanation. The North and South Downs are lowland chalk hills, not uplands."),

    ("geog:3.1.3.2a", &[
        "Explain how waves form and compare constructive and destructive waves",
        "Explain mechanical and chemical weathering: freeze-thaw, salt weathering and carbonation",
        "Describe and explain sliding, slumping and rock falls",
        "Explain erosion by hydraulic power, abrasion and attrition",
        "Explain longshore drift step by step, and why sediment is deposited at the coast",
    ], "Weathering breaks rock down in place; erosion removes it. In longshore drift the backwash runs straight down the beach because of gravity, not back out at an angle."),

    ("geog:3.1.3.2b", &[
        "Explain how rock type and geological structure produce discordant and concordant coasts",
        "Explain the formation of headlands and bays, cliffs and wave-cut platforms, and caves, arches and stacks",
        "Explain the formation of beaches, sand dunes, spits and bars",
        "Identify the major landforms of erosion and deposition on the Dorset coast around Swanage",
    ], "Formation answers must name the process at each stage, in order. A description of shapes with no hydraulic power, abrasion or longshore drift stays in the bottom level."),

    ("geog:3.1.3.2c", &[
        "Explain the costs and benefits of sea walls, rock armour, gabions and groynes",
        "Explain the costs and benefits of beach nourishment, reprofiling and dune regeneration",
        "Explain managed retreat and why it is used, using Medmerry as an example",
        "Use Lyme Regis to explain the reasons for management, the strategy, and its effects and conflicts",
        "Evaluate how successful a coastal management scheme has been",
    ], "Costs and benefits questions need both, each developed. The cost examiners want most is that groynes starve beaches further down the coast of sediment."),

    ("geog:3.1.3.3a", &[
        "Describe how a river's long profile changes from source to mouth, using the word concave and the change in gradient",
        "Describe how the cross profile changes from a steep V-shaped valley to a wide, flat valley floor",
        "Explain how width, depth, discharge and velocity change downstream, using the Bradshaw model",
        "Explain the four erosion processes: hydraulic action, abrasion, attrition and solution",
        "Explain how a river transports its load by traction, saltation, suspension and solution",
        "Explain why and where deposition happens when a river loses energy",
    ], "Attrition and abrasion get swapped more than anything else. Attrition is rocks knocking into each other and wearing down the load; abrasion is rocks scraping the bed and banks and wearing away the channel."),

    ("geog:3.1.3.3b", &[
        "Explain how interlocking spurs, waterfalls and gorges form through erosion in the upper course",
        "Explain how meanders and ox-bow lakes form through a combination of erosion and deposition",
        "Explain how flood plains, levées and estuaries form through deposition in the lower course",
        "Draw and annotate diagrams that show the stages in how each landform forms",
        "Identify the major landforms of the River Tees, including High Force, the meanders near Yarm and the estuary at Seal Sands",
    ], "On an explain question, describing the landform earns almost nothing. You have to give the sequence of formation, for example hard rock over soft rock, then undercutting, an overhang, collapse and retreat to form a gorge."),

    ("geog:3.1.3.3c", &[
        "Explain how physical factors (precipitation, geology, relief) and human factors (land use) increase or reduce flood risk",
        "Label a flood hydrograph and measure lag time and peak discharge from it",
        "Explain why some hydrographs are flashy and others are subdued",
        "Assess the costs and benefits of hard engineering: dams and reservoirs, straightening, embankments and flood relief channels",
        "Assess the costs and benefits of soft engineering: flood warnings, flood plain zoning, planting trees and river restoration",
        "Use the Banbury scheme to explain why it was needed, what was built, and its social, economic and environmental issues",
    ], "Lag time is measured from peak rainfall to peak discharge, not from the start of the rain, and it needs a unit. Reading it from the wrong point on the hydrograph loses the mark every time."),

    ("geog:3.2.1a", &[
        "Describe the global pattern of urban change and use data to compare urbanisation in HICs, NEEs and LICs",
        "Explain how rural-urban migration drives urbanisation, using push and pull factors",
        "Explain how natural increase adds to urban growth, linking it to young migrants",
        "Explain what a megacity is and describe where megacities are emerging",
        "Use figures from graphs and maps to describe urban trends",
    ], "Urbanisation is an increase in the proportion of people living in towns and cities, not just cities getting bigger. HICs have the highest level of urbanisation but the slowest rate of change."),

    ("geog:3.2.1b", &[
        "Describe Rio de Janeiro's location and explain its regional, national and international importance",
        "Explain the causes of Rio's growth: natural increase and migration",
        "Explain the social and economic opportunities created by urban growth in Rio",
        "Explain the challenges of urban growth in Rio: favelas, water, sanitation, energy, services, unemployment, crime and pollution",
        "Evaluate how far the Favela Bairro Project improved quality of life for the urban poor",
    ], "Generic answers such as 'favelas have no clean water' stay in the lower levels. Use Rio-specific evidence, and when you evaluate Favela Bairro include its problems as well as its successes before giving a judgement."),

    ("geog:3.2.1c", &[
        "Describe the distribution of population and major cities in the UK",
        "Describe Bristol's location and explain its importance within the UK and the wider world",
        "Explain how national and international migration have affected Bristol's growth and character",
        "Explain the social, economic and environmental opportunities created by urban change in Bristol",
        "Explain the challenges of urban change in Bristol, including inequality, dereliction, brownfield versus greenfield sites, and waste",
        "Explain the impact of urban sprawl on the rural-urban fringe and the growth of commuter settlements",
    ], "Answers that say what changed but not how it affects people stay in the lower levels. Finish every chain with the effect on residents, and back it with a Bristol fact."),

    ("geog:3.2.1d", &[
        "Explain why Bristol's Temple Quarter needed regeneration, using derelict and brownfield land as evidence",
        "Describe the main features of the Temple Quarter project: the Enterprise Zone, Engine Shed, the university campus and the Temple Meads upgrade",
        "Explain how water and energy conservation, waste recycling and green space make urban living sustainable, using Freiburg and Vauban",
        "Explain how transport strategies such as MetroBus, park and ride and London's Congestion Charge reduce traffic congestion",
        "Judge how successful regeneration and transport schemes have been, and for whom",
    ], "Reasons for regeneration are the problems; features of the project are the solutions. Answers that blur the two, or list features without saying what problem each one fixes, stay in the lower levels."),

    ("geog:3.2.2a", &[
        "Compare ways of classifying countries: HIC/NEE/LIC, World Bank income groups, HDI categories and the Brandt Line",
        "Define GNI per head, birth and death rates, infant mortality, life expectancy, people per doctor, literacy, access to safe water and HDI, and say which are economic and which social",
        "Explain the limitations of each measure, especially averages hiding inequality",
        "Explain how a country's stage on the Demographic Transition Model links to its level of development",
        "Use development data tables and graphs, quoting figures with the correct units",
    ], "The death rate is not a reliable sign of poverty: ageing HICs like the UK can have higher death rates than LICs with young populations. Saying 'rich countries have low death rates' loses the mark."),

    ("geog:3.2.2b", &[
        "Explain the physical, economic and historical causes of uneven development",
        "Describe the consequences of uneven development: gaps in wealth and health, and international migration",
        "Outline how investment, industrial development, tourism, aid, intermediate technology, fairtrade, debt relief and microfinance reduce the development gap, with a benefit and a drawback for each",
        "Use Jamaica to explain how tourism helps close the development gap, and why leakage limits the gains",
        "Judge how far a strategy can reduce the gap, giving evidence on both sides",
    ], "Naming a cause without the 'so' that shows how it holds development back. 'Chad is landlocked' is a statement; 'so its exports cost more to reach a port' is where the mark is."),

    ("geog:3.2.2c", &[
        "Describe Nigeria's location and explain its regional and global importance",
        "Explain Nigeria's political, social, cultural and environmental context",
        "Describe how Nigeria's industrial structure has changed and explain how manufacturing stimulates development",
        "Weigh the advantages and disadvantages of TNCs such as Shell and Unilever to Nigeria",
        "Explain Nigeria's changing political and trading links, the types and impacts of aid, and the environmental effects of development",
        "Judge how far economic development has improved quality of life, and for whom",
    ], "Generic answers that could fit any country. Every paragraph needs a Nigerian fact such as Bodo, the Niger Delta, Dangote or ECOWAS, or the answer is capped at Level 1 or 2."),

    ("geog:3.2.2d", &[
        "Explain how de-industrialisation, globalisation and government policies have changed the UK economy",
        "Describe the move to a post-industrial economy and explain why science parks such as Cambridge Science Park locate where they do",
        "Explain how Torr Quarry reduces the environmental impact of industry",
        "Compare the social and economic changes in South Cambridgeshire (growth) and the Outer Hebrides (decline)",
        "Evaluate new road, rail, port and airport developments and the strategies used to reduce the north-south divide",
        "Describe the UK's place in the world through trade, culture, transport, communications, the EU and the Commonwealth",
    ], "Rural change needs both areas and both kinds of change, social and economic. Answers that describe only the growing area, or only house prices, can't reach the top level."),

    ("geog:3.2.3.1", &[
        "Explain why food, water and energy matter to economic and social well-being, and how the three are linked",
        "Describe global inequalities in the supply and consumption of resources, and give reasons for them",
        "Explain the UK trends in food: high-value imports from LICs, food miles and local sourcing, and agribusiness",
        "Describe the UK's areas of water surplus and deficit, and explain why transfers such as Kielder and the Elan Valley are needed",
        "Describe how the UK energy mix has changed and assess the economic and environmental issues of exploiting different energy sources",
    ], "Deficit means demand is greater than supply, and the deficit is in the south-east, not the north-west. Swapping them throws away the easiest marks in the section."),

    ("geog:3.2.3.2a", &[
        "Define food security and food insecurity, and separate areas of surplus from areas of deficit",
        "Describe the global pattern of calorie intake and food supply from a world map, quoting values",
        "Explain how rising population and economic development increase food consumption",
        "Explain how climate, technology, pests and disease, water stress, conflict and poverty affect food supply, with named examples",
        "Explain the impacts of food insecurity: famine, undernutrition, soil erosion, rising prices and social unrest",
    ], "Explaining rising food demand only through population growth. Economic development and the shift to meat and dairy is the other half of the answer, and the mark scheme expects both."),

    ("geog:3.2.3.2b", &[
        "Explain how irrigation, hydroponics and aeroponics, the new green revolution, biotechnology and appropriate technology increase food supply",
        "Evaluate the advantages and disadvantages of the Indus Basin Irrigation System in Pakistan, using named dams and facts",
        "Explain how organic farming, permaculture, urban farming, sustainable fish and meat, seasonal eating and less waste make food supply more sustainable",
        "Explain how sand dams in Makueni County, Kenya, increase sustainable food supplies in an LIC",
        "Judge whether large-scale or local schemes are the better way to feed people sustainably",
    ], "Calling a scheme sustainable without saying why. Name the reason: local materials and labour, community ownership, no fuel, no damage to the environment."),

    ("geog:3.3.1", &[
        "Describe how the pre-release resource booklet works and use the twelve weeks to prepare",
        "Identify the stakeholders in an issue and explain their conflicting viewpoints, using a conflict matrix",
        "Assess options by their social, economic and environmental impacts, at different scales and over short and long time periods",
        "Explain the links between physical and human impacts of a proposal",
        "Make a clear decision and justify it in a 9-mark answer, using booklet evidence, admitting its drawbacks and rejecting the alternatives",
    ], "A 9-mark decision with no specific evidence from the resource booklet cannot reach the top level, however good the argument. Quote the figures and data."),

    ("geog:3.3.2", &[
        "Name the six stages of a geographical enquiry and say what each one involves",
        "Explain what makes a suitable enquiry question, the theory behind it, and how fieldwork risks are reduced",
        "Describe and justify data collection methods, including random, systematic and stratified sampling",
        "Choose, use and adapt presentation methods, and analyse results using statistics, links between data sets and anomalies",
        "Draw evidenced conclusions and evaluate the accuracy and reliability of your own two enquiries",
    ], "Own-enquiry answers written in general terms. Without your place, your sites, your equipment and your actual results the answer is held at the bottom level."),

    ("geog:3.4a", &[
        "Use latitude and longitude, and describe distributions and patterns on atlas maps with evidence and exceptions",
        "Give four- and six-figure grid references and measure straight and curved distances at 1:50 000 and 1:25 000",
        "Read height from contours and spot heights, recognise relief features, calculate gradient and draw a cross-section",
        "Describe river and coastal landscapes from an OS map and infer settlement, communications, land use and tourism",
        "Interpret ground, aerial and satellite photographs, draw sketch maps and field sketches, and annotate them",
    ], "Giving northings before eastings in a grid reference loses the whole mark. Along the corridor first, then up the stairs."),

    ("geog:3.4b", &[
        "Choose, justify and draw the right graph for a data set, from bar charts and histograms to scattergraphs and population pyramids",
        "Complete and interpret choropleth, isoline, dot, desire-line, proportional-symbol and flow-line maps",
        "Calculate mean, median, mode, range, quartiles, interquartile range and percentage change, showing working and units",
        "Describe relationships on a scattergraph, draw a line of best fit, and interpolate or extrapolate from it",
        "Design a data collection sheet and judge the accuracy, sample size and reliability of data",
        "Spot how selective presentation of statistics can mislead, and write a well-evidenced conclusion",
    ], "Percentage change is divided by the original value, not the new one, and a median needs the data in rank order first. Calculations with no working or no units throw away marks even when the method is right."),

    ("spa:4.4", &[
        "Use the five minutes' reading time to underline key words and predict the Spanish you will hear",
        "Spot distractors: plans that change, past habits against present ones, and negatives such as ya no, nunca and tampoco",
        "Decide between true, false and not mentioned from the evidence in the recording",
        "Transcribe a Higher dictation using the Spanish sound-spelling rules, including words not on the list",
        "Place written accents correctly using the stress rules",
    ], "Writing the first detail that fits, when a later pero, al final or ya no changes the answer."),
    ("spa:4.5", &[
        "Plan the 15 minutes' preparation across the role-play, the reading-aloud text and the photo card",
        "Convey each role-play message without ambiguity, in the right time frame, and ask the required question",
        "Read a text aloud with accurate Spanish sounds and stress",
        "Describe both photos in detail and develop every answer in the unprepared conversation",
        "Use opinions with reasons, three time frames and listed Higher structures to reach the top band",
    ], "Giving minimal answers in the conversation instead of developing each one with a reason, an example and another time frame."),
    ("spa:4.6", &[
        "Answer each Section A question type: multiple choice, which person, true or false or not mentioned, and short answers",
        "Infer feelings and attitudes from clues in a text",
        "Work out unknown words from context and from the endings -ito, -ísimo, -mente and -idad",
        "Translate Spanish into natural English element by element, with every tense, person and negative correct",
        "Recognise false friends such as actual, éxito, sensible, largo and asistir",
    ], "Losing a translation element by missing a small word such as ya no, todavía or cada, or by getting the tense wrong."),
    ("spa:4.7", &[
        "Translate five English sentences into Spanish, conveying all 15 elements accurately",
        "Plan and write the 90-word task, covering all three bullets in past, present and future",
        "Plan and write the 150-word task, developing both bullets with regular, accurate complex language",
        "Use listed Higher structures safely: the subjunctive after cuando, que and para que, desde hace, acabar de, ya no and lo + adjective",
        "Check verbs, agreements and accents in a final pass",
    ], "Copying English structures into Spanish, such as soy 15 for I am 15 or he estado jugando por for I have been playing for."),

    ("ger:4.4", &[
        "Use the five minutes' reading time to underline key words and predict the German you will hear",
        "Spot distractors: früher against jetzt, corrections with sondern, and negatives such as nicht, kein, nie and nicht mehr",
        "Decide between true, false and not mentioned from the evidence in the recording",
        "Transcribe a Higher dictation with the German sound-spelling rules: ei and ie, w, z, sch, sp and st, ch, umlauts, ß and -er",
        "Spell words from outside the vocabulary list from their sounds, and use grammar to fix capitals and endings",
    ], "Writing down the first detail that fits, when a nicht, kein or jetzt later in the sentence changes the answer."),
    ("ger:4.5", &[
        "Plan the 15 minutes' preparation across the role-play, the reading-aloud text and the photo card",
        "Convey each role-play message without ambiguity, in the time frame the task sets, and ask the required question",
        "Read a text aloud with accurate German sounds, especially ei, ie, w, z, sp, st, ch, umlauts and final -er",
        "Describe both photos in detail and develop every answer in the unprepared conversation",
        "Build extended answers with a reason, an example, a second time frame and a conditional such as wenn ich ... hätte, würde ich",
    ], "Giving short, minimal answers in the conversation, which caps the mark however accurate the German is."),
    ("ger:4.6", &[
        "Answer multiple-choice, which-person, true/false/not-mentioned and short-answer questions in English",
        "Tell not mentioned from false by finding contradicting evidence in the text",
        "Infer the meaning of unlisted words from prefixes, suffixes, compounds and context",
        "Recognise the genitive after wegen, trotz and während in Higher texts",
        "Translate German into natural English element by element, keeping tense, person, negatives and qualifiers",
    ], "Falling for false friends and tense traps: bekommen means get, also means so, and seit with the present means have been doing."),
    ("ger:4.7", &[
        "Translate English into German so that all 15 elements are conveyed, with accurate verbs, word order and endings",
        "Cover all three bullets of the 90-word task with all three time frames",
        "Cover both bullets of the 150-word task, with developed ideas and regular complex language",
        "Use Higher structures accurately: seit with the present, hätte, wäre and würde, sollte, and the simple past in written narrative",
        "Plan each extended answer in a few minutes and check verbs, word order and endings at the end",
    ], "Putting the verb in the wrong place: it comes second after an opening time phrase and at the end after weil, dass, wenn and obwohl."),

    ("fre:4.4a", &[
        "Use the five minutes' reading time to underline question words and predict French vocabulary",
        "Spot distractors created by negatives such as ne...plus, ne...que and ne...jamais",
        "Separate time frames using time words and tenses before choosing an answer",
        "Distinguish false from not mentioned in true / false / not mentioned questions",
        "Give precise English answers with exactly the number of details asked for",
    ], "Writing down the first keyword heard, when a negative, a time word or a mais later in the sentence makes it the wrong answer."),
    ("fre:4.4b", &[
        "Spell French sounds using the AQA sound-symbol correspondences",
        "Add the silent endings that grammar requires: plural -s, -ent verb endings and feminine -e",
        "Choose between é, -er and -ez using the grammar of the sentence",
        "Recognise liaison and nasal vowels without writing extra letters",
        "Build plausible spellings for the two words from outside the vocabulary list",
    ], "Leaving out silent endings and agreements, such as writing ils parle or elles sont arrivé, which the grammar mark punishes."),
    ("fre:4.5", &[
        "Convey each role-play task without ambiguity, including the question task",
        "Read a 50-word text aloud with accurate silent letters, liaisons and nasal vowels",
        "Describe both photos on the photo card clearly for about ninety seconds",
        "Develop conversation answers with opinions, reasons, examples and three time frames",
        "Use the 15 minutes' preparation and the 12-minute limit to best effect",
    ], "Giving minimal one-sentence answers in the conversation, which keeps the communication mark in the bottom bands however accurate they are."),
    ("fre:4.6", &[
        "Answer comprehension questions in precise English, avoiding negative and time-frame traps",
        "Infer feelings, attitudes and meanings that a text implies but does not state",
        "Work out unknown words from context, cognates and derivations such as -ment, -ion and -eur",
        "Translate a passage into natural English, rendering every tense and every word",
        "Recognise false friends such as journée, actuellement, car and sensible",
    ], "Losing translation sections by flattening tenses or leaving out small words such as souvent, déjà or ne...que."),
    ("fre:4.7", &[
        "Translate five English sentences into French so that all 15 elements are conveyed accurately",
        "Cover every bullet in the 90-word and 150-word tasks with developed ideas",
        "Use past, present and future time frames securely in extended writing",
        "Build in Higher structures from the AQA list with accurate verb forms",
        "Plan time and length across the 75-minute paper",
    ], "Missing a bullet point or writing Question 2 in only one time frame, which caps both the content and the language marks."),

    ("spa:3.2.1a", &[
        "Form feminine and plural nouns, including -z to -ces and -ión to -iones",
        "Use definite and indefinite articles where Spanish and English differ",
        "Make este, ese, cada, mismo, otro, todo, algún and ningún agree with their nouns",
        "Use the possessive adjectives mi, tu, su, nuestro and vuestro correctly",
        "Use an infinitive as a noun for the English -ing subject",
    ], "Possessives and determiners agree with the noun that follows, and general statements need the article: todos los días, mis amigos, me gusta la música."),
    ("spa:3.2.1b", &[
        "Leave out subject pronouns except for contrast or emphasis",
        "Place direct, indirect and reflexive pronouns with one verb, two verbs and commands",
        "Use que, esto, eso, alguno and ninguno as pronouns",
        "Form negatives with no, nada, nunca, nadie and ninguno",
        "Ask questions with intonation or a question word followed by the verb",
    ], "No goes before the object pronoun, never between it and the verb: No lo sé, not Lo no sé."),
    ("spa:3.2.1c", &[
        "Conjugate regular -ar, -er and -ir verbs in all six persons",
        "Apply the five stem-change clusters: encontrar, pensar, pedir, conocer and poner",
        "Use the irregular present of ser, estar, ir, tener and hacer",
        "Use tener with frío, calor, hambre, sed, miedo and años",
    ], "Stem-changers keep the infinitive stem in the nosotros and vosotros forms: podemos, not puedemos."),
    ("spa:3.2.1d", &[
        "Form the regular preterite with the correct accents",
        "Use the irregular preterite of ir, ser and dar",
        "Use the irregular stems tuv-, pud-, hic-, vin-, estuv-, pus-, quis-, dij- and traj-",
        "Recognise and write the yo spelling changes llegué, busqué and empecé",
    ], "Regular preterite endings need their accents (compré, compró), while irregular stems take none (tuve, hizo)."),
    ("spa:3.2.1e", &[
        "Form the present continuous with estar and the present participle",
        "Form the present perfect with haber and regular or listed irregular participles",
        "Use the imperfect singular for used to and was doing, including era, iba and veía",
        "Choose between the imperfect and the preterite in a past narrative",
    ], "Object pronouns and no go before haber, never between haber and the participle: Lo he visto."),
    ("spa:3.2.1f", &[
        "Talk about plans with ir a and an infinitive in every person",
        "Form the singular future and conditional, including tendr-, har-, podr- and pondr-",
        "Use habrá and habría for there will be and there would be",
        "Give positive tú commands, including sé, ve, ten, ven, haz, di, pon and sal",
    ], "Future and conditional endings go on the whole infinitive or the irregular stem: tendré and haré, never teneré or haceré."),
    ("spa:3.2.1g", &[
        "Use deber, poder, querer, tener que and saber with an infinitive",
        "Make gustar-type verbs agree with the thing liked",
        "Use reflexive verbs in the singular and plural, including each other",
        "Use hay, hay que, se puede, se necesita and hace with weather nouns",
    ], "Gustar agrees with the thing liked, not the person: me gustan los perros, a mi madre le gusta leer."),
    ("spa:3.2.1h", &[
        "Make adjectives agree in gender and number, including nationalities",
        "Place adjectives correctly, including buen, mal, primer, tercer and gran",
        "Choose ser or estar with adjectives, including listo and aburrido",
        "Compare with más, menos, tan ... como, mejor and peor",
        "Position adverbs of time, manner and place",
    ], "Adjectives agree with every noun they describe, including plurals and nationalities: mis amigas españolas son simpáticas."),
    ("spa:3.2.1i", &[
        "Use the personal a before a person as direct object",
        "Show possession with de instead of an apostrophe",
        "Follow para, sin, antes de and después de with an infinitive",
        "Use verbs that take a preposition, such as llegar a, dejar de and volver a",
        "Recognise and form words with -ito, -ísimo, -mente, -idad and -able",
    ], "The personal a before a person is often forgotten: Visito a mis abuelos and Conozco al hermano de Pedro both need it."),
    ("spa:3.2.2a", &[
        "Place nos and os as objects and plural reflexive pronouns with two verbs",
        "Use lo que, el que and el cual, and relative clauses with donde and cuando",
        "Use possessive pronouns and possessives after ser: el mío, es tuya",
        "Use pronouns after prepositions, conmigo, contigo and emphatic a mí",
        "Use aquel and aquello for that over there or long ago",
    ], "With me and with you are conmigo and contigo; con mí and con ti are always wrong."),
    ("spa:3.2.2b", &[
        "Form the future and conditional in all persons, with sabr-, querr-, vendr-, dir- and saldr-",
        "Form the imperfect in all persons, including éramos, íbamos and veíamos",
        "Apply the preterite stem changes pidió and durmió in the third persons",
        "Write the credit-bearing spelling changes cojo, llegué, busqué, empecé and leyó",
        "Give positive vosotros commands",
    ], "Preterite stem changes in -ir verbs apply only to the él and ellos forms: pidió and durmieron, but pedí and dormimos."),
    ("spa:3.2.2c", &[
        "Form the singular present subjunctive of hacer, ser, ir, venir and tener",
        "Use the subjunctive after cuando with a future meaning",
        "Use the subjunctive after que with wishing, commands, requests and emotions",
        "Use the subjunctive after para que, and the infinitive when the subject stays the same",
    ], "Cuando about the future takes the subjunctive, not the future: Cuando sea mayor, seré médico."),
    ("spa:3.2.2d", &[
        "Use acabar de, seguir and llevar with the right verb forms",
        "Say how long you have been doing something with desde hace and the present",
        "Form the passive with ser and por, and with se",
        "Use lo with adjectives and form superlatives with de",
        "Use ya no, tampoco and ni ... ni, and parece, basta, falta, hace falta and vale la pena",
    ], "Desde hace takes the present tense: Vivo aquí desde hace dos años, not He vivido aquí por dos años."),
    ("spa:3.2.3", &[
        "Pronounce the vowels and the listed letter groups when reading aloud",
        "Spell words heard in the dictation from the sound-symbol rules",
        "Find the stressed syllable of any word",
        "Decide when a written accent is needed",
    ], "Accents are left off words that break the stress rules: canción, fácil and música all need one."),

    ("spa:3.1.1a", &[
        "Describe family and friends using ser, estar, tener and llevar correctly",
        "Explain how you get on with people using llevarse bien/mal con and entenderse",
        "Give and justify opinions about relationships with gustar-type verbs such as encantar and molestar",
        "Talk about family life in the past, present and future, contrasting the imperfect and preterite",
        "Use Higher structures from the list, such as lo que más me importa and cuando sea mayor",
    ], "Using ser instead of tener for age, hair and eyes: tengo quince años, tiene los ojos azules."),
    ("spa:3.1.1b", &[
        "Describe your diet, exercise and sleep habits with frequency expressions",
        "Give advice with hay que, se debe and hace falta plus the infinitive",
        "Contrast old habits in the imperfect with current ones in the present and perfect",
        "Use doler and other gustar-type verbs with the correct agreement",
        "Explain plans for a healthier lifestyle with ir a, the future tense and quieren que haga",
    ], "Making doler and gustar-type verbs agree with the speaker instead of the thing: me duelen las piernas, me encantan las verduras."),
    ("spa:3.1.1c", &[
        "Give and justify opinions about school subjects, teachers and rules",
        "Describe school rules with se puede, no se puede, hay que and tener que",
        "Compare primary school in the imperfect with secondary school in the present",
        "Talk about part-time jobs and future careers, using ser plus a job with no article",
        "Use cuando tenga and quieren que vaya to talk about plans at Higher",
    ], "Writing me gusta with a plural subject: me gustan las matemáticas, me gustan las ciencias."),
    ("spa:3.1.2a", &[
        "Describe free-time activities using jugar a for sports and tocar for instruments",
        "Compare activities with más...que, menos...que and prefiero...porque",
        "Narrate an outing in the preterite with descriptions in the imperfect",
        "Order food, ask a question and deal with the bill in a restaurant role-play",
        "Use desde hace and llevar plus a gerund for activities you still do",
    ], "Confusing jugar and tocar, or writing gustan with infinitives: juego al fútbol, toco la guitarra, me gusta bailar."),
    ("spa:3.1.2b", &[
        "Explain what happens at the festivals on the AQA list: la Tomatina, las Fallas, los Sanfermines, la Semana Santa, el Día de Reyes, el Día de Muertos",
        "Give dates correctly and say where festivals take place, using se celebra at Higher",
        "Describe a celebration with the scene in the imperfect and the events in the preterite",
        "Give justified opinions on traditions, including controversial ones such as la corrida",
        "Use espero que haga, me alegra que venga and cuando vaya within the AQA subjunctive list",
    ], "Describing the scene of a festival in the preterite instead of the imperfect: había mucha gente, hacía calor."),
    ("spa:3.1.2c", &[
        "Describe a famous person and justify why you respect them",
        "Argue for and against celebrities as role models, using lo bueno and lo malo de",
        "Use the perfect and preterite for achievements and the conditional for would you like to be famous",
        "Use the personal a with seguir, conocer and respetar",
        "Use the passive with ser and esperar que with a listed subjunctive at Higher",
    ], "Leaving out the personal a before people: sigo a Rosalía, conocí a un actor, respeto a los deportistas."),
    ("spa:3.1.3a", &[
        "Describe a holiday with events in the preterite and weather and description in the imperfect",
        "Talk about transport, accommodation and places of interest in Spain and Latin America",
        "Use estar for location and temporary states, and ser for characteristics",
        "Explain a problem on a trip and how it was solved",
        "Describe future or ideal holidays with the future, the conditional and cuando vaya",
    ], "Putting the weather and descriptions of a past holiday in the preterite: hacía sol, el hotel era cómodo, había mucha gente."),
    ("spa:3.1.3b", &[
        "Explain how and why you use technology with usar...para plus the infinitive",
        "Weigh up the advantages and disadvantages of social media and reach a conclusion",
        "Replace nouns with direct object pronouns placed before the verb",
        "Compare past and present media habits using the imperfect veía and the present",
        "Use the passive with se and quieren que tenga at Higher",
    ], "Making gustar-type verbs singular with plural nouns: me molestan los anuncios, me encantan las series."),
    ("spa:3.1.3c", &[
        "Describe where you live using es, está and hay",
        "Compare town and countryside and your area now with how it used to be",
        "Explain environmental problems and their consequences with si plus present plus future",
        "Propose solutions with hay que, se debe and el gobierno debería",
        "Use quiero que sea and para que tenga within the AQA subjunctive list",
    ], "Mixing up es, está and hay: mi pueblo es pequeño, está en el norte y hay un río."),

    ("ger:3.2.1a", &[
        "Recall the gender and plural of listed nouns, including compound nouns and -in feminines",
        "Choose der, ein and kein correctly in the nominative, accusative and dative",
        "Add the dative plural -n and use kein instead of nicht ein",
        "Form nominalised infinitives such as das Schwimmen with a capital letter",
    ], "The masculine accusative is forgotten: Ich habe einen Bruder and Es gibt einen Park, not ein."),
    ("ger:3.2.1b", &[
        "Use dieser, jeder, welcher and the possessives with the right endings in three cases",
        "Replace nouns with singular accusative and dative pronouns such as ihn, sie, ihm and mir",
        "Build subject relative clauses with der, die and das and send the verb to the end",
        "Use accusative reflexive pronouns in all persons, including each other with uns and euch",
        "Ask questions with wer, wen and wem, and use jemand and niemand",
    ], "His and her are confused: sein means his, ihr means her and their, so it is ihre Mutter for her mother."),
    ("ger:3.2.1c", &[
        "Conjugate weak and strong verbs in the present in all persons, including vowel changes",
        "Use haben, sein, werden and wissen accurately, including haben with Hunger, Durst and Angst",
        "Form questions with verb-first order and with question words",
        "Use the present with a time phrase to refer to the future",
    ], "Ich bin spielen is written for I am playing; German uses the simple present, ich spiele."),
    ("ger:3.2.1d", &[
        "Keep the verb second and invert the subject after a time phrase or other opening",
        "Send the second verb to the end of a main clause",
        "Send the verb to the end after weil, dass, wenn and other subordinating conjunctions",
        "Split separable verbs in present-tense main clauses",
        "Place nicht, nie and nichts correctly and order phrases time, manner, place",
    ], "The verb is left in second place after weil or dass: it must be weil ich müde bin, not weil ich bin müde."),
    ("ger:3.2.1e", &[
        "Form past participles for weak, strong, -ieren, inseparable and separable verbs",
        "Choose haben or sein as the auxiliary and place the participle at the end",
        "Use früher with the perfect tense to say what you used to do",
        "Use war, hatte and es gab for was, had and there was",
    ], "Movement verbs and bleiben take sein: ich bin gefahren and wir sind geblieben, not ich habe gefahren."),
    ("ger:3.2.1f", &[
        "Form the future with werden and an infinitive at the end",
        "Conjugate the six modal verbs and möcht- in all persons of the present",
        "Use the singular past modals konnte, musste, wollte, durfte, sollte and mochte",
        "Build um ... zu, ohne ... zu and statt ... zu phrases and use hoffen and versuchen with zu",
    ], "Ich will means I want, not I will: I will is ich werde, and muss nicht means don't have to."),
    ("ger:3.2.1g", &[
        "Add the correct endings to adjectives after der-words and ein-words in three cases",
        "Use plural adjectives without an article and leave adjectives after sein uninflected",
        "Compare with als and so ... wie, including besser, höher, mehr and lieber",
        "Say what you like and prefer doing with gern and lieber",
    ], "After ein the adjective must show the gender: ein alter Mann, ein altes Haus, and every dative adjective ends in -en."),
    ("ger:3.2.1h", &[
        "Use the accusative after bis, durch, für and ohne and the dative after aus, bei, mit, nach, von and zu",
        "Choose accusative or dative after an, auf and in for movement or position",
        "Use the contractions am, im, ins, beim, vom, zum and zur",
        "Decode and build words with Lieblings-, Haupt-, un-, -ung, -er, -s and ordinal endings",
    ], "Mit and zu always take the dative: mit dem Bus and zur Schule, never mit den Bus."),
    ("ger:3.2.2a", &[
        "Form plurals of weak masculine nouns and use adjectival nouns such as die Reichen and etwas Neues",
        "Recognise the genitive after trotz, wegen and während and for possession in reading and listening",
        "Use plural object pronouns and order two objects correctly",
        "Build relative clauses with wo and was and use dative reflexive pronouns",
        "Decode nouns and adjectives made with -chen, -heit, -keit and -los",
    ], "With a pronoun and a noun the pronoun comes first: Ich gebe ihm das Buch, not Ich gebe das Buch ihm."),
    ("ger:3.2.2b", &[
        "Write a narrative in the simple past using weak verbs and the listed strong forms",
        "Use past modals in all persons",
        "Give commands with the du, ihr and Sie imperative, including sein",
        "Use seit with the present tense for how long something has been happening",
    ], "Seit needs the present tense: Ich lerne seit drei Jahren Deutsch, not ich habe seit drei Jahren gelernt."),
    ("ger:3.2.2c", &[
        "Conjugate hätte, wäre, würde and sollte in all persons",
        "Build wenn-sentences with verb, comma, verb word order",
        "Give advice and opinions about change with sollte and an infinitive",
        "Replace English passives with man and an active verb",
    ], "Leaving off the umlaut turns the conditional into the past: hätte and wäre mean would have and would be, hatte and war mean had and was."),
    ("ger:3.2.2d", &[
        "Send both verbs to the end of subordinate clauses, with the conjugated verb last",
        "Use separable verbs in subordinate clauses and correct with nicht ... sondern",
        "Use gegen, um, laut and the extra dual-case prepositions, and beim with an infinitive",
        "Replace preposition phrases with da- and wo- compounds",
        "Form superlatives before nouns and with am, including am besten and am liebsten",
    ], "In a two-verb subordinate clause the conjugated verb goes last: weil ich das Spiel verloren habe, not weil ich habe das Spiel verloren."),
    ("ger:3.2.3", &[
        "Pronounce the AQA sound-symbol correspondences, including w, v, z, sp, st, ei, ie and ch",
        "Distinguish long and short vowels and umlauts when reading aloud",
        "Spell unfamiliar words from their sounds in the dictation",
        "Apply capitals, ß or ss, and final -d, -g or -b correctly in transcription",
    ], "English sounds creep into reading aloud: w is said like English v, v like f, and z like ts."),

    ("ger:3.1.1a", &[
        "Describe your family and friends: appearance, personality and relationships",
        "Explain how you get on with people using sich verstehen mit and auskommen mit",
        "Give and justify opinions with weil, dass and Meiner Meinung nach",
        "Talk about family life and relationships in the past, present and future",
    ], "Mit takes the dative, so it is mit meiner Mutter, mit meinem Vater and mit meinen Eltern, never mit meine Mutter."),
    ("ger:3.1.1b", &[
        "Describe your diet, exercise and sleep habits with frequency expressions",
        "Contrast past and present habits using früher with the perfect and jetzt",
        "Discuss smoking, alcohol, drugs and stress with justified opinions",
        "Set healthy goals with werden, sollte and wenn ich ... hätte, würde ich",
    ], "Kein goes before a noun and nicht negates a verb: Ich trinke keinen Alkohol, but Ich rauche nicht."),
    ("ger:3.1.1c", &[
        "Describe your school, subjects, teachers and rules using modal verbs",
        "Compare German and British schools using Gymnasium, Abitur and Oberstufe",
        "Talk about part-time jobs and your plans for further study or a career",
        "Explain future plans with werden, möchte and um ... zu",
    ], "Jobs take no article after sein and werden: Ich möchte Ärztin werden, not eine Ärztin."),
    ("ger:3.1.2a", &[
        "Describe sport, music, film and eating out with frequency, place and company",
        "Express preferences with gern, lieber and am liebsten",
        "Narrate a recent outing in the perfect tense with the correct auxiliary",
        "Order food and ask questions in a restaurant or ticket role-play",
    ], "Movement verbs take sein in the perfect: ich bin geschwommen, gewandert and ausgegangen, never ich habe geschwommen."),
    ("ger:3.1.2b", &[
        "Describe German-speaking festivals: Weihnachten, Silvester, Ostern, Karneval and the Oktoberfest",
        "Explain customs with man and give dates with ordinal numbers",
        "Describe how you celebrated a birthday or festival in the past",
        "Compare celebrations in German-speaking countries and at home",
    ], "Schenken puts the person in the dative: Ich schenke meiner Mutter ein Buch, not meine Mutter."),
    ("ger:3.1.2c", &[
        "Describe a role model and explain why you admire them",
        "Define people with subject relative clauses using der, die and das",
        "Weigh up the advantages and disadvantages of fame and social media stars",
        "Imagine being famous with wenn ich ... wäre, würde ich",
    ], "In a relative clause the pronoun matches the noun and the verb goes to the end: ein Sänger, der viel für andere macht."),
    ("ger:3.1.3a", &[
        "Describe holidays: destinations, transport, accommodation and activities",
        "Use nach, in die, an and in correctly for destinations and locations",
        "Narrate a past holiday in the perfect, and in the simple past for Higher written stories",
        "Plan a future trip and describe a dream holiday with the conditional",
    ], "Travel verbs take sein in the perfect: wir sind nach Wien gefahren and geflogen, never wir haben gefahren."),
    ("ger:3.1.3b", &[
        "Describe how you use phones, apps, social media, TV and streaming",
        "Give advantages and disadvantages of technology with justified opinions",
        "Use separable verbs such as herunterladen, hochladen and fernsehen in all tenses",
        "Use um ... zu, ohne ... zu and statt ... zu to explain purpose and alternatives",
    ], "Separable participles put -ge- between prefix and stem: heruntergeladen, hochgeladen and ferngesehen, not geherunterladen."),
    ("ger:3.1.3c", &[
        "Describe where you live and compare life in a town and in the countryside",
        "Explain environmental problems, their causes and possible solutions",
        "Give advice with man sollte and man muss, and purpose with damit and um ... zu",
        "Describe what you have done and will do to protect the environment",
    ], "Es gibt takes the accusative, so it is es gibt einen Park and keinen Bahnhof, and comparisons use als: ruhiger als, never ruhiger wie."),

    // ---------- History (Pearson Edexcel GCSE History (1HI0)) ----------
    ("hist:11.1a", &[
        "Explain the supernatural and religious explanations of disease, c1250–c1500: God's punishment, a test of faith and astrology",
        "Explain the rational explanations: Hippocrates' Four Humours, Galen's Theory of Opposites and miasma",
        "Explain why the Church and the authority of Galen kept these ideas in place for so long",
        "Show how a single physician could combine religious, astrological and rational ideas in one diagnosis",
        "Answer a 12-mark 'Explain why' question with a point of your own beyond the two stimulus bullets",
    ], "Answers that only use the two stimulus points are capped at 8 out of 12. Always add a reason of your own, and say how the Church acted rather than just naming it."),

    ("hist:11.1b", &[
        "Link each medieval prevention and treatment to the idea behind it: prayer to God, bloodletting and purging to the Humours, herbs and fires to miasma",
        "Describe who treated the sick — physicians, apothecaries, barber surgeons and women at home — and how they differed",
        "Explain what medieval hospitals did, and why most offered care and prayer rather than cure",
        "Describe the Black Death of 1348–49: how people explained it, tried to prevent it and treated it",
        "Explain one similarity or difference between medieval treatment and a later period, with precise evidence for both",
    ], "A list of treatments without the idea behind them stays in Level 2. Say 'because they believed in miasma, they…' every time."),

    ("hist:11.2a", &[
        "Explain what changed and what stayed the same in ideas about the cause of disease, c1500–c1700",
        "Explain Vesalius's contribution, with the 1543 De Fabrica and a specific error of Galen's he corrected",
        "Explain Sydenham's approach: observing symptoms and treating diseases as separate things that could be classified",
        "Explain how the printing press and the Royal Society spread new ideas, and why the Church could no longer stop them",
        "Link factors — individuals, technology and a weaker Church — in a 12- or 16-mark answer",
    ], "Vesalius and Sydenham changed knowledge, not treatment. Saying they changed how patients were treated loses the mark; keep knowledge and practice apart."),

    ("hist:11.2b", &[
        "Explain the continuity and change in prevention and treatment, c1500–c1700, including new remedies from overseas",
        "Describe how care changed after the Dissolution of the Monasteries, and the role of re-founded hospitals and care at home",
        "Explain what Harvey discovered about the circulation of the blood, what Galen had said, and why the discovery had little impact on treatment",
        "Describe how people and the authorities responded to the Great Plague of 1665, with Plague Orders detail",
        "Compare the Black Death (1348) with the Great Plague (1665) for one similarity and one difference",
    ], "Harvey changed knowledge, not treatment: bloodletting carried on. Saying he cured people, or that 1665 was handled better because people understood plague, loses the mark."),

    ("hist:11.3a", &[
        "Explain how Pasteur's swan-necked flask experiments and 1861 germ theory overturned spontaneous generation",
        "Describe Koch's methods and the bacteria he identified (anthrax 1876, TB 1882, cholera 1883)",
        "Explain why germ theory was slow to change British medicine, and where it did have influence",
        "Describe Jenner's 1796 experiment, the opposition he faced and why vaccination spread",
        "Trace government action on vaccination from free (1840) to compulsory (1853) to opt-out (1898)",
        "Weigh germ theory against other developments c1700-c1900 using a clear criterion",
    ], "Students mix up Jenner and Pasteur: Jenner vaccinated against smallpox in 1796 without knowing germs existed; Pasteur's germ theory came in 1861 and only then could vaccines be made for other diseases."),

    ("hist:11.3b", &[
        "Explain how Nightingale changed nursing and hospital design, and the limits of her influence",
        "Explain the impact of anaesthetics on surgery, including opposition and the black period",
        "Explain how Lister's antiseptics and later aseptic methods tackled infection, and why they were resisted",
        "Explain why the government moved from the voluntary 1848 Act to the compulsory Public Health Act of 1875",
        "Describe how Snow traced the 1854 Broad Street cholera outbreak and assess the significance of his work",
    ], "Students treat anaesthetics as making surgery safe straight away, but they led to the black period of rising infection deaths until antiseptic and aseptic methods caught up."),

    ("hist:11.4a", &[
        "Explain how understanding of the causes of illness changed after 1900: genetics and lifestyle factors",
        "Name the improvements in diagnosis — blood tests, CT, MRI, ultrasound, ECG — and explain why they mattered",
        "Explain how the NHS (1948) changed access to care, and its limits",
        "Tell magic bullets (Salvarsan, Prontosil) apart from antibiotics",
        "Explain the roles of Fleming, Florey, Chain and government in developing penicillin into a mass-produced drug",
    ], "Crediting Fleming alone for penicillin caps you at Level 2. He spotted its effect in 1928; Florey and Chain made it a drug, and government money mass-produced it."),

    ("hist:11.4b", &[
        "Describe the high-tech treatments of the twentieth century — plastic surgery, transplants, keyhole surgery — with a name and date for each",
        "Explain how mass vaccination and government lifestyle campaigns changed prevention",
        "Describe how science and technology are used to diagnose and treat lung cancer today",
        "Explain what the government has done about smoking and lung cancer, with dated laws",
        "Weigh government against science and attitudes in a 16-mark judgement, using a criterion",
    ], "Vague case study detail earns little. 'Scans and chemo' needs to become CT, bronchoscopy, biopsy, targeted therapy, and the smoking ban needs its date (2007)."),

    ("hist:11.5a", &[
        "Locate the Ypres salient, the Somme, Arras and Cambrai and give a key fact about each for medical treatment",
        "Describe the organisation of the trench system and why it made moving the wounded difficult",
        "Explain how the terrain, roads and communications affected the treatment of casualties",
        "Describe the illnesses caused by trench life: trench foot, trench fever and shell shock",
        "Describe the wounds caused by bullets, shells and shrapnel, and why infection and head injuries were such problems",
        "Compare the effects of chlorine, phosgene and mustard gas",
    ], "In 'Describe two features' questions the second mark for each feature needs a precise supporting detail; naming a feature with a vague follow-up sentence loses half the marks."),

    ("hist:11.5b", &[
        "Describe the work of the RAMC and of nurses (QAIMNS, VADs, FANY) on the Western Front",
        "Put the stages of the chain of evacuation in order and give one precise fact about what happened at each",
        "Explain how stretcher bearers, horse and motor ambulances, trains and barges moved the wounded, and why the terrain made it hard",
        "Describe the underground hospital at Arras and what it shows about adapting to the front",
        "Explain how wound excision, the Thomas splint, mobile X-rays and the Cambrai blood depot changed treatment",
        "Link each new technique to the pre-war context: germ theory and aseptic surgery, X-rays, blood groups and storage",
    ], "A field ambulance was an RAMC unit that ran the dressing stations, not a vehicle. Answers that say it drove men to hospital lose the detail mark."),

    ("hist:11.5c", &[
        "Name national sources (army records, newspapers, government reports, medical articles) and local sources (personal accounts, photographs, hospital records, army statistics) for the Western Front",
        "Weigh the strengths and weaknesses of each type of source for a specific enquiry, including the effect of censorship",
        "Judge how useful two sources are for an enquiry using content, provenance and your own knowledge",
        "Frame a focused question that follows up a detail in a source",
        "Select a realistic type of source to answer that question and explain how it would help",
    ], "Usefulness is always for the enquiry in the question. Calling a source 'biased' or 'only one person's view' without saying how that affects this enquiry keeps the answer in the bottom levels."),

    ("hist:B4.1a", &[
        "Describe Elizabethan society and government in 1558: the hierarchy, the court, Privy Council, Parliament and JPs",
        "Explain why Elizabeth's legitimacy was questioned, and keep that separate from the problems of her gender",
        "Explain the marriage question and the strengths of character Elizabeth brought to the throne",
        "Explain the challenges she faced at home and abroad in 1558: debt, religion, France and Scotland",
        "Reach a 16-mark judgement on which challenge was most serious, backed by a criterion",
    ], "Legitimacy is about Henry VIII's marriages and the Pope, not about Elizabeth being a woman. Mixing legitimacy and gender up costs the mark."),

    ("hist:B4.1b", &[
        "Explain why a religious settlement was needed in 1559 and what Elizabeth wanted from it",
        "Describe the features of the Act of Supremacy and the Act of Uniformity, and keep the two apart",
        "Explain why the settlement was called a 'middle way'",
        "Explain the impact of the settlement, with evidence: bishops removed, most parish clergy staying, church papists in the north",
        "Describe the role of the Church of England in society: the parish, Church courts, homilies and tithes",
    ], "Elizabeth was Supreme Governor, not Supreme Head. And an 'impact' question wants evidence of what happened, not a list of the settlement's features."),

    ("hist:B4.1c", &[
        "Explain the nature and extent of the Puritan challenge, including the crucifix and vestments controversies",
        "Explain the nature and extent of the Catholic challenge, and the roles of the nobility, the Papacy and foreign powers",
        "Explain Mary, Queen of Scots' claim to the English throne through Margaret Tudor",
        "Describe Mary's downfall in Scotland and her arrival in England in 1568",
        "Explain why every option Elizabeth had for dealing with Mary in 1568–69 was dangerous",
    ], "Answer both 'nature' (what kind of challenge) and 'extent' (how serious). Most answers describe the challenge and never judge how much of a threat it was."),

    ("hist:B4.2a", &[
        "Explain why the Northern Earls rebelled in 1569, giving political as well as religious reasons",
        "Assess the significance of the revolt and of the excommunication that followed it",
        "Describe the features of the Ridolfi, Throckmorton and Babington plots and explain what each one changed",
        "Explain how Walsingham used spies, double agents and code-breakers to uncover plots",
        "Explain why Mary, Queen of Scots was executed in 1587 and not earlier, and what her death changed",
    ], "The three plots get mixed up: Ridolfi 1571 was a Spanish invasion under Alba, Throckmorton 1583 a French invasion under Guise, and Babington 1586 an assassination plan exposed by Mary's own letter. Swapping their details loses the supporting-detail marks."),

    ("hist:B4.2b", &[
        "Explain the political, religious and commercial rivalry between England and Spain, and keep the three apart",
        "Describe privateering and Drake's voyages, and explain why Elizabeth backed them while denying responsibility",
        "Explain the 1584–85 trigger for war in the Netherlands and the Treaty of Nonsuch",
        "Explain Leicester's campaign in the Netherlands and why it went badly",
        "Explain the consequences of Drake's raid on Cadiz in 1587",
    ], "Cadiz answers tell the story and forget the consequence. Say what it did: it delayed the Armada by a year and destroyed the seasoned barrel staves."),

    ("hist:B4.2c", &[
        "Explain why Philip II launched the Armada in 1588",
        "Describe the Spanish invasion plan and why it depended on meeting Parma's army",
        "Describe the key events of the campaign, from the Channel to the fireships at Calais, Gravelines and the voyage home",
        "Explain the reasons for the English victory, including ships, tactics, leadership, Spanish mistakes and the weather",
        "Judge which reason mattered most, separating what stopped the invasion from what destroyed the fleet",
    ], "Most answers blur what stopped the invasion (no link with Parma, fireships, Gravelines) with what destroyed the fleet (storms on the way home). Keep them apart."),

    ("hist:B4.3a", &[
        "Describe education at home and in schools, and say which social groups got which kind",
        "Describe Elizabethan sport, pastimes and the theatre, and explain why the theatre was both popular and opposed",
        "Explain why poverty and vagabondage increased, separating long-term from short-term causes",
        "Explain how attitudes to the poor changed, and what stayed the same",
        "Describe the poor laws of 1563, 1572 and 1576, and what each one added",
    ], "Say which social group you mean. 'Elizabethans went to grammar school' is wrong: most did not, and answers that ignore rank and gender lose the mark."),

    ("hist:B4.3b", &[
        "Explain the factors that prompted exploration, separating trade and profit from new technology",
        "Describe the ships and navigation instruments that made long voyages possible, and say what each one did",
        "Explain why Drake sailed round the world in 1577–80 and weigh up the significance of the voyage",
        "Explain why Raleigh and his investors tried to colonise Virginia, and judge how significant Raleigh was",
        "Explain why the first settlement at Roanoke (1585–86) failed, showing how the causes were linked",
    ], "Mixing up the two Roanoke colonies: the first settlement (1585–86) was Lane's soldiers, who went home with Drake; the 'Lost Colony' was John White's families in 1587. Also, Raleigh never went to Virginia himself."),

    ("hist:P4.1a", &[
        "Explain why the Grand Alliance formed in 1941 and what was agreed at Tehran, Yalta and Potsdam",
        "Explain how ideology and the different aims of Stalin, Truman and Churchill turned wartime allies into rivals",
        "Explain the importance of the atomic bomb and the Long and Novikov telegrams for US–Soviet relations",
        "Describe how the USSR set up satellite states in Eastern Europe, 1945–48",
        "Write a narrative account that links events with 'this led to…' and goes beyond the two bullet points",
    ], "Keep the conferences apart. Tehran agreed the second front, Yalta the four zones and free elections, and Potsdam brought Truman, Attlee and the bomb. Mixing them up loses the marks for precise knowledge."),

    ("hist:P4.1b", &[
        "Explain the difference between the Truman Doctrine (the policy of containment) and the Marshall Plan (the money)",
        "Explain how the USSR responded with Cominform (1947) and Comecon (1949)",
        "Explain the causes, events and results of the Berlin Blockade and Airlift, 1948–49",
        "Explain why NATO was formed in 1949 and how Germany was divided into the FRG and the GDR",
        "Explain two consequences of an event, each with a specific result and a date or figure",
    ], "The Truman Doctrine and the Marshall Plan are not the same thing. The Doctrine is the policy and the Plan is the money, and Marshall Aid was offered to the East as well. Stalin's refusal is where the marks are."),

    ("hist:P4.1c", &[
        "Describe the arms race of the 1950s, from the H-bomb to the ICBM and Sputnik, and explain its significance",
        "Explain why the Warsaw Pact was set up in 1955",
        "Explain how Khrushchev's Secret Speech and de-Stalinisation encouraged unrest in Eastern Europe",
        "Explain the causes and events of the Hungarian Uprising of 1956 and Khrushchev's response",
        "Explain the international reaction to the invasion of Hungary, including why the West did not intervene",
    ], "Nagy's reforms were not the trigger. Soviet tanks went in because he announced Hungary would leave the Warsaw Pact. Soviet troops crushed the rising on their own; it was not a Warsaw Pact invasion."),

    ("hist:P4.2a", &[
        "Explain the refugee problem and brain drain from East Germany through Berlin, with figures",
        "Explain Khrushchev's 1958 Berlin ultimatum and what happened at the Geneva, Camp David, Paris and Vienna summits",
        "Explain why the Berlin Wall was built in August 1961 and how the USA responded",
        "Explain the impact of the Wall on US–Soviet relations, including how it reduced the risk of war",
        "Explain the importance of Kennedy's visit to West Berlin in June 1963",
    ], "The Wall was built to stop East Germans leaving, not to keep Westerners out. Say 'refugee problem' and 'brain drain' and give the figures, about 2.7 million people between 1949 and 1961."),

    ("hist:P4.2b", &[
        "Explain why the Cuban Revolution damaged relations with the USA, and describe the Bay of Pigs invasion of April 1961",
        "Explain why Khrushchev placed missiles in Cuba",
        "Write a narrative account of the Thirteen Days of October 1962, in the right order",
        "Explain how the crisis ended, including the secret deal over the US missiles in Turkey",
        "Explain the consequences of the crisis: the hotline, the Test Ban, Outer Space and Non-Proliferation treaties",
    ], "Consequence answers that retell the crisis score little. Say what it led to, and remember the 1963 Test Ban Treaty did not ban underground tests or cut the number of weapons."),

    ("hist:P4.2c", &[
        "Explain why there was opposition to Soviet control in Czechoslovakia by 1968",
        "Describe Dubček's reforms, 'socialism with a human face', and the Action Programme",
        "Explain why the USSR was alarmed by the Prague Spring and why it invaded in August 1968",
        "Explain the Brezhnev Doctrine and how Soviet control was re-established under Husák",
        "Explain the international reaction to the invasion, and why the West did not act",
    ], "Do not mix up 1956 and 1968. Czechoslovakia did not try to leave the Warsaw Pact and mostly resisted without violence, and the Brezhnev Doctrine came after the invasion to justify it."),

    ("hist:P4.3a", &[
        "Explain why détente happened in the late 1960s and 1970s",
        "Explain what SALT 1 (1972), the Helsinki Accords (1975) and SALT 2 (1979) agreed, and what each side gained",
        "Explain why the USSR invaded Afghanistan in December 1979",
        "Explain the consequences of the invasion: the Carter Doctrine, the end of détente and the Olympic boycotts",
        "Write a narrative account of détente and its collapse that stays inside the question's dates",
    ], "Détente was managed rivalry, not friendship, and SALT 2 was signed but never ratified. Saying it came into force or reduced weapons loses the mark."),

    ("hist:P4.3b", &[
        "Explain how Reagan's 'Second Cold War' raised tension in 1981–85: the build-up, missiles in Europe and the Reagan Doctrine",
        "Explain the significance of the Strategic Defence Initiative for MAD, the Soviet economy and the Reykjavik summit",
        "Explain Gorbachev's new thinking, and how it and the summits led to the INF Treaty of 1987",
        "Explain how the Soviet grip on Eastern Europe loosened in 1989, and the importance of the fall of the Berlin Wall",
        "Explain why the Warsaw Pact ended and the Soviet Union collapsed in 1991, in the right order",
    ], "Saying Reagan alone ended the Cold War limits your mark. Link his pressure to the weak Soviet economy and to Gorbachev's own choices, and keep 1989–91 in order: Wall, reunification, Warsaw Pact, coup, collapse."),

    ("hist:31.1a", &[
        "Explain why Germany was at breaking point in autumn 1918: defeat, hunger, war debt and political division",
        "Explain how the Kaiser abdicated and the Republic was set up, with the dates of the abdication (9 November), the armistice (11 November) and the move to Weimar",
        "Describe the Weimar Constitution: the President, Chancellor, Reichstag, Reichsrat, proportional representation and Article 48",
        "Weigh the strengths of the Constitution against its weaknesses, and link each weakness to what it later allowed to happen",
        "Answer a Paper 3 inference question by going one step beyond what Source A says",
    ], "Naming a weakness is not enough. Say how it worked, for example proportional representation led to coalitions that kept collapsing, or Article 48 let Hindenburg rule without the Reichstag from 1930."),

    ("hist:31.1b", &[
        "Explain why the Treaty of Versailles and the stab-in-the-back myth made the Republic unpopular, term by term",
        "Describe the Spartacist uprising of January 1919 and explain how the Freikorps crushed it",
        "Explain why the Kapp Putsch of March 1920 happened, why the army would not stop it and how the general strike defeated it",
        "Explain the reasons for the French occupation of the Ruhr in 1923 and its effects, including passive resistance",
        "Explain the causes of hyperinflation and who lost and who gained from it",
    ], "The army did not crush the Kapp Putsch. It refused to act, and a general strike by workers defeated it. Getting this backwards loses the mark and the whole point about how weak the Republic was."),

    ("hist:31.1c", &[
        "Explain how Stresemann ended hyperinflation: calling off passive resistance, the Rentenmark and then the Reichsmark",
        "Compare the Dawes Plan (1924) with the Young Plan (1929): what each changed about reparations and loans",
        "Explain how American loans drove recovery, and why that made the recovery fragile",
        "Explain how the Locarno Pact, League membership and the Kellogg–Briand Pact won Germany international acceptance",
        "Judge how far 1924–29 really were Golden Years, using the limits as well as the successes",
    ], "Treating the Golden Years as a complete recovery loses marks. Always add that it rested on short-term American loans, and that farmers and the unemployed never shared in it."),

    ("hist:31.1d", &[
        "Describe how the standard of living changed in 1924–29, with figures for wages, hours, housing and welfare",
        "Explain which groups gained from the Golden Years and which did not",
        "Explain how the position of women changed in politics, work and leisure, and how far the 'New Woman' was typical",
        "Describe developments in architecture, art and the cinema, naming the Bauhaus, Dix, Grosz and key films",
        "Explain why Weimar culture divided Germans and how the Nazis later exploited the resentment",
    ], "Writing as if every German's life improved. Say which group you mean: industrial workers and young urban women gained most, while farmers, the middle classes and most rural women saw little change."),

    ("hist:31.2a", &[
        "Describe Hitler's early career and how he joined the German Workers' Party and set up the Nazi Party in 1919–20",
        "Describe the Twenty-Five Point Programme and explain the role of the SA",
        "Explain the reasons for, events and consequences of the Munich Putsch, and the main ideas of Mein Kampf",
        "Explain why support for the Nazis was limited in 1924–28",
        "Explain how Hitler reorganised the party, including the Bamberg Conference of 1926 and the Führerprinzip",
    ], "Calling the Munich Putsch simply a failure. It failed as a seizure of power, but the trial made Hitler nationally famous and the defeat pushed him towards the legal route to power; the best answers weigh both."),

    ("hist:31.2b", &[
        "Explain why unemployment in Germany rose to over 6 million after the Wall Street Crash, and describe its impact on workers, the middle classes and farmers",
        "Explain why support for the Communist Party grew and how that growth helped the Nazis",
        "Explain why Nazi support rose from 2.6% in 1928 to 37.3% in July 1932, using the appeal of Hitler, propaganda and the SA",
        "Describe the results of the presidential and Reichstag elections of 1932",
        "Explain why Hitler became Chancellor in January 1933, including the roles of Hindenburg and von Papen",
    ], "Hitler was appointed Chancellor, not elected. The Nazi vote fell in November 1932, and it was the Papen deal with Hindenburg that put him in office. Answers that skip this lose the top level."),

    ("hist:31.3a", &[
        "Explain how Hitler used the Reichstag Fire of February 1933, including the emergency decree and the March election",
        "Explain how the Enabling Act was passed and why it gave Hitler the power to make laws without the Reichstag",
        "Describe how trade unions and other political parties were removed in 1933 and local government brought under Nazi control",
        "Explain why Röhm and the SA threatened Hitler, and the causes and consequences of the Night of the Long Knives",
        "Explain how Hindenburg's death and the army's oath made Hitler Führer in August 1934",
        "Explain why Hitler was able to build a dictatorship so quickly, linking legality, terror and the weakness of his opponents",
    ], "The Reichstag Fire Decree (28 February 1933) and the Enabling Act (23 March 1933) are different measures. The first suspended civil rights; the second let Hitler make laws. Mixing them up costs the mark."),

    ("hist:31.3b", &[
        "Describe the roles of the SS, the Gestapo and concentration camps in the Nazi police state",
        "Explain how the Nazis controlled the legal system, including judges, Special Courts and the People's Court",
        "Explain how Goebbels used censorship, radio, film, rallies and the 1936 Berlin Olympics to influence attitudes",
        "Describe Nazi control of art, architecture, literature and film through the Reich Chamber of Culture",
        "Explain how the Nazis tried to control the Catholic and Protestant Churches through the Concordat and the Reich Church, and why they only partly succeeded",
    ], "The Gestapo was small and relied on ordinary Germans denouncing each other. Describing it as a vast all-seeing force, or mixing it up with the SS, misses the point examiners reward."),

    ("hist:31.3c", &[
        "Explain the extent of support for the Nazi regime, using plebiscites, economic recovery, foreign policy success and the Hitler myth",
        "Explain why opposition to the Nazis was so weak in the years 1933–39",
        "Describe Church opposition, including Niemöller, the Pastors' Emergency League and the Confessing Church, and explain its limits",
        "Describe the Swing Youth and the Edelweiss Pirates and explain why some young people rejected the Hitler Youth",
        "Tell apart support, conformity, grumbling, non-conformity, opposition and resistance, and use the right word in an answer",
    ], "Calling every kind of disagreement \"resistance\". The Swing Youth were non-conformists and Niemöller was defending the Church's independence; answers that say how limited opposition was reach the top level."),

    ("hist:31.4a", &[
        "Explain Nazi views on women and the family, including 'Kinder, Küche, Kirche' and the concern about the falling birth rate",
        "Describe Nazi policies on marriage and the family: marriage loans, the Mother's Cross, Lebensborn and the 1938 divorce law",
        "Explain how Nazi policy on women's employment and appearance worked, and why it changed after 1936–37",
        "Describe the Hitler Youth and the League of German Maidens, and explain what the Nazis wanted from the young",
        "Explain how the Nazis controlled education through teachers, the curriculum and elite schools",
        "Judge how successful Nazi policies towards women and the young were by 1939",
    ], "Saying women were banned from work. Women were pushed out of the professions, but labour shortages from 1937 reversed policy and women's employment rose to about 7.14 million by 1939."),

    ("hist:31.4b", &[
        "Explain how the Nazis reduced unemployment through the National Labour Service, the autobahns and other public works",
        "Explain why rearmament, conscription and the Four-Year Plan were the biggest causes of the fall in unemployment",
        "Explain what 'invisible unemployment' was and which groups were left out of the official figures",
        "Describe the German Labour Front, Strength Through Joy and Beauty of Labour, and explain their purpose",
        "Judge whether German workers' standard of living improved, using hours, wages, prices and rights",
    ], "Saying workers were better off because wages rose. Weekly wages rose mainly through longer hours (about 43 to 49 a week) while food prices rose too, so real wages barely improved — and workers lost their unions."),

    ("hist:31.4c", &[
        "Explain Nazi racial beliefs, including the Aryan master race, Untermenschen, eugenics and anti-Semitism",
        "Describe how the Nazis treated Slavs, Roma and Sinti, homosexuals and people with disabilities, naming a law or measure for each",
        "Describe the boycott of Jewish shops and businesses in April 1933 and the two Nuremberg Laws of 1935",
        "Explain the causes, events and consequences of Kristallnacht in November 1938",
        "Explain why the persecution of Jews and other minorities increased between 1933 and 1939",
    ], "Getting the sequence wrong. The boycott was 1 April 1933, the Nuremberg Laws 15 September 1935 and Kristallnacht 9–10 November 1938 — and one Nuremberg Law removed citizenship while the other banned marriage between Jews and Germans."),

    ("fre:3.2.1a", &[
        "Use le, la, l', les, un, une and des to match the gender and number of the noun",
        "Form feminine person nouns and plural nouns by the regular patterns",
        "Choose between the definite article for likes and general statements and the partitive for amounts",
        "Change articles to de after negatives and expressions of quantity",
        "Use the infinitive as a noun where English uses -ing",
    ], "After a negative the article becomes de: je n'ai pas de frère, not je n'ai pas un frère."),
    ("fre:3.2.1b", &[
        "Use ce, cet, cette, ces and the possessives so they agree with the noun that follows",
        "Ask questions with quel, quelle, quels, quelles and use chaque, plusieurs, même, autre, tout and quelque",
        "Place singular direct and indirect object pronouns and reflexive pronouns in front of the verb",
        "Use moi and toi after prepositions and join sentences with qui",
    ], "Son, sa and ses agree with the thing owned, not the owner: sa mère can mean his mother."),
    ("fre:3.2.1c", &[
        "Conjugate regular -er verbs and the seven anchor-verb patterns in all persons of the present",
        "Use aller, avoir, être and faire accurately, including avoir faim, froid and ans",
        "Make sentences negative with ne ... pas, jamais, rien and personne",
        "Ask questions using intonation, est-ce que and inversion",
    ], "There is no -ing form in French: je joue means I am playing, and je suis jouer scores nothing."),
    ("fre:3.2.1d", &[
        "Form the perfect tense with avoir or être and the correct past participle",
        "Recognise the être verbs and reflexive verbs, and make the participle agree",
        "Place negatives and object pronouns correctly around the auxiliary",
        "Write a past-tense paragraph that mixes verbs from different clusters",
    ], "Verbs of movement take être and agree: elle est allée, not elle a allé."),
    ("fre:3.2.1e", &[
        "Talk about the future with aller + infinitive",
        "Form the imperfect in the singular and use it for habits and descriptions",
        "Choose between the perfect for events and the imperfect for background",
        "Give instructions and advice with the tu and vous imperative",
    ], "Habits and descriptions in the past need the imperfect: quand j'étais petit, je jouais, not j'ai joué."),
    ("fre:3.2.1f", &[
        "Use devoir, pouvoir, savoir and vouloir in all persons followed by an infinitive",
        "Conjugate reflexive verbs in all persons, including the reciprocal each other meaning",
        "Use il y a, il y avait, il y aura, il fait, il faut and il est for time and weather",
        "Use the perfect of modals such as j'ai dû and j'ai pu at Higher",
    ], "A modal is followed straight by an infinitive with no à or de: je dois travailler."),
    ("fre:3.2.1g", &[
        "Make adjectives agree using the regular feminine and plural patterns",
        "Place adjectives after the noun, except the listed set that go before it",
        "Compare with plus, moins and aussi ... que, meilleur, pire and mieux",
        "Place adverbs of time, manner, frequency and place correctly, including in the perfect",
    ], "After être the adjective agrees with the subject: mes sœurs sont sportives."),
    ("fre:3.2.1h", &[
        "Contract à and de with le and les to au, aux, du and des",
        "Use en, au and à correctly with countries and towns",
        "Show possession with de and add purpose with pour and sans + infinitive",
        "Recognise verbs and adjectives that take à or de",
        "Work out unfamiliar words in reading from -ième, in-, -able, -ation, -ment and Higher -eur patterns",
    ], "Feminine countries take en and masculine ones au: en France, au Canada, never à France."),
    ("fre:3.2.2a", &[
        "Replace places with y and quantities or de + noun with en",
        "Use the plural object pronouns nous, vous, les and leur in front of the verb",
        "Use emphatic pronouns such as lui, eux and elles after prepositions",
        "Join sentences with que and où as well as qui",
    ], "The pronoun leur (to them) never takes -s: je leur parle."),
    ("fre:3.2.2b", &[
        "Form the future of regular -er verbs and use aurai, ferai, irai and serai",
        "Form the conditional of -er verbs and use aurais, ferais, irais, serais and voudrais",
        "Form the imperfect in the plural for -er verbs, the anchor verbs, avoir, être and faire",
        "Show three time frames in one answer using a range of future forms",
    ], "Will is -rai and would is -rais: je jouerai means I will play, je jouerais means I would play."),
    ("fre:3.2.2c", &[
        "Use depuis with the present tense for actions that are still going on",
        "Say what has just happened with venir de and what is happening with être en train de",
        "Form present participles and use en + -ant for while or by doing",
        "Link actions with avant de + infinitive and après avoir + past participle",
    ], "Depuis takes the present tense for something still going on: j'habite ici depuis deux ans."),
    ("fre:3.2.2d", &[
        "Use ne ... plus, ne ... ni ... ni, ne ... pas encore, ne ... que and ne ... aucun",
        "Use personne ne and rien ne as the subject of a verb",
        "Form the present passive with être, an agreeing participle and par",
        "Give advice with il est ... de, il manque, il vaut mieux and il vaut la peine de",
        "Use superlative adjectives and adverbs, including le meilleur and le mieux",
    ], "Ne ... que means only, not not: je n'ai que dix euros means I only have ten euros."),

    ("fre:3.1.1a", &[
        "Describe yourself, your family and friends: appearance, personality, relationships",
        "Explain how you get on with people using s'entendre avec and se disputer avec, with reasons",
        "Talk about family life in the past, present and future, including marriage and partnership",
        "Use depuis with the present tense and qui to describe people",
    ], "Adjectives agree with the person described, not with the speaker: ma mère est petite, mes frères sont petits."),
    ("fre:3.1.1b", &[
        "Describe your diet, exercise and sleep habits with frequency expressions",
        "Use partitive articles and de after quantities and negatives accurately",
        "Contrast old and new habits with the imperfect and the present",
        "Give health advice with il faut, il vaut mieux and il est important de",
    ], "After a negative or a quantity the article becomes de: je ne mange pas de viande, beaucoup de légumes."),
    ("fre:3.1.1c", &[
        "Describe your school, subjects and rules, giving justified opinions",
        "Talk about work experience and part-time jobs in the perfect tense",
        "Explain your plans after GCSEs with the future, the conditional and si + present",
        "Name jobs and their feminine forms, with no article after être",
    ], "Jobs after être take no article: je voudrais être médecin, not un médecin."),
    ("fre:3.1.2a", &[
        "Describe sport, music, cinema and eating out with frequency and opinions",
        "Use jouer à, jouer de and faire de with the correct contracted articles",
        "Narrate an outing with the perfect for events and the imperfect for description",
        "Handle a restaurant or ticket role-play, including asking a question",
    ], "Games take jouer à and instruments jouer de, and activities take faire de: je joue au foot, je fais de la natation."),
    ("fre:3.1.2b", &[
        "Recognise the listed festivals: la Fête Nationale, Noël, Pâques, l'Aïd and la Saint Valentin",
        "Describe customs with on and give opinions about traditions",
        "Narrate a celebration in the perfect and childhood traditions in the imperfect",
        "Use offrir with an indirect object pronoun and the present passive with par",
    ], "Festivals and dates take their own prepositions: à Noël, à Pâques, pour l'Aïd, le 14 juillet with no preposition."),
    ("fre:3.1.2c", &[
        "Describe a celebrity or role model and justify why you respect them",
        "Weigh up the advantages and drawbacks of fame in a balanced argument",
        "Use direct and indirect object pronouns before the verb, including les, lui and leur",
        "Say whether you would like to be famous using structures on the AQA list",
    ], "Object pronouns go before the verb, and before avoir in the perfect: je la respecte, je lui ai écrit."),
    ("fre:3.1.3a", &[
        "Describe past holidays, transport and accommodation with the perfect and imperfect",
        "Use en, au and à correctly with countries, regions and towns",
        "Recognise the listed francophone places, from La Réunion to le Québec",
        "Plan a future trip and handle a hotel or station role-play",
    ], "Place prepositions depend on gender and type: en France, au Maroc, à Paris."),
    ("fre:3.1.3b", &[
        "Explain how you use phones, apps, social media, TV and streaming",
        "Give balanced advantages and dangers of technology with examples",
        "Use venir de, depuis and en + present participle with technology verbs",
        "Contrast past and present screen habits and plan changes for the future",
    ], "Les réseaux sociaux is plural, so the verb and adjective are too: les réseaux sociaux sont dangereux."),
    ("fre:3.1.3c", &[
        "Describe where you live, its advantages and its problems",
        "Explain environmental problems and their causes: pollution, traffic, waste, climate",
        "Say what you have done and will do for the environment",
        "Argue for solutions with il faut, il vaut mieux and the nous imperative",
    ], "Quantities and negatives take de: trop de voitures, il n'y a pas de parc."),
    // ---------- Music (Eduqas C660QS): ME Musical elements ----------
    ("music:MEa", &[
        "Describe a melody's shape, range and movement using conjunct, disjunct, scalic, arpeggio and sequence",
        "Name intervals from a semitone to an octave, and recognise the sound each one makes",
        "Identify major, minor, pentatonic and chromatic melodic writing, blue notes and microtones",
        "Recognise ornaments such as the trill and appoggiatura, and describe phrasing as regular or irregular",
    ], "Count your features: a 2-mark melody answer needs two separate features such as 'conjunct' and 'narrow range', not one feature said twice."),

    ("music:MEb", &[
        "Tell major from minor tonality, and say where a piece modulates and to which related key",
        "Explain diatonic, chromatic and dissonant harmony, and where each is used for effect",
        "Describe harmonic rhythm as fast or slow, and count chords per bar",
        "Recognise a pedal, a drone and power chords by their sound and their notation",
    ], "'It changes key' earns nothing on its own. Name the new key or its relationship: dominant, relative major, relative minor."),

    ("music:MEc", &[
        "Identify simple time (2/4, 3/4, 4/4), compound time (6/8, 9/8, 12/8) and irregular metres (5/4, 7/8)",
        "Recognise syncopation, dotted rhythms, triplets, swing rhythms and driving rhythms",
        "Use the Italian tempo terms Adagio to Vivace, and accelerando, rallentando and rubato, accurately",
        "Explain how rhythm and tempo shape the character of a piece",
    ], "Tempo is speed and dynamics is volume. Answers that write 'it gets faster' when the music gets louder lose the mark every year."),

    ("music:MEd", &[
        "Identify orchestral, keyboard, popular and Indian instruments and the four voice types",
        "Name performance techniques such as pizzicato, arco, tremolo, double stopping, muted, glissando, vibrato and falsetto",
        "Use the dynamic markings pp to ff, crescendo, diminuendo and sforzando correctly",
        "Describe how a sonority or dynamic creates a particular effect",
    ], "Listing instruments is not describing sonority. Say how they are played (pizzicato, muted, high register) to reach the second mark."),

    // ---------- MC Musical contexts ----------
    ("music:MC", &[
        "Explain how the purpose of a piece (dance, worship, film, concert, commission) shapes its musical features",
        "Explain how occasion, audience and venue affect the way music is written and performed",
        "Place music in its social, historical and cultural context using features you can hear",
    ], "A context answer must link to a musical feature. 'It was for dancing' needs 'so it has a steady, regular pulse and short, balanced phrases'."),

    // ---------- ML Musical language ----------
    ("music:MLa", &[
        "Name any note on the treble or bass stave, including ledger lines and accidentals",
        "Give the value of every note and rest from semibreve to semiquaver, dotted notes and triplets",
        "Read time signatures in simple and compound time, and work out a missing time signature from a bar",
        "Find a named feature (a tie, a rest, an accidental, a dotted rhythm) in printed music by bar number",
    ], "The bass clef is not the treble clef moved down. Read it from its own landmarks (the F line between the dots) or every note comes out a third wrong."),

    ("music:MLb", &[
        "Write and recognise major key signatures up to four sharps and four flats",
        "Find the relative minor of each major key, and tell them apart using the raised seventh",
        "Work out the key of a printed melody from its key signature, accidentals and final note",
    ], "A key signature fits two keys. Look for the raised 7th (for example A sharp in B minor) before writing a major key."),

    ("music:MLc", &[
        "Build the triads I, ii, iii, IV, V and vi in any major key up to four sharps or flats",
        "Translate between Roman numerals and chord symbols (for example C, Dm, Em, F, G7, Am in C major)",
        "Tell primary chords from secondary chords, and recognise a dominant seventh",
        "Name a chord from its notes on a stave, including a chord in inversion or written as a slash chord",
    ], "Lower-case numerals mean minor chords. Writing ii as major, or naming the chord D when the notes are D-F-A, costs the mark."),

    ("music:MLd", &[
        "Complete the missing pitches of a short melody when the rhythm is given, using the major scale",
        "Complete a missing rhythm when the pitches are given, checking each bar adds up",
        "Use step, leap, repeated notes and the chord underneath to narrow down each pitch",
    ], "Each correct pitch scores, so never leave a gap. A guess that keeps the right shape (step up, leap down) often earns partial credit."),

    // ---------- AoS1 Musical forms and devices ----------
    ("music:AoS1a", &[
        "Describe the principal features of Baroque, Classical and Romantic music",
        "Identify an era from features you hear: harpsichord and continuo, Alberti bass and balanced phrases, or a large orchestra and chromatic harmony",
        "Explain how orchestras, dynamics and harmony changed between 1650 and 1910",
    ], "Give features, not dates. 'It sounds old' scores nothing; 'harpsichord continuo and terraced dynamics, so Baroque' scores."),

    ("music:AoS1b", &[
        "Describe binary (AB), ternary (ABA), rondo (ABACA), minuet and trio, theme and variations and strophic forms",
        "Identify a form from a description of its sections, repeats and key changes",
        "Explain how composers vary a theme: melody, rhythm, harmony, tonality, texture or instrumentation",
        "Compare ternary with rondo and binary with ternary",
    ], "Minuet and trio is ternary overall (minuet-trio-minuet), but each part is itself in binary. Say both when the question asks for detail."),

    ("music:AoS1c", &[
        "Define and recognise repetition, contrast, sequence, imitation, canon, ostinato and anacrusis",
        "Recognise syncopation, dotted rhythms, conjunct and disjunct movement and ornamentation",
        "Explain melodic and rhythmic motifs and regular phrasing",
        "Explain how a device creates and develops a piece",
    ], "Sequence and imitation are confused constantly. A sequence repeats at a different pitch in the same part; imitation is copied by a different part."),

    ("music:AoS1d", &[
        "Identify perfect, imperfect, plagal and interrupted cadences by their chords",
        "Recognise simple chord progressions, pedal notes, drones, broken chords and Alberti bass",
        "Explain modulation to the dominant and to the relative minor, and how it is heard",
        "Name chords at cadence points on a printed score",
    ], "An imperfect cadence ends ON chord V; a perfect cadence goes FROM V to I. Check which chord is last before choosing."),

    ("music:AoS1e", &[
        "Describe the Badinerie's context, instrumentation (flute, strings, harpsichord continuo), key (B minor) and binary form",
        "Explain the key scheme: B minor to F sharp minor in Section A, and back to B minor through E minor and D major in Section B",
        "Identify motifs X and Y, sequences, imitation between flute and cello, trills and appoggiaturas",
        "Explain the features that make the movement typically Baroque",
    ], "The whole movement is in 2/4 and binary form with both halves repeated (AABB). Calling it ternary or rondo throws away easy marks."),

    // ---------- AoS2 Music for ensemble ----------
    ("music:AoS2a", &[
        "Define and identify monophonic, homophonic, polyphonic, unison, chordal and layered textures",
        "Recognise melody and accompaniment, round, canon, countermelody and descant",
        "Describe how texture changes during a piece and why",
    ], "Describe texture with a term and a change: 'monophonic at first, then homophonic when the strings enter' beats a single word."),

    ("music:AoS2b", &[
        "Describe the string quartet and the roles of its four instruments",
        "Explain basso continuo and which instruments played it",
        "Explain what a sonata is in the Baroque and Classical periods",
        "Describe how texture is used in chamber music",
    ], "Basso continuo is two jobs: a bass instrument playing the line and a keyboard or lute filling in the chords. Name both."),

    ("music:AoS2c", &[
        "Describe solos, duets, trios, ensembles and chorus numbers in musicals",
        "Explain the role of backing vocals and how vocal lines are combined",
        "Recognise features of musical theatre songs: belt, recitative-like singing, the pit orchestra, word painting",
        "Explain how music tells a story and shows character in a musical",
    ], "Say how the voices are combined (in unison, in harmony, in call and response, overlapping) rather than just how many sing."),

    ("music:AoS2d", &[
        "Describe the jazz/blues trio and the rhythm section (drums, bass, piano or guitar)",
        "Write out the chord pattern of the 12-bar blues and explain blue notes",
        "Recognise swing rhythm, improvisation, scat, walking bass, call and response and syncopation",
        "Explain how texture is used in jazz and blues ensembles",
    ], "The 12-bar blues ends I-I (or I-V turnaround) after V-IV. Writing V-V-I-I or missing the IV in bar 10 loses the mark."),

    // ---------- AoS3 Film music ----------
    ("music:AoS3a", &[
        "Explain how composers use the elements to create mood and respond to a commission or stimulus",
        "Describe how instrumental and vocal timbres create colour and atmosphere",
        "Explain how performers interpret a score and how audience and venue affect a performance",
        "Write an extended answer linking musical features to the action on screen",
    ], "In the 10-mark answer every feature must be tied to the scene. A list of correct terms with no 'which creates...' stays in the lower bands."),

    ("music:AoS3b", &[
        "Define leitmotif and explain how it represents a character, object or idea",
        "Explain thematic transformation: changing a theme's tonality, tempo, rhythm, instrumentation or dynamics",
        "Describe how a transformed theme changes the meaning for the audience",
    ], "Thematic transformation keeps the theme recognisable. Say what stays the same as well as what changes."),

    ("music:AoS3c", &[
        "Explain how dynamics and contrast create special effects such as shock or suspense",
        "Describe how music technology enhances sonority: synthesisers, reverb, layering, sampling",
        "Explain minimalist techniques used in film: ostinato, phasing, layering, additive melody, gradual change",
    ], "Minimalism is not just 'repetitive'. Name the technique (phasing, additive rhythm, layered ostinati) and the slow change it creates."),

    // ---------- AoS4 Popular music ----------
    ("music:AoS4a", &[
        "Describe strophic, 32-bar song form (AABA), 12-bar blues and verse-chorus structure",
        "Identify intro, verse, pre-chorus, chorus, middle 8, bridge, instrumental break, fill and outro",
        "Recognise riffs, standard chord progressions, primary and secondary chords and cadences in songs",
        "Identify the section a printed or described extract comes from",
    ], "A verse has the same music with new words each time; a chorus repeats both music and words. Use that to label sections, not the order."),

    ("music:AoS4b", &[
        "Describe vocal sounds: lead and backing vocals, syllabic and melismatic, falsetto, belt, rap, a cappella",
        "Explain how instrumental, synthesised, amplified and computer-generated sounds are used",
        "Explain loops, samples, panning, phasing, reverb, balance and backing tracks",
        "Explain how original music may be modified: covers, remixes and arrangements",
    ], "Panning is left-right placement, not volume. Phasing is a sweeping effect from two copies drifting out of time, not an echo."),

    ("music:AoS4c", &[
        "Describe the features of pop and rock and pop",
        "Describe bhangra: dhol, chaal rhythm, tumbi, shouts, fast dance tempo and its fusion with pop",
        "Explain fusion as the combination of two or more styles, and identify the styles in an example",
    ], "The bhangra drum is the dhol, played with two sticks; it is not the tabla. Naming tabla costs the mark."),

    ("music:AoS4d", &[
        "Describe Africa's background: Toto, Toto IV, written by David Paich and Jeff Porcaro, single released 1982",
        "Explain its verse-chorus structure, nine-bar verse phrases, and the key contrast between B major verses and A major choruses",
        "Identify riff a and riff b, the pentatonic writing, syncopation, 2/2 time and the chord patterns of verse and chorus",
        "Explain how the band evokes African music through sonority, ostinato and rhythm",
    ], "The chorus is in A major, not B major. Knowing that the chorus changes key, and that the verse phrases are nine bars long, wins set-work marks."),

    // ---------- Religious Studies (AQA GCSE Religious Studies A (8062)) ----------
    ("rs:3.1.2.1a", &[
        "Explain what Christians mean by God being omnipotent, loving and just, with a source for each",
        "Explain the oneness of God and the Trinity: one God in three Persons",
        "Explain the problem of evil and suffering, telling moral evil from natural evil, and how Christians respond",
        "Explain the roles of the Word and the Spirit in creation, using Genesis 1:1-3 and John 1:1-3",
        "Compare literal and non-literal Christian views of creation, and say who holds them",
    ], "Explaining the Trinity as three gods, or one God playing three roles, loses the mark. Say 'one God in three Persons'."),

    ("rs:3.1.2.1b", &[
        "Explain Christian beliefs about resurrection, including St Paul's idea of a transformed spiritual body",
        "Describe the particular judgement, the Last Judgement and the Parable of the Sheep and the Goats",
        "Compare different Christian beliefs about heaven and hell, including hell as punishment, separation or annihilation",
        "Explain the Catholic belief in purgatory and why Protestants reject it",
        "Explain why beliefs about the afterlife matter to how Christians live and face death",
    ], "Contrasting-belief questions need two genuinely different views, such as hell as eternal punishment against hell as separation from God. Two ways of describing heaven as happy count as one belief."),

    ("rs:3.1.2.1c", &[
        "Explain the incarnation and what Christians mean when they call Jesus the Son of God",
        "Describe the crucifixion, resurrection and ascension and explain why each matters to Christians",
        "Explain sin and original sin, including how Catholic and Orthodox views differ",
        "Compare the roles of law, grace and the Holy Spirit in salvation, including Protestant and Catholic views",
        "Explain atonement and the different ways Christians understand how Jesus' death saves",
    ], "Answers on atonement often stop at \"Jesus died for our sins\". The marks go to explaining how his death restores the relationship with God, for example as a sacrifice, a substitution or a ransom."),

    ("rs:3.1.2.2a", &[
        "Describe liturgical, non-liturgical and informal worship with a named example of each, such as the Mass, a Baptist service and a Quaker meeting",
        "Explain how the Bible is used in Christian worship",
        "Explain the significance of private worship, including the rosary, icons and personal prayer",
        "Compare set prayers with informal prayer and explain why Christians use each",
        "Explain the meaning of each part of the Lord's Prayer and why it matters to Christians",
    ], "\"Significance\" questions want why a form of worship matters to Christians. A description of what happens at Mass or a Quaker meeting only gets the first mark of each point."),

    ("rs:3.1.2.2b", &[
        "Explain what a sacrament is and why Catholics and Orthodox have seven, most Protestants two, and Quakers none",
        "Describe infant baptism and believers' baptism and give the reasons each side uses, with Bible references",
        "Explain the significance of baptism, using Matthew 28:19 and Romans 6:3-4",
        "Describe how Holy Communion is celebrated in contrasting churches",
        "Compare transubstantiation, spiritual presence and the memorial view, naming who holds each",
    ], "Answers blur transubstantiation and the memorial view, or give a 'contrasting' pair that is really the same point. Name the denomination and its reason for each belief."),

    ("rs:3.1.2.2c", &[
        "Explain the role and importance of pilgrimage, including why some Christians think it unnecessary",
        "Describe Lourdes and Iona: their history, what pilgrims do there and why it matters to them",
        "Contrast Lourdes and Iona as two different kinds of pilgrimage",
        "Explain how Christmas and Easter are celebrated and the beliefs behind them",
        "Assess the importance of Christmas and Easter for Christians in Great Britain today",
    ], "Students describe Lourdes and Iona but never state the contrast or say why the visit matters to the pilgrim. Say what the pilgrim does and what it does for their faith."),

    ("rs:3.1.2.2d", &[
        "Explain how churches serve the local community, using food banks and Street Pastors as examples",
        "Explain mission, evangelism and Church growth, and why Christians see them as important",
        "Describe how Christians work for reconciliation, with named examples such as Coventry and Corrymeela",
        "Explain how churches respond to the persecution of Christians",
        "Describe the work of Christian Aid and the Christian teaching behind it",
    ], "Answers describe what a food bank or charity does but never say why Christians do it. Link each action to a teaching such as the Sheep and the Goats (Matthew 25) or the Good Samaritan."),

    ("rs:3.1.5.1a", &[
        "List the six Sunni articles of faith and the five Shi'a roots of Usul ad-Din, and explain their similarities and differences",
        "Explain Tawhid using Surah 112, and why shirk is the greatest sin",
        "Explain what Muslims mean by God's omnipotence, beneficence, mercy, fairness and justice, with a Qur'an reference for each",
        "Explain Adalat and why Shi'a Muslims make it a root of faith",
        "Compare immanence and transcendence and show how Muslims hold both together",
    ], "Students mix up the two lists, or say Shi'a Muslims do not believe in angels. Learn the Sunni six and the Shi'a five as separate tables: Adalat and Imamah are the Shi'a additions."),

    ("rs:3.1.5.1b", &[
        "Describe the nature of angels in Islam, with a Qur'an reference",
        "Explain the roles of Jibril and Mika'il, and tell them apart from Israfil and Izra'il",
        "Explain predestination (al-Qadr) and how Muslims fit it with human freedom and responsibility",
        "Explain the difference between Sunni and Shi'a understandings, including Adalat",
        "Explain Akhirah: the Day of Judgement, resurrection, heaven and hell, and how the belief changes the way Muslims live",
    ], "In a 4-mark influence question, describing the belief earns only half the marks. The second mark in each point needs what Muslims actually do because of it."),

    ("rs:3.1.5.1c", &[
        "Explain what Risalah means and why Muslims believe Muhammad is the Seal of the Prophets",
        "Explain the role and importance of Adam, Ibrahim and Muhammad, with a Qur'an reference for each",
        "Match each holy book to its prophet and explain why the Qur'an has the highest authority",
        "Explain why most Muslims give the Torah, Psalms, Gospel and Scrolls of Abraham less authority than the Qur'an",
        "Explain the Shi'a belief in the imamate and how it differs from the Sunni view of leadership after Muhammad",
    ], "Pairing a book with the wrong prophet — Zabur goes with Dawud and Injil with Isa. The other common slip is calling the Shi'a Imams prophets, when Muhammad is the Seal of the Prophets."),

    ("rs:3.1.5.2a", &[
        "List the Five Pillars of Sunni Islam and the Ten Obligatory Acts of Shi'a Islam, and explain how the two lists differ",
        "Explain the meaning of the Shahadah and where it comes in a Muslim's life, from birth to death",
        "Describe how Muslims prepare for and perform salah: times, qiblah, wudu, rak'ahs and recitations",
        "Explain how salah is practised in the mosque, at home, elsewhere, and at Friday Jummah",
        "Explain the key differences between Sunni and Shi'a salah, and different Muslim views about the importance of prayer",
    ], "Losing the second mark in contrasting questions by giving two points from the same side. A Sunni way against a Shi'a way (e.g. five separate prayer times against combining them into three) is the safest contrast."),

    ("rs:3.1.5.2b", &[
        "Explain the origins and purpose of fasting in Ramadan, using Qur'an 2:183 and 2:185",
        "Describe the duties of sawm and explain its benefits for individuals and the community",
        "Explain who is excused from fasting, why, and what they do instead",
        "Explain what happened on the Night of Power and what Qur'an 96:1-5 teaches",
        "Explain how and why zakah is given and how it benefits those who receive it",
        "Explain Khums in Shi'a Islam and compare it with zakah",
    ], "Swapping the numbers is the most common slip: zakah is 2.5% of wealth above the nisab and Khums is 20% of surplus income. Also give reasons, not just a list, for the exceptions from fasting. Qur'an 2:185, \"Allah intends for you ease\", earns the extra mark."),

    ("rs:3.1.5.2c", &[
        "Explain the origins of Hajj in the story of Ibrahim, Hajar and Isma'il and the Prophet's Farewell Pilgrimage",
        "Describe ihram and explain what the clothing and restrictions mean",
        "Describe in order the actions at the Ka'aba, Mina, Arafat and Muzdalifah, and explain the significance of each",
        "Explain the role and significance of Hajj for Sunni and Shi'a Muslims, with Qur'an 3:97 or 22:27",
        "Evaluate how important Hajj is for Muslims in Britain today, weighing cost, danger and its religious meaning",
    ], "Students describe what pilgrims do but not why. In every 4- and 5-mark point the second mark is for the significance, such as Arafat being a rehearsal for the Day of Judgement."),

    ("rs:3.1.5.2d", &[
        "Explain the meaning of jihad and the difference between greater and lesser jihad",
        "Explain the origins of lesser jihad and list the conditions for declaring it, with Qur'an references",
        "Explain why most Muslims reject the use of jihad to justify terrorism",
        "Describe the origins, meanings and practices of Id-ul-Fitr and Id-ul-Adha",
        "Contrast how Sunni and Shi'a Muslims mark Ashura, and explain why the festivals matter for Muslims in Great Britain today",
    ], "Translating jihad as \"holy war\" and giving vague conditions. Start with \"striving\", split greater and lesser, and name the conditions: legitimate authority, just cause, last resort, no harm to innocents."),

    ("rs:3.2.1.1a", &[
        "Explain Christian, Muslim and non-religious views on heterosexual and homosexual relationships, and on sex before and outside marriage",
        "Explain contrasting beliefs about contraception and family planning, including Humanae Vitae and Muslim teaching",
        "Explain the nature and purpose of marriage in Christianity and Islam, with Mark 10:9 or Qur'an 30:21",
        "Explain different views on same-sex marriage and cohabitation in Britain today",
        "Explain religious and non-religious views on divorce and remarriage, and use the arguments from the sanctity of marriage vows and from compassion",
    ], "On contraception, sex before marriage and homosexual relationships the question demands Christianity plus another religion. A Christianity-versus-humanism answer doesn't meet it, so bring in Islam."),

    ("rs:3.2.1.1b", &[
        "Describe different types of family and explain religious teaching on the roles of parents and children",
        "Explain the purposes of the family: procreation, stability and protection of children, and educating children in a faith",
        "Explain Christian, Muslim and non-religious views on same-sex parents and polygamy",
        "Explain contrasting beliefs about the roles of men and women and about gender equality, including women's leadership in religion",
        "Define gender prejudice and discrimination, give real examples, and explain religious responses to them",
    ], "Students treat all Christians or all Muslims as agreeing on gender. Name the tradition, e.g. the Church of England ordains women but the Catholic Church does not, and balance Qur'an 4:34 with 33:35."),

    ("rs:3.2.1.2a", &[
        "Explain the Big Bang theory and its evidence, and how literal and non-literal Christians and Muslims relate it to creation",
        "Explain stewardship, dominion, responsibility and awe and wonder, with a Christian and a Muslim source for each",
        "Describe how humans use and abuse the environment and how Christians, Muslims and humanists respond",
        "Explain religious and non-religious views on using animals for food, including halal rules and vegetarianism",
        "Explain contrasting beliefs about animal experimentation, comparing Christianity with Islam (a non-religious view can be added, never substituted)",
    ], "Writing that dominion lets humans do what they like with nature loses the development mark. Most believers read dominion as responsible rule, so link it to stewardship (Genesis 2:15) or khalifah (Qur'an 2:30)."),

    ("rs:3.2.1.2b", &[
        "Explain evolution and how literal and non-literal Christians, Muslims and humanists relate it to the origins of human life",
        "Explain the difference between sanctity of life and quality of life, with a Christian and a Muslim source",
        "Explain contrasting beliefs about abortion, including when the mother's life is at risk",
        "Explain contrasting beliefs about euthanasia, telling voluntary from non-voluntary and active from passive",
        "Explain Christian, Muslim and humanist beliefs about death and the afterlife and how they shape the value placed on life",
    ], "Writing that all Christians or all Muslims reject abortion in every case loses marks. The Catholic double effect, the Church of England's limited conditions and Islam's 120-day teaching all show diversity, and nearly every tradition accepts abortion to save the mother's life."),

    ("rs:3.2.1.4a", &[
        "Define peace, justice, forgiveness and reconciliation and explain why they matter to Christians and Muslims, with a source for each",
        "Explain contrasting beliefs about violence, including violent protest, and why religious believers condemn terrorism",
        "Explain the reasons for war — greed, self-defence and retaliation — and religious responses to each",
        "State and apply the criteria of the just war theory, and compare them with the conditions for lesser jihad",
        "Explain holy war and contrasting beliefs about pacifism, from Quakers to the just war majority to Islam",
    ], "Listing the just war criteria without explaining or applying them earns little. Say what each criterion means and test it against a real war, and show that most Christians follow the theory while Quakers are pacifists."),

    ("rs:3.2.1.4b", &[
        "Evaluate whether religion causes war today, using real conflicts and separating religious labels from political causes",
        "Explain nuclear, chemical and biological weapons and the arguments for and against nuclear deterrence",
        "Explain contrasting beliefs about weapons of mass destruction, comparing Christianity with Islam",
        "Describe the peace-making work of individuals influenced by religious teaching, such as Desmond Tutu",
        "Explain how one present-day religious organisation, such as Christian Aid or Islamic Relief, helps victims of war, and the teachings behind it",
    ], "Answers about WMD that do not say whether they mean using or possessing them lose marks. Almost all believers condemn using them; the real disagreement is about keeping them as a deterrent."),

    ("rs:3.2.1.5a", &[
        "Explain the difference between a crime and a sin, and Christian, Muslim and humanist views about obeying the law",
        "Explain religious beliefs about good and evil intentions and actions, including whether it can ever be good to cause suffering",
        "Describe the main reasons for crime: poverty and upbringing, mental illness and addiction, greed and hate, and opposition to an unjust law",
        "Explain different views about people who break the law for each of these reasons, with a source for each side",
        "Explain Christian, Muslim and non-religious views about hate crimes, theft and murder",
    ], "Giving a cause of crime (greed, jealousy, poverty) when the question asks for a type of crime. The mark scheme refuses causes, so answer with hate crime, theft or murder."),

    ("rs:3.2.1.5b", &[
        "Define retribution, deterrence and reformation and explain Christian, Muslim and humanist views on each aim of punishment",
        "Explain religious views about prison, corporal punishment and community service, with arguments for and against each",
        "Explain contrasting Christian and Muslim beliefs about forgiveness, corporal punishment and the death penalty",
        "Describe how the death penalty was abolished in Britain and where it is still used",
        "Use the principle of utility and the sanctity of life to argue both for and against the death penalty",
    ], "Losing the second mark in the contrasting-beliefs questions by giving two views that agree or leaving out Christianity. Set a Christian view (most oppose the death penalty because of the sanctity of life) against a Muslim one (many accept it for murder under 17:33)."),
    // ---------- Drama (AQA 8261) ----------
    ("drama:3.1.1a", &[
        "Name the twelve theatre roles in the specification and say what each does day to day",
        "Explain what each role is accountable for in rehearsal and in performance",
        "Distinguish roles that are easily confused: director and stage manager, technician and designer, stage manager and theatre manager",
        "Explain how an understudy and a stage manager keep a production running when things go wrong",
    ], "The designer plans the lighting; the technician rigs, focuses and operates it. Section A regularly offers the designer as the wrong answer to an operating question."),

    ("drama:3.1.1b", &[
        "Label the nine stage positions from the performer's point of view, upstage to downstage and left to right",
        "Describe the six staging configurations: proscenium arch, end on, thrust, traverse, in the round and promenade",
        "Explain how each configuration affects sightlines, entrances, set and the actor-audience relationship",
        "Choose and justify a configuration for a given scene or effect",
    ], "Stage left is the actor's left as they face the audience, not the audience's left. Reversing it throws away an easy mark."),

    ("drama:3.1.1c", &[
        "Define genre, form, style and structure and identify each in a play you have studied",
        "Explain how a playwright's language and stage directions give the performer and designer clues",
        "Identify the practical demands of a text: locations, scene changes, effects, doubling and special requirements",
        "Use this vocabulary accurately in Section B and C answers",
    ], "Genre and style are not the same word: tragedy is a genre, naturalism is a style. Mixing them up undermines an otherwise good answer."),

    ("drama:3.1.1d", &[
        "Explain sub-text and show how a performer plays what a character means rather than what they say",
        "Analyse a character's motivation and how characters interact within a scene",
        "Explain how mood, atmosphere, pace and rhythm are created and changed across a scene",
        "Locate the dramatic climax of a scene and of the play and explain how it is built",
    ], "Naming a mood is not analysing it. Say which moment, which choice creates it, and what the audience feels as a result."),

    ("drama:3.1.1e", &[
        "Summarise the social, cultural and historical context in which your set play is set",
        "Explain the theatrical conventions of the period in which it was written",
        "Turn context into concrete staging and design choices",
        "Explain why a play's setting and the time it was written can be different, and why both matter",
    ], "Context has to change something on stage. A paragraph of history with no costume, set or acting choice attached earns nothing in Drama."),

    ("drama:3.1.1f", &[
        "Use the vocal terms accurately: accent, volume, pitch, timing, pace, intonation, phrasing, emotional range, delivery of lines",
        "Use the physical terms accurately: build, age, height, facial features, movement, posture, gesture, facial expression",
        "Describe a vocal or physical choice precisely enough that another actor could reproduce it",
        "Explain the effect of each choice on the audience's understanding of the character",
    ], "'Say it angrily' is not a vocal skill. Name the skill (raised volume, clipped pace, a falling pitch) and pin it to a word or moment."),

    ("drama:3.1.1g", &[
        "Explain performance conventions such as direct address, aside, soliloquy, freeze, flashback, multi-role and chorus",
        "Use proxemics, levels and stage positions to show relationships and status",
        "Explain how the actor-audience configuration shapes the relationship between performers and audience",
        "Describe how blocking changes across a scene to show a shift in power or feeling",
    ], "Proxemics answers lose marks for vagueness: say where each actor is, where they move to, and what the change in distance shows."),

    ("drama:3.1.1h", &[
        "Apply the design fundamentals of scale, shape, colour and texture to a set or prop design",
        "Explain set types and features: composite, permanent, minimalist, naturalistic, flats, cyclorama, revolves, trucks, flying, projection, multimedia, pyrotechnics, smoke",
        "Design props that tell the audience about character, period and place",
        "Explain how a set supports scene changes, entrances and the chosen configuration",
    ], "A set design that lists furniture without saying what it communicates stays in the bottom bands. Every item needs a reason."),

    ("drama:3.1.1i", &[
        "Specify a costume precisely: garment, fabric, colour, cut, fit, condition and accessories",
        "Design hair and make-up that show age, status, health and period",
        "Explain how costume communicates character, period, location and mood",
        "Describe puppet types (rod, string, shadow, hand, human-arm) and what a puppet designer decides",
    ], "Colour alone is not a costume design. Examiners want fabric, cut and condition too, each tied to the character or context."),

    ("drama:3.1.1j", &[
        "Use lighting terms accurately: direction, colour, intensity, gels, gobos, profile, Fresnel, flood, follow spot, fades and blackout",
        "Explain how lighting establishes time, place, mood and focus",
        "Use sound terms accurately: direction, amplification, live and recorded sound, music, sound effects, underscoring, soundscape",
        "Explain how sound establishes location and period and builds or releases tension",
    ], "'The lighting was dim' gains little. Name the colour, angle, intensity and the cue change, and say what it made the audience feel."),

    ("drama:3.1.2a", &[
        "Answer the compulsory 4-mark design question for costume or setting in the context given",
        "Give precise design detail rather than general description",
        "Link each design choice to the social, cultural and historical context in the question",
        "Finish a full-mark answer in about five minutes",
    ], "Four marks go on precision. A design that is right for the period but vague about fabric, colour and condition rarely gets beyond 2."),

    ("drama:3.1.2b", &[
        "Plan the vocal and physical delivery of a single line from the extract",
        "Cover both voice and body, attached to specific words in the line",
        "Explain the effect each choice creates for the audience in this moment of the play",
        "Show awareness of the character's situation and motivation at that point",
    ], "Covering only voice or only movement caps the answer. The question asks for both vocal and physical skills."),

    ("drama:3.1.2c", &[
        "Plan blocking for the shaded section: positions, moves, levels and proxemics",
        "Describe interaction with the other actor: eye contact, touch, reactions and listening",
        "Tie every move to a line or moment in the shaded section",
        "Explain how the space shows the relationship or the effect the question names",
    ], "The question is about the performance space and interaction, not voice. Answers that drift into vocal skills lose focus and marks."),

    ("drama:3.1.2d", &[
        "Build a 20-mark performer answer across the extract and the play as a whole",
        "Use an extensive range of vocal and physical skills, each with precise detail",
        "Justify each choice by the character's journey and the playwright's intentions",
        "Refer to at least two other moments in the play to show knowledge of the whole",
    ], "Staying inside the extract caps the answer below the top band. The question explicitly asks about the role in the play as a whole."),

    ("drama:3.1.2e", &[
        "Choose one design skill (lighting, sound, set, costume or puppets) and design it for the extract",
        "Describe the design with precise technical detail and cue changes",
        "Explain how the design supports the action and communicates meaning",
        "Show how the same design approach works for the play as a whole",
    ], "A designer answer must describe effects that support the action. A static description of what the stage looks like misses half the question."),

    ("drama:3.1.2f", &[
        "Summarise the plot of The Crucible act by act and its key moments",
        "Explain the context: Salem 1692, Puritan society, and Miller's 1950s America and McCarthyism",
        "Analyse the main characters: Proctor, Elizabeth, Abigail, Hale, Danforth, Parris, Mary Warren",
        "Plan performance and design ideas for key moments across the four acts",
    ], "The play is set in 1692 but written in 1953. Costume and set questions want the 1690s Puritan world unless the question says otherwise."),

    ("drama:3.1.2g", &[
        "Summarise the plot of Blood Brothers and its key moments, from the pact to the final shooting",
        "Explain the context: Liverpool from the 1950s to the 1980s, class, unemployment and superstition",
        "Analyse the main characters: Mrs Johnstone, Mrs Lyons, Mickey, Edward, Linda, Sammy and the Narrator",
        "Plan performance and design ideas, including playing the twins as children and the role of the Narrator",
    ], "The boys are played by adults at seven, fourteen and as young men. Answers that ignore how an adult performer shows each age lose precision."),

    ("drama:3.1.2h", &[
        "Summarise the plot of Noughts and Crosses and its key moments",
        "Explain the context: a dystopian society that reverses real racial power and the history of segregation it draws on",
        "Analyse the main characters: Sephy, Callum, Jude, Meggie, Ryan, Kamal and Jasmine",
        "Plan performance and design ideas for its fast, episodic, epic structure",
    ], "The play reverses real-world racial power: Crosses are the ruling dark-skinned group. Getting that backwards wrecks any design or context answer."),

    ("drama:3.1.2i", &[
        "Summarise the plot of Around the World in 80 Days and its key moments, from the wager to the missing day",
        "Explain the context: Victorian England, the British Empire and the new age of steam travel",
        "Analyse the main characters: Fogg, Passepartout, Fix and Aouda",
        "Plan performance and design ideas for its storytelling style, multi-role playing and suggested locations",
    ], "This is storytelling theatre with a small cast in many roles. Designs that try to build every location naturalistically miss the style."),

    ("drama:3.1.2j", &[
        "Summarise the plot of Things I Know to Be True and its key moments across the four seasons",
        "Explain the context: a contemporary working-class family in suburban South Australia",
        "Analyse the main characters: Bob, Fran, Pip, Mark/Mia, Ben and Rosie",
        "Plan performance and design ideas, including direct address monologues and physical theatre",
    ], "Each child's monologue is spoken to the audience. Answers that play those speeches as if to another character lose the convention."),

    ("drama:3.1.2k", &[
        "Summarise the plot of Romeo and Juliet and its key moments act by act",
        "Explain the context: Verona in the late sixteenth century and Elizabethan theatre conventions",
        "Analyse the main characters: Romeo, Juliet, the Nurse, Mercutio, Tybalt, Friar Laurence, Capulet and Lady Capulet",
        "Plan performance ideas for verse and prose, and design ideas for key moments",
    ], "Verse is a performance challenge, not just a literary one. Say how the actor handles the rhythm and the line ends, not only what the words mean."),

    ("drama:3.1.2l", &[
        "Summarise the plot of A Taste of Honey and its key moments",
        "Explain the context: working-class Salford in the late 1950s and kitchen sink drama",
        "Analyse the main characters: Jo, Helen, Peter, Boy and Geof",
        "Plan performance and design ideas, including the music-hall touches of the first production",
    ], "It is naturalistic but not only naturalistic: Joan Littlewood's first production added live jazz and direct address, and answers can use both."),

    ("drama:3.1.2m", &[
        "Summarise the plot of The Great Wave and its key moments across more than twenty years",
        "Explain the context: North Korea's abductions of Japanese citizens in the late 1970s and 1980s and the politics that followed",
        "Analyse the main characters: Hanako, Reiko, Etsuko, Tetsuo and Jung Sun",
        "Plan performance and design ideas for a fast-moving political thriller set in two countries",
    ], "The play moves between Japan and North Korea over more than twenty years. Designs must show which country and which decade each scene is in."),

    ("drama:3.1.2n", &[
        "Summarise the plot of The Empress and its key moments, from the voyage to the final scenes",
        "Explain the context: Queen Victoria's last years, the British Empire in India and Indians living in Britain",
        "Analyse the main characters: Rani, Abdul Karim, Queen Victoria, Hari, Firoza and Dadabhai Naoroji",
        "Plan performance and design ideas for an ensemble play with many locations",
    ], "The play is set between 1887 and 1901. Costume and setting answers need late Victorian detail, for both the British and the Indian characters."),

    ("drama:3.1.3a", &[
        "Prepare to see a production: research the play, the company and the production's style",
        "Take useful notes during and straight after the performance on acting, design and key moments",
        "Identify the production's interpretation and what the company was trying to communicate",
        "Build a revision bank of precise moments for acting and for each design area",
    ], "Section C marks precision. Students who wrote no notes after the show end up describing it in general terms and stall in the middle bands."),

    ("drama:3.1.3b", &[
        "Describe how performers used vocal and physical skills with precise, moment-by-moment detail",
        "Analyse how those skills communicated character, relationships, mood or meaning",
        "Evaluate how successful the performance was and justify every judgement",
        "Plan and write a 32-mark response in about 45 minutes",
    ], "Twenty of the 32 marks are for analysis and evaluation. Retelling the plot or describing without judging keeps the answer below half marks."),

    ("drama:3.1.3c", &[
        "Describe set, costume, lighting and sound design with accurate technical vocabulary",
        "Analyse how the design created location, period, mood or meaning at specific moments",
        "Evaluate how successful the design was and justify each judgement",
        "Answer the design option in Section C with a clear line of argument",
    ], "Answer only the design area the question names. A lighting question answered with costume detail earns almost nothing."),
    // ---------- Physical Education (AQA 8582) ----------
    ("pe:3.1.1.1a", &[
        "Identify the bones at the head/neck, shoulder, chest, elbow, hip, knee and ankle, including the patella in front of the knee",
        "Explain the six functions of the skeleton and apply each to a movement or situation in sport",
        "Label the structures of a synovial joint and explain how each one helps to prevent injury",
        "Link hinge and ball and socket joints to the movements they allow: flexion, extension, abduction, adduction, rotation, circumduction, plantar flexion and dorsiflexion",
    ], "Blood cell production and mineral storage are functions too, and a question asking for a sporting example wants the function applied - 'the cranium protects the brain when heading a football', not just 'protection'."),

    ("pe:3.1.1.1b", &[
        "Locate the thirteen named muscles and muscle groups and say which joint each acts on",
        "Explain how agonist and antagonist work as a pair at the shoulder, elbow, hip, knee and ankle",
        "State the role of tendons in attaching muscle to bone",
        "Distinguish isometric from isotonic contractions, and concentric from eccentric, in a named sporting action",
    ], "The lowering phase of a squat or press-up is an eccentric contraction of the muscle that did the lifting - the quadriceps or triceps - not a concentric contraction of the opposite muscle."),

    ("pe:3.1.1.2a", &[
        "Put the pathway of air in order from mouth and nose to alveoli",
        "Explain how each feature of the alveoli assists gaseous exchange by diffusion",
        "Describe how oxygen is carried as oxyhaemoglobin, and that haemoglobin can also carry carbon dioxide",
        "Compare the structure of arteries, capillaries and veins and link each to its function",
        "Explain how vasoconstriction and vasodilation redistribute blood to the working muscles during exercise",
    ], "Diffusion goes from high to low concentration - say which gas moves which way and why, not just 'gases are exchanged'."),

    ("pe:3.1.1.2b", &[
        "Name the four chambers of the heart and the blood vessels entering and leaving it",
        "Describe the pathway of blood and the cardiac cycle (diastole and systole) starting from any chamber",
        "Explain that valves open under pressure and close to prevent backflow",
        "Use Q = SV x HR to calculate and explain changes in cardiac output",
        "Interpret heart rate graphs, including the anticipatory rise and changes of intensity",
    ], "The pulmonary artery carries deoxygenated blood and the pulmonary vein oxygenated blood - the reverse of the usual rule, and the most common lost mark on the pathway."),

    ("pe:3.1.1.2c", &[
        "Explain inhalation and exhalation at rest through the intercostals, rib cage, diaphragm and changes in air pressure",
        "Explain the extra muscles used during exercise: pectorals and sternocleidomastoid to inhale, abdominals to exhale",
        "Identify tidal volume, inspiratory and expiratory reserve volumes and residual volume on a spirometer trace",
        "Describe and continue a trace to show how the volumes change from rest to exercise",
    ], "Tidal volume goes up during exercise while both reserve volumes go down - residual volume stays roughly the same. Students who say 'everything increases' lose both marks."),

    ("pe:3.1.1.3", &[
        "Define aerobic and anaerobic exercise and write both word summaries",
        "Justify whether an activity is aerobic or anaerobic from its duration and intensity",
        "Define EPOC (oxygen debt) and explain why breathing stays high after vigorous exercise",
        "Evaluate cool-down, diet manipulation and ice baths/massage as recovery methods for different activities",
    ], "Anaerobic means without enough oxygen, not with no oxygen at all - and a justification needs the intensity and duration of the named activity, not just the label."),

    ("pe:3.1.1.4", &[
        "List the immediate effects of exercise and the short-term effects up to 36 hours afterwards",
        "Explain the long-term effects of months of training, including hypertrophy of the heart and bradycardia",
        "Link long-term effects to the component of fitness they improve and to performance in a named activity",
    ], "Short-term and long-term have fixed meanings here: DOMS and nausea are short-term (up to 36 hours), a lower resting heart rate is long-term. Mixing the timescales scores nothing."),

    ("pe:3.1.2.1", &[
        "Identify first, second and third class levers in sporting actions at the elbow, knee and ankle",
        "Draw linear lever diagrams labelling fulcrum, load, effort, effort arm and load arm",
        "Calculate and interpret mechanical advantage as effort arm divided by load (resistance) arm",
        "Analyse the joint movements in push-ups, throw-ins, running, kicking, jumping, squats and bowling",
    ], "Second class has the load in the middle, and a diagram without the arms labelled cannot score the mechanical advantage mark."),

    ("pe:3.1.2.2", &[
        "Name the three planes and three axes of movement",
        "Pair each sporting action with its plane and axis: somersault/forward roll/running, 360 degree twist/discus, cartwheel",
        "Apply the pairs to unfamiliar actions such as a star jump",
    ], "The planes and axes pair across, not by name: the sagittal plane goes with the transverse axis, and the frontal plane with the sagittal axis."),

    ("pe:3.1.3.1", &[
        "Define health and fitness",
        "Explain how ill health can reduce fitness because a person cannot train",
        "Explain how someone can increase fitness while unhealthy, with an example",
    ], "A definition of health that says only 'free from illness' is incomplete - it must cover physical, mental and social well-being."),

    ("pe:3.1.3.2a", &[
        "Define the ten components of fitness and the four types of strength",
        "Give a sporting example for each component",
        "Justify why a component is or is not needed in a named activity or position",
    ], "Power is strength multiplied by speed, and agility is changing direction quickly while keeping control - vague definitions like 'being quick' do not score."),

    ("pe:3.1.3.2b", &[
        "Give the reasons for fitness testing and its limitations",
        "Describe the procedure for each of the eleven named tests, including equipment, rules and how the score is measured",
        "Evaluate whether a test is relevant to a performer in a given activity",
        "Define qualitative and quantitative data and compare test scores with national averages",
    ], "An evaluate question on a test needs both sides and must be applied to the named performer - a list of general limitations caps the answer."),

    ("pe:3.1.3.3a", &[
        "Define specificity, progressive overload, reversibility and tedium",
        "Define frequency, intensity, time and type",
        "Apply the principles to plan or improve a training programme for a named performer",
    ], "Progressive overload means gradually increasing the demand - an answer that just says 'training harder' misses the 'gradually' that earns the mark."),

    ("pe:3.1.3.3b", &[
        "Describe circuit, continuous, fartlek, interval/HIIT, static stretching, weight and plyometric training",
        "State the advantages and disadvantages (effects on the body) of each method",
        "Select and justify a method for an aerobic or anaerobic performer, taking training thresholds and rest into account",
    ], "Plyometrics works because an eccentric contraction is followed by a larger concentric one - learn that phrase, it is the physiological mark."),

    ("pe:3.1.3.4a", &[
        "Define training threshold and calculate maximum heart rate and the aerobic (60-80%) and anaerobic (80-90%) zones",
        "Use one rep max to set strength/power (above 70%, 3 sets of 4-8) and muscular endurance (below 70%, 3 sets of 12-15) loads",
        "Explain the factors that prevent injury in training",
    ], "Show the working for a training-zone calculation: 220 minus age first, then each percentage - a zone given with no method often scores only one of three marks."),

    ("pe:3.1.3.4b", &[
        "Explain how high altitude training works and why it raises red blood cell count",
        "Evaluate the benefits and limitations of altitude training for different performers",
        "Name the three training seasons and explain the aims of each",
        "Apply the seasons to a named sport's calendar",
    ], "Altitude training suits endurance performers; claiming it helps a sprinter or weightlifter without qualification is marked as a misunderstanding."),

    ("pe:3.1.3.5", &[
        "List the parts of a warm-up and a cool-down",
        "Explain the physical and psychological benefits of warming up",
        "Explain the benefits of cooling down, including removal of lactic acid and preventing DOMS",
        "Justify appropriate warm-up and cool-down activities for a named sport",
    ], "Stretching in a warm-up should be matched to the activity - generic answers like 'do some stretches' lose the application marks."),

    ("pe:3.1.4.1", &[
        "Define quantitative and qualitative data",
        "Name the methods for collecting each: questionnaires and surveys, interviews and observations",
        "Identify whether a given piece of data is qualitative or quantitative and justify it",
    ], "A questionnaire can collect both kinds of data - the justification must refer to whether the answer is a number or a description."),

    ("pe:3.1.4.2", &[
        "Present data in a table with clear headings and units",
        "Plot bar charts and line graphs with correctly labelled x and y axes",
        "Choose the right type of graph for the data given",
    ], "Label both axes with what is measured and its unit - an unlabelled axis loses the mark even when the plotting is perfect."),

    ("pe:3.1.4.3", &[
        "Read values and describe trends from tables, bar charts, line graphs and pie charts",
        "Carry out simple calculations such as differences, percentages and averages from the data",
        "Draw conclusions and explain what the data suggests about performance or participation",
    ], "Quote figures from the data when you describe a trend - 'it goes down' scores less than 'it falls from 72 to 62 bpm over ten weeks'."),

    ("pe:3.2.1.1", &[
        "Define skill and ability and tell them apart",
        "Classify skills on the basic/complex, open/closed, self-paced/externally paced and gross/fine continua",
        "Justify a classification with reference to the sporting example",
        "Define performance and outcome goals and give suitable examples",
    ], "A classification mark needs a justification tied to the example - 'a penalty is closed because the environment is stable: the ball is still and the keeper cannot move until it is struck'."),

    ("pe:3.2.1.2", &[
        "Evaluate the use of performance and outcome goals, including for beginners",
        "Explain each part of SMART: specific, measurable, accepted, realistic, time-bound",
        "Apply SMART targets to improve a named performer's performance",
    ], "In AQA's version the A is 'accepted', not 'achievable'; using the wrong word loses the mark on a recall question."),

    ("pe:3.2.1.3", &[
        "Draw the basic information processing model in box format",
        "Explain input (display, senses, selective attention), decision making (short- and long-term memory), output and feedback",
        "Apply the model to a skill from a sporting example",
    ], "Selective attention belongs to the input stage and memory to decision making - placing them in the wrong box is a common slip."),

    ("pe:3.2.1.4", &[
        "Describe visual, verbal, manual and mechanical guidance with an example of how each is given",
        "Describe positive/negative, knowledge of results/knowledge of performance, and intrinsic/extrinsic feedback",
        "Evaluate which guidance and feedback suit beginners and which suit elite performers",
    ], "Always say who the performer is: beginners need visual guidance and positive, extrinsic feedback; elite performers can use intrinsic feedback and knowledge of performance."),

    ("pe:3.2.1.5a", &[
        "Define arousal and draw a labelled inverted-U graph",
        "Describe the relationship between arousal and performance, including under- and over-arousal",
        "Link high or low optimal arousal to gross and fine skills",
        "Explain how deep breathing, mental rehearsal/visualisation/imagery and positive self-talk are carried out",
    ], "Link arousal to the skill, not the sport - a rugby tackle needs high arousal but a conversion kick in the same match needs low arousal."),

    ("pe:3.2.1.5b", &[
        "Define direct and indirect aggression with sporting examples",
        "Describe the characteristics of introverts and extroverts and the sports that suit each",
        "Define intrinsic and extrinsic (tangible and intangible) motivation",
        "Evaluate the merits of intrinsic and extrinsic motivation",
    ], "Direct aggression involves physical contact with the opponent; a legal tackle in rugby is still direct aggression, so do not assume aggression means foul play."),

    ("pe:3.2.2.1", &[
        "Describe how engagement patterns differ by gender, race/religion/culture, age, family/friends/peers and disability",
        "Explain how the twelve named factors, such as role models, accessibility and disposable income, affect participation",
        "Make justified links between a factor and a particular social group",
    ], "Each point needs a link to the group: 'media coverage' alone scores nothing, 'little TV coverage of women's sport means fewer female role models' scores."),

    ("pe:3.2.2.2", &[
        "Define commercialisation, sponsorship and the media, and explain the golden triangle",
        "Name the types of sponsorship and of media",
        "Explain positive and negative impacts of sponsorship and the media on the performer, sport, official, spectator and sponsor",
        "Explain positive and negative impacts of technology on the same five groups",
    ], "Make sure the impact is on the group the question names - an answer about the performer when the question asks about officials scores nothing."),

    ("pe:3.2.2.3a", &[
        "Define etiquette, sportsmanship, gamesmanship and contract to compete, with examples",
        "Describe the categories of prohibited substances and blood doping, with their effects and side effects",
        "Explain why beta blockers are restricted and which performers might use them",
        "Evaluate the advantages and disadvantages of PEDs for the performer and for the sport",
    ], "Match the drug to the performer: EPO and blood doping for endurance, anabolic agents for power, beta blockers for fine control, diuretics for weight categories."),

    ("pe:3.2.2.3b", &[
        "Explain the positive and negative influences of spectators at events",
        "Explain the reasons why hooliganism occurs",
        "Describe strategies used to combat hooliganism and evaluate how effective they are",
    ], "Evaluating a strategy means weighing it - e.g. extra security improves safety but is expensive - not listing more strategies."),

    ("pe:3.2.3.1", &[
        "Explain the physical, mental (emotional) and social health and well-being benefits of physical activity",
        "Explain how exercise improves fitness and the ability to work",
        "Explain how exercise can suit the needs of different people",
    ], "Keep the three kinds of health separate - serotonin release is a mental benefit, making friends is social; putting a benefit under the wrong heading loses the mark."),

    ("pe:3.2.3.2", &[
        "Define sedentary lifestyle and describe its possible consequences",
        "Define obesity and explain how it limits performance and causes physical, mental and social ill health",
        "Define the endomorph, mesomorph and ectomorph somatotypes",
        "Identify and justify the best somatotype for a sport or position",
    ], "Justify a somatotype with the demand of the position - 'an ectomorph suits a high jumper because low body weight and long levers help clear the bar' - not just the body shape."),

    ("pe:3.2.3.3", &[
        "Explain energy use in calories, with average daily needs and the factors that change them",
        "Explain the reasons for a balanced diet and its proportions of carbohydrate, fat and protein",
        "Describe the role of carbohydrates, fat, protein and vitamins/minerals",
        "Explain the effects of dehydration on performance and evaluate them for different activities",
    ], "Fat gives more energy than carbohydrate but only at low intensity - saying fat is the main fuel for exercise is the classic error."),
    // ---------- Media Studies (WJEC Eduqas C680QS) ----------
    ("media:2a", &[
        "Analyse any product with denotation and connotation, naming the sign before its meaning",
        "Explain how selection, combination and exclusion of elements create meaning, narrative and point of view",
        "Apply genre theory: repetition and variation, hybridity, intertextuality and how genres change over time",
        "Apply Propp's character roles and the idea of enigmas to print and audio-visual products",
        "Explain the relationship between technology and the look of a media product",
    ], "Describing what is on the page earns little. Every point needs the connotation and why the producer chose it for this audience."),

    ("media:2b", &[
        "Explain why the media re-present rather than present reality, through selection, construction and mediation",
        "Explain how stereotypes become established, change over time and let audiences read products quickly",
        "Explain why some social groups are under-represented or misrepresented, with examples from the set products",
        "Apply feminist approaches, including the male gaze, to the representation of gender",
        "Explain how audiences' own experiences and beliefs affect how they read a representation",
    ], "Saying a representation is 'stereotypical' is not analysis. Name the stereotype, show how media language builds it, and say whose interests it serves."),

    ("media:2c", &[
        "Explain conglomerate ownership, diversification and vertical integration with a real example",
        "Compare commercial, public service (licence fee) and not-for-profit funding models",
        "Explain convergence across platforms and how it helps organisations reach audiences",
        "Name the regulators (Ofcom, IPSO, the BBFC, PEGI) and explain why digital media are harder to regulate",
        "Explain how production processes, personnel and technology shape a final product",
    ], "Industry answers lose marks for vague claims. Name the organisation, the owner and the funding model, then explain the effect on the product."),

    ("media:2d", &[
        "Explain how and why products target mass and niche audiences, and the assumptions producers make",
        "Categorise audiences by demographics and psychographics",
        "Apply Blumler and Katz's Uses and Gratifications theory with a specific example for each need",
        "Compare active and passive audience ideas, and explain why interpretations differ and change over time",
        "Explain how media use connects to identity, including actual and desired self",
    ], "Listing the four gratifications is worth little. Each one needs a concrete feature of the set product that meets it."),

    ("media:2e", &[
        "Explain how a product reflects the time it was made through its representations, values and conventions",
        "Explain how social and cultural contexts shape production, marketing and audience response",
        "Explain how political contexts and ownership shape a product's viewpoint",
        "Use context to explain meaning rather than as a separate paragraph of history",
    ], "Context bolted on at the end scores low. Tie each piece of context to a specific choice in the product."),

    ("media:2.1a", &[
        "Analyse the layout, typography, images and cover lines of the Vogue (July 2021) and GQ (August 2019) covers",
        "Explain how each cover constructs representations of gender and ethnicity",
        "Explain how social and cultural contexts, including changes in editorial leadership and anti-racism debates, shaped the covers",
        "Compare the set covers with an unseen magazine cover in the same form",
    ], "Candidates describe the cover star and forget the cover lines. The written codes anchor the image and carry half the meaning."),

    ("media:2.1b", &[
        "Analyse how the two Bond posters use images, colour, typography and layout to sell the film",
        "Compare the representation of gender in the 1974 and 2021 posters",
        "Explain how historical and social contexts shaped each poster, including changing attitudes to women",
        "Apply genre and Propp's roles to the characters on each poster",
    ], "The Man with the Golden Gun is an illustrated poster from 1974. Judge it in its context, not only by today's standards."),

    ("media:2.1c", &[
        "Analyse how the Guardian (6 May 2025) and Sun (22 March 2025) front pages use layout, images and headlines",
        "Contrast broadsheet-style and tabloid conventions, and each paper's house style",
        "Explain how each front page represents gender, age, events and political viewpoints",
        "Explain how the political stance and audience of each paper shape its choices",
    ], "Many answers only analyse the main story. Off-leads, puffs and the skyline reveal the paper's audience and values too."),

    ("media:2.1d", &[
        "Analyse how the Quality Street (1956) and NHS 111 (2023) adverts use images, layout and written codes",
        "Explain how each advert represents gender, family and social groups",
        "Explain how 1950s post-war consumer culture and the post-Covid NHS shaped each advert",
        "Contrast a commercial product advert with a public-information campaign",
    ], "Quality Street answers often ignore the historical context. Link the gender roles and the luxury appeal to 1950s Britain."),

    ("media:2.1e", &[
        "Plan and write the 5-mark context question on a set product in about eight minutes",
        "Write a 25-mark comparison that moves between the set and unseen products in every paragraph",
        "Structure an argument around similarities, differences and the producers' choices",
        "Use theory and terminology to sharpen, not replace, analysis",
    ], "The 25-mark answer that analyses one product and then the other never gets past the middle band. Compare in every paragraph."),

    ("media:2.1f", &[
        "Explain The Sun's ownership by News UK and News Corp, and what conglomerate ownership means for it",
        "Explain how The Sun uses its website, app and social media to reach audiences (convergence)",
        "Explain how IPSO regulates the press and the challenges digital news brings",
        "Explain who reads The Sun, how it targets them and why they read it",
    ], "Section B is not textual analysis. Use the product as an example of the industry and audience issue asked about."),

    ("media:2.1g", &[
        "Explain the BBC's public service remit and licence-fee funding, and Ofcom's role",
        "Trace how Desert Island Discs has evolved since 1942 as a talk-radio programme",
        "Explain how BBC Sounds, podcasting and the archive reach new audiences",
        "Apply Uses and Gratifications to why people listen",
    ], "Radio answers often confuse commercial and public service radio. Know that the BBC carries no adverts and why."),

    ("media:2.1h", &[
        "Explain how No Time to Die was produced, financed and distributed across studios and franchise partners",
        "Explain vertical integration, conglomerates and franchises using the Bond example",
        "Explain how the 007 website and cross-media promotion show convergence",
        "Explain the BBFC's role and how films reach global audiences",
    ], "Film is industries only. Analysing the film's content earns nothing; explain the business behind it."),

    ("media:2.1i", &[
        "Explain Epic Games' free-to-play business model and in-game purchases",
        "Explain how Fortnite reaches audiences across platforms and uses events and partnerships",
        "Explain PEGI regulation and the challenges of regulating online games",
        "Explain why audiences play, using Uses and Gratifications and ideas of identity",
    ], "A free game still makes money. Show exactly how, with V-Bucks, battle passes and brand collaborations."),

    ("media:2.2a", &[
        "Identify the conventions of crime drama and sitcom, and how they have changed since the 1970s and 1990s",
        "Explain public service and commercial broadcasting, and the growth of streaming",
        "Explain how TV audiences are targeted and how their viewing has changed",
        "Explain how Ofcom regulates broadcasters and the challenge of streaming",
    ], "Genre answers list conventions. The marks are for explaining why a convention is used and how it is varied."),

    ("media:2.2b", &[
        "Analyse how Trigger Point's camerawork, editing, sound and mise-en-scène create tension",
        "Explain how the episode represents gender and ethnicity, and how it challenges stereotypes",
        "Explain ITV's commercial funding, HTM Television's production and the ITV/ITVX premiere",
        "Compare the episode with The Sweeney to show how the genre has changed",
    ], "Answers on Trigger Point forget industry. Know who made it, who paid for it and how it was released."),

    ("media:2.2c", &[
        "Analyse the conventions of 1970s action-led police drama in The Sweeney",
        "Explain how its representations of gender and masculinity reflect the 1970s",
        "Explain the ITV and commercial production context of the programme",
    ], "The Sweeney is a comparison text. Use it to show change over time, not as a separate essay."),

    ("media:2.2d", &[
        "Analyse how Man Like Mobeen uses sitcom conventions and where it varies them",
        "Explain how it represents British Muslim, working-class and Birmingham communities",
        "Explain BBC Three, iPlayer and BBC One as its routes to audiences",
        "Compare it with Friends to show how the sitcom has changed",
    ], "Say how the humour works on stereotypes. Answers that only say it 'breaks stereotypes' stay in the middle band."),

    ("media:2.2e", &[
        "Analyse how Modern Family's mockumentary form varies sitcom conventions",
        "Explain how the episode plays with stereotypes of family, gender, sexuality and ethnicity",
        "Explain ABC and Disney ownership, and licensing to streaming services",
        "Compare it with Friends to show how the sitcom has changed",
    ], "The episode's title is a joke about stereotypes. Explain how it uses them and then undercuts them."),

    ("media:2.2f", &[
        "Analyse the typical sitcom conventions of the Friends pilot",
        "Explain its representations of gender, sexuality and ethnicity in the 1990s context",
        "Explain NBC production and Channel 4's purchase for UK audiences",
    ], "Friends is not 'diverse'. Explain why its lack of diversity is itself a point about its time."),

    ("media:2.2g", &[
        "Identify music video conventions: performance, narrative and concept",
        "Explain how music videos work as marketing and how the music industry makes money",
        "Explain how record labels, streaming and YouTube distribute music globally",
    ], "Music industry answers stay generic. Tie every point to one of the set artists."),

    ("media:2.2h", &[
        "Analyse an artist's home page, its images and topical material",
        "Explain how websites link to videos, audio and online shops",
        "Explain how social and participatory media let fans interact and become producers",
    ], "Participatory media means audiences create and share, not just 'like'. Give concrete examples of fans producing content."),

    ("media:2.2i", &[
        "Analyse how Good as Hell uses performance, narrative, colour and setting",
        "Explain how it represents Lizzo, young women and black university culture",
        "Explain the body-positivity context and Lizzo's online brand",
        "Analyse lizzomusic.com and her social media",
    ], "Body positivity is the obvious point. Add how the marching-band setting and the three students' stories build the message."),

    ("media:2.2j", &[
        "Analyse how The Man uses its male alter ego, settings and editing to satirise double standards",
        "Explain how it represents gender and Swift's self-representation as director",
        "Explain the feminist and industry contexts, including ownership of her recordings",
        "Analyse taylorswift.com and her social media",
    ], "Say what the satire targets. Listing the scenes without the double standard behind each one gains little."),

    ("media:2.2k", &[
        "Analyse how the animated Superheroes video constructs meaning",
        "Explain how it represents black British children and aspiration",
        "Explain the 2020 Black Lives Matter context and Stormzy's education work",
        "Analyse stormzy.com and his social media",
    ], "The video is animated. Explain why animation suits its message rather than treating it like live action."),

    ("media:2.2l", &[
        "Analyse how Intentions blends documentary and performance",
        "Explain how it represents women, families and Bieber himself",
        "Explain the context of celebrity charity and image rebuilding",
        "Analyse justinbiebermusic.com and his social media",
    ], "Judge the video's purpose. Strong answers weigh genuine charity against promotion."),

    ("media:2.2m", &[
        "Analyse how Rio uses exotic locations, costume and editing",
        "Explain how it represents men, women and wealth",
        "Explain the early-MTV and 1980s consumer context",
    ], "Rio is marketing for an MTV age. Link the glamour to how videos sold bands in 1982."),

    ("media:2.2n", &[
        "Analyse how Waterfalls uses narrative, performance and early CGI",
        "Explain how it represents women, young black men and social issues",
        "Explain the 1990s context of HIV/AIDS awareness and R&B on MTV",
    ], "Waterfalls tells two stories. Explain how each narrative carries a warning, not only that it is 'about social issues'."),
    // ---------- Design and Technology (AQA GCSE 8552) ----------
    ("dt:3.1.1a", &[
        "Explain how automation and robotics change the layout of the workplace, the building and the tools people use",
        "Describe crowdfunding, virtual marketing and retail, co-operatives and fair trade as routes to launching an innovation",
        "Compare CAD, CAM, flexible manufacturing systems, just in time and lean manufacturing, with a benefit and a drawback of each",
        "Explain why a manufacturer would combine JIT with lean production, and what happens when a supply chain fails",
    ], "JIT is about stock arriving exactly when it is needed, not about making products quickly. Answers that say JIT means fast production score nothing."),

    ("dt:3.1.1b", &[
        "Explain the difference between finite and non-finite resources, and the problem of waste disposal",
        "Explain technology push and market pull with a real product for each, and how new technology changes job roles",
        "Explain how products are designed for disabled and elderly users and for different faiths and cultures",
        "Evaluate the environmental gains and costs of new products: continuous improvement, efficient working, pollution and global warming",
        "Weigh planned obsolescence against design for maintenance, using ethics and the environment as criteria",
    ], "Planned obsolescence is a deliberate design decision to limit a product's life. Saying it is when a product simply becomes out of date loses the mark."),

    ("dt:3.1.2", &[
        "Describe how electricity is generated from coal, gas and oil, and from nuclear fission",
        "Describe how wind, solar, tidal, hydro-electric and biomass generate power",
        "Give balanced arguments for and against fossil fuels, nuclear power and each renewable source",
        "Explain how kinetic pumped storage works, and compare alkaline and rechargeable batteries for a product",
    ], "Biomass is renewable but it is not carbon-free at the point of burning. Say it is carbon neutral over its life because regrowth absorbs the carbon released."),

    ("dt:3.1.3", &[
        "Explain what makes a material modern, with graphene, metal foams, titanium, coated metals, LCDs and nanomaterials as examples",
        "Define a smart material and explain how shape memory alloys, thermochromic and photochromic pigments respond to a stimulus",
        "Explain what a composite is and why GRP and carbon fibre reinforced plastic outperform their separate parts",
        "Describe technical textiles such as conductive and fire-resistant fabrics, Kevlar and microencapsulated microfibres",
    ], "A smart material changes a property in response to a stimulus and changes back. A composite does not respond to anything, so mixing the two definitions up costs the whole question."),

    ("dt:3.1.4", &[
        "Draw and explain a block diagram of input, process and output for a product",
        "Choose between light sensors, temperature sensors, pressure sensors and switches as inputs",
        "Explain how a programmed microcontroller works as a counter, a timer and a decision maker",
        "Choose buzzers, speakers or lamps as outputs and justify the choice",
    ], "A microcontroller is the process block, not an input or an output. Put the sensor, the chip and the buzzer in the right boxes."),

    ("dt:3.1.5", &[
        "Name and recognise linear, rotary, reciprocating and oscillating motion",
        "Identify first, second and third order levers and calculate mechanical advantage and distance moved",
        "Explain how bell cranks and push-pull linkages change the direction of motion",
        "Explain cams and followers, and calculate gear and pulley ratios and output speeds",
    ], "Always show the ratio working: driven teeth divided by driver teeth. The answer alone rarely earns the marks, and a ratio upside down gives the wrong speed."),

    ("dt:3.1.6.1a", &[
        "Name the listed papers (bleed proof, cartridge, grid, layout, tracing) and boards (corrugated, duplex, foil lined, foam core, ink jet card, solid white) with a use for each",
        "Classify hardwoods (ash, beech, mahogany, oak, balsa) and softwoods (larch, pine, spruce), and explain the botanical difference",
        "Compare MDF, plywood and chipboard and say why manufactured boards come in large stable sheets",
        "Match each paper, board or timber to a product and justify it with a property",
    ], "Hardwood and softwood describe the tree, not how hard the timber is. Balsa is a hardwood; saying otherwise is a classic lost mark."),

    ("dt:3.1.6.1b", &[
        "Classify ferrous metals, non-ferrous metals and alloys from the listed examples, and explain what makes a metal ferrous",
        "Explain why an alloy is made, using brass, stainless steel and high speed steel",
        "Explain the difference between thermoforming and thermosetting polymers and name the listed examples of each",
        "Classify natural, synthetic and blended fibres, and woven, non-woven and knitted textiles",
        "Match each metal, polymer or textile to a product and justify it with a property",
    ], "Thermoforming polymers can be reheated and reshaped; thermosets cannot, because of their cross-links. Mixing the two up loses any question about recycling or moulding."),

    ("dt:3.1.6.2", &[
        "Define absorbency, density, fusibility, and electrical and thermal conductivity",
        "Define strength, hardness, toughness, malleability, ductility and elasticity",
        "Apply the properties to the main material categories and to a product's function",
        "Distinguish physical properties from working properties",
    ], "Hardness and toughness are different: hardness resists scratching and wear, toughness resists breaking on impact. Glass is hard but not tough."),

    ("dt:3.2.1", &[
        "Explain how functionality, aesthetics, environmental factors, availability and cost shape a material choice",
        "Explain social, cultural and ethical factors, including FSC-certified sources",
        "Justify a material choice for a named product against several factors",
        "Calculate material costs, including bulk-buying discounts",
    ], "State the factor, then link it to the product. 'It is cheap' earns nothing; 'pine is cheap to buy in bulk, keeping the flat-pack price low' earns both marks."),

    ("dt:3.2.2", &[
        "Define tension, compression, bending, torsion and shear, with a product example of each",
        "Explain how lamination, bending, folding, webbing and fabric interfacing reinforce or stiffen a material",
        "Choose a reinforcing method suitable for your chosen material category",
    ], "Torsion is twisting and shear is sliding apart across a section. Describe the force acting, not the damage it causes."),

    ("dt:3.2.3", &[
        "Explain the ecological impact of deforestation, mining, drilling and farming",
        "Explain product mileage and the carbon produced in manufacture, and how a designer can reduce both",
        "Apply the six Rs: reduce, refuse, reuse, repair, recycle and rethink",
        "Explain the social footprint: safe working conditions, pollution and the effect on others",
    ], "Reuse and recycle are different: reuse keeps the product in its form, recycling reprocesses the material. Swapping them loses marks in six Rs questions."),

    ("dt:3.2.4", &[
        "Describe how paper is made from cellulose fibres in wood and grasses",
        "Describe how timber is felled, converted and seasoned, and how manufactured boards are made",
        "Describe how metals are extracted from ore and refined, and how polymers come from crude oil by fractional distillation and cracking",
        "Describe how textile fibres come from animal, vegetable and chemical sources and are spun into yarn",
        "Outline the stages of a life cycle assessment",
    ], "Conversion and seasoning are different stages: conversion cuts the log into boards, seasoning dries them. Most students write one when asked for the other."),

    ("dt:3.2.5a", &[
        "Explain how properties are chosen for the commercial products named for each category: packaging, toys and flat-pack, utensils and hand tools, seating and electrical fittings, sportswear and furnishings, vehicles and appliances",
        "Explain how properties are modified: moisture-resistant additives, seasoning, annealing, UV stabilisers, flame retardants, photosensitive PCB board and anodising",
        "Link each modification to the performance problem it solves",
    ], "Annealing softens a metal so it can be worked; it does not harden it. Say what the treatment does to the property and why that helps the product."),

    ("dt:3.2.5b", &[
        "Describe how card is cut, creased, scored, folded and perforated",
        "Describe cutting, drilling, chiselling, sanding and planing timber",
        "Describe cutting, drilling, turning, milling, casting, brazing and welding metals",
        "Describe cutting, drilling, casting, deforming, printing and welding polymers; sewing, pleating, gathering, quilting and piping textiles; cutting, drilling and soldering in electronics",
    ], "Name a specific tool and the step order. 'Cut it out' is not a process; 'mark out, clamp, cut with a coping saw, then file to the line' is."),

    ("dt:3.2.6", &[
        "Name the stock forms for each category, from paper sizes and board thickness to rod, bar and tube and E12 resistors",
        "Explain how each category is sold: by size, length, width, thickness, gauge, diameter, weight, roll or rating",
        "Name standard components such as KD fittings, rivets, zips and DIL IC packages",
        "Calculate the quantity, area or volume of stock needed and its cost",
    ], "Convert all units before calculating. Mixing millimetres with metres is the single most common reason the answer is out by a factor of a thousand."),

    ("dt:3.2.7", &[
        "Define prototype (one-off), batch, mass and continuous production with a product for each",
        "Explain why the manufacturing method changes with volume: tooling cost, labour skill, unit cost and flexibility",
        "Analyse and evaluate the different scales for a given product in an extended answer",
    ], "An 8-mark scales answer with no product examples is capped at 6. Name a real product for every scale you discuss."),

    ("dt:3.2.8a", &[
        "Explain how reference points, templates, jigs and patterns speed up making and keep parts identical",
        "Classify processes as wastage, addition, or deforming and reforming",
        "Describe how die cutting, turning, milling, 3D printing, soldering, vacuum forming, blow moulding, injection moulding and extrusion work",
    ], "A jig guides the tool and holds the work; a template is drawn round. Describing one as the other loses the mark."),

    ("dt:3.2.8b", &[
        "Explain what manufacturing to minimum and maximum sizes means, and read a tolerance",
        "Describe each category's commercial processes: offset lithography and die cutting, routing and turning, milling and casting, injection moulding and extrusion, weaving, dyeing and printing, pick and place and flow soldering",
        "Explain the quality control check for each category: registration marks, go/no-go fixtures, depth stops, laser settings, checking a print repeat, PCB exposure and etching times",
    ], "Quality control is a measurable check during manufacture. 'Look at it to see if it is good' is not quality control; name the check and what it measures."),

    ("dt:3.2.9", &[
        "Explain why finishes are applied: function (protection, corrosion) and aesthetics",
        "Describe printing, embossing and UV varnishing; painting, varnishing and tanalising; dip coating, powder coating and galvanising",
        "Describe polishing, printing and vinyl decals for polymers; printing, dyes and stain protection for textiles; PCB lacquering and lubrication",
        "Explain how to prepare a surface before a finish is applied",
    ], "Galvanising is a zinc coating on steel, not a paint. Name the coating material and how it protects: a barrier plus sacrificial protection."),

    ("dt:3.3.1", &[
        "Explain market research, interviews, human factors, focus groups and product analysis as ways to understand users",
        "Use anthropometric data and percentiles, and explain why designers often design for the 5th to 95th percentile",
        "Write a design brief and a design and manufacturing specification",
        "Explain why a brief is modified after investigation",
    ], "Anthropometrics is body measurement data; ergonomics is how a product fits the body in use. Use both words correctly or lose both marks."),

    ("dt:3.3.2", &[
        "Explain how deforestation creates constraints and opportunities for designers",
        "Explain how rising carbon dioxide levels and global warming influence material and energy choices",
        "Explain the need for fair trade and its effect on producers and prices",
    ], "Link the issue to a design decision. Describing global warming earns nothing unless you say what the designer does differently because of it."),

    ("dt:3.3.3", &[
        "Describe the work and influence of at least two of the listed designers",
        "Describe the approach and products of at least two of the listed companies",
        "Explain how studying past and present designers informs your own designing",
    ], "Name specific products and features. 'Dyson makes good vacuums' is not credit-worthy; 'bagless cyclone technology' is."),

    ("dt:3.3.4", &[
        "Explain collaboration, user-centred design and a systems approach",
        "Explain iterative design: sketch, model, test, evaluate, improve",
        "Explain design fixation and at least two ways to avoid it",
    ], "Iterative design is a repeated loop of testing and improving, not simply 'making several designs'. Describe the loop."),

    ("dt:3.3.5", &[
        "Choose between freehand, isometric, perspective, 2D and 3D drawings for a purpose",
        "Draw and read third angle orthographic drawings with conventions, dimensions and scale",
        "Explain system and schematic diagrams, exploded diagrams and annotated drawings",
        "Explain modelling with materials, audio and visual recording, mathematical modelling and computer-based tools",
    ], "In third angle, the plan goes above the front view and the right-hand view goes on the right. Swapping them turns it into first angle."),

    ("dt:3.3.6", &[
        "Explain how a prototype must meet the brief, the client's needs, innovation, function, aesthetics and marketability",
        "Evaluate a prototype critically, respond to feedback and suggest modifications",
        "Judge whether a prototype is fit for purpose",
    ], "Evaluate means judge. Every point needs a strength or weakness and a reason, not a description of what the product looks like."),

    ("dt:3.3.7", &[
        "Select materials and components for a prototype by functional need, cost and availability",
        "Use SI units and commercially available stock forms when specifying",
        "Justify a choice of alloy or other material for a stated function",
    ], "Answer all three factors separately when the question lists them. A combined sentence often earns only one of the marks."),

    ("dt:3.3.8", &[
        "Explain what a tolerance is and write it as a plus or minus value",
        "Calculate the largest and smallest acceptable sizes",
        "Explain why tolerances matter for fitting parts, resistors and seam allowances",
    ], "A tolerance of ±0.5 mm on 40 mm means 39.5 to 40.5 mm. Students often add the tolerance only once and give a single number."),

    ("dt:3.3.9", &[
        "Explain nesting and tessellation, and calculate how many parts fit a sheet",
        "Calculate area, volume, material requirements and percentage waste",
        "Explain allowances for cutting (kerf), seams and joints",
        "Use datums, reference points and coordinates for accurate marking out",
    ], "Answer the question asked: the number of parts that fit is a whole number, rounded down; the number of sheets needed is rounded up."),

    ("dt:3.3.10", &[
        "Select hand tools, machines and digital equipment appropriate to a material and task",
        "Explain safe working with each tool, including guards, PPE, extraction and risk assessment",
        "Justify a choice of equipment for quality of outcome",
    ], "Safety precautions must be specific to the machine. 'Be careful' earns nothing; 'lower the guard on the pillar drill and clamp the work in a machine vice' earns the mark."),

    ("dt:3.3.11", &[
        "Select wastage, addition, deforming and reforming techniques for a task and describe them in order",
        "Explain how to work each technique accurately and safely",
        "Explain how to prepare a surface, and choose and apply a finish for function and aesthetics",
        "Explain how corrosion and oxidation affect materials and how finishes protect them",
    ], "Preparation is part of the finish. A finish answer that skips cleaning, degreasing or sanding misses the first marking point."),
    // ---------- Food Preparation and Nutrition (AQA GCSE 8585) ----------
    ("food:3.2.1.1", &[
        "Explain the functions of protein: growth, repair and maintenance of body tissue, and a secondary source of energy",
        "Distinguish high and low biological value proteins, with plant and animal sources of each",
        "Explain protein complementation with meal examples such as beans on toast or dhal with rice",
        "Compare the protein alternatives TVP, soya, mycoprotein and tofu: source, nutrients and uses",
        "State the effects of deficiency (poor growth, kwashiorkor, weak immunity) and excess, and the adult reference intake",
    ], "HBV means all the essential amino acids, not 'a lot of protein'. Soya and quinoa are plant HBV proteins, and examiners reward knowing that exception."),

    ("food:3.2.1.2", &[
        "Explain the functions of fat: energy store, insulation, protecting organs and supplying vitamins A, D, E and K",
        "Compare saturated and unsaturated fats by structure, state at room temperature and typical sources",
        "Explain the link between high saturated fat intake, raised cholesterol and coronary heart disease",
        "Apply the reference values (no more than 35% of energy from fat, 11% from saturated fat) to modify a recipe",
    ], "Saying fats are 'bad for you' scores nothing. Name the type of fat, the health effect and a specific swap, such as grilling instead of frying or using a lower-fat cheese."),

    ("food:3.2.1.3", &[
        "Explain the function of carbohydrate as the body's main energy source, and the role of dietary fibre",
        "Classify carbohydrates as monosaccharides, disaccharides and polysaccharides, with examples of each",
        "Explain the difference between free sugars and naturally occurring sugars, and the 5% free-sugar limit",
        "Explain the effects of too little fibre (constipation, bowel disease) and too much sugar (weight gain, tooth decay, type 2 diabetes)",
        "Modify a recipe to increase fibre, for example wholemeal flour, skins left on, added pulses or oats",
    ], "Fibre is not digested, so it gives no energy. Students who say fibre 'gives slow-release energy' lose the mark; that is starch."),

    ("food:3.2.2.1a", &[
        "State the functions of vitamins A, D, E and K and a main source of each",
        "Explain the deficiency diseases: night blindness (A), rickets and osteomalacia (D), poor blood clotting (K)",
        "Explain why fat-soluble vitamins can build up to harmful levels, including vitamin A in pregnancy",
        "Explain how vitamin D works with calcium, and why sunlight is a major source",
    ], "Vitamin D does not build bones by itself: it helps the body absorb calcium. Answers that miss the calcium link rarely get the second mark."),

    ("food:3.2.2.1b", &[
        "State the functions, sources and deficiencies of B1, B2, B3, folic acid, B12 and vitamin C",
        "Explain why vitamin B12 is a concern for vegans and folic acid for women planning pregnancy",
        "Explain how water-soluble vitamins are lost by leaching into cooking water, by heat and by oxidation",
        "Give practical ways to conserve them: steam or microwave, cook briefly, cut just before cooking, use the cooking water",
        "Explain the antioxidant role of vitamins A, C and E in protecting body cells from damage",
    ], "A method to conserve vitamin C needs its reason. 'Steam the vegetables' is one mark; 'so the vitamin C does not dissolve into the water' earns the second."),

    ("food:3.2.2.2", &[
        "State the function, sources and effects of deficiency and excess for calcium, iron, sodium, fluoride, iodine and phosphorus",
        "Explain how vitamin C helps iron absorption and vitamin D helps calcium absorption",
        "Distinguish haem iron (meat) from non-haem iron (plants) and why vegetarians need to plan for iron",
        "Explain the risks of too much salt and give ways to cut salt when cooking, such as herbs and spices",
    ], "Iron deficiency anaemia causes tiredness and pale skin; it is not 'weak bones'. Mixing up the calcium and iron effects is the commonest lost mark here."),

    ("food:3.2.2.3", &[
        "Explain the functions of water: removing waste, cooling the body through sweat, and helping digestion",
        "Describe how water is lost from the body and the signs of dehydration",
        "State the daily fluid guidance (6 to 8 glasses) and what counts towards it",
        "Identify occasions when extra fluid is needed: exercise, hot weather, illness, pregnancy and breastfeeding",
    ], "Fluids count, not just plain water. But sugary drinks carry free sugars, so the best answers say which drinks to choose as well as how much."),

    ("food:3.2.3.1a", &[
        "Describe the Eatwell Guide's five groups and the current healthy eating guidelines",
        "Apply the guidelines to judge a meal, a menu or a food diary",
        "Explain portion size control and how to cost a recipe per portion",
        "Explain how to maintain a healthy body weight through energy balance",
    ], "Evaluating a food diary means naming the actual foods in it. A general lecture on healthy eating that ignores the diary caps you in the lowest level."),

    ("food:3.2.3.1b", &[
        "Explain how nutritional needs change for young children, teenagers, adults and the elderly",
        "Plan a balanced meal for vegetarian and vegan diets, with the nutrients at risk and how to replace them",
        "Plan meals for coeliac, lactose intolerant and high-fibre diets, naming safe substitutes",
        "Justify each choice with the nutrient it supplies and why that group needs it",
    ], "A teenage girl needs iron because of menstruation and calcium for peak bone mass. Listing nutrients without the reason for that life stage loses half the marks."),

    ("food:3.2.3.2", &[
        "Define basal metabolic rate and physical activity level, and the factors that affect BMR",
        "Explain energy balance and what happens when intake and output do not match",
        "Recall the recommended energy split: protein 15%, fat no more than 35%, carbohydrate 50% (free sugars no more than 5%)",
        "Calculate energy from nutrients using 4 kcal/g for protein and carbohydrate and 9 kcal/g for fat",
    ], "Energy calculations need the working shown. Grams times kcal per gram, then the percentage of the total: a bare answer usually scores only the final mark."),

    ("food:3.2.3.3", &[
        "Use food tables and nutritional analysis software to calculate the energy and nutrients in a recipe",
        "Compare the results with dietary reference values for the target group",
        "Modify a recipe to meet guidelines and explain the effect of each change",
        "Interpret nutritional data in a table and draw a justified conclusion",
    ], "When asked to modify a recipe, change the named ingredient and say what it improves. 'Use healthier ingredients' is too vague to score."),

    ("food:3.2.3.4", &[
        "Explain the dietary causes of obesity, coronary heart disease and high blood pressure",
        "Explain bone health: rickets and osteoporosis, and the roles of calcium, vitamin D and exercise",
        "Explain dental caries and the role of free sugars and fluoride",
        "Explain iron deficiency anaemia and type 2 diabetes, with dietary advice to reduce each risk",
    ], "Diet-related disease answers need a chain: which nutrient, what it does to the body, and the health result. A list of 'bad foods' does not reach the higher levels."),

    ("food:3.3.1.1", &[
        "Explain the reasons for cooking food: safety, flavour, texture, shelf life and variety",
        "Explain conduction, convection and radiation with a cooking example of each",
        "Explain why a sauce must be stirred (agitation) as it heats",
        "Describe how cooking changes the appearance, colour, flavour, texture and smell of food",
    ], "Most cooking uses more than one method. Boiling pasta is convection in the water and conduction through the pan; naming only one loses the mark."),

    ("food:3.3.1.2", &[
        "Classify cooking methods as water-based, dry and fat-based, with examples of each",
        "Explain how a method conserves or reduces nutrients, especially water-soluble vitamins and fat",
        "Choose and justify a method for a named food and outcome, such as braising a tough cut of meat",
        "Explain how marinades, browning and glazing change flavour, texture and appearance",
    ], "Justify the choice. 'Steam broccoli' earns a mark; 'because it does not sit in water, so less vitamin C leaches out' earns the rest."),

    ("food:3.3.2.1", &[
        "Explain denaturation by heat, acid and mechanical action, with examples such as marinades and whisking",
        "Explain coagulation of egg, meat and fish proteins and the temperatures involved",
        "Explain how gluten forms from glutenin and gliadin when flour is mixed with water and kneaded",
        "Explain foam formation when egg white is whisked, and why fat or yolk stops it forming",
        "Explain faults such as over-coagulated scrambled egg and curdled custard",
    ], "Denaturation is the unfolding of the protein; coagulation is the setting that follows. Using the words the wrong way round costs marks in almost every series."),

    ("food:3.3.2.2", &[
        "Explain gelatinisation in a sauce: starch granules absorb liquid, swell and burst to thicken it",
        "Explain how the starch-to-liquid ratio affects viscosity",
        "Explain dextrinisation (dry heat on starch, as in toast or crusts) and caramelisation (heat on sugar)",
        "Explain faults such as a lumpy or thin white sauce and how to prevent them",
    ], "Gelatinisation needs liquid and heat. Dextrinisation is dry heat on starch; caramelisation is heat on sugar. Swapping them is the classic error."),

    ("food:3.3.2.3", &[
        "Explain shortening: fat coats flour particles and stops long gluten strands forming, giving a crumbly texture",
        "Explain aeration in creaming, and plasticity in spreading and pastry",
        "Explain emulsification: an emulsifier such as lecithin in egg yolk holds oil and water together",
        "Explain faults such as a curdled cake mixture or split mayonnaise and how to prevent them",
    ], "For shortening, say what the fat stops: gluten development. 'Fat makes pastry short' just restates the word."),

    ("food:3.3.2.4", &[
        "Explain enzymic browning: cut cells release enzymes that react with oxygen and turn fruit brown",
        "Explain how acid, blanching, chilling and covering prevent enzymic browning",
        "Explain oxidation as a cause of vitamin C loss when vegetables are cut and left exposed",
        "Apply both ideas to a practical preparation task",
    ], "Lemon juice works because its acid lowers the pH and slows the enzyme. Students who say it 'stops air getting in' confuse two separate methods."),

    ("food:3.3.2.5", &[
        "Explain chemical raising agents: bicarbonate of soda, baking powder and self-raising flour produce carbon dioxide",
        "Explain mechanical methods that trap air: whisking, beating, folding, sieving, creaming and rubbing in",
        "Explain steam as a raising agent in choux pastry, Yorkshire puddings and batters",
        "Explain yeast fermentation and the conditions yeast needs: warmth, moisture, food and time",
        "Explain faults such as a sunken cake or a dense loaf",
    ], "Bicarbonate of soda alone leaves a soapy, bitter taste and a yellow colour unless an acid is present. That detail is what separates a grade 9 answer."),

    ("food:3.4.1.1", &[
        "Explain the growth conditions for microorganisms: temperature, moisture, food and time",
        "Explain how temperature control, pH and removing water control growth",
        "Define high-risk foods and give examples",
        "Explain that enzymes are biological catalysts, and how blanching and acids control them",
    ], "High-risk foods are ready to eat, moist and high in protein, and need no further cooking. Raw chicken is not a high-risk food by that definition, because it will be cooked."),

    ("food:3.4.1.2", &[
        "Describe the signs of enzymic action: ripening bananas and browning fruit",
        "Recognise mould growth on bread and cheese, and why mouldy food should be discarded",
        "Describe yeast action on fruits such as grapes, strawberries and tomatoes",
        "Explain how washing, chilling and correct storage slow spoilage",
    ], "Spoilage is not the same as food poisoning. Spoiled food looks or smells wrong; food with pathogenic bacteria can look perfectly normal."),

    ("food:3.4.1.3", &[
        "Explain how yeast is used to raise bread",
        "Explain how bacteria are used to make yoghurt and cheese",
        "Explain how moulds are used to make blue cheese",
        "Sequence the stages of cheese or yoghurt making and explain what each stage does",
    ], "In yoghurt the bacteria turn lactose into lactic acid, and the acid sets the milk protein. Missing the acid step leaves the explanation incomplete."),

    ("food:3.4.1.4", &[
        "Identify the sources of contamination: raw foods, surfaces and equipment, people, pests, waste",
        "Match campylobacter, E. coli, salmonella, listeria and staphylococcus aureus to their main sources",
        "Describe the general symptoms of food poisoning",
        "Explain the controls for each bacterium, such as thorough cooking, chilling and hand hygiene",
    ], "Staphylococcus aureus comes from people (skin, nose, cuts), so the control is personal hygiene, not just cooking. Matching the control to the source is the mark."),

    ("food:3.4.2.1", &[
        "Recall the key temperatures: freezer -18°C, fridge 0 to below 5°C, danger zone 5 to 63°C, cook and reheat to 75°C",
        "Explain use-by and best-before dates and which foods carry each",
        "Explain correct use of fridges and freezers, including where to store raw meat",
        "Explain ambient storage and why food should be covered",
    ], "Use-by is about safety and best-before about quality. Saying food is 'unsafe' after its best-before date loses the mark."),

    ("food:3.4.2.2", &[
        "Explain personal hygiene rules when preparing food",
        "Explain how to prevent cross-contamination: separate boards and utensils, raw below cooked",
        "Explain safe defrosting and reheating, and care with high-risk foods",
        "Explain how to use and clean a temperature probe correctly",
    ], "Give the reason with each rule. 'Tie hair back' is one mark; 'so hair and bacteria do not fall into the food' completes it."),

    ("food:3.5.1.1", &[
        "Explain how PAL, occasion, cost, preference, enjoyment, availability, healthy eating, income, lifestyle, seasonality, time of day and time available affect food choice",
        "Cost a recipe and a single portion from ingredient prices",
        "Modify a recipe to cut its cost while keeping it nutritious",
        "Apply the factors to a named person or family in a scenario",
    ], "Costing questions need the portion step. Work out the cost of the amount used, add the totals, then divide by the number of portions."),

    ("food:3.5.1.2a", &[
        "Describe the dietary practices of Buddhism, Christianity, Hinduism, Islam, Judaism, Rastafarianism and Sikhism",
        "Explain the reasons behind the rules, such as halal and kosher slaughter or the Ital diet",
        "Adapt a recipe so it suits a named religious or cultural group",
        "Explain how culture shapes food choice beyond religion",
    ], "Not every Hindu is vegetarian, but beef is avoided. Precise wording ('most', 'many', 'avoid') protects marks that sweeping statements lose."),

    ("food:3.5.1.2b", &[
        "Explain food choices linked to animal welfare, Fairtrade, local produce, organic and GM foods",
        "Distinguish a food intolerance (gluten, lactose) from a food allergy",
        "Explain the risks of allergies to nuts, egg, milk, wheat, fish and shellfish, including anaphylaxis",
        "Adapt recipes for people with intolerances and allergies",
    ], "An allergy is an immune-system reaction that can be fatal; an intolerance is a digestive reaction. Mixing them up is marked wrong every time."),

    ("food:3.5.1.3", &[
        "State the mandatory information on food labels",
        "Identify non-mandatory information such as provenance and serving suggestions",
        "Interpret a nutrition label and traffic-light front-of-pack labelling",
        "Explain how marketing influences choice: multi-buy offers, meal deals, advertising, media and point of sale",
    ], "Marketing answers should say why the technique works on the consumer, for example a meal deal encouraging extra spending on drinks and snacks."),

    ("food:3.5.2", &[
        "Describe the distinctive features of British cuisine and two international cuisines",
        "Describe the equipment and cooking methods typical of each cuisine",
        "Describe eating patterns and presentation styles",
        "Compare traditional and modern variations of a recipe",
    ], "Use named dishes, ingredients and equipment. 'Chinese food uses a wok and is stir fried' is thin; adding soy, ginger, rice and sharing dishes gives the detail."),

    ("food:3.5.3", &[
        "Explain how taste receptors and the olfactory system work together when tasting food",
        "Describe preference tests (paired preference, hedonic) and the triangle discrimination test",
        "Describe grading tests: ranking, rating and profiling (star diagrams)",
        "Explain how to set up a fair taste panel under controlled conditions",
    ], "Controlled conditions are about fairness: same portion size, same plates, coded samples, water between tastings. Each distinct control is a separate mark."),

    ("food:3.6.1.1", &[
        "Explain where and how food is grown, reared and caught",
        "Compare organic and conventional farming",
        "Compare free-range and intensive production",
        "Explain sustainable fishing, and the advantages and disadvantages of local, seasonal and GM foods",
    ], "Give both sides for farming methods. Free range has better welfare but costs more and uses more land; one-sided answers are capped."),

    ("food:3.6.1.2", &[
        "Explain the environmental impact of transport, food miles and the carbon footprint of food",
        "Explain reasons for buying seasonal and locally produced food",
        "Explain food waste in the home, in production and by retailers, and ways to reduce it",
        "Explain the environmental issues of packaging and more sustainable alternatives",
    ], "Evaluate, don't just list. Local food cuts food miles, but a tomato grown in a heated UK greenhouse can have a larger carbon footprint than one shipped from Spain."),

    ("food:3.6.1.3", &[
        "Define food security and explain the challenge of feeding a growing world population",
        "Explain how climate change, drought, flooding and insufficient land affect food supply",
        "Explain the roles of Fairtrade, GM foods and cutting food waste in food security",
        "Evaluate solutions at local and global level",
    ], "Food security is access to enough safe, nutritious, affordable food, not simply 'having enough food'. The definition has several parts and each earns credit."),

    ("food:3.6.2.1", &[
        "Distinguish primary processing (such as milling wheat or heat treating milk) from secondary processing",
        "Compare pasteurised, UHT, sterilised and micro-filtered milk",
        "Explain secondary processing examples: flour into bread and pasta, milk into cheese and yoghurt, fruit into jam",
        "Explain vitamin loss from heating and drying, and the effect of heat on the flavour and colour of milk",
    ], "Primary processing makes a raw material usable; secondary processing turns it into a different product. Flour is primary, bread secondary."),

    ("food:3.6.2.2", &[
        "Explain fortification, with the UK examples: white flour, breakfast cereals and fat spreads",
        "Explain how cholesterol-lowering spreads work and judge their efficacy",
        "Evaluate the use of additives: colourings, emulsifiers and stabilisers, flavourings and preservatives",
        "Evaluate the positive and negative aspects of GM foods",
    ], "Evaluation questions on additives and GM need both sides and a conclusion. Listing only disadvantages cannot reach the top level."),

    // ---------- Maths (Pearson Edexcel GCSE Mathematics (1MA1) Higher) ----------
    ("maths_edx:N1-3", &[
        "Order integers, decimals and fractions, including negatives, and use =, ≠, <, >, ≤ and ≥ correctly",
        "Add, subtract, multiply and divide integers, decimals, fractions and mixed numbers, positive and negative, by written methods",
        "Use a given multiplication fact and place value to write down related results",
        "Apply the order of operations, including powers, roots and reciprocals, and treat fraction lines and root signs as brackets",
        "Use inverse operations to check answers, work backwards and cancel before calculating",
    ], "Mixed numbers multiplied or divided without first becoming improper fractions, and × and ÷ not worked left to right, cost the most marks. On a \"show that\" every fraction step must be written."),

    ("maths_edx:N4-5", &[
        "Write a number as a product of its prime factors in index form, and use it to test for squares, cubes and divisibility",
        "Find the HCF and LCM of two or three numbers, including from given prime factorisations",
        "Recognise HCF and LCM problems in context and finish the answer in context",
        "List outcomes systematically so none are missed or repeated",
        "Use the product rule to count arrangements, handling restrictions first and dividing when order does not matter",
    ], "Prime factors written as a list instead of a product, and HCF and LCM mixed up, lose the easiest marks. In counting questions, fill the restricted position first."),

    ("maths_edx:N6-7", &[
        "Recall squares, cubes and powers of 2, 3, 4 and 5, and the roots that go with them",
        "Estimate powers and roots of any positive number by trapping them between known powers",
        "Use the index laws with positive, zero, negative and fractional indices, with numbers and letters",
        "Evaluate expressions like a^(−m/n) without a calculator: reciprocal, root, then power",
        "Write numbers as powers of a given base and solve equations such as 9^x = 27",
    ], "A negative index means a reciprocal, not a negative number, and a fractional index means a root: 64^(1/2) is 8, not 32. Take the root before the power."),

    ("maths_edx:N8", &[
        "Calculate exactly with fractions, surds and multiples of π, never rounding when an exact answer is asked for",
        "Simplify surds by taking out the largest square factor, and add, subtract, multiply and divide them",
        "Expand brackets containing surds, including squared brackets and the difference of two squares",
        "Rationalise denominators of the form √a, b√a and a ± √b",
        "Give lengths, areas and volumes exactly in terms of π and in surd form",
    ], "When the question says \"exact\" or \"in terms of π\", a rounded decimal loses the final mark. When rationalising, multiply the top by the same thing as the bottom."),

    ("maths_edx:N9", &[
        "Convert between ordinary numbers and standard form A × 10ⁿ with 1 ≤ A < 10, for large and small numbers",
        "Order numbers given in standard form, comparing powers first",
        "Multiply, divide, add and subtract in standard form without a calculator, adjusting A back into range",
        "Enter standard form correctly on a calculator and write the display properly",
        "Solve problems in context with standard form, including how many times bigger and per-unit questions",
    ], "Answers like 24 × 10³ are not in standard form, and adding numbers is not done by adding powers. Remember that making A smaller makes the power bigger."),

    ("maths_edx:N10-12", &[
        "Convert between terminating decimals and fractions, and decide from the denominator's prime factors whether a fraction terminates",
        "Write fractions as recurring decimals using dot notation",
        "Prove algebraically that a recurring decimal equals a given fraction, including when the repeat starts after the first decimal place",
        "Move between ratios and fractions of a whole, and combine fractions in multi-step ratio problems",
        "Use fractions, percentages and multipliers as operators, including finding the whole from a given part",
    ], "In recurring-decimal proofs the two multiples of x must be written out with matching recurring tails before subtracting; writing 100x = 45.45 without the dots or \"…\" loses the method mark."),

    ("maths_edx:N13-16", &[
        "Convert between metric units, including area, volume, time and compound units such as km/h to m/s",
        "Round to a given number of decimal places or significant figures and estimate by rounding to 1 significant figure",
        "Write error intervals for rounded and truncated values using inequality notation",
        "Find upper and lower bounds of sums, differences, products and quotients by choosing the right bound of each input",
        "Use bounds to give an answer to a suitable degree of accuracy, with a reason",
    ], "For the maximum of a − b or a ÷ b you must use the lower bound of b. Using upper bound with upper bound throughout is the commonest way to lose every mark on a bounds question."),

    ("maths_edx:A1-3", &[
        "Write and read algebraic notation exactly, including powers, fractions as coefficients and brackets",
        "Substitute positive, negative and fractional values into expressions and scientific formulae without sign errors",
        "Tell apart expressions, equations, formulae, identities and inequalities, and terms and factors",
        "Use a counter-example to show a statement is not an identity",
        "Match coefficients in an identity to find unknown constants",
    ], "Substitute every negative value in brackets. Writing −3² instead of (−3)² turns +9 into −9 and costs the accuracy mark."),

    ("maths_edx:A4", &[
        "Expand single, double and triple brackets and collect like terms, including expressions with surds",
        "Factorise fully by taking out the highest common factor, including a bracket as the common factor",
        "Factorise quadratics x² + bx + c and ax² + bx + c, and the difference of two squares",
        "Simplify expressions using the laws of indices, including powers of products",
        "Simplify, multiply, divide, add and subtract algebraic fractions by factorising first",
    ], "Only whole factors cancel in an algebraic fraction: factorise the top and bottom completely before cancelling, and bracket the second numerator when subtracting."),

    ("maths_edx:A5-6", &[
        "Recall and use standard formulae such as the area of a circle, Pythagoras and the equations of motion",
        "Change the subject of a formula, including ones with fractions, powers and roots",
        "Rearrange when the new subject appears twice by collecting terms and factorising",
        "Show that two expressions are identical by expanding and simplifying one side",
        "Write algebraic proofs using n, 2n and 2n + 1, ending with a concluding sentence, and disprove statements with a counter-example",
    ], "When the subject appears twice, collect those terms on one side and factorise it out. In a proof, finish with a sentence saying why the result has the property — \"= 8n\" on its own often loses the final mark."),

    ("maths_edx:A7", &[
        "Use function notation to find outputs, including f of an expression, and solve f(x) = k for the input",
        "Find an inverse function by writing y = f(x) and making x the subject, including when x appears twice",
        "Find composite functions such as fg(x), gf(x) and ff(x), applying the inner function first",
        "Solve equations involving composite and inverse functions, including ones that lead to quadratics",
        "Find unknown constants in a function from given input and output values",
    ], "In fg(x) the function g acts first: substitute the whole of g(x), in brackets, into f. Doing them in the wrong order gives gf(x) and scores nothing."),

    ("maths_edx:A8-10", &[
        "Plot straight-line graphs from a table of values or from the intercepts, and find midpoints",
        "Find and interpret the gradient and y-intercept of a line, rearranging ax + by = c into y = mx + c first",
        "Find the equation of a line through two points or through one point with a given gradient",
        "Use gradients to identify and form parallel and perpendicular lines, including perpendicular bisectors",
        "Interpret the gradient of a real-life line as a rate with units and the intercept as a starting value",
    ], "Rearrange to y = mx + c before reading the gradient: 2y = 6x + 8 has gradient 3, not 6. For a perpendicular gradient, flip the fraction and change the sign."),

    ("maths_edx:A11-12", &[
        "Read the roots, y-intercept and turning point of a quadratic from its graph, and find the roots by factorising",
        "Complete the square to find the turning point of a quadratic, including when the coefficient of x² is not 1 or is negative",
        "Sketch linear, quadratic, cubic, reciprocal and exponential graphs with their intercepts and asymptotes labelled",
        "Sketch y = sin x, y = cos x and y = tan x for angles of any size and use their symmetry to find every solution in a range",
        "Interpret turning points and intercepts in context, such as the maximum height of a projectile",
    ], "The turning point of (x − 3)² − 4 is (3, −4), not (−3, −4): the x-coordinate has the opposite sign to the number in the bracket. And a trig equation almost always has more than the one solution your calculator gives."),

    ("maths_edx:A13", &[
        "Sketch y = f(x) + a and y = f(x + a) as translations, and describe them with a column vector",
        "Sketch y = −f(x) and y = f(−x) as reflections in the x-axis and the y-axis",
        "Find the image of a turning point, labelled point or asymptote under a transformation",
        "Complete the square to show a quadratic is a translation of y = x² and state the vector",
        "Apply translations and reflections to the graphs of sin x, cos x, tan x, 1/x and kˣ",
    ], "A change inside the bracket moves the graph the opposite way to how it looks: f(x + 3) is a translation 3 to the left, by the column vector (−3, 0)."),

    ("maths_edx:A14-15", &[
        "Plot reciprocal, exponential and non-standard graphs and read approximate solutions from them",
        "Find the gradient of a straight section and interpret it as a speed, acceleration or rate with units",
        "Estimate the gradient of a curve at a point by drawing a tangent, and an average rate using a chord",
        "Work out the area under a velocity–time graph as the distance travelled, using strips for curves",
        "Say whether a strip estimate is an overestimate or underestimate, with a reason",
        "Find and use an exponential model y = abˣ for growth or decay in context",
    ], "On a velocity–time graph the gradient is the acceleration and the area underneath is the distance. Mixing the two up — or forgetting to convert minutes to hours — loses the marks."),

    ("maths_edx:A16", &[
        "Recognise x² + y² = r² as a circle with centre the origin and read off its radius, in surd form when needed",
        "Write the equation of a circle centred at the origin that passes through a given point",
        "Decide whether a point lies on, inside or outside a circle",
        "Find the equation of the tangent at a given point using the radius gradient and the negative reciprocal",
        "Use a tangent's equation to find where it meets the axes and solve follow-on area problems",
    ], "The tangent is perpendicular to the radius, so its gradient is the negative reciprocal of the radius gradient — using the radius gradient itself, or forgetting to change the sign, loses most of the marks."),

    ("maths_edx:A17", &[
        "Solve linear equations with the unknown on both sides, including brackets and negative coefficients",
        "Clear fractions by multiplying every term by the lowest common multiple, including algebraic denominators",
        "Recognise when an equation has no solution or is an identity",
        "Find approximate solutions from where two graphs cross, and confirm them algebraically",
        "Form a linear equation from a context such as angles or ages, solve it and answer the question asked",
    ], "When clearing fractions, multiply every term — including the whole numbers — and put brackets round each numerator, so that −(x − 2)/3 becomes −4(x − 2) = −4x + 8."),

    ("maths_edx:A18", &[
        "Rearrange a quadratic equation to ax² + bx + c = 0, including ones with brackets or algebraic fractions",
        "Solve a quadratic by factorising, including when the coefficient of x² is not 1",
        "Solve by completing the square, giving exact answers in surd form",
        "Recall and use the quadratic formula, giving answers to the accuracy asked for",
        "Find approximate solutions from a graph, drawing a straight line to solve related equations",
        "Form a quadratic from a context, solve it and reject any impossible solutions",
    ], "Get zero on one side before you factorise: (x + 3)(x − 2) = 14 does not mean x + 3 = 14. And in the formula, keep the signs of a, b and c in brackets so that −b and −4ac come out right."),

    ("maths_edx:A19", &[
        "Solve two linear simultaneous equations by elimination or substitution",
        "Form and solve simultaneous equations from a worded context, answering in context",
        "Solve a linear and a quadratic equation simultaneously by substitution, including a line and a circle",
        "Pair each x-value with its correct y-value using the linear equation",
        "Show that a line is a tangent to a curve from a repeated root",
        "Find approximate solutions from where two graphs intersect",
    ], "Squaring the bracket wrongly after substituting, e.g. writing (2x + 1)² as 4x² + 1, and then giving x-values and y-values that are not paired up."),

    ("maths_edx:A20", &[
        "Show that an equation has a solution in an interval by a change of sign, and state the conclusion",
        "Rearrange an equation into the form x = g(x) exactly as asked",
        "Use an iterative formula with a starting value to find x₁, x₂, x₃ on a calculator",
        "Iterate to a solution to a given accuracy and confirm it by testing the bounds",
        "Recognise when an iteration converges or diverges, and find the equation it solves",
    ], "Working out f(a) and f(b) but never writing \"change of sign, so there is a solution between a and b\" — the conclusion is the mark."),

    ("maths_edx:A21-22", &[
        "Translate a worded situation or diagram into an expression, formula or equation, then solve it and interpret the answer",
        "Form and solve two simultaneous equations from a context",
        "Solve linear inequalities, including double inequalities, and list integer solutions",
        "Solve quadratic inequalities by finding critical values and sketching the parabola",
        "Show solution sets on a number line and in set notation",
        "Draw and interpret regions defined by inequalities in two variables",
    ], "Forgetting to reverse the inequality when dividing by a negative, and writing a 'greater than' quadratic inequality as one statement instead of two separate intervals outside the roots."),

    ("maths_edx:A23-25", &[
        "Generate terms from a term-to-term rule or an nth-term formula",
        "Recognise square, cube and triangular numbers, arithmetic, geometric and Fibonacci-type sequences",
        "Find the nth term of a linear sequence and decide whether a number is a term",
        "Find the nth term of a quadratic sequence using second differences",
        "Work with geometric sequences whose ratio is a fraction or a surd, keeping answers exact",
        "Form and solve equations to find missing terms of Fibonacci-type sequences",
    ], "Forgetting to halve the second difference in a quadratic sequence, so 4, 11, 22, 37 starts 4n² instead of 2n²."),

    ("maths_edx:R1-2", &[
        "Convert between metric units of length, mass, capacity and time, including hours and minutes",
        "Convert area and volume units by squaring or cubing the length factor",
        "Use speed, density and pressure, rearranging each formula and converting compound units",
        "Compare rates such as pay and unit prices to find the best buy",
        "Use scale factors, scale drawings and map scales, including areas on maps",
    ], "Converting m² to cm² by multiplying by 100 instead of 10 000, and treating 2.4 hours as 2 hours 40 minutes."),

    ("maths_edx:R3-8", &[
        "Write one quantity as a fraction of another, including fractions greater than 1",
        "Simplify ratios, including mixed units, decimals and the form 1 : n",
        "Share an amount in a ratio when given the total, one share or the difference",
        "Move between ratios and fractions, and use equal ratios to scale recipes, convert currencies and compare mixtures",
        "Combine two ratios and solve problems where a ratio changes",
        "Link a fixed ratio to a straight-line graph through the origin",
    ], "Dividing by the wrong number of parts — treating a difference or one person's share as if it were the total — and confusing boys : girls = 3 : 5 with boys being 3/5 of the class."),

    ("maths_edx:R9", &[
        "Convert between percentages, fractions and decimals, including percentages over 100%",
        "Increase or decrease an amount by a percentage using a single multiplier",
        "Work out a percentage change, profit or loss, always dividing by the original value",
        "Find the original value after a percentage change by dividing by the multiplier",
        "Calculate simple interest and work backwards from it to find a rate or a number of years",
        "Combine successive percentage changes by multiplying their multipliers",
    ], "Reverse percentages: after a 20% cut to £84, the original is 84 ÷ 0.8 = £105, not £84 plus 20% of £84. And percentage change always divides by the original, never the new value."),

    ("maths_edx:R10", &[
        "Decide whether a situation is direct proportion, inverse proportion or neither",
        "Solve direct proportion problems with the unitary method or a scale factor, including best buys",
        "Solve inverse proportion problems by finding the total work first (worker-days, machine-hours)",
        "Use y = kx and y = k/x, finding k from a pair of values, a table or a graph",
        "Recognise the graphs: a straight line through the origin for direct, a y = k/x curve for inverse",
        "Handle problems with several quantities and round sensibly in context",
    ], "A straight line only shows direct proportion if it passes through the origin, and more workers means less time. Sense-check which way the answer should move before you calculate."),

    ("maths_edx:R11", &[
        "Use speed = distance ÷ time, density = mass ÷ volume and pressure = force ÷ area, rearranging as needed",
        "Convert between hours and minutes correctly, and give times in the form asked for",
        "Work out average speed or mixture density from totals, never by averaging",
        "Convert compound units such as km/h to m/s, g/cm³ to kg/m³ and N/cm² to N/m²",
        "Solve problems with other rates: pay and overtime, unit pricing, flow rates and population density",
    ], "Minutes are not decimals of an hour: 1 h 45 min is 1.75 h, not 1.45 h. And average speed is total distance over total time, not the mean of the speeds."),

    ("maths_edx:R12", &[
        "Compare lengths, areas and volumes of similar shapes using ratios a : b, a² : b² and a³ : b³",
        "Work back from an area or volume ratio to the length ratio by square- or cube-rooting",
        "Use linear, area and volume scale factors with models, maps and similar solids, including masses",
        "Explain why the trigonometric ratios are fixed for a given angle using similar triangles",
        "Write ratios of lengths, areas and volumes in simplest form or as 1 : n",
    ], "To go from an area ratio to a volume ratio, square-root to get lengths first and then cube. Squaring or cubing the wrong ratio is the mark most people drop."),

    ("maths_edx:R13", &[
        "Turn a proportion statement, including squares, cubes and square roots, into an equation with a constant k",
        "Find k from one pair of values and write the formula out in full",
        "Use the formula to find either variable, rejecting negative roots where they make no sense",
        "Explain that X inversely proportional to Y is the same as X proportional to 1/Y",
        "Work out the effect on y of multiplying or changing x by a percentage, without finding k",
        "Recognise the graph shapes of direct and inverse proportion relationships",
    ], "Inverse square means y = k/x², so k = y × x²; people divide instead of multiply, or write y = kx² by mistake. Always write the full formula once you have found k."),

    ("maths_edx:R14-15", &[
        "Interpret the gradient of a straight-line graph as a rate of change, in context and with units",
        "Recognise graphs of direct proportion (a line through the origin) and inverse proportion (a y = k/x curve)",
        "Work out an average rate of change as the gradient of a chord, from a graph, a table or an equation",
        "Estimate an instantaneous rate of change by drawing a tangent and finding its gradient",
        "Estimate a rate at a point from an equation using chords over smaller and smaller intervals",
    ], "A rate at one moment needs a tangent, not a chord, and its gradient must be read using the axis scales and stated with units in context, such as metres per second."),

    ("maths_edx:R16", &[
        "Use a multiplier raised to a power to find the result of compound interest, depreciation, growth or decay",
        "Work backwards from a final amount by dividing by the multiplier to the power n",
        "Find the rate from start and end values, and find the time needed by trial and improvement",
        "Compare simple and compound interest and choose the better option with evidence",
        "Generate and interpret terms of an iterative process such as uₙ₊₁ = 1.2uₙ − 500",
    ], "A 15% loss is × 0.85 applied every year, not 15% of the original taken off each time. In \"how many years\" questions, write down the values either side of the target — the bare number earns little."),

    ("maths_edx:G1-2", &[
        "Use the correct words and labels: vertex, parallel, perpendicular, regular, angle ABC at the middle letter, side a opposite angle A",
        "Construct a perpendicular bisector, a perpendicular to a line from or at a point, and an angle bisector with ruler and compasses only",
        "Build 60°, 30°, 90° and 45° angles and triangles from their sides",
        "Draw the locus for a rule and shade the region that satisfies several rules at once, to scale",
        "Explain why the perpendicular distance is the shortest distance from a point to a line",
    ], "The construction arcs are the method mark: a neat line with the arcs rubbed out scores almost nothing. Keep the compasses at the same width for both ends."),

    ("maths_edx:G3-4", &[
        "Use angles on a straight line, around a point and vertically opposite, giving the reason in standard words",
        "Find alternate, corresponding and co-interior angles on parallel lines",
        "Derive and use the angle sum of a triangle, the exterior angle of a triangle and the angle sum of any polygon",
        "Find interior and exterior angles of regular polygons and the number of sides from an angle",
        "State and derive the properties of squares, rectangles, rhombuses, parallelograms, kites and trapeziums",
    ], "The reason marks are separate from the angle marks: \"Z angles\" or \"because they are parallel\" scores nothing — write \"alternate angles are equal\". For regular polygons, 360 ÷ n is the exterior angle, not the interior."),

    ("maths_edx:G5-6", &[
        "Know the congruence conditions SSS, SAS, ASA and RHS, and why AAA and SSA do not prove congruence",
        "Prove two triangles congruent with three matching facts, each with a reason",
        "Use corresponding sides and angles of congruent triangles to prove lengths or angles equal",
        "Combine angle facts, similarity and quadrilateral properties to derive results such as equal base angles of an isosceles triangle",
        "Follow and write a proof of Pythagoras' theorem and other short geometric proofs",
    ], "Each of the three pairs needs its reason — \"(given)\", \"(radii)\", \"(common)\". Then finish: if you were asked to prove a side equal, stopping at \"so the triangles are congruent\" drops the last mark."),

    ("maths_edx:G7-8", &[
        "Reflect, rotate, translate and enlarge shapes on coordinate axes, using coordinate rules and tracing paper",
        "Describe fully a single transformation: mirror line; centre, angle and direction; column vector; scale factor and centre",
        "Enlarge by fractional and negative scale factors, and find the centre and scale factor from an object and image",
        "Combine reflections, rotations and translations and describe the single equivalent transformation",
        "Identify invariant points and lines, and say what a transformation keeps the same",
    ], "\"Describe fully\" means every detail: a rotation needs its centre, angle and direction; an enlargement needs its scale factor and centre. Two transformations in reply to \"a single transformation\" scores nothing."),

    ("maths_edx:G9-10", &[
        "Name the parts of a circle: centre, radius, chord, diameter, circumference, tangent, arc, sector and segment",
        "Spot and apply the eight circle theorems, including the alternate segment theorem, with the reason in standard words",
        "Combine circle theorems with isosceles triangles, angle sums and parallel lines in multi-step problems",
        "Use the tangent–radius right angle and the perpendicular bisector of a chord to find lengths",
        "Prove the circle theorems and use them to prove related results",
    ], "Every step needs its theorem in the standard words — \"angle at the centre is twice the angle at the circumference\", not \"double rule\". And two radii make an isosceles triangle: that step is the one most often missing."),

    ("maths_edx:G11-15", &[
        "Find midpoints, lengths and gradients from coordinates and use them to find missing vertices or prove what shape a set of points makes",
        "Count faces, edges and vertices of prisms, pyramids and curved solids, and check with F + V − E = 2",
        "Draw plans and elevations from a 3D drawing, and interpret them to rebuild a solid or find its volume",
        "Convert units of length, area, volume, capacity, mass and time, squaring or cubing the length factor for area and volume",
        "Use map scales and three-figure bearings, including back bearings and angle facts with parallel north lines",
    ], "Area and volume conversions: 1 m² is 10 000 cm² and 1 m³ is 1 000 000 cm³, and a map scale must be squared before it is used on an area."),

    ("maths_edx:G16-18", &[
        "Find areas of triangles, parallelograms, trapezia, circles and composite shapes, and perimeters including arcs",
        "Find the volume and surface area of prisms and cylinders, giving answers in terms of π when asked",
        "Use given formulae for cones, spheres and pyramids, finding slant or perpendicular height with Pythagoras when needed",
        "Find volumes of frustums and composite solids, using similarity for the part removed",
        "Calculate arc lengths, sector areas and segment areas, and work backwards to a missing angle or radius",
    ], "Using the slant height where the perpendicular height is needed (or the other way round) in cone and pyramid questions, and leaving the two radii out of a sector's perimeter."),

    ("maths_edx:G19", &[
        "Prove two triangles congruent using SSS, SAS, ASA or RHS, giving a reason for every statement",
        "Show that two triangles are similar and match their corresponding sides using the equal angles",
        "Find missing lengths in similar shapes, including nested and hourglass triangles",
        "Use area scale factor k² and volume scale factor k³, including for surface area, capacity and mass",
        "Work backwards from an area or volume ratio to the length scale factor by square- or cube-rooting",
    ], "Using the length scale factor on an area or volume: if lengths are multiplied by k, areas are multiplied by k² and volumes by k³, so root first when you are given areas or volumes."),

    ("maths_edx:G20-21", &[
        "Use Pythagoras' theorem to find any side of a right-angled triangle, leaving surds exact on the non-calculator paper",
        "Choose sin, cos or tan to find a missing side or angle, including angles of elevation and depression",
        "Split isosceles and other triangles with a perpendicular to create right-angled triangles",
        "Find lengths and angles in 3D solids, including the angle between a line and a plane",
        "Recall the exact values of sin and cos for 0°, 30°, 45°, 60° and 90°, and tan for 0°, 30°, 45° and 60°, and use them without a calculator",
    ], "In 3D, taking the angle to an edge instead of to the line's projection on the base, and rounding the base diagonal before using it in the second triangle."),

    ("maths_edx:G22-23", &[
        "Label any triangle with each side opposite its matching angle and choose the sine rule, cosine rule or area formula from what is known",
        "Use the sine rule to find sides and angles, giving both possible angles when the obtuse one is also valid",
        "Use the cosine rule to find a side from two sides and the included angle, or an angle from three sides",
        "Use Area = ½ab sin C to find an area, or work backwards to a missing side or angle",
        "Solve multi-step, bearings and 3D problems by building the right triangle first, keeping full accuracy between steps",
    ], "Pairing a side with the wrong angle in the sine rule, and using an angle that is not between the two sides in the cosine rule or ½ab sin C."),

    ("maths_edx:G24-25", &[
        "Describe translations with column vectors and add, subtract and scale column vectors, finding magnitudes with Pythagoras",
        "Write any vector on a diagram as a route through known vectors, using AB = b − a",
        "Find the position of a midpoint or a point dividing a line in a given ratio",
        "Prove lines parallel by showing one vector is a multiple of the other, and points collinear by adding a common point",
        "Find unknown scalars by equating coefficients, including where two lines meet",
    ], "Getting the direction wrong (AB is b − a, not a − b), and stopping a collinear proof at \"parallel\" without stating the common point."),

    ("maths_edx:P1-5", &[
        "Find probabilities of equally likely outcomes and use the fact that exhaustive, mutually exclusive probabilities sum to 1",
        "Estimate a probability from an experiment using relative frequency, and explain why more trials give a better estimate",
        "Work out expected frequencies and use them to judge whether a game or dice is fair",
        "Complete and read frequency trees",
        "Solve probability problems where the numbers of counters are unknown",
    ], "When P(D) is written as 3x, people solve for x and stop. Multiply back to get P(D) before using it, and pool all the trials when asked for the best estimate."),

    ("maths_edx:P6-8", &[
        "List outcomes systematically and use sample space grids to find probabilities of combined events",
        "Complete Venn diagrams from the intersection outwards and read set notation (∩, ∪, complement)",
        "Draw and use tree diagrams for independent and dependent events, multiplying along branches and adding paths",
        "Use 1 − P(none) for 'at least one' questions",
        "Form and solve an equation or quadratic when the number of counters is unknown, and state the assumptions behind a calculation",
    ], "Without replacement, the second-stage fractions must drop in both numerator and denominator. People who keep 5/8 × 5/8 lose every mark on the question."),

    ("maths_edx:P9", &[
        "Find a conditional probability from a two-way table, dividing by the group you are given",
        "Read conditional probabilities from Venn diagrams, using counts or probabilities in the regions",
        "Use tree diagrams backwards: given the outcome, find the probability of the first stage",
        "Turn percentages into expected frequencies out of 1000 to find and interpret conditional probabilities",
        "Use P(A | B) = P(A ∩ B) ÷ P(B), and compare P(A | B) with P(A) to test independence",
    ], "The denominator is the group after 'given that', not the grand total. Dividing by everyone is the mark people drop most."),

    ("maths_edx:S1", &[
        "Explain the difference between a population, a census and a sample, and why samples are used",
        "Say why a sample may be biased, in context, and how to make it random and representative",
        "Estimate population totals by scaling up a proportion or a mean from a sample, stating the assumption",
        "Use capture–recapture to estimate a population size, and explain how broken assumptions affect the estimate",
        "Work out a stratified sample in proportion to group sizes",
    ], "\"The sample is biased\" with no reason scores nothing. Name who is over- or under-represented and why, in the context of the question."),

    ("maths_edx:S2-4", &[
        "Choose and draw the right chart for the data: pie charts, pictograms, line charts and time series",
        "Find the mean, median, mode and range from frequency tables, and estimate the mean of grouped data using midpoints",
        "Draw and read histograms with unequal class widths using frequency density, including parts of bars",
        "Draw cumulative frequency graphs and read off the median, quartiles and numbers above or below a value",
        "Find quartiles and the IQR, identify outliers, draw box plots and compare two distributions in context",
    ], "With unequal class widths the histogram's height is frequency density, not frequency. Comparisons must quote an average and a spread with their values and say what they mean in context."),

    ("maths_edx:S5-6", &[
        "Describe a population using a suitable average and measure of spread, choosing the median and IQR when there are extreme values",
        "Plot scatter graphs and describe the type and strength of correlation in context",
        "Draw a line of best fit through the mean point, ignoring outliers, and use it to make predictions",
        "Explain why interpolation is fairly reliable but extrapolation is not",
        "Explain why correlation does not prove causation, naming a likely hidden factor, and interpret a gradient in context",
    ], "Writing \"positive correlation\" without saying what it means in context, and treating correlation as proof that one thing causes the other, are the marks most often dropped."),

    // ---------- Maths (AQA GCSE Mathematics (8300) Higher) ----------
    ("maths_aqa:N1-3", &[
        "Order integers, decimals and fractions, including negatives, and use =, ≠, <, >, ≤ and ≥ correctly",
        "Add, subtract, multiply and divide integers, decimals, fractions and mixed numbers, positive and negative, by written methods",
        "Use a given multiplication fact and place value to write down related results",
        "Apply the order of operations, including powers, roots and reciprocals, and treat fraction lines and root signs as brackets",
        "Use inverse operations to check answers, work backwards and cancel before calculating",
    ], "Mixed numbers multiplied or divided without first becoming improper fractions, and × and ÷ not worked left to right, cost the most marks. On a \"show that\" every fraction step must be written."),

    ("maths_aqa:N4-5", &[
        "Write a number as a product of its prime factors in index form, and use it to test for squares, cubes and divisibility",
        "Find the HCF and LCM of two or three numbers, including from given prime factorisations",
        "Recognise HCF and LCM problems in context and finish the answer in context",
        "List outcomes systematically so none are missed or repeated",
        "Use the product rule to count arrangements, handling restrictions first and dividing when order does not matter",
    ], "Prime factors written as a list instead of a product, and HCF and LCM mixed up, lose the easiest marks. In counting questions, fill the restricted position first."),

    ("maths_aqa:N6-7", &[
        "Recall squares, cubes and powers of 2, 3, 4 and 5, and the roots that go with them",
        "Estimate powers and roots of any positive number by trapping them between known powers",
        "Use the index laws with positive, zero, negative and fractional indices, with numbers and letters",
        "Evaluate expressions like a^(−m/n) without a calculator: reciprocal, root, then power",
        "Write numbers as powers of a given base and solve equations such as 9^x = 27",
    ], "A negative index means a reciprocal, not a negative number, and a fractional index means a root: 64^(1/2) is 8, not 32. Take the root before the power."),

    ("maths_aqa:N8", &[
        "Calculate exactly with fractions, surds and multiples of π, never rounding when an exact answer is asked for",
        "Simplify surds by taking out the largest square factor, and add, subtract, multiply and divide them",
        "Expand brackets containing surds, including squared brackets and the difference of two squares",
        "Rationalise denominators of the form √a, b√a and a ± √b",
        "Give lengths, areas and volumes exactly in terms of π and in surd form",
    ], "When the question says \"exact\" or \"in terms of π\", a rounded decimal loses the final mark. When rationalising, multiply the top by the same thing as the bottom."),

    ("maths_aqa:N9", &[
        "Convert between ordinary numbers and standard form A × 10ⁿ with 1 ≤ A < 10, for large and small numbers",
        "Order numbers given in standard form, comparing powers first",
        "Multiply, divide, add and subtract in standard form without a calculator, adjusting A back into range",
        "Enter standard form correctly on a calculator and write the display properly",
        "Solve problems in context with standard form, including how many times bigger and per-unit questions",
    ], "Answers like 24 × 10³ are not in standard form, and adding numbers is not done by adding powers. Remember that making A smaller makes the power bigger."),

    ("maths_aqa:N10-12", &[
        "Convert between terminating decimals and fractions, and decide from the denominator's prime factors whether a fraction terminates",
        "Write fractions as recurring decimals using dot notation",
        "Prove algebraically that a recurring decimal equals a given fraction, including when the repeat starts after the first decimal place",
        "Move between ratios and fractions of a whole, and combine fractions in multi-step ratio problems",
        "Use fractions, percentages and multipliers as operators, including finding the whole from a given part",
    ], "In recurring-decimal proofs the two multiples of x must be written out with matching recurring tails before subtracting; writing 100x = 45.45 without the dots or \"…\" loses the method mark."),

    ("maths_aqa:N13-16", &[
        "Convert between metric units, including area, volume, time and compound units such as km/h to m/s",
        "Round to a given number of decimal places or significant figures and estimate by rounding to 1 significant figure",
        "Write error intervals for rounded and truncated values using inequality notation",
        "Find upper and lower bounds of sums, differences, products and quotients by choosing the right bound of each input",
        "Use bounds to give an answer to a suitable degree of accuracy, with a reason",
    ], "For the maximum of a − b or a ÷ b you must use the lower bound of b. Using upper bound with upper bound throughout is the commonest way to lose every mark on a bounds question."),

    ("maths_aqa:A1-3", &[
        "Write and read algebraic notation exactly, including powers, fractions as coefficients and brackets",
        "Substitute positive, negative and fractional values into expressions and scientific formulae without sign errors",
        "Tell apart expressions, equations, formulae, identities and inequalities, and terms and factors",
        "Use a counter-example to show a statement is not an identity",
        "Match coefficients in an identity to find unknown constants",
    ], "Substitute every negative value in brackets. Writing −3² instead of (−3)² turns +9 into −9 and costs the accuracy mark."),

    ("maths_aqa:A4", &[
        "Expand single, double and triple brackets and collect like terms, including expressions with surds",
        "Factorise fully by taking out the highest common factor, including a bracket as the common factor",
        "Factorise quadratics x² + bx + c and ax² + bx + c, and the difference of two squares",
        "Simplify expressions using the laws of indices, including powers of products",
        "Simplify, multiply, divide, add and subtract algebraic fractions by factorising first",
    ], "Only whole factors cancel in an algebraic fraction: factorise the top and bottom completely before cancelling, and bracket the second numerator when subtracting."),

    ("maths_aqa:A5-6", &[
        "Recall and use standard formulae such as the area of a circle, Pythagoras and the equations of motion",
        "Change the subject of a formula, including ones with fractions, powers and roots",
        "Rearrange when the new subject appears twice by collecting terms and factorising",
        "Show that two expressions are identical by expanding and simplifying one side",
        "Write algebraic proofs using n, 2n and 2n + 1, ending with a concluding sentence, and disprove statements with a counter-example",
    ], "When the subject appears twice, collect those terms on one side and factorise it out. In a proof, finish with a sentence saying why the result has the property — \"= 8n\" on its own often loses the final mark."),

    ("maths_aqa:A7", &[
        "Use function notation to find outputs, including f of an expression, and solve f(x) = k for the input",
        "Find an inverse function by writing y = f(x) and making x the subject, including when x appears twice",
        "Find composite functions such as fg(x), gf(x) and ff(x), applying the inner function first",
        "Solve equations involving composite and inverse functions, including ones that lead to quadratics",
        "Find unknown constants in a function from given input and output values",
    ], "In fg(x) the function g acts first: substitute the whole of g(x), in brackets, into f. Doing them in the wrong order gives gf(x) and scores nothing."),

    ("maths_aqa:A8-10", &[
        "Plot straight-line graphs from a table of values or from the intercepts, and find midpoints",
        "Find and interpret the gradient and y-intercept of a line, rearranging ax + by = c into y = mx + c first",
        "Find the equation of a line through two points or through one point with a given gradient",
        "Use gradients to identify and form parallel and perpendicular lines, including perpendicular bisectors",
        "Interpret the gradient of a real-life line as a rate with units and the intercept as a starting value",
    ], "Rearrange to y = mx + c before reading the gradient: 2y = 6x + 8 has gradient 3, not 6. For a perpendicular gradient, flip the fraction and change the sign."),

    ("maths_aqa:A11-12", &[
        "Read the roots, y-intercept and turning point of a quadratic from its graph, and find the roots by factorising",
        "Complete the square to find the turning point of a quadratic, including when the coefficient of x² is not 1 or is negative",
        "Sketch linear, quadratic, cubic, reciprocal and exponential graphs with their intercepts and asymptotes labelled",
        "Sketch y = sin x, y = cos x and y = tan x for angles of any size and use their symmetry to find every solution in a range",
        "Interpret turning points and intercepts in context, such as the maximum height of a projectile",
    ], "The turning point of (x − 3)² − 4 is (3, −4), not (−3, −4): the x-coordinate has the opposite sign to the number in the bracket. And a trig equation almost always has more than the one solution your calculator gives."),

    ("maths_aqa:A13", &[
        "Sketch y = f(x) + a and y = f(x + a) as translations, and describe them with a column vector",
        "Sketch y = −f(x) and y = f(−x) as reflections in the x-axis and the y-axis",
        "Find the image of a turning point, labelled point or asymptote under a transformation",
        "Complete the square to show a quadratic is a translation of y = x² and state the vector",
        "Apply translations and reflections to the graphs of sin x, cos x, tan x, 1/x and kˣ",
    ], "A change inside the bracket moves the graph the opposite way to how it looks: f(x + 3) is a translation 3 to the left, by the column vector (−3, 0)."),

    ("maths_aqa:A14-15", &[
        "Plot reciprocal, exponential and non-standard graphs and read approximate solutions from them",
        "Find the gradient of a straight section and interpret it as a speed, acceleration or rate with units",
        "Estimate the gradient of a curve at a point by drawing a tangent, and an average rate using a chord",
        "Work out the area under a velocity–time graph as the distance travelled, using strips for curves",
        "Say whether a strip estimate is an overestimate or underestimate, with a reason",
        "Find and use an exponential model y = abˣ for growth or decay in context",
    ], "On a velocity–time graph the gradient is the acceleration and the area underneath is the distance. Mixing the two up — or forgetting to convert minutes to hours — loses the marks."),

    ("maths_aqa:A16", &[
        "Recognise x² + y² = r² as a circle with centre the origin and read off its radius, in surd form when needed",
        "Write the equation of a circle centred at the origin that passes through a given point",
        "Decide whether a point lies on, inside or outside a circle",
        "Find the equation of the tangent at a given point using the radius gradient and the negative reciprocal",
        "Use a tangent's equation to find where it meets the axes and solve follow-on area problems",
    ], "The tangent is perpendicular to the radius, so its gradient is the negative reciprocal of the radius gradient — using the radius gradient itself, or forgetting to change the sign, loses most of the marks."),

    ("maths_aqa:A17", &[
        "Solve linear equations with the unknown on both sides, including brackets and negative coefficients",
        "Clear fractions by multiplying every term by the lowest common multiple, including algebraic denominators",
        "Recognise when an equation has no solution or is an identity",
        "Find approximate solutions from where two graphs cross, and confirm them algebraically",
        "Form a linear equation from a context such as angles or ages, solve it and answer the question asked",
    ], "When clearing fractions, multiply every term — including the whole numbers — and put brackets round each numerator, so that −(x − 2)/3 becomes −4(x − 2) = −4x + 8."),

    ("maths_aqa:A18", &[
        "Rearrange a quadratic equation to ax² + bx + c = 0, including ones with brackets or algebraic fractions",
        "Solve a quadratic by factorising, including when the coefficient of x² is not 1",
        "Solve by completing the square, giving exact answers in surd form",
        "Recall and use the quadratic formula, giving answers to the accuracy asked for",
        "Find approximate solutions from a graph, drawing a straight line to solve related equations",
        "Form a quadratic from a context, solve it and reject any impossible solutions",
    ], "Get zero on one side before you factorise: (x + 3)(x − 2) = 14 does not mean x + 3 = 14. And in the formula, keep the signs of a, b and c in brackets so that −b and −4ac come out right."),

    ("maths_aqa:A19", &[
        "Solve two linear simultaneous equations by elimination or substitution",
        "Form and solve simultaneous equations from a worded context, answering in context",
        "Solve a linear and a quadratic equation simultaneously by substitution, including a line and a circle",
        "Pair each x-value with its correct y-value using the linear equation",
        "Show that a line is a tangent to a curve from a repeated root",
        "Find approximate solutions from where two graphs intersect",
    ], "Squaring the bracket wrongly after substituting, e.g. writing (2x + 1)² as 4x² + 1, and then giving x-values and y-values that are not paired up."),

    ("maths_aqa:A20", &[
        "Show that an equation has a solution in an interval by a change of sign, and state the conclusion",
        "Rearrange an equation into the form x = g(x) exactly as asked",
        "Use an iterative formula with a starting value to find x₁, x₂, x₃ on a calculator",
        "Iterate to a solution to a given accuracy and confirm it by testing the bounds",
        "Recognise when an iteration converges or diverges, and find the equation it solves",
    ], "Working out f(a) and f(b) but never writing \"change of sign, so there is a solution between a and b\" — the conclusion is the mark."),

    ("maths_aqa:A21-22", &[
        "Translate a worded situation or diagram into an expression, formula or equation, then solve it and interpret the answer",
        "Form and solve two simultaneous equations from a context",
        "Solve linear inequalities, including double inequalities, and list integer solutions",
        "Solve quadratic inequalities by finding critical values and sketching the parabola",
        "Show solution sets on a number line and in set notation",
        "Draw and interpret regions defined by inequalities in two variables",
    ], "Forgetting to reverse the inequality when dividing by a negative, and writing a 'greater than' quadratic inequality as one statement instead of two separate intervals outside the roots."),

    ("maths_aqa:A23-25", &[
        "Generate terms from a term-to-term rule or an nth-term formula",
        "Recognise square, cube and triangular numbers, arithmetic, geometric and Fibonacci-type sequences",
        "Find the nth term of a linear sequence and decide whether a number is a term",
        "Find the nth term of a quadratic sequence using second differences",
        "Work with geometric sequences whose ratio is a fraction or a surd, keeping answers exact",
        "Form and solve equations to find missing terms of Fibonacci-type sequences",
    ], "Forgetting to halve the second difference in a quadratic sequence, so 4, 11, 22, 37 starts 4n² instead of 2n²."),

    ("maths_aqa:R1-2", &[
        "Convert between metric units of length, mass, capacity and time, including hours and minutes",
        "Convert area and volume units by squaring or cubing the length factor",
        "Use speed, density and pressure, rearranging each formula and converting compound units",
        "Compare rates such as pay and unit prices to find the best buy",
        "Use scale factors, scale drawings and map scales, including areas on maps",
    ], "Converting m² to cm² by multiplying by 100 instead of 10 000, and treating 2.4 hours as 2 hours 40 minutes."),

    ("maths_aqa:R3-8", &[
        "Write one quantity as a fraction of another, including fractions greater than 1",
        "Simplify ratios, including mixed units, decimals and the form 1 : n",
        "Share an amount in a ratio when given the total, one share or the difference",
        "Move between ratios and fractions, and use equal ratios to scale recipes, convert currencies and compare mixtures",
        "Combine two ratios and solve problems where a ratio changes",
        "Link a fixed ratio to a straight-line graph through the origin",
    ], "Dividing by the wrong number of parts — treating a difference or one person's share as if it were the total — and confusing boys : girls = 3 : 5 with boys being 3/5 of the class."),

    ("maths_aqa:R9", &[
        "Convert between percentages, fractions and decimals, including percentages over 100%",
        "Increase or decrease an amount by a percentage using a single multiplier",
        "Work out a percentage change, profit or loss, always dividing by the original value",
        "Find the original value after a percentage change by dividing by the multiplier",
        "Calculate simple interest and work backwards from it to find a rate or a number of years",
        "Combine successive percentage changes by multiplying their multipliers",
    ], "Reverse percentages: after a 20% cut to £84, the original is 84 ÷ 0.8 = £105, not £84 plus 20% of £84. And percentage change always divides by the original, never the new value."),

    ("maths_aqa:R10", &[
        "Decide whether a situation is direct proportion, inverse proportion or neither",
        "Solve direct proportion problems with the unitary method or a scale factor, including best buys",
        "Solve inverse proportion problems by finding the total work first (worker-days, machine-hours)",
        "Use y = kx and y = k/x, finding k from a pair of values, a table or a graph",
        "Recognise the graphs: a straight line through the origin for direct, a y = k/x curve for inverse",
        "Handle problems with several quantities and round sensibly in context",
    ], "A straight line only shows direct proportion if it passes through the origin, and more workers means less time. Sense-check which way the answer should move before you calculate."),

    ("maths_aqa:R11", &[
        "Use speed = distance ÷ time, density = mass ÷ volume and pressure = force ÷ area, rearranging as needed",
        "Convert between hours and minutes correctly, and give times in the form asked for",
        "Work out average speed or mixture density from totals, never by averaging",
        "Convert compound units such as km/h to m/s, g/cm³ to kg/m³ and N/cm² to N/m²",
        "Solve problems with other rates: pay and overtime, unit pricing, flow rates and population density",
    ], "Minutes are not decimals of an hour: 1 h 45 min is 1.75 h, not 1.45 h. And average speed is total distance over total time, not the mean of the speeds."),

    ("maths_aqa:R12", &[
        "Compare lengths, areas and volumes of similar shapes using ratios a : b, a² : b² and a³ : b³",
        "Work back from an area or volume ratio to the length ratio by square- or cube-rooting",
        "Use linear, area and volume scale factors with models, maps and similar solids, including masses",
        "Explain why the trigonometric ratios are fixed for a given angle using similar triangles",
        "Write ratios of lengths, areas and volumes in simplest form or as 1 : n",
    ], "To go from an area ratio to a volume ratio, square-root to get lengths first and then cube. Squaring or cubing the wrong ratio is the mark most people drop."),

    ("maths_aqa:R13", &[
        "Turn a proportion statement, including squares, cubes and square roots, into an equation with a constant k",
        "Find k from one pair of values and write the formula out in full",
        "Use the formula to find either variable, rejecting negative roots where they make no sense",
        "Explain that X inversely proportional to Y is the same as X proportional to 1/Y",
        "Work out the effect on y of multiplying or changing x by a percentage, without finding k",
        "Recognise the graph shapes of direct and inverse proportion relationships",
    ], "Inverse square means y = k/x², so k = y × x²; people divide instead of multiply, or write y = kx² by mistake. Always write the full formula once you have found k."),

    ("maths_aqa:R14-15", &[
        "Interpret the gradient of a straight-line graph as a rate of change, in context and with units",
        "Recognise graphs of direct proportion (a line through the origin) and inverse proportion (a y = k/x curve)",
        "Work out an average rate of change as the gradient of a chord, from a graph, a table or an equation",
        "Estimate an instantaneous rate of change by drawing a tangent and finding its gradient",
        "Estimate a rate at a point from an equation using chords over smaller and smaller intervals",
    ], "A rate at one moment needs a tangent, not a chord, and its gradient must be read using the axis scales and stated with units in context, such as metres per second."),

    ("maths_aqa:R16", &[
        "Use a multiplier raised to a power to find the result of compound interest, depreciation, growth or decay",
        "Work backwards from a final amount by dividing by the multiplier to the power n",
        "Find the rate from start and end values, and find the time needed by trial and improvement",
        "Compare simple and compound interest and choose the better option with evidence",
        "Generate and interpret terms of an iterative process such as uₙ₊₁ = 1.2uₙ − 500",
    ], "A 15% loss is × 0.85 applied every year, not 15% of the original taken off each time. In \"how many years\" questions, write down the values either side of the target — the bare number earns little."),

    ("maths_aqa:G1-2", &[
        "Use the correct words and labels: vertex, parallel, perpendicular, regular, angle ABC at the middle letter, side a opposite angle A",
        "Construct a perpendicular bisector, a perpendicular to a line from or at a point, and an angle bisector with ruler and compasses only",
        "Build 60°, 30°, 90° and 45° angles and triangles from their sides",
        "Draw the locus for a rule and shade the region that satisfies several rules at once, to scale",
        "Explain why the perpendicular distance is the shortest distance from a point to a line",
    ], "The construction arcs are the method mark: a neat line with the arcs rubbed out scores almost nothing. Keep the compasses at the same width for both ends."),

    ("maths_aqa:G3-4", &[
        "Use angles on a straight line, around a point and vertically opposite, giving the reason in standard words",
        "Find alternate, corresponding and co-interior angles on parallel lines",
        "Derive and use the angle sum of a triangle, the exterior angle of a triangle and the angle sum of any polygon",
        "Find interior and exterior angles of regular polygons and the number of sides from an angle",
        "State and derive the properties of squares, rectangles, rhombuses, parallelograms, kites and trapeziums",
    ], "The reason marks are separate from the angle marks: \"Z angles\" or \"because they are parallel\" scores nothing — write \"alternate angles are equal\". For regular polygons, 360 ÷ n is the exterior angle, not the interior."),

    ("maths_aqa:G5-6", &[
        "Know the congruence conditions SSS, SAS, ASA and RHS, and why AAA and SSA do not prove congruence",
        "Prove two triangles congruent with three matching facts, each with a reason",
        "Use corresponding sides and angles of congruent triangles to prove lengths or angles equal",
        "Combine angle facts, similarity and quadrilateral properties to derive results such as equal base angles of an isosceles triangle",
        "Follow and write a proof of Pythagoras' theorem and other short geometric proofs",
    ], "Each of the three pairs needs its reason — \"(given)\", \"(radii)\", \"(common)\". Then finish: if you were asked to prove a side equal, stopping at \"so the triangles are congruent\" drops the last mark."),

    ("maths_aqa:G7-8", &[
        "Reflect, rotate, translate and enlarge shapes on coordinate axes, using coordinate rules and tracing paper",
        "Describe fully a single transformation: mirror line; centre, angle and direction; column vector; scale factor and centre",
        "Enlarge by fractional and negative scale factors, and find the centre and scale factor from an object and image",
        "Combine reflections, rotations and translations and describe the single equivalent transformation",
        "Identify invariant points and lines, and say what a transformation keeps the same",
    ], "\"Describe fully\" means every detail: a rotation needs its centre, angle and direction; an enlargement needs its scale factor and centre. Two transformations in reply to \"a single transformation\" scores nothing."),

    ("maths_aqa:G9-10", &[
        "Name the parts of a circle: centre, radius, chord, diameter, circumference, tangent, arc, sector and segment",
        "Spot and apply the eight circle theorems, including the alternate segment theorem, with the reason in standard words",
        "Combine circle theorems with isosceles triangles, angle sums and parallel lines in multi-step problems",
        "Use the tangent–radius right angle and the perpendicular bisector of a chord to find lengths",
        "Prove the circle theorems and use them to prove related results",
    ], "Every step needs its theorem in the standard words — \"angle at the centre is twice the angle at the circumference\", not \"double rule\". And two radii make an isosceles triangle: that step is the one most often missing."),

    ("maths_aqa:G11-15", &[
        "Find midpoints, lengths and gradients from coordinates and use them to find missing vertices or prove what shape a set of points makes",
        "Count faces, edges and vertices of prisms, pyramids and curved solids, and check with F + V − E = 2",
        "Draw plans and elevations from a 3D drawing, and interpret them to rebuild a solid or find its volume",
        "Convert units of length, area, volume, capacity, mass and time, squaring or cubing the length factor for area and volume",
        "Use map scales and three-figure bearings, including back bearings and angle facts with parallel north lines",
    ], "Area and volume conversions: 1 m² is 10 000 cm² and 1 m³ is 1 000 000 cm³, and a map scale must be squared before it is used on an area."),

    ("maths_aqa:G16-18", &[
        "Find areas of triangles, parallelograms, trapezia, circles and composite shapes, and perimeters including arcs",
        "Find the volume and surface area of prisms and cylinders, giving answers in terms of π when asked",
        "Use given formulae for cones, spheres and pyramids, finding slant or perpendicular height with Pythagoras when needed",
        "Find volumes of frustums and composite solids, using similarity for the part removed",
        "Calculate arc lengths, sector areas and segment areas, and work backwards to a missing angle or radius",
    ], "Using the slant height where the perpendicular height is needed (or the other way round) in cone and pyramid questions, and leaving the two radii out of a sector's perimeter."),

    ("maths_aqa:G19", &[
        "Prove two triangles congruent using SSS, SAS, ASA or RHS, giving a reason for every statement",
        "Show that two triangles are similar and match their corresponding sides using the equal angles",
        "Find missing lengths in similar shapes, including nested and hourglass triangles",
        "Use area scale factor k² and volume scale factor k³, including for surface area, capacity and mass",
        "Work backwards from an area or volume ratio to the length scale factor by square- or cube-rooting",
    ], "Using the length scale factor on an area or volume: if lengths are multiplied by k, areas are multiplied by k² and volumes by k³, so root first when you are given areas or volumes."),

    ("maths_aqa:G20-21", &[
        "Use Pythagoras' theorem to find any side of a right-angled triangle, leaving surds exact on the non-calculator paper",
        "Choose sin, cos or tan to find a missing side or angle, including angles of elevation and depression",
        "Split isosceles and other triangles with a perpendicular to create right-angled triangles",
        "Find lengths and angles in 3D solids, including the angle between a line and a plane",
        "Recall the exact values of sin and cos for 0°, 30°, 45°, 60° and 90°, and tan for 0°, 30°, 45° and 60°, and use them without a calculator",
    ], "In 3D, taking the angle to an edge instead of to the line's projection on the base, and rounding the base diagonal before using it in the second triangle."),

    ("maths_aqa:G22-23", &[
        "Label any triangle with each side opposite its matching angle and choose the sine rule, cosine rule or area formula from what is known",
        "Use the sine rule to find sides and angles, giving both possible angles when the obtuse one is also valid",
        "Use the cosine rule to find a side from two sides and the included angle, or an angle from three sides",
        "Use Area = ½ab sin C to find an area, or work backwards to a missing side or angle",
        "Solve multi-step, bearings and 3D problems by building the right triangle first, keeping full accuracy between steps",
    ], "Pairing a side with the wrong angle in the sine rule, and using an angle that is not between the two sides in the cosine rule or ½ab sin C."),

    ("maths_aqa:G24-25", &[
        "Describe translations with column vectors and add, subtract and scale column vectors, finding magnitudes with Pythagoras",
        "Write any vector on a diagram as a route through known vectors, using AB = b − a",
        "Find the position of a midpoint or a point dividing a line in a given ratio",
        "Prove lines parallel by showing one vector is a multiple of the other, and points collinear by adding a common point",
        "Find unknown scalars by equating coefficients, including where two lines meet",
    ], "Getting the direction wrong (AB is b − a, not a − b), and stopping a collinear proof at \"parallel\" without stating the common point."),

    ("maths_aqa:P1-5", &[
        "Find probabilities of equally likely outcomes and use the fact that exhaustive, mutually exclusive probabilities sum to 1",
        "Estimate a probability from an experiment using relative frequency, and explain why more trials give a better estimate",
        "Work out expected frequencies and use them to judge whether a game or dice is fair",
        "Complete and read frequency trees",
        "Solve probability problems where the numbers of counters are unknown",
    ], "When P(D) is written as 3x, people solve for x and stop. Multiply back to get P(D) before using it, and pool all the trials when asked for the best estimate."),

    ("maths_aqa:P6-8", &[
        "List outcomes systematically and use sample space grids to find probabilities of combined events",
        "Complete Venn diagrams from the intersection outwards and read set notation (∩, ∪, complement)",
        "Draw and use tree diagrams for independent and dependent events, multiplying along branches and adding paths",
        "Use 1 − P(none) for 'at least one' questions",
        "Form and solve an equation or quadratic when the number of counters is unknown, and state the assumptions behind a calculation",
    ], "Without replacement, the second-stage fractions must drop in both numerator and denominator. People who keep 5/8 × 5/8 lose every mark on the question."),

    ("maths_aqa:P9", &[
        "Find a conditional probability from a two-way table, dividing by the group you are given",
        "Read conditional probabilities from Venn diagrams, using counts or probabilities in the regions",
        "Use tree diagrams backwards: given the outcome, find the probability of the first stage",
        "Turn percentages into expected frequencies out of 1000 to find and interpret conditional probabilities",
        "Use P(A | B) = P(A ∩ B) ÷ P(B), and compare P(A | B) with P(A) to test independence",
    ], "The denominator is the group after 'given that', not the grand total. Dividing by everyone is the mark people drop most."),

    ("maths_aqa:S1", &[
        "Explain the difference between a population, a census and a sample, and why samples are used",
        "Say why a sample may be biased, in context, and how to make it random and representative",
        "Estimate population totals by scaling up a proportion or a mean from a sample, stating the assumption",
        "Use capture–recapture to estimate a population size, and explain how broken assumptions affect the estimate",
        "Work out a stratified sample in proportion to group sizes",
    ], "\"The sample is biased\" with no reason scores nothing. Name who is over- or under-represented and why, in the context of the question."),

    ("maths_aqa:S2-4", &[
        "Choose and draw the right chart for the data: pie charts, pictograms, line charts and time series",
        "Find the mean, median, mode and range from frequency tables, and estimate the mean of grouped data using midpoints",
        "Draw and read histograms with unequal class widths using frequency density, including parts of bars",
        "Draw cumulative frequency graphs and read off the median, quartiles and numbers above or below a value",
        "Find quartiles and the IQR, identify outliers, draw box plots and compare two distributions in context",
    ], "With unequal class widths the histogram's height is frequency density, not frequency. Comparisons must quote an average and a spread with their values and say what they mean in context."),

    ("maths_aqa:S5-6", &[
        "Describe a population using a suitable average and measure of spread, choosing the median and IQR when there are extreme values",
        "Plot scatter graphs and describe the type and strength of correlation in context",
        "Draw a line of best fit through the mean point, ignoring outliers, and use it to make predictions",
        "Explain why interpolation is fairly reliable but extrapolation is not",
        "Explain why correlation does not prove causation, naming a likely hidden factor, and interpret a gradient in context",
    ], "Writing \"positive correlation\" without saying what it means in context, and treating correlation as proof that one thing causes the other, are the marks most often dropped."),

    // ---------- Maths (OCR GCSE Mathematics (J560) Higher) ----------
    ("maths_ocr:N1-3", &[
        "Order integers, decimals and fractions, including negatives, and use =, ≠, <, >, ≤ and ≥ correctly",
        "Add, subtract, multiply and divide integers, decimals, fractions and mixed numbers, positive and negative, by written methods",
        "Use a given multiplication fact and place value to write down related results",
        "Apply the order of operations, including powers, roots and reciprocals, and treat fraction lines and root signs as brackets",
        "Use inverse operations to check answers, work backwards and cancel before calculating",
    ], "Mixed numbers multiplied or divided without first becoming improper fractions, and × and ÷ not worked left to right, cost the most marks. On a \"show that\" every fraction step must be written."),

    ("maths_ocr:N4-5", &[
        "Write a number as a product of its prime factors in index form, and use it to test for squares, cubes and divisibility",
        "Find the HCF and LCM of two or three numbers, including from given prime factorisations",
        "Recognise HCF and LCM problems in context and finish the answer in context",
        "List outcomes systematically so none are missed or repeated",
        "Use the product rule to count arrangements, handling restrictions first and dividing when order does not matter",
    ], "Prime factors written as a list instead of a product, and HCF and LCM mixed up, lose the easiest marks. In counting questions, fill the restricted position first."),

    ("maths_ocr:N6-7", &[
        "Recall squares, cubes and powers of 2, 3, 4 and 5, and the roots that go with them",
        "Estimate powers and roots of any positive number by trapping them between known powers",
        "Use the index laws with positive, zero, negative and fractional indices, with numbers and letters",
        "Evaluate expressions like a^(−m/n) without a calculator: reciprocal, root, then power",
        "Write numbers as powers of a given base and solve equations such as 9^x = 27",
    ], "A negative index means a reciprocal, not a negative number, and a fractional index means a root: 64^(1/2) is 8, not 32. Take the root before the power."),

    ("maths_ocr:N8", &[
        "Calculate exactly with fractions, surds and multiples of π, never rounding when an exact answer is asked for",
        "Simplify surds by taking out the largest square factor, and add, subtract, multiply and divide them",
        "Expand brackets containing surds, including squared brackets and the difference of two squares",
        "Rationalise denominators of the form √a, b√a and a ± √b",
        "Give lengths, areas and volumes exactly in terms of π and in surd form",
    ], "When the question says \"exact\" or \"in terms of π\", a rounded decimal loses the final mark. When rationalising, multiply the top by the same thing as the bottom."),

    ("maths_ocr:N9", &[
        "Convert between ordinary numbers and standard form A × 10ⁿ with 1 ≤ A < 10, for large and small numbers",
        "Order numbers given in standard form, comparing powers first",
        "Multiply, divide, add and subtract in standard form without a calculator, adjusting A back into range",
        "Enter standard form correctly on a calculator and write the display properly",
        "Solve problems in context with standard form, including how many times bigger and per-unit questions",
    ], "Answers like 24 × 10³ are not in standard form, and adding numbers is not done by adding powers. Remember that making A smaller makes the power bigger."),

    ("maths_ocr:N10-12", &[
        "Convert between terminating decimals and fractions, and decide from the denominator's prime factors whether a fraction terminates",
        "Write fractions as recurring decimals using dot notation",
        "Prove algebraically that a recurring decimal equals a given fraction, including when the repeat starts after the first decimal place",
        "Move between ratios and fractions of a whole, and combine fractions in multi-step ratio problems",
        "Use fractions, percentages and multipliers as operators, including finding the whole from a given part",
    ], "In recurring-decimal proofs the two multiples of x must be written out with matching recurring tails before subtracting; writing 100x = 45.45 without the dots or \"…\" loses the method mark."),

    ("maths_ocr:N13-16", &[
        "Convert between metric units, including area, volume, time and compound units such as km/h to m/s",
        "Round to a given number of decimal places or significant figures and estimate by rounding to 1 significant figure",
        "Write error intervals for rounded and truncated values using inequality notation",
        "Find upper and lower bounds of sums, differences, products and quotients by choosing the right bound of each input",
        "Use bounds to give an answer to a suitable degree of accuracy, with a reason",
    ], "For the maximum of a − b or a ÷ b you must use the lower bound of b. Using upper bound with upper bound throughout is the commonest way to lose every mark on a bounds question."),

    ("maths_ocr:A1-3", &[
        "Write and read algebraic notation exactly, including powers, fractions as coefficients and brackets",
        "Substitute positive, negative and fractional values into expressions and scientific formulae without sign errors",
        "Tell apart expressions, equations, formulae, identities and inequalities, and terms and factors",
        "Use a counter-example to show a statement is not an identity",
        "Match coefficients in an identity to find unknown constants",
    ], "Substitute every negative value in brackets. Writing −3² instead of (−3)² turns +9 into −9 and costs the accuracy mark."),

    ("maths_ocr:A4", &[
        "Expand single, double and triple brackets and collect like terms, including expressions with surds",
        "Factorise fully by taking out the highest common factor, including a bracket as the common factor",
        "Factorise quadratics x² + bx + c and ax² + bx + c, and the difference of two squares",
        "Simplify expressions using the laws of indices, including powers of products",
        "Simplify, multiply, divide, add and subtract algebraic fractions by factorising first",
    ], "Only whole factors cancel in an algebraic fraction: factorise the top and bottom completely before cancelling, and bracket the second numerator when subtracting."),

    ("maths_ocr:A5-6", &[
        "Recall and use standard formulae such as the area of a circle, Pythagoras and the equations of motion",
        "Change the subject of a formula, including ones with fractions, powers and roots",
        "Rearrange when the new subject appears twice by collecting terms and factorising",
        "Show that two expressions are identical by expanding and simplifying one side",
        "Write algebraic proofs using n, 2n and 2n + 1, ending with a concluding sentence, and disprove statements with a counter-example",
    ], "When the subject appears twice, collect those terms on one side and factorise it out. In a proof, finish with a sentence saying why the result has the property — \"= 8n\" on its own often loses the final mark."),

    ("maths_ocr:A7", &[
        "Use function notation to find outputs, including f of an expression, and solve f(x) = k for the input",
        "Find an inverse function by writing y = f(x) and making x the subject, including when x appears twice",
        "Find composite functions such as fg(x), gf(x) and ff(x), applying the inner function first",
        "Solve equations involving composite and inverse functions, including ones that lead to quadratics",
        "Find unknown constants in a function from given input and output values",
    ], "In fg(x) the function g acts first: substitute the whole of g(x), in brackets, into f. Doing them in the wrong order gives gf(x) and scores nothing."),

    ("maths_ocr:A8-10", &[
        "Plot straight-line graphs from a table of values or from the intercepts, and find midpoints",
        "Find and interpret the gradient and y-intercept of a line, rearranging ax + by = c into y = mx + c first",
        "Find the equation of a line through two points or through one point with a given gradient",
        "Use gradients to identify and form parallel and perpendicular lines, including perpendicular bisectors",
        "Interpret the gradient of a real-life line as a rate with units and the intercept as a starting value",
    ], "Rearrange to y = mx + c before reading the gradient: 2y = 6x + 8 has gradient 3, not 6. For a perpendicular gradient, flip the fraction and change the sign."),

    ("maths_ocr:A11-12", &[
        "Read the roots, y-intercept and turning point of a quadratic from its graph, and find the roots by factorising",
        "Complete the square to find the turning point of a quadratic, including when the coefficient of x² is not 1 or is negative",
        "Sketch linear, quadratic, cubic, reciprocal and exponential graphs with their intercepts and asymptotes labelled",
        "Sketch y = sin x, y = cos x and y = tan x for angles of any size and use their symmetry to find every solution in a range",
        "Interpret turning points and intercepts in context, such as the maximum height of a projectile",
    ], "The turning point of (x − 3)² − 4 is (3, −4), not (−3, −4): the x-coordinate has the opposite sign to the number in the bracket. And a trig equation almost always has more than the one solution your calculator gives."),

    ("maths_ocr:A13", &[
        "Sketch y = f(x) + a and y = f(x + a) as translations, and describe them with a column vector",
        "Sketch y = −f(x) and y = f(−x) as reflections in the x-axis and the y-axis",
        "Find the image of a turning point, labelled point or asymptote under a transformation",
        "Complete the square to show a quadratic is a translation of y = x² and state the vector",
        "Apply translations and reflections to the graphs of sin x, cos x, tan x, 1/x and kˣ",
    ], "A change inside the bracket moves the graph the opposite way to how it looks: f(x + 3) is a translation 3 to the left, by the column vector (−3, 0)."),

    ("maths_ocr:A14-15", &[
        "Plot reciprocal, exponential and non-standard graphs and read approximate solutions from them",
        "Find the gradient of a straight section and interpret it as a speed, acceleration or rate with units",
        "Estimate the gradient of a curve at a point by drawing a tangent, and an average rate using a chord",
        "Work out the area under a velocity–time graph as the distance travelled, using strips for curves",
        "Say whether a strip estimate is an overestimate or underestimate, with a reason",
        "Find and use an exponential model y = abˣ for growth or decay in context",
    ], "On a velocity–time graph the gradient is the acceleration and the area underneath is the distance. Mixing the two up — or forgetting to convert minutes to hours — loses the marks."),

    ("maths_ocr:A16", &[
        "Recognise x² + y² = r² as a circle with centre the origin and read off its radius, in surd form when needed",
        "Write the equation of a circle centred at the origin that passes through a given point",
        "Decide whether a point lies on, inside or outside a circle",
        "Find the equation of the tangent at a given point using the radius gradient and the negative reciprocal",
        "Use a tangent's equation to find where it meets the axes and solve follow-on area problems",
    ], "The tangent is perpendicular to the radius, so its gradient is the negative reciprocal of the radius gradient — using the radius gradient itself, or forgetting to change the sign, loses most of the marks."),

    ("maths_ocr:A17", &[
        "Solve linear equations with the unknown on both sides, including brackets and negative coefficients",
        "Clear fractions by multiplying every term by the lowest common multiple, including algebraic denominators",
        "Recognise when an equation has no solution or is an identity",
        "Find approximate solutions from where two graphs cross, and confirm them algebraically",
        "Form a linear equation from a context such as angles or ages, solve it and answer the question asked",
    ], "When clearing fractions, multiply every term — including the whole numbers — and put brackets round each numerator, so that −(x − 2)/3 becomes −4(x − 2) = −4x + 8."),

    ("maths_ocr:A18", &[
        "Rearrange a quadratic equation to ax² + bx + c = 0, including ones with brackets or algebraic fractions",
        "Solve a quadratic by factorising, including when the coefficient of x² is not 1",
        "Solve by completing the square, giving exact answers in surd form",
        "Recall and use the quadratic formula, giving answers to the accuracy asked for",
        "Find approximate solutions from a graph, drawing a straight line to solve related equations",
        "Form a quadratic from a context, solve it and reject any impossible solutions",
    ], "Get zero on one side before you factorise: (x + 3)(x − 2) = 14 does not mean x + 3 = 14. And in the formula, keep the signs of a, b and c in brackets so that −b and −4ac come out right."),

    ("maths_ocr:A19", &[
        "Solve two linear simultaneous equations by elimination or substitution",
        "Form and solve simultaneous equations from a worded context, answering in context",
        "Solve a linear and a quadratic equation simultaneously by substitution, including a line and a circle",
        "Pair each x-value with its correct y-value using the linear equation",
        "Show that a line is a tangent to a curve from a repeated root",
        "Find approximate solutions from where two graphs intersect",
    ], "Squaring the bracket wrongly after substituting, e.g. writing (2x + 1)² as 4x² + 1, and then giving x-values and y-values that are not paired up."),

    ("maths_ocr:A20", &[
        "Show that an equation has a solution in an interval by a change of sign, and state the conclusion",
        "Rearrange an equation into the form x = g(x) exactly as asked",
        "Use an iterative formula with a starting value to find x₁, x₂, x₃ on a calculator",
        "Iterate to a solution to a given accuracy and confirm it by testing the bounds",
        "Recognise when an iteration converges or diverges, and find the equation it solves",
    ], "Working out f(a) and f(b) but never writing \"change of sign, so there is a solution between a and b\" — the conclusion is the mark."),

    ("maths_ocr:A21-22", &[
        "Translate a worded situation or diagram into an expression, formula or equation, then solve it and interpret the answer",
        "Form and solve two simultaneous equations from a context",
        "Solve linear inequalities, including double inequalities, and list integer solutions",
        "Solve quadratic inequalities by finding critical values and sketching the parabola",
        "Show solution sets on a number line and in set notation",
        "Draw and interpret regions defined by inequalities in two variables",
    ], "Forgetting to reverse the inequality when dividing by a negative, and writing a 'greater than' quadratic inequality as one statement instead of two separate intervals outside the roots."),

    ("maths_ocr:A23-25", &[
        "Generate terms from a term-to-term rule or an nth-term formula",
        "Recognise square, cube and triangular numbers, arithmetic, geometric and Fibonacci-type sequences",
        "Find the nth term of a linear sequence and decide whether a number is a term",
        "Find the nth term of a quadratic sequence using second differences",
        "Work with geometric sequences whose ratio is a fraction or a surd, keeping answers exact",
        "Form and solve equations to find missing terms of Fibonacci-type sequences",
    ], "Forgetting to halve the second difference in a quadratic sequence, so 4, 11, 22, 37 starts 4n² instead of 2n²."),

    ("maths_ocr:R1-2", &[
        "Convert between metric units of length, mass, capacity and time, including hours and minutes",
        "Convert area and volume units by squaring or cubing the length factor",
        "Use speed, density and pressure, rearranging each formula and converting compound units",
        "Compare rates such as pay and unit prices to find the best buy",
        "Use scale factors, scale drawings and map scales, including areas on maps",
    ], "Converting m² to cm² by multiplying by 100 instead of 10 000, and treating 2.4 hours as 2 hours 40 minutes."),

    ("maths_ocr:R3-8", &[
        "Write one quantity as a fraction of another, including fractions greater than 1",
        "Simplify ratios, including mixed units, decimals and the form 1 : n",
        "Share an amount in a ratio when given the total, one share or the difference",
        "Move between ratios and fractions, and use equal ratios to scale recipes, convert currencies and compare mixtures",
        "Combine two ratios and solve problems where a ratio changes",
        "Link a fixed ratio to a straight-line graph through the origin",
    ], "Dividing by the wrong number of parts — treating a difference or one person's share as if it were the total — and confusing boys : girls = 3 : 5 with boys being 3/5 of the class."),

    ("maths_ocr:R9", &[
        "Convert between percentages, fractions and decimals, including percentages over 100%",
        "Increase or decrease an amount by a percentage using a single multiplier",
        "Work out a percentage change, profit or loss, always dividing by the original value",
        "Find the original value after a percentage change by dividing by the multiplier",
        "Calculate simple interest and work backwards from it to find a rate or a number of years",
        "Combine successive percentage changes by multiplying their multipliers",
    ], "Reverse percentages: after a 20% cut to £84, the original is 84 ÷ 0.8 = £105, not £84 plus 20% of £84. And percentage change always divides by the original, never the new value."),

    ("maths_ocr:R10", &[
        "Decide whether a situation is direct proportion, inverse proportion or neither",
        "Solve direct proportion problems with the unitary method or a scale factor, including best buys",
        "Solve inverse proportion problems by finding the total work first (worker-days, machine-hours)",
        "Use y = kx and y = k/x, finding k from a pair of values, a table or a graph",
        "Recognise the graphs: a straight line through the origin for direct, a y = k/x curve for inverse",
        "Handle problems with several quantities and round sensibly in context",
    ], "A straight line only shows direct proportion if it passes through the origin, and more workers means less time. Sense-check which way the answer should move before you calculate."),

    ("maths_ocr:R11", &[
        "Use speed = distance ÷ time, density = mass ÷ volume and pressure = force ÷ area, rearranging as needed",
        "Convert between hours and minutes correctly, and give times in the form asked for",
        "Work out average speed or mixture density from totals, never by averaging",
        "Convert compound units such as km/h to m/s, g/cm³ to kg/m³ and N/cm² to N/m²",
        "Solve problems with other rates: pay and overtime, unit pricing, flow rates and population density",
    ], "Minutes are not decimals of an hour: 1 h 45 min is 1.75 h, not 1.45 h. And average speed is total distance over total time, not the mean of the speeds."),

    ("maths_ocr:R12", &[
        "Compare lengths, areas and volumes of similar shapes using ratios a : b, a² : b² and a³ : b³",
        "Work back from an area or volume ratio to the length ratio by square- or cube-rooting",
        "Use linear, area and volume scale factors with models, maps and similar solids, including masses",
        "Explain why the trigonometric ratios are fixed for a given angle using similar triangles",
        "Write ratios of lengths, areas and volumes in simplest form or as 1 : n",
    ], "To go from an area ratio to a volume ratio, square-root to get lengths first and then cube. Squaring or cubing the wrong ratio is the mark most people drop."),

    ("maths_ocr:R13", &[
        "Turn a proportion statement, including squares, cubes and square roots, into an equation with a constant k",
        "Find k from one pair of values and write the formula out in full",
        "Use the formula to find either variable, rejecting negative roots where they make no sense",
        "Explain that X inversely proportional to Y is the same as X proportional to 1/Y",
        "Work out the effect on y of multiplying or changing x by a percentage, without finding k",
        "Recognise the graph shapes of direct and inverse proportion relationships",
    ], "Inverse square means y = k/x², so k = y × x²; people divide instead of multiply, or write y = kx² by mistake. Always write the full formula once you have found k."),

    ("maths_ocr:R14-15", &[
        "Interpret the gradient of a straight-line graph as a rate of change, in context and with units",
        "Recognise graphs of direct proportion (a line through the origin) and inverse proportion (a y = k/x curve)",
        "Work out an average rate of change as the gradient of a chord, from a graph, a table or an equation",
        "Estimate an instantaneous rate of change by drawing a tangent and finding its gradient",
        "Estimate a rate at a point from an equation using chords over smaller and smaller intervals",
    ], "A rate at one moment needs a tangent, not a chord, and its gradient must be read using the axis scales and stated with units in context, such as metres per second."),

    ("maths_ocr:R16", &[
        "Use a multiplier raised to a power to find the result of compound interest, depreciation, growth or decay",
        "Work backwards from a final amount by dividing by the multiplier to the power n",
        "Find the rate from start and end values, and find the time needed by trial and improvement",
        "Compare simple and compound interest and choose the better option with evidence",
        "Generate and interpret terms of an iterative process such as uₙ₊₁ = 1.2uₙ − 500",
    ], "A 15% loss is × 0.85 applied every year, not 15% of the original taken off each time. In \"how many years\" questions, write down the values either side of the target — the bare number earns little."),

    ("maths_ocr:G1-2", &[
        "Use the correct words and labels: vertex, parallel, perpendicular, regular, angle ABC at the middle letter, side a opposite angle A",
        "Construct a perpendicular bisector, a perpendicular to a line from or at a point, and an angle bisector with ruler and compasses only",
        "Build 60°, 30°, 90° and 45° angles and triangles from their sides",
        "Draw the locus for a rule and shade the region that satisfies several rules at once, to scale",
        "Explain why the perpendicular distance is the shortest distance from a point to a line",
    ], "The construction arcs are the method mark: a neat line with the arcs rubbed out scores almost nothing. Keep the compasses at the same width for both ends."),

    ("maths_ocr:G3-4", &[
        "Use angles on a straight line, around a point and vertically opposite, giving the reason in standard words",
        "Find alternate, corresponding and co-interior angles on parallel lines",
        "Derive and use the angle sum of a triangle, the exterior angle of a triangle and the angle sum of any polygon",
        "Find interior and exterior angles of regular polygons and the number of sides from an angle",
        "State and derive the properties of squares, rectangles, rhombuses, parallelograms, kites and trapeziums",
    ], "The reason marks are separate from the angle marks: \"Z angles\" or \"because they are parallel\" scores nothing — write \"alternate angles are equal\". For regular polygons, 360 ÷ n is the exterior angle, not the interior."),

    ("maths_ocr:G5-6", &[
        "Know the congruence conditions SSS, SAS, ASA and RHS, and why AAA and SSA do not prove congruence",
        "Prove two triangles congruent with three matching facts, each with a reason",
        "Use corresponding sides and angles of congruent triangles to prove lengths or angles equal",
        "Combine angle facts, similarity and quadrilateral properties to derive results such as equal base angles of an isosceles triangle",
        "Follow and write a proof of Pythagoras' theorem and other short geometric proofs",
    ], "Each of the three pairs needs its reason — \"(given)\", \"(radii)\", \"(common)\". Then finish: if you were asked to prove a side equal, stopping at \"so the triangles are congruent\" drops the last mark."),

    ("maths_ocr:G7-8", &[
        "Reflect, rotate, translate and enlarge shapes on coordinate axes, using coordinate rules and tracing paper",
        "Describe fully a single transformation: mirror line; centre, angle and direction; column vector; scale factor and centre",
        "Enlarge by fractional and negative scale factors, and find the centre and scale factor from an object and image",
        "Combine reflections, rotations and translations and describe the single equivalent transformation",
        "Identify invariant points and lines, and say what a transformation keeps the same",
    ], "\"Describe fully\" means every detail: a rotation needs its centre, angle and direction; an enlargement needs its scale factor and centre. Two transformations in reply to \"a single transformation\" scores nothing."),

    ("maths_ocr:G9-10", &[
        "Name the parts of a circle: centre, radius, chord, diameter, circumference, tangent, arc, sector and segment",
        "Spot and apply the eight circle theorems, including the alternate segment theorem, with the reason in standard words",
        "Combine circle theorems with isosceles triangles, angle sums and parallel lines in multi-step problems",
        "Use the tangent–radius right angle and the perpendicular bisector of a chord to find lengths",
        "Prove the circle theorems and use them to prove related results",
    ], "Every step needs its theorem in the standard words — \"angle at the centre is twice the angle at the circumference\", not \"double rule\". And two radii make an isosceles triangle: that step is the one most often missing."),

    ("maths_ocr:G11-15", &[
        "Find midpoints, lengths and gradients from coordinates and use them to find missing vertices or prove what shape a set of points makes",
        "Count faces, edges and vertices of prisms, pyramids and curved solids, and check with F + V − E = 2",
        "Draw plans and elevations from a 3D drawing, and interpret them to rebuild a solid or find its volume",
        "Convert units of length, area, volume, capacity, mass and time, squaring or cubing the length factor for area and volume",
        "Use map scales and three-figure bearings, including back bearings and angle facts with parallel north lines",
    ], "Area and volume conversions: 1 m² is 10 000 cm² and 1 m³ is 1 000 000 cm³, and a map scale must be squared before it is used on an area."),

    ("maths_ocr:G16-18", &[
        "Find areas of triangles, parallelograms, trapezia, circles and composite shapes, and perimeters including arcs",
        "Find the volume and surface area of prisms and cylinders, giving answers in terms of π when asked",
        "Use given formulae for cones, spheres and pyramids, finding slant or perpendicular height with Pythagoras when needed",
        "Find volumes of frustums and composite solids, using similarity for the part removed",
        "Calculate arc lengths, sector areas and segment areas, and work backwards to a missing angle or radius",
    ], "Using the slant height where the perpendicular height is needed (or the other way round) in cone and pyramid questions, and leaving the two radii out of a sector's perimeter."),

    ("maths_ocr:G19", &[
        "Prove two triangles congruent using SSS, SAS, ASA or RHS, giving a reason for every statement",
        "Show that two triangles are similar and match their corresponding sides using the equal angles",
        "Find missing lengths in similar shapes, including nested and hourglass triangles",
        "Use area scale factor k² and volume scale factor k³, including for surface area, capacity and mass",
        "Work backwards from an area or volume ratio to the length scale factor by square- or cube-rooting",
    ], "Using the length scale factor on an area or volume: if lengths are multiplied by k, areas are multiplied by k² and volumes by k³, so root first when you are given areas or volumes."),

    ("maths_ocr:G20-21", &[
        "Use Pythagoras' theorem to find any side of a right-angled triangle, leaving surds exact on the non-calculator paper",
        "Choose sin, cos or tan to find a missing side or angle, including angles of elevation and depression",
        "Split isosceles and other triangles with a perpendicular to create right-angled triangles",
        "Find lengths and angles in 3D solids, including the angle between a line and a plane",
        "Recall the exact values of sin and cos for 0°, 30°, 45°, 60° and 90°, and tan for 0°, 30°, 45° and 60°, and use them without a calculator",
    ], "In 3D, taking the angle to an edge instead of to the line's projection on the base, and rounding the base diagonal before using it in the second triangle."),

    ("maths_ocr:G22-23", &[
        "Label any triangle with each side opposite its matching angle and choose the sine rule, cosine rule or area formula from what is known",
        "Use the sine rule to find sides and angles, giving both possible angles when the obtuse one is also valid",
        "Use the cosine rule to find a side from two sides and the included angle, or an angle from three sides",
        "Use Area = ½ab sin C to find an area, or work backwards to a missing side or angle",
        "Solve multi-step, bearings and 3D problems by building the right triangle first, keeping full accuracy between steps",
    ], "Pairing a side with the wrong angle in the sine rule, and using an angle that is not between the two sides in the cosine rule or ½ab sin C."),

    ("maths_ocr:G24-25", &[
        "Describe translations with column vectors and add, subtract and scale column vectors, finding magnitudes with Pythagoras",
        "Write any vector on a diagram as a route through known vectors, using AB = b − a",
        "Find the position of a midpoint or a point dividing a line in a given ratio",
        "Prove lines parallel by showing one vector is a multiple of the other, and points collinear by adding a common point",
        "Find unknown scalars by equating coefficients, including where two lines meet",
    ], "Getting the direction wrong (AB is b − a, not a − b), and stopping a collinear proof at \"parallel\" without stating the common point."),

    ("maths_ocr:P1-5", &[
        "Find probabilities of equally likely outcomes and use the fact that exhaustive, mutually exclusive probabilities sum to 1",
        "Estimate a probability from an experiment using relative frequency, and explain why more trials give a better estimate",
        "Work out expected frequencies and use them to judge whether a game or dice is fair",
        "Complete and read frequency trees",
        "Solve probability problems where the numbers of counters are unknown",
    ], "When P(D) is written as 3x, people solve for x and stop. Multiply back to get P(D) before using it, and pool all the trials when asked for the best estimate."),

    ("maths_ocr:P6-8", &[
        "List outcomes systematically and use sample space grids to find probabilities of combined events",
        "Complete Venn diagrams from the intersection outwards and read set notation (∩, ∪, complement)",
        "Draw and use tree diagrams for independent and dependent events, multiplying along branches and adding paths",
        "Use 1 − P(none) for 'at least one' questions",
        "Form and solve an equation or quadratic when the number of counters is unknown, and state the assumptions behind a calculation",
    ], "Without replacement, the second-stage fractions must drop in both numerator and denominator. People who keep 5/8 × 5/8 lose every mark on the question."),

    ("maths_ocr:P9", &[
        "Find a conditional probability from a two-way table, dividing by the group you are given",
        "Read conditional probabilities from Venn diagrams, using counts or probabilities in the regions",
        "Use tree diagrams backwards: given the outcome, find the probability of the first stage",
        "Turn percentages into expected frequencies out of 1000 to find and interpret conditional probabilities",
        "Use P(A | B) = P(A ∩ B) ÷ P(B), and compare P(A | B) with P(A) to test independence",
    ], "The denominator is the group after 'given that', not the grand total. Dividing by everyone is the mark people drop most."),

    ("maths_ocr:S1", &[
        "Explain the difference between a population, a census and a sample, and why samples are used",
        "Say why a sample may be biased, in context, and how to make it random and representative",
        "Estimate population totals by scaling up a proportion or a mean from a sample, stating the assumption",
        "Use capture–recapture to estimate a population size, and explain how broken assumptions affect the estimate",
        "Work out a stratified sample in proportion to group sizes",
    ], "\"The sample is biased\" with no reason scores nothing. Name who is over- or under-represented and why, in the context of the question."),

    ("maths_ocr:S2-4", &[
        "Choose and draw the right chart for the data: pie charts, pictograms, line charts and time series",
        "Find the mean, median, mode and range from frequency tables, and estimate the mean of grouped data using midpoints",
        "Draw and read histograms with unequal class widths using frequency density, including parts of bars",
        "Draw cumulative frequency graphs and read off the median, quartiles and numbers above or below a value",
        "Find quartiles and the IQR, identify outliers, draw box plots and compare two distributions in context",
    ], "With unequal class widths the histogram's height is frequency density, not frequency. Comparisons must quote an average and a spread with their values and say what they mean in context."),

    ("maths_ocr:S5-6", &[
        "Describe a population using a suitable average and measure of spread, choosing the median and IQR when there are extreme values",
        "Plot scatter graphs and describe the type and strength of correlation in context",
        "Draw a line of best fit through the mean point, ignoring outliers, and use it to make predictions",
        "Explain why interpolation is fairly reliable but extrapolation is not",
        "Explain why correlation does not prove causation, naming a likely hidden factor, and interpret a gradient in context",
    ], "Writing \"positive correlation\" without saying what it means in context, and treating correlation as proof that one thing causes the other, are the marks most often dropped."),

    // ---------- Biology (AQA GCSE Biology (8461) Higher) ----------
    ("bio_aqa:4.1.1", &[
        "Compare eukaryotic and prokaryotic cells, including plasmids and the DNA loop",
        "Link each sub-cellular structure in plant, animal and bacterial cells to its function",
        "Explain how sperm, nerve, muscle, root hair, xylem and phloem cells are adapted",
        "Use magnification = image size / real size with unit conversions and standard form",
        "Compare light and electron microscopes in terms of magnification and resolution",
        "Describe aseptic technique and calculate bacterial numbers and clear-zone areas (separate science only)",
    ], "In magnification sums, convert both sizes to the same unit before dividing. 1 mm is 1000 µm, and magnification has no unit."),

    ("bio_aqa:4.1.2", &[
        "Describe chromosomes, genes and how chromosomes are paired in body cells",
        "Describe the three stages of the cell cycle and why DNA and organelles are copied first",
        "Recognise growth, repair and replacement as situations where mitosis happens",
        "Calculate the time spent in mitosis from cell counts",
        "Describe the function of stem cells in embryos, adult bone marrow and plant meristems",
        "Evaluate the benefits, risks and ethical issues of stem cell treatments and therapeutic cloning",
    ], "Describe the cell cycle in AQA's three stages and end with two genetically identical cells. In stem cell questions, \"evaluate\" needs both sides and a conclusion."),

    ("bio_aqa:4.1.3", &[
        "Define diffusion, osmosis and active transport and explain how they differ",
        "Explain how concentration gradient, temperature and surface area affect the rate of diffusion",
        "Calculate surface area to volume ratios and use them to explain the need for exchange surfaces and transport systems",
        "Explain how the small intestine, lungs, gills, roots and leaves are adapted for exchange",
        "Carry out the osmosis practical, calculate percentage change in mass and interpret the graph",
    ], "An osmosis definition needs water, dilute to more concentrated solution, and a partially permeable membrane. Active transport needs both \"against the gradient\" and \"energy from respiration\"."),

    ("bio_aqa:4.2.1", &[
        "Define cell, tissue, organ and organ system using AQA's wording",
        "Put the levels of organisation in order from sub-cellular structure to organism",
        "Classify examples such as blood, the heart and the leaf into the correct level",
        "Explain how the tissues in an organ such as the stomach each contribute to its function",
        "Compare the sizes of cells, tissues, organs and systems using the same units and standard form",
    ], "An organ is a group of different tissues, not just a lot of cells. Blood is a tissue, the leaf is an organ, and those two catch people out most."),

    ("bio_aqa:4.2.2a", &[
        "Recall where amylase, proteases and lipases are made, where they work and their products",
        "Explain enzyme specificity using the active site and the lock and key model",
        "Explain the effect of temperature and pH on enzyme activity, using denaturing",
        "Explain how bile neutralises acid and emulsifies fat to speed up lipase",
        "Carry out the food tests and the amylase pH practical, and calculate rates as 1/time",
    ], "Enzymes are denatured, not killed: say the active site changes shape so the substrate no longer fits. Bile is not an enzyme; it emulsifies fat and neutralises acid."),

    ("bio_aqa:4.2.2b", &[
        "Trace blood through the double circulatory system, naming the chambers and the aorta, vena cava, pulmonary artery, pulmonary vein and coronary arteries",
        "Explain how the lungs are adapted for gas exchange, from trachea and bronchi to alveoli and their capillaries",
        "Describe the natural pacemaker in the right atrium and what artificial pacemakers do",
        "Explain how the structures of arteries, veins and capillaries suit their functions",
        "Calculate rates of blood flow and heart rate with the right units",
        "Identify red cells, white cells and platelets and explain how each is adapted, alongside the role of plasma",
    ], "The pulmonary artery carries deoxygenated blood and the pulmonary vein oxygenated blood. And arteries do not pump: their thick walls withstand high pressure."),

    ("bio_aqa:4.2.2c", &[
        "Explain how fatty deposits in the coronary arteries starve heart muscle of oxygen",
        "Evaluate stents, statins, replacement valves, transplants and artificial hearts, weighing benefits against risks",
        "Describe how health is affected by disease, diet, stress and life situations, and how different diseases interact",
        "Link named lifestyle risk factors to their diseases and discuss their human and financial costs",
        "Interpret disease data from tables, charts and scatter diagrams, judging sampling and correlation versus cause",
        "Distinguish benign from malignant tumours and name lifestyle and genetic risk factors for cancer",
    ], "A correlation between a risk factor and a disease does not prove cause on its own. Say a causal mechanism is needed, or that other factors could be involved."),

    ("bio_aqa:4.2.3", &[
        "Explain how the epidermis, palisade and spongy mesophyll, xylem, phloem, meristem and guard cells suit their jobs in the leaf and plant",
        "Explain how root hair cells take up water by osmosis and mineral ions by active transport",
        "Describe transpiration and translocation and compare xylem with phloem",
        "Explain how temperature, humidity, air movement and light intensity change the rate of transpiration",
        "Measure transpiration with a potometer and calculate rates, means and volumes from your readings",
    ], "Root hair cells take in water by osmosis but mineral ions by active transport. And humidity slows transpiration because the water vapour concentration gradient is less steep."),

    ("bio_aqa:4.3.1a", &[
        "Name the four types of pathogen and explain how bacteria and viruses make us ill",
        "Explain how pathogens spread by direct contact, water and air, and how each route can be blocked",
        "Give the pathogen, symptoms, spread and control for measles, HIV and TMV",
        "Do the same for Salmonella, gonorrhoea, rose black spot and malaria",
        "Explain why TMV and rose black spot reduce plant growth",
        "Interpret data on cases of a disease before and after a control measure",
    ], "Malaria is caused by a protist, and the mosquito is only the vector. For every control method, say which link in the chain of spread it breaks."),

    ("bio_aqa:4.3.1b", &[
        "Describe how the skin, nose, trachea and bronchi, and stomach stop pathogens getting in",
        "Explain how white blood cells defend the body by phagocytosis, antibody production and antitoxin production",
        "Explain how vaccination protects a person, and how immunising most of a population stops a pathogen spreading",
        "Explain what antibiotics and painkillers can and cannot do, including why antibiotics do not work on viruses",
        "Describe how a new drug is discovered and tested, from preclinical work to double blind trials and peer review",
    ], "Antibiotics kill bacteria only, never viruses, and painkillers kill nothing at all. Say it plainly whenever a question mentions a viral illness."),

    ("bio_aqa:4.3.2", &[
        "Describe how monoclonal antibodies are produced, from injecting a mouse to purifying the antibody",
        "Explain why lymphocytes are fused with tumour cells to make hybridoma cells",
        "Explain how monoclonal antibodies work in pregnancy tests, laboratory tests and research",
        "Explain how monoclonal antibodies deliver radioactive substances or drugs to cancer cells",
        "Evaluate the advantages, disadvantages and ethical issues of monoclonal antibodies",
    ], "Every \"explain how it works\" mark hangs on specificity: the antibody binds only to one antigen. Say that, then say what is attached and what it does."),

    ("bio_aqa:4.3.3", &[
        "Describe the symptoms, spread and control of tobacco mosaic virus, rose black spot and aphids",
        "List the signs used to detect plant disease and the ways to identify the pathogen (Higher tier)",
        "Explain how nitrate and magnesium deficiencies cause stunted growth and chlorosis",
        "Describe and explain the physical, chemical and mechanical defences of plants",
        "Link leaf damage to reduced photosynthesis and reduced growth",
    ], "Magnesium is for chlorophyll and nitrate is for protein. Swap them, or call black spot anything but a fungus, and the mark is gone."),

    ("bio_aqa:4.4.1", &[
        "Write the word equation for photosynthesis, recognise the symbols, and explain why it is endothermic",
        "Explain how light intensity, carbon dioxide, temperature and chlorophyll affect the rate, and calculate rates",
        "Read limiting-factor graphs, including two- and three-factor graphs and the inverse square law (Higher tier)",
        "Carry out and evaluate Required practical 6 (Trilogy 5) on light intensity and pondweed",
        "Use limiting factors to judge the cost-effectiveness of heat, light and CO2 in greenhouses (Higher tier)",
        "List the uses of glucose in plants and explain why nitrate is also needed",
    ], "On a plateau, \"another factor is limiting\" is the mark, and at Higher you must name it from the other lines. Doubling the lamp distance quarters the light, it does not halve it."),

    ("bio_aqa:4.4.2", &[
        "Describe respiration as a continuous exothermic reaction and list what organisms use the energy for",
        "Write the equations for aerobic respiration and for anaerobic respiration in muscles, plants and yeast",
        "Compare aerobic and anaerobic respiration: oxygen, products and energy transferred",
        "Explain the body's response to exercise, lactic acid and fatigue, and oxygen debt (Higher tier)",
        "Explain metabolism and the role of sugars, amino acids, fatty acids and glycerol",
    ], "Respiration transfers energy; it never \"produces\" it. Muscles make lactic acid only, while yeast and plants make ethanol and carbon dioxide."),

    ("bio_aqa:4.5.1", &[
        "Define homeostasis as regulating internal conditions to keep optimum conditions for function",
        "Explain why homeostasis matters for enzyme action and cell functions",
        "Name the conditions controlled in the human body: blood glucose, body temperature and water levels",
        "Describe the roles of receptors, coordination centres and effectors, and put the control chain in order",
        "Apply the control-system pattern to unfamiliar examples and data",
    ], "Effectors are only muscles or glands, and a receptor is a cell. Call the brain an effector, or define homeostasis as \"keeping things the same\", and the mark goes."),

    ("bio_aqa:4.5.2", &[
        "Describe the pathway from stimulus to response, naming receptor, CNS and effector",
        "Explain how each structure in a reflex arc, including the synapse and relay neurone, relates to its function and why reflexes are fast",
        "Plan the ruler-drop reaction-time practical, naming variables, calculating means and converting results to graphs",
        "Identify the cerebral cortex, cerebellum and medulla and explain why the brain is hard to investigate and treat (biology only)",
        "Explain accommodation, adaptation to dim light, and how lenses correct myopia and hyperopia (biology only)",
        "Explain how vasodilation, vasoconstriction, sweating and shivering control body temperature (biology only)",
    ], "Impulses are electrical along neurones but a chemical diffuses across the synapse, and reflexes skip the conscious brain, not the whole CNS. In the eye, the ciliary muscles contract while the suspensory ligaments slacken; ligaments never contract."),

    ("bio_aqa:4.5.3a", &[
        "Describe how the endocrine system works and compare it with the nervous system",
        "Identify the pituitary, thyroid, adrenal glands, pancreas, ovaries and testes on a diagram of the body",
        "Explain how insulin, and at Higher tier glucagon, control blood glucose by negative feedback",
        "Compare Type 1 and Type 2 diabetes and their treatments, and interpret glucose graphs",
        "Explain how the kidneys filter and selectively reabsorb, and how ADH controls water balance (biology only)",
        "Describe how dialysis works and evaluate it against a kidney transplant (biology only)",
    ], "Glucose, glycogen and glucagon get mixed up more than anything else on this topic. Insulin moves glucose into cells, and glucose is stored as glycogen in liver and muscle."),

    ("bio_aqa:4.5.3b", &[
        "Describe the roles of oestrogen and testosterone at puberty and of FSH, LH, oestrogen and progesterone in the menstrual cycle",
        "Explain how FSH, oestrogen, LH and progesterone stimulate and inhibit each other, and read hormone graphs (Higher tier)",
        "Explain how each hormonal and non-hormonal method of contraception works and evaluate them",
        "Describe how FSH and LH fertility drugs and IVF treat infertility, and evaluate the issues (Higher tier)",
        "Explain the roles of adrenaline and thyroxine and how negative feedback controls thyroxine (Higher tier)",
    ], "FSH matures the egg and LH releases it, and both come from the pituitary, not the ovary. On a graph, LH is the sharp spike just before ovulation."),

    ("bio_aqa:4.5.4", &[
        "Explain phototropism and gravitropism in shoots and roots using the unequal distribution of auxin",
        "Carry out Required practical 8 on light or gravity and seedling growth, recording lengths and labelled drawings",
        "State the roles of gibberellins in germination and ethene in cell division and ripening (Higher tier)",
        "Describe how auxins, ethene and gibberellins are used in agriculture and horticulture (Higher tier)",
        "Evaluate the use of hormone weed killers, including their effect on biodiversity (Higher tier)",
    ], "Auxin causes cell elongation, not cell division, and it moves to the shaded side. In roots it slows elongation, so roots bend the opposite way to shoots."),

    ("bio_aqa:4.6.1a", &[
        "Compare sexual and asexual reproduction, including which type of cell division each uses",
        "Explain how meiosis halves the chromosome number and fertilisation restores it",
        "Explain the advantages of each type of reproduction using malarial parasites, fungi, strawberries and daffodils (biology only)",
        "Describe DNA, genes and the genome, and discuss why understanding the human genome matters",
        "Describe DNA as a polymer of nucleotides and use the triplet code and A–T, C–G pairing in calculations (biology only)",
        "Describe protein synthesis and explain how mutations in coding and non-coding DNA can change a protein or its expression (Higher tier, biology only)",
    ], "Meiosis gives four genetically different cells with half the chromosomes, while mitosis gives two identical ones. Gametes carry a single set (23 in humans), not 23 pairs."),

    ("bio_aqa:4.6.1b", &[
        "Explain gamete, chromosome, gene, allele, dominant, recessive, homozygous, heterozygous, genotype and phenotype",
        "Complete and (Higher tier) construct Punnett squares, giving outcomes as ratios, probabilities and percentages",
        "Interpret family trees to decide whether an allele is dominant or recessive and work out genotypes",
        "Describe polydactyly (dominant) and cystic fibrosis (recessive) and make informed judgements about embryo screening",
        "Explain sex determination with XX and XY and show it with a genetic cross",
    ], "A 1 in 4 chance is a probability for each child, not exactly one in every four children. And heterozygous means two different alleles, not two different genes."),

    ("bio_aqa:4.6.2", &[
        "Describe genetic, environmental and combined causes of variation and state that all variants arise from mutations",
        "Explain how evolution happens by natural selection, step by step, in any context",
        "Describe the process of selective breeding and explain its benefits and the risks of inbreeding",
        "Evaluate genetic engineering and GM crops, and (Higher tier) describe its main steps using enzymes and a vector",
        "Separate science only: describe tissue culture, cuttings, embryo transplants and adult cell cloning, with their benefits and risks",
    ], "In natural selection answers people skip 'passes on the allele to offspring' and 'over many generations', or say the organism adapted on purpose. Mutations are random; the environment only selects."),

    ("bio_aqa:4.6.3", &[
        "Describe the evidence for evolution: fossils, antibiotic resistance and the inheritance of genes",
        "Describe three ways fossils form and explain why the fossil record is incomplete",
        "Explain how antibiotic-resistant bacteria such as MRSA develop and how to slow their spread",
        "Describe factors that cause extinction and read evolutionary trees",
        "Separate science only: describe the work of Darwin, Wallace, Lamarck and Mendel, why their ideas took time to be accepted, and the steps of speciation",
    ], "Saying the antibiotic made the bacteria mutate. The resistant mutation was already there by chance; the antibiotic just kills the rest, so the resistant strain survives and multiplies."),

    ("bio_aqa:4.6.4", &[
        "List Linnaeus's seven levels of classification in order, from kingdom to species",
        "Use binomial names (genus and species) to decide which organisms are most closely related",
        "Explain how better microscopes and chemical analysis changed classification",
        "Name Woese's three domains and say what each contains",
        "Interpret evolutionary trees to find common ancestors, relatedness and extinct species",
    ], "Getting the seven levels in the wrong order, or naming the three domains as animals, plants and bacteria. They are Archaea, Bacteria and Eukaryota."),

    ("bio_aqa:4.7.1", &[
        "Describe the levels of organisation from individual to population, community and ecosystem",
        "Suggest the resources plants and animals compete for in a given habitat",
        "Explain interdependence and how removing one species affects a whole community",
        "Explain how changes in abiotic and biotic factors affect a community, using data and calculating percentage change",
        "Explain structural, behavioural and functional adaptations, including extremophiles",
    ], "Mixing up community (living things only) with ecosystem (living plus non-living), and saying plants compete for food. Plants compete for light, space, water and mineral ions."),

    ("bio_aqa:4.7.2", &[
        "Name producers and primary, secondary and tertiary consumers in a food chain",
        "Explain predator–prey cycles, including why predator numbers lag behind prey",
        "Estimate population size from random quadrats and use transects to link distribution to an abiotic factor (Required practical 9; Trilogy 7)",
        "Explain how the carbon and water cycles work and the role of microorganisms in returning CO₂ and mineral ions",
        "Explain how temperature, water and oxygen affect the rate of decay, including compost, biogas and the milk practical (separate science only)",
        "Evaluate how changes in temperature, water and atmospheric gases affect species distribution (separate science only, Higher tier)",
    ], "In quadrat sums, work out how many quadrats fit in the whole area (a 50 cm quadrat is 0.25 m²) before multiplying by the mean. On predator–prey graphs, say the predator peak lags behind the prey peak and give the reason."),

    ("bio_aqa:4.7.3", &[
        "Define biodiversity and explain why high biodiversity keeps ecosystems stable",
        "Describe how pollution of water, air and land, and land use for building, quarrying, farming and landfill, reduce biodiversity",
        "Explain why destroying peat bogs and tropical forests reduces biodiversity and raises carbon dioxide levels",
        "Describe biological consequences of global warming and why the evidence is trusted but still partly uncertain",
        "Evaluate breeding programmes, habitat protection, hedgerows, emission cuts and recycling as ways to maintain biodiversity",
        "Calculate percentage changes and rates from environmental data",
    ], "Deforestation raises carbon dioxide in two ways: burning and decay release it, and fewer trees remove it by photosynthesis. Evaluate questions need both sides and a conclusion."),

    ("bio_aqa:4.7.4", &[
        "Number and name trophic levels from producers to apex predators",
        "Describe how decomposers secrete enzymes and absorb small soluble molecules by diffusion",
        "Construct accurate, to-scale pyramids of biomass from data, with trophic level 1 at the bottom",
        "Explain how biomass is lost between trophic levels through faeces, respiration waste, urine and glucose used in respiration",
        "Calculate the efficiency of biomass transfer as a percentage or fraction",
        "Explain why fewer organisms are found at higher trophic levels and why food chains are short",
    ], "Efficiency is the level above divided by the level below, times 100; an answer over 100 % means it was divided the wrong way. \"Energy is lost as heat\" on its own does not earn the marks: name faeces, respiration waste and urine."),

    ("bio_aqa:4.7.5", &[
        "Describe the biological factors threatening food security and interpret population and food production statistics",
        "Explain how limiting movement, controlling temperature and high-protein feed make animal farming more efficient",
        "Evaluate intensive farming, including ethical objections",
        "Explain how net mesh size and fishing quotas keep fish stocks at a sustainable level",
        "Describe how mycoprotein is made from Fusarium and how GM bacteria produce human insulin",
        "Evaluate GM crops such as golden rice as a solution to feeding a growing population",
    ], "Intensive farming works because animals lose less energy to the surroundings: less movement and kept warm means less glucose respired and more biomass. Fishing methods must be linked to fish surviving to breed."),
    // ---------- English Language (WJEC Eduqas GCSE C700QS) ----------
    ("englang_edq:2.1a", &[
        "Read an unseen 60-100 line extract of 20th-century fiction actively in about ten minutes",
        "Work out who is narrating, whose point of view we share, and how the extract is organised",
        "Recognise how 20th-century fiction reveals character through action, dialogue and what is left unsaid",
        "Plan the 50 minutes of Section A so the three 10-mark questions get the most time",
    ], "Every question names its own lines. Answers that wander outside the given lines earn nothing for the stray material, however good it is."),

    ("englang_edq:2.1b", &[
        "List five separate things from the named lines, explicit or inferred",
        "Write short, selective points rather than copying whole sentences",
        "Make a fair inference where the text implies something without stating it",
        "Finish in about five minutes",
    ], "Copying out a whole sentence is not selecting. Five short, distinct points in your own words or brief quotation are what gain the five marks."),

    ("englang_edq:2.1c", &[
        "Identify accurate impressions of a character or place from a short section",
        "Support each impression with a well-chosen word or phrase",
        "Comment briefly on what the language suggests, using terminology where it helps",
        "Write a compact answer worth five marks in about seven minutes",
    ], "This is a language question even though it is only five marks. A list of impressions with no comment on the words that create them stays at 2 or 3."),

    ("englang_edq:2.1d", &[
        "Explore what a character thinks or feels, or how an atmosphere is built, in a named section",
        "Analyse words, imagery and sentence forms and explain their effect on the reader",
        "Track how the feeling develops or shifts across the lines",
        "Use subject terminology accurately and only when it adds to the point",
    ], "Feature-spotting caps the mark. Naming a simile earns little; explaining what it makes us feel about the character is where the 7-10 band lives."),

    ("englang_edq:2.1e", &[
        "Answer an impressions or relationship question using both language and structure",
        "Comment on the organisation of events: openings, turning points, shifts in focus, pace and endings",
        "Explain how dialogue, sentence length and paragraphing shape the reader's response",
        "Integrate structure into the argument rather than adding it as a separate paragraph",
    ], "The question says language and structure. Answers that never mention the order of events, shifts or pace cannot reach the top band."),

    ("englang_edq:2.1f", &[
        "Take a clear stance on a reader's statement and sustain it",
        "Evaluate using the final section and the passage as a whole",
        "Explain how the writer creates your thoughts and feelings, not just what happens",
        "Weigh evidence on both sides before reaching a judgement",
    ], "Retelling the plot is not evaluation. Every paragraph needs a judgement (how far, and why) tied to a method the writer uses."),

    ("englang_edq:2.1g", &[
        "Choose quickly between the four title types: a title, a story that begins, a story that ends, a 'write about a time when'",
        "Plan a narrative focused on one situation that can be done well in 450-600 words",
        "Use a given opening or ending sentence exactly, and make it matter",
        "Shape the story with a clear turning point and a deliberate ending",
    ], "The task must be a narrative or recount. A purely descriptive piece, a poem or a play script cannot reach the full mark range."),

    ("englang_edq:2.1h", &[
        "Establish a convincing narrative voice and keep it consistent",
        "Reveal character through action, small detail and sparing dialogue",
        "Use description inside the story to create setting and mood without stopping the plot",
        "Control pace: slow the key moment, summarise the rest",
    ], "Over-plotting is the classic failure. A story that races through events has no room for the detail and characterisation that earn AO5's top band."),

    ("englang_edq:2.1i", &[
        "Vary sentence structures deliberately for effect",
        "Punctuate accurately, including speech, apostrophes, semicolons and colons",
        "Spell ambitious and irregular words correctly and choose precise vocabulary",
        "Keep tense and agreement secure and proofread in the final five minutes",
    ], "Sixteen of the forty marks are for technical accuracy. Comma splices and slips into the wrong tense pull a strong story down a whole band."),

    ("englang_edq:2.2a", &[
        "Read a 21st-century text and a 19th-century text of about 900-1200 words between them in ten minutes",
        "Identify the form, purpose, audience and viewpoint of each non-fiction text",
        "Decode 19th-century vocabulary, long sentences and formal conventions without panicking",
        "Recognise the non-fiction forms the paper uses: articles, letters, diaries, autobiography, reports and accounts",
    ], "Older texts are not harder to analyse, only slower to read. Skimming the 19th-century text leads to misreading, which costs marks on A3, A4, A5 and A6."),

    ("englang_edq:2.2b", &[
        "Find exact details quickly in the stated text",
        "Answer briefly: a word, phrase or short sentence",
        "Interpret 19th-century wording accurately in A3",
        "Spend no more than three or four minutes on each set of three marks",
    ], "Answering from the wrong text is the commonest way to lose these marks. In the sample paper and every paper since, A1 is on the first (21st-century) text and A3 on the second (19th-century) one."),

    ("englang_edq:2.2c", &[
        "Explain how a writer tries to achieve the aim named in the question",
        "Cover what is said, the use of language, tone and structure, and other methods such as quotation, facts and headlines",
        "Select a wide range of relevant details across the whole text",
        "Analyse effects with accurate subject terminology",
    ], "The three bullets are the mark scheme. Answers that cover only language, and ignore tone, structure and other methods, rarely pass 6 out of 10."),

    ("englang_edq:2.2d", &[
        "Evaluate a statement about the 19th-century text and decide how far you agree",
        "Comment on both what the writer says and how it is said",
        "Support each judgement with well-selected references",
        "Sustain a coherent stance, acknowledging where the evidence points the other way",
    ], "Agreeing with the statement and listing quotations is not evaluation. The top band needs a weighed, sustained judgement on the text's effect."),

    ("englang_edq:2.2e", &[
        "Select relevant information on a stated focus from both texts",
        "Synthesise: bring details together, showing links or differences",
        "Make it clear which text each detail comes from",
        "Answer briefly, in about five minutes",
    ], "This is information, not analysis. Commenting on language wastes time, and using only one text caps the answer at 1 mark."),

    ("englang_edq:2.2f", &[
        "Compare the writers' ideas or experiences on the focus given",
        "Compare how each writer conveys them: language, tone, structure and form",
        "Sustain comparison with comparative connectives and paired evidence",
        "Use well-chosen evidence from both texts",
    ], "Writing about Text A and then Text B with no linking is description, not comparison, and is held in the lower bands."),

    ("englang_edq:2.2g", &[
        "Write in the forms the paper sets: letters, articles, reviews, talks or speeches, reports and guides",
        "Signal each form's conventions quickly and accurately",
        "Adapt register and tone to the stated audience and real-life context",
        "Keep the purpose in view in every paragraph",
    ], "Elaborate addresses, columns and invented headlines earn nothing. Show the form briefly and spend the time on content and register."),

    ("englang_edq:2.2h", &[
        "Persuade, argue and advise using rhetorical devices such as rhetorical questions, antithesis and parenthesis",
        "Develop ideas with reasons, examples and plausible detail",
        "Anticipate and answer an opposing view",
        "Choose devices for their effect rather than ticking them off",
    ], "A string of devices is not an argument. Ideas that are 'convincingly developed and supported' are what the top band rewards."),

    ("englang_edq:2.2i", &[
        "Plan two 300-400 word responses in five minutes each",
        "Split the hour evenly: thirty minutes per task",
        "Paragraph for sequence and use discourse markers for cohesion",
        "Write accurately under time pressure and check both pieces",
    ], "Running out of time on the second task is costly: each task is worth 20 marks, and a short second piece cannot reach the higher bands."),

    // ---------- Chemistry (AQA GCSE Chemistry (8462) Higher) ----------
    ("chem_aqa:4.1.1", &[
        "Tell elements, compounds and mixtures apart, and choose a separation technique for a given mixture",
        "Describe how the model of the atom changed from solid spheres to the nuclear model, linking each change to its evidence",
        "Work out the protons, neutrons and electrons in any atom or ion from its atomic and mass numbers",
        "Calculate relative atomic mass from isotope abundances",
        "Write the electronic structure of the first 20 elements as numbers and as diagrams",
        "Write and balance symbol equations, and (Higher tier) half and ionic equations",
    ], "In alpha scattering answers, each observation has to be tied to what it shows: most went straight through, so the atom is mostly empty space. A list of observations with no conclusions drops most of the marks."),

    ("chem_aqa:4.1.2", &[
        "Link an element's position in the periodic table to its electron arrangement and atomic number",
        "Describe how Mendeleev built his table and why his gaps and predictions got it accepted",
        "Explain the differences between metals and non-metals using their properties and outer electrons",
        "Describe the reactions of lithium, sodium and potassium with water, oxygen and chlorine, and write their equations",
        "Explain the opposite reactivity trends in Group 1 and Group 7, and predict properties down Groups 0, 1 and 7",
        "Predict and describe halogen displacement reactions",
    ], "Reactivity trends need the whole chain: more shells, outer electron further from the nucleus, weaker attraction, so lost more easily in Group 1 or gained less easily in Group 7. Stopping at \"the atom is bigger\" drops the marks."),

    ("chem_aqa:4.1.3", &[
        "Compare transition metals with Group 1 for melting point, density, strength, hardness and reactivity with oxygen, water and halogens",
        "Illustrate each difference using chromium, manganese, iron, cobalt, nickel or copper",
        "Work out the charge on a transition metal ion from a formula and name the compound with a Roman numeral",
        "Give examples of coloured transition metal compounds",
        "Name transition metals or their compounds used as catalysts and the reactions they speed up",
    ], "Comparisons need both metals and a comparative word: \"iron is denser than sodium\", not \"iron is dense\". One-sided statements are the usual lost marks."),

    ("chem_aqa:4.2.1", &[
        "Decide whether a substance has ionic, covalent or metallic bonding from the elements in it",
        "Work out ion charges from group numbers and draw dot and cross diagrams for Group 1 or 2 metals with Group 6 or 7 non-metals",
        "Draw dot and cross diagrams for H₂, Cl₂, O₂, N₂, HCl, H₂O, NH₃ and CH₄, and line diagrams for molecules, polymers and giant structures",
        "Describe the structure of sodium chloride and the limitations of each way of representing it",
        "Deduce empirical and molecular formulae from diagrams and models",
        "Describe metallic bonding as positive ions held by delocalised electrons",
    ], "The ionic bond is the electrostatic attraction between oppositely charged ions, not the electron transfer and never \"sharing\". In dot and cross diagrams, missing square brackets, charges or lone pairs costs marks every time."),

    ("chem_aqa:4.2.2", &[
        "Use the particle model to explain melting, boiling, freezing and condensing, and (Higher tier) state its limitations",
        "Predict the state of a substance at a given temperature from its melting and boiling points",
        "Explain the melting points and conductivity of ionic compounds, small molecules, polymers, giant covalent structures and metals from their bonding",
        "Explain why larger molecules have higher boiling points and why alloys are harder than pure metals",
        "Add the correct state symbols to equations",
    ], "When a substance made of small molecules melts or boils, the weak intermolecular forces are overcome, not the covalent bonds. Saying the bonds break is the most common mark lost on this topic."),

    ("chem_aqa:4.2.3", &[
        "Explain the hardness, high melting point and non-conductivity of diamond from its four covalent bonds per carbon",
        "Explain why graphite is soft, has a high melting point and conducts, using its layers and delocalised electrons",
        "Describe graphene as a single layer of graphite and link its properties to uses in electronics and composites",
        "Recognise diamond, graphite, graphene, C₆₀ and carbon nanotubes from diagrams and descriptions",
        "Give uses of fullerenes and nanotubes and explain why C₆₀ melts far lower than diamond",
    ], "Graphite is soft because there are no covalent bonds between its layers, only weak forces, and it conducts because one electron per carbon is delocalised and carries charge. Saying \"weak covalent bonds between layers\" or \"strong bonds\" without \"many\" and \"lots of energy\" loses the mark."),

    ("chem_aqa:4.2.4", &[
        "Give the size ranges of nanoparticles (1–100 nm), fine particles (PM2.5) and coarse particles (PM10) in nm and in standard form",
        "Compare nanoparticle sizes with atoms and molecules using orders of magnitude",
        "Calculate the surface area to volume ratio of a cube and say how it changes when the side shrinks by a factor of 10",
        "Explain why nanoparticles can behave differently from bulk material and why less may be needed",
        "Evaluate a use of nanoparticles from given information, weighing benefits against possible risks",
    ], "The mark is for \"high surface area to volume ratio\", not \"large surface area\" — a nanoparticle's own surface is tiny. In evaluate questions, give a benefit, a risk and a justified conclusion, or you cap at half marks."),

    ("chem_aqa:4.3.1", &[
        "State the law of conservation of mass and balance symbol equations by changing only the numbers in front of formulae",
        "Calculate relative formula mass, including formulae with brackets, and show that masses in a balanced equation add up",
        "Calculate the percentage by mass of an element in a compound",
        "Explain apparent mass changes in open containers when a gas is gained from or lost to the air",
        "Calculate a mean, its range and its uncertainty, and write a result as mean ± uncertainty",
    ], "Mass is never \"lost\" — in an open container a gas escapes or oxygen joins from the air, and you must say so. In calculations, multiply out brackets and big numbers fully and never give Mr a unit."),

    ("chem_aqa:4.3.2", &[
        "(Higher) Convert between mass, moles and number of particles using Mr and the Avogadro constant, 6.02 × 10²³ per mole",
        "(Higher) Calculate the mass of a reactant or product from a balanced equation and the mass of another substance",
        "(Higher) Work out the balancing numbers in an equation from the masses of reactants and products",
        "(Higher) Identify the limiting reactant and explain how it fixes the amount of product",
        "Calculate concentration in g/dm³ and the mass of solute in a given volume, converting cm³ to dm³ first",
    ], "Read the mole ratio from the balanced equation every time — 2Mg : O₂ is 2 : 1, not 1 : 1 — and always convert cm³ to dm³ before using the concentration equation."),

    ("chem_aqa:4.3.3", &[
        "Give the three reasons why the actual yield is less than the theoretical amount",
        "Calculate percentage yield and rearrange the equation to find an actual or theoretical mass",
        "(Higher) Calculate the theoretical mass of product from a reactant mass and the balanced equation",
        "Calculate atom economy from a balanced equation, using the balancing numbers",
        "Explain why high atom economy matters for sustainability and cost",
        "(Higher) Choose a reaction pathway using atom economy, yield, rate, equilibrium position and by-products",
    ], "Atom economy uses the Mr of all reactants multiplied by their balancing numbers on the bottom — not just one reactant. Do not confuse it with yield: atom economy comes from the equation, yield from the experiment."),

    ("chem_aqa:4.3.4", &[
        "Calculate concentration in mol/dm³ from moles (or mass and Mr) and volume, converting cm³ to dm³ first",
        "Rearrange c = n ÷ V to find the moles or mass of solute in a given volume of solution",
        "Convert concentrations between mol/dm³ and g/dm³ using Mr",
        "Explain how concentration depends on the mass of solute and the volume of solution",
        "Use reacting volumes, a known concentration and the mole ratio from a balanced equation to find an unknown concentration",
        "Select concordant titres and calculate a mean titre",
    ], "Most lost marks come from leaving volumes in cm³ instead of dividing by 1000, and from ignoring the 1 : 2 mole ratio when sulfuric acid or sodium carbonate is used."),

    ("chem_aqa:4.3.5", &[
        "State that equal amounts in moles of gases occupy the same volume at the same temperature and pressure, and explain why",
        "Use 24 dm³ as the volume of one mole of any gas at room temperature and pressure (20 °C, 1 atm)",
        "Calculate the volume of a gas from its mass and Mr, and the mass or Mr from a volume",
        "Work out volumes of gaseous reactants and products directly from the mole ratio in a balanced equation",
        "Calculate the volume of gas made from a given mass of a solid reactant, including any gas left in excess",
    ], "Dividing a volume in cm³ by 24 instead of 24 000 is the classic slip, closely followed by counting liquid water as a gas when adding up final volumes."),

    ("chem_aqa:4.4.1", &[
        "Explain oxidation and reduction as gain and loss of oxygen, and identify what is oxidised and reduced in an equation",
        "Recall the reactions of potassium, sodium, lithium, calcium, magnesium, zinc, iron and copper with water and dilute acids, and place them in order of reactivity",
        "Explain reactivity in terms of a metal's tendency to form positive ions, and deduce an order of reactivity from experimental results",
        "Predict displacement reactions and explain why metals below carbon are extracted by reduction with carbon",
        "(Higher tier) Define oxidation and reduction in terms of electrons and write ionic and half equations for displacement reactions",
    ], "Students lose marks by naming the metal rather than the metal oxide or metal ion as the substance reduced, and by writing ionic equations that still contain spectator ions or whose charges do not balance."),

    ("chem_aqa:4.4.2", &[
        "Predict the salt and other products when acids react with metals, alkalis, bases and carbonates, and deduce salt formulae from the charges on ions",
        "Describe how to make a pure, dry sample of a soluble salt from an insoluble oxide or carbonate (Required practical 1; Trilogy Required practical 8)",
        "Use universal indicator or a pH probe to find pH, and explain neutralisation as H⁺ + OH⁻ → H₂O",
        "(Separate science only) Describe how to carry out an accurate titration and, at Higher tier, calculate concentrations in mol/dm³ and g/dm³ (Required practical 2)",
        "(Higher tier) Explain metal–acid reactions as redox, and distinguish strong/weak from concentrated/dilute acids",
        "(Higher tier) Use the rule that each fall of one pH unit means ten times the hydrogen ion concentration",
    ], "The most common slips are confusing strong with concentrated, and leaving out a reason or a step (excess solid, filtering, evaporating to the crystallisation point) in the salt-making method."),

    ("chem_aqa:4.4.3", &[
        "Explain why ionic compounds conduct when molten or dissolved, and which ions move to the cathode and the anode",
        "Predict the products of electrolysing molten binary ionic compounds and aqueous solutions with inert electrodes",
        "Explain why aluminium is extracted by electrolysis of aluminium oxide in cryolite and why the carbon anode must be replaced",
        "Plan Required practical 3 (Trilogy Required practical 9): electrolyse aqueous solutions, state a hypothesis and identify the products with gas tests",
        "(Higher tier) Write, complete and balance half equations at each electrode and identify them as oxidation or reduction",
    ], "In aqueous solutions people name the metal (such as sodium) at the cathode or sulfur at the anode, instead of applying the rules: hydrogen unless the metal is less reactive than hydrogen, oxygen unless a halide is present."),

    ("chem_aqa:4.5.1", &[
        "Decide whether a reaction is exothermic or endothermic from the temperature change of the surroundings, and give examples and uses of each",
        "Evaluate hand warmers, self-heating cans and cold packs from given data, ending with a justified judgement",
        "Carry out and evaluate Required practical 4 (Trilogy 10): variables, insulation, highest temperature and the crossing point of two best-fit lines",
        "Draw and read reaction profiles, with activation energy from reactants to peak and overall change from reactants to products",
        "(Higher tier) Calculate the energy change from bond energies as bonds broken minus bonds formed, and explain the sign in terms of bonds",
    ], "Activation energy arrows drawn from the axis or the products instead of from the reactants lose easy marks. In bond energy sums, forgetting the balancing numbers (2O₂ is two O=O bonds) or dropping the minus sign throws away the answer mark."),

    ("chem_aqa:4.5.2", &[
        "Describe how a simple cell is made from two different metals and an electrolyte, and why batteries connect cells in series",
        "Use cell voltage data to put metals in order of reactivity and predict the voltage of a new metal pair",
        "Explain why non-rechargeable cells stop working and how rechargeable cells are recharged by reversing the reactions",
        "Describe a hydrogen fuel cell and write its overall equation, and (Higher tier) the half equations at each electrode",
        "Evaluate hydrogen fuel cells against rechargeable batteries with points on both sides and a justified conclusion",
    ], "Fuel cell evaluations that only say \"it just makes water\" lose most of the marks: you must also deal with how hydrogen is made and stored and finish with a judgement. In half equations, electrons on the wrong side or unbalanced charge cost the mark."),

    ("chem_aqa:4.6.1", &[
        "Calculate mean rates in g/s or cm³/s, and (Higher tier) in mol/s, from the quantity used or formed and the time taken",
        "Draw and interpret product-against-time graphs, draw tangents, and (Higher tier) calculate a tangent's gradient as the rate at a given time",
        "Explain the effects of concentration, pressure, surface area, temperature and catalysts using collision theory and activation energy",
        "Use surface area to volume ratios and simple proportionality to predict how rate changes",
        "Plan and evaluate Required practical 5 (Trilogy 11): gas volume and disappearing-cross methods, variables, rate = 1/time and safety",
    ], "Write \"more frequent collisions\", not just \"more collisions\", and for temperature give both effects: more frequent collisions and more collisions with at least the activation energy. Catalysts lower the activation energy by giving another pathway; they do not give particles more energy."),

    ("chem_aqa:4.6.2", &[
        "Write reversible reactions with the ⇌ symbol and describe the ammonium chloride and hydrated copper sulfate examples, including colours",
        "Explain that a reversible reaction is exothermic one way and endothermic the other, transferring the same amount of energy",
        "Describe dynamic equilibrium in a closed system as forward and reverse reactions at the same rate with constant amounts",
        "(Higher tier) Use Le Chatelier's Principle to predict and explain the effects of concentration, temperature and pressure on the position of equilibrium",
        "(Higher tier) Interpret yield data to deduce whether a forward reaction is exothermic or endothermic and which side has fewer gas molecules",
    ], "At equilibrium the rates are equal and the amounts are constant, not equal, and the reactions have not stopped. For Le Chatelier answers, name the direction favoured and why (endothermic for a temperature rise, fewer gas molecules for a pressure rise) before giving the effect on yield."),

    ("chem_aqa:4.7.1", &[
        "Describe crude oil as a finite mixture of hydrocarbons, name the first four alkanes and recognise alkanes from CₙH₂ₙ₊₂ and displayed formulae",
        "Explain fractional distillation in terms of evaporation, a temperature gradient and condensation at each boiling point",
        "Recall how boiling point, viscosity and flammability change with molecule size and link this to use as fuels",
        "Write balanced equations for the complete combustion of hydrocarbons and for cracking reactions",
        "Describe catalytic and steam cracking, the bromine water test for alkenes, and why cracking is needed (supply and demand, polymers)",
    ], "In fractional distillation, fractions are separated because they condense at different heights where the column is cooler than their boiling point; \"they boil off at different levels\" loses the mark. For the alkene test say bromine water turns from orange to colourless, not \"clear\"."),

    ("chem_aqa:4.7.2", &[
        "Recognise alkenes, alcohols and carboxylic acids from names, formulae and functional groups, and draw displayed formulae of the first members",
        "Describe the addition of hydrogen, steam and halogens to alkenes with their conditions, and draw the products",
        "Describe what the first four alcohols do with sodium, air, water and oxidising agents, and balance their combustion equations",
        "Give the conditions for fermentation and compare it with hydration of ethene as a way to make ethanol",
        "Describe the reactions of carboxylic acids with carbonates, water and alcohols, naming ethyl ethanoate",
        "(Higher tier) Explain why carboxylic acids are weak acids in terms of partial ionisation and pH",
    ], "Conditions written as 'heat and a catalyst' lose the marks: hydration needs steam, 300 °C, 60–70 atm and phosphoric acid, and fermentation needs yeast, 30–35 °C and no air. Forgetting that the carbon in –COOH counts makes CH₃COOH wrongly 'methanoic acid'."),

    ("chem_aqa:4.7.3", &[
        "Recognise addition polymers and their monomers, using the C=C in the monomer as the clue",
        "Draw the repeating unit of an addition polymer from a given alkene monomer, and work back from a repeating unit to the monomer",
        "(Higher tier) Explain condensation polymerisation from the functional groups of the monomers, using ethanediol and hexanedioic acid making a polyester",
        "(Higher tier) Describe how amino acids such as glycine form polypeptides and proteins by condensation",
        "Describe DNA as two nucleotide polymer chains in a double helix, and name the monomers of proteins, starch and cellulose",
    ], "Repeating units drawn with the double bond still in, without bonds through the brackets, or with n missing lose the drawing marks. In condensation polymerisation, forgetting that water is the second product is the commonest slip."),

    ("chem_aqa:4.8.1", &[
        "Use melting and boiling point data to decide whether a substance is pure, and contrast the chemical and everyday meanings of pure",
        "Identify a formulation from given information: a designed mixture with measured components, each with a purpose",
        "Explain how paper chromatography separates a mixture in terms of the stationary phase and the mobile phase",
        "Calculate Rf values from chromatograms to a sensible number of significant figures, and use them to identify substances and judge purity",
        "Carry out and evaluate Required practical 6 (Trilogy 12): pencil start line above the solvent, small spots, lid, solvent front marked",
    ], "Rf answers bigger than 1 (divided the wrong way round) or distances measured from the bottom of the paper instead of the start line throw away the calculation marks. Saying impurities raise the melting point loses easy marks: they lower it and spread it over a range."),

    ("chem_aqa:4.8.2", &[
        "Describe the test and positive result for hydrogen: a burning splint at the mouth of the tube gives a squeaky pop",
        "Describe the test and positive result for oxygen: a glowing splint relights",
        "Describe the test and positive result for carbon dioxide: bubbled through limewater it turns milky, and explain the calcium carbonate precipitate",
        "Describe the test and positive result for chlorine: damp litmus paper is bleached white, done in a fume cupboard",
        "Use gas test results to identify the products of reactions and electrolysis",
    ], "Mixing up the burning splint (hydrogen, pop) with the glowing splint (oxygen, relights) loses both marks. Writing 'glows brighter', 'limewater changes colour' or leaving out 'damp' for chlorine all miss the result mark."),

    ("chem_aqa:4.8.3", &[
        "Identify lithium, sodium, potassium, calcium and copper ions from flame test colours, and explain why colours can be masked in mixtures",
        "Identify aluminium, calcium, magnesium, copper(II), iron(II) and iron(III) ions with sodium hydroxide, and write balanced equations for the hydroxide precipitates",
        "Describe the tests for carbonate, halide and sulfate ions, including which acid is added first and why",
        "Carry out Required practical 7 (chemistry only, no Trilogy equivalent): identify both ions in an unknown single ionic compound",
        "State the advantages of instrumental methods and interpret flame emission spectra against a reference set",
    ], "Ion tests lose marks when the acid is missing or wrong: nitric acid goes with silver nitrate and hydrochloric acid with barium chloride. Writing 'red' for lithium (crimson) or calcium (orange-red), and swapping the iron(II) green and iron(III) brown precipitates, are the other common slips."),

    ("chem_aqa:4.9.1", &[
        "State the proportions of gases in today's atmosphere (about four-fifths nitrogen, one-fifth oxygen, small amounts of carbon dioxide, water vapour and noble gases) and use them in percentage calculations",
        "Describe the theory of the early atmosphere: volcanic gases, mainly carbon dioxide, little or no oxygen, water vapour condensing to form the oceans",
        "Explain how oxygen increased through photosynthesis by algae (from 2.7 billion years ago) and then plants",
        "Explain how carbon dioxide decreased by dissolving in the oceans, photosynthesis, and the formation of sedimentary rocks and fossil fuels",
        "Describe and explain the formation of limestone, coal, crude oil and natural gas",
        "Evaluate theories about the early atmosphere from given evidence, recognising why the evidence is limited",
    ], "Saying oxygen came from volcanoes, or that carbon dioxide simply 'disappeared', loses the marks: name photosynthesis by algae and plants, dissolving in the oceans, and locking up in rocks and fossil fuels. Mixing up which deposits came from plants, plankton and shells is the other common slip."),

    ("chem_aqa:4.9.2", &[
        "Name water vapour, carbon dioxide and methane as greenhouse gases and describe the greenhouse effect in terms of short and long wavelength radiation",
        "Recall two human activities that increase carbon dioxide and two that increase methane",
        "Evaluate reports about climate change: peer review, bias, uncertainty in the data and the limits of models",
        "Describe four potential effects of global climate change and discuss their scale and risk",
        "Define the carbon footprint, describe actions that reduce it, and give reasons why those actions may be limited",
        "Calculate percentage changes in greenhouse gas levels and carbon footprints from given data",
    ], "Saying greenhouse gases 'trap heat' or 'reflect' radiation, or bringing in the ozone layer, loses the mechanism marks: incoming radiation is short wavelength, and greenhouse gases absorb and re-emit the long wavelength infrared that the Earth gives out."),

    ("chem_aqa:4.9.3", &[
        "List the gases and particulates released when fuels burn: carbon dioxide, water vapour, carbon monoxide, sulfur dioxide, oxides of nitrogen, soot and unburned hydrocarbons",
        "Describe how carbon monoxide, soot, sulfur dioxide and oxides of nitrogen are produced, including the conditions for each",
        "Predict the products of combustion from the composition of a fuel and the oxygen supply",
        "Explain the problems each pollutant causes: toxicity, respiratory problems, acid rain, global dimming and health effects",
        "Write and balance equations for complete and incomplete combustion",
    ], "Oxides of nitrogen do not come from the fuel: nitrogen and oxygen from the air react at the high temperature of the engine. The other common slip is mixing up the effects: sulfur dioxide and oxides of nitrogen cause acid rain, and particulates cause global dimming."),

    ("chem_aqa:4.10.1", &[
        "Distinguish finite from renewable resources, give examples of natural products replaced by synthetic ones, and define sustainable development",
        "Distinguish potable water from pure water and give reasons for each step in producing potable water from fresh water and from salty water",
        "Carry out and evaluate Required practical 8 (Trilogy 13): pH, mass of dissolved solids and distillation of water samples, including concentration in g/dm³",
        "Describe the stages of sewage treatment and compare the ease of getting potable water from ground, waste and salt water",
        "(Higher tier) Describe and evaluate phytomining and bioleaching, and how copper is then obtained by displacement with scrap iron or electrolysis",
    ], "Saying filtration kills microbes or removes salt loses the mark: filter beds remove solid particles, sterilising (chlorine, ozone or UV) kills microbes, and only desalination removes dissolved salts. In sewage treatment, sludge is digested anaerobically and effluent is treated aerobically, not the other way round."),

    ("chem_aqa:4.10.2", &[
        "Name the four stages of a life cycle assessment and include transport at each stage",
        "Explain why LCA is not purely objective and how selective LCAs can be misused, for example in advertising",
        "Carry out a simple comparative LCA of plastic and paper shopping bags and reach a justified conclusion",
        "Interpret LCA data using ratios, percentages, per-use values and sensible significant figures",
        "Evaluate reducing, reusing and recycling materials such as glass and metals, including their limits",
    ], "Evaluating without quoting the data or reaching a conclusion caps the marks: compare the figures given, stage by stage, then give a justified judgement. Remember that pollutant effects need value judgements, so an LCA is never purely objective."),

    ("chem_aqa:4.10.3", &[
        "Describe and interpret experiments showing that both air and water are needed for iron to rust",
        "Explain barrier protection and sacrificial protection (galvanising) in terms of relative reactivity, and why aluminium does not corrode further",
        "Recall the composition and a use of bronze, brass, gold alloys, high and low carbon steel, stainless steel and aluminium alloys, and calculate gold content from carats",
        "Explain how LD and HD poly(ethene) both come from ethene, and the structural difference between thermosoftening and thermosetting polymers",
        "Describe glass, clay ceramics and composites, and compare materials quantitatively from data to select one for a given use",
    ], "Saying zinc or magnesium 'protects' iron without saying it is more reactive than iron, so corrodes instead, loses the mark. Swapping high carbon steel (strong but brittle) with low carbon steel (soft and easily shaped) is the other frequent slip."),

    ("chem_aqa:4.10.4", &[
        "Describe the Haber process: nitrogen from the air, hydrogen from natural gas, iron catalyst, about 450 °C and 200 atmospheres, ammonia removed by cooling and unreacted gases recycled",
        "(Higher tier) Explain the choice of temperature, pressure and catalyst as a trade-off between equilibrium yield, rate and cost, and interpret graphs of yield against conditions",
        "Recall that ammonia makes ammonium salts and nitric acid, and that potassium salts and phosphate rock are mined",
        "Recall the products when phosphate rock is treated with nitric, sulfuric and phosphoric acid",
        "Calculate the percentage of an element in a fertiliser and compare laboratory and industrial preparation of an ammonium salt",
    ], "Writing that the iron catalyst increases the yield loses the mark: it only increases the rate. For temperature and pressure, a full answer gives the effect on yield and on rate or cost, then calls the chosen value a compromise."),

    // ---------- Physics (AQA GCSE Physics (8463) Higher) ----------
    ("phys_aqa:4.1.1", &[
        "Describe the changes in energy stores for an object thrown upwards, a collision, a braking vehicle and a kettle",
        "Recall and use Ek = ½mv² and Ep = mgh, and use Ee = ½ke² from the equation sheet",
        "Use ΔE = mcΔθ and explain what specific heat capacity means",
        "Describe Required practical 1 and explain why the measured specific heat capacity is usually too high",
        "Recall and use P = E ÷ t and P = W ÷ t, comparing devices that transfer the same energy at different rates",
        "Show on a common scale in joules how energy is redistributed when a system changes",
    ], "Only the speed is squared in kinetic energy and only the extension in elastic energy. Forgetting to convert grams, centimetres or minutes before substituting costs the answer mark."),

    ("phys_aqa:4.1.2", &[
        "State that energy cannot be created or destroyed and describe energy transfers in a closed system with no net change",
        "Describe how energy is dissipated in every change and explain how lubrication and thermal insulation reduce unwanted transfers",
        "Explain how the thickness and thermal conductivity of walls affect the rate of cooling of a building",
        "Recall and use both efficiency equations, giving answers as a decimal or a percentage",
        "Describe ways to increase the efficiency of an intended energy transfer (Higher tier)",
        "Plan and evaluate Required practical 2 on thermal insulators (separate science only)",
    ], "Efficiency is useful divided by total, so it can never be above 1 or 100%. Turn a percentage into a decimal before rearranging to find an input."),

    ("phys_aqa:4.1.3", &[
        "Describe the main energy resources and sort them into renewable and non-renewable",
        "Compare how resources are used for transport, electricity generation and heating",
        "Explain why some energy resources are more reliable than others",
        "Describe the environmental impact of each resource and evaluate choices with a justified conclusion",
        "Explain patterns and trends in energy use from graphs and data, calculating percentages and percentage change",
        "Explain why science can identify environmental problems but cannot always solve them, for political, social, ethical and economic reasons",
    ], "Comparisons must cover both resources on every point, and an evaluate question needs a justified conclusion. Renewable does not mean harmless: hydro floods habitats, and nuclear emits no carbon dioxide."),

    ("phys_aqa:4.2.1", &[
        "Draw and interpret circuit diagrams using the standard symbols, with ammeters in series and voltmeters in parallel",
        "Recall and use Q = It, knowing that current is the same at every point in a single loop",
        "Recall and use V = IR, rearranging it confidently and converting mA and minutes",
        "Describe Required practical 3: how the resistance of a wire depends on its length, and resistors in series and parallel",
        "Describe Required practical 4 and sketch and explain the I–V graphs of a resistor, a filament lamp and a diode",
        "Explain how thermistors and LDRs change resistance and how they are used in thermostats and automatic lights",
    ], "For a curved I–V graph, work out resistance as V divided by I at the point, not from the gradient. When explaining the lamp's curve, say the resistance rises because the filament gets hotter."),

    ("phys_aqa:4.2.2", &[
        "State the rules for current, potential difference and resistance in series and in parallel circuits",
        "Use R total = R1 + R2 and V = IR to calculate currents, pds and resistances in series circuits, including unknown resistors",
        "Work out branch currents and the supply current in parallel circuits, knowing the total resistance is less than the smallest resistor",
        "Explain why adding resistors in series increases total resistance while adding them in parallel decreases it",
        "Explain how series circuits are used for measuring and testing, such as current-limiting resistors and thermistor or LDR sensor circuits",
        "Build and check series and parallel circuits from a circuit diagram",
    ], "Series keeps the current the same and shares the pd; parallel keeps the pd the same and shares the current. Adding resistances only works in series: in parallel the total is less than the smallest resistor."),

    ("phys_aqa:4.2.3", &[
        "Explain the difference between direct and alternating potential difference",
        "State the frequency (50 Hz) and potential difference (about 230 V) of the UK mains",
        "Identify the live, neutral and earth wires by colour and state the potential of each",
        "Explain why a live wire can be dangerous even when a switch in the circuit is open",
        "Explain the dangers of any connection between the live wire and earth",
    ], "An open switch stops the current but not the danger: the live wire is still at about 230 V, so touching it puts a large pd across your body. Students who say \"switched off means safe\" lose the mark."),

    ("phys_aqa:4.2.4", &[
        "Recall and use P = VI and P = I²R, rearranging either to find any quantity",
        "Recall and use E = Pt and E = QV, converting kW and minutes or hours first",
        "Explain how a device's power relates to the pd across it, the current through it and the energy it transfers over time",
        "Describe the energy transfers in everyday appliances and link power ratings to changes in stored energy",
        "Explain why the National Grid uses step-up and step-down transformers and why this is efficient",
        "(Higher tier) Use Vp × Ip = Vs × Is for an ideal transformer",
    ], "The National Grid answer must go through the current: high pd means low current, and the heating loss depends on current squared. \"High voltage means less energy lost\" on its own earns almost nothing."),

    ("phys_aqa:4.2.5", &[
        "Describe how rubbing two insulators transfers electrons and leaves equal and opposite charges",
        "Describe evidence that like charges repel and unlike charges attract without contact",
        "Explain sparking in terms of charge build-up and a strong electric field",
        "Draw the radial electric field pattern for an isolated charged sphere",
        "Use the idea of an electric field to explain non-contact forces and why they grow as distance shrinks",
    ], "Only electrons move when objects are charged by rubbing. Saying a rod became positive because it gained protons or positive charge loses the mark every time."),

    ("phys_aqa:4.3.1", &[
        "Draw and describe particle diagrams for solids, liquids and gases",
        "Recall and use density = mass ÷ volume, converting between g/cm³ and kg/m³",
        "Explain differences in density between states using the spacing of particles",
        "Carry out Required practical 5: find the density of regular solids, irregular solids and liquids",
        "Explain why mass is conserved in a change of state and why it is a physical change",
    ], "Density differences come from how far apart the particles are, not from the particles getting lighter. And 1 cm³ is a millionth of a cubic metre, so 1 g/cm³ is 1000 kg/m³."),

    ("phys_aqa:4.3.2", &[
        "Define internal energy as the total kinetic and potential energy of the particles in a system",
        "Explain that heating either raises the temperature or changes the state",
        "Use ΔE = mcΔθ and E = mL from the equation sheet, splitting multi-stage problems into steps",
        "Distinguish specific heat capacity from specific latent heat, including their units",
        "Interpret heating and cooling graphs, using plateau times to find latent heat",
        "Describe how to measure the specific latent heat of fusion of ice using a control funnel",
    ], "On the flat part of a heating graph the temperature is constant but the internal energy is still rising: the energy goes into the particles' potential energy. Saying no energy is being transferred loses the mark."),

    ("phys_aqa:4.3.3", &[
        "Explain how the random motion of gas molecules causes pressure on the walls of a container",
        "Explain why heating a gas at constant volume increases its pressure, linking speed, collision rate and force",
        "Separate science only: explain why increasing the volume of a gas at constant temperature decreases its pressure",
        "Separate science only: use pV = constant from the equation sheet to calculate a new pressure or volume",
        "Separate science only (Higher tier): explain how doing work on a gas, as in a bicycle pump, raises its temperature",
    ], "When a gas is squashed at constant temperature the molecules hit the walls more often, not harder: their speed has not changed. Saying the collisions are harder, or that the molecules hit each other more, loses the mark."),

    ("phys_aqa:4.4.1", &[
        "Describe the structure of an atom, including the size of the atom and nucleus in standard form and where the mass is",
        "Work out the numbers of protons, neutrons and electrons from a nuclear symbol, for atoms, isotopes and positive ions",
        "Explain how electrons move between energy levels when an atom absorbs or emits electromagnetic radiation",
        "Describe the plum pudding and nuclear models and the differences between them",
        "Explain how the alpha particle scattering results led to the nuclear model, then the roles of Bohr, protons and Chadwick's neutrons",
    ], "Isotopes have the same number of protons but different numbers of neutrons. Writing that they differ in protons or electrons, or giving scattering observations without the conclusion each one supports, loses marks."),

    ("phys_aqa:4.4.2", &[
        "Describe alpha, beta, gamma and neutron radiation and compare their penetration, range in air and ionising power",
        "Choose and justify the best type of radiation for a given use",
        "Write balanced nuclear equations for alpha and beta decay, and state the effect of gamma emission",
        "Explain half-life and its link to random decay, and find it from a graph or data",
        "Higher tier: calculate the net decline, as a ratio, after a given number of half-lives",
        "Compare the hazards of contamination and irradiation and describe precautions against each",
    ], "An irradiated object does not become radioactive; only contamination puts radioactive atoms onto or into something. Mixing these up, or saying beta particles come from the electron shells, loses easy marks."),

    ("phys_aqa:4.4.3", &[
        "Separate science only: name natural and man-made sources of background radiation and explain how occupation and location affect dose",
        "Convert and compare radiation doses in sieverts and millisieverts, using standard form",
        "Explain why the hazard from a radioactive material depends on its half-life, giving both intensity and duration",
        "Describe how nuclear radiation is used to explore internal organs and to control or destroy unwanted tissue, justifying the radiation and half-life chosen",
        "Evaluate the perceived risks of medical uses of radiation against data and consequences, reaching a conclusion",
    ], "A short half-life means intense radiation that soon dies away, while a long half-life means weaker radiation that lasts; answers that say only one side, or that a short half-life is simply safer, drop marks."),

    ("phys_aqa:4.4.4", &[
        "Separate science only: describe nuclear fission of uranium-235 or plutonium-239, starting with the absorption of a neutron",
        "List the products of fission: two smaller nuclei, two or three neutrons, gamma rays and energy as kinetic energy",
        "Draw and interpret diagrams of fission and of a chain reaction, and balance fission equations",
        "Explain how a chain reaction is controlled in a reactor and why it is uncontrolled in a nuclear weapon",
        "Describe nuclear fusion and explain that some mass is converted into the energy of radiation",
        "Compare fission and fusion",
    ], "Control rods absorb neutrons; the moderator slows them down. Swapping the two, or leaving out the two or three neutrons released in fission, is the most common lost mark."),

    ("phys_aqa:4.5.1", &[
        "Classify quantities as scalars or vectors and represent a vector as an arrow whose length shows its magnitude",
        "Sort forces into contact and non-contact forces and describe interaction pairs between two objects",
        "Recall and use W = mg, explaining the difference between mass and weight and that weight is proportional to mass",
        "Calculate the resultant of forces acting along a straight line and state its direction",
        "Draw free body diagrams and use them to describe balanced and unbalanced forces (Higher tier)",
        "Use scale drawings to resolve a force into two components and to find a resultant or show equilibrium (Higher tier)",
    ], "Mass is in kilograms and never changes; weight is a force in newtons that depends on g. A resultant force needs a direction as well as a size."),

    ("phys_aqa:4.5.2", &[
        "Recall and use W = Fs, using the distance moved along the line of action of the force",
        "Explain why no work is done when there is no displacement in the direction of the force",
        "Convert between newton-metres and joules, and between J, kJ and MJ",
        "Describe the energy transfer between stores when a force does work",
        "Explain why work done against friction raises the temperature of an object",
        "Combine W = Fs with kinetic or gravitational potential energy in two-step problems",
    ], "Use the distance moved in the direction of the force: the vertical height when lifting. And when given a mass, find the weight with W = mg before using W = Fs."),

    ("phys_aqa:4.5.3", &[
        "Give examples of stretching, bending and compressing, and explain why a stationary object needs more than one force to change shape",
        "Describe the difference between elastic and inelastic deformation",
        "Recall and use F = ke, working out extension as new length minus original length in metres",
        "Distinguish linear from non-linear force-extension graphs and find the spring constant from the gradient",
        "Apply Ee = 0.5ke² to calculate the energy stored in a spring and the energy it transfers",
        "Carry out and evaluate Required practical 6 (Trilogy 18): force and extension for a spring",
    ], "Extension is the increase in length, not the new length, and it must be in metres before you use F = ke or Ee = 0.5ke²."),

    ("phys_aqa:4.5.4", &[
        "Describe examples in which forces cause rotation, naming the pivot and direction of turning",
        "Recall and use M = Fd, using the perpendicular distance from the pivot to the line of action of the force",
        "Apply the principle of moments to find an unknown force or distance on a balanced object",
        "Explain how a lever acts as a force multiplier by transmitting the rotational effect of a force",
        "Explain how meshing gears of different sizes change the moment, speed and direction of rotation",
    ], "The distance in M = Fd is the perpendicular distance from the pivot, in metres. In balance questions, include every force's moment on the correct side before solving."),

    ("phys_aqa:4.5.5", &[
        "Recall and use p = F/A, converting areas to square metres, and state that fluid pressure acts at right angles to every surface",
        "Apply p = hρg to find the pressure due to a column of liquid and the pressure difference between two depths (Higher tier)",
        "Explain why pressure in a liquid increases with depth and with the density of the liquid (Higher tier)",
        "Explain upthrust in terms of greater pressure on the bottom of an object, and describe the factors that decide floating and sinking (Higher tier)",
        "Describe a simple model of the atmosphere and explain why atmospheric pressure decreases with height",
    ], "Explain pressure changes with the cause, not just the fact: deeper means a taller, heavier column of liquid above each square metre, and higher means less air above and fewer molecular collisions. Convert cm² to m² by dividing by 10 000."),

    ("phys_aqa:4.5.6a", &[
        "Explain the difference between scalars and vectors for distance, displacement, speed and velocity, giving a displacement as a magnitude and a direction",
        "Recall typical speeds for walking, running, cycling, transport and sound, and use s = vt and average speed with correct unit conversions",
        "Recall and use a = Δv/t, and estimate everyday accelerations using the ≈ symbol",
        "Find speed from the gradient of a distance–time graph (by a tangent on a curve at Higher tier) and acceleration from the gradient of a velocity–time graph",
        "Find distance from the area under a velocity–time graph, counting squares where needed (Higher tier)",
        "Apply v² − u² = 2as from the equation sheet, and explain why circular motion at constant speed is accelerated motion (Higher tier)",
    ], "Read the y-axis before interpreting a graph: a horizontal line means stopped on a distance–time graph but constant velocity on a velocity–time graph. Average speed is total distance ÷ total time, never the mean of two speeds."),

    ("phys_aqa:4.5.6b", &[
        "State and apply Newton's First Law to objects at rest, at constant velocity, and changing speed or direction",
        "Recall and use F = ma with the resultant force, and explain inertia and inertial mass as force ÷ acceleration (Higher tier)",
        "Estimate the forces and accelerations involved in everyday road transport, using the ≈ symbol",
        "State Newton's Third Law and identify equal and opposite force pairs acting on different objects in equilibrium situations",
        "Explain how a falling object reaches terminal velocity, and draw and interpret its velocity–time graph (separate science only)",
        "Describe Required practical 7 (Trilogy 19) to find how acceleration depends on force and on mass, keeping the total mass constant when varying force",
    ], "In F = ma, F is the resultant force — subtract the opposing forces first. Weight and the normal contact force on the same object are balanced forces, not a Newton's Third Law pair."),

    ("phys_aqa:4.5.6c", &[
        "State that stopping distance is thinking distance plus braking distance, and calculate each part using s = vt and v² − u² = 2as",
        "Recall that reaction times are typically 0.2–0.9 s and explain how tiredness, drugs, alcohol and distractions increase thinking distance",
        "Describe and evaluate methods for measuring reaction time, such as the ruler-drop test",
        "Explain how wet or icy roads and worn brakes or tyres increase braking distance, and the implications for safety",
        "Explain braking in terms of work done by friction reducing kinetic energy, and the dangers of large decelerations",
        "Estimate braking forces for road vehicles (Higher tier), and estimate and read stopping distances over a range of speeds (separate science only)",
    ], "Driver factors (tiredness, alcohol, drugs, distractions) change the thinking distance; road, weather, brakes and tyres change the braking distance. Saying reaction time affects braking distance loses the mark."),

    ("phys_aqa:4.5.7", &[
        "Recall and use p = mv, treating momentum as a vector with a sign for direction (Higher tier)",
        "State that total momentum before an event equals total momentum after it in a closed system",
        "Describe and explain collisions and explosions, such as recoil, using conservation of momentum",
        "Calculate velocities after collisions and explosions, including objects that stick together or move in opposite directions (separate science only)",
        "Apply F = mΔv/Δt from the equation sheet and explain how air bags, seat belts, crash mats, helmets and cushioned surfaces reduce force (separate science only)",
    ], "Momentum is a vector, so a velocity in the opposite direction needs a minus sign. Safety features do not reduce the change in momentum — they increase the time, so the rate of change of momentum and the force are smaller."),

    ("phys_aqa:4.6.1", &[
        "Describe the difference between transverse and longitudinal waves, with examples, and the evidence that the wave and not the medium travels",
        "Define amplitude, wavelength, frequency, period and wave speed, and identify amplitude and wavelength on diagrams",
        "Recall and use v = fλ, and apply T = 1/f from the equation sheet",
        "Describe methods to measure the speed of sound in air and of ripples, including Required practical 8 (Trilogy 20)",
        "Draw ray diagrams for reflection and describe Required practical 9 on reflection and refraction of light (separate science only)",
        "Explain hearing limits (20 Hz to 20 kHz), ultrasound, echo sounding and seismic evidence for the Earth's core (separate science only, Higher tier)",
    ], "Compare the direction of oscillation with the direction of energy transfer when describing transverse and longitudinal waves. Amplitude is measured from the rest line to a crest, not crest to trough, and echo distances must be halved."),

    ("phys_aqa:4.6.2", &[
        "List the EM spectrum in order of wavelength and frequency, and use v = fλ with 3.0 × 10⁸ m/s for EM waves",
        "Draw refraction ray diagrams and, on Higher tier, explain refraction with wave fronts: one side slows first, wavelength shortens, frequency stays the same",
        "Describe Required practical 10 (Trilogy RP21) and state that matt black surfaces are the best emitters and absorbers of infrared",
        "Give a use for each EM wave and, on Higher tier, explain why its properties suit that use",
        "Describe the dangers of UV, X-rays and gamma rays and draw conclusions from radiation dose data in sieverts",
        "Separate science only: draw lens ray diagrams, calculate magnification, and explain colour, filters and specular and diffuse reflection",
    ], "Explaining a use without the property behind it loses the mark: say why the wave suits the job, such as bone absorbing X-rays while soft tissue transmits them. In refraction, the frequency never changes — only the speed and wavelength."),

    ("phys_aqa:4.6.3", &[
        "State that all objects emit and absorb infrared, and that hotter objects emit more in a given time",
        "Define a perfect black body and explain why it is the best possible emitter",
        "Describe how the intensity and the peak wavelength of emitted radiation change as temperature rises",
        "Explain, using rates of absorption and emission, why an object warms, cools or stays at constant temperature (Higher tier)",
        "Explain how absorption, reflection and emission of radiation set the temperature of the Earth, and interpret diagrams of it (Higher tier)",
    ], "Talk about rates: an object stays at a constant temperature because it absorbs radiation at the same rate as it emits it, not because it has stopped emitting. Hotter objects peak at shorter wavelengths, not longer."),

    ("phys_aqa:4.7.1", &[
        "Describe how like poles repel and unlike poles attract, and that magnetic forces act without contact",
        "Compare permanent and induced magnets, and explain why induced magnetism always attracts",
        "Name the magnetic materials (iron, steel, cobalt, nickel) and define the direction of a magnetic field",
        "Draw the field of a bar magnet with arrows from north to south and lines closest at the poles",
        "Describe how to plot a field with a compass, and explain how compass behaviour shows the Earth's core is magnetic",
    ], "Field lines need arrows pointing from north to south, and they must be closest together at the poles. Remember that attraction does not prove something is a magnet — only repulsion does."),

    ("phys_aqa:4.7.2", &[
        "Describe how to show the magnetic effect of a current, and draw the fields around a straight wire and a solenoid with their directions",
        "Explain why a solenoid strengthens the field and how an iron core makes an electromagnet",
        "Use Fleming's left-hand rule and recall the factors that affect the size of the motor-effect force (Higher tier)",
        "Calculate force, current, flux density or length using F = BIl, converting cm to m and mT to T (Higher tier)",
        "Explain how the forces on a coil and a split-ring commutator make a d.c. motor rotate (Higher tier)",
        "Separate science only: explain electromagnetic devices from diagrams, and explain how a moving-coil loudspeaker works (Higher tier)",
    ], "In a motor answer, say that the two sides of the coil carry current in opposite directions, so the forces are opposite and give a turning effect — and that the commutator reverses the current every half turn. Use the left hand for the motor effect and convert lengths to metres before using F = BIl."),

    ("phys_aqa:4.7.3", &[
        "Describe the generator effect and recall the factors that affect the size and direction of an induced potential difference",
        "Explain that an induced current makes a magnetic field that opposes the change causing it, and apply this in new contexts",
        "Explain how an alternator produces a.c. and a dynamo produces d.c., and draw and interpret their p.d.–time graphs",
        "Explain how a moving-coil microphone and a transformer work",
        "Use Vp/Vs = np/ns and VsIs = VpIp to find p.d.s, turns and the current drawn from the supply",
        "Explain, with calculations, why the National Grid transmits at high p.d. to reduce current and heating losses",
    ], "A changing magnetic field is essential: a stationary magnet or a steady d.c. induces nothing. In National Grid answers, say that a higher p.d. means a lower current for the same power, so less energy is wasted heating the cables (I²R)."),

    ("phys_aqa:4.8.1", &[
        "Describe the solar system — the Sun, eight planets, dwarf planets and moons — as a small part of the Milky Way galaxy",
        "Explain how gravity pulls a nebula together until fusion starts, and why a main sequence star is in equilibrium",
        "Describe the life cycles of a Sun-sized star and a much more massive star, in the right order",
        "Explain how fusion makes new elements, with those heavier than iron made and spread by supernovae",
        "Compare planets, moons and artificial satellites, and explain (Higher tier) why a circular orbit has constant speed but changing velocity, and why orbit radius changes with speed",
    ], "Match the life cycle to the mass: a Sun-sized star ends as a red giant then a white dwarf; only a much more massive star goes supernova. When explaining stability, name both forces — gravity inwards and pressure from fusion outwards — and say they are balanced."),

    ("phys_aqa:4.8.2", &[
        "Describe red-shift as an increase in the wavelength of light from receding galaxies, and explain it qualitatively",
        "State that more distant galaxies show bigger red-shifts because they are moving away faster",
        "Explain how this pattern is evidence that space itself is expanding",
        "Explain how red-shift supports the Big Bang theory of a universe that began very small, extremely hot and dense",
        "Describe how observations lead to theories, and explain that the accelerating expansion, dark mass and dark energy are still not understood",
    ], "Red-shift means the wavelength increases, not that the galaxy looks red. In Big Bang answers, link every step: galaxies moving apart now, closer together in the past, so a very small, hot and dense beginning."),
    // ---------- Economics (AQA GCSE 8136) ----------
    ("econ_aqa:3.1.1.1", &[
        "Distinguish needs from wants, and explain how both change over time",
        "Explain that the central purpose of economic activity is producing goods and services to satisfy needs and wants",
        "State the three key economic decisions: what, how and for whom to produce",
        "Explain how consumers, producers and government interact as the main economic groups",
    ], "A holiday, a phone or a games console is a want, not a need - and 'the government' is an economic group in its own right, not just a regulator of the other two."),

    ("econ_aqa:3.1.1.2", &[
        "Explain what makes something an economic resource",
        "Identify examples of land, labour, capital and enterprise in a named business",
        "State the reward to each factor: rent, wages, interest and profit",
    ], "Capital means man-made aids to production such as machinery and buildings, not money - 'income' and 'profit' are never factors of production."),

    ("econ_aqa:3.1.1.3", &[
        "Explain the basic economic problem of scarce resources and unlimited wants",
        "Explain how consumers, producers and government weigh up costs and benefits to make a choice",
        "Define opportunity cost and apply it to a named decision",
    ], "Opportunity cost is the single next best alternative given up, not the money spent and not every other possible use."),

    ("econ_aqa:3.1.2.1", &[
        "Define a market as any opportunity for buyers and sellers to interact to set a price",
        "Explain how markets allocate scarce resources through price signals",
        "Explain the difference between factor markets and product markets, with an example of each",
    ], "Factor markets trade factors of production such as labour; product markets trade goods AND services - 'goods versus services' is the answer the examiners mark wrong."),

    ("econ_aqa:3.1.2.2", &[
        "Classify activities into the primary, secondary and tertiary sectors",
        "Describe the relative sizes of the three sectors in the UK and how they have changed",
        "Distinguish a good from a service",
        "Calculate and explain the consequences of a change in the size of a sector",
    ], "The secondary sector is mainly manufacturing and construction - its decline means lost factory jobs and structural unemployment, not fewer shops."),

    ("econ_aqa:3.1.2.3", &[
        "Define specialisation and the division of labour",
        "Explain how and why individuals, firms and countries specialise, and why specialisation leads to exchange",
        "Explain the costs and benefits of the division of labour to the worker and to the firm",
    ], "Read who the question is about: a disadvantage 'for an individual worker' is boredom or redundancy risk, not the firm's loss of output."),

    ("econ_aqa:3.1.3.1", &[
        "Define demand as the quantity bought at a given price in a given time period",
        "Explain the factors that shift demand: income, related goods, tastes, advertising, population",
        "Construct an individual demand curve from consumer data",
        "Distinguish a movement along the demand curve from a shift of it",
    ], "Only a change in the good's own price moves along the curve; every other factor shifts it - and 'more demand' must say which way the curve moves."),

    ("econ_aqa:3.1.3.2", &[
        "Define supply and explain why the supply curve slopes upwards",
        "Explain the factors that shift supply: costs, technology, taxes and subsidies, number of firms, weather",
        "Construct an individual firm's supply curve from production data",
        "Distinguish a movement along the supply curve from a shift of it",
    ], "A rise in the cost of a raw material shifts SUPPLY to the left - students often shift demand instead and lose every diagram mark."),

    ("econ_aqa:3.1.3.3", &[
        "Explain how demand and supply determine equilibrium price and quantity on a diagram",
        "Explain why excess demand and excess supply push the price to equilibrium",
        "Analyse the effect of shifts in demand or supply on equilibrium price and quantity in real markets",
        "Show and calculate total revenue on a demand and supply diagram",
    ], "Say which curve shifts and in which direction before stating the price change - under half of candidates managed this in June 2025."),

    ("econ_aqa:3.1.3.4", &[
        "Define complementary and substitute goods, with examples",
        "Explain how a change in price, demand or supply in one market affects related markets",
        "Show the knock-on effect on a second demand and supply diagram",
    ], "A rise in the price of a good raises demand for its substitute but lowers demand for its complement - trace the chain one step at a time."),

    ("econ_aqa:3.1.3.5", &[
        "Calculate price elasticity of demand from given data and interpret the answer",
        "Distinguish price elastic from price inelastic demand",
        "Explain the factors that affect PED: substitutes, necessity, share of income, habit, time",
        "Explain the implications of PED for producers' pricing and revenue, and for consumers",
    ], "Divide the percentage change in QUANTITY by the percentage change in PRICE - reversing it, or using raw numbers instead of percentages, scores nothing."),

    ("econ_aqa:3.1.3.6", &[
        "Calculate price elasticity of supply from given data and interpret the answer",
        "Distinguish price elastic from price inelastic supply",
        "Explain the factors that affect PES: time, spare capacity, stocks, availability of inputs",
        "Explain the implications of PES for producers and consumers",
    ], "A PES question wants supply-side factors such as spare capacity or growing time - many candidates answer with PED factors like substitutes."),

    ("econ_aqa:3.1.4.1", &[
        "Explain business objectives: profit, sales growth, market share and survival",
        "Calculate total, average, fixed and variable costs, and total and average revenue",
        "Calculate profit and explain how firms raise it by cutting average costs or raising revenue",
        "Explain why higher prices give producers an incentive to expand production",
        "Evaluate conflicts between producers' motives and ethical and moral interests",
    ], "Business objectives are a firm's aims - 'low inflation' or 'economic growth' are government objectives and score nothing here."),

    ("econ_aqa:3.1.4.2", &[
        "Distinguish production (total output) from productivity (output per input)",
        "Calculate labour productivity and its percentage change from data",
        "Explain the factors that raise productivity: training, capital, specialisation, motivation, management",
        "Analyse the benefits of higher productivity for firms, workers and the economy",
    ], "More output is not higher productivity unless output per worker rises - examiners report candidates conflating the two every year."),

    ("econ_aqa:3.1.4.3", &[
        "Define economies of scale as the fall in average cost as production rises",
        "Explain managerial, purchasing, financial, technical and risk-bearing economies of scale",
        "Explain diseconomies of scale and why average costs can rise as a firm grows",
        "Evaluate the costs and benefits of growth for a business",
    ], "Economies of scale lower AVERAGE (unit) cost, not total cost - and making a wide range of products to spread risk is risk-bearing, not technical."),

    ("econ_aqa:3.1.5.1", &[
        "Explain that there is a range of market structures from competitive to monopoly",
        "Use the number of producers, product differentiation and ease of entry to distinguish them",
        "Calculate and interpret market shares",
    ], "Barriers to entry are LOW in a competitive market and HIGH in a concentrated one - swapping them is the error the examiners name."),

    ("econ_aqa:3.1.5.2", &[
        "Describe the main characteristics of a competitive market",
        "Explain how producers operate in a competitive market",
        "Explain the impact of competition on consumers, producers and workers",
        "Explain why profits are lower in a competitive market than in a concentrated one",
    ], "'State two ways' needs two different features - 'many firms' and 'few firms' are one point stated twice."),

    ("econ_aqa:3.1.5.3", &[
        "Define monopoly and oligopoly",
        "Explain how producers operate in a non-competitive market: price setting, differentiation, collusion",
        "Explain the causes of monopoly and oligopoly power, including barriers to entry",
        "Evaluate the consequences of monopoly power for consumers, including possible benefits from economies of scale and innovation",
    ], "A 15-marker asking about consumers wants effects on consumers - answers that drift to profits for producers or tax for government were weaker in 2025."),

    ("econ_aqa:3.1.5.4", &[
        "Explain wage determination using demand for and supply of labour",
        "Draw the effect of a change in the demand for a product on the labour market for its workers",
        "Explain wage differentials within and between occupations",
        "Distinguish gross from net pay and calculate net pay from deductions",
    ], "Demand for labour is derived from demand for the product, so more grocery sales shift the DEMAND for drivers right - not their supply."),

    ("econ_aqa:3.1.6.1", &[
        "Define market failure as the market's inability to allocate resources efficiently",
        "Explain the costs of a misallocation of resources",
        "Explain the methods of government intervention: taxes, subsidies, regulation, provision, information",
        "Evaluate how well an intervention corrects a misallocation",
    ], "Evaluate the tool, not just describe it - a tax on a good with price inelastic demand raises money but changes behaviour very little."),

    ("econ_aqa:3.1.6.2", &[
        "Define an externality as the gap between social and private costs or benefits",
        "Distinguish positive from negative externalities and identify them in context",
        "Explain how both production and consumption create negative externalities",
    ], "An externality falls on a THIRD party - a higher price paid by the buyer or a lower profit for the firm is a private cost, not an externality."),

    ("econ_aqa:3.2.1.1", &[
        "Explain what an interest rate is and why different loans and savings carry different rates",
        "Explain how changes in interest rates affect consumers' decisions to save, borrow and spend",
        "Explain how changes in interest rates affect producers' decisions to save, borrow and invest",
        "Calculate interest on savings, including for part of a year",
    ], "'How much interest' means the interest alone - adding back the deposit was the commonest error in June 2025, and loans cost more than mortgages because they carry more risk."),

    ("econ_aqa:3.2.1.2", &[
        "Identify the main sources of UK government revenue and main areas of spending",
        "Distinguish direct from indirect taxes, with examples",
        "Explain progressive, proportional and regressive taxation",
    ], "Regressive means a larger PERCENTAGE of a low income, even though the rich pay more pounds - VAT is the classic example."),

    ("econ_aqa:3.2.2.1", &[
        "State the principal economic objectives: full employment, price stability, growth and the balance of payments",
        "Explain other objectives such as reducing inequality and managing environmental change",
        "Analyse how a policy for one objective can harm another",
        "Evaluate how pursuing an objective affects different groups of people",
    ], "A conflict needs the chain: the policy, what it does to spending, and why that worsens the second objective - naming two objectives is not enough."),

    ("econ_aqa:3.2.2.2", &[
        "Explain what economic growth is and why it matters",
        "Distinguish GDP, real GDP and GDP per capita and calculate growth rates and per-capita figures",
        "Explain the causes, costs and benefits of economic growth",
        "Explain government policies to achieve growth",
    ], "Nominal GDP can rise while living standards fall - adjust for inflation (real GDP) and for population (per capita) before judging."),

    ("econ_aqa:3.2.2.3", &[
        "Calculate the unemployment rate and explain how employment and unemployment are measured",
        "Explain structural, seasonal, frictional and cyclical unemployment and their causes",
        "Explain the consequences of unemployment for individuals, firms, government and communities",
        "Evaluate government policies to reduce each type of unemployment",
    ], "The unemployment rate divides by the economically active (employed plus unemployed), not the whole working-age population - the wrong denominator cost most of the marks in 2025."),

    ("econ_aqa:3.2.2.4", &[
        "Define inflation and the rate of inflation",
        "Explain how the CPI measures inflation and calculate inflation from index figures",
        "Explain cost-push and demand-pull inflation",
        "Explain the consequences of inflation for savers, borrowers, workers, firms and government",
    ], "A falling inflation rate still means prices are rising, just more slowly - only a negative rate is deflation."),

    ("econ_aqa:3.2.2.5", &[
        "Explain the components of the current account and calculate its balance",
        "Explain the meaning and significance of a current account deficit or surplus",
        "Explain the reasons for a deficit or surplus, including productivity and the exchange rate",
        "Explain government policies to influence the balance of payments",
    ], "The current account balance is about trade and income flows with other countries - don't confuse it with the government's budget balance."),

    ("econ_aqa:3.2.2.6", &[
        "Describe the distribution of income and wealth in the UK",
        "Explain the causes of income and wealth inequality",
        "Explain the consequences of inequality",
        "Evaluate redistribution through taxation and government spending",
    ], "Income is a flow and wealth is a stock - a policy that taxes income does not touch the inequality in property and savings."),

    ("econ_aqa:3.2.3.1", &[
        "Explain how fiscal policy affects income and spending in the economy",
        "Explain how fiscal policy can be used to achieve government objectives",
        "Define a balanced budget, a budget deficit and a budget surplus, and explain their consequences",
        "Calculate the budget balance and express it as a percentage of GDP",
    ], "Calculate the budget balance as revenue minus spending, divide by GDP (not by spending) and round to the places asked."),

    ("econ_aqa:3.2.3.2", &[
        "Explain what monetary policy is and who carries it out in the UK",
        "Explain how interest rate changes are used to control inflation",
        "Explain how monetary policy can help achieve other objectives, and its limits",
    ], "Monetary policy is interest rates (and money supply) set by the Bank of England - tax and spending changes are fiscal policy."),

    ("econ_aqa:3.2.3.3", &[
        "Explain the supply-side policies named in the spec: education and training, lower direct taxes, lower taxes on profits, trade union reform, privatisation and deregulation",
        "Explain how supply-side policies help achieve growth, employment and competitiveness",
        "Evaluate the advantages and disadvantages of supply-side policies",
    ], "Supply-side policies work by raising productive capacity, so they are slow - say how long the effect takes and who pays for it."),

    ("econ_aqa:3.2.3.4", &[
        "Explain policies to reduce negative externalities: taxes, regulation, fines, information",
        "Explain policies to encourage positive externalities: subsidies, state provision, information",
        "Evaluate how effective each policy is in a given context",
    ], "A market-failure question wants market tools such as a tax, subsidy or labelling - answers about interest rates or growth policies scored nothing in 2025."),

    ("econ_aqa:3.2.4.1", &[
        "Explain why trade is important to economies",
        "Describe the main types of UK exports and imports",
        "Explain the advantages of trade and the consequences of global interdependence for the UK",
    ], "Services are the UK's largest export - answers that treat UK trade as only goods miss the point of the data."),

    ("econ_aqa:3.2.4.2", &[
        "Explain how exchange rates are determined by the demand for and supply of a currency",
        "Draw and label a change in the exchange rate on a currency diagram",
        "Explain the effects of appreciation and depreciation on consumers and producers",
    ], "More UK imports means more pounds SUPPLIED to buy foreign currency, so the supply curve shifts right and the pound falls."),

    ("econ_aqa:3.2.4.3", &[
        "Explain the arguments for and against free trade",
        "Explain what a free-trade agreement is and the significance of agreements such as the EU",
        "Analyse the impact of new free-trade agreements on UK consumers, producers and workers",
    ], "A free-trade agreement helps exporters AND brings more competition from imports - analysis needs both sides to reach the top level."),

    ("econ_aqa:3.2.4.4", &[
        "Explain the factors behind globalisation, including technology and multinational companies",
        "Explain the benefits and drawbacks of globalisation for producers, workers and consumers in the UK",
        "Explain the benefits and drawbacks of globalisation for less developed countries",
        "Evaluate the moral, ethical and sustainability issues in UK firms' overseas trade",
    ], "Apply the effect to the group named - a drawback 'for producers in developed countries' is cheaper foreign competition, not low wages abroad."),

    ("econ_aqa:3.2.5.1", &[
        "Explain the four functions of money with examples",
        "Explain why money is more than the notes and coins in circulation",
    ], "A price label shows money as a unit of account; paying for it shows a medium of exchange - match the function to what the money is doing in the example."),

    ("econ_aqa:3.2.5.2", &[
        "Identify the main agents in the financial sector: the Bank of England, commercial banks and building societies",
        "Explain the Bank of England's role in setting interest rates and keeping the financial system stable",
        "Explain how high street banks fund investment and serve savers and borrowers",
    ], "Keeping inflation at target is the Bank of England's job, not a high street bank's - and a building society is owned by its members, not shareholders."),
    // ---------- English Literature (WJEC Eduqas C720QS) ----------
    ("englit_edq:C1Aa", &[
        "Track the plot act by act, from the witches' prophecies to Malcolm's coronation, naming the turning points",
        "Explain how Shakespeare presents Macbeth, Lady Macbeth, Banquo, Macduff and the witches, and how each changes",
        "Explain the play's big ideas: ambition, guilt, kingship and tyranny, fate and free will, appearance and reality",
        "Use the Jacobean context (James I, the divine right of kings, witchcraft, the Gunpowder Plot) to explain audience response",
        "Learn short quotations from every act so you can range across the whole play without the text",
    ], "Eduqas Section A is closed book. A student who knows only the famous soliloquies runs out of evidence for Acts 4 and 5 in the essay question."),

    ("englit_edq:C1Ab", &[
        "Place the extract in the play in one sentence: who, where, just before and just after",
        "Answer the fixed question: how the characters speak and behave, and how an audience might respond",
        "Analyse language, verse and prose, stage directions and dramatic devices closely, word by word",
        "Explore more than one possible audience response, then and now",
        "Work through the whole extract in about 20 minutes and stay inside it",
    ], "The extract question is about the extract only. Wandering into the rest of the play earns nothing here, and answering several plays' extracts is a rubric breach examiners see every year."),

    ("englit_edq:C1Ac", &[
        "Plan an argument across the whole play in five minutes, choosing moments from the beginning, middle and end",
        "Write about how Shakespeare presents a character, relationship or theme, not just what happens",
        "Support each point with short, embedded quotations and analyse their language and dramatic effect",
        "Handle the common Eduqas forms: a character or theme 'and how Shakespeare presents it', and 'for which character do you have the most sympathy'",
        "Write accurately enough to earn the 5 AO4 marks for spelling, punctuation, vocabulary and sentence structures",
    ], "Retelling the story is the commonest reason essays stall in the middle bands. Every paragraph needs a method and an effect, not just an event."),

    ("englit_edq:C1Ba", &[
        "Explain the speaker's contrast between joy outdoors in summer and misery in the classroom",
        "Analyse the caged-bird and blighted-plant imagery and what it says about childhood and education",
        "Explain the five-line stanzas, the rhyme and the shift to direct appeal to the parents",
        "Place it in Blake's Romantic attack on institutions that crush natural freedom",
    ], "This is not a poem about a lazy child. The argument is that schooling destroys the growth it is meant to nurture, so the final stanzas' seasonal imagery is the point."),

    ("englit_edq:C1Bb", &[
        "Explain the speaker's move from loneliness to joy, and from the walk itself to memory",
        "Analyse the personification of the daffodils and the imagery of light, dance and abundance",
        "Explain the regular six-line stanzas, rhyme and rhythm and how the final stanza changes tense",
        "Place it in Wordsworth's Romanticism: nature, emotion and 'the inward eye' of memory",
    ], "The last stanza is the heart of the poem: the value of the daffodils comes later, in memory. An answer that stops at pretty flowers misses the 'wealth' the speaker only understands afterwards."),

    ("englit_edq:C1Bc", &[
        "Explain the speaker's story: a cottage girl seduced and cast off by a lord who then marries her cousin",
        "Analyse the dramatic monologue voice, the rhetorical questions and the shift from shame to defiance",
        "Explain the imagery of possession (the glove, the silken knot) and of purity and fallenness",
        "Use the Victorian context: the 'fallen woman', class power and the double standard",
    ], "The ending turns the poem: her son, his only heir, is her 'shame' and her 'pride'. Leaving out the final stanza throws away the speaker's triumph."),

    ("englit_edq:C1Bd", &[
        "Explain the speaker's argument: her thoughts of the beloved are no substitute for his presence",
        "Analyse the extended metaphor of the vine and the palm tree and how it is reversed",
        "Explain the Petrarchan sonnet form and the volta, and how enjambment and exclamation show feeling",
        "Place it in Sonnets from the Portuguese and her courtship with Robert Browning",
    ], "It is a love poem that ends by dismissing thought itself. Students who call it simply 'devoted' miss the paradox of the last line: being near him is better than thinking of him."),

    ("englit_edq:C1Be", &[
        "Explain what happens to Hodge and how the poem moves from burial to permanent belonging to a foreign land",
        "Analyse the Afrikaans landscape words and the strange stars, and what they show about displacement",
        "Explain the three regular stanzas and how the ending turns loss into a kind of transformation",
        "Use the context of the Boer War and Hardy's anti-heroic view of ordinary soldiers",
    ], "Hodge is a nickname for a country labourer: his name is part of the point. Treating him as a glorious hero reverses Hardy's purpose."),

    ("englit_edq:C1Bf", &[
        "Explain the contrast between the young man's life before the war and his life now, disabled and ignored",
        "Analyse the imagery of loss, colour and youth, and the shifts of time between stanzas",
        "Explain the irregular stanzas and the questions that end the poem",
        "Place it in Owen's First World War experience and his attack on recruitment and public indifference",
    ], "Owen blames vanity and pressure to enlist as much as the war itself. Missing why the young man joined up loses half of Owen's argument."),

    ("englit_edq:C1Bg", &[
        "Explain the speaker's longing to return home and what home means to him",
        "Analyse the sensory and natural imagery, colour and sound",
        "Explain the sonnet form and the repetition of the title phrase, and how the final couplet changes the mood",
        "Place it in McKay's life: born in Jamaica, writing in America during the Harlem Renaissance",
    ], "The joyful imagery is shadowed by the last line's pain. Answers that read it only as happy nostalgia miss the exile behind it."),

    ("englit_edq:C1Bh", &[
        "Explain the speaker's change of view about his photograph of a sleeping beggar in Bombay",
        "Analyse the imagery that turns the man into stone and fossil, and the passers-by's indifference",
        "Explain the title's double meaning and the poem's turn from 'then' to 'now'",
        "Discuss the context of poverty, art and the outsider's eye",
    ], "The poem criticises the speaker as much as the city. Calling it a simple poem about poverty ignores the guilt in the final lines."),

    ("englit_edq:C1Bi", &[
        "Explain the two stanzas: the birth of a daughter and a later argument with her as a teenager",
        "Analyse the imagery of the rope, the room and the struggle to become separate",
        "Explain the free verse, enjambment and direct address to the child",
        "Place it in Clarke's work as a Welsh poet writing about family and motherhood",
    ], "The conflict is loving, not hostile. The best answers show that the 'struggle' and the love are the same thing."),

    ("englit_edq:C1Bj", &[
        "Explain the childhood experience of picking blackberries and watching them rot",
        "Analyse the sensory imagery of taste, blood and greed, and the shift to decay",
        "Explain the two-part structure, the couplets and half-rhyme, and the adult voice at the end",
        "Place it in Heaney's rural Northern Irish childhood and his theme of lost innocence",
    ], "The poem is about desire and disappointment, not just fruit. Answers that never reach the final couplet's lesson about hope miss the point."),

    ("englit_edq:C1Bk", &[
        "Explain the story: a kamikaze pilot turns back and is shunned by his family and community",
        "Analyse the natural imagery of sea, fish and childhood that draws him home",
        "Explain the third-person framing, the daughter's voice and the shift into direct speech",
        "Use the context of Japanese honour codes and the kamikaze missions in the Second World War",
    ], "He survives, but his family treats him as if he had died. The poem's real conflict is the silence afterwards, not the flight."),

    ("englit_edq:C1Bl", &[
        "Explain the photographer's work in the darkroom and his memories of the war zones",
        "Analyse the religious imagery, the developing photograph and the contrast with rural England",
        "Explain the regular stanzas and rhyme as order imposed on chaos",
        "Discuss the context of war photography and the indifference of newspaper readers",
    ], "The last stanza turns on the readers who glance at the pictures and forget. An answer that ends with the photographer's trauma misses Duffy's accusation."),

    ("englit_edq:C1Bm", &[
        "Explain the speaker's anxious wait for a lover's call",
        "Analyse the imagery of disaster, waiting and the phone as an object of devotion",
        "Explain the long lines, the free verse and the breathless, broken final lines",
        "Discuss love as obsession and loneliness in a modern relationship",
    ], "The poem is funny and painful at once. Readers who see only misery, or only comedy, flatten the tone the poem depends on."),

    ("englit_edq:C1Bn", &[
        "Explain the soldier's account of a killing and how the memory haunts him",
        "Analyse the colloquial voice, the violent imagery and the repeated 'probably armed, possibly not'",
        "Explain how the structure shifts from the event to its aftermath at home",
        "Place it in Armitage's work with soldiers' testimony from recent conflicts and the reality of trauma",
    ], "The killing takes a few stanzas; the guilt takes the rest. That imbalance is the poem's argument about post-traumatic stress."),

    ("englit_edq:C1Bo", &[
        "Explain the speaker's account of how her parents met and what their love was like",
        "Analyse the extended metaphor of love as a comic book: fragile, handled, worn",
        "Explain the conversational free verse, lower-case style and the shift from story to reflection",
        "Place it in Ewing's work as a Chicago writer of poetry and comics",
    ], "The ending is not simply sad. The love did not last, but the speaker values it, which is why it has 'a good ending'."),

    ("englit_edq:C1Bp", &[
        "Group the fifteen poems by theme: conflict and war, love and relationships, childhood and family, place and belonging, memory, nature",
        "For each poem, name two others it can be compared with and the link between them",
        "Know each poem's context in a sentence or two: poet, period and situation",
        "Know which poems share forms and methods: sonnets, dramatic monologues, free verse, narrative",
    ], "Question 2 names a theme and asks you to choose the second poem. Pre-planned pairings save the minutes you need to write."),

    ("englit_edq:C1Bq", &[
        "Answer the first question on a printed poem in about 20 minutes, covering content, methods and context",
        "Choose the best second poem for the comparison quickly, by the theme in the question",
        "Compare content, structure, methods and contexts inside each paragraph, not poem by poem",
        "Write about the chosen poem from memory with short quotations you have learned",
    ], "In the comparison you write about your chosen poem from memory. Students who learned no quotations from it are capped however good the analysis of the printed poem is."),

    ("englit_edq:C2Aa", &[
        "Summarise the three acts and the Inspector's questioning of each character in turn",
        "Explain how Priestley presents the Birlings, Gerald, Eva Smith and the Inspector",
        "Explain the play's ideas: responsibility, class, gender, age and the gap between the generations",
        "Use the 1912 setting and 1945 first performance to explain dramatic irony and Priestley's message",
        "Analyse stagecraft: lighting, entrances and exits, timing, the doorbell and the final telephone call",
    ], "Priestley is arguing, not just storytelling. Saying what he wants the 1945 audience to think lifts an answer above plot."),

    ("englit_edq:C2Ab", &[
        "Use the printed extract as a launch pad, then range across the whole text",
        "Answer the 'write about X and how they are presented / important to the text as a whole' question with an argument",
        "Analyse language, structure and form, including dramatic methods for plays",
        "Write in clear, accurate, varied sentences for the 5 AO4 marks",
        "Finish a 40-mark answer in about 45 minutes",
    ], "There are no AO3 context marks in this section. A long paragraph of history earns nothing unless it explains a choice the writer makes."),

    ("englit_edq:C2Ba", &[
        "Summarise the five staves and how Scrooge changes in each",
        "Explain how Dickens presents Scrooge, the ghosts, the Cratchits, Fred and other characters",
        "Explain the novel's ideas: poverty and social responsibility, redemption, family, Christmas and generosity",
        "Use the 1840s context: the Poor Law, workhouses, child poverty and Dickens's purpose",
        "Learn short quotations from every stave",
    ], "Scrooge's change is gradual. Answers that jump from 'mean' to 'generous' miss every mark for how Dickens structures the transformation."),

    ("englit_edq:C2Bb", &[
        "Analyse the printed extract closely, then link it to other moments across the novel",
        "Answer the 'write about X and how they are presented at different points' question with a clear argument",
        "Use context (AO3) to explain the writer's purpose, woven into your points",
        "Balance AO1, AO2 and AO3, which carry equal weight in this question",
        "Finish a 40-mark answer in about 45 minutes",
    ], "Context counts for a third of the marks here, unlike Section A. Students who leave it out, or bolt it on in a separate paragraph, lose marks they could have had."),

    ("englit_edq:C2Ca", &[
        "Read the unseen poem twice and summarise what it is about in a sentence",
        "Explain what the poem is about and how it is organised",
        "Explain the ideas the poet may want readers to think about",
        "Analyse words, phrases and images and their effects, and give your own response",
        "Spend about 20 minutes on the 15-mark question",
    ], "The question asks for the poem's effect on you. Feature-spotting without saying how it makes you feel or think caps the mark."),

    ("englit_edq:C2Cb", &[
        "Compare what the two poems are about and how they are organised",
        "Compare the ideas the poets want readers to think about",
        "Compare their choices of words, phrases and images and the effects they create",
        "Compare your response to the two poems",
        "Spend about 40 minutes on the 25-mark comparison",
    ], "The second question is a comparison. Writing about the new poem alone, without linking back to the first, cannot reach the top bands."),
    // ---------- Business (Pearson Edexcel GCSE 1BS0) ----------
    ("bus_edx:1.1.1", &[
        "Explain why new business ideas come about: changes in technology, changes in what consumers want, and products or services becoming obsolete",
        "Explain how new ideas come about: original ideas, and adapting existing products, services or ideas",
        "Apply these causes to a given start-up and say which mattered most",
    ], "Saying a product 'became obsolete' is not an explanation. Say what replaced it, why customers switched, and what opportunity that opened for a new business."),

    ("bus_edx:1.1.2", &[
        "Explain the risks of starting a business: business failure, financial loss and lack of security",
        "Explain the rewards: business success, profit and independence",
        "Analyse why an entrepreneur accepts the risk, and how a given owner is affected by both risk and reward",
    ], "'Lack of security' means giving up a steady wage and having no guaranteed income. It has nothing to do with security guards or cyber attacks, and examiners reported many students misreading it in 2025."),

    ("bus_edx:1.1.3", &[
        "Explain the purpose of business activity: to produce goods or services, to meet customer needs and to add value",
        "Explain the five ways to add value in the spec: convenience, branding, quality, design and a unique selling point",
        "Describe the role of the entrepreneur: organising resources, making business decisions and taking risks",
        "Explain how added value lets a business charge more than its inputs cost",
    ], "Added value is a money idea: the gap between the selling price and the cost of the inputs. An answer that ends at 'more customers' instead of 'a higher price than the inputs cost' stalls at two marks."),

    ("bus_edx:1.2.1", &[
        "Identify the customer needs in the spec: price, quality, choice and convenience",
        "Explain why identifying and understanding customers generates sales and helps a business survive",
        "Judge which need matters most to the customers in a given case",
    ], "Choose the need that fits the case. A budget takeaway's customers want price and convenience; claiming they want premium quality ignores the context and loses the application marks."),

    ("bus_edx:1.2.2", &[
        "Explain the purposes of market research: understanding customer needs, finding gaps in the market, reducing risk and informing decisions",
        "Compare primary methods (survey, questionnaire, focus group, observation) with secondary sources (internet, market reports, government reports)",
        "Distinguish qualitative from quantitative data and explain how social media is used to collect research data",
        "Evaluate the reliability of market research data: sample size, bias and how up to date it is",
    ], "Primary research is not automatically accurate. It is only as reliable as its sample and its questions, and it costs a small business time and money to collect."),

    ("bus_edx:1.2.3", &[
        "Segment a market by location, demographics, lifestyle, income and age",
        "Explain how segmentation helps a business target its customers",
        "Draw and read a market map, and use it to find a gap in the market and see the competition",
        "Explain why a gap on a market map may not be a profitable opportunity",
    ], "A gap on a market map may be empty because nobody wants that combination. Say the gap needs research to confirm demand before you recommend filling it."),

    ("bus_edx:1.2.4", &[
        "Assess competitors' strengths and weaknesses on price, quality, location, product range and customer service",
        "Explain how competition affects a business's decisions on price, product, promotion and costs",
        "Explain how customers benefit from competition",
    ], "Competition questions want a business decision, not just 'they lose customers'. Say what the business would change (prices, service, its product) and what that costs."),

    ("bus_edx:1.3.1", &[
        "Explain the difference between business aims and business objectives",
        "Explain the financial aims and objectives: survival, profit, sales, market share and financial security",
        "Explain the non-financial aims and objectives: social objectives, personal satisfaction, challenge, independence and control",
        "Explain why aims and objectives differ between businesses",
    ], "On this spec, sales and market share are financial objectives, while independence and challenge are non-financial. Edexcel's multiple-choice questions test exactly this split."),

    ("bus_edx:1.3.2a", &[
        "Calculate revenue, fixed costs, variable costs and total costs",
        "Calculate profit or loss",
        "Calculate the interest on a loan in pounds and as a percentage",
        "Explain how a change in price, output or costs changes profit",
    ], "Formulae are not given and a 'calculate' answer earns its two marks only if it is right. Learn interest (%) = (total repayment - amount borrowed) / amount borrowed × 100 by heart."),

    ("bus_edx:1.3.2b", &[
        "Calculate break-even output as fixed costs / (selling price - variable cost per unit), and break even in revenue",
        "Calculate the margin of safety from actual or budgeted sales",
        "Read a break-even diagram: fixed cost, total cost and revenue lines, the break-even point, and the profit and loss areas",
        "Explain how a change in price or costs moves the break-even point",
    ], "Be ready to work the formula backwards. In 2025 a question gave the break-even output and asked for fixed costs (break-even output × (price - variable cost per unit)), and most students could not do it."),

    ("bus_edx:1.3.3", &[
        "Explain why cash matters: paying suppliers, overheads and employees, and avoiding insolvency",
        "Explain the difference between cash and profit",
        "Complete and interpret a cash-flow forecast: inflows, outflows, net cash flow, opening and closing balances",
        "Explain how a forecast helps a small business spot and plan for a shortfall",
    ], "A cash-flow forecast does not show profit. Examiners gave zero in 2025 to answers that said it is used to work out profit or break even."),

    ("bus_edx:1.3.4", &[
        "Explain the short-term sources of finance: overdraft and trade credit",
        "Explain the long-term sources: personal savings, venture capital, share capital, loans, retained profit and crowdfunding",
        "Match a source of finance to the need, the amount, the cost and the owner's wish to keep control",
        "Explain why a new start-up rarely has retained profit to use",
    ], "Match the source to the purpose. An overdraft to buy a van, or a five-year loan to cover one customer paying late, is a mismatch the examiner will not credit."),

    ("bus_edx:1.4.1", &[
        "Explain limited and unlimited liability and what each means for the owner's personal assets",
        "Compare sole traders, partnerships and private limited companies, with the advantages and disadvantages of each",
        "Explain the advantages and disadvantages of starting as a franchise",
    ], "Generic disadvantages ('it is hard work') earn nothing. Name a feature that belongs to that type of ownership, such as unlimited liability for a sole trader, and develop it."),

    ("bus_edx:1.4.2", &[
        "Explain the pull of proximity to the market, labour, materials and competitors",
        "Explain how the nature of the business activity shapes the location decision",
        "Explain how the internet affects location: e-commerce, fixed premises or both",
        "Judge which factor matters most for a given business",
    ], "If the question names one factor, write only about that factor. In 2025, students who drifted from proximity to the market on to suppliers had half their answer ignored."),

    ("bus_edx:1.4.3", &[
        "Explain each element of the marketing mix: price, product, promotion and place",
        "Explain how the elements are balanced to suit the competitive environment",
        "Explain how changing consumer needs and technology (e-commerce, digital communication) change the mix",
    ], "E-commerce is a way of selling (place), not a way of promoting. Examiners give no credit when it is described as advertising."),

    ("bus_edx:1.4.4", &[
        "Describe what a business plan contains: the idea, aims and objectives, target market, forecast revenue, cost and profit, cash-flow forecast, sources of finance, location and marketing mix",
        "Explain how a business plan reduces risk",
        "Explain how a business plan helps the owner obtain finance",
        "Explain the limits of a plan: forecasts can be wrong and plans go out of date",
    ], "A business plan reduces risk; it does not remove it or guarantee a loan. A lender trusts a plan only as far as its forecasts are realistic."),

    ("bus_edx:1.5.1", &[
        "Identify the stakeholders in the spec and their objectives: shareholders, employees, customers, managers, suppliers, local community, pressure groups and government",
        "Explain how stakeholders are affected by business activity",
        "Explain how stakeholders influence business activity",
        "Analyse conflicts between stakeholder groups",
    ], "Stakeholders and shareholders are not the same thing. A conflict answer must name two groups and explain why their objectives clash."),

    ("bus_edx:1.5.2", &[
        "Explain the types of technology in the spec: e-commerce, social media, digital communication and payment systems",
        "Explain how technology affects sales, costs and the marketing mix",
        "Weigh the costs and drawbacks of new technology for a small business",
    ], "Technology is not free. Strong answers include its set-up, training or fee costs (such as card-payment fees or website upkeep) as well as the benefits."),

    ("bus_edx:1.5.3", &[
        "Explain the principles of consumer law: quality and consumer rights",
        "Explain the principles of employment law: recruitment, pay, discrimination, and health and safety",
        "Explain the cost to a business of meeting legislation",
        "Explain the consequences of meeting and of not meeting its legal obligations",
    ], "Edexcel tests principles, not Acts. You do not need Act names or dates, but you do need to say what the business must do and what happens to it if it does not."),

    ("bus_edx:1.5.4", &[
        "Explain how unemployment and changing levels of consumer income affect businesses",
        "Explain how inflation and changes in interest rates affect businesses",
        "Explain how changes in government taxation and exchange rates affect businesses",
        "Judge how strongly a given business is affected, depending on its products and customers",
    ], "Exchange-rate chains are easy to get backwards. A stronger pound makes imports cheaper and exports dearer: write the chain out step by step and check it."),

    ("bus_edx:1.5.5", &[
        "Explain possible responses by a business to changes in technology",
        "Explain possible responses to changes in legislation",
        "Explain possible responses to changes in the economic climate",
        "Evaluate which response suits a given business best",
    ], "The question asks about the business's response, not just the change. Name a realistic action the owner would take and weigh what it costs."),

    ("bus_edx:2.1.1", &[
        "Explain internal (organic) growth: new products through innovation and research and development, and new markets through the marketing mix, technology or expanding overseas",
        "Explain external (inorganic) growth: mergers and takeovers",
        "Explain what a public limited company is, and the advantages and disadvantages of becoming one",
        "Compare internal sources of finance for growth (retained profit, selling assets) with external ones (loan capital, share capital, stock market flotation)",
    ], "A merger is agreed between two firms; a takeover is one firm buying control of another. Mixing them up, or calling a plc 'owned by the public', loses easy marks."),

    ("bus_edx:2.1.2", &[
        "Explain why aims and objectives change: market conditions, technology, performance, legislation and internal reasons",
        "Explain how they change: survival or growth, entering or exiting markets, growing or reducing the workforce, increasing or decreasing the product range",
        "Apply both to a business at a given stage of its life",
    ], "Always give the reason with the change. 'It switched from growth to survival because a new rival cut prices and its sales fell' scores; a list of possible objectives does not."),

    ("bus_edx:2.1.3", &[
        "Explain the impact of imports (competition from overseas, buying from overseas) and exports",
        "Explain why businesses change location and what multinationals are",
        "Explain the barriers to international trade: tariffs and trade blocs",
        "Explain how businesses compete internationally through the internet, e-commerce and a changed marketing mix",
    ], "A tariff is a tax on imports. It makes imported goods dearer but does not ban them; a ban or a limit on quantity is a different measure and is not on this spec."),

    ("bus_edx:2.1.4", &[
        "Explain how ethical considerations influence business activity, and the trade-off between ethics and profit",
        "Explain how environmental considerations and sustainability influence business activity, and their trade-off with profit",
        "Explain how pressure group activity can affect the marketing mix",
    ], "Ethical does not mean legal: an ethical choice goes beyond what the law requires. A high mark needs the cost side of the trade-off as well as the gain in reputation."),

    ("bus_edx:2.2.1", &[
        "Explain the design mix (function, aesthetics, cost) and balance it for a given product",
        "Describe the phases of the product life cycle",
        "Explain extension strategies and when to use them",
        "Explain why differentiating a product or service matters",
    ], "Draw the product life cycle with sales on the vertical axis against time, and name the phases correctly: development, introduction, growth, maturity, decline."),

    ("bus_edx:2.2.2", &[
        "Explain the main pricing strategies, such as penetration, skimming, competitive, cost-plus, premium and psychological pricing",
        "Explain the influences on pricing strategy: technology, competition, market segments and the product life cycle",
        "Recommend a pricing strategy for a given product and justify it",
    ], "Name the strategy and tie it to the influence in the case. 'Skimming, because the product is new and has no rivals yet' scores; a definition of skimming on its own does not."),

    ("bus_edx:2.2.3", &[
        "Explain promotion strategies for different market segments: advertising, sponsorship, product trials, special offers and branding",
        "Explain how technology is used in promotion: targeted online advertising, viral advertising via social media and e-newsletters",
        "Choose a promotion method for a given segment and justify it",
    ], "Match the method to the segment and the budget. National TV advertising for a small café's local customers is not a credible recommendation."),

    ("bus_edx:2.2.4", &[
        "Explain the methods of distribution in the spec: retailers and e-tailers (e-commerce)",
        "Compare selling through retailers with selling online",
        "Recommend a distribution method for a given business",
    ], "Place means how the product reaches the customer. Answers about where the shop is (location) instead of the channel miss the point."),

    ("bus_edx:2.2.5", &[
        "Explain how each element of the marketing mix influences the others",
        "Explain how the marketing mix is used to build competitive advantage",
        "Explain how an integrated marketing mix supports competitive advantage",
    ], "An integrated mix means the four Ps agree. A premium product discounted through a bargain retailer is inconsistent; say which element breaks the pattern."),

    ("bus_edx:2.3.1", &[
        "Explain the purpose of business operations: producing goods and providing services",
        "Compare job, batch and flow production and their effects on productivity, costs and prices",
        "Explain how technology in production balances cost, productivity, quality and flexibility",
    ], "Productivity is output per worker or per hour, not total output. More output from more staff is not higher productivity."),

    ("bus_edx:2.3.2", &[
        "Read and interpret a bar gate stock graph: maximum stock, buffer stock, reorder level, reorder quantity and lead time",
        "Explain just in time (JIT) stock control and its advantages and disadvantages",
        "Explain procurement and what makes a good supplier relationship: quality, delivery (cost, speed, reliability), availability, cost and trust",
        "Explain how logistics and supply decisions affect costs, reputation and customer satisfaction",
    ], "On a bar gate graph, lead time is the time from placing an order to its arrival, read along the time axis; students often give a stock level instead."),

    ("bus_edx:2.3.3", &[
        "Explain quality control and quality assurance and the difference between them",
        "Explain why quality matters for both goods and services",
        "Explain how managing quality helps a business control costs and gain a competitive advantage",
    ], "Quality control inspects finished output; quality assurance builds checks into every stage so faults are prevented. Swapping them is the commonest error."),

    ("bus_edx:2.3.4", &[
        "Explain the parts of the sales process: product knowledge, speed and efficiency of service, customer engagement, responses to customer feedback and post-sales service",
        "Explain why good customer service matters to a business",
        "Apply the sales process to a given business",
    ], "Develop a customer-service point into a business gain: loyal, repeat customers and good reviews bring more sales without extra spending on promotion."),

    ("bus_edx:2.4.1", &[
        "Calculate gross profit and net profit",
        "Calculate the gross profit margin and the net profit margin",
        "Calculate the average rate of return on an investment",
        "Interpret what the results show about a business",
    ], "Margins are divided by sales revenue, not by costs. Edexcel often asks for two decimal places, and rounding early or giving a whole number loses the mark."),

    ("bus_edx:2.4.2", &[
        "Use information from graphs and charts to support, inform and justify a decision",
        "Use financial data, marketing data and market data to judge business performance",
        "Explain the limitations of financial information in understanding performance and making decisions",
    ], "Quote the figures and do something with them: a change, a percentage or a comparison. Copying numbers from the table is not analysis."),

    ("bus_edx:2.5.1", &[
        "Compare hierarchical and flat structures, and centralised and decentralised ones, and say when each is appropriate",
        "Explain the importance of effective communication, the impact of too little or too much communication, and the barriers to it",
        "Explain different ways of working: part-time, full-time and flexible hours; permanent, temporary and freelance contracts",
        "Explain how technology affects ways of working: efficiency and remote working",
    ], "'Excessive communication' is on the spec: too many emails and meetings waste time and lower motivation. Students tend to assume more communication is always better."),

    ("bus_edx:2.5.2", &[
        "Explain the key job roles and their responsibilities: directors, senior managers, supervisors or team leaders, and operational and support staff",
        "Explain the recruitment documents: job description, person specification, application form and CV",
        "Compare internal and external recruitment and choose one for a given business need",
    ], "The job description describes the job; the person specification describes the ideal person for it. Edexcel's multiple-choice questions test this pair regularly."),

    ("bus_edx:2.5.3", &[
        "Explain the ways of training and developing staff: formal and informal training, self-learning, ongoing training, target setting and performance reviews",
        "Explain the link between training, motivation and retention",
        "Explain why businesses retrain staff to use new technology",
    ], "Link the benefit of training to the business: lower staff turnover cuts recruitment costs, and better skills raise productivity and quality."),

    ("bus_edx:2.5.4", &[
        "Explain why motivation matters: attracting employees, retaining them and productivity",
        "Explain the financial methods: remuneration, bonus, commission, promotion and fringe benefits",
        "Explain the non-financial methods: job rotation, job enrichment and autonomy",
        "Recommend a method of motivation for a given workforce and justify it",
    ], "Job rotation moves workers between tasks of similar difficulty; job enrichment gives them more challenging, responsible work. Confusing the two is a frequent slip."),
    // ---------- Computer Science (AQA GCSE 8525) ----------
    ("cs_aqa:3.1.1a", &[
        "Explain the terms algorithm, decomposition and abstraction, and why an algorithm is not the same as a program",
        "Read and write algorithms in AQA pseudo-code: assignment with ←, OUTPUT, USERINPUT, IF, WHILE, REPEAT and FOR",
        "Read and draw flowcharts using the terminal, process, input/output and decision symbols",
        "Apply decomposition and abstraction to a described problem, such as a game or a booking system",
    ], "Abstraction is removing unnecessary detail; decomposition is breaking a problem into sub-problems. Swapping the two definitions is the commonest lost mark in 3.1."),

    ("cs_aqa:3.1.1b", &[
        "Identify where inputs, processing and outputs happen in a given algorithm",
        "Complete a trace table for an algorithm with selection, loops and arrays",
        "Determine the purpose of a simple algorithm by tracing it or by inspection",
        "Use DIV and MOD correctly while tracing",
    ], "A trace table records a new row only when a value changes. Students who re-copy unchanged values or skip the final value of the loop variable lose the accuracy marks."),

    ("cs_aqa:3.1.2", &[
        "Explain that more than one algorithm can solve the same problem",
        "Compare two algorithms for the same task by how much time they take, counting comparisons or loop passes",
        "Explain how a change such as stopping a loop early makes an algorithm more efficient",
    ], "Exam questions on efficiency mean time efficiency only. 'It uses less code' is not the same as 'it is more efficient'."),

    ("cs_aqa:3.1.3", &[
        "Explain step by step how a linear search works",
        "Explain step by step how a binary search works, and list the items it examines",
        "Compare linear and binary search: sorted data, speed on large lists, simplicity",
    ], "Binary search only works on sorted data. Leaving that condition out of a comparison answer is the classic missed mark."),

    ("cs_aqa:3.1.4", &[
        "Explain how merge sort works: splitting to single items, then merging in order",
        "Explain how bubble sort works and show the list after each pass",
        "Compare merge and bubble sort: speed on large lists, memory use, ease of coding",
    ], "Merge sort is not 'splitting and then sorting the halves': the order comes from the merging. Say how two lists are merged by comparing their first items."),

    ("cs_aqa:3.2.1", &[
        "Explain what a data type is and why choosing the right one matters",
        "Choose integer, real, Boolean, character or string for a given item of data",
        "Recognise that languages may use other names, such as float for real",
    ], "Telephone numbers and codes with leading zeros are strings, not integers, because they are never calculated with and the zero would be lost."),

    ("cs_aqa:3.2.2a", &[
        "Use variable and constant declarations and assignment, and explain why named constants are used",
        "Write programs using sequence and selection, including nested IF and ELSE IF",
        "Explain why meaningful identifier names matter for variables, constants and subroutines",
        "Interpret and write algorithms that combine these statement types",
    ], "A constant's value cannot change while the program runs. 'It never changes' scores only if you add why that helps: no accidental changes, and one place to edit."),

    ("cs_aqa:3.2.2b", &[
        "Distinguish definite (count-controlled) from indefinite (condition-controlled) iteration",
        "Use condition-controlled loops with the test at the start (WHILE) and at the end (REPEAT…UNTIL)",
        "Write and trace nested iteration and loops inside selection",
        "Choose the right type of loop for a described task",
    ], "A REPEAT…UNTIL loop always runs at least once, but a WHILE loop may not run at all. Questions that ask for the difference want exactly that."),

    ("cs_aqa:3.2.3", &[
        "Use addition, subtraction, multiplication and real division in programs",
        "Use integer division (DIV) and remainder (MOD), and work out their values",
        "Apply DIV and MOD to problems such as converting minutes to hours, or testing for even numbers",
    ], "11 DIV 2 is 5 and 11 MOD 2 is 1. Writing 5.5 for DIV, or 0.5 for MOD, loses the mark every year."),

    ("cs_aqa:3.2.4", &[
        "Use equal to, not equal to, less than, greater than, less than or equal to and greater than or equal to",
        "Interpret relational operators in conditions within given algorithms",
        "Choose the correct operator for a range check at the boundaries",
    ], "Using < where ≤ is needed rejects a valid boundary value: a logic error examiners build questions around."),

    ("cs_aqa:3.2.5", &[
        "Use NOT, AND and OR, and combinations of them, in conditions",
        "Work out the result of a compound condition for given values",
        "Write conditions for loops and selection that combine several tests",
    ], "x > 1 AND < 10 is not a valid condition. Each side of AND or OR needs its own complete comparison: x > 1 AND x < 10."),

    ("cs_aqa:3.2.6", &[
        "Explain what a data structure is and why one is useful",
        "Use one-dimensional arrays, including looping through them with an index",
        "Use two-dimensional arrays, accessing elements as array[row][column]",
        "Define and use records, and arrays of records, to store related data of different types",
    ], "Arrays are indexed from 0 in AQA papers, so the last index is LEN(array) - 1. Looping to LEN(array) is the most common error in array code."),

    ("cs_aqa:3.2.7", &[
        "Obtain input from the keyboard and store it in a variable",
        "Convert input to the right data type before calculating with it",
        "Output text, variables and calculated values to the screen in a clear format",
    ], "Keyboard input arrives as a string. Code that adds two inputs without converting them joins them instead: '2' + '3' gives '23'."),

    ("cs_aqa:3.2.8", &[
        "Use string length, position, substring and concatenation",
        "Convert characters to character codes and back",
        "Convert between strings and integers or reals in both directions",
        "Apply string handling to tasks such as checking the format of a code or building a username",
    ], "AQA's SUBSTRING(start, end, string) includes both end positions. Counting from 1, or excluding the end character, gives the wrong substring."),

    ("cs_aqa:3.2.9", &[
        "Generate random integers within a given range in programs",
        "Use random numbers in programs such as dice games and quizzes",
        "Make sure both ends of the range are included as the task requires",
    ], "RANDOM_INT(1, 6) includes 6, but Python's randrange(1, 6) does not. Getting the range off by one is the mark that is dropped."),

    ("cs_aqa:3.2.10", &[
        "Explain what a subroutine is and the advantages of using subroutines",
        "Describe how parameters pass data into a subroutine and how return values pass data out",
        "Explain what local variables are, and why using them is good practice",
        "Describe the structured approach to programming and explain its advantages",
    ], "A function returns a value and a procedure does not. In code questions, returning a value but never using it in the calling statement loses the mark."),

    ("cs_aqa:3.2.11a", &[
        "Write validation routines: presence, length and range checks on input",
        "Write a simple authentication routine with a username and password",
        "Explain why validation and authentication make programs robust and secure",
    ], "Validation checks data is sensible, not that it is correct. A range check accepts any age from 11 to 16, even if it is the wrong age."),

    ("cs_aqa:3.2.11b", &[
        "Explain the purpose of testing algorithms and programs",
        "Describe normal, boundary and erroneous test data, and choose examples for a given range",
        "Distinguish syntax errors from logic errors, and identify and correct both in code",
    ], "Boundary data sits on and either side of the limit. For 1 to 10, test 0, 1, 10 and 11. Giving only one side loses the mark."),

    ("cs_aqa:3.3.1", &[
        "Explain decimal, binary and hexadecimal as base 10, base 2 and base 16",
        "Explain that computers use binary for all data and instructions, and that a bit pattern can mean different things",
        "Explain why hexadecimal is used in computer science",
    ], "Hexadecimal is for people, not computers. Saying it saves memory or is faster to process scores nothing: the computer still stores binary."),

    ("cs_aqa:3.3.2", &[
        "Convert between binary and decimal for values from 0 to 255",
        "Convert between binary and hexadecimal using nibbles",
        "Convert between decimal and hexadecimal in both directions",
    ], "Write 8-bit answers with all eight bits, leading zeros included, and show working when asked. A right answer with no working can still drop a mark."),

    ("cs_aqa:3.3.3", &[
        "Define the bit and the byte",
        "State the decimal prefixes kilo, mega, giga and tera, each 1,000 times the one before",
        "Convert and compare quantities of data between bits, bytes and the prefixes",
    ], "Divide by 8 to go from bits to bytes before dividing by 1,000s. AQA uses 1 kB = 1,000 bytes, so 1,024 gives the wrong answer."),

    ("cs_aqa:3.3.4", &[
        "Add up to three 8-bit binary numbers, carrying correctly",
        "Apply a logical left or right shift to an 8-bit number",
        "Explain that shifts multiply or divide by powers of 2, and where bits are lost",
    ], "A left shift of n places multiplies by 2 to the power n, not by n. A shift of 3 multiplies by 8."),

    ("cs_aqa:3.3.5", &[
        "Explain what a character set is",
        "Describe 7-bit ASCII and Unicode, and use an encoding table to convert both ways",
        "Use the fact that codes run in sequence to work out other codes",
        "Explain the purpose of Unicode and its advantages over ASCII, and that it matches ASCII up to 127",
    ], "Unicode's advantage is representing many more characters, such as other alphabets and symbols. 'It is newer' or 'it is better' earns nothing."),

    ("cs_aqa:3.3.6", &[
        "Explain what a pixel is and how a bitmap is made of pixels and colour depth",
        "Describe image size (width x height in pixels) and colour depth",
        "Calculate bitmap file size from width, height and colour depth, in bits and bytes",
        "Convert between a simple bitmap and its binary data",
    ], "Colour depth is bits per pixel, not the number of colours. 2 bits give 4 colours; 8 bits give 256."),

    ("cs_aqa:3.3.7", &[
        "Explain that sound is analogue and is sampled to store it digitally",
        "Describe sampling rate and sample resolution",
        "Calculate sound file size from rate, resolution and length",
        "Explain how rate and resolution affect quality and file size",
    ], "Sampling rate is samples per second in hertz; sample resolution is bits per sample. Mixing the two up loses the definition marks."),

    ("cs_aqa:3.3.8", &[
        "Explain what compression is and why data is compressed",
        "Explain how Huffman coding works, and read codes from a Huffman tree",
        "Calculate the bits needed for Huffman-coded data and for 7-bit ASCII, and the bits saved",
        "Explain run length encoding and write data as frequency/data pairs",
    ], "In RLE the pairs are frequency then data value. 5 0 means five 0s, and reversing the order loses the mark."),

    ("cs_aqa:3.4.1", &[
        "Define hardware and software",
        "Explain the relationship between hardware and software",
        "Classify components and programs as hardware or software",
    ], "Software is the programs, not 'what you can't touch'. Definitions that rely only on touch rarely earn the mark."),

    ("cs_aqa:3.4.2", &[
        "Construct truth tables for NOT, AND, OR and XOR, and for circuits with up to three inputs",
        "Create, modify and interpret logic circuit diagrams",
        "Write Boolean expressions using . for AND, + for OR, ⊕ for XOR and an overbar for NOT",
        "Convert between a circuit and its Boolean expression in both directions",
    ], "AQA wants the symbols, not words: A.B + C̅, not A AND B OR NOT C. A correct expression in words is capped."),

    ("cs_aqa:3.4.3", &[
        "Explain system software and application software, with examples of each",
        "Explain why an operating system is needed",
        "Describe how the OS manages processors, memory, I/O devices, applications and security",
        "Explain the purpose of utility programs, with examples",
    ], "A utility program is system software that maintains the computer, such as backup or antivirus. It is not application software."),

    ("cs_aqa:3.4.4", &[
        "Explain the differences between low-level and high-level languages, and why most programs are high level",
        "Distinguish machine code from assembly language, including the 1:1 correspondence",
        "Explain the advantages and disadvantages of low-level programming",
        "Compare compilers, interpreters and assemblers, and say when each is appropriate",
    ], "Interpreters do not produce machine code: they call their own machine code routines for each statement. Claiming they translate to machine code line by line is marked wrong."),

    ("cs_aqa:3.4.5a", &[
        "Explain the role of main memory, the ALU, control unit, clock, registers and buses",
        "Explain the fetch, decode and execute stages of the fetch-execute cycle",
        "Explain how clock speed, number of cores and cache size affect CPU performance",
    ], "More cores only help when the work can be split between them. Saying four cores make it four times faster is marked wrong."),

    ("cs_aqa:3.4.5b", &[
        "Describe RAM, ROM, cache and registers: what each holds and why it is needed",
        "Explain volatile and non-volatile, main memory against secondary storage, and why secondary storage is needed",
        "Explain how solid state and magnetic storage work, and compare them",
        "Explain cloud storage and compare it with local storage",
        "Explain how an embedded system differs from a non-embedded one, with examples",
    ], "Cloud storage still uses magnetic or solid state drives, just at a remote location. Answers implying it is a different kind of storage medium lose the mark."),

    ("cs_aqa:3.5a", &[
        "Define a computer network and discuss the advantages and disadvantages of networking",
        "Describe PAN (Bluetooth), LAN and WAN, including ownership and the Internet as the largest WAN",
        "Compare wired and wireless networks, and fibre and copper cable",
        "Explain authentication, encryption, firewalls and MAC address filtering, and how they work together",
    ], "A LAN is defined by a small area and single ownership, not by the number of devices or by being wired."),

    ("cs_aqa:3.5b", &[
        "Define a network protocol",
        "Explain the purpose of TCP, IP, HTTP, HTTPS, SMTP and IMAP",
        "Describe the four layers of the TCP/IP model and what each does",
        "Place each protocol at its layer: application, transport or internet",
    ], "SMTP sends email and IMAP retrieves it from the server. Swapping them, or saying IMAP sends email, is a reliable lost mark."),

    ("cs_aqa:3.6.1", &[
        "Define cyber security",
        "Describe the main purposes of cyber security: protecting networks, computers, programs and data",
        "Explain why organisations and individuals need cyber security",
    ], "The definition needs both halves: what is protected (networks, computers, programs, data) and what from (attack, damage, unauthorised access)."),

    ("cs_aqa:3.6.2", &[
        "Explain the threats from pharming, weak and default passwords, misconfigured access rights, removable media and unpatched software",
        "Explain how each threat could be exploited by an attacker",
        "Explain what penetration testing is, and the difference between insider and external tests",
    ], "Pharming redirects website traffic to a fake site; phishing uses fake messages. Confusing them is the commonest error in this topic."),

    ("cs_aqa:3.6.2.1", &[
        "Define social engineering and explain why it targets people rather than technology",
        "Explain blagging, phishing and shouldering, with examples",
        "Explain how organisations and individuals can protect against social engineering",
    ], "'Install antivirus' does not stop blagging or shouldering. Protection against social engineering is mainly staff training and procedures."),

    ("cs_aqa:3.6.2.2", &[
        "Define malware",
        "Describe viruses, trojans and spyware and how each one behaves",
        "Explain how to protect against malware",
    ], "A trojan pretends to be useful software and does not replicate itself; a virus attaches to files and spreads. The difference is examined."),

    ("cs_aqa:3.6.3", &[
        "Explain biometric measures, especially on mobile devices",
        "Explain password systems, CAPTCHA and email confirmations",
        "Explain why automatic software updates are a security measure",
        "Match each measure to the threat it reduces",
    ], "CAPTCHA stops automated programs (bots), not people. Saying it stops hackers in general loses the mark."),

    ("cs_aqa:3.7.1", &[
        "Explain the concept of a database and of a relational database",
        "Use the terms table, record, field, data type, primary key and foreign key",
        "Explain how relational databases reduce data redundancy and inconsistency",
    ], "A foreign key is a field in one table that is the primary key of another table. 'A key from another table' without that link is too vague."),

    ("cs_aqa:3.7.2", &[
        "Write SELECT queries with FROM, WHERE and ORDER BY ASC or DESC",
        "Write queries that draw data from two tables, linked on a key",
        "Insert records with INSERT INTO … VALUES",
        "Change and remove records with UPDATE … SET … WHERE and DELETE FROM … WHERE",
    ], "In a two-table query, the tables must be linked on the key in the WHERE clause. Without it, every row is joined to every other, and the linking mark is lost."),

    ("cs_aqa:3.8", &[
        "Explain the ethical, legal and environmental impacts and risks of digital technology on society",
        "Discuss data privacy, including the tension between citizens' privacy and government security access",
        "Apply these to the named areas: cyber security, mobile and wearable technologies, wireless networking, cloud storage, hacking, implants and autonomous vehicles",
        "Write a balanced 9-mark discussion that covers all the bullets in the question",
    ], "The 9-mark questions are marked on coverage of every bullet. A strong answer on one bullet, with nothing on the others, stays in the bottom level."),
    // ---------- History (AQA 8145) ----------
    ("hist_aqa:1AB.1a", &[
        "Describe how power was shared between the Kaiser, the Chancellor, the Bundesrat and the elected Reichstag, and explain why the Reichstag's growing role limited the Kaiser",
        "Explain the influence of Prussia and the army (Prussian militarism) on how Germany was governed",
        "Explain how rapid industrialisation created a large working class, and why the SPD became the largest party in the Reichstag by 1912",
        "Explain the social reforms used to weaken socialism, and the domestic importance of the Navy Laws of 1898 and 1900",
    ], "The Navy Laws are on the specification for their domestic importance - rallying Germans behind the Kaiser, pleasing industrialists and the middle classes, and creating budget battles with the Reichstag - not only as a cause of the naval race with Britain."),

    ("hist_aqa:1AB.1b", &[
        "Explain how war weariness, shortages, defeat and the naval mutinies led to the end of the monarchy in November 1918",
        "Explain why reparations, the occupation of the Ruhr (1923) and hyperinflation damaged the new republic",
        "Describe the Spartacist rising (1919), the Kapp Putsch (1920) and the Munich Putsch (1923), and explain why each failed",
        "Assess how far Germany recovered under Stresemann (1924-29): the Rentenmark, the Dawes and Young Plans, Locarno and joining the League",
        "Describe Weimar culture and explain why it divided Germans",
    ], "Recovery rested on American loans under the Dawes Plan, and farmers and the extremist parties never went away - the specification asks for the extent of recovery, so a 'golden age' answer with no limits stays in the middle levels."),

    ("hist_aqa:1AB.2", &[
        "Explain how the Depression after 1929 raised unemployment and increased support for the Nazis and the Communists",
        "Explain Hitler's appeal to different groups, Nazi propaganda and the role of the SA",
        "Use election results to explain the failure of Weimar democracy, and the roles of Papen and Hindenburg in Hitler's appointment in January 1933",
        "Explain how the Reichstag Fire, the Enabling Act, the banning of trade unions and other parties, and the Night of the Long Knives created a dictatorship, with Hitler Führer by August 1934",
    ], "Hitler was never elected to power: the Nazi vote peaked at about 37% in July 1932 and he was appointed Chancellor through a deal involving Papen and Hindenburg - 'he won the election' loses the mark."),

    ("hist_aqa:1AB.3a", &[
        "Explain how the Nazis cut unemployment: public works such as the autobahns, the Reich Labour Service, rearmament and conscription",
        "Explain the aim of self-sufficiency (autarky) and the Four Year Plan from 1936",
        "Weigh the benefits and drawbacks of the Nazi economy for workers, including the Labour Front, Strength through Joy and Beauty of Labour",
        "Explain the impact of the Second World War on the economy and the German people: rationing, bombing, labour shortages, forced labour and refugees",
    ], "The fall in unemployment hid 'invisible unemployment' - women and Jews pushed out of jobs and men in the Labour Service or the army were not counted - and that is the evaluation point examiners look for."),

    ("hist_aqa:1AB.3b", &[
        "Explain Nazi policies towards women (Kinder, Küche, Kirche, marriage loans, the Mother's Cross) and how far women's lives changed",
        "Explain how the Hitler Youth, the League of German Maidens and the education system were used to shape the young",
        "Explain how the Nazis tried to control the churches: the Concordat (1933), the Reich Church and the Confessing Church",
        "Explain Aryan racial ideas and the stages of persecution: the 1933 boycott, the Nuremberg Laws (1935), Kristallnacht (1938), the ghettos and the Final Solution",
    ], "Policies did not have the same effect on everyone - labour shortages pulled women back into work in the late 1930s, and some young people rejected the Hitler Youth - so qualify any claim that the Nazis 'controlled' a whole group."),

    ("hist_aqa:1AB.3c", &[
        "Explain how Goebbels used propaganda and censorship: rallies, radio, film, newspapers and the 1936 Olympics",
        "Explain how Nazi culture controlled art, music and literature, including the book burnings",
        "Explain how the police state worked: Himmler, the SS, the Gestapo, informers, the courts and concentration camps",
        "Assess the extent of opposition and resistance: the Edelweiss Pirates, the Swing Youth, the White Rose and the July 1944 bomb plot",
    ], "Judge opposition by its scale and results: most Germans neither resisted nor fully believed, and the most dangerous plot came from army officers in July 1944, not from ordinary people."),

    ("hist_aqa:1AD.1a", &[
        "Explain the causes of the 1920s boom: mass production and Ford, hire purchase, advertising and Republican policies of tariffs, low taxes and laissez-faire",
        "Explain why the stock market boomed, including buying shares on the margin",
        "Explain who did not share in the boom: farmers, workers in older industries and African Americans",
        "Describe the growth of cinema and jazz, and explain how far women's lives changed, including the flappers",
    ], "The boom was uneven - farmers faced overproduction and falling prices all decade - so 'everyone was better off' throws away the inequality marks the specification names."),

    ("hist_aqa:1AD.1b", &[
        "Explain how prohibition led to bootlegging, speakeasies and organised crime, and why it was ended in 1933",
        "Explain the causes of racial tension and the experiences of immigrants, including the immigration quotas of 1921 and 1924",
        "Describe the Ku Klux Klan's beliefs, methods and impact in the 1920s",
        "Explain the Red Scare and the significance of the Sacco and Vanzetti case",
    ], "Sacco and Vanzetti matter for what the case showed - prejudice against immigrants and radicals in the courts - so explain its significance rather than retelling the robbery."),

    ("hist_aqa:1AD.2a", &[
        "Explain the effects of the Wall Street Crash and the Depression on the unemployed, farmers and businessmen",
        "Explain Hoover's response and why he became unpopular, including the Hoovervilles and the Bonus Army",
        "Explain why Roosevelt won in 1932, and describe the main New Deal measures such as the alphabet agencies, the TVA and the Social Security Act",
        "Evaluate the New Deal's successes and limits for different groups, and the opposition from the Supreme Court, Republicans and radicals such as Huey Long",
    ], "Unemployment was still high in the late 1930s and fell fully only with war production - balance the New Deal's real gains against that to reach the top level."),

    ("hist_aqa:1AD.2b", &[
        "Explain how the Second World War ended the Depression through war production and full employment",
        "Explain Lend Lease (1941) and why exports and war orders mattered to recovery",
        "Explain how far the war changed life for African Americans, including migration north and the Double V campaign",
        "Explain how far the war changed life for women, including war work and what happened after 1945",
    ], "Change was real but limited - the armed forces stayed segregated and many women left war jobs after 1945 - so 'the war transformed their lives' needs qualifying."),

    ("hist_aqa:1AD.3a", &[
        "Explain the causes of post-war prosperity and the growth of consumerism and the suburbs",
        "Explain the idea of the American Dream and who was left out of it",
        "Explain the causes and impact of McCarthyism at home",
        "Describe the impact of rock and roll and television on popular culture and young people",
    ], "McCarthyism sits in this topic as a domestic story: link it to fear of communism and its effect on Americans' freedoms and careers, not to events abroad."),

    ("hist_aqa:1AD.3b", &[
        "Explain segregation laws in the South and the importance of Brown v Topeka (1954) and Little Rock (1957)",
        "Explain Martin Luther King's peaceful methods: Montgomery (1955-56), Birmingham, the March on Washington (1963) and Selma",
        "Compare Malcolm X and the Black Power movement with King's approach",
        "Explain the importance of the Civil Rights Acts of 1964 and 1968",
    ], "The specification names the 1964 and 1968 Civil Rights Acts - know what each did (banning segregation in public places and discrimination in jobs; banning discrimination in housing) rather than lumping them together."),

    ("hist_aqa:1AD.3c", &[
        "Explain Kennedy's New Frontier and Johnson's Great Society policies on poverty, education and health, including Medicare and Medicaid",
        "Explain how the feminist movement developed in the 1960s and early 1970s, including Betty Friedan and the National Organisation for Women (1966)",
        "Explain the fight for equal pay and the Equal Pay Act of 1963",
        "Explain the significance of Roe v Wade (1973), the equal rights advances of the early 1970s, and the opposition to the Equal Rights Amendment led by Phyllis Schlafly",
    ], "The Equal Rights Amendment passed Congress in 1972 but was never ratified by enough states - the opposition to it is named in the specification, so use it to show the limits of change."),

    ("hist_aqa:1BB.1", &[
        "Explain the aims of Wilson (the Fourteen Points), Clemenceau and Lloyd George, and how far each achieved them",
        "Describe the terms of the Treaty of Versailles: territory, military limits, war guilt (Article 231) and reparations",
        "Explain why Germans called the treaty a Diktat and objected to it",
        "Evaluate the strengths and weaknesses of the settlement, including the problems of the new states",
    ], "Reparations were fixed at £6,600 million in 1921, not in the treaty itself, and 'how far did they achieve their aims' needs a verdict for each of the Big Three, not just for Clemenceau."),

    ("hist_aqa:1BB.2a", &[
        "Describe the League's covenant, organisation (Assembly, Council, Secretariat, Court) and powers, including sanctions",
        "Explain how its membership weakened it: the USA never joined, Germany joined in 1926 and the USSR only in 1934",
        "Explain the work of the League's agencies, such as the International Labour Organisation and the Health and Refugees bodies",
        "Judge its 1920s record: the Aaland Islands, Upper Silesia, Vilna, Corfu and Bulgaria, alongside Locarno (1925) and the Kellogg-Briand Pact (1928)",
    ], "Vilna and Corfu were failures, the Aaland Islands, Upper Silesia and Bulgaria were successes - and Locarno was agreed outside the League, which some historians see as a sign of its weakness."),

    ("hist_aqa:1BB.2b", &[
        "Explain how the Depression weakened the League and encouraged aggression",
        "Explain the Manchurian crisis (1931-33): the Mukden incident, the Lytton Report and Japan leaving the League",
        "Explain the Abyssinian crisis (1935-36): sanctions that left out oil, the open Suez Canal and the Hoare-Laval Pact",
        "Explain why the League failed to stop war in 1939",
    ], "Abyssinia hurt the League more than Manchuria because Britain and France, its leading members, were seen to betray it through the Hoare-Laval Pact."),

    ("hist_aqa:1BB.3a", &[
        "Explain Hitler's aims (overturning Versailles, uniting German speakers, Lebensraum) and how Britain and France reacted",
        "Explain the Dollfuss affair (1934), the Saar plebiscite (1935), rearmament and conscription, the Stresa Front and the Anglo-German Naval Agreement",
        "Explain the remilitarisation of the Rhineland (1936), the Rome-Berlin Axis, the Anti-Comintern Pact and the Anschluss (1938)",
        "Weigh the reasons for and against appeasement, and explain the Sudeten crisis and the Munich Agreement (1938)",
    ], "The Anglo-German Naval Agreement broke the Stresa Front because Britain approved a German breach of Versailles - precise links like this one lift a narrative account to the top level."),

    ("hist_aqa:1BB.3b", &[
        "Explain why the occupation of the rest of Czechoslovakia in March 1939 ended appeasement",
        "Explain why Stalin signed the Nazi-Soviet Pact in August 1939 and what it allowed Hitler to do",
        "Explain why the invasion of Poland led to war in September 1939",
        "Judge the responsibility of Hitler, Stalin and Chamberlain for the outbreak of war",
    ], "The specification names Hitler, Stalin and Chamberlain - an essay on responsibility that leaves out Stalin and the Nazi-Soviet Pact misses one of the named individuals."),

    ("hist_aqa:1BC.1a", &[
        "Explain what was agreed at Yalta and Potsdam in 1945, and why relations worsened between the two conferences",
        "Explain how Germany and Berlin were divided into zones",
        "Compare the ideologies of the USA and the USSR, and the aims of Stalin, Churchill, Roosevelt, Attlee and Truman",
        "Explain the effect of the atomic bomb on post-war relations between the superpowers",
    ], "Potsdam had new Western leaders - Truman instead of Roosevelt, and Attlee replacing Churchill during the conference - and that change is part of why tension rose."),

    ("hist_aqa:1BC.1b", &[
        "Explain how the USSR took control of Eastern Europe, and the meaning of the Iron Curtain",
        "Explain the purposes of the Truman Doctrine and the Marshall Plan (1947), and Stalin's reaction: Cominform and Comecon",
        "Explain why Tito's Yugoslavia broke with Stalin",
        "Explain the causes, events and results of the Berlin Blockade and Airlift (1948-49)",
    ], "The Truman Doctrine was the political promise of containment and the Marshall Plan was the economic aid that carried it out - Stalin called it dollar imperialism - so keep the two distinct."),

    ("hist_aqa:1BC.2", &[
        "Explain the importance of Mao's victory in China (1949), the Korean War (1950-53) and the fighting in Vietnam for superpower relations",
        "Explain the arms race and the membership and purposes of NATO (1949) and the Warsaw Pact (1955)",
        "Explain the space race: Sputnik, ICBMs, Polaris, Gagarin and Apollo",
        "Explain the Thaw and the Hungarian rising of 1956: Nagy's reforms, Soviet fears and the invasion",
        "Explain the U2 crisis of 1960 and its effect on the Paris summit",
    ], "The UN fought in Korea only because the USSR was boycotting the Security Council at the time - a precise detail that turns 'the USA intervened' into analysis."),

    ("hist_aqa:1BC.3a", &[
        "Explain why the Berlin Wall was built in 1961, and Kennedy's response",
        "Explain Castro's revolution and the Bay of Pigs invasion (1961)",
        "Explain the roles of Khrushchev, Kennedy and Castro in the Cuban Missile Crisis (October 1962), and why the USA feared missiles on Cuba",
        "Evaluate the dangers and results of the crisis, including the hotline and the Test Ban Treaty",
    ], "The crisis ended with a public deal (no US invasion of Cuba) and a secret one (US missiles out of Turkey) - the best answers use both to explain why each side could claim success."),

    ("hist_aqa:1BC.3b", &[
        "Explain Dubček's reforms in the Prague Spring of 1968 and why the USSR and its allies invaded",
        "Explain the Brezhnev Doctrine and the effects of the Prague Spring on East-West relations and the Warsaw Pact",
        "Explain the sources of tension that remained, including the Soviet record on human rights",
        "Explain the reasons for détente and SALT 1 (1972), and the parts played by Brezhnev and Nixon",
    ], "Czechoslovakia was invaded by Warsaw Pact forces, not by Soviet troops alone, and the West's weak response showed it accepted the Soviet sphere - a link that explains why détente could follow."),

    ("hist_aqa:2AA.1", &[
        "Explain medieval ideas about the causes of disease: supernatural and religious explanations, the Four Humours and miasma",
        "Explain why the ideas of Hippocrates and Galen dominated, and how the Church supported them",
        "Describe the training and methods of medieval physicians, and the Church's role in care and hospitals",
        "Explain the nature and importance of Islamic medicine and surgery, and medieval surgical ideas and techniques",
        "Explain public health in towns and monasteries, and beliefs about the causes, treatment and prevention of the Black Death",
    ], "The Church both helped medicine (hospitals, care, copying texts) and held it back (backing Galen, discouraging dissection) - a one-sided answer on religion cannot reach Level 4."),

    ("hist_aqa:2AA.2", &[
        "Explain how the Renaissance challenged medical authority: Vesalius in anatomy, Paré in surgery and Harvey in physiology",
        "Explain why their discoveries changed treatment so little at the time, and the opposition to change",
        "Describe traditional and new treatments, quackery, and responses to plague",
        "Explain the growth of hospitals, changes in the training and status of surgeons and physicians, and the work of John Hunter",
        "Explain Jenner's smallpox vaccination (1796) and the opposition to it",
    ], "Jenner did not know why vaccination worked - germ theory came decades later - which is why opposition was strong and why he changed prevention but not ideas about cause."),

    ("hist_aqa:2AA.3a", &[
        "Explain Pasteur's germ theory, Koch's work on identifying bacteria, and how their rivalry drove microbe hunting",
        "Explain Pasteur's development of vaccines and Ehrlich's magic bullet, Salvarsan 606",
        "Describe everyday treatments and remedies in the 19th century",
        "Explain how anaesthetics (Simpson and chloroform), antiseptics (Lister and carbolic acid) and aseptic surgery tackled pain and infection",
    ], "Anaesthetics at first raised death rates, because surgeons attempted longer operations before infection was understood - a key example that progress was not smooth."),

    ("hist_aqa:2AA.3b", &[
        "Explain the public health problems of industrial towns and the impact of the cholera epidemics",
        "Explain the role of public health reformers such as Edwin Chadwick and John Snow",
        "Compare the 1848 and 1875 Public Health Acts, and explain why government moved from laissez-faire to compulsion",
        "Explain how factors combined to bring change: science, the vote for working men, local government and engineering",
    ], "The 1848 Act let towns choose whether to act; the 1875 Act made them act - confusing the two is the most common slip in public health answers."),

    ("hist_aqa:2AA.4a", &[
        "Explain Fleming's discovery of penicillin and its development by Florey and Chain, including the roles of war and government",
        "Explain the growth of the pharmaceutical industry, new diseases, antibiotic resistance and alternative treatments",
        "Explain how war advanced surgery: plastic surgery, blood transfusions and X-rays",
        "Describe modern surgical methods: transplants, keyhole surgery, lasers and radiation therapy",
    ], "Fleming found penicillin by chance but could not produce it; Florey and Chain made it a usable drug, and wartime government and industry mass-produced it - give each factor its share."),

    ("hist_aqa:2AA.4b", &[
        "Explain the importance of Booth's and Rowntree's surveys and the poor health of Boer War recruits",
        "Explain the Liberal social reforms of 1906-11, such as school meals, old age pensions and National Insurance",
        "Explain the impact of the two world wars on public health, poverty and housing",
        "Explain the Beveridge Report (1942), the creation and development of the NHS (1948), and the costs and choices facing healthcare today",
    ], "Many doctors opposed the NHS at first, and Bevan won them round with concessions - a significance answer needs the reaction at the time as well as the long-term impact."),

    ("hist_aqa:2AB.1", &[
        "Explain the barons' dissatisfaction with King John and how it was resolved in 1215",
        "Describe the terms of Magna Carta and assess its short- and long-term impact",
        "Explain the conflict between Henry III and his barons, the role of Simon de Montfort, the Provisions of Oxford and the Parliament of 1265",
        "Explain the social, economic and political causes of the Peasants' Revolt (1381), the actions of rebels and government, and its impact",
    ], "Magna Carta was annulled within weeks and civil war followed; its importance is long-term - separate the short-term failure from the later symbol to reach the top level on significance."),

    ("hist_aqa:2AB.2", &[
        "Explain the causes of the Pilgrimage of Grace (1536), Henry VIII's reaction and its impact on royal authority",
        "Explain the causes of the English Revolution, the New Model Army and the growth of radical ideas",
        "Explain the significance of the trial and execution of Charles I, Cromwell and the Commonwealth",
        "Explain the causes, impact and significance of the American Revolution",
    ], "Similarity questions often pair the Pilgrimage of Grace with the barons' revolt or the Peasants' Revolt - learn causes, methods and outcomes in a way that lets you compare them."),

    ("hist_aqa:2AB.3", &[
        "Explain radical protest before 1832, and the causes and impact of the Great Reform Act and later reform",
        "Explain the causes, actions and impact of Chartism",
        "Explain the methods and impact of campaigning groups: the anti-slavery movement, the Anti-Corn Law League, and factory and social reformers",
        "Explain the development of trade unionism: the GNCTU, the Tolpuddle Martyrs, New Model Unions and new unionism, including the match girls' and dockers' strikes",
    ], "Chartism failed at the time but most of its six points later became law - a significance answer needs both the short-term failure and the long-term influence."),

    ("hist_aqa:2AB.4", &[
        "Explain the campaign for women's suffrage: its reasons, methods and the government's responses, including the role of the Pankhursts",
        "Explain why the franchise was extended to women in 1918 and 1928, its impact, and progress towards equality after 1945",
        "Explain the causes, events and impact of the General Strike (1926) and trade union reform in the late 20th century",
        "Explain the growth of a multi-racial society since 1945: discrimination, protest and reform, the Brixton riots (1981) and the Scarman Report",
    ], "Whether militancy helped or harmed the cause is debated - weigh the suffragettes against the suffragists and the effect of the First World War rather than leaving any out."),

    ("hist_aqa:2BA.1a", &[
        "Explain the claims of Harold Godwinson, William of Normandy, Harald Hardrada and Edgar Aethling after Edward the Confessor died in January 1066",
        "Explain the events and significance of the Battle of Stamford Bridge",
        "Explain why William won the Battle of Hastings: tactics, leadership, luck and Harold's decisions",
        "Explain Norman military innovations, including cavalry and castles",
    ], "Fulford and Stamford Bridge weakened and delayed Harold, but William's preparation, cavalry and feigned retreats matter too - an answer that says 'William was lucky' and stops is a Level 2 answer."),

    ("hist_aqa:2BA.1b", &[
        "Explain the revolts of 1067-75, including Exeter, the northern risings, Hereward and the Revolt of the Earls",
        "Explain the causes and consequences of the Harrying of the North (1069-70)",
        "Explain how William's leadership and government kept control, combining force with Anglo-Saxon institutions",
        "Explain William II's inheritance in 1087 and how he dealt with the challenges to it",
    ], "William kept control by mixing force (castles, the Harrying, new landholders) with continuity (sheriffs, writs, the coinage) - give both sides to answer 'how' questions fully."),

    ("hist_aqa:2BA.2", &[
        "Explain feudalism: roles, rights and responsibilities, landholding, lordship, patronage and military service",
        "Compare Anglo-Saxon and Norman government and aristocracies",
        "Explain the Norman legal system, including trial by ordeal, murdrum, forest law and inheritance",
        "Explain the purpose and significance of the Domesday Book (1086)",
        "Explain how life in towns and villages changed and stayed the same: buildings, work, food, roles and the farming year",
    ], "For most peasants daily work changed little after 1066 - the main change was a new lord - so continuity matters as much as change in this topic."),

    ("hist_aqa:2BA.3", &[
        "Describe the Anglo-Saxon Church before 1066",
        "Explain Archbishop Lanfranc's reforms: church and cathedral building, organisation and separate Church courts",
        "Explain Church-state relations, William II and the Church, the Church's wealth, relations with the Papacy and the Investiture Controversy",
        "Explain the Norman reform of monasticism: abbeys, monastic life, learning, schools, and the use of Latin and English",
    ], "Lanfranc's reforms replaced English bishops and abbots with Normans, so Church reform was also a tool of Norman control - linking religion to power is what earns the high levels."),

    ("hist_aqa:2BA.4", &[
        "Apply the aspects AQA lists to any site: location, function, structure, the people connected with it, design, how the design reflects the culture of the time, and its links to wider events",
        "Explain the location, purpose, design and building of the White Tower, begun under William I and completed under William II",
        "Explain how the White Tower reflected Norman power, culture and religion, including its chapel",
        "Use the site as evidence for change and continuity in Norman England, and plan the 16-mark essay on it",
    ], "The 16-mark question is about the period through the site: an answer that describes the building without linking it to Norman control, culture or events will not reach the top levels."),

    ("hist_aqa:2BC.1", &[
        "Describe Elizabeth's background and character, and how court life and patronage worked",
        "Explain the roles of her key ministers, such as William Cecil, Walsingham and Robert Dudley",
        "Explain her relations with Parliament, including disputes over marriage, the succession, religion and monopolies",
        "Assess the strength of her authority at the end of her reign, including Essex's rebellion in 1601",
    ], "Essex's rebellion was small and quickly crushed - it can be used as evidence that her authority was still strong, not only as a sign of decline."),

    ("hist_aqa:2BC.2", &[
        "Explain whether the period was a Golden Age: living standards, fashions, growing prosperity and the rise of the gentry",
        "Explain the achievements of Elizabethan theatre and attitudes to it",
        "Explain the reasons for the increase in poverty, attitudes to the poor and the government's response, including the 1601 Poor Law",
        "Explain the voyages of Hawkins and Drake, Drake's circumnavigation (1577-80) and the role of Raleigh",
    ], "Elizabethans split the poor into the deserving and the undeserving (sturdy beggars), and the laws treated them differently - use those terms precisely."),

    ("hist_aqa:2BC.3a", &[
        "Explain the challenges to the religious settlement from English Catholics and Puritans",
        "Explain the Northern Rebellion (1569), the excommunication (1570) and the missionary priests",
        "Explain the Catholic plots against Elizabeth and her government's responses",
        "Explain the background of Mary, Queen of Scots, how Elizabeth and Parliament treated her, the threat she posed, and the impact of her execution in 1587",
    ], "Mary's execution in 1587 removed the Catholic heir but helped push Philip II towards the Armada - link it to what came next."),

    ("hist_aqa:2BC.3b", &[
        "Explain the reasons for conflict with Spain: religion, trade and privateering, the Netherlands and Mary's execution",
        "Describe the events, including Drake's raid on Cadiz (1587) and the Armada campaign of 1588",
        "Explain naval warfare, tactics and technology, including galleons, guns and fireships",
        "Evaluate the reasons for the Armada's defeat: English tactics, Spanish mistakes, leadership and the weather",
    ], "Many Armada ships were lost in storms off Scotland and Ireland on the way home rather than in battle - weigh luck against English tactics in any 'main reason' essay."),

    ("hist_aqa:2BC.4", &[
        "Apply the aspects AQA lists to any site: location, function, structure, the people connected with it, design, how the design reflects the culture of the time, and its links to wider events",
        "Explain how Robert Dudley, Earl of Leicester, turned Kenilworth from a medieval fortress into a palace after 1563",
        "Explain how Leicester's Building, the gatehouse and the garden reflected Elizabethan wealth, fashion and patronage",
        "Explain the significance of Elizabeth's visit of 1575 and plan the 16-mark essay on the site",
    ], "Kenilworth was rebuilt to impress the Queen - use it as evidence of patronage, court culture and Dudley's marriage hopes, not just as a castle."),
    // ---------- Religious Studies (Eduqas C120QS, Route A, Islam) ----------
    ("rs_edq:2.1a", &[
        "Explain Christian and Muslim teachings on the purpose of families and the roles of women and men in the family",
        "Describe Christian and Muslim marriage ceremonies and explain what they show about the purpose of marriage, using Mark 10:6-8 and Qur'an 30:21",
        "Compare attitudes to cohabitation and to marrying outside the faith, including Church of England teaching and arranged marriage in Britain",
        "Explain the difference between Sunni and Shi'a views on temporary marriage (mut'ah)",
        "Define the concepts commitment, cohabitation, responsibilities and roles",
    ], "Part (c) asks for two religions or two religious traditions. Writing only about Christianity, or adding a humanist view, caps the answer however detailed it is."),

    ("rs_edq:2.1b", &[
        "Explain Christian attitudes to adultery, divorce, annulment and remarriage, interpreting Matthew 19:8-9 and Mark 10:9",
        "Explain Muslim attitudes to divorce, separation and remarriage, using Qur'an 4:35, 4:128-130 and 2:229",
        "Explain Aquinas's Natural Law and the second primary precept, and how it shapes Catholic teaching on contraception",
        "Compare Christian and Muslim teachings on the purpose of sex and the use of contraception, including Qur'an 17:32",
        "Define adultery, divorce and contraception precisely",
    ], "Annulment is not a Catholic divorce. It declares that a valid marriage never existed, and saying otherwise loses the AO1 credit."),

    ("rs_edq:2.1c", &[
        "Explain diverse Christian attitudes to same-sex relationships, interpreting Leviticus 20:13 and 1 Timothy 1:8-10",
        "Explain Muslim attitudes to same-sex relationships with reference to Qur'an 7:80-81",
        "Compare Catholic, Orthodox and Anglican views on women in worship and authority, using 1 Timothy 2:11-12 and Galatians 3:27-29",
        "Explain diverse Muslim views on the roles of women and men in worship and authority, using Qur'an 2:228, 40:40 and 4:1",
        "Evaluate whether men and women can have equal roles in religion",
    ], "Candidates often say the Catholic Church gives women no role in worship. Women read, lead prayer and serve as religious sisters; only ordination is reserved to men."),

    ("rs_edq:2.1d", &[
        "Explain literal and non-literal Christian readings of Genesis 1 and 2",
        "Explain Muslim teaching on the origin of the universe, using Qur'an 36:81",
        "Describe Brian Cox's account of the Big Bang and the non-religious views of Charles Darwin and Richard Dawkins on evolution",
        "Evaluate how far religious and scientific accounts of origins conflict",
    ], "Describing Darwin means natural selection and gradual change, not a list of arguments against creationism. The 2023 report also found candidates writing about Charles Dickens."),

    ("rs_edq:2.1e", &[
        "Explain dominion and stewardship in Christianity, using Genesis 1:28 and Psalm 8:6",
        "Explain khalifah and fitra and why they make Muslims responsible for the environment, using Qur'an 7:54",
        "Describe the work of Humanist Climate Action and non-religious views on sustainability and global citizenship",
        "Explain Peter Singer's idea of speciesism and religious responses to it",
        "Define environmental sustainability",
    ], "Questions on responsibility for the environment want why believers are responsible (steward, khalifah), not a general description of pollution."),

    ("rs_edq:2.1f", &[
        "Explain sanctity of life and quality of life and apply them to abortion and euthanasia",
        "Explain diverse Christian attitudes to abortion and euthanasia, using Genesis 1:31 and Jeremiah 1:5",
        "Explain Muslim attitudes to abortion and euthanasia, using Qur'an 5:32, 6:151 and 30:40",
        "Explain non-religious arguments, including Humanist support for assisted dying (Dignity in Dying)",
        "Evaluate a statement on abortion or euthanasia using religious and non-religious views",
    ], "The Life and Death 15-mark question must include non-religious views such as Humanists and Atheists. Leaving them out keeps the answer out of the top bands."),

    ("rs_edq:2.1g", &[
        "Explain Christian beliefs about the soul, judgement, heaven and hell, using John 11:24-27 and 1 Corinthians 15:42-44",
        "Explain Muslim beliefs about the soul, judgement, akhirah, heaven and hell, using Qur'an 46:33 and 3:16",
        "Describe how Christian, Muslim and Humanist funerals in Britain reflect beliefs about the afterlife",
        "Explain the diversity of views about Muslim worship at graves",
        "Define afterlife and soul",
    ], "A funeral answer must link each feature to a belief: describing hymns and coffins without saying what they show about the afterlife stays in the lowest band."),

    ("rs_edq:2.1h", &[
        "Explain what makes an act wrong: absolute and relative morality, conscience, virtues, sin, free will and doing the will of Allah",
        "Explain Original Sin and Irenaeus's and John Hick's soul-making theodicy as accounts of the origin of evil",
        "Explain Muslim teaching on evil: Iblis tests humans (Qur'an 2:34 and 2:155) and the relationship of al-Qadr to free will",
        "Evaluate the philosophical challenge that evil and suffering pose to belief in God",
        "Define good, evil, free will, morality, sin and suffering",
    ], "Explaining the origins of evil needs a religious account (Original Sin, Iblis, soul-making). Listing types of evil (moral and natural) answers a different question."),

    ("rs_edq:2.1i", &[
        "Explain the causes of crime and the four aims of punishment: justice, retribution, deterrence and reformation, using Qur'an 16:90",
        "Explain how criminals should be treated and the work of prison reformers and prison chaplains",
        "Compare conservative and liberal Christian responses to the death penalty, interpreting Exodus 20:13 and Matthew 5:38-39, 43-47",
        "Explain varied Muslim responses to the death penalty, including interpretations of Shari'ah",
        "Define justice and punishment",
    ], "Retribution is not revenge. It means the punishment fits the crime and is given by a lawful authority, and confusing the two loses marks in parts (a) and (b)."),

    ("rs_edq:2.1j", &[
        "Explain Christian teaching on forgiveness, interpreting Matthew 18:21-22 and Matthew 6:14-15",
        "Explain Muslim teaching on forgiveness, using Qur'an 42:30 and 64:14",
        "Describe an example of forgiveness arising from personal belief",
        "Evaluate whether forgiveness should replace punishment",
    ], "Forgiving someone does not mean they escape punishment. Strong answers separate personal forgiveness from the justice the law still requires."),

    ("rs_edq:2.1k", &[
        "Explain Christian and Muslim teaching on the dignity of human life, using Genesis 1:26-27 and Qur'an 5:32",
        "Explain agape in action and ummah in action as ways of promoting human rights",
        "Describe an example of conflict between personal conviction and the laws of a country",
        "Explain censorship, freedom of religious expression and religious extremism, including Islamophobia",
        "Define human rights, social justice, censorship, extremism and personal conviction",
    ], "Extremism questions need balance: stating that one religion is violent is a factual error and is not credited as evaluation."),

    ("rs_edq:2.1l", &[
        "Distinguish prejudice from discrimination and explain how each affects individuals and society",
        "Explain Christian teaching on prejudice and discrimination, using Galatians 3:27-29 and Martin Luther King's teaching on equality",
        "Explain Muslim teaching on prejudice and racial discrimination, using Qur'an 5:8 and 49:13 and the work of the Christian/Muslim Forum",
        "Evaluate whether religion helps to reduce prejudice",
    ], "Prejudice is an attitude and discrimination is an action. A definition that blurs them earns one mark, not two."),

    ("rs_edq:2.1m", &[
        "Distinguish relative and absolute poverty",
        "Explain Christian and Muslim ethical teaching on acquiring and using wealth, using Luke 16:19-31 and Qur'an 2:177",
        "Describe how Christian Aid and Islamic Relief work to alleviate poverty",
        "Evaluate whether religious believers should give away their wealth",
    ], "Describing a charity means saying what it actually does (emergency aid, long-term development, campaigning), not just that it 'helps poor people'."),

    ("rs_edq:2.2a", &[
        "Explain God as omnipotent (Exodus 7-11, 14:21) and omnibenevolent (Psalm 86:15, John 3:16, Romans 8:37-39)",
        "Explain the problem of evil and suffering, using Epicurus and the Book of Job (1:8-12, 42:1-6)",
        "Explain the Trinity as one God in three persons, using John 10:30 and John 14:6-11",
        "Evaluate which belief about God is most important for Christians",
        "Define omnipotent, omnibenevolent and Trinity",
    ], "The Trinity is one God, not three gods: answers that say Christians worship three gods contradict the doctrine and lose the definition mark."),

    ("rs_edq:2.2b", &[
        "Explain Genesis 1-3 on creation and on the nature and role of humans",
        "Compare literal and non-literal ways of interpreting Genesis",
        "Explain the role of the Word and the Spirit in creation, using John 1:1-5",
        "Explain the Bible as the Word of God: inspiration, revelation, ways of interpreting it, and its authority beside other sources",
    ], "Non-literal Christians still believe God created the world. Saying they 'do not believe in creation' is a common misreading."),

    ("rs_edq:2.2c", &[
        "Explain the incarnation, using John 1:14 and Luke 1:28-33",
        "Explain the crucifixion and atonement, using Matthew 27:28-50, Matthew 26:26-29, Leviticus 16:20-22 and Isaiah 53:3-9",
        "Explain the resurrection and ascension, using Luke 24:1-9, 1 Corinthians 15:3-8, 12-14 and Luke 24:50-53",
        "Explain sin as preventing salvation, and the roles of grace and the Holy Spirit (Acts 2:1-6), including in Evangelical worship",
        "Define incarnation, atonement and resurrection",
    ], "Atonement is the repairing of the relationship between God and humans through Jesus' death. Defining it as 'saying sorry' earns nothing."),

    ("rs_edq:2.2d", &[
        "Explain Christian eschatological beliefs, using John 11:25-26 and John 14:2-7",
        "Explain judgement, using the Sheep and the Goats (Matthew 25:31-46) and the Rich Man and Lazarus (Luke 16:19-31)",
        "Explain bodily and spiritual resurrection, using 1 Corinthians 15:42-55",
        "Compare traditional and contemporary beliefs about heaven and hell",
    ], "Purgatory is a Catholic belief, not one held by all Christians. Presenting it as universal loses accuracy marks."),

    ("rs_edq:2.2e", &[
        "Explain the nature and significance of liturgical, informal and individual worship, using Matthew 18:20",
        "Explain the importance of prayer, including the Lord's Prayer, set prayers and informal prayers",
        "Describe worship in the Society of Friends (Quakers) and in Evangelical churches",
        "Evaluate whether one form of worship is better than another",
    ], "Quaker worship is mostly silent waiting with no set liturgy or priest. Describing it as a hymn-singing service is a factual error."),

    ("rs_edq:2.2f", &[
        "Explain diverse Christian beliefs about what a sacrament is and how many there are",
        "Describe the meaning and celebration of baptism, using John 3:3-6, including infant and believers' baptism",
        "Explain different Catholic and Protestant interpretations of the Eucharist, including transubstantiation and memorial",
        "Define sacraments",
    ], "The Eucharist question wants different beliefs. A description of what happens at Mass without explaining what different Christians believe about the bread and wine stays low."),

    ("rs_edq:2.2g", &[
        "Explain the importance of pilgrimage for Christians",
        "Describe pilgrimage to Walsingham and to Taizé and explain what pilgrims gain",
        "Describe how Christians celebrate Christmas and Easter and explain the beliefs behind the celebrations",
        "Evaluate whether pilgrimage or festivals matter more to Christian faith",
    ], "Taizé is an ecumenical community in France, not a shrine to Mary. Mixing it up with Walsingham is a frequent error."),

    ("rs_edq:2.2h", &[
        "Use the 2001, 2011 and 2021 census results to describe religious diversity in Britain",
        "Explain how UK laws, festivals and traditions are rooted in Christianity while celebrating other traditions",
        "Explain the role of the local church as a place of worship and a social and community centre",
        "Explain the importance of mission, evangelism and church growth",
        "Define evangelism",
    ], "The 2021 census showed fewer than half of respondents in England and Wales calling themselves Christian, yet Christianity is still the largest religion. Answers often get one half of that wrong."),

    ("rs_edq:2.2i", &[
        "Describe the work of Tearfund as Christian belief in action",
        "Explain the persecution of Christians past and present and how Christians respond",
        "Explain how Christians work for reconciliation through the World Council of Churches and the ecumenical movement",
        "Evaluate the most important role of the worldwide Church",
    ], "In 2025 many candidates skipped or misread the reconciliation question. Reconciliation means restoring broken relationships, including between denominations, not only forgiving a crime."),

    ("rs_edq:2.3a", &[
        "Explain Tawhid, the oneness of Allah, using Qur'an 3:18",
        "Explain the attributes of Allah: immanence, transcendence, omnipotence, beneficence, mercy, fairness and justice (Qur'an 46:33)",
        "Explain Adalat (divine justice) in Shi'a Islam",
        "Define Tawhid and shariah",
    ], "Immanent means close to and active in the world; transcendent means beyond it. Swapping them is the most common definition error."),

    ("rs_edq:2.3b", &[
        "Name and explain the six articles of faith in Sunni Islam",
        "Name and explain the five roots of Usul ad-Din in Shi'a Islam",
        "Explain Muslim attitudes to the kutub: Sahifah, Tawrat, Zabur, Injil and the Qur'an",
        "Compare the Sunni and Shi'a foundations of faith",
    ], "The kutub are not equal in authority: Muslims believe the earlier books were revealed by Allah but altered or lost, while the Qur'an is preserved and final."),

    ("rs_edq:2.3c", &[
        "Explain the nature and importance of prophethood, using Qur'an 2:136",
        "Explain the importance of Adam as the first prophet and of Ibrahim as father of Isma'il and Ishaq",
        "Explain Isa as a prophet in Islam (Qur'an 2:87) and how this differs from Christian belief",
        "Explain Muhammad as the Seal of the Prophets",
        "Evaluate whether all prophets are equally important",
    ], "Muslims honour Isa as a prophet, not as the Son of God. The 2025 report says pre-learnt answers on one prophet failed to evaluate the question set."),

    ("rs_edq:2.3d", &[
        "Explain the significance of angels in Islam, using Qur'an 2:97-98 and 2:285",
        "Explain Muslim beliefs about angels and free will",
        "Describe the roles of Jibril, Mika'il and Israfil, and the importance of Jibril's revelation of the Qur'an",
    ], "Angels in Islam have no free will and always obey Allah. Saying they can choose to disobey contradicts the belief."),

    ("rs_edq:2.3e", &[
        "Explain al-Qadr (predestination) and its implications for human freedom",
        "Explain human responsibility and accountability on the Day of Judgement",
        "Explain Muslim beliefs about the nature, stages and purpose of heaven and the nature and purpose of hell",
        "Evaluate whether belief in al-Qadr removes free will",
    ], "The 2025 heaven question was lost by candidates who wrote about Judgement Day without linking it back to the nature and purpose of heaven."),

    ("rs_edq:2.3f", &[
        "Explain the Shahadah and its place in Muslim life (Qur'an 3:18)",
        "Describe salah at the mosque and at home, including Jummah prayer (Qur'an 15:98-99, 29:45)",
        "Explain zakah and how it is spent, and sawm in Ramadan (Qur'an 2:184), including issues for Muslims fasting in Britain",
        "Describe Hajj (Qur'an 2:125) and issues for British Muslims undertaking it",
        "Define mosque, halal, haram and ummah",
    ], "Zakah is a fixed obligatory payment on savings above a threshold, distinct from voluntary sadaqah. Calling it a donation of any amount loses accuracy."),

    ("rs_edq:2.3g", &[
        "Name the Ten Obligatory Acts and explain how they overlap with the Five Pillars",
        "Explain how Shi'a Muslims perform salah, observe sawm, pay zakah and khums, and make pilgrimage to Makkah and to Shi'a shrines",
        "Explain amr-bil-maroof, nahi anil munkar, tawalla and tabarra",
        "Evaluate the importance of the Ten Obligatory Acts for Shi'a Muslims",
    ], "Khums is a Shi'a payment of one fifth of surplus income, not the same as zakah. Treating them as one practice loses the Shi'a detail the question wants."),

    ("rs_edq:2.3h", &[
        "Explain Greater Jihad as the daily struggle to live as a good Muslim, including in Britain today",
        "Explain the origins of Lesser Jihad and the conditions for declaring it, using Qur'an 2:190 and 22:39",
        "Evaluate whether Greater Jihad is more important than Lesser Jihad",
        "Define Greater and Lesser Jihad",
    ], "Jihad means struggle. Translating it as 'holy war' is a misconception, and Lesser Jihad has strict conditions that rule out terrorism."),

    ("rs_edq:2.3i", &[
        "Describe how Muslims in Britain and worldwide celebrate Id-ul-Adha and Id-ul-Fitr and explain their origins",
        "Explain how Shi'a Muslims commemorate Ashura and why",
        "Explain the importance of the Night of Power and how the Qur'an is viewed and treated",
        "Evaluate whether all Muslims celebrate festivals in the same way",
    ], "Ashura is a day of mourning for Shi'a Muslims, remembering Husayn's death at Karbala. Calling it a celebration misses its meaning."),
    // ---------- Geography B (Pearson Edexcel GCSE 1GB0) ----------
    ("geog_edxb:1.1", &[
        "Explain how the Hadley, Ferrel and Polar cells and ocean currents move heat from the equator towards the poles",
        "Explain why sinking air at about 30° gives high pressure and arid areas, and rising air at the equator gives low pressure and heavy rain",
        "Read and compare climate graphs for a hot desert and an equatorial location",
        "Describe how surface winds such as the trade winds link to the pressure belts",
    ], "Rising air gives rain because it cools and condenses; sinking air warms and stays dry. Answers that say 'high pressure is hot' without the sinking-air mechanism stay at 1 mark."),

    ("geog_edxb:1.2", &[
        "Explain the four natural causes of climate change: orbital (Milankovitch) changes, solar output, volcanic eruptions and asteroid collisions",
        "Explain how ice cores, tree rings and historical sources each give evidence of past climate",
        "Describe glacial and interglacial periods in the Quaternary and the UK's climate since Roman times, including the Little Ice Age",
        "Interpret line graphs and bar charts that show past climate change",
    ], "Each piece of evidence needs its 'how': ice cores trap air bubbles whose gases show past temperatures. Naming the evidence alone earns only the first mark of a 2-mark explain."),

    ("geog_edxb:1.3", &[
        "Explain how industry, transport, energy and farming release carbon dioxide and methane",
        "Explain the enhanced greenhouse effect and how it differs from the natural greenhouse effect",
        "Use the evidence of warming: global temperature rise, warming oceans and sea level rise, shrinking Arctic ice and more extreme weather",
        "Explain why projections of temperature and sea level to 2100 vary: emissions scenarios, feedbacks and uncertain physical processes",
    ], "The greenhouse effect is natural and keeps the Earth warm; the enhanced effect is the human addition. Students who call the greenhouse effect itself 'caused by humans' lose the definition mark."),

    ("geog_edxb:1.4", &[
        "Describe the characteristics of a tropical cyclone: very low pressure, rotation, the eye and eyewall, and spiral rain bands",
        "Describe where and when tropical cyclones form, their source areas and typical tracks, and how these may change",
        "Explain the conditions for formation: oceans above about 26.5 °C, low wind shear, distance of 5° or more from the equator for the Coriolis effect",
        "Explain why some cyclones intensify and why they dissipate over land or cooler water",
    ], "Within about 5° of the equator the Coriolis effect is too weak to make the storm spin. Writing only 'it is too hot near the equator' scores nothing."),

    ("geog_edxb:1.5", &[
        "Describe the physical hazards of tropical cyclones: high winds, intense rainfall, storm surges, coastal flooding and landslides",
        "Explain the impacts of each hazard on people and on environments",
        "Explain physical, social and economic reasons why some countries are more vulnerable than others",
        "Use the Saffir-Simpson scale and storm data to judge a cyclone's magnitude",
    ], "Vulnerability is about people and places, not the storm. 'A stronger storm hits them' is magnitude; low-lying coasts, poverty and weak warnings are vulnerability."),

    ("geog_edxb:1.6", &[
        "Explain how forecasting, satellite tracking, warnings, evacuation and storm-surge defences help countries prepare",
        "Evaluate preparation and response for Hurricane Katrina in the USA (2005), including the levee failures in New Orleans",
        "Evaluate preparation and response for Typhoon Haiyan in the Philippines (2013), including the storm surge at Tacloban",
        "Compare how a developed and an emerging country prepared and responded, and judge which was more effective",
    ], "An 8-mark evaluate needs a judgement on effectiveness. Describing what happened in Katrina, however well, without saying how well it worked stays in Level 2."),

    ("geog_edxb:1.7", &[
        "Describe the inner core, outer core, mantle, asthenosphere and crust by temperature, density, composition and state",
        "Explain how radioactive decay in the core heats the mantle and drives convection currents",
        "Explain how convection, slab pull and ridge push move the plates",
        "Interpret a cross-section of the Earth",
    ], "The asthenosphere is the semi-molten upper mantle that plates move on. Calling the whole mantle 'liquid' is a common error that costs the mark."),

    ("geog_edxb:1.8", &[
        "Describe the distribution of conservative, convergent and divergent plate boundaries and hotspots on a world map",
        "Explain why convergent boundaries give explosive composite volcanoes with viscous, gas-rich magma",
        "Explain why divergent boundaries and hotspots give gentle shield volcanoes with runny basaltic lava",
        "Explain why earthquake hazards vary with depth and magnitude, and how undersea earthquakes cause tsunamis",
        "Use the Richter (or moment magnitude) scale to compare earthquakes",
    ], "Volcano type follows from magma type. Linking boundary, magma viscosity and explosivity in one chain is what turns a 2-mark answer into a 4-mark one."),

    ("geog_edxb:1.9", &[
        "Explain the primary and secondary impacts of the Tōhoku earthquake and tsunami in Japan (2011)",
        "Explain the primary and secondary impacts of the Gorkha earthquake in Nepal (2015)",
        "Evaluate short-term relief and long-term planning, preparation and prediction in each country",
        "Explain why impacts differed: level of development, building codes, remoteness and the hazard itself",
    ], "Japan shows that development does not remove risk: the tsunami overtopped sea walls. Top answers use that to judge effectiveness rather than claiming the rich country simply coped."),

    ("geog_edxb:2.1", &[
        "Contrast economic and broader social and political definitions of development",
        "Explain GDP per capita, the HDI, measures of inequality and corruption indices, and the limits of each",
        "Compare countries' rankings on a single measure with a composite one",
        "Explain how fertility, death, infant and maternal mortality rates and population pyramids differ between developing, emerging and developed countries",
    ], "The HDI combines life expectancy, education and income. Naming only 'wealth and health' misses the education component and the mark."),

    ("geog_edxb:2.2", &[
        "Explain social, historical, environmental, economic and political causes of global inequality, including colonialism",
        "Explain the consequences of inequality for people and countries",
        "Describe Rostow's five stages of modernisation and use it to explain development",
        "Explain Frank's dependency theory of core and periphery, and compare it with Rostow",
        "Use income quintiles to analyse inequality",
    ], "Rostow says every country can develop through stages; Frank says the core keeps the periphery poor. Mixing up which theory blames the rich countries is the classic slip."),

    ("geog_edxb:2.3", &[
        "Compare top-down and bottom-up strategies by scale, aims, funding and technology",
        "Explain how TNCs and governments drive globalisation, and why some countries benefit more than others",
        "Evaluate NGO-led intermediate technology, IGO-funded large infrastructure and TNC investment",
        "Use named examples of each approach",
    ], "Intermediate technology means affordable, locally repairable equipment. Describing it as 'cheap technology' without the local maintenance and skills point drops the second mark."),

    ("geog_edxb:2.4", &[
        "Describe India's site, situation and connectivity, and why its location matters nationally, regionally and globally",
        "Explain India's environmental and cultural diversity within the country",
        "Describe India's broad political, social, cultural and environmental context in South Asia and the world",
    ], "Site and situation are different: site is the physical land, situation is the position relative to other places. Swapping them is a common lost mark."),

    ("geog_edxb:2.5", &[
        "Describe India's economic trends since 1990: GDP, GNI per capita, changing sectors, trade and FDI",
        "Explain how the 1991 economic reforms opened India to foreign investment",
        "Explain the role of globalisation: communications, transport, TNCs and outsourcing, for example IT services in Bengaluru",
        "Explain the role of government policy: aid, education, infrastructure and pro-FDI policy",
        "Use numerical data and proportional flow-line maps to profile India's economy and trade",
    ], "Assess-the-importance questions need a weighed judgement. Listing globalisation factors without comparing them with government policy cannot reach Level 3."),

    ("geog_edxb:2.6", &[
        "Explain how rapid growth has changed India's fertility and death rates",
        "Explain rural-urban migration and city growth, and the core-periphery contrast between richer and poorer states",
        "Evaluate positive and negative impacts on different age and gender groups",
        "Explain environmental impacts at different scales: air, water and land pollution, health and greenhouse gases",
        "Calculate difference from the mean for core and periphery regions",
    ], "Impacts must name the group. 'Some people are better off' is vague; 'young urban graduates in IT jobs' versus 'elderly rural farmers' is specific and creditworthy."),

    ("geog_edxb:2.7", &[
        "Explain how India's regional influence and role in international organisations have grown",
        "Explain India's changing relationships with the EU and the USA",
        "Evaluate conflicting views on the costs and benefits of international relations and TNC investment",
    ], "Conflicting-views questions need named stakeholders with opposing views, such as a TNC, a government and local workers. One-sided answers are capped."),

    ("geog_edxb:3.1", &[
        "Describe past (since 1980), current and projected global urbanisation and how it differs between regions",
        "Describe the global pattern of megacities by size, location and growth rate",
        "Explain urban primacy and why some cities have disproportionate economic or political influence",
        "Calculate rates of change and percentage growth from line graphs",
    ], "A megacity has over 10 million people. Using 'a very big city' without the figure loses the definition mark."),

    ("geog_edxb:3.2", &[
        "Explain how economic change and national and international migration cause cities to grow or decline",
        "Explain why cities in developing, emerging and developed countries have different urban economies",
        "Compare formal and informal employment, the importance of each sector and working conditions",
    ], "The informal sector is work that is untaxed and unregulated, not simply 'illegal'. Calling it illegal is marked wrong."),

    ("geog_edxb:3.3", &[
        "Explain urbanisation, suburbanisation, de-industrialisation, counter-urbanisation and regeneration as stages of change",
        "Describe commercial, industrial and residential land uses",
        "Explain how accessibility, availability, cost and planning regulations shape land use",
        "Identify land-use zones on satellite images",
    ], "Counter-urbanisation is people leaving the city for rural areas; suburbanisation is moving to the edge of the city. Confusing them is the commonest error."),

    ("geog_edxb:3.4", &[
        "Explain the significance of Mumbai's site, situation and connectivity nationally, regionally and globally",
        "Describe Mumbai's structure: CBD, inner city, suburbs and urban-rural fringe, by function and building age",
        "Explain how the peninsula site and harbour shaped the city's growth",
    ], "Questions on Mumbai's location need two developed ways it helped the economy, such as the deep-water port and trade links. A bare list of features earns half marks."),

    ("geog_edxb:3.5", &[
        "Explain past and present population growth in Mumbai: natural increase, national and international migration, and investment",
        "Explain how growth has shaped Mumbai's spatial growth northwards and onto reclaimed land and the mainland",
        "Explain changing urban functions and land use as the city grows",
        "Use historic maps and satellite images to investigate spatial growth",
    ], "Migration and natural increase are separate causes. Strong answers explain both and say which matters more now, rather than treating growth as one cause."),

    ("geog_edxb:3.6", &[
        "Explain opportunities in Mumbai: education, health care, and formal and informal jobs",
        "Explain challenges caused by rapid growth: housing shortages, insecure property rights, water, waste, jobs, services and congestion",
        "Describe the contrast between areas of extreme wealth and informal settlements such as Dharavi",
        "Explain the political and economic challenges of managing a megacity",
        "Judge variations in quality of life using quantitative and qualitative data",
    ], "Dharavi is not only a problem area: it has a large informal economy. Answers that see slums only as failure miss the 'opportunities' half of an evaluate question."),

    ("geog_edxb:3.7", &[
        "Evaluate city-wide top-down strategies in Mumbai for water, waste, transport and air quality, such as Vision Mumbai and the Metro",
        "Evaluate community and NGO-led bottom-up strategies for housing, health and education",
        "Compare the advantages and disadvantages of the two approaches",
    ], "Top-down and bottom-up are judged on who benefits. Answers that praise a scheme without saying who was displaced or left out stay in Level 2."),

    ("geog_edxb:4.1", &[
        "Explain how geology and past tectonic and glacial processes shaped upland and lowland Britain",
        "Describe the characteristics and distribution of chalk, carboniferous limestone, clay, granite, schist and slate",
        "Explain the Tees-Exe line divide between harder upland rocks in the north and west and softer lowland rocks in the south and east",
        "Use simple geological cross-sections to link geology and relief",
    ], "Upland Britain is mostly igneous and metamorphic rock, lowland mostly sedimentary. Students who place granite in the south-east, or call chalk igneous, lose easy marks."),

    ("geog_edxb:4.2", &[
        "Explain how weathering, climate, post-glacial river and slope processes create distinctive upland and lowland landscapes",
        "Explain how agriculture, forestry and settlement have changed UK landscapes over time",
        "Locate key uplands, lowland basins and rivers on an outline map of the UK",
        "Recognise physical and human features on 1:25 000 and 1:50 000 OS maps",
    ], "Questions on human influence need a named activity and its effect on the landscape, for example drainage of fens for farming. 'Humans built towns' is too vague."),

    ("geog_edxb:4.3", &[
        "Explain how concordant and discordant coasts, joints and faults, and hard and soft rock shape erosional landforms",
        "Explain the formation of headlands and bays, caves, arches, stacks, cliffs and wave-cut platforms",
        "Explain how UK climate, destructive waves, weathering and mass movement control the rate of cliff retreat",
        "Explain how longshore drift and constructive waves form beaches, spits and bars",
        "Calculate mean rates of erosion and recognise coastal landforms on OS maps",
    ], "Landform sequences need every stage in order: crack, cave, arch, stack, stump. Skipping a stage or naming no process (hydraulic action, abrasion) caps the answer."),

    ("geog_edxb:4.4", &[
        "Explain how development, agriculture, industry and coastal management directly or indirectly change coastlines",
        "Explain why the Holderness coast erodes so fast: soft boulder clay, long fetch and narrow beaches",
        "Explain how defences at places such as Mappleton changed erosion further along the coast",
        "Explain the significance of Holderness's location, including the link to Spurn Head",
    ], "Defences have knock-on effects down-drift. Answers that judge Mappleton's groynes only by Mappleton itself miss the faster erosion to the south, which examiners reward."),

    ("geog_edxb:4.5", &[
        "Explain why coastal flood risk is rising: sea level rise, stormier weather and changing erosion and deposition",
        "Explain the threats to people and environments from coastal flooding and erosion",
        "Evaluate hard engineering (groynes, sea walls) and soft engineering (beach replenishment, slope stabilisation)",
        "Evaluate do nothing and strategic realignment within Integrated Coastal Zone Management",
        "Use simple cost-benefit analysis and OS maps to compare coastal defence options",
    ], "Conflict questions need named groups with opposing views, such as homeowners against the Environment Agency. A list of costs and benefits with no people in it stays in Level 2."),

    ("geog_edxb:4.6", &[
        "Describe how channel width and depth, valley profile, gradient, discharge, velocity and sediment change downstream, using the River Tees",
        "Explain erosion, transport and deposition processes and the formation of waterfalls, interlocking spurs, meanders, oxbow lakes, flood plains, levees and deltas",
        "Explain how climate, geology and slope processes influence river landscapes and sediment load",
        "Explain how geology, soil, slope, basin shape and antecedent conditions affect storm hydrographs and lag time",
        "Draw a valley cross-section from contours and recognise river landforms on OS maps",
    ], "Velocity usually increases downstream because the channel is smoother and deeper. Saying the river 'slows down' because the gradient is gentler is the most frequent wrong answer."),

    ("geog_edxb:4.7", &[
        "Explain how urbanisation, land-use change and deforestation alter storm hydrographs",
        "Explain how physical and human processes combined to flood the River Eden in Cumbria during Storm Desmond (December 2015)",
        "Explain the significance of the Eden's location, including Carlisle at the confluence of three rivers",
        "Draw a simple storm hydrograph from rainfall and discharge data",
    ], "Urban surfaces are impermeable, so water reaches the river faster and lag time shortens. Answers must give that chain; 'more buildings cause floods' scores one mark at most."),

    ("geog_edxb:4.8", &[
        "Explain why flood risk is increasing: more frequent storms and land-use change",
        "Explain the threats from river flooding to people and environments",
        "Evaluate hard engineering (flood walls, embankments, flood barriers) and soft engineering (flood-plain retention, river restoration)",
        "Use cost-benefit analysis and OS maps to judge river management options",
    ], "Soft engineering is not free of costs: flood-plain retention takes farmland out of use. Balanced answers name a cost for each approach."),

    ("geog_edxb:5.1", &[
        "Compare urban cores and rural areas by population density, age structure, economic activity and settlement",
        "Explain how UK and EU policies, such as enterprise zones, transport investment and regional development, tried to reduce differences",
        "Interpret UK population pyramids from different times",
    ], "Policy answers need a named policy and how it narrows the gap. 'The government gives money' is too vague for the second mark."),

    ("geog_edxb:5.2", &[
        "Explain how national and international migration over the past 50 years has changed the UK's population numbers, distribution and age structure",
        "Explain how UK and EU immigration policy has increased ethnic and cultural diversity",
        "Explain the decline of the primary and secondary sectors and the rise of the tertiary and quaternary sectors in contrasting regions",
        "Explain why globalisation, free trade and privatisation have increased FDI and the role of TNCs in the UK",
        "Use census data and Eurostat to investigate population change, FDI and immigration",
    ], "De-industrialisation needs its causes: cheaper overseas production, mechanisation and global competition. Saying 'factories closed' without why earns one mark."),

    ("geog_edxb:5.3", &[
        "Explain the significance of London's site, situation and connectivity nationally, regionally and globally",
        "Describe London's structure: CBD, inner city, suburbs and urban-rural fringe",
        "Compare the zones by function, building age and density, land use and environmental quality",
    ], "Connectivity means transport and communication links, such as Heathrow, the Thames, the rail network and the Channel Tunnel. Naming London's size or wealth instead does not answer it."),

    ("geog_edxb:5.4", &[
        "Explain the causes of national and international migration to London",
        "Explain how migration has shaped the age structure, ethnicity, housing, services and culture of different parts of the city",
        "Explain why employment, services, education and health differ between parts of London, for example Tower Hamlets and Richmond upon Thames",
        "Use census and IMD data to compare areas within the city",
    ], "Inequality answers must compare two named areas with evidence. A description of one poor area is only half an answer."),

    ("geog_edxb:5.5", &[
        "Explain how parts of London declined through de-industrialisation, depopulation and decentralisation",
        "Explain the effects of out-of-town shopping centres, retail and business parks, e-commerce and transport changes",
        "Explain how other parts grew through urban sprawl, financial services, TNC investment, gentrification, studentification, culture and leisure",
    ], "Gentrification brings investment but pushes out poorer residents through rising rents. Answers that treat it as wholly positive cannot score top marks."),

    ("geog_edxb:5.6", &[
        "Evaluate the positive and negative impacts of regeneration and rebranding on people, for example London Docklands and the Olympic Park at Stratford",
        "Explain strategies for more sustainable urban living: recycling, employment, green spaces, transport, and affordable energy-efficient housing",
        "Use crime and IMD data to judge the extent of inner-city problems",
    ], "Stratford's regeneration created jobs and homes, but few of the new homes were affordable to existing residents. Using that tension is what lifts an 8-mark answer."),

    ("geog_edxb:5.7", &[
        "Explain the flows of goods, services and labour between London and accessible rural areas",
        "Evaluate the economic, social and environmental costs and benefits of this interdependence for both",
        "Explain how a rural area in London's commuter belt, such as the Chilterns, has changed through counter-urbanisation, housing pressure and leisure",
    ], "Interdependence works both ways. Answers that only describe commuters going into London miss the food, water, leisure and labour flows back to the countryside."),

    ("geog_edxb:5.8", &[
        "Explain the challenges of housing availability and affordability, declining primary jobs, health care and education in rural areas",
        "Explain how these affect quality of life, measured by the IMD, for elderly and young people",
        "Evaluate rural diversification (farm shops, accommodation, leisure) and tourism projects, including their environmental impacts",
        "Identify rural land-use types on OS maps",
    ], "Rural deprivation is often hidden: wealthy commuters and poor older residents live in the same village. Averages for the whole area can mislead, and saying so earns credit."),

    ("geog_edxb:6a", &[
        "Write an enquiry question on how coastal management affects coastal processes and communities",
        "Explain a quantitative method for beach morphology and sediment (beach profiles, sediment size and roundness) and a qualitative method on management success",
        "Use a geology map and one other secondary source, and present and analyse the results",
        "Draw evidenced conclusions and evaluate the reliability and accuracy of methods, data and conclusions",
    ], "Fieldwork answers must refer to your own location and data. Generic textbook methods with no site, sample size or result stay in the lowest level."),

    ("geog_edxb:6b", &[
        "Write an enquiry question on how drainage basin and channel characteristics influence flood risk",
        "Explain a quantitative method for channel characteristics (width, depth, velocity, cross-sectional area) and a qualitative method on flood-risk factors",
        "Use a flood risk map and one other secondary source, and present and analyse the results",
        "Draw evidenced conclusions and evaluate the reliability and accuracy of methods, data and conclusions",
    ], "A method answer needs the equipment, how it was used and why it suits the question. 'We measured the river' earns nothing beyond the first mark."),

    ("geog_edxb:6c", &[
        "Write an enquiry question on how and why quality of life varies within an urban area",
        "Explain a qualitative method on views and perceptions of quality of life and a quantitative method on environmental quality",
        "Use census data (ONS) and one other secondary source, and present and analyse the results",
        "Draw evidenced conclusions and evaluate the reliability and accuracy of methods, data and conclusions",
    ], "Environmental quality surveys are subjective. Saying how you reduced the bias, for example by pairing up or using a fixed scoring sheet, earns the evaluation marks."),

    ("geog_edxb:6d", &[
        "Write an enquiry question on how and why deprivation varies within a rural area",
        "Explain a qualitative method on views and perceptions of rural life and a quantitative method on environmental quality",
        "Use census data (ONS) and one other secondary source, and present and analyse the results",
        "Draw evidenced conclusions and evaluate the reliability and accuracy of methods, data and conclusions",
    ], "Small rural samples are a weakness. Evaluate questions reward naming the sample size and explaining why it limits how far the conclusion can be trusted."),

    ("geog_edxb:7.1", &[
        "Describe the global distribution and characteristics of tropical, temperate and boreal forests, grasslands, deserts and tundra",
        "Explain how temperature, precipitation and sunshine hours control biome distribution",
        "Explain how altitude, rock, soil and drainage alter biomes locally",
        "Explain how the biotic and abiotic components of a biome interact",
        "Compare climate graphs for different biomes",
    ], "Describe-the-distribution answers need latitudes and named continents, such as tundra north of about 60° N in Canada and Russia. 'Near the poles' alone earns one mark."),

    ("geog_edxb:7.2", &[
        "Explain how the biosphere provides food, medicine, building materials and fuel for indigenous and local people",
        "Explain commercial exploitation of the biosphere for energy, water and minerals",
        "Explain the biosphere's services: regulating the atmosphere, soil health and the water cycle",
        "Explain how population growth, affluence, urbanisation and industrialisation raise demand for food, energy and water",
        "Compare the views of Malthus and Boserup on population and resources",
    ], "Malthus predicted population would outstrip food; Boserup argued necessity drives invention. Swapping the two is marked wrong every time."),

    ("geog_edxb:8.1", &[
        "Explain how climate, soil, water, plants, animals and people are interdependent in the tropical rainforest",
        "Explain plant adaptations (layers, buttress roots, drip tips) and animal adaptations to the climate",
        "Explain why nutrient cycling is fast, and how this supports high biodiversity and complex food webs",
        "Interpret nutrient cycle and food web diagrams",
    ], "Rainforest soils are poor because nutrients sit in the biomass, not the soil. Saying the soil is fertile is a common misconception that loses the mark."),

    ("geog_edxb:8.2", &[
        "Explain how climate, soil, water, plants, animals and people are interdependent in the taiga",
        "Explain taiga plant adaptations (cone shape, needles, simple structure) and migratory animal adaptations",
        "Explain why the taiga has low productivity, slow nutrient cycling and low biodiversity",
    ], "The taiga's nutrient store is the litter, because cold slows decomposition. Students who copy the rainforest pattern (biomass store) lose the comparison marks."),

    ("geog_edxb:8.3", &[
        "Explain the direct causes of deforestation: hardwood logging, subsistence and commercial farming, and fuelwood",
        "Explain how demand for biofuels, minerals and hydroelectric power adds to deforestation",
        "Explain why climate change is an indirect threat through drought and ecosystem stress",
        "Use GIS and satellite images to identify the pattern of forest loss",
    ], "Direct and indirect threats are marked separately. Calling climate change a direct cause of deforestation shows you have not read the spec's distinction."),

    ("geog_edxb:8.4", &[
        "Explain direct threats to the taiga from softwood logging and pulp and paper production",
        "Explain indirect threats from minerals, fossil fuels (such as the Alberta oil sands) and HEP",
        "Explain how acid precipitation, forest fires, pests and diseases reduce biodiversity",
    ], "Acid rain damages trees by leaching nutrients from the soil and harming needles. 'It burns the trees' is not accepted."),

    ("geog_edxb:8.5", &[
        "Evaluate global actions to protect rainforests: CITES and REDD",
        "Explain why deforestation rates are rising in some areas and falling in others",
        "Explain the challenge of sustainable forest management",
        "Evaluate alternative livelihoods such as ecotourism and sustainable farming",
    ], "CITES controls trade in endangered species; REDD pays countries to keep forests standing. Describing either as 'banning logging' is wrong."),

    ("geog_edxb:8.6", &[
        "Explain the challenges of creating and maintaining wilderness areas, national parks and sustainable forestry in the taiga",
        "Explain the conflicting views of governments, companies, indigenous peoples and conservationists on protecting or exploiting the taiga",
        "Judge which view should take priority, using evidence",
    ], "Protecting the taiga is hard because it is vast and remote, which makes monitoring costly. That practical point is what most answers leave out."),

    ("geog_edxb:9.1", &[
        "Classify energy resources as non-renewable (coal, oil, gas), renewable (solar, wind, HEP) and recyclable (nuclear, biofuels)",
        "Explain the environmental impacts of mining and drilling: landscape scarring, oil spills, carbon emissions and forest loss",
        "Explain the landscape impacts of renewables: reservoirs flooding land, land use for wind turbines and solar farms",
    ], "Nuclear and biofuels are 'recyclable' in this spec, not renewable. Pearson marks the spec's own classification."),

    ("geog_edxb:9.2", &[
        "Explain how access to energy depends on technology and physical resources: geology, accessibility, climate and landscape",
        "Describe the global pattern of energy use per person",
        "Explain why energy use varies: economic development, reliance on traditional fuels and demand from different sectors",
        "Interpret world maps of energy resources",
    ], "Describe-the-pattern answers need data or named regions, such as North America very high and sub-Saharan Africa very low. 'Rich countries use more' alone is one mark."),

    ("geog_edxb:9.3", &[
        "Describe how oil reserves and production are unevenly distributed",
        "Explain why oil consumption is rising: higher GDP per capita and rapid industrialisation in emerging economies",
        "Explain how conflicts, diplomatic relations, recession, boom and over- or under-supply affect oil supply and price",
        "Draw and interpret graphs of oil price and production over time",
    ], "Price questions need the supply-demand link: a war that cuts supply raises the price, a recession that cuts demand lowers it. Naming an event without the mechanism scores one mark."),

    ("geog_edxb:9.4", &[
        "Evaluate the economic benefits and costs of new conventional oil and gas in sensitive or isolated areas such as the Arctic",
        "Explain the environmental costs of tar sands and shale gas for water quality and ecosystems",
        "Weigh energy security against environmental damage",
    ], "Tar sands and fracking are 'unconventional'; Arctic drilling is new but conventional. Pearson questions separate the two, so use the right label."),

    ("geog_edxb:9.5", &[
        "Explain how energy efficiency and conservation in transport and the home reduce demand and emissions",
        "Evaluate biofuels, wind, solar and HEP as alternatives to fossil fuels",
        "Evaluate hydrogen as a future technology",
        "Explain how alternatives improve energy security and diversify the energy mix",
    ], "Every alternative has a cost, such as intermittency for wind and solar or land use for biofuels. Evaluations that list only benefits cannot reach Level 3."),

    ("geog_edxb:9.6", &[
        "Explain the contrasting views of consumers, TNCs, governments, climate scientists and environmental groups on energy futures",
        "Compare business as usual with a sustainable energy future",
        "Explain how rising affluence, environmental concern and education change attitudes in some developed countries",
        "Calculate carbon and ecological footprints",
        "Plan and write the Paper 3 Section D 12-mark decision: choose one option, justify it from the booklet, and weigh it against the other two",
    ], "The 12-mark decision rewards rejecting the other options with reasons. Answers that only praise the chosen option, or ignore the resource booklet, are capped in Level 2."),

    // ---------- Combined Science (AQA GCSE Combined Science: Trilogy (8464) Higher) ----------
    ("combsci_aqa:4.1.1", &[
        "Compare eukaryotic and prokaryotic cells, including plasmids and the DNA loop",
        "Link each sub-cellular structure in plant, animal and bacterial cells to its function",
        "Explain how sperm, nerve, muscle, root hair, xylem and phloem cells are adapted",
        "Use magnification = image size / real size with unit conversions and standard form",
        "Compare light and electron microscopes in terms of magnification and resolution",
        "Describe aseptic technique and calculate bacterial numbers and clear-zone areas (separate science only)",
    ], "In magnification sums, convert both sizes to the same unit before dividing. 1 mm is 1000 µm, and magnification has no unit."),

    ("combsci_aqa:4.1.2", &[
        "Describe chromosomes, genes and how chromosomes are paired in body cells",
        "Describe the three stages of the cell cycle and why DNA and organelles are copied first",
        "Recognise growth, repair and replacement as situations where mitosis happens",
        "Calculate the time spent in mitosis from cell counts",
        "Describe the function of stem cells in embryos, adult bone marrow and plant meristems",
        "Evaluate the benefits, risks and ethical issues of stem cell treatments and therapeutic cloning",
    ], "Describe the cell cycle in AQA's three stages and end with two genetically identical cells. In stem cell questions, \"evaluate\" needs both sides and a conclusion."),

    ("combsci_aqa:4.1.3", &[
        "Define diffusion, osmosis and active transport and explain how they differ",
        "Explain how concentration gradient, temperature and surface area affect the rate of diffusion",
        "Calculate surface area to volume ratios and use them to explain the need for exchange surfaces and transport systems",
        "Explain how the small intestine, lungs, gills, roots and leaves are adapted for exchange",
        "Carry out the osmosis practical, calculate percentage change in mass and interpret the graph",
    ], "An osmosis definition needs water, dilute to more concentrated solution, and a partially permeable membrane. Active transport needs both \"against the gradient\" and \"energy from respiration\"."),

    ("combsci_aqa:4.2.1", &[
        "Define cell, tissue, organ and organ system using AQA's wording",
        "Put the levels of organisation in order from sub-cellular structure to organism",
        "Classify examples such as blood, the heart and the leaf into the correct level",
        "Explain how the tissues in an organ such as the stomach each contribute to its function",
        "Compare the sizes of cells, tissues, organs and systems using the same units and standard form",
    ], "An organ is a group of different tissues, not just a lot of cells. Blood is a tissue, the leaf is an organ, and those two catch people out most."),

    ("combsci_aqa:4.2.2a", &[
        "Recall where amylase, proteases and lipases are made, where they work and their products",
        "Explain enzyme specificity using the active site and the lock and key model",
        "Explain the effect of temperature and pH on enzyme activity, using denaturing",
        "Explain how bile neutralises acid and emulsifies fat to speed up lipase",
        "Carry out the food tests and the amylase pH practical, and calculate rates as 1/time",
    ], "Enzymes are denatured, not killed: say the active site changes shape so the substrate no longer fits. Bile is not an enzyme; it emulsifies fat and neutralises acid."),

    ("combsci_aqa:4.2.2b", &[
        "Trace blood through the double circulatory system, naming the chambers and the aorta, vena cava, pulmonary artery, pulmonary vein and coronary arteries",
        "Explain how the lungs are adapted for gas exchange, from trachea and bronchi to alveoli and their capillaries",
        "Describe the natural pacemaker in the right atrium and what artificial pacemakers do",
        "Explain how the structures of arteries, veins and capillaries suit their functions",
        "Calculate rates of blood flow and heart rate with the right units",
        "Identify red cells, white cells and platelets and explain how each is adapted, alongside the role of plasma",
    ], "The pulmonary artery carries deoxygenated blood and the pulmonary vein oxygenated blood. And arteries do not pump: their thick walls withstand high pressure."),

    ("combsci_aqa:4.2.2c", &[
        "Explain how fatty deposits in the coronary arteries starve heart muscle of oxygen",
        "Evaluate stents, statins, replacement valves, transplants and artificial hearts, weighing benefits against risks",
        "Describe how health is affected by disease, diet, stress and life situations, and how different diseases interact",
        "Link named lifestyle risk factors to their diseases and discuss their human and financial costs",
        "Interpret disease data from tables, charts and scatter diagrams, judging sampling and correlation versus cause",
        "Distinguish benign from malignant tumours and name lifestyle and genetic risk factors for cancer",
    ], "A correlation between a risk factor and a disease does not prove cause on its own. Say a causal mechanism is needed, or that other factors could be involved."),

    ("combsci_aqa:4.2.3", &[
        "Explain how the epidermis, palisade and spongy mesophyll, xylem, phloem, meristem and guard cells suit their jobs in the leaf and plant",
        "Explain how root hair cells take up water by osmosis and mineral ions by active transport",
        "Describe transpiration and translocation and compare xylem with phloem",
        "Explain how temperature, humidity, air movement and light intensity change the rate of transpiration",
        "Measure transpiration with a potometer and calculate rates, means and volumes from your readings",
    ], "Root hair cells take in water by osmosis but mineral ions by active transport. And humidity slows transpiration because the water vapour concentration gradient is less steep."),

    ("combsci_aqa:4.3.1a", &[
        "Name the four types of pathogen and explain how bacteria and viruses make us ill",
        "Explain how pathogens spread by direct contact, water and air, and how each route can be blocked",
        "Give the pathogen, symptoms, spread and control for measles, HIV and TMV",
        "Do the same for Salmonella, gonorrhoea, rose black spot and malaria",
        "Explain why TMV and rose black spot reduce plant growth",
        "Interpret data on cases of a disease before and after a control measure",
    ], "Malaria is caused by a protist, and the mosquito is only the vector. For every control method, say which link in the chain of spread it breaks."),

    ("combsci_aqa:4.3.1b", &[
        "Describe how the skin, nose, trachea and bronchi, and stomach stop pathogens getting in",
        "Explain how white blood cells defend the body by phagocytosis, antibody production and antitoxin production",
        "Explain how vaccination protects a person, and how immunising most of a population stops a pathogen spreading",
        "Explain what antibiotics and painkillers can and cannot do, including why antibiotics do not work on viruses",
        "Describe how a new drug is discovered and tested, from preclinical work to double blind trials and peer review",
    ], "Antibiotics kill bacteria only, never viruses, and painkillers kill nothing at all. Say it plainly whenever a question mentions a viral illness."),

    ("combsci_aqa:4.4.1", &[
        "Write the word equation for photosynthesis, recognise the symbols, and explain why it is endothermic",
        "Explain how light intensity, carbon dioxide, temperature and chlorophyll affect the rate, and calculate rates",
        "Read limiting-factor graphs, including two- and three-factor graphs and the inverse square law (Higher tier)",
        "Carry out and evaluate Required practical 6 (Trilogy 5) on light intensity and pondweed",
        "Use limiting factors to judge the cost-effectiveness of heat, light and CO2 in greenhouses (Higher tier)",
        "List the uses of glucose in plants and explain why nitrate is also needed",
    ], "On a plateau, \"another factor is limiting\" is the mark, and at Higher you must name it from the other lines. Doubling the lamp distance quarters the light, it does not halve it."),

    ("combsci_aqa:4.4.2", &[
        "Describe respiration as a continuous exothermic reaction and list what organisms use the energy for",
        "Write the equations for aerobic respiration and for anaerobic respiration in muscles, plants and yeast",
        "Compare aerobic and anaerobic respiration: oxygen, products and energy transferred",
        "Explain the body's response to exercise, lactic acid and fatigue, and oxygen debt (Higher tier)",
        "Explain metabolism and the role of sugars, amino acids, fatty acids and glycerol",
    ], "Respiration transfers energy; it never \"produces\" it. Muscles make lactic acid only, while yeast and plants make ethanol and carbon dioxide."),

    ("combsci_aqa:4.5.1", &[
        "Define homeostasis as regulating internal conditions to keep optimum conditions for function",
        "Explain why homeostasis matters for enzyme action and cell functions",
        "Name the conditions controlled in the human body: blood glucose, body temperature and water levels",
        "Describe the roles of receptors, coordination centres and effectors, and put the control chain in order",
        "Apply the control-system pattern to unfamiliar examples and data",
    ], "Effectors are only muscles or glands, and a receptor is a cell. Call the brain an effector, or define homeostasis as \"keeping things the same\", and the mark goes."),

    ("combsci_aqa:4.5.2", &[
        "Describe the pathway from stimulus to response, naming receptor, CNS and effector",
        "Explain how each structure in a reflex arc, including the synapse and relay neurone, relates to its function and why reflexes are fast",
        "Plan the ruler-drop reaction-time practical, naming variables, calculating means and converting results to graphs",
        "Identify the cerebral cortex, cerebellum and medulla and explain why the brain is hard to investigate and treat (biology only)",
        "Explain accommodation, adaptation to dim light, and how lenses correct myopia and hyperopia (biology only)",
        "Explain how vasodilation, vasoconstriction, sweating and shivering control body temperature (biology only)",
    ], "Impulses are electrical along neurones but a chemical diffuses across the synapse, and reflexes skip the conscious brain, not the whole CNS. In the eye, the ciliary muscles contract while the suspensory ligaments slacken; ligaments never contract."),

    ("combsci_aqa:4.5.3a", &[
        "Describe how the endocrine system works and compare it with the nervous system",
        "Identify the pituitary, thyroid, adrenal glands, pancreas, ovaries and testes on a diagram of the body",
        "Explain how insulin, and at Higher tier glucagon, control blood glucose by negative feedback",
        "Compare Type 1 and Type 2 diabetes and their treatments, and interpret glucose graphs",
        "Explain how the kidneys filter and selectively reabsorb, and how ADH controls water balance (biology only)",
        "Describe how dialysis works and evaluate it against a kidney transplant (biology only)",
    ], "Glucose, glycogen and glucagon get mixed up more than anything else on this topic. Insulin moves glucose into cells, and glucose is stored as glycogen in liver and muscle."),

    ("combsci_aqa:4.5.3b", &[
        "Describe the roles of oestrogen and testosterone at puberty and of FSH, LH, oestrogen and progesterone in the menstrual cycle",
        "Explain how FSH, oestrogen, LH and progesterone stimulate and inhibit each other, and read hormone graphs (Higher tier)",
        "Explain how each hormonal and non-hormonal method of contraception works and evaluate them",
        "Describe how FSH and LH fertility drugs and IVF treat infertility, and evaluate the issues (Higher tier)",
        "Explain the roles of adrenaline and thyroxine and how negative feedback controls thyroxine (Higher tier)",
    ], "FSH matures the egg and LH releases it, and both come from the pituitary, not the ovary. On a graph, LH is the sharp spike just before ovulation."),

    ("combsci_aqa:4.6.1a", &[
        "Compare sexual and asexual reproduction, including which type of cell division each uses",
        "Explain how meiosis halves the chromosome number and fertilisation restores it",
        "Explain the advantages of each type of reproduction using malarial parasites, fungi, strawberries and daffodils (biology only)",
        "Describe DNA, genes and the genome, and discuss why understanding the human genome matters",
        "Describe DNA as a polymer of nucleotides and use the triplet code and A–T, C–G pairing in calculations (biology only)",
        "Describe protein synthesis and explain how mutations in coding and non-coding DNA can change a protein or its expression (Higher tier, biology only)",
    ], "Meiosis gives four genetically different cells with half the chromosomes, while mitosis gives two identical ones. Gametes carry a single set (23 in humans), not 23 pairs."),

    ("combsci_aqa:4.6.1b", &[
        "Explain gamete, chromosome, gene, allele, dominant, recessive, homozygous, heterozygous, genotype and phenotype",
        "Complete and (Higher tier) construct Punnett squares, giving outcomes as ratios, probabilities and percentages",
        "Interpret family trees to decide whether an allele is dominant or recessive and work out genotypes",
        "Describe polydactyly (dominant) and cystic fibrosis (recessive) and make informed judgements about embryo screening",
        "Explain sex determination with XX and XY and show it with a genetic cross",
    ], "A 1 in 4 chance is a probability for each child, not exactly one in every four children. And heterozygous means two different alleles, not two different genes."),

    ("combsci_aqa:4.6.2", &[
        "Describe genetic, environmental and combined causes of variation and state that all variants arise from mutations",
        "Explain how evolution happens by natural selection, step by step, in any context",
        "Describe the process of selective breeding and explain its benefits and the risks of inbreeding",
        "Evaluate genetic engineering and GM crops, and (Higher tier) describe its main steps using enzymes and a vector",
        "Separate science only: describe tissue culture, cuttings, embryo transplants and adult cell cloning, with their benefits and risks",
    ], "In natural selection answers people skip 'passes on the allele to offspring' and 'over many generations', or say the organism adapted on purpose. Mutations are random; the environment only selects."),

    ("combsci_aqa:4.6.3", &[
        "Describe the evidence for evolution: fossils, antibiotic resistance and the inheritance of genes",
        "Describe three ways fossils form and explain why the fossil record is incomplete",
        "Explain how antibiotic-resistant bacteria such as MRSA develop and how to slow their spread",
        "Describe factors that cause extinction and read evolutionary trees",
        "Separate science only: describe the work of Darwin, Wallace, Lamarck and Mendel, why their ideas took time to be accepted, and the steps of speciation",
    ], "Saying the antibiotic made the bacteria mutate. The resistant mutation was already there by chance; the antibiotic just kills the rest, so the resistant strain survives and multiplies."),

    ("combsci_aqa:4.6.4", &[
        "List Linnaeus's seven levels of classification in order, from kingdom to species",
        "Use binomial names (genus and species) to decide which organisms are most closely related",
        "Explain how better microscopes and chemical analysis changed classification",
        "Name Woese's three domains and say what each contains",
        "Interpret evolutionary trees to find common ancestors, relatedness and extinct species",
    ], "Getting the seven levels in the wrong order, or naming the three domains as animals, plants and bacteria. They are Archaea, Bacteria and Eukaryota."),

    ("combsci_aqa:4.7.1", &[
        "Describe the levels of organisation from individual to population, community and ecosystem",
        "Suggest the resources plants and animals compete for in a given habitat",
        "Explain interdependence and how removing one species affects a whole community",
        "Explain how changes in abiotic and biotic factors affect a community, using data and calculating percentage change",
        "Explain structural, behavioural and functional adaptations, including extremophiles",
    ], "Mixing up community (living things only) with ecosystem (living plus non-living), and saying plants compete for food. Plants compete for light, space, water and mineral ions."),

    ("combsci_aqa:4.7.2", &[
        "Name producers and primary, secondary and tertiary consumers in a food chain",
        "Explain predator–prey cycles, including why predator numbers lag behind prey",
        "Estimate population size from random quadrats and use transects to link distribution to an abiotic factor (Required practical 9; Trilogy 7)",
        "Explain how the carbon and water cycles work and the role of microorganisms in returning CO₂ and mineral ions",
        "Explain how temperature, water and oxygen affect the rate of decay, including compost, biogas and the milk practical (separate science only)",
        "Evaluate how changes in temperature, water and atmospheric gases affect species distribution (separate science only, Higher tier)",
    ], "In quadrat sums, work out how many quadrats fit in the whole area (a 50 cm quadrat is 0.25 m²) before multiplying by the mean. On predator–prey graphs, say the predator peak lags behind the prey peak and give the reason."),

    ("combsci_aqa:4.7.3", &[
        "Define biodiversity and explain why high biodiversity keeps ecosystems stable",
        "Describe how pollution of water, air and land, and land use for building, quarrying, farming and landfill, reduce biodiversity",
        "Explain why destroying peat bogs and tropical forests reduces biodiversity and raises carbon dioxide levels",
        "Describe biological consequences of global warming and why the evidence is trusted but still partly uncertain",
        "Evaluate breeding programmes, habitat protection, hedgerows, emission cuts and recycling as ways to maintain biodiversity",
        "Calculate percentage changes and rates from environmental data",
    ], "Deforestation raises carbon dioxide in two ways: burning and decay release it, and fewer trees remove it by photosynthesis. Evaluate questions need both sides and a conclusion."),

    ("combsci_aqa:5.1.1", &[
        "Tell elements, compounds and mixtures apart, and choose a separation technique for a given mixture",
        "Describe how the model of the atom changed from solid spheres to the nuclear model, linking each change to its evidence",
        "Work out the protons, neutrons and electrons in any atom or ion from its atomic and mass numbers",
        "Calculate relative atomic mass from isotope abundances",
        "Write the electronic structure of the first 20 elements as numbers and as diagrams",
        "Write and balance symbol equations, and (Higher tier) half and ionic equations",
    ], "In alpha scattering answers, each observation has to be tied to what it shows: most went straight through, so the atom is mostly empty space. A list of observations with no conclusions drops most of the marks."),

    ("combsci_aqa:5.1.2", &[
        "Link an element's position in the periodic table to its electron arrangement and atomic number",
        "Describe how Mendeleev built his table and why his gaps and predictions got it accepted",
        "Explain the differences between metals and non-metals using their properties and outer electrons",
        "Describe the reactions of lithium, sodium and potassium with water, oxygen and chlorine, and write their equations",
        "Explain the opposite reactivity trends in Group 1 and Group 7, and predict properties down Groups 0, 1 and 7",
        "Predict and describe halogen displacement reactions",
    ], "Reactivity trends need the whole chain: more shells, outer electron further from the nucleus, weaker attraction, so lost more easily in Group 1 or gained less easily in Group 7. Stopping at \"the atom is bigger\" drops the marks."),

    ("combsci_aqa:5.2.1", &[
        "Decide whether a substance has ionic, covalent or metallic bonding from the elements in it",
        "Work out ion charges from group numbers and draw dot and cross diagrams for Group 1 or 2 metals with Group 6 or 7 non-metals",
        "Draw dot and cross diagrams for H₂, Cl₂, O₂, N₂, HCl, H₂O, NH₃ and CH₄, and line diagrams for molecules, polymers and giant structures",
        "Describe the structure of sodium chloride and the limitations of each way of representing it",
        "Deduce empirical and molecular formulae from diagrams and models",
        "Describe metallic bonding as positive ions held by delocalised electrons",
    ], "The ionic bond is the electrostatic attraction between oppositely charged ions, not the electron transfer and never \"sharing\". In dot and cross diagrams, missing square brackets, charges or lone pairs costs marks every time."),

    ("combsci_aqa:5.2.2", &[
        "Use the particle model to explain melting, boiling, freezing and condensing, and (Higher tier) state its limitations",
        "Predict the state of a substance at a given temperature from its melting and boiling points",
        "Explain the melting points and conductivity of ionic compounds, small molecules, polymers, giant covalent structures and metals from their bonding",
        "Explain why larger molecules have higher boiling points and why alloys are harder than pure metals",
        "Add the correct state symbols to equations",
    ], "When a substance made of small molecules melts or boils, the weak intermolecular forces are overcome, not the covalent bonds. Saying the bonds break is the most common mark lost on this topic."),

    ("combsci_aqa:5.2.3", &[
        "Explain the hardness, high melting point and non-conductivity of diamond from its four covalent bonds per carbon",
        "Explain why graphite is soft, has a high melting point and conducts, using its layers and delocalised electrons",
        "Describe graphene as a single layer of graphite and link its properties to uses in electronics and composites",
        "Recognise diamond, graphite, graphene, C₆₀ and carbon nanotubes from diagrams and descriptions",
        "Give uses of fullerenes and nanotubes and explain why C₆₀ melts far lower than diamond",
    ], "Graphite is soft because there are no covalent bonds between its layers, only weak forces, and it conducts because one electron per carbon is delocalised and carries charge. Saying \"weak covalent bonds between layers\" or \"strong bonds\" without \"many\" and \"lots of energy\" loses the mark."),

    ("combsci_aqa:5.3.1", &[
        "State the law of conservation of mass and balance symbol equations by changing only the numbers in front of formulae",
        "Calculate relative formula mass, including formulae with brackets, and show that masses in a balanced equation add up",
        "Calculate the percentage by mass of an element in a compound",
        "Explain apparent mass changes in open containers when a gas is gained from or lost to the air",
        "Calculate a mean, its range and its uncertainty, and write a result as mean ± uncertainty",
    ], "Mass is never \"lost\" — in an open container a gas escapes or oxygen joins from the air, and you must say so. In calculations, multiply out brackets and big numbers fully and never give Mr a unit."),

    ("combsci_aqa:5.3.2", &[
        "(Higher) Convert between mass, moles and number of particles using Mr and the Avogadro constant, 6.02 × 10²³ per mole",
        "(Higher) Calculate the mass of a reactant or product from a balanced equation and the mass of another substance",
        "(Higher) Work out the balancing numbers in an equation from the masses of reactants and products",
        "(Higher) Identify the limiting reactant and explain how it fixes the amount of product",
        "Calculate concentration in g/dm³ and the mass of solute in a given volume, converting cm³ to dm³ first",
    ], "Read the mole ratio from the balanced equation every time — 2Mg : O₂ is 2 : 1, not 1 : 1 — and always convert cm³ to dm³ before using the concentration equation."),

    ("combsci_aqa:5.4.1", &[
        "Explain oxidation and reduction as gain and loss of oxygen, and identify what is oxidised and reduced in an equation",
        "Recall the reactions of potassium, sodium, lithium, calcium, magnesium, zinc, iron and copper with water and dilute acids, and place them in order of reactivity",
        "Explain reactivity in terms of a metal's tendency to form positive ions, and deduce an order of reactivity from experimental results",
        "Predict displacement reactions and explain why metals below carbon are extracted by reduction with carbon",
        "(Higher tier) Define oxidation and reduction in terms of electrons and write ionic and half equations for displacement reactions",
    ], "Students lose marks by naming the metal rather than the metal oxide or metal ion as the substance reduced, and by writing ionic equations that still contain spectator ions or whose charges do not balance."),

    ("combsci_aqa:5.4.2", &[
        "Predict the salt and other products when acids react with metals, alkalis, bases and carbonates, and deduce salt formulae from the charges on ions",
        "Describe how to make a pure, dry sample of a soluble salt from an insoluble oxide or carbonate (Required practical 1; Trilogy Required practical 8)",
        "Use universal indicator or a pH probe to find pH, and explain neutralisation as H⁺ + OH⁻ → H₂O",
        "(Separate science only) Describe how to carry out an accurate titration and, at Higher tier, calculate concentrations in mol/dm³ and g/dm³ (Required practical 2)",
        "(Higher tier) Explain metal–acid reactions as redox, and distinguish strong/weak from concentrated/dilute acids",
        "(Higher tier) Use the rule that each fall of one pH unit means ten times the hydrogen ion concentration",
    ], "The most common slips are confusing strong with concentrated, and leaving out a reason or a step (excess solid, filtering, evaporating to the crystallisation point) in the salt-making method."),

    ("combsci_aqa:5.4.3", &[
        "Explain why ionic compounds conduct when molten or dissolved, and which ions move to the cathode and the anode",
        "Predict the products of electrolysing molten binary ionic compounds and aqueous solutions with inert electrodes",
        "Explain why aluminium is extracted by electrolysis of aluminium oxide in cryolite and why the carbon anode must be replaced",
        "Plan Required practical 3 (Trilogy Required practical 9): electrolyse aqueous solutions, state a hypothesis and identify the products with gas tests",
        "(Higher tier) Write, complete and balance half equations at each electrode and identify them as oxidation or reduction",
    ], "In aqueous solutions people name the metal (such as sodium) at the cathode or sulfur at the anode, instead of applying the rules: hydrogen unless the metal is less reactive than hydrogen, oxygen unless a halide is present."),

    ("combsci_aqa:5.5.1", &[
        "Decide whether a reaction is exothermic or endothermic from the temperature change of the surroundings, and give examples and uses of each",
        "Evaluate hand warmers, self-heating cans and cold packs from given data, ending with a justified judgement",
        "Carry out and evaluate Required practical 4 (Trilogy 10): variables, insulation, highest temperature and the crossing point of two best-fit lines",
        "Draw and read reaction profiles, with activation energy from reactants to peak and overall change from reactants to products",
        "(Higher tier) Calculate the energy change from bond energies as bonds broken minus bonds formed, and explain the sign in terms of bonds",
    ], "Activation energy arrows drawn from the axis or the products instead of from the reactants lose easy marks. In bond energy sums, forgetting the balancing numbers (2O₂ is two O=O bonds) or dropping the minus sign throws away the answer mark."),

    ("combsci_aqa:5.6.1", &[
        "Calculate mean rates in g/s or cm³/s, and (Higher tier) in mol/s, from the quantity used or formed and the time taken",
        "Draw and interpret product-against-time graphs, draw tangents, and (Higher tier) calculate a tangent's gradient as the rate at a given time",
        "Explain the effects of concentration, pressure, surface area, temperature and catalysts using collision theory and activation energy",
        "Use surface area to volume ratios and simple proportionality to predict how rate changes",
        "Plan and evaluate Required practical 5 (Trilogy 11): gas volume and disappearing-cross methods, variables, rate = 1/time and safety",
    ], "Write \"more frequent collisions\", not just \"more collisions\", and for temperature give both effects: more frequent collisions and more collisions with at least the activation energy. Catalysts lower the activation energy by giving another pathway; they do not give particles more energy."),

    ("combsci_aqa:5.6.2", &[
        "Write reversible reactions with the ⇌ symbol and describe the ammonium chloride and hydrated copper sulfate examples, including colours",
        "Explain that a reversible reaction is exothermic one way and endothermic the other, transferring the same amount of energy",
        "Describe dynamic equilibrium in a closed system as forward and reverse reactions at the same rate with constant amounts",
        "(Higher tier) Use Le Chatelier's Principle to predict and explain the effects of concentration, temperature and pressure on the position of equilibrium",
        "(Higher tier) Interpret yield data to deduce whether a forward reaction is exothermic or endothermic and which side has fewer gas molecules",
    ], "At equilibrium the rates are equal and the amounts are constant, not equal, and the reactions have not stopped. For Le Chatelier answers, name the direction favoured and why (endothermic for a temperature rise, fewer gas molecules for a pressure rise) before giving the effect on yield."),

    ("combsci_aqa:5.7.1", &[
        "Describe crude oil as a finite mixture of hydrocarbons, name the first four alkanes and recognise alkanes from CₙH₂ₙ₊₂ and displayed formulae",
        "Explain fractional distillation in terms of evaporation, a temperature gradient and condensation at each boiling point",
        "Recall how boiling point, viscosity and flammability change with molecule size and link this to use as fuels",
        "Write balanced equations for the complete combustion of hydrocarbons and for cracking reactions",
        "Describe catalytic and steam cracking, the bromine water test for alkenes, and why cracking is needed (supply and demand, polymers)",
    ], "In fractional distillation, fractions are separated because they condense at different heights where the column is cooler than their boiling point; \"they boil off at different levels\" loses the mark. For the alkene test say bromine water turns from orange to colourless, not \"clear\"."),

    ("combsci_aqa:5.8.1", &[
        "Use melting and boiling point data to decide whether a substance is pure, and contrast the chemical and everyday meanings of pure",
        "Identify a formulation from given information: a designed mixture with measured components, each with a purpose",
        "Explain how paper chromatography separates a mixture in terms of the stationary phase and the mobile phase",
        "Calculate Rf values from chromatograms to a sensible number of significant figures, and use them to identify substances and judge purity",
        "Carry out and evaluate Required practical 6 (Trilogy 12): pencil start line above the solvent, small spots, lid, solvent front marked",
    ], "Rf answers bigger than 1 (divided the wrong way round) or distances measured from the bottom of the paper instead of the start line throw away the calculation marks. Saying impurities raise the melting point loses easy marks: they lower it and spread it over a range."),

    ("combsci_aqa:5.8.2", &[
        "Describe the test and positive result for hydrogen: a burning splint at the mouth of the tube gives a squeaky pop",
        "Describe the test and positive result for oxygen: a glowing splint relights",
        "Describe the test and positive result for carbon dioxide: bubbled through limewater it turns milky, and explain the calcium carbonate precipitate",
        "Describe the test and positive result for chlorine: damp litmus paper is bleached white, done in a fume cupboard",
        "Use gas test results to identify the products of reactions and electrolysis",
    ], "Mixing up the burning splint (hydrogen, pop) with the glowing splint (oxygen, relights) loses both marks. Writing 'glows brighter', 'limewater changes colour' or leaving out 'damp' for chlorine all miss the result mark."),

    ("combsci_aqa:5.9.1", &[
        "State the proportions of gases in today's atmosphere (about four-fifths nitrogen, one-fifth oxygen, small amounts of carbon dioxide, water vapour and noble gases) and use them in percentage calculations",
        "Describe the theory of the early atmosphere: volcanic gases, mainly carbon dioxide, little or no oxygen, water vapour condensing to form the oceans",
        "Explain how oxygen increased through photosynthesis by algae (from 2.7 billion years ago) and then plants",
        "Explain how carbon dioxide decreased by dissolving in the oceans, photosynthesis, and the formation of sedimentary rocks and fossil fuels",
        "Describe and explain the formation of limestone, coal, crude oil and natural gas",
        "Evaluate theories about the early atmosphere from given evidence, recognising why the evidence is limited",
    ], "Saying oxygen came from volcanoes, or that carbon dioxide simply 'disappeared', loses the marks: name photosynthesis by algae and plants, dissolving in the oceans, and locking up in rocks and fossil fuels. Mixing up which deposits came from plants, plankton and shells is the other common slip."),

    ("combsci_aqa:5.9.2", &[
        "Name water vapour, carbon dioxide and methane as greenhouse gases and describe the greenhouse effect in terms of short and long wavelength radiation",
        "Recall two human activities that increase carbon dioxide and two that increase methane",
        "Evaluate reports about climate change: peer review, bias, uncertainty in the data and the limits of models",
        "Describe four potential effects of global climate change and discuss their scale and risk",
        "Define the carbon footprint, describe actions that reduce it, and give reasons why those actions may be limited",
        "Calculate percentage changes in greenhouse gas levels and carbon footprints from given data",
    ], "Saying greenhouse gases 'trap heat' or 'reflect' radiation, or bringing in the ozone layer, loses the mechanism marks: incoming radiation is short wavelength, and greenhouse gases absorb and re-emit the long wavelength infrared that the Earth gives out."),

    ("combsci_aqa:5.9.3", &[
        "List the gases and particulates released when fuels burn: carbon dioxide, water vapour, carbon monoxide, sulfur dioxide, oxides of nitrogen, soot and unburned hydrocarbons",
        "Describe how carbon monoxide, soot, sulfur dioxide and oxides of nitrogen are produced, including the conditions for each",
        "Predict the products of combustion from the composition of a fuel and the oxygen supply",
        "Explain the problems each pollutant causes: toxicity, respiratory problems, acid rain, global dimming and health effects",
        "Write and balance equations for complete and incomplete combustion",
    ], "Oxides of nitrogen do not come from the fuel: nitrogen and oxygen from the air react at the high temperature of the engine. The other common slip is mixing up the effects: sulfur dioxide and oxides of nitrogen cause acid rain, and particulates cause global dimming."),

    ("combsci_aqa:5.10.1", &[
        "Distinguish finite from renewable resources, give examples of natural products replaced by synthetic ones, and define sustainable development",
        "Distinguish potable water from pure water and give reasons for each step in producing potable water from fresh water and from salty water",
        "Carry out and evaluate Required practical 8 (Trilogy 13): pH, mass of dissolved solids and distillation of water samples, including concentration in g/dm³",
        "Describe the stages of sewage treatment and compare the ease of getting potable water from ground, waste and salt water",
        "(Higher tier) Describe and evaluate phytomining and bioleaching, and how copper is then obtained by displacement with scrap iron or electrolysis",
    ], "Saying filtration kills microbes or removes salt loses the mark: filter beds remove solid particles, sterilising (chlorine, ozone or UV) kills microbes, and only desalination removes dissolved salts. In sewage treatment, sludge is digested anaerobically and effluent is treated aerobically, not the other way round."),

    ("combsci_aqa:5.10.2", &[
        "Name the four stages of a life cycle assessment and include transport at each stage",
        "Explain why LCA is not purely objective and how selective LCAs can be misused, for example in advertising",
        "Carry out a simple comparative LCA of plastic and paper shopping bags and reach a justified conclusion",
        "Interpret LCA data using ratios, percentages, per-use values and sensible significant figures",
        "Evaluate reducing, reusing and recycling materials such as glass and metals, including their limits",
    ], "Evaluating without quoting the data or reaching a conclusion caps the marks: compare the figures given, stage by stage, then give a justified judgement. Remember that pollutant effects need value judgements, so an LCA is never purely objective."),

    ("combsci_aqa:6.1.1", &[
        "Describe the changes in energy stores for an object thrown upwards, a collision, a braking vehicle and a kettle",
        "Recall and use Ek = ½mv² and Ep = mgh, and use Ee = ½ke² from the equation sheet",
        "Use ΔE = mcΔθ and explain what specific heat capacity means",
        "Describe Required practical 1 and explain why the measured specific heat capacity is usually too high",
        "Recall and use P = E ÷ t and P = W ÷ t, comparing devices that transfer the same energy at different rates",
        "Show on a common scale in joules how energy is redistributed when a system changes",
    ], "Only the speed is squared in kinetic energy and only the extension in elastic energy. Forgetting to convert grams, centimetres or minutes before substituting costs the answer mark."),

    ("combsci_aqa:6.1.2", &[
        "State that energy cannot be created or destroyed and describe energy transfers in a closed system with no net change",
        "Describe how energy is dissipated in every change and explain how lubrication and thermal insulation reduce unwanted transfers",
        "Explain how the thickness and thermal conductivity of walls affect the rate of cooling of a building",
        "Recall and use both efficiency equations, giving answers as a decimal or a percentage",
        "Describe ways to increase the efficiency of an intended energy transfer (Higher tier)",
        "Plan and evaluate Required practical 2 on thermal insulators (separate science only)",
    ], "Efficiency is useful divided by total, so it can never be above 1 or 100%. Turn a percentage into a decimal before rearranging to find an input."),

    ("combsci_aqa:6.1.3", &[
        "Describe the main energy resources and sort them into renewable and non-renewable",
        "Compare how resources are used for transport, electricity generation and heating",
        "Explain why some energy resources are more reliable than others",
        "Describe the environmental impact of each resource and evaluate choices with a justified conclusion",
        "Explain patterns and trends in energy use from graphs and data, calculating percentages and percentage change",
        "Explain why science can identify environmental problems but cannot always solve them, for political, social, ethical and economic reasons",
    ], "Comparisons must cover both resources on every point, and an evaluate question needs a justified conclusion. Renewable does not mean harmless: hydro floods habitats, and nuclear emits no carbon dioxide."),

    ("combsci_aqa:6.2.1", &[
        "Draw and interpret circuit diagrams using the standard symbols, with ammeters in series and voltmeters in parallel",
        "Recall and use Q = It, knowing that current is the same at every point in a single loop",
        "Recall and use V = IR, rearranging it confidently and converting mA and minutes",
        "Describe Required practical 3: how the resistance of a wire depends on its length, and resistors in series and parallel",
        "Describe Required practical 4 and sketch and explain the I–V graphs of a resistor, a filament lamp and a diode",
        "Explain how thermistors and LDRs change resistance and how they are used in thermostats and automatic lights",
    ], "For a curved I–V graph, work out resistance as V divided by I at the point, not from the gradient. When explaining the lamp's curve, say the resistance rises because the filament gets hotter."),

    ("combsci_aqa:6.2.2", &[
        "State the rules for current, potential difference and resistance in series and in parallel circuits",
        "Use R total = R1 + R2 and V = IR to calculate currents, pds and resistances in series circuits, including unknown resistors",
        "Work out branch currents and the supply current in parallel circuits, knowing the total resistance is less than the smallest resistor",
        "Explain why adding resistors in series increases total resistance while adding them in parallel decreases it",
        "Explain how series circuits are used for measuring and testing, such as current-limiting resistors and thermistor or LDR sensor circuits",
        "Build and check series and parallel circuits from a circuit diagram",
    ], "Series keeps the current the same and shares the pd; parallel keeps the pd the same and shares the current. Adding resistances only works in series: in parallel the total is less than the smallest resistor."),

    ("combsci_aqa:6.2.3", &[
        "Explain the difference between direct and alternating potential difference",
        "State the frequency (50 Hz) and potential difference (about 230 V) of the UK mains",
        "Identify the live, neutral and earth wires by colour and state the potential of each",
        "Explain why a live wire can be dangerous even when a switch in the circuit is open",
        "Explain the dangers of any connection between the live wire and earth",
    ], "An open switch stops the current but not the danger: the live wire is still at about 230 V, so touching it puts a large pd across your body. Students who say \"switched off means safe\" lose the mark."),

    ("combsci_aqa:6.2.4", &[
        "Recall and use P = VI and P = I²R, rearranging either to find any quantity",
        "Recall and use E = Pt and E = QV, converting kW and minutes or hours first",
        "Explain how a device's power relates to the pd across it, the current through it and the energy it transfers over time",
        "Describe the energy transfers in everyday appliances and link power ratings to changes in stored energy",
        "Explain why the National Grid uses step-up and step-down transformers and why this is efficient",
        "(Higher tier) Use Vp × Ip = Vs × Is for an ideal transformer",
    ], "The National Grid answer must go through the current: high pd means low current, and the heating loss depends on current squared. \"High voltage means less energy lost\" on its own earns almost nothing."),

    ("combsci_aqa:6.3.1", &[
        "Draw and describe particle diagrams for solids, liquids and gases",
        "Recall and use density = mass ÷ volume, converting between g/cm³ and kg/m³",
        "Explain differences in density between states using the spacing of particles",
        "Carry out Required practical 5: find the density of regular solids, irregular solids and liquids",
        "Explain why mass is conserved in a change of state and why it is a physical change",
    ], "Density differences come from how far apart the particles are, not from the particles getting lighter. And 1 cm³ is a millionth of a cubic metre, so 1 g/cm³ is 1000 kg/m³."),

    ("combsci_aqa:6.3.2", &[
        "Define internal energy as the total kinetic and potential energy of the particles in a system",
        "Explain that heating either raises the temperature or changes the state",
        "Use ΔE = mcΔθ and E = mL from the equation sheet, splitting multi-stage problems into steps",
        "Distinguish specific heat capacity from specific latent heat, including their units",
        "Interpret heating and cooling graphs, using plateau times to find latent heat",
        "Describe how to measure the specific latent heat of fusion of ice using a control funnel",
    ], "On the flat part of a heating graph the temperature is constant but the internal energy is still rising: the energy goes into the particles' potential energy. Saying no energy is being transferred loses the mark."),

    ("combsci_aqa:6.3.3", &[
        "Explain how the random motion of gas molecules causes pressure on the walls of a container",
        "Explain why heating a gas at constant volume increases its pressure, linking speed, collision rate and force",
        "Separate science only: explain why increasing the volume of a gas at constant temperature decreases its pressure",
        "Separate science only: use pV = constant from the equation sheet to calculate a new pressure or volume",
        "Separate science only (Higher tier): explain how doing work on a gas, as in a bicycle pump, raises its temperature",
    ], "When a gas is squashed at constant temperature the molecules hit the walls more often, not harder: their speed has not changed. Saying the collisions are harder, or that the molecules hit each other more, loses the mark."),

    ("combsci_aqa:6.4.1", &[
        "Describe the structure of an atom, including the size of the atom and nucleus in standard form and where the mass is",
        "Work out the numbers of protons, neutrons and electrons from a nuclear symbol, for atoms, isotopes and positive ions",
        "Explain how electrons move between energy levels when an atom absorbs or emits electromagnetic radiation",
        "Describe the plum pudding and nuclear models and the differences between them",
        "Explain how the alpha particle scattering results led to the nuclear model, then the roles of Bohr, protons and Chadwick's neutrons",
    ], "Isotopes have the same number of protons but different numbers of neutrons. Writing that they differ in protons or electrons, or giving scattering observations without the conclusion each one supports, loses marks."),

    ("combsci_aqa:6.4.2", &[
        "Describe alpha, beta, gamma and neutron radiation and compare their penetration, range in air and ionising power",
        "Choose and justify the best type of radiation for a given use",
        "Write balanced nuclear equations for alpha and beta decay, and state the effect of gamma emission",
        "Explain half-life and its link to random decay, and find it from a graph or data",
        "Higher tier: calculate the net decline, as a ratio, after a given number of half-lives",
        "Compare the hazards of contamination and irradiation and describe precautions against each",
    ], "An irradiated object does not become radioactive; only contamination puts radioactive atoms onto or into something. Mixing these up, or saying beta particles come from the electron shells, loses easy marks."),

    ("combsci_aqa:6.5.1", &[
        "Classify quantities as scalars or vectors and represent a vector as an arrow whose length shows its magnitude",
        "Sort forces into contact and non-contact forces and describe interaction pairs between two objects",
        "Recall and use W = mg, explaining the difference between mass and weight and that weight is proportional to mass",
        "Calculate the resultant of forces acting along a straight line and state its direction",
        "Draw free body diagrams and use them to describe balanced and unbalanced forces (Higher tier)",
        "Use scale drawings to resolve a force into two components and to find a resultant or show equilibrium (Higher tier)",
    ], "Mass is in kilograms and never changes; weight is a force in newtons that depends on g. A resultant force needs a direction as well as a size."),

    ("combsci_aqa:6.5.2", &[
        "Recall and use W = Fs, using the distance moved along the line of action of the force",
        "Explain why no work is done when there is no displacement in the direction of the force",
        "Convert between newton-metres and joules, and between J, kJ and MJ",
        "Describe the energy transfer between stores when a force does work",
        "Explain why work done against friction raises the temperature of an object",
        "Combine W = Fs with kinetic or gravitational potential energy in two-step problems",
    ], "Use the distance moved in the direction of the force: the vertical height when lifting. And when given a mass, find the weight with W = mg before using W = Fs."),

    ("combsci_aqa:6.5.3", &[
        "Give examples of stretching, bending and compressing, and explain why a stationary object needs more than one force to change shape",
        "Describe the difference between elastic and inelastic deformation",
        "Recall and use F = ke, working out extension as new length minus original length in metres",
        "Distinguish linear from non-linear force-extension graphs and find the spring constant from the gradient",
        "Apply Ee = 0.5ke² to calculate the energy stored in a spring and the energy it transfers",
        "Carry out and evaluate Required practical 6 (Trilogy 18): force and extension for a spring",
    ], "Extension is the increase in length, not the new length, and it must be in metres before you use F = ke or Ee = 0.5ke²."),

    ("combsci_aqa:6.5.4a", &[
        "Explain the difference between scalars and vectors for distance, displacement, speed and velocity, giving a displacement as a magnitude and a direction",
        "Recall typical speeds for walking, running, cycling, transport and sound, and use s = vt and average speed with correct unit conversions",
        "Recall and use a = Δv/t, and estimate everyday accelerations using the ≈ symbol",
        "Find speed from the gradient of a distance–time graph (by a tangent on a curve at Higher tier) and acceleration from the gradient of a velocity–time graph",
        "Find distance from the area under a velocity–time graph, counting squares where needed (Higher tier)",
        "Apply v² − u² = 2as from the equation sheet, and explain why circular motion at constant speed is accelerated motion (Higher tier)",
    ], "Read the y-axis before interpreting a graph: a horizontal line means stopped on a distance–time graph but constant velocity on a velocity–time graph. Average speed is total distance ÷ total time, never the mean of two speeds."),

    ("combsci_aqa:6.5.4b", &[
        "State and apply Newton's First Law to objects at rest, at constant velocity, and changing speed or direction",
        "Recall and use F = ma with the resultant force, and explain inertia and inertial mass as force ÷ acceleration (Higher tier)",
        "Estimate the forces and accelerations involved in everyday road transport, using the ≈ symbol",
        "State Newton's Third Law and identify equal and opposite force pairs acting on different objects in equilibrium situations",
        "Explain how a falling object reaches terminal velocity, and draw and interpret its velocity–time graph (separate science only)",
        "Describe Required practical 7 (Trilogy 19) to find how acceleration depends on force and on mass, keeping the total mass constant when varying force",
    ], "In F = ma, F is the resultant force — subtract the opposing forces first. Weight and the normal contact force on the same object are balanced forces, not a Newton's Third Law pair."),

    ("combsci_aqa:6.5.4c", &[
        "State that stopping distance is thinking distance plus braking distance, and calculate each part using s = vt and v² − u² = 2as",
        "Recall that reaction times are typically 0.2–0.9 s and explain how tiredness, drugs, alcohol and distractions increase thinking distance",
        "Describe and evaluate methods for measuring reaction time, such as the ruler-drop test",
        "Explain how wet or icy roads and worn brakes or tyres increase braking distance, and the implications for safety",
        "Explain braking in terms of work done by friction reducing kinetic energy, and the dangers of large decelerations",
        "Estimate braking forces for road vehicles (Higher tier), and estimate and read stopping distances over a range of speeds (separate science only)",
    ], "Driver factors (tiredness, alcohol, drugs, distractions) change the thinking distance; road, weather, brakes and tyres change the braking distance. Saying reaction time affects braking distance loses the mark."),

    ("combsci_aqa:6.5.5", &[
        "Recall and use p = mv, treating momentum as a vector with a sign for direction (Higher tier)",
        "State that total momentum before an event equals total momentum after it in a closed system",
        "Describe and explain collisions and explosions, such as recoil, using conservation of momentum",
        "Calculate velocities after collisions and explosions, including objects that stick together or move in opposite directions (separate science only)",
        "Apply F = mΔv/Δt from the equation sheet and explain how air bags, seat belts, crash mats, helmets and cushioned surfaces reduce force (separate science only)",
    ], "Momentum is a vector, so a velocity in the opposite direction needs a minus sign. Safety features do not reduce the change in momentum — they increase the time, so the rate of change of momentum and the force are smaller."),

    ("combsci_aqa:6.6.1", &[
        "Describe the difference between transverse and longitudinal waves, with examples, and the evidence that the wave and not the medium travels",
        "Define amplitude, wavelength, frequency, period and wave speed, and identify amplitude and wavelength on diagrams",
        "Recall and use v = fλ, and apply T = 1/f from the equation sheet",
        "Describe methods to measure the speed of sound in air and of ripples, including Required practical 8 (Trilogy 20)",
        "Draw ray diagrams for reflection and describe Required practical 9 on reflection and refraction of light (separate science only)",
        "Explain hearing limits (20 Hz to 20 kHz), ultrasound, echo sounding and seismic evidence for the Earth's core (separate science only, Higher tier)",
    ], "Compare the direction of oscillation with the direction of energy transfer when describing transverse and longitudinal waves. Amplitude is measured from the rest line to a crest, not crest to trough, and echo distances must be halved."),

    ("combsci_aqa:6.6.2", &[
        "List the EM spectrum in order of wavelength and frequency, and use v = fλ with 3.0 × 10⁸ m/s for EM waves",
        "Draw refraction ray diagrams and, on Higher tier, explain refraction with wave fronts: one side slows first, wavelength shortens, frequency stays the same",
        "Describe Required practical 10 (Trilogy RP21) and state that matt black surfaces are the best emitters and absorbers of infrared",
        "Give a use for each EM wave and, on Higher tier, explain why its properties suit that use",
        "Describe the dangers of UV, X-rays and gamma rays and draw conclusions from radiation dose data in sieverts",
        "Separate science only: draw lens ray diagrams, calculate magnification, and explain colour, filters and specular and diffuse reflection",
    ], "Explaining a use without the property behind it loses the mark: say why the wave suits the job, such as bone absorbing X-rays while soft tissue transmits them. In refraction, the frequency never changes — only the speed and wavelength."),

    ("combsci_aqa:6.7.1", &[
        "Describe how like poles repel and unlike poles attract, and that magnetic forces act without contact",
        "Compare permanent and induced magnets, and explain why induced magnetism always attracts",
        "Name the magnetic materials (iron, steel, cobalt, nickel) and define the direction of a magnetic field",
        "Draw the field of a bar magnet with arrows from north to south and lines closest at the poles",
        "Describe how to plot a field with a compass, and explain how compass behaviour shows the Earth's core is magnetic",
    ], "Field lines need arrows pointing from north to south, and they must be closest together at the poles. Remember that attraction does not prove something is a magnet — only repulsion does."),

    ("combsci_aqa:6.7.2", &[
        "Describe how to show the magnetic effect of a current, and draw the fields around a straight wire and a solenoid with their directions",
        "Explain why a solenoid strengthens the field and how an iron core makes an electromagnet",
        "Use Fleming's left-hand rule and recall the factors that affect the size of the motor-effect force (Higher tier)",
        "Calculate force, current, flux density or length using F = BIl, converting cm to m and mT to T (Higher tier)",
        "Explain how the forces on a coil and a split-ring commutator make a d.c. motor rotate (Higher tier)",
        "Separate science only: explain electromagnetic devices from diagrams, and explain how a moving-coil loudspeaker works (Higher tier)",
    ], "In a motor answer, say that the two sides of the coil carry current in opposite directions, so the forces are opposite and give a turning effect — and that the commutator reverses the current every half turn. Use the left hand for the motor effect and convert lengths to metres before using F = BIl."),
    // ---------- Economics (OCR GCSE J205) ----------
    ("econ_ocr:1.1", &[
        "Explain the roles of consumers, producers and the government in the economy",
        "Explain how the three economic groups depend on one another",
        "Explain land, labour, capital and enterprise, with examples from a named business",
        "Explain how the factors of production are combined to produce goods and services",
    ], "Capital means man-made resources such as machinery and buildings, not money - and enterprise is the risk-taking and organising, not the entrepreneur's name."),

    ("econ_ocr:1.2", &[
        "Explain scarce resources and unlimited wants, and why they force choices",
        "Explain the economic problem: what to produce, how to produce it and for whom",
        "Define opportunity cost and apply it to a choice made by a consumer, producer or government",
        "Evaluate the costs and benefits of an economic choice, including its effect on economic, social and environmental sustainability",
    ], "Opportunity cost is the single next best alternative given up - not the money spent and not every other possible use."),

    ("econ_ocr:2.1a", &[
        "Explain what a market is and why it need not be a physical place",
        "Explain the features of the primary, secondary and tertiary sectors, and the difference between producing goods and services",
        "Explain the difference between factor and product markets",
        "Explain how factor and product markets depend on each other",
    ], "The factor market is not the primary sector and the product market is not the secondary or tertiary sector - the 2025 examiners named this confusion."),

    ("econ_ocr:2.1b", &[
        "Explain specialisation and the division of labour",
        "Explain why specialisation leads to exchange, and the role of money in it",
        "Evaluate the costs and benefits of specialisation for producers, workers, regions and countries",
    ], "Answer for the group the question names: a cost of specialisation 'for a region' is dependence on one industry and structural unemployment, not a worker's boredom."),

    ("econ_ocr:2.2a", &[
        "Explain demand as the quantity consumers are willing and able to buy at each price",
        "Draw individual and market demand curves from data",
        "Draw and explain the difference between a movement along the demand curve and a shift of it",
        "Analyse the causes and consequences of shifts in demand for consumers and producers",
    ], "Only a change in the good's own price moves along the demand curve; every other factor shifts it, and you must say which way."),

    ("econ_ocr:2.2b", &[
        "Explain price elasticity of demand and calculate it from percentage changes",
        "Distinguish price elastic, price inelastic and unitary demand",
        "Draw demand curves of different elasticity",
        "Explain the factors that make demand more or less price elastic",
        "Evaluate the importance of PED for consumers and producers, including its effect on total revenue",
    ], "Elastic and inelastic are not interchangeable - the 2024 examiners found many candidates muddled them and so got the consequences of a supply change the wrong way round."),

    ("econ_ocr:2.3a", &[
        "Explain supply as the quantity producers are willing and able to sell at each price",
        "Draw individual and market supply curves from data",
        "Draw and explain the difference between a movement along the supply curve and a shift of it",
        "Analyse the causes and consequences of shifts in supply for consumers and producers",
    ], "When plotting a supply curve from data, plot each point and join them - a line of best fit lost a mark in 2025."),

    ("econ_ocr:2.3b", &[
        "Explain price elasticity of supply and calculate it from percentage changes",
        "Distinguish price elastic from price inelastic supply",
        "Draw supply curves of different elasticity",
        "Explain the factors that affect PES: time, spare capacity, stocks and the availability of inputs",
        "Evaluate the importance of PES for consumers and producers",
    ], "Inelastic supply means the percentage change in quantity supplied is SMALLER than the percentage change in price - many 2025 answers missed 'percentage' or wrote about demand."),

    ("econ_ocr:2.4", &[
        "Explain price as a reflection of worth and its role in distributing resources efficiently",
        "Explain equilibrium price and quantity, and how excess demand and supply are removed",
        "Draw and analyse the interaction of demand and supply",
        "Analyse how changes in demand and supply affect equilibrium price and quantity",
        "Explain the role of markets in setting prices and allocating resources",
    ], "'The role of the market' needs the mechanism - a surplus pushes price down and a shortage pushes it up until demand equals supply; most 2025 answers only defined equilibrium."),

    ("econ_ocr:2.5", &[
        "Explain why producers compete and how they compete",
        "Analyse how competition affects price",
        "Evaluate the economic impact of competition on producers and consumers",
        "Explain monopoly and oligopoly and how they differ from competitive markets",
    ], "'A few large firms' against 'many small firms' is the difference - 'large firms' against 'smaller firms' scored only one mark in 2024."),

    ("econ_ocr:2.6a", &[
        "Explain the role of producers, including individuals, firms and the government",
        "Distinguish production from productivity and calculate labour productivity",
        "Evaluate the importance of production and productivity for the economy",
    ], "Productivity is output per worker (or per input), not total output - more workers can raise production while productivity falls."),

    ("econ_ocr:2.6b", &[
        "Calculate total cost, average cost, total revenue, average revenue, profit and loss",
        "Evaluate how costs and revenues affect profit and a producer's supply",
        "Explain internal and external economies of scale",
        "Evaluate the importance of costs, revenue, profit and loss for producers",
    ], "Revenue is not profit: profit is total revenue minus total cost - the 2024 examiners found the two used interchangeably."),

    ("econ_ocr:2.7", &[
        "Explain the role and operation of the labour market, including the interaction of workers and employers",
        "Analyse how wages are determined by the demand for and supply of labour",
        "Explain the factors that affect the demand for and supply of labour",
        "Explain and calculate gross and net pay, including income tax, National Insurance and pension deductions",
    ], "Net pay is gross pay minus ALL the deductions - income tax, National Insurance and pension - and a monthly figure means divide the annual one by 12."),

    ("econ_ocr:2.8a", &[
        "Explain money's role as a medium of exchange",
        "Explain the roles of banks, building societies and insurance companies",
        "Evaluate the importance of the financial sector for consumers, producers and government",
    ], "Banks lend, hold deposits and offer overdrafts and payment services - they do not give businesses subsidies, a 2025 error."),

    ("econ_ocr:2.8b", &[
        "Analyse how changes in interest rates affect saving, borrowing and investment",
        "Calculate interest on savings and loans, and the effect of a change in the rate",
        "Explain why a fall in interest rates can raise spending and investment, and its limits",
    ], "Work out the interest at each rate, then the difference - and give the answer in pounds with the £ sign."),

    ("econ_ocr:3.1", &[
        "Explain economic growth and how it is measured using GDP and GDP per capita",
        "Calculate growth rates and GDP per capita, and analyse GDP data",
        "Analyse the determinants of growth: investment, technology, workforce, education and training, natural resources and government policy",
        "Evaluate the costs and benefits of growth, including economic, social and environmental sustainability",
    ], "A fall in the growth rate is not a fall in GDP - output still rises if the rate stays above zero."),

    ("econ_ocr:3.2", &[
        "Explain employment and unemployment and how the Claimant Count measures unemployment",
        "Calculate the unemployment rate and analyse unemployment data",
        "Explain cyclical, frictional, seasonal and structural unemployment",
        "Evaluate the causes and consequences of unemployment for individuals, regions and the government",
    ], "'Explain the trend' needs a reason for the change, not just a description - and more unemployment means a surplus of workers, not fewer workers for firms."),

    ("econ_ocr:3.3", &[
        "Explain the distribution of income, the types of income and the difference between income and wealth",
        "Calculate shares of income and wealth from data",
        "Evaluate the causes of unequal income and wealth and the consequences for an economy",
    ], "Income is a flow and wealth is a stock - read a quintile chart carefully, because adding the wrong fifths cost marks in 2024."),

    ("econ_ocr:3.4", &[
        "Explain price stability and inflation, and the difference between real and nominal values",
        "Explain how the Consumer Price Index measures inflation",
        "Calculate the effect of inflation on prices and analyse inflation data",
        "Evaluate the causes of inflation and its consequences for consumers, producers, savers and the government",
    ], "Price stability is a low and steady rate of inflation, not zero inflation and not just 'constant' inflation - a constant 15% is not stable prices."),

    ("econ_ocr:3.5a", &[
        "Explain the purposes of government spending and the sources of government revenue",
        "Distinguish direct from indirect taxes, with examples",
        "Explain a balanced budget, a budget surplus and a budget deficit",
        "Calculate a budget balance and the effect of a tax on prices",
    ], "The government budget is not the balance of payments - a 'deficit' must say which one, and VAT is indirect while income tax is direct."),

    ("econ_ocr:3.5b", &[
        "Explain fiscal policy and how it can be used to achieve economic objectives",
        "Analyse how taxes and government spending affect markets and the whole economy",
        "Evaluate the costs, including opportunity cost, and benefits of fiscal policy",
        "Evaluate the consequences of redistributing income and wealth, including progressive taxes",
    ], "A progressive tax takes a higher PERCENTAGE of income as income rises - 'the rich pay more' is not enough, and the revenue must be spent to redistribute."),

    ("econ_ocr:3.6", &[
        "Explain monetary policy and how the Bank of England uses interest rates to reach objectives",
        "Analyse how monetary policy affects growth, employment and price stability",
        "Evaluate the effects of monetary policy on consumer spending, borrowing, saving and investment",
    ], "Monetary policy is interest rates set by the Bank of England; tax and government spending are fiscal policy - and a 6-mark analyse needs the impact at the end of the chain."),

    ("econ_ocr:3.7", &[
        "Explain supply side policy and how it raises the productive capacity of the economy",
        "Explain examples such as education and training, infrastructure, tax incentives and deregulation",
        "Evaluate the costs, including opportunity cost, and benefits of supply side policies",
    ], "A supply side policy works by making the economy able to produce more - higher profits for builders or more imports is not why a railway is supply side."),

    ("econ_ocr:3.8", &[
        "Explain positive and negative externalities with examples",
        "Explain taxation, subsidies, state provision, legislation and regulation, and information provision as remedies",
        "Evaluate the use and impact of each policy, including its costs and opportunity cost",
    ], "An indirect tax shifts the SUPPLY curve left, not demand - and when demand is price inelastic the fall in quantity is small."),

    ("econ_ocr:4.1", &[
        "Explain why countries import and export goods and services",
        "Explain the benefits of trade for consumers and producers",
        "Explain free trade agreements, including the European Union",
    ], "A free trade agreement removes or cuts tariffs and quotas between members - it is not a single currency and it is not monetary policy."),

    ("econ_ocr:4.2", &[
        "Explain the balance of payments on current account and its parts",
        "Explain a balanced current account, a surplus and a deficit, and calculate them",
        "Analyse data on exports and imports",
        "Evaluate the causes of a deficit or surplus and its importance to the UK economy",
    ], "Describe the BALANCE (deficit or surplus), not the value of trade - and never answer a current-account question with tax rises to pay off the government's debt."),

    ("econ_ocr:4.3", &[
        "Draw and analyse how the exchange rate is set by the demand for and supply of a currency",
        "Calculate currency conversions",
        "Analyse exchange rate data",
        "Evaluate the effects of a change in the exchange rate on consumers and producers",
    ], "To convert pounds to a foreign currency multiply by the rate; to convert back to pounds divide - and give the currency sign with the answer."),

    ("econ_ocr:4.4a", &[
        "Explain globalisation and its driving factors: technology, transport, communications, multinationals and trade liberalisation",
        "Explain how development is measured: GDP per capita, life expectancy, access to health care, technology and education",
        "Explain why GDP per capita alone is an incomplete measure of development",
    ], "GDP per capita is an average - it hides inequality and says nothing directly about health or education."),

    ("econ_ocr:4.4b", &[
        "Evaluate the costs and benefits of globalisation for producers, workers and consumers in developed countries",
        "Evaluate the costs and benefits of globalisation for producers, workers and consumers in less developed countries",
        "Evaluate the impact of globalisation on economic, social and environmental sustainability",
    ], "Keep to the group named: a 2025 MCQ wanted the cost to consumers in developed countries (dominant global brands), not the effect of immigration."),
    // ---------- French (Pearson Edexcel GCSE 1FR1, Higher) ----------
    ("fre_edx:T1", &[
        "Describe yourself, your family and friends, including step-family and personality, with accurate agreements",
        "Explain how you get on with people using s'entendre avec, se disputer and faire confiance à",
        "Give and justify opinions on friendship, relationships and equality, including discrimination and harassment",
        "Answer Paper 3 questions on family and equality texts, including inferring an unfamiliar word from context",
        "Write an 80-90-word message to a friend covering four bullets in three time frames",
    ], "Copying a whole French phrase as the answer to a reading question. Edexcel wants a short answer in English, and isolated French words score nothing."),
    ("fre_edx:T2", &[
        "Talk and write about healthy and unhealthy habits, diet, sleep and mental wellbeing",
        "Use il faut, il vaut mieux and il est important de + infinitive to give advice",
        "Describe illness and injury with avoir mal à and se blesser, and handle a doctor's surgery role play",
        "Weigh up the pros and cons of a lifestyle choice for a 130-150-word formal task",
    ], "Losing the past time frame: saying what you do now but never what you used to do, which caps the language mark in both writing questions."),
    ("fre_edx:T3", &[
        "Describe your town or village and compare it with the countryside using comparatives",
        "Buy things, return them and complain in a shop or market role play",
        "Explain environmental problems and solutions with the passive, il faut and si + present + future",
        "Use transport vocabulary to buy tickets and ask two questions at a train station",
    ], "Giving only advantages when the bullet asks for the pros and cons: both sides are needed for the bullet to count as fully answered."),
    ("fre_edx:T4", &[
        "Describe how you use your phone, apps, social media and games, and their benefits and dangers",
        "Talk about music, concerts, TV, series and films, including one you saw recently",
        "Use object pronouns, y and en in sentences about technology and media",
        "Buy cinema or concert tickets and ask two questions in a role play",
    ], "Writing je les ai regardé: after a preceding direct object the participle agrees, la série que j'ai regardée."),
    ("fre_edx:T5", &[
        "Give opinions on school subjects, rules, uniform and pressure, with reasons",
        "Describe part-time jobs, work experience and apprenticeships",
        "Explain future plans for study and work using the future, the conditional and si clauses",
        "Write a formal 130-150-word article covering four bullets, including a past event and future plans",
    ], "Using the wrong future form: j'allerai and je serais for I will be. The list's future of aller is irai and of être is serai."),
    ("fre_edx:T6", &[
        "Describe past, usual and ideal holidays in three time frames",
        "Use en, au, aux and à correctly with countries and towns",
        "Book accommodation and sort out problems in hotel, campsite and tourist-office role plays",
        "Read brochures and adverts for detail, including opening times, prices and conditions",
    ], "Mixing the perfect and the imperfect in a holiday story: il faisait beau and c'était génial for description, nous sommes allés for events."),
    ("fre_edx:G1a", &[
        "Form feminine and plural nouns by the listed patterns",
        "Use definite, indefinite and partitive articles, including where French differs from English",
        "Change the article to de after negatives and expressions of quantity",
        "Use an infinitive as a noun where English uses -ing",
        "Use dans with an article and en without one, and the negative determiner aucun at Higher",
    ], "Writing je n'ai pas des frères: after a negative the article becomes de, je n'ai pas de frères."),
    ("fre_edx:G1b", &[
        "Make ce, mon, quel and tout agree with the noun that follows",
        "Place direct, indirect and reflexive pronouns before the verb, and in the perfect before the auxiliary",
        "Use emphatic pronouns after prepositions, as in chez moi and avec elle",
        "Join sentences with the relative pronoun qui",
    ], "Treating son and sa as his and her: they agree with the thing owned, so sa mère can mean his mother."),
    ("fre_edx:G1c", &[
        "Replace places with y and quantities or de + noun with en, including il y en a",
        "Use les and leur before the verb, never next to another object pronoun",
        "Use personne ne and rien ne as the subject of a verb",
        "Choose between qui, que and où in relative clauses",
        "Recognise dont, le mien and moi-même in reading",
    ], "Adding -s to the pronoun leur: je leur parle, because only the possessive leurs takes an -s."),
    ("fre_edx:G2a", &[
        "Use ne...pas, jamais, rien and personne, and at Higher ne...plus, ne...ni...ni, ne...pas encore and ne...que",
        "Place negatives correctly around the auxiliary in the perfect and before an infinitive",
        "Ask questions by intonation, with est-ce que and by inversion, with and without a question word",
        "Ask the two questions a Higher role play requires",
    ], "Reading ne...que as a negative: je n'ai que dix euros means I only have ten euros."),
    ("fre_edx:G2b", &[
        "Use il y a, il faut, il fait and il est, and at Higher il est + adjective + de, il manque, il vaut mieux and il vaut la peine de",
        "Use reflexive verbs in every person and tense, with the pronoun matching the subject",
        "Form the present passive with être, an agreeing participle and par",
        "Use être en train de and venir de for actions in progress and just completed",
    ], "Writing je veux s'amuser: the reflexive pronoun must match the subject, je veux m'amuser."),
    ("fre_edx:G3a", &[
        "Conjugate -er verbs and the listed -ir and -re patterns in every person of the present",
        "Use aller, avoir, être, faire, mettre and the modals devoir, pouvoir, savoir and vouloir + infinitive",
        "Use the singular-only irregulars such as boire, croire, voir and recevoir, and connaître and écrire in full at Higher",
        "Use the present with depuis for something that has been going on for a period of time",
    ], "Putting depuis with the perfect: j'habite ici depuis cinq ans, not j'ai habité ici depuis cinq ans."),
    ("fre_edx:G3b", &[
        "Form the perfect with avoir or être and the right past participle",
        "Make the participle agree after être and after a preceding direct object",
        "Use reflexive verbs and the modals j'ai dû, j'ai pu and j'ai voulu in the perfect",
        "Recognise the Higher participles découvert, plaint, convaincu and tu",
    ], "Using avoir with a movement verb: elle est allée, never elle a allé."),
    ("fre_edx:G3c", &[
        "Form the imperfect from the nous stem, with être as the exception",
        "Use the imperfect for habits, description and actions in progress",
        "Choose between the imperfect and the perfect in a past narrative",
        "Use c'était, il y avait and il faisait to describe the past",
    ], "Using the perfect for what you used to do: quand j'étais petit, je jouais, not j'ai joué."),
    ("fre_edx:G3d", &[
        "Talk about the future with aller + infinitive in every person",
        "Form the future of -er verbs and use irai, aurai, serai and ferai",
        "Form the conditional of -er verbs and use irais, aurais, serais, ferais and voudrais",
        "Use si + present + future to talk about conditions and plans",
    ], "Confusing will and would: je jouerai means I will play, je jouerais means I would play, and the dictation can test the difference."),
    ("fre_edx:G3e", &[
        "Give instructions with the tu and vous imperative, dropping the -s of -er verbs in the tu form",
        "Use the nous imperative for let's and the imperative of être at Higher",
        "Form the present participle and use en + -ant for while or by doing",
    ], "Keeping the -s in the tu imperative of -er verbs: mange and va, not manges and vas."),
    ("fre_edx:G4-5", &[
        "Make adjectives agree using the listed feminine and plural patterns",
        "Place adjectives after the noun, except the listed group that goes before it",
        "Compare with plus, moins and aussi...que, and use le meilleur, le mieux and le pire at Higher",
        "Place adverbs of time, manner, frequency and place, including in the perfect",
    ], "Forgetting agreement after être: mes sœurs sont sportives, not sportif."),
    ("fre_edx:G6", &[
        "Use à and de after the verbs and adjectives that need them before a noun or an infinitive",
        "Use en, au, aux and à with places, and the contractions au, aux, du and des",
        "Show possession with de and purpose with pour and sans + infinitive",
        "Use avant de + infinitive and après avoir + past participle at Higher",
    ], "Writing après manger: after doing something is après avoir mangé."),
    ("fre_edx:G7", &[
        "Work out words formed with in- and im- meaning un- or not",
        "Recognise ordinals in -ième, adjectives in -able, nouns in -ion and -ation and adverbs in -ment and -emment",
        "Recognise agent nouns in -eur and -ateur at Higher",
        "Infer the two off-list words in Paper 3 from context, cognates and word families",
    ], "Trusting false friends: actuellement means currently, and sensible means sensitive."),
    ("fre_edx:G8", &[
        "Pronounce and spell every sound-symbol correspondence on the Edexcel list",
        "Mark silent letters, liaisons and nasal vowels in a read-aloud text",
        "Use grammar to choose between homophones such as a and à, et and est, ces, ses, c'est and s'est",
        "Spell words you have never seen from their sound in the dictation",
    ], "Leaving out accents and silent letters in the dictation: frere for frère does not count, because the SSC is wrong."),
    ("fre_edx:P1", &[
        "Read a 50-55-word text aloud with accurate sound-symbol correspondences",
        "Complete a transactional role play, asking two questions and answering one prompt in the future",
        "Describe the people, location and activity in a photo with developed detail",
        "Sustain a five-minute conversation with extended, justified answers in three time frames",
        "Use the 15 minutes' preparation and one A4 sheet of notes to best effect",
    ], "Making a statement when the prompt asks a question: a role-play question must be a question to score."),
    ("fre_edx:P2", &[
        "Use the five minutes' reading time to predict vocabulary and underline question words",
        "Avoid distractors created by negatives, time frames and words like mais and par contre",
        "Answer multiple-choice, multiple-response and short-answer questions in precise English",
        "Transcribe the six dictation sentences accurately, including off-list words, using the SSCs",
    ], "Writing down the first word that matches the question, when a negative or a change of time frame later in the extract makes it wrong."),
    ("fre_edx:P3", &[
        "Answer gap-fill, multiple-choice, multiple-response and table questions accurately in English",
        "Infer the meaning of two unfamiliar words from context",
        "Separate past, present and future events in a text",
        "Translate a 60-word passage into natural English, rendering every tense and every word",
    ], "Translating depuis + present as since + present: il habite ici depuis deux ans means he has been living here for two years."),
    ("fre_edx:P4", &[
        "Address all four bullets of the 80-90-word and 130-150-word tasks with developed ideas",
        "Use past, present and future time frames successfully in both open-response questions",
        "Give pros and cons, opinions and reasons, with complex language as Pearson defines it",
        "Translate a short English paragraph into French accurately",
        "Plan and check within 1 hour 20 minutes",
    ], "Skipping a bullet point: a missing bullet caps the response-to-task mark however good the French is."),
    // ---------- German (Pearson Edexcel GCSE 1GN1) ----------
    ("ger_edx:T1", &[
        "Describe family members and friends: appearance, personality and how you get on, using sich verstehen mit",
        "Discuss equality and inclusion: different family set-ups, gender, disability and treating people fairly",
        "Give and justify opinions with weil, denn and dass, including what makes a good friend",
        "Narrate something you did with friends or family and say what you will do together next",
    ], "Sich verstehen mit takes the dative: Ich verstehe mich gut mit meiner Schwester, never mit meine Schwester."),

    ("ger_edx:T2", &[
        "Describe your diet, exercise and sleep with frequency words such as regelmäßig, ab und zu and nie",
        "Discuss physical and mental wellbeing, stress and pressure, using Higher words such as Druck and die mentale Gesundheit",
        "Contrast what you used to do (früher + perfect) with what you do now and what you will do",
        "Give advice with man sollte and set conditions with wenn ich mehr Zeit hätte, würde ich ...",
    ], "Kein negates a noun and nicht a verb or adjective: Ich esse kein Fleisch, but Ich rauche nicht."),

    ("ger_edx:T3", &[
        "Describe places in your town and say where things are, using prepositions with the dative",
        "Talk about shopping, transport and getting around, including buying tickets",
        "Discuss environmental problems and solutions: waste, energy, pollution and sustainable transport",
        "Weigh up the pros and cons of living in a town or in the countryside",
    ], "In + dative says where something is, in + accusative where you are going: Ich bin in der Stadt, but Ich fahre in die Stadt."),

    ("ger_edx:T4", &[
        "Describe how you use social media, apps and games, with frequency and opinions",
        "Discuss the risks online: cyberbullying, cybercrime and screen time",
        "Talk about music, TV and film preferences with gern, lieber and am liebsten",
        "Review a film or programme you saw recently, in the past tense",
    ], "Gefallen works backwards: Der Film hat mir gut gefallen means I liked the film; Ich habe den Film gefallen is wrong."),

    ("ger_edx:T5", &[
        "Describe your school, routine, teachers and rules with dürfen and müssen",
        "Compare school in German-speaking countries, e.g. the Oberstufe and sitzen bleiben",
        "Talk about part-time work, skills and future opportunities, including working or travelling abroad",
        "Explain future plans with werden, möchte, ich hoffe ... zu and um ... zu",
    ], "After um ... zu the infinitive goes last with zu in front of it: um Geld zu verdienen, not um zu verdienen Geld."),

    ("ger_edx:T6", &[
        "Describe tourist attractions in German-speaking countries, such as Wien, Köln, München and the Donau",
        "Book accommodation, buy tickets and complain about a problem in a role-play setting",
        "Narrate a past trip in the perfect and simple past, and plan the next one with werden",
        "Compare holidays and accommodation with comparatives and superlatives",
    ], "Fahren and fliegen take sein in the perfect: Wir sind nach Österreich gefahren, never wir haben ... gefahren."),

    ("ger_edx:G1a", &[
        "Build compound nouns, taking the gender of the last word: die Haustür, das Schulbuch",
        "Form plurals by Pearson's patterns, add -n in the dative plural, and make feminine person nouns with -in",
        "Use nouns made from verbs and adjectives: das Schwimmen, das Englisch",
        "Recognise and use weak masculine nouns (den Jungen) and adjectival nouns: die Reichen, das Gute, etwas Gutes",
    ], "The dative plural adds -n: mit meinen Freunden, mit den Kindern; mit meinen Freunde loses accuracy."),

    ("ger_edx:G1b", &[
        "Choose der, die, das and ein, eine in the nominative, accusative and dative from the noun's job in the sentence",
        "Use kein, dieser, jeder, letzter, nächster, welcher and the possessives mein to Ihr with the right ending",
        "Tell viel and wenig (uncountable) from viele and wenige (plural), and use alle and einige",
        "Recognise the genitive for possession and after trotz and wegen in reading and listening",
    ], "Time phrases go in the accusative: letzten Sommer, nächste Woche, jeden Tag; letzter Sommer is wrong."),

    ("ger_edx:G1c", &[
        "Use subject pronouns, including man, and singular and plural object pronouns in the accusative and dative",
        "Use reflexive verbs with accusative and dative reflexive pronouns: ich freue mich, ich wasche mir die Hände",
        "Ask questions with wer, wen and wem, and use jemand and niemand",
        "Write relative clauses with der, die, das (subject and object) and with wo and was, sending the verb to the end",
        "Order two objects correctly: dative noun before accusative noun, pronoun before noun",
    ], "A relative pronoun takes its gender from the noun but its case from its own clause: der Film, den ich gesehen habe, not der Film, der ich gesehen habe."),

    ("ger_edx:G2a", &[
        "Form questions by inversion and with was, wann, wie, wer, wo, wohin, woher and warum",
        "Conjugate weak and strong verbs in the present, including the vowel change in the du and er/sie/es forms",
        "Use haben, sein, werden and wissen in every person, and haben Hunger, Durst and Angst",
        "Use the present with a time adverb for the future, and seit + present for 'have been ...ing'",
    ], "Seit takes the present tense: Ich lerne seit vier Jahren Deutsch, not Ich habe seit vier Jahren Deutsch gelernt."),

    ("ger_edx:G2b", &[
        "Form past participles: ge-...-t, -iert, inseparable prefixes, and strong participles with vowel changes",
        "Choose haben or sein and put the participle at the end",
        "Use the perfect with früher for 'used to', and war, hatte and es gab for description in the past",
        "Place separable prefixes correctly: ich bin früh aufgestanden",
    ], "Verbs of movement and change of state take sein: ich bin gefahren, ich bin geblieben, ich bin gewesen."),

    ("ger_edx:G2c", &[
        "Recognise and write the simple past of weak verbs and of the strong verbs on Pearson's list (ging, fuhr, kam, sah ...)",
        "Use modal verbs in the simple past in all persons: konnte, musste, durfte, wollte, sollte, mochte",
        "Give instructions with the du, ihr and Sie imperatives, including sei, seid and seien Sie",
    ], "Modals in the simple past drop the umlaut: ich konnte, ich musste, ich durfte; ich könnte means 'I could' in the sense of 'would be able to'."),

    ("ger_edx:G2d", &[
        "Form the future with werden and the infinitive at the end",
        "Use all six modal verbs in the present, and möchte with a noun or an infinitive",
        "Write conditional sentences with wenn, hätte, wäre and würde + infinitive",
        "Give advice with sollte: Man sollte weniger Fleisch essen",
    ], "In a wenn clause the verb goes to the end, then the main clause starts with its verb: Wenn ich reich wäre, würde ich reisen."),

    ("ger_edx:G2e", &[
        "Keep the verb second, invert after a time phrase, and send a second verb to the end",
        "Send the verb to the end after weil, dass, wenn, obwohl, als and relative pronouns, with one or two verbs and with separable verbs",
        "Use separable and reflexive verbs in main and subordinate clauses",
        "Negate with nicht, nie, nichts and kein, and correct with nicht ... sondern",
        "Order adverbs time, manner, place",
    ], "After weil the conjugated verb goes last, after any infinitive: weil ich morgen nicht kommen kann, not weil ich kann morgen nicht kommen."),

    ("ger_edx:G3-4", &[
        "Add the right adjective ending after der-words, after ein-words and with no article, in three cases",
        "Compare with -er als and so ... wie, including besser, höher, mehr, größer and teurer",
        "Use superlatives before and after the noun: der beste Film, am besten, am höchsten, am meisten",
        "Express likes and preferences with gern, lieber and am liebsten, and place adverbs time, manner, place",
    ], "An adjective after the noun and sein takes no ending: Der Film war spannend, but ein spannender Film."),

    ("ger_edx:G5", &[
        "Use the right case after accusative, dative and two-way prepositions, including the Higher ones such as gegen, seit, zwischen and neben",
        "Contract preposition and article: am, beim, im, vom, zum, zur, ins",
        "Use verbs with fixed prepositions, and da- and wo- compounds: Ich freue mich darauf; Worauf wartest du?",
        "Build infinitive clauses with um ... zu, ohne ... zu and statt ... zu, and use beim + a verb noun",
        "Avoid the passive with man: Hier spricht man Deutsch",
    ], "Two-way prepositions take the accusative for movement and the dative for position: Ich lege das Buch auf den Tisch, but Das Buch liegt auf dem Tisch."),

    ("ger_edx:G6", &[
        "Work out nouns and adjectives built with Lieblings-, Haupt- and un-",
        "Decode -ung and -er nouns made from verbs, ordinal numbers, and adverbs such as montags",
        "Recognise the Higher suffixes -chen, -lein, -heit, -keit and -los",
        "Use word parts and context to answer the Reading paper's inference questions",
    ], "-los means 'without': arbeitslos is unemployed, not hard-working."),

    ("ger_edx:G7", &[
        "Pronounce and spell long and short vowels, the umlauts, ei, ie, eu and äu",
        "Say and write sch, sp-, st-, s, ß, z, w, v, j, qu and -tion the German way",
        "Hear hard and soft ch, final -b, -d and -g, -ig, and vocalic and consonantal r",
        "Transcribe dictation sentences accurately from the sound alone",
    ], "Ei sounds like English 'eye' and ie like 'ee': mein and schreiben, but sie and lieben; swapping them turns blieb into bleib."),

    ("ger_edx:P1", &[
        "Read aloud a 50-55 word text clearly, applying every sound-symbol correspondence",
        "Answer the two unprepared present-tense questions after the read aloud with an opinion and a reason",
        "Complete a five-prompt role play, asking two questions and answering one about the future",
        "Describe a picture's people, location and activity, then answer a present and a past question about it",
        "Sustain a five-minute conversation with developed answers in three time frames",
    ], "In the role play, a prompt that asks you to ask a question needs a real question: a statement in its place scores 0 for that prompt."),

    ("ger_edx:P2", &[
        "Use the five minutes' reading time to predict the German you will hear for each question",
        "Pick out key points, details and opinions from recordings played three times",
        "Avoid distractors by tracking negatives, time words and changes of mind",
        "Write the dictation: six missing words, then four whole sentences, spelled by the sound-symbol rules",
    ], "Section A answers must be in English: a German word copied from the recording earns nothing."),

    ("ger_edx:P3", &[
        "Answer gap-fill, multiple-choice, multiple-response and short English answers on eight texts",
        "Infer the meaning of two unlisted words from context and word parts",
        "Translate a five-sentence paragraph into natural, complete English in about 10 minutes",
        "Check tense, person and every small word in the translation against the German",
    ], "Translation marks go on omitted words: every word of the German, including vielleicht, oft and regelmäßig, must reach your English."),

    ("ger_edx:P4", &[
        "Choose the option in each question you can answer best, and address all four bullet points",
        "Write 80-90 words for Question 1 and 130-150 for Question 2, developing each bullet with an opinion, a reason or a detail",
        "Use past, present and future time frames successfully and add complex language: weil, wenn, relative and um ... zu clauses",
        "Translate a paragraph into German in about 10 minutes, keeping word order and cases accurate",
    ], "A missed bullet point caps the response-to-stimulus mark: with three of four bullets the best band is 8-10, whatever the quality."),
    // ---------- Spanish (Pearson Edexcel GCSE 1SP1) ----------
    ("spa_edx:T1", &[
        "Describe family members and friends, their personality and appearance, with adjectives that agree",
        "Explain how you get on with people and what makes a good friend, using llevarse bien con and reasons",
        "Discuss relationships, marriage and living arrangements in past, present and future time frames",
        "Give and justify opinions on equality, identity and inclusion using the list's vocabulary",
    ], "Mixing up ser and estar in descriptions (es simpático for personality, está cansado for a state) and forgetting that adjectives agree with the person described."),
    ("spa_edx:T2", &[
        "Describe a healthy or unhealthy routine with reflexive verbs and frequency expressions",
        "Discuss mental wellbeing, stress and sleep, and give advice with hay que, se debe and deberías",
        "Talk about food, drink and meals, including ordering and complaining in a café or restaurant",
        "Describe the sports you do, did and will do, with jugar a, hacer and practicar",
    ], "Writing juego fútbol or hago al fútbol: it is juego al fútbol, but hago natación with no a."),
    ("spa_edx:T3", &[
        "Describe your town or area: what there is and is not, and what you can do there",
        "Shop for clothes and food, and report a problem or make a complaint in a shop",
        "Compare ways of getting around and ask for and give directions",
        "Discuss environmental problems, the natural world and what you have done or will do to help",
    ], "Using hay with the definite article (hay el parque) or estar for 'there is': hay un parque, but el parque está cerca."),
    ("spa_edx:T4", &[
        "Discuss how you use your phone, social media and online gaming, and their advantages and dangers",
        "Give opinions on music, TV series and films with reasons, using me encanta, me interesa and lo mejor es",
        "Narrate a film, series or concert you saw, mixing the preterite and the imperfect",
        "Talk about how you will use technology in the future",
    ], "Gustar-type verbs agree with the thing liked, not with you: me gustan las series, me interesan los videojuegos."),
    ("spa_edx:T5", &[
        "Describe your school, subjects, teachers and rules, with opinions and reasons",
        "Compare school now with primary school using the imperfect",
        "Discuss future study, jobs and travel using the future, the conditional and cuando + subjunctive",
        "Weigh up the pros and cons of university, apprenticeships, jobs and working abroad",
    ], "Writing quiero ser un médico: Spanish drops the article before a job after ser - quiero ser médico."),
    ("spa_edx:T6", &[
        "Narrate a past holiday with the preterite for events and the imperfect for descriptions and weather",
        "Book accommodation in a hotel or campsite and complain about a problem",
        "Describe tourist attractions and what you can do there, using se puede and hay que",
        "Talk about the festivals on the list: Carnaval, las Fallas, la Tomatina, el Día de los Muertos and Nochevieja",
    ], "Weather in the past is hacía calor or hizo calor, never era calor or estaba calor."),
    ("spa_edx:G1a", &[
        "Form feminine nouns (-o to -a, add -a after -or, no change for -ante, -ente and -ista) and plurals, including -z to -ces and -ión to -iones",
        "Use definite and indefinite articles where Spanish differs from English, including with general nouns",
        "Use al and del, and an infinitive as a noun for the English -ing subject",
        "Form language and nationality nouns: el inglés, la española, los españoles",
    ], "Nouns ending in -ión lose the accent in the plural (la competición, las competiciones); nouns in -z change to -ces (la vez, las veces)."),
    ("spa_edx:G1b", &[
        "Make este, ese, cada, mismo, otro, todo, algún and ningún agree, and use the possessives mi, tu, su, nuestro and vuestro",
        "Leave out subject pronouns except for contrast or emphasis",
        "Place one direct, indirect or reflexive pronoun with one verb, two verbs and a command: lo leo, puedo leerlo, ¡léelo!",
        "Use que as a relative pronoun, esto and eso, alguno and ninguno, and the question words cuál, cuánto and quién",
    ], "A pronoun added to the end of a command usually needs a written accent to keep the stress: ¡léelo!, ¡prepárate!"),
    ("spa_edx:G1c", &[
        "Use aquel, aquella and aquello for 'that' at a distance",
        "Place nos and os with one and two verbs, including plural reflexives",
        "Use lo que, el que and el cual with agreement, and cuando, donde and que as relatives",
        "Use the possessive pronouns el mío, el tuyo, el suyo, el nuestro and el vuestro, and mío or tuyo after ser",
        "Use pronouns after prepositions, conmigo and contigo, and emphatic a mí and a ti",
    ], "Writing con mí or con ti instead of conmigo and contigo."),
    ("spa_edx:G2a", &[
        "Form negatives with no, nada, nunca, nadie and ninguno, before and after the verb",
        "Use the Higher negatives ya no, tampoco, ni...ni and no...ni",
        "Ask questions with intonation, and with a question word followed by the verb and then the subject",
        "Put a preposition in front of a question word: ¿con quién?, ¿de dónde?",
    ], "A negative word after the verb needs no in front of it (no veo nada, no viene nadie), but one before the verb does not (nunca viene)."),
    ("spa_edx:G2b", &[
        "Use hay, hay que, se puede and se necesita for general 'you' and 'one', and hace + noun for weather",
        "Use reflexive verbs in every tense, including reciprocal plurals such as nos vemos",
        "Use deber, poder, querer, saber and tener que + infinitive, and quisiera and me gustaría",
        "Use interesar-type verbs and the Higher impersonals parece, basta, falta, hace falta and vale la pena",
    ], "The verb after a modal stays in the infinitive: puedo ir, never puedo voy."),
    ("spa_edx:G2c", &[
        "Form the passive with ser + past participle + por, making the participle agree",
        "Use se + third person for a passive meaning: se venden entradas",
        "Use acabar de + infinitive for 'have just'",
        "Use seguir + gerund and llevar + time + gerund for actions still going on",
        "Use desde hace + present tense for 'have been ...ing for'",
    ], "'I have been living here for two years' is vivo aquí desde hace dos años or llevo dos años viviendo aquí - the present, not the perfect."),
    ("spa_edx:G3a", &[
        "Conjugate regular -ar, -er and -ir verbs in all six persons",
        "Apply the five anchor patterns: encontrar (o to ue), pensar (e to ie), pedir (e to i), conocer (-zco) and poner (-go)",
        "Use the irregular present of ser, estar, ir, hacer and tener, and tener expressions such as tengo frío",
        "Form the present continuous with estar + gerund, including leyendo and pidiendo",
        "Apply the Higher g-to-j spelling change: coger, cojo",
    ], "Stem changes never reach nosotros and vosotros: podemos, pensáis, pedimos."),
    ("spa_edx:G3b", &[
        "Form the regular preterite with its accents",
        "Use the irregular preterite of ir, ser, dar, tener, poder, hacer, venir, estar, poner, querer, decir and traer",
        "Apply the Higher spelling changes (busqué, utilicé, jugué, leyó) and the -ir stem changes (durmió, prefirieron)",
        "Form the perfect tense with haber and regular or irregular past participles such as visto and hecho",
        "Choose between the preterite and the perfect",
    ], "Regular preterite endings carry accents (compré, compró), but irregular stems do not (hice, hizo, fui, fue)."),
    ("spa_edx:G3c", &[
        "Form the imperfect of regular verbs and of ser, ir and ver in all persons",
        "Use the imperfect for habits ('used to') and for ongoing description ('was ...ing')",
        "Form the imperfect continuous with estaba + gerund",
        "Choose between the imperfect and the preterite in a past narrative",
    ], "Background description takes the imperfect, not the preterite: hacía sol, había mucha gente, era divertido."),
    ("spa_edx:G3d", &[
        "Use ir a + infinitive in all persons",
        "Form the future of regular verbs and of tener, hacer, poder, poner, saber, querer, venir, decir and salir, plus habrá",
        "Form the conditional of the same verbs, plus habría, and use me gustaría and quisiera",
        "Combine time frames with si + present + future",
    ], "Future and conditional endings go on the whole infinitive (comeré, viviría) except for the listed stems tendr-, har-, podr-, pondr-, sabr-, querr-, vendr-, dir- and saldr-."),
    ("spa_edx:G3e", &[
        "Give affirmative tú and vosotros commands, including sé, ve, ten, ven, haz, di, pon and sal",
        "Attach object and reflexive pronouns to a command with the right accent",
        "Form the present subjunctive of hacer, ser, ir, venir and tener in the singular",
        "Use the subjunctive after cuando with a future meaning, after verbs of wishing, command, request and emotion + que, and after para que",
    ], "The subjunctive needs a change of subject after querer and para: quiero ir, but quiero que vayas."),
    ("spa_edx:G4-5", &[
        "Make adjectives agree in gender and number, including -z to -ces, nationalities and consonant endings",
        "Place adjectives after the noun, with buen, mal, gran, primer and algún before it, and meaning changes such as un viejo amigo",
        "Use lo + adjective (lo bueno, lo malo) and possessives after ser (es mío)",
        "Form comparatives and superlatives of adjectives and adverbs, including mejor, peor, el mejor and tan...como",
        "Place adverbs of time, manner and place correctly",
    ], "Writing más bueno or el más mejor instead of mejor and el mejor."),
    ("spa_edx:G6", &[
        "Use the personal a before a person who is the object of a verb",
        "Use de for possession, and para, sin, antes de and después de + infinitive",
        "Use the prepositions verbs need (disfrutar de, llegar a) and those that change a verb's meaning",
        "Choose between por and para, and use the Higher según, a pesar de, debido a, hacia and a través de",
    ], "After a preposition Spanish uses the infinitive, not an -ing form: antes de salir, sin hablar."),
    ("spa_edx:G7", &[
        "Recognise -ito and -ita as 'little' or as affection",
        "Recognise -ísimo as 'very'",
        "Turn adjectives into -ly adverbs with -mente on the feminine form",
        "Recognise -idad nouns as '-ity' and -able adjectives as '-able'",
        "Work out an unknown derived word in the Reading paper from a base word on the list",
    ], "-mente goes on the feminine form: rápida gives rápidamente, not rápidomente."),
    ("spa_edx:G8", &[
        "Pronounce the vowels and ll, ch, ñ, j, h, v, r and rr correctly",
        "Apply the c and g rules before a, o and u versus e and i, including que, qui, gue and gui",
        "Find the stressed syllable from the spelling, and write accents where the rules require",
        "Transcribe unseen Spanish words from their sounds for the dictation",
    ], "A dictation word that tests stress is lost at Higher without its accent: movil for móvil, telefono for teléfono."),
    ("spa_edx:P1", &[
        "Use the 15 minutes' preparation to annotate the read-aloud text and plan the role play and picture",
        "Read 50-55 words aloud with accurate sounds and stress, then answer two unprepared opinion questions",
        "Complete the five role-play prompts, asking two questions and answering one in a future time frame",
        "Describe a picture's people, location and activity, answer two questions (the second in the past) and sustain a conversation in three time frames",
    ], "A one-word answer in the role play or the picture questions scores at most 1 of 2: always give a short full sentence."),
    ("spa_edx:P2", &[
        "Use the five minutes' reading time to predict the vocabulary each question needs",
        "Answer multiple choice, multiple response, gap-from-box and short-answer questions in English",
        "Spot distractors: negatives, changes of mind, past versus future, and who said what",
        "Transcribe the six dictation sentences using sound-symbol correspondences and the accent rules",
    ], "Choosing the option you heard a word for when the speaker then rejects it with pero, ya no or sin embargo."),
    ("spa_edx:P3", &[
        "Answer each Section A question type accurately in English, with the number of details asked for",
        "Infer the meaning of two words not on the vocabulary list from context and word families",
        "Use derived words, cognates and glosses, and avoid false friends",
        "Translate a Spanish paragraph into natural English with every tense, person and negative right",
    ], "Translating word for word ('I live in Madrid since three years') or slipping a tense in the Section B paragraph."),
    ("spa_edx:P4", &[
        "Cover all four bullet points of the 80-90 word task with developed ideas in three time frames",
        "Write the 130-150 word task with a real pros-and-cons paragraph and frequent complex language",
        "Translate a short English paragraph into accurate Spanish",
        "Plan the 80 minutes and check verbs, agreements and accents at the end",
    ], "Missing a bullet point caps the response mark below the top band, however good the rest of the answer is."),
];

/// Objectives written for a topic, or an empty slice if it has none yet.
pub fn objectives_for(topic_id: &str) -> &'static [&'static str] {
    TOPIC_DETAIL.iter().find(|(id, _, _)| *id == topic_id).map(|(_, o, _)| *o).unwrap_or(&[])
}

/// The mark people drop on a topic, or an empty string.
pub fn watch_for(topic_id: &str) -> &'static str {
    TOPIC_DETAIL.iter().find(|(id, _, _)| *id == topic_id).map(|(_, _, w)| *w).unwrap_or("")
}
