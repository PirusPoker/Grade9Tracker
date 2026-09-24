//! Built-in lessons: the teaching itself, not just the order of it.
//!
//! One Markdown file per topic under `lessons/<subject>/<code>.md`, compiled
//! into the binary so the app teaches offline. The format is ordinary
//! Markdown with `$...$` / `$$...$$` maths, plus four fenced blocks the UI
//! renders specially:
//!
//! ```text
//! ::: terms             the key terms, one "- **term**: definition" line each
//! ::: method            the steps, as a numbered list
//! ::: example Title     a worked example, every line shown
//! ::: q                 a practice question; the answer follows a `---` line
//! ::: watch             where the marks go in the exam
//! ::: test              an end-of-topic test question, same shape as `q`;
//!                       optional: without any, the test uses the `q`s
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

/// Computer Science, OCR GCSE (9-1) J277.
const CS: &[(&str, &str)] = &[
    ("cs:1.1.1", include_str!("../lessons/cs/1.1.1.md")),
    ("cs:1.1.2", include_str!("../lessons/cs/1.1.2.md")),
    ("cs:1.1.3", include_str!("../lessons/cs/1.1.3.md")),
    ("cs:1.2.1", include_str!("../lessons/cs/1.2.1.md")),
    ("cs:1.2.2", include_str!("../lessons/cs/1.2.2.md")),
    ("cs:1.2.3", include_str!("../lessons/cs/1.2.3.md")),
    ("cs:1.2.4", include_str!("../lessons/cs/1.2.4.md")),
    ("cs:1.2.5", include_str!("../lessons/cs/1.2.5.md")),
    ("cs:1.3.1", include_str!("../lessons/cs/1.3.1.md")),
    ("cs:1.3.2", include_str!("../lessons/cs/1.3.2.md")),
    ("cs:1.4.1", include_str!("../lessons/cs/1.4.1.md")),
    ("cs:1.4.2", include_str!("../lessons/cs/1.4.2.md")),
    ("cs:1.5.1", include_str!("../lessons/cs/1.5.1.md")),
    ("cs:1.5.2", include_str!("../lessons/cs/1.5.2.md")),
    ("cs:1.6.1", include_str!("../lessons/cs/1.6.1.md")),
    ("cs:2.1.1", include_str!("../lessons/cs/2.1.1.md")),
    ("cs:2.1.2", include_str!("../lessons/cs/2.1.2.md")),
    ("cs:2.1.3", include_str!("../lessons/cs/2.1.3.md")),
    ("cs:2.2.1", include_str!("../lessons/cs/2.2.1.md")),
    ("cs:2.2.2", include_str!("../lessons/cs/2.2.2.md")),
    ("cs:2.2.3", include_str!("../lessons/cs/2.2.3.md")),
    ("cs:2.3.1", include_str!("../lessons/cs/2.3.1.md")),
    ("cs:2.3.2", include_str!("../lessons/cs/2.3.2.md")),
    ("cs:2.4.1", include_str!("../lessons/cs/2.4.1.md")),
    ("cs:2.5.1", include_str!("../lessons/cs/2.5.1.md")),
    ("cs:2.5.2", include_str!("../lessons/cs/2.5.2.md")),
];

