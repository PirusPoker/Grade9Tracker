//! Built-in mock papers: original exam-style papers that ramp from grade 4–5
//! questions to grade 8–9, mixing topics the way a real paper does.
//!
//! One Markdown file per paper under `papers/<subject>/<name>.md`, compiled in
//! so they work offline. The file starts with a header of `key: value` lines,
//! then a blank line, then the questions:
//!
//! ```text
//! title: Paper 1 (Calculator)
//! time: 90                      minutes on the clock
//! marks: 80                     must equal the sum of the questions' marks
//! boundaries: 4:35 5:45 6:55 7:65 8:75 9:85    grade: minimum percentage
//! intro: one sentence of instructions shown before the paper starts
//!
//! ::: q 4-5 2.7a 1.4b           the grade band, then the topic codes the
//!                               question draws on; bands never go backwards
//! 1. The question, with parts marked (2) and the total at the end. (5)
//! ---
//! The mark scheme.
//! :::
//! ```
//!
//! A test below fails the build if a paper is malformed or its marks do not
//! add up.

use serde::Serialize;

/// (paper id, subject id, text). The id is `<subject>/<file stem>`.
const PAPERS: &[(&str, &str, &str)] = &[
    ("maths/p1", "maths", include_str!("../papers/maths/p1.md")),
    ("maths/p2", "maths", include_str!("../papers/maths/p2.md")),
    ("phys/p1", "phys", include_str!("../papers/phys/p1.md")),
    ("phys/p2", "phys", include_str!("../papers/phys/p2.md")),
    ("chem/p1", "chem", include_str!("../papers/chem/p1.md")),
    ("chem/p2", "chem", include_str!("../papers/chem/p2.md")),
    ("bio/p1", "bio", include_str!("../papers/bio/p1.md")),
    ("bio/p2", "bio", include_str!("../papers/bio/p2.md")),
    ("fpm/p1", "fpm", include_str!("../papers/fpm/p1.md")),
    ("fpm/p2", "fpm", include_str!("../papers/fpm/p2.md")),
    ("cs/p1", "cs", include_str!("../papers/cs/p1.md")),
    ("cs/p2", "cs", include_str!("../papers/cs/p2.md")),
    ("econ/p1", "econ", include_str!("../papers/econ/p1.md")),
    ("econ/p2", "econ", include_str!("../papers/econ/p2.md")),
    ("bus/p1", "bus", include_str!("../papers/bus/p1.md")),
    ("bus/p2", "bus", include_str!("../papers/bus/p2.md")),
    ("englit/p1", "englit", include_str!("../papers/englit/p1.md")),
    ("englit/p2", "englit", include_str!("../papers/englit/p2.md")),
    ("englang/p1", "englang", include_str!("../papers/englang/p1.md")),
    ("englang/p2", "englang", include_str!("../papers/englang/p2.md")),
    ("geog/p1", "geog", include_str!("../papers/geog/p1.md")),
    ("geog/p2", "geog", include_str!("../papers/geog/p2.md")),
    ("spa/p1", "spa", include_str!("../papers/spa/p1.md")),
    ("spa/p2", "spa", include_str!("../papers/spa/p2.md")),
    ("ger/p1", "ger", include_str!("../papers/ger/p1.md")),
    ("ger/p2", "ger", include_str!("../papers/ger/p2.md")),
    ("fre/p1", "fre", include_str!("../papers/fre/p1.md")),
    ("fre/p2", "fre", include_str!("../papers/fre/p2.md")),
    ("hist/p1", "hist", include_str!("../papers/hist/p1.md")),
    ("hist/p2", "hist", include_str!("../papers/hist/p2.md")),
    ("music/p1", "music", include_str!("../papers/music/p1.md")),
    ("music/p2", "music", include_str!("../papers/music/p2.md")),
    ("rs/p1", "rs", include_str!("../papers/rs/p1.md")),
    ("rs/p2", "rs", include_str!("../papers/rs/p2.md")),
    ("drama/p1", "drama", include_str!("../papers/drama/p1.md")),
    ("drama/p2", "drama", include_str!("../papers/drama/p2.md")),
    ("pe/p1", "pe", include_str!("../papers/pe/p1.md")),
    ("pe/p2", "pe", include_str!("../papers/pe/p2.md")),
    ("media/p1", "media", include_str!("../papers/media/p1.md")),
    ("media/p2", "media", include_str!("../papers/media/p2.md")),
    ("dt/p1", "dt", include_str!("../papers/dt/p1.md")),
    ("dt/p2", "dt", include_str!("../papers/dt/p2.md")),
    ("food/p1", "food", include_str!("../papers/food/p1.md")),
    ("food/p2", "food", include_str!("../papers/food/p2.md")),
    ("maths_edx/p1", "maths_edx", include_str!("../papers/maths_edx/p1.md")),
    ("maths_edx/p2", "maths_edx", include_str!("../papers/maths_edx/p2.md")),
    ("maths_aqa/p1", "maths_aqa", include_str!("../papers/maths_aqa/p1.md")),
    ("maths_aqa/p2", "maths_aqa", include_str!("../papers/maths_aqa/p2.md")),
    ("maths_ocr/p1", "maths_ocr", include_str!("../papers/maths_ocr/p1.md")),
    ("maths_ocr/p2", "maths_ocr", include_str!("../papers/maths_ocr/p2.md")),
    ("englang_edq/p1", "englang_edq", include_str!("../papers/englang_edq/p1.md")),
    ("englang_edq/p2", "englang_edq", include_str!("../papers/englang_edq/p2.md")),
    ("bio_aqa/p1", "bio_aqa", include_str!("../papers/bio_aqa/p1.md")),
    ("bio_aqa/p2", "bio_aqa", include_str!("../papers/bio_aqa/p2.md")),
    ("econ_aqa/p1", "econ_aqa", include_str!("../papers/econ_aqa/p1.md")),
    ("econ_aqa/p2", "econ_aqa", include_str!("../papers/econ_aqa/p2.md")),
    ("chem_aqa/p2", "chem_aqa", include_str!("../papers/chem_aqa/p2.md")),
    ("phys_aqa/p1", "phys_aqa", include_str!("../papers/phys_aqa/p1.md")),
    ("phys_aqa/p2", "phys_aqa", include_str!("../papers/phys_aqa/p2.md")),
    ("englit_edq/p1", "englit_edq", include_str!("../papers/englit_edq/p1.md")),
    ("englit_edq/p2", "englit_edq", include_str!("../papers/englit_edq/p2.md")),
    ("bus_edx/p1", "bus_edx", include_str!("../papers/bus_edx/p1.md")),
    ("bus_edx/p2", "bus_edx", include_str!("../papers/bus_edx/p2.md")),
    ("cs_aqa/p1", "cs_aqa", include_str!("../papers/cs_aqa/p1.md")),
    ("cs_aqa/p2", "cs_aqa", include_str!("../papers/cs_aqa/p2.md")),
    ("hist_aqa/p1", "hist_aqa", include_str!("../papers/hist_aqa/p1.md")),
    ("hist_aqa/p2", "hist_aqa", include_str!("../papers/hist_aqa/p2.md")),
];

