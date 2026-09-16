//! The editable plan configuration.
//!
//! Everything the scheduler needs — subjects, topics, term dates, weekly hours
//! and the links each session sends you to — lives here as owned, serialisable
//! data so it can all be changed from inside the app. `PlanConfig::default()`
//! seeds it from the Edexcel specs hardcoded in `plan.rs`; after that the
//! user's own copy is read from `config.json` in the app data folder.

use serde::{Deserialize, Serialize};

/// Somewhere to go and work: a Save My Exams section, a past-paper archive, a
/// textbook PDF — whatever the session should open.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResourceLink {
    pub label: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TopicCfg {
    pub code: String,
    pub title: String,
    /// Study hours this topic is worth. The scheduler splits it across weeks.
    pub hours: f64,
    /// An exact link for this one topic, pinned once and remembered. Empty
    /// falls back to the subject's own links.
    #[serde(default)]
    pub url: String,
    /// What you must be able to do by the end of the session.
    #[serde(default)]
    pub objectives: Vec<String>,
    /// The mark people drop on this topic.
    #[serde(default)]
    pub watch: String,
    /// Videos pinned to this topic, first to watch first. Empty means the
    /// built-in list applies (see videos.rs); anything here replaces it.
    #[serde(default)]
    pub videos: Vec<VideoCfg>,
}

/// A YouTube video someone has pinned to a topic themselves.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoCfg {
    /// Any YouTube link or a bare video id. Sanitised down to the id.
    pub url: String,
    #[serde(default)]
    pub title: String,
}

/// Weekly hours for one subject, taking effect from `from_week` until the next
/// band starts. Lets a subject ramp up or down over the two years — Further
/// Maths starts at zero, maths eases off once it does.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RateBand {
    pub from_week: u32,
    /// Hours per week during term.
    pub term: f64,
    /// Hours per week during the long summer holiday.
    pub summer: f64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SubjectCfg {
    pub id: String,
    pub name: String,
    pub full: String,
    pub color: String,
    pub papers: String,
    pub spec: String,
    /// Spec areas, cycled through for past-paper practice once content is done.
    pub sections: Vec<String>,
    pub topics: Vec<TopicCfg>,
    pub rates: Vec<RateBand>,
    #[serde(default)]
    pub resources: Vec<ResourceLink>,
    /// `ahead` (default): the app schedules every topic in order, running ahead
    /// of school. `school`: school sets the pace, so the weekly hours go on
    /// closed-book recall of what has been covered rather than on new content,
    /// and topics are ticked as Known when school finishes them.
    #[serde(default)]
    pub pace: String,
    /// Fixed sessions that happen every term week regardless of content, such
    /// as a timed writing piece for English.
    #[serde(default)]
    pub weekly: Vec<WeeklyCfg>,
}

/// A session that recurs every term week, independent of the topic list.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyCfg {
    pub label: String,
    pub hours: f64,
    /// A line of instructions shown on the card.
    #[serde(default)]
    pub note: String,
}

/// One run of consecutive weeks in the calendar.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlockCfg {
    /// First Monday of the block, `YYYY-MM-DD`.
    pub start: String,
    pub weeks: u32,
    /// `term` | `half` | `holiday` | `summer` | `exam`
    pub kind: String,
    pub label: String,
    pub year: u8,
    /// `A1` `A2` `S1` `S2` `U1` `U2` for terms, `H` for breaks, `X` for exams.
    pub block: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlanConfig {
    pub subjects: Vec<SubjectCfg>,
    pub blocks: Vec<BlockCfg>,
    /// Days after something is marked shaky before it comes back, in order.
    pub review_gaps: Vec<u32>,
}

/// Default weekly hours per subject, by id. Reproduces the original schedule:
/// maths front-loaded before Further Maths starts in week 8, then easing off in
/// Year 11 as Further Maths takes over.
const DEFAULT_RATES: &[(&str, &[(u32, f64, f64)])] = &[
    // School-paced subjects: these hours are closed-book recall on what school
    // has covered, not new content.
    ("maths", &[(1, 1.0, 0.5)]),
    ("fpm", &[(1, 0.5, 0.5)]),
    // Running ahead: Economics and Business are the two lightest courses and
    // the user has only just started them, so getting ahead turns school
    // lessons into revision.
    ("bus", &[(1, 1.5, 0.5)]),
    ("econ", &[(1, 1.5, 0.75)]),
    ("cs", &[(1, 1.0, 1.0)]),
    // English: recall on the texts and poems, plus a fixed timed piece a week.
    ("englit", &[(1, 0.5, 0.5)]),
    ("englang", &[(1, 0.0, 0.0)]),
    // Separate sciences, taught across both years at school. Recall only.
    ("bio", &[(1, 0.75, 0.5)]),
    ("chem", &[(1, 0.75, 0.5)]),
    ("phys", &[(1, 0.75, 0.5)]),
];

