//! Built-in lessons: the teaching itself, not just the order of it.
//!
//! One Markdown file per topic under `lessons/<subject>/<code>.md`, compiled
//! into the binary so the app teaches offline. The format is ordinary
//! Markdown with `$...$` / `$$...$$` maths, plus four fenced blocks the UI
//! renders specially:
//!
//! ```text
//! ::: method            the steps, as a numbered list
//! ::: example Title     a worked example, every line shown
//! ::: q                 a practice question; the answer follows a `---` line
//! ::: watch             where the marks go in the exam
//! :::
//! ```
//!
//! A test below fails the build if a topic that should have a lesson has
//! none, if a lesson is thin, or if a question has no answer.

/// (topic id, lesson text). Further Pure Mathematics, Pearson 4PM1.
const FPM: &[(&str, &str)] = &[
    ("fpm:1a", include_str!("../lessons/fpm/1a.md")),
    ("fpm:1b", include_str!("../lessons/fpm/1b.md")),
    ("fpm:2", include_str!("../lessons/fpm/2.md")),
    ("fpm:3a", include_str!("../lessons/fpm/3a.md")),
    ("fpm:3b", include_str!("../lessons/fpm/3b.md")),
    ("fpm:3c", include_str!("../lessons/fpm/3c.md")),
    ("fpm:4", include_str!("../lessons/fpm/4.md")),
    ("fpm:5", include_str!("../lessons/fpm/5.md")),
    ("fpm:6", include_str!("../lessons/fpm/6.md")),
    ("fpm:7", include_str!("../lessons/fpm/7.md")),
    ("fpm:8", include_str!("../lessons/fpm/8.md")),
    ("fpm:9a", include_str!("../lessons/fpm/9a.md")),
    ("fpm:9b", include_str!("../lessons/fpm/9b.md")),
    ("fpm:9c", include_str!("../lessons/fpm/9c.md")),
    ("fpm:9d", include_str!("../lessons/fpm/9d.md")),
    ("fpm:10a", include_str!("../lessons/fpm/10a.md")),
    ("fpm:10b", include_str!("../lessons/fpm/10b.md")),
    ("fpm:10c", include_str!("../lessons/fpm/10c.md")),
];