/// Business, AQA GCSE (9-1) 8132.
const BUS: &[(&str, &str)] = &[
    ("bus:3.1.1", include_str!("../lessons/bus/3.1.1.md")),
    ("bus:3.1.2", include_str!("../lessons/bus/3.1.2.md")),
    ("bus:3.1.3", include_str!("../lessons/bus/3.1.3.md")),
    ("bus:3.1.4", include_str!("../lessons/bus/3.1.4.md")),
    ("bus:3.1.5", include_str!("../lessons/bus/3.1.5.md")),
    ("bus:3.1.6", include_str!("../lessons/bus/3.1.6.md")),
    ("bus:3.1.7", include_str!("../lessons/bus/3.1.7.md")),
    ("bus:3.2.1", include_str!("../lessons/bus/3.2.1.md")),
    ("bus:3.2.2", include_str!("../lessons/bus/3.2.2.md")),
    ("bus:3.2.3", include_str!("../lessons/bus/3.2.3.md")),
    ("bus:3.2.4", include_str!("../lessons/bus/3.2.4.md")),
    ("bus:3.2.5", include_str!("../lessons/bus/3.2.5.md")),
    ("bus:3.2.6", include_str!("../lessons/bus/3.2.6.md")),
    ("bus:3.3.1", include_str!("../lessons/bus/3.3.1.md")),
    ("bus:3.3.2", include_str!("../lessons/bus/3.3.2.md")),
    ("bus:3.3.3", include_str!("../lessons/bus/3.3.3.md")),
    ("bus:3.3.4", include_str!("../lessons/bus/3.3.4.md")),
    ("bus:3.4.1", include_str!("../lessons/bus/3.4.1.md")),
    ("bus:3.4.2", include_str!("../lessons/bus/3.4.2.md")),
    ("bus:3.4.3", include_str!("../lessons/bus/3.4.3.md")),
    ("bus:3.4.4", include_str!("../lessons/bus/3.4.4.md")),
    ("bus:3.5.1", include_str!("../lessons/bus/3.5.1.md")),
    ("bus:3.5.2", include_str!("../lessons/bus/3.5.2.md")),
    ("bus:3.5.3", include_str!("../lessons/bus/3.5.3.md")),
    ("bus:3.5.4", include_str!("../lessons/bus/3.5.4.md")),
    ("bus:3.6.1", include_str!("../lessons/bus/3.6.1.md")),
    ("bus:3.6.2", include_str!("../lessons/bus/3.6.2.md")),
    ("bus:3.6.3", include_str!("../lessons/bus/3.6.3.md")),
    ("bus:3.6.4", include_str!("../lessons/bus/3.6.4.md")),
];

/// Biology, Pearson Edexcel International GCSE 4BI1.
const BIO: &[(&str, &str)] = &[
    ("bio:1a", include_str!("../lessons/bio/1a.md")),
    ("bio:2a", include_str!("../lessons/bio/2a.md")),
    ("bio:2b", include_str!("../lessons/bio/2b.md")),
    ("bio:2c", include_str!("../lessons/bio/2c.md")),
    ("bio:2d", include_str!("../lessons/bio/2d.md")),
    ("bio:2e", include_str!("../lessons/bio/2e.md")),
    ("bio:2f", include_str!("../lessons/bio/2f.md")),
    ("bio:2g", include_str!("../lessons/bio/2g.md")),
    ("bio:2h", include_str!("../lessons/bio/2h.md")),
    ("bio:2i", include_str!("../lessons/bio/2i.md")),
    ("bio:2j", include_str!("../lessons/bio/2j.md")),
    ("bio:2k", include_str!("../lessons/bio/2k.md")),
    ("bio:2l", include_str!("../lessons/bio/2l.md")),
    ("bio:2m", include_str!("../lessons/bio/2m.md")),
    ("bio:2n", include_str!("../lessons/bio/2n.md")),
    ("bio:2o", include_str!("../lessons/bio/2o.md")),
    ("bio:3a", include_str!("../lessons/bio/3a.md")),
    ("bio:3b", include_str!("../lessons/bio/3b.md")),
    ("bio:3c", include_str!("../lessons/bio/3c.md")),
    ("bio:3d", include_str!("../lessons/bio/3d.md")),
    ("bio:3e", include_str!("../lessons/bio/3e.md")),
    ("bio:3f", include_str!("../lessons/bio/3f.md")),
    ("bio:4a", include_str!("../lessons/bio/4a.md")),
    ("bio:4b", include_str!("../lessons/bio/4b.md")),
    ("bio:4c", include_str!("../lessons/bio/4c.md")),
    ("bio:4d", include_str!("../lessons/bio/4d.md")),
    ("bio:5a", include_str!("../lessons/bio/5a.md")),
    ("bio:5b", include_str!("../lessons/bio/5b.md")),
    ("bio:5c", include_str!("../lessons/bio/5c.md")),
    ("bio:5d", include_str!("../lessons/bio/5d.md")),
    ("bio:5e", include_str!("../lessons/bio/5e.md")),
];