/// Which subjects the app runs ahead in, and which school paces. Decided with
/// the user on 14 September 2026: Maths is already ahead of school and needs
/// protecting rather than extending; the sciences and English are taught
/// through both years and are better served by recall than by front-loading.
const DEFAULT_PACE: &[(&str, &str)] = &[
    ("maths", "school"), ("fpm", "school"),
    ("bus", "ahead"), ("econ", "ahead"), ("cs", "ahead"),
    ("englit", "school"), ("englang", "school"),
    ("bio", "school"), ("chem", "school"), ("phys", "school"),
];

/// Fixed weekly sessions. Writing under exam timing is a separate skill from
/// knowing the texts, and the writing questions need no text knowledge at all,
/// so the practice can start from week one.
const DEFAULT_WEEKLY: &[(&str, &[(&str, f64, &str)])] = &[
    ("englang", &[("Timed writing piece", 1.0,
        "45 minutes, handwritten, no stopping: Paper 1 Q5 (creative) one week, Paper 2 Q5 (non-fiction) the next. Mark it against the mark scheme tomorrow, cold - the marking is where the learning is.")]),
];

fn default_pace_for(id: &str) -> &'static str {
    DEFAULT_PACE.iter().find(|(k, _)| *k == id).map(|(_, v)| *v).unwrap_or("ahead")
}

fn default_weekly_for(id: &str) -> Vec<WeeklyCfg> {
    lookup(DEFAULT_WEEKLY, id).iter()
        .map(|(label, hours, note)| WeeklyCfg { label: label.to_string(), hours: *hours, note: note.to_string() })
        .collect()
}