#[derive(Serialize, Clone, Debug)]
pub struct PaperInfo {
    pub id: String,
    pub subject: String,
    pub title: String,
    pub time: u32,
    pub marks: u32,
    pub questions: usize,
    pub intro: String,
    /// `[(grade, minimum percentage)]`, highest grade first.
    pub boundaries: Vec<(u32, u32)>,
}

/// The header lines of a paper, as (key, value) pairs.
fn header(text: &str) -> Vec<(&str, &str)> {
    text.lines()
        .take_while(|l| !l.trim().is_empty())
        .filter_map(|l| l.split_once(':').map(|(k, v)| (k.trim(), v.trim())))
        .collect()
}

fn field<'a>(h: &[(&'a str, &'a str)], key: &str) -> Option<&'a str> {
    h.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

fn parse_boundaries(s: &str) -> Vec<(u32, u32)> {
    let mut out: Vec<(u32, u32)> = s
        .split_whitespace()
        .filter_map(|p| p.split_once(':'))
        .filter_map(|(g, m)| Some((g.parse().ok()?, m.parse().ok()?)))
        .collect();
    out.sort_by(|a, b| b.0.cmp(&a.0));
    out
}

fn info(id: &str, subject: &str, text: &str) -> PaperInfo {
    let h = header(text);
    PaperInfo {
        id: id.to_string(),
        subject: subject.to_string(),
        title: field(&h, "title").unwrap_or(id).to_string(),
        time: field(&h, "time").and_then(|v| v.parse().ok()).unwrap_or(60),
        marks: field(&h, "marks").and_then(|v| v.parse().ok()).unwrap_or(0),
        questions: text.lines().filter(|l| l.trim_end().starts_with("::: q")).count(),
        intro: field(&h, "intro").unwrap_or("").to_string(),
        boundaries: parse_boundaries(field(&h, "boundaries").unwrap_or("")),
    }
}

/// Every paper, in the order they are listed.
pub fn all() -> Vec<PaperInfo> {
    PAPERS.iter().map(|(id, subject, text)| info(id, subject, text)).collect()
}

/// The full text of one paper.
pub fn paper(id: &str) -> Option<&'static str> {
    PAPERS.iter().find(|(pid, _, _)| *pid == id).map(|(_, _, text)| *text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn questions(text: &str) -> Vec<(String, Vec<String>, String)> {
        // (band, topic codes, body) for each `::: q <band> <topics…>` block
        let mut out = Vec::new();
        let mut cur: Option<(String, Vec<String>, String)> = None;
        for line in text.lines() {
            let t = line.trim_end();
            if let Some(rest) = t.strip_prefix("::: ") {
                assert!(cur.is_none(), "block opened inside a block: {t}");
                let mut it = rest.split_whitespace();
                assert_eq!(it.next(), Some("q"), "papers only contain q blocks: {t}");
                let band = it.next().unwrap_or("").to_string();
                let topics: Vec<String> = it.map(str::to_string).collect();
                cur = Some((band, topics, String::new()));
            } else if t == ":::" {
                out.push(cur.take().expect("::: without an open block"));
            } else if let Some((_, _, b)) = cur.as_mut() {
                b.push_str(line);
                b.push('\n');
            }
        }
        assert!(cur.is_none(), "unclosed block");
        out
    }

    fn last_marks(q: &str) -> u32 {
        let q = q.trim_end();
        let open = q.rfind('(').expect("question ends with its marks in brackets");
        assert!(q.ends_with(')'), "question ends with its marks in brackets: {q}");
        q[open + 1..q.len() - 1].trim().parse().expect("marks are a number")
    }

    #[test]
    fn every_paper_is_well_formed() {
        let catalog = crate::config::catalog();
        let bands = ["4-5", "6-7", "8-9"];
        for (id, subject, text) in PAPERS {
            let def = catalog.iter().find(|s| s.id == *subject).unwrap_or_else(|| panic!("{id}: unknown subject {subject}"));
            let i = info(id, subject, text);
            assert!(!i.title.is_empty() && i.time > 0 && i.marks > 0, "{id}: header needs title, time and marks");
            assert_eq!(i.boundaries.len(), 6, "{id}: boundaries must give grades 4 to 9");
            assert!(i.boundaries.windows(2).all(|w| w[0].1 > w[1].1), "{id}: boundaries must decrease with grade");
            let qs = questions(text);
            // A paper of long written answers (a languages writing paper is three
            // questions) may have fewer blocks, as long as each one is substantial.
            let long_answers = qs.len() >= 3 && i.marks >= 7 * qs.len() as u32;
            assert!(qs.len() >= 8 || long_answers, "{id}: too few questions ({})", qs.len());
            let mut total = 0;
            let mut last_band = 0;
            for (n, (band, topics, body)) in qs.iter().enumerate() {
                assert!(!topics.is_empty(), "{id}: question {} names no topics", n + 1);
                for code in topics {
                    assert!(def.topics.iter().any(|t| t.code == *code), "{id}: question {} names topic {code}, which {subject} does not have", n + 1);
                }
                let bi = bands.iter().position(|b| b == band).unwrap_or_else(|| panic!("{id}: question {} has band {band:?}, expected one of {bands:?}", n + 1));
                assert!(bi >= last_band, "{id}: question {} steps down a band", n + 1);
                last_band = bi;
                let parts: Vec<&str> = body.split("\n---\n").collect();
                assert_eq!(parts.len(), 2, "{id}: question {} has no single --- divider", n + 1);
                assert!(parts[0].trim().len() > 10 && parts[1].trim().len() > 5, "{id}: question {} is thin", n + 1);
                total += last_marks(parts[0]);
                assert!(!body.contains("TODO"), "{id}: TODO left in");
            }
            assert_eq!(total, i.marks, "{id}: questions add up to {total} marks, header says {}", i.marks);
            assert!(last_band == 2, "{id}: the paper should reach the 8-9 band");
            let dollars = text.replace("\\$", "").matches('$').count();
            assert_eq!(dollars % 2, 0, "{id}: unbalanced $ maths delimiters");
        }
    }
}