/// Mathematics A (4MA1) Higher, Pearson Edexcel International GCSE.
const MATHS: &[(&str, &str)] = &[
    ("maths:1.1", include_str!("../lessons/maths/1.1.md")),
    ("maths:1.2", include_str!("../lessons/maths/1.2.md")),
    ("maths:1.3", include_str!("../lessons/maths/1.3.md")),
    ("maths:1.4a", include_str!("../lessons/maths/1.4a.md")),
    ("maths:1.4b", include_str!("../lessons/maths/1.4b.md")),
    ("maths:1.5", include_str!("../lessons/maths/1.5.md")),
    ("maths:1.6", include_str!("../lessons/maths/1.6.md")),
    ("maths:1.7", include_str!("../lessons/maths/1.7.md")),
    ("maths:1.8", include_str!("../lessons/maths/1.8.md")),
    ("maths:1.9", include_str!("../lessons/maths/1.9.md")),
    ("maths:1.10", include_str!("../lessons/maths/1.10.md")),
    ("maths:1.11", include_str!("../lessons/maths/1.11.md")),
    ("maths:2.1", include_str!("../lessons/maths/2.1.md")),
    ("maths:2.2a", include_str!("../lessons/maths/2.2a.md")),
    ("maths:2.2b", include_str!("../lessons/maths/2.2b.md")),
    ("maths:2.3", include_str!("../lessons/maths/2.3.md")),
    ("maths:2.4", include_str!("../lessons/maths/2.4.md")),
    ("maths:2.5", include_str!("../lessons/maths/2.5.md")),
    ("maths:2.6", include_str!("../lessons/maths/2.6.md")),
    ("maths:2.7a", include_str!("../lessons/maths/2.7a.md")),
    ("maths:2.7b", include_str!("../lessons/maths/2.7b.md")),
    ("maths:2.8", include_str!("../lessons/maths/2.8.md")),
    ("maths:3.1a", include_str!("../lessons/maths/3.1a.md")),
    ("maths:3.1b", include_str!("../lessons/maths/3.1b.md")),
    ("maths:3.2", include_str!("../lessons/maths/3.2.md")),
    ("maths:3.3a", include_str!("../lessons/maths/3.3a.md")),
    ("maths:3.3b", include_str!("../lessons/maths/3.3b.md")),
    ("maths:3.3c", include_str!("../lessons/maths/3.3c.md")),
    ("maths:3.4", include_str!("../lessons/maths/3.4.md")),
    ("maths:4.1", include_str!("../lessons/maths/4.1.md")),
    ("maths:4.2", include_str!("../lessons/maths/4.2.md")),
    ("maths:4.3", include_str!("../lessons/maths/4.3.md")),
    ("maths:4.4", include_str!("../lessons/maths/4.4.md")),
    ("maths:4.5", include_str!("../lessons/maths/4.5.md")),
    ("maths:4.6", include_str!("../lessons/maths/4.6.md")),
    ("maths:4.7", include_str!("../lessons/maths/4.7.md")),
    ("maths:4.8a", include_str!("../lessons/maths/4.8a.md")),
    ("maths:4.8b", include_str!("../lessons/maths/4.8b.md")),
    ("maths:4.8c", include_str!("../lessons/maths/4.8c.md")),
    ("maths:4.9", include_str!("../lessons/maths/4.9.md")),
    ("maths:4.10", include_str!("../lessons/maths/4.10.md")),
    ("maths:4.11", include_str!("../lessons/maths/4.11.md")),
    ("maths:5.1", include_str!("../lessons/maths/5.1.md")),
    ("maths:5.2", include_str!("../lessons/maths/5.2.md")),
    ("maths:6.1a", include_str!("../lessons/maths/6.1a.md")),
    ("maths:6.1b", include_str!("../lessons/maths/6.1b.md")),
    ("maths:6.1c", include_str!("../lessons/maths/6.1c.md")),
    ("maths:6.2", include_str!("../lessons/maths/6.2.md")),
    ("maths:6.3a", include_str!("../lessons/maths/6.3a.md")),
    ("maths:6.3b", include_str!("../lessons/maths/6.3b.md")),
];

/// Economics, Cambridge IGCSE (9-1) 0987.
const ECON: &[(&str, &str)] = &[
    ("econ:1.1", include_str!("../lessons/econ/1.1.md")),
    ("econ:1.2", include_str!("../lessons/econ/1.2.md")),
    ("econ:1.3", include_str!("../lessons/econ/1.3.md")),
    ("econ:1.4", include_str!("../lessons/econ/1.4.md")),
    ("econ:2.1", include_str!("../lessons/econ/2.1.md")),
    ("econ:2.2", include_str!("../lessons/econ/2.2.md")),
    ("econ:2.3", include_str!("../lessons/econ/2.3.md")),
    ("econ:2.4", include_str!("../lessons/econ/2.4.md")),
    ("econ:2.5", include_str!("../lessons/econ/2.5.md")),
    ("econ:2.6", include_str!("../lessons/econ/2.6.md")),
    ("econ:2.7", include_str!("../lessons/econ/2.7.md")),
    ("econ:2.8", include_str!("../lessons/econ/2.8.md")),
    ("econ:2.9", include_str!("../lessons/econ/2.9.md")),
    ("econ:2.10", include_str!("../lessons/econ/2.10.md")),
    ("econ:3.1", include_str!("../lessons/econ/3.1.md")),
    ("econ:3.2", include_str!("../lessons/econ/3.2.md")),
    ("econ:3.3", include_str!("../lessons/econ/3.3.md")),
    ("econ:3.4", include_str!("../lessons/econ/3.4.md")),
    ("econ:3.5", include_str!("../lessons/econ/3.5.md")),
    ("econ:3.6", include_str!("../lessons/econ/3.6.md")),
    ("econ:3.7", include_str!("../lessons/econ/3.7.md")),
    ("econ:4.1", include_str!("../lessons/econ/4.1.md")),
    ("econ:4.2", include_str!("../lessons/econ/4.2.md")),
    ("econ:4.3", include_str!("../lessons/econ/4.3.md")),
    ("econ:4.4", include_str!("../lessons/econ/4.4.md")),
    ("econ:4.5", include_str!("../lessons/econ/4.5.md")),
    ("econ:4.6", include_str!("../lessons/econ/4.6.md")),
    ("econ:4.7", include_str!("../lessons/econ/4.7.md")),
    ("econ:5.1", include_str!("../lessons/econ/5.1.md")),
    ("econ:5.2", include_str!("../lessons/econ/5.2.md")),
    ("econ:5.3", include_str!("../lessons/econ/5.3.md")),
    ("econ:5.4", include_str!("../lessons/econ/5.4.md")),
    ("econ:6.1", include_str!("../lessons/econ/6.1.md")),
    ("econ:6.2", include_str!("../lessons/econ/6.2.md")),
    ("econ:6.3", include_str!("../lessons/econ/6.3.md")),
    ("econ:6.4", include_str!("../lessons/econ/6.4.md")),
];