/// Chemistry, Pearson Edexcel International GCSE 4CH1.
const CHEM: &[(&str, &str)] = &[
    ("chem:1a", include_str!("../lessons/chem/1a.md")),
    ("chem:1b", include_str!("../lessons/chem/1b.md")),
    ("chem:1c", include_str!("../lessons/chem/1c.md")),
    ("chem:1d", include_str!("../lessons/chem/1d.md")),
    ("chem:1e", include_str!("../lessons/chem/1e.md")),
    ("chem:1f", include_str!("../lessons/chem/1f.md")),
    ("chem:1g", include_str!("../lessons/chem/1g.md")),
    ("chem:1h", include_str!("../lessons/chem/1h.md")),
    ("chem:1i", include_str!("../lessons/chem/1i.md")),
    ("chem:1j", include_str!("../lessons/chem/1j.md")),
    ("chem:1k", include_str!("../lessons/chem/1k.md")),
    ("chem:1l", include_str!("../lessons/chem/1l.md")),
    ("chem:2a", include_str!("../lessons/chem/2a.md")),
    ("chem:2b", include_str!("../lessons/chem/2b.md")),
    ("chem:2c", include_str!("../lessons/chem/2c.md")),
    ("chem:2d", include_str!("../lessons/chem/2d.md")),
    ("chem:2e", include_str!("../lessons/chem/2e.md")),
    ("chem:2f", include_str!("../lessons/chem/2f.md")),
    ("chem:2g", include_str!("../lessons/chem/2g.md")),
    ("chem:2h", include_str!("../lessons/chem/2h.md")),
    ("chem:3a", include_str!("../lessons/chem/3a.md")),
    ("chem:3b", include_str!("../lessons/chem/3b.md")),
    ("chem:3c", include_str!("../lessons/chem/3c.md")),
    ("chem:4a", include_str!("../lessons/chem/4a.md")),
    ("chem:4b", include_str!("../lessons/chem/4b.md")),
    ("chem:4c", include_str!("../lessons/chem/4c.md")),
    ("chem:4d", include_str!("../lessons/chem/4d.md")),
    ("chem:4e", include_str!("../lessons/chem/4e.md")),
    ("chem:4f", include_str!("../lessons/chem/4f.md")),
    ("chem:4g", include_str!("../lessons/chem/4g.md")),
    ("chem:4h", include_str!("../lessons/chem/4h.md")),
    ("chem:4i", include_str!("../lessons/chem/4i.md")),
];

/// Physics, Pearson Edexcel International GCSE 4PH1.
const PHYS: &[(&str, &str)] = &[
    ("phys:1a", include_str!("../lessons/phys/1a.md")),
    ("phys:1b", include_str!("../lessons/phys/1b.md")),
    ("phys:1c", include_str!("../lessons/phys/1c.md")),
    ("phys:1d", include_str!("../lessons/phys/1d.md")),
    ("phys:1e", include_str!("../lessons/phys/1e.md")),
    ("phys:1f", include_str!("../lessons/phys/1f.md")),
    ("phys:1g", include_str!("../lessons/phys/1g.md")),
    ("phys:1h", include_str!("../lessons/phys/1h.md")),
    ("phys:2a", include_str!("../lessons/phys/2a.md")),
    ("phys:2b", include_str!("../lessons/phys/2b.md")),
    ("phys:2c", include_str!("../lessons/phys/2c.md")),
    ("phys:2d", include_str!("../lessons/phys/2d.md")),
    ("phys:2e", include_str!("../lessons/phys/2e.md")),
    ("phys:2f", include_str!("../lessons/phys/2f.md")),
    ("phys:3a", include_str!("../lessons/phys/3a.md")),
    ("phys:3b", include_str!("../lessons/phys/3b.md")),
    ("phys:3c", include_str!("../lessons/phys/3c.md")),
    ("phys:3d", include_str!("../lessons/phys/3d.md")),
    ("phys:3e", include_str!("../lessons/phys/3e.md")),
    ("phys:4a", include_str!("../lessons/phys/4a.md")),
    ("phys:4b", include_str!("../lessons/phys/4b.md")),
    ("phys:4c", include_str!("../lessons/phys/4c.md")),
    ("phys:4d", include_str!("../lessons/phys/4d.md")),
    ("phys:4e", include_str!("../lessons/phys/4e.md")),
    ("phys:5a", include_str!("../lessons/phys/5a.md")),
    ("phys:5b", include_str!("../lessons/phys/5b.md")),
    ("phys:5c", include_str!("../lessons/phys/5c.md")),
    ("phys:5d", include_str!("../lessons/phys/5d.md")),
    ("phys:6a", include_str!("../lessons/phys/6a.md")),
    ("phys:6b", include_str!("../lessons/phys/6b.md")),
    ("phys:6c", include_str!("../lessons/phys/6c.md")),
    ("phys:6d", include_str!("../lessons/phys/6d.md")),
    ("phys:7a", include_str!("../lessons/phys/7a.md")),
    ("phys:7b", include_str!("../lessons/phys/7b.md")),
    ("phys:7c", include_str!("../lessons/phys/7c.md")),
    ("phys:7d", include_str!("../lessons/phys/7d.md")),
    ("phys:7e", include_str!("../lessons/phys/7e.md")),
    ("phys:7f", include_str!("../lessons/phys/7f.md")),
    ("phys:8a", include_str!("../lessons/phys/8a.md")),
    ("phys:8b", include_str!("../lessons/phys/8b.md")),
    ("phys:8c", include_str!("../lessons/phys/8c.md")),
];

