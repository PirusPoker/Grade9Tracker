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

/// Every lesson, across subjects.
fn all() -> impl Iterator<Item = &'static (&'static str, &'static str)> {
    FPM.iter()
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

    #[test]
    fn every_further_pure_topic_has_a_lesson() {
        let cfg = crate::config::PlanConfig::default();
        let fpm = cfg.subjects.iter().find(|s| s.id == "fpm").expect("fpm subject");
        let missing: Vec<_> = fpm.topics.iter().map(|t| format!("fpm:{}", t.code)).filter(|id| !has_lesson(id)).collect();
        assert!(missing.is_empty(), "topics with no lesson: {missing:?}");
        let extra: Vec<_> = FPM.iter().map(|(id, _)| *id)
            .filter(|id| !fpm.topics.iter().any(|t| format!("fpm:{}", t.code) == *id)).collect();
        assert!(extra.is_empty(), "lessons for topics that do not exist: {extra:?}");
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