/// Where each subject's sessions send you.
///
/// Save My Exams needs your own account — those are deep links into the right
/// section of it. The rest are free. Every address here was checked to respond,
/// and the ones that are not obviously named were opened to confirm they hold
/// the right subject: a Physics & Maths Tutor Business URL returned a healthy
/// page that turned out to be English Language, so it is deliberately absent.
///
/// The first three appear on the session card; the rest sit behind "more".
/// All of them are editable in the Plan tab.
const DEFAULT_LINKS: &[(&str, &[(&str, &str)])] = &[
    ("maths", &[
        ("Revision notes", "https://www.savemyexams.com/igcse/maths/edexcel/a/18/higher/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/igcse/maths/edexcel/a/18/higher/topic-questions/"),
        ("Maths Genie", "https://mathsgenie.co.uk/igcse/maths/edexcel"),
        ("PMT by topic", "https://www.physicsandmathstutor.com/maths-revision/gcse-questions-edexcel-igcse/"),
        ("PMT past papers", "https://www.physicsandmathstutor.com/past-papers/gcse-maths/edexcel-igcse-a-paper-1/"),
        ("Dr Frost (self-marking)", "https://www.drfrost.org/revision/papers/igcse"),
        ("Corbettmaths", "https://corbettmaths.com/"),
        ("Pearson papers & reports", "https://qualifications.pearson.com/en/qualifications/edexcel-international-gcses/international-gcse-mathematics-a-2016.html"),
    ]),
    ("fpm", &[
        ("Revision notes", "https://www.savemyexams.com/igcse/further-maths/edexcel/19/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/igcse/further-maths/edexcel/19/topic-questions/"),
        // The best Further Pure resource there is: papers back to 2011, free.
        ("PMT past papers", "https://www.physicsandmathstutor.com/past-papers/gcse-maths/edexcel-igcse-further-paper-1/"),
        ("Pearson papers & reports", "https://qualifications.pearson.com/en/qualifications/edexcel-international-gcses/international-gcse-further-pure-mathematics-2017.html"),
    ]),
    // AQA GCSE Business 8132. Every link opened and checked against its content -
    // the PMT "gcse-business/aqa-paper-1" URL returns 200 but serves A-level
    // Chemistry, so it is deliberately not listed here.
    ("bus", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/business/aqa/17/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/business/aqa/17/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/business/aqa/past-papers/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/business/gcse/business-8132/assessment-resources"),
    ]),
    // Cambridge IGCSE (9-1) Economics 0987. On Save My Exams choose the
    // "First exams 2027" course, not "Last exams 2026" - the 2027-2029 syllabus
    // is the one that applies to a 2028 sitting.
    ("econ", &[
        ("Revision notes (pick First exams 2027)", "https://www.savemyexams.com/igcse/economics/cie/25/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/igcse/economics/cie/25/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/igcse/economics/cie/past-papers/"),
        ("Cambridge syllabus & papers", "https://www.cambridgeinternational.org/programmes-and-qualifications/cambridge-igcse-economics-9-1-0987/"),
    ]),
    // OCR GCSE Computer Science J277 - opened and checked against content.
    ("cs", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/computer-science/ocr/22/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/computer-science/ocr/22/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/computer-science/ocr/past-papers/"),
        ("OCR papers & mark schemes", "https://www.ocr.org.uk/qualifications/gcse/computer-science-j277-from-2020/assessment/"),
    ]),
    // Both Englishes are AQA GCSE, not Edexcel IGCSE - the Power and Conflict
    // cluster and the Spoken Language endorsement are what identify the board.
    ("englit", &[
        ("AQA spec", "https://www.aqa.org.uk/subjects/english/gcse/english-8702/specification"),
        ("AQA papers & reports", "https://www.aqa.org.uk/subjects/english/gcse/english-8702/assessment-resources"),
    ]),
    ("englang", &[
        ("AQA spec", "https://www.aqa.org.uk/subjects/english/gcse/english-8700/specification"),
        ("AQA papers & reports", "https://www.aqa.org.uk/subjects/english/gcse/english-8700/assessment-resources"),
    ]),
    ("bio", &[
        ("Revision notes", "https://www.savemyexams.com/igcse/biology/edexcel/19/revision-notes/"),
        ("PMT past papers", "https://www.physicsandmathstutor.com/past-papers/gcse-biology/edexcel-igcse-paper-1/"),
        ("Pearson papers & reports", "https://qualifications.pearson.com/en/qualifications/edexcel-international-gcses/international-gcse-biology-2017.html"),
    ]),
    ("chem", &[
        ("Revision notes", "https://www.savemyexams.com/igcse/chemistry/edexcel/19/revision-notes/"),
        ("PMT past papers", "https://www.physicsandmathstutor.com/past-papers/gcse-chemistry/edexcel-igcse-paper-1/"),
        ("Pearson papers & reports", "https://qualifications.pearson.com/en/qualifications/edexcel-international-gcses/international-gcse-chemistry-2017.html"),
    ]),
    ("phys", &[
        ("Revision notes", "https://www.savemyexams.com/igcse/physics/edexcel/19/revision-notes/"),
        ("PMT past papers", "https://www.physicsandmathstutor.com/past-papers/gcse-physics/edexcel-igcse-paper-1/"),
        ("Pearson papers & reports", "https://qualifications.pearson.com/en/qualifications/edexcel-international-gcses/international-gcse-physics-2017.html"),
    ]),
];