/// English Literature, AQA GCSE 8702.
const ENGLIT: &[(&str, &str)] = &[
    ("englit:3.1.1a", include_str!("../lessons/englit/3.1.1a.md")),
    ("englit:3.1.1b", include_str!("../lessons/englit/3.1.1b.md")),
    ("englit:3.1.1c", include_str!("../lessons/englit/3.1.1c.md")),
    ("englit:3.1.1d", include_str!("../lessons/englit/3.1.1d.md")),
    ("englit:3.1.1e", include_str!("../lessons/englit/3.1.1e.md")),
    ("englit:3.1.1f", include_str!("../lessons/englit/3.1.1f.md")),
    ("englit:3.1.2a", include_str!("../lessons/englit/3.1.2a.md")),
    ("englit:3.1.2b", include_str!("../lessons/englit/3.1.2b.md")),
    ("englit:3.1.2c", include_str!("../lessons/englit/3.1.2c.md")),
    ("englit:3.1.2d", include_str!("../lessons/englit/3.1.2d.md")),
    ("englit:3.1.2e", include_str!("../lessons/englit/3.1.2e.md")),
    ("englit:3.2.1a", include_str!("../lessons/englit/3.2.1a.md")),
    ("englit:3.2.1b", include_str!("../lessons/englit/3.2.1b.md")),
    ("englit:3.2.1c", include_str!("../lessons/englit/3.2.1c.md")),
    ("englit:3.2.1d", include_str!("../lessons/englit/3.2.1d.md")),
    ("englit:3.2.1e", include_str!("../lessons/englit/3.2.1e.md")),
    ("englit:3.2.2a", include_str!("../lessons/englit/3.2.2a.md")),
    ("englit:3.2.2b", include_str!("../lessons/englit/3.2.2b.md")),
    ("englit:3.2.2c", include_str!("../lessons/englit/3.2.2c.md")),
    ("englit:3.2.2d", include_str!("../lessons/englit/3.2.2d.md")),
    ("englit:3.2.2e", include_str!("../lessons/englit/3.2.2e.md")),
    ("englit:3.2.2f", include_str!("../lessons/englit/3.2.2f.md")),
    ("englit:3.2.2g", include_str!("../lessons/englit/3.2.2g.md")),
    ("englit:3.2.2h", include_str!("../lessons/englit/3.2.2h.md")),
    ("englit:3.2.2i", include_str!("../lessons/englit/3.2.2i.md")),
    ("englit:3.2.2j", include_str!("../lessons/englit/3.2.2j.md")),
    ("englit:3.2.2k", include_str!("../lessons/englit/3.2.2k.md")),
    ("englit:3.2.2l", include_str!("../lessons/englit/3.2.2l.md")),
    ("englit:3.2.2m", include_str!("../lessons/englit/3.2.2m.md")),
    ("englit:3.2.2n", include_str!("../lessons/englit/3.2.2n.md")),
    ("englit:3.2.2o", include_str!("../lessons/englit/3.2.2o.md")),
    ("englit:3.2.2p", include_str!("../lessons/englit/3.2.2p.md")),
    ("englit:3.2.2q", include_str!("../lessons/englit/3.2.2q.md")),
    ("englit:3.2.3a", include_str!("../lessons/englit/3.2.3a.md")),
    ("englit:3.2.3b", include_str!("../lessons/englit/3.2.3b.md")),
    ("englit:3.3a", include_str!("../lessons/englit/3.3a.md")),
    ("englit:3.3b", include_str!("../lessons/englit/3.3b.md")),
];