/// Every lesson, across subjects.
fn all() -> impl Iterator<Item = &'static (&'static str, &'static str)> {
    FPM.iter().chain(MATHS.iter()).chain(ECON.iter())
}

/// The lesson for a topic id such as `fpm:9a`, if one has been written.
pub fn lesson(topic_id: &str) -> Option<&'static str> {
    all().find(|(id, _)| *id == topic_id).map(|(_, text)| *text)
}

pub fn has_lesson(topic_id: &str) -> bool {
    lesson(topic_id).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blocks(text: &str, kind: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut cur: Option<String> = None;
        for line in text.lines() {
            let t = line.trim_end();
            if let Some(rest) = t.strip_prefix("::: ") {
                if cur.is_some() {
                    panic!("block opened inside a block: {t}");
                }
                cur = Some(String::new());
                if rest.split_whitespace().next() != Some(kind) {
                    cur = Some("\u{0}other".to_string());
                }
            } else if t == ":::" {
                let b = cur.take().expect("::: without an open block");
                if !b.starts_with('\u{0}') {
                    out.push(b);
                }
            } else if let Some(b) = cur.as_mut() {
                b.push_str(line);
                b.push('\n');
            }
        }
        assert!(cur.is_none(), "unclosed block");
        out
    }

    /// Subjects the app claims to teach: every topic must carry a lesson,
    /// and no lesson may name a topic that does not exist.
    #[test]
    fn every_topic_of_a_taught_subject_has_a_lesson() {
        let cfg = crate::config::PlanConfig::default();
        for (subject, table) in [("fpm", FPM), ("maths", MATHS), ("econ", ECON)] {
            let def = cfg.subjects.iter().find(|s| s.id == subject).expect("subject");
            let missing: Vec<_> = def.topics.iter().map(|t| format!("{subject}:{}", t.code)).filter(|id| !has_lesson(id)).collect();
            assert!(missing.is_empty(), "{subject}: topics with no lesson: {missing:?}");
            let extra: Vec<_> = table.iter().map(|(id, _)| *id)
                .filter(|id| !def.topics.iter().any(|t| format!("{subject}:{}", t.code) == *id)).collect();
            assert!(extra.is_empty(), "{subject}: lessons for topics that do not exist: {extra:?}");
        }
    }

    #[test]
    fn every_lesson_is_a_full_lesson() {
        for (id, text) in all() {
            assert!(text.contains("## What it is"), "{id}: no What it is");
            assert_eq!(blocks(text, "method").len(), 1, "{id}: needs exactly one method block");
            assert!(blocks(text, "example").len() >= 3, "{id}: fewer than three worked examples");
            let qs = blocks(text, "q");
            assert!(qs.len() >= 6, "{id}: fewer than six practice questions ({})", qs.len());
            for (i, q) in qs.iter().enumerate() {
                let parts: Vec<&str> = q.split("\n---\n").collect();
                assert_eq!(parts.len(), 2, "{id}: question {} has no single --- answer divider", i + 1);
                assert!(parts[0].trim().len() > 10, "{id}: question {} is empty", i + 1);
                assert!(parts[1].trim().len() > 5, "{id}: question {} has no answer", i + 1);
            }
            assert_eq!(blocks(text, "watch").len(), 1, "{id}: needs exactly one watch block");
            let dollars = text.replace("\\$", "").matches('$').count();
            assert_eq!(dollars % 2, 0, "{id}: unbalanced $ maths delimiters");
            assert!(!text.contains("TODO"), "{id}: TODO left in");
        }
    }
}