fn lookup<'a, T>(table: &'a [(&str, &'a [T])], id: &str) -> &'a [T] {
    table.iter().find(|(k, _)| *k == id).map(|(_, v)| *v).unwrap_or(&[])
}

fn default_rates_for(id: &str) -> Vec<RateBand> {
    let bands = lookup(DEFAULT_RATES, id);
    if bands.is_empty() {
        return vec![RateBand { from_week: 1, term: 1.0, summer: 0.5 }];
    }
    bands.iter().map(|(from_week, term, summer)| RateBand { from_week: *from_week, term: *term, summer: *summer }).collect()
}

fn default_links_for(id: &str) -> Vec<ResourceLink> {
    lookup(DEFAULT_LINKS, id).iter().map(|(label, url)| ResourceLink { label: label.to_string(), url: url.to_string() }).collect()
}

impl Default for PlanConfig {
    fn default() -> Self {
        PlanConfig {
            subjects: crate::plan::SUBJECTS
                .iter()
                .map(|d| SubjectCfg {
                    id: d.id.into(),
                    name: d.name.into(),
                    full: d.full.into(),
                    color: d.color.into(),
                    papers: d.papers.into(),
                    spec: d.spec.into(),
                    sections: d.sections.iter().map(|s| s.to_string()).collect(),
                    topics: d.topics.iter().map(|(code, title, hours)| {
                        let id = format!("{}:{}", d.id, code);
                        TopicCfg {
                            code: code.to_string(), title: title.to_string(), hours: *hours, url: String::new(),
                            objectives: crate::course::objectives_for(&id).iter().map(|s| s.to_string()).collect(),
                            watch: crate::course::watch_for(&id).to_string(),
                            videos: Vec::new(),
                        }
                    }).collect(),
                    rates: default_rates_for(d.id),
                    resources: default_links_for(d.id),
                    pace: default_pace_for(d.id).into(),
                    weekly: default_weekly_for(d.id),
                })
                .collect(),
            blocks: crate::plan::BLOCKS
                .iter()
                .map(|(start, weeks, kind, label, year, block)| BlockCfg {
                    start: start.to_string(),
                    weeks: *weeks,
                    kind: kind.to_string(),
                    label: label.to_string(),
                    year: *year,
                    block: block.to_string(),
                })
                .collect(),
            review_gaps: vec![7, 21, 56],
        }
    }
}

impl PlanConfig {
    /// Weekly hours for a subject in a given week, honouring the rate bands.
    pub fn hours_for(&self, subject: &SubjectCfg, week_n: u32, kind: &str) -> f64 {
        match subject.rates.iter().filter(|b| b.from_week <= week_n).max_by_key(|b| b.from_week) {
            Some(b) => match kind {
                "term" => b.term,
                "summer" => b.summer,
                _ => 0.0,
            },
            None => 0.0,
        }
    }

    /// Repair anything that could have been broken in the editor, so a bad edit
    /// degrades gracefully instead of producing an unusable plan.
    pub fn sanitise(&mut self) {
        self.subjects.retain(|s| !s.id.trim().is_empty());
        for s in &mut self.subjects {
            s.topics.retain(|t| !t.title.trim().is_empty());
            for t in &mut s.topics {
                t.hours = t.hours.clamp(0.5, 40.0);
                t.url = sane_url(&t.url);
                t.objectives.retain(|o| !o.trim().is_empty());
                // Keep only real YouTube videos, stored as the bare id.
                t.videos.retain_mut(|v| match crate::videos::video_id(&v.url) {
                    Some(id) => { v.url = id; v.title = v.title.trim().to_string(); true }
                    None => false,
                });
            }
            if s.sections.is_empty() {
                s.sections.push("Past papers".into());
            }
            s.rates.retain(|b| b.from_week >= 1);
            for b in &mut s.rates {
                b.term = b.term.clamp(0.0, 40.0);
                b.summer = b.summer.clamp(0.0, 40.0);
            }
            if s.rates.is_empty() {
                s.rates.push(RateBand { from_week: 1, term: 1.0, summer: 0.5 });
            }
            s.rates.sort_by_key(|b| b.from_week);
            if s.pace != "school" { s.pace = "ahead".into(); }
            for wk in &mut s.weekly { wk.hours = wk.hours.clamp(0.25, 10.0); }
            s.weekly.retain(|wk| !wk.label.trim().is_empty());
            for r in &mut s.resources {
                r.url = sane_url(&r.url);
            }
            s.resources.retain(|r| !r.url.is_empty() && !r.label.trim().is_empty());
        }
        self.blocks.retain(|b| b.weeks > 0 && chrono::NaiveDate::parse_from_str(&b.start, "%Y-%m-%d").is_ok());
        if self.blocks.is_empty() {
            self.blocks = PlanConfig::default().blocks;
        }
        self.blocks.sort_by(|a, b| a.start.cmp(&b.start));
        if self.review_gaps.is_empty() {
            self.review_gaps = vec![7, 21, 56];
        }
    }
}

/// Only http(s) links are kept. Anything else is dropped rather than handed to
/// the system browser.
fn sane_url(raw: &str) -> String {
    let t = raw.trim();
    if t.starts_with("https://") || t.starts_with("http://") { t.to_string() } else { String::new() }
}