/// English Language, AQA GCSE 8700.
const ENGLANG: &[(&str, &str)] = &[
    ("englang:1.1a", include_str!("../lessons/englang/1.1a.md")),
    ("englang:1.1b", include_str!("../lessons/englang/1.1b.md")),
    ("englang:1.1c", include_str!("../lessons/englang/1.1c.md")),
    ("englang:1.1d", include_str!("../lessons/englang/1.1d.md")),
    ("englang:1.2a", include_str!("../lessons/englang/1.2a.md")),
    ("englang:1.2b", include_str!("../lessons/englang/1.2b.md")),
    ("englang:1.2c", include_str!("../lessons/englang/1.2c.md")),
    ("englang:2.1a", include_str!("../lessons/englang/2.1a.md")),
    ("englang:2.1b", include_str!("../lessons/englang/2.1b.md")),
    ("englang:2.1c", include_str!("../lessons/englang/2.1c.md")),
    ("englang:2.1d", include_str!("../lessons/englang/2.1d.md")),
    ("englang:2.2a", include_str!("../lessons/englang/2.2a.md")),
    ("englang:2.2b", include_str!("../lessons/englang/2.2b.md")),
    ("englang:2.2c", include_str!("../lessons/englang/2.2c.md")),
    ("englang:3a", include_str!("../lessons/englang/3a.md")),
    ("englang:3b", include_str!("../lessons/englang/3b.md")),
];

/// Every lesson, across subjects.
fn all() -> impl Iterator<Item = &'static (&'static str, &'static str)> {
    FPM.iter().chain(MATHS.iter()).chain(ECON.iter()).chain(CS.iter()).chain(BUS.iter()).chain(BIO.iter()).chain(CHEM.iter()).chain(PHYS.iter()).chain(ENGLIT.iter()).chain(ENGLANG.iter())
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
        for (subject, table) in [("fpm", FPM), ("maths", MATHS), ("econ", ECON), ("cs", CS), ("bus", BUS), ("bio", BIO), ("chem", CHEM), ("phys", PHYS), ("englit", ENGLIT), ("englang", ENGLANG)] {
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
            let tests = blocks(text, "test");
            assert!(tests.is_empty() || tests.len() >= 6, "{id}: a test needs at least six questions ({})", tests.len());
            for (i, q) in qs.iter().chain(tests.iter()).enumerate() {
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

    /// Subjects whose lessons all carry a Key terms block. Every subject is
    /// being brought up to this; add each one as its lessons are done.
    const WITH_TERMS: &[&str] = &["bus", "econ", "cs", "bio", "chem", "phys", "maths"];

    #[test]
    fn every_lesson_has_its_key_terms() {
        for (id, text) in all() {
            let subject = id.split(':').next().unwrap_or("");
            let terms = blocks(text, "terms");
            if !WITH_TERMS.contains(&subject) && terms.is_empty() {
                continue;
            }
            assert_eq!(terms.len(), 1, "{id}: needs exactly one terms block");
            let lines: Vec<&str> = terms[0].lines().filter(|l| !l.trim().is_empty()).collect();
            assert!(lines.len() >= 4, "{id}: fewer than four key terms");
            for l in &lines {
                let ok = l.starts_with("- **")
                    && l.find("**: ").map_or(false, |i| i > 4 && l.len() > i + 12);
                assert!(ok, "{id}: key term not in \"- **term**: definition\" form: {l}");
            }
            let terms_at = text.find("::: terms").unwrap();
            let method_at = text.find("::: method").unwrap();
            assert!(terms_at < method_at, "{id}: key terms should come before the method");
        }
    }
}
