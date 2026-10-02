//! The editable plan configuration.
//!
//! Everything the scheduler needs — subjects, topics, term dates, weekly hours
//! and the links each session sends you to — lives here as owned, serialisable
//! data so it can all be changed from inside the app. `PlanConfig::default()`
//! is the app author's own plan (the starter ten and their school's term
//! dates), kept so profiles created before the app was shared keep it.
//! Everyone else starts from `PlanConfig::blank()`: no subjects, and a
//! standard school calendar they then set up for themselves. After that the
//! user's own copy is read from `config.json` in their profile folder.

use chrono::{Datelike, Duration, NaiveDate};
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
    // Languages: taught through both years at school, so recall and vocabulary
    // rather than front-loading.
    ("fre", &[(1, 0.75, 0.5)]),
    ("spa", &[(1, 0.75, 0.5)]),
    ("ger", &[(1, 0.75, 0.5)]),
    // AQA GCSE Geography (8035)
    ("geog", &[(1, 1.0, 0.5)]),
    // Pearson Edexcel GCSE History (1HI0)
    ("hist", &[(1, 1.0, 0.5)]),
    // Music: one listening paper, taught through both years at school. Recall
    // of terms and set works rather than front-loading.
    ("music", &[(1, 0.5, 0.25)]),
    // AQA GCSE Religious Studies A (8062)
    ("rs", &[(1, 0.75, 0.5)]),
    // Drama: one written paper, taught through both years at school.
    ("drama", &[(1, 0.5, 0.25)]),
    // PE: taught through both years at school, with the practical NEA done
    // there too - recall on the written theory only.
    ("pe", &[(1, 0.5, 0.25)]),
    // Media Studies: taught across both years at school; recall on the set products.
    ("media", &[(1, 0.5, 0.25)]),
    ("dt", &[(1, 0.5, 0.25)]),
    // Food: taught through both years at school, and the NEA takes the lesson
    // time in Year 11, so recall on the written-paper content only.
    ("food", &[(1, 0.5, 0.25)]),
    // Pearson Edexcel GCSE Mathematics (1MA1) Higher
    ("maths_edx", &[(1, 1.0, 0.5)]),
    // AQA GCSE Mathematics (8300) Higher
    ("maths_aqa", &[(1, 1.0, 0.5)]),
    // OCR GCSE Mathematics (J560) Higher
    ("maths_ocr", &[(1, 1.0, 0.5)]),
    // AQA GCSE Biology (8461) Higher
    ("bio_aqa", &[(1, 1.0, 0.5)]),
    // English Language (Eduqas): taught through both years at school. Modest
    // recall; the writing practice belongs in a weekly timed piece (see (h)).
    ("englang_edq", &[(1, 0.0, 0.0)]),
    // AQA GCSE Chemistry (8462) Higher
    ("chem_aqa", &[(1, 1.0, 0.5)]),
    // AQA GCSE Physics (8463) Higher
    ("phys_aqa", &[(1, 1.0, 0.5)]),
    // AQA GCSE Economics 8136: second-board Economics, school-paced - recall on
    // what school has covered, with no NEA.
    ("econ_aqa", &[(1, 0.5, 0.25)]),
    // English Literature (Eduqas): as englit - recall on the texts and poems.
    ("englit_edq", &[(1, 0.5, 0.5)]),
    // Business (Edexcel): two written papers, taught through both years at school.
    ("bus_edx", &[(1, 0.5, 0.25)]),
    // Computer Science (AQA 8525): taught at school across both years, so
    // recall only, at a modest rate.
    ("cs_aqa", &[(1, 0.5, 0.25)]),
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
    ("fre", "school"), ("spa", "school"), ("ger", "school"),
    ("geog", "ahead"),
    ("hist", "ahead"),
    ("music", "school"),
    ("rs", "ahead"),
    ("drama", "school"),
    ("pe", "school"),
    ("media", "school"),
    ("dt", "school"),
    ("food", "school"),
    ("maths_edx", "ahead"),
    ("maths_aqa", "ahead"),
    ("maths_ocr", "ahead"),
    ("bio_aqa", "ahead"),
    ("englang_edq", "school"),
    ("chem_aqa", "ahead"),
    ("phys_aqa", "ahead"),
    ("econ_aqa", "school"),
    ("englit_edq", "school"),
    ("bus_edx", "school"),
    ("cs_aqa", "school"),
];

/// Fixed weekly sessions. Writing under exam timing is a separate skill from
/// knowing the texts, and the writing questions need no text knowledge at all,
/// so the practice can start from week one.
const DEFAULT_WEEKLY: &[(&str, &[(&str, f64, &str)])] = &[
    ("englang", &[("Timed writing piece", 1.0,
        "45 minutes, handwritten, no stopping: Paper 1 Q5 (creative) one week, Paper 2 Q5 (non-fiction) the next. Mark it against the mark scheme tomorrow, cold - the marking is where the learning is.")]),
    ("fre", &[("Vocabulary and speaking", 0.5,
        "Two 15-minute bursts: learn 20 words from the AQA French vocabulary list (Appendix 2 of the specification) with the French side covered, then say three sentences aloud using them - one in the past, one in the present, one in the future.")]),
    ("spa", &[("Vocabulary and speaking", 0.5,
        "Two 15-minute bursts: learn 20 words from the AQA Spanish vocabulary list (Appendix 2 of the specification) with the Spanish side covered, then say three sentences aloud using them - one in the past, one in the present, one in the future.")]),
    ("ger", &[("Vocabulary and speaking", 0.5,
        "Two 15-minute bursts: learn 20 words from the AQA German vocabulary list (Appendix 2 of the specification) with the German side covered, then say three sentences aloud using them - one in the past, one in the present, one in the future.")]),
    ("englang_edq", &[("Timed writing piece", 1.0,
        "Handwritten, no stopping: one week a Component 1 Section B story (45 minutes, 450-600 words, from a choice of four titles), the next week both Component 2 Section B tasks (30 minutes each, 300-400 words). Mark it against the mark scheme tomorrow, cold - the marking is where the learning is.")]),
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
    // AQA GCSE languages (2024 specifications). Every link opened and checked:
    // the Save My Exams pages are its 2024-spec AQA course for that language.
    ("fre", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/french/aqa/24/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/french/aqa/24/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/french/aqa/past-papers/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/french/gcse/french-8652/assessment-resources"),
        ("AQA spec (vocabulary list is Appendix 2)", "https://www.aqa.org.uk/subjects/french/gcse/french-8652/specification"),
    ]),
    ("spa", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/spanish/aqa/24/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/spanish/aqa/24/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/spanish/aqa/past-papers/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/spanish/gcse/spanish-8692/assessment-resources"),
        ("AQA spec (vocabulary list is Appendix 2)", "https://www.aqa.org.uk/subjects/spanish/gcse/spanish-8692/specification"),
    ]),
    ("ger", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/german/aqa/24/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/german/aqa/24/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/german/aqa/past-papers/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/german/gcse/german-8662/assessment-resources"),
        ("AQA spec (vocabulary list is Appendix 2)", "https://www.aqa.org.uk/subjects/german/gcse/german-8662/specification"),
    ]),
    // AQA GCSE Geography (8035). Each link opened and checked 29 September 2026
    ("geog", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/geography/aqa/18/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/geography/aqa/18/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/geography/aqa/past-papers/"),
        ("Internet Geography", "https://www.internetgeography.net/aqa-gcse-geography/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/geography/gcse/geography-8035/assessment-resources"),
    ]),
    // Pearson Edexcel GCSE History (1HI0). Each link opened and checked 29 September 2026
    ("hist", &[
        ("Paper 1 notes", "https://www.savemyexams.com/gcse/history/edexcel/24/the-thematic-historic-environment-paper-1/revision-notes/"),
        ("Paper 2 Cold War notes", "https://www.savemyexams.com/gcse/history/edexcel/24/period-study-paper-2-booklet-p/revision-notes/"),
        ("Paper 2 Elizabeth notes", "https://www.savemyexams.com/gcse/history/edexcel/24/british-depth-study-paper-2-booklet-b/revision-notes/"),
        ("Paper 3 notes", "https://www.savemyexams.com/gcse/history/edexcel/24/modern-depth-study-paper-3/revision-notes/"),
        ("Past papers", "https://www.savemyexams.com/gcse/history/edexcel/past-papers/"),
        ("Pearson papers & mark schemes", "https://qualifications.pearson.com/en/qualifications/edexcel-gcses/history-2016.coursematerials.html"),
    ]),
    // WJEC Eduqas GCSE Music C660QS. Every link opened and checked: Bitesize's
    // Eduqas examspec, Eduqas's own knowledge organisers and set-work packs, and
    // the Component 3 papers (question papers and mark schemes; the exam audio
    // is only released to centres).
    ("music", &[
        ("BBC Bitesize (Eduqas)", "https://www.bbc.co.uk/bitesize/examspecs/zbmct39"),
        ("Eduqas knowledge organisers", "https://resources.eduqas.co.uk/Pages/ResourceSingle.aspx?rIid=1508"),
        ("Component 3 past papers", "https://www.savemyexams.com/gcse/music/wjec-eduqas/past-papers/component-3/"),
        ("Set work: Badinerie notes & score", "https://resources.eduqas.co.uk/Pages/ResourceSingle.aspx?rIid=1444"),
        ("Set work: Africa notes & score", "https://resources.eduqas.co.uk/Pages/ResourceSingle.aspx?rIid=1445"),
    ]),
    // AQA GCSE Religious Studies A (8062). Each link opened and checked 29 September 2026
    ("rs", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/religious-studies/aqa/a/18/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/religious-studies/aqa/a/18/topic-questions/"),
        ("Mock exams", "https://www.savemyexams.com/gcse/religious-studies/aqa/a/18/mock-exams/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/religious-studies/gcse/religious-studies-a-8062/assessment-resources"),
    ]),
    // AQA GCSE Drama 8261 - opened and checked against content.
    ("drama", &[
        ("BBC Bitesize (AQA)", "https://www.bbc.co.uk/bitesize/examspecs/zrnjwty"),
        ("Past papers", "https://www.savemyexams.com/gcse/drama/aqa/past-papers/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/drama/gcse/drama-8261/assessment-resources"),
        ("AQA spec (set plays are 3.1.2)", "https://www.aqa.org.uk/subjects/drama/gcse/drama-8261/specification"),
    ]),
    // AQA GCSE Physical Education 8582. Every link opened on 29 September 2026:
    // the Save My Exams pages are its AQA GCSE PE course (spec 8582), and the
    // BBC Bitesize page is its AQA-specific PE exam spec.
    ("pe", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/physical-education/aqa/16/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/physical-education/aqa/16/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/physical-education/aqa/past-papers/"),
        ("BBC Bitesize (AQA)", "https://www.bbc.co.uk/bitesize/examspecs/zp49cwx"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/physical-education/gcse/physical-education-8582/assessment-resources"),
    ]),
    // WJEC Eduqas GCSE Media Studies (C680QS). Every link opened and checked on
    // 29 September 2026: the Save My Exams and Seneca courses are their Eduqas
    // GCSE Media Studies notes (both list Desert Island Discs and Trigger Point).
    ("media", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/media-studies/wjec-eduqas/17/revision-notes/"),
        ("Seneca revision notes", "https://senecalearning.com/en-GB/revision-notes/gcse/media-studies/eduqas"),
        ("Eduqas set product factsheets (Component 1)", "https://resources.eduqas.co.uk/Pages/ResourceSingle.aspx?rIid=1885"),
        ("Eduqas set product factsheets (Component 2)", "https://resources.eduqas.co.uk/Pages/ResourceSingle.aspx?rIid=2072"),
        ("Eduqas spec, papers & mark schemes", "https://www.eduqas.co.uk/qualifications/media-studies-gcse/#tab_pastpapers"),
    ]),
    // AQA GCSE Design and Technology 8552. Every link opened on 29 September
    // 2026 and checked: the Save My Exams pages are its AQA 8552 course
    // (core, specialist, designing and making), Bitesize is its AQA 9-1 course.
    ("dt", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/design-and-technology/aqa/17/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/design-and-technology/aqa/17/topic-questions/"),
        ("BBC Bitesize (AQA)", "https://www.bbc.co.uk/bitesize/examspecs/zby2bdm"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/design-and-technology/gcse/design-and-technology-8552/assessment-resources"),
        ("AQA specification", "https://www.aqa.org.uk/subjects/design-and-technology/gcse/design-and-technology-8552/specification"),
    ]),
    // AQA GCSE Food Preparation and Nutrition 8585. Every link opened and checked
    // on 29 September 2026: the Save My Exams pages are its AQA 8585 course
    // (sections match the spec's 3.1-3.6, past papers are 8585/W). BBC Bitesize's
    // GCSE food page is CCEA Home Economics, not AQA, so it is not listed.
    ("food", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/food-and-nutrition/aqa/food-preparation-and-nutrition/16/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/food-and-nutrition/aqa/food-preparation-and-nutrition/16/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/food-and-nutrition/aqa/food-preparation-and-nutrition/past-papers/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/food-preparation-and-nutrition/gcse/food-preparation-and-nutrition-8585/assessment-resources"),
        ("AQA spec", "https://www.aqa.org.uk/subjects/food-preparation-and-nutrition/gcse/food-preparation-and-nutrition-8585/specification"),
    ]),
    // Pearson Edexcel GCSE Mathematics (1MA1) Higher. Each link opened and checked 30 September 2026
    ("maths_edx", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/maths/edexcel/22/higher/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/maths/edexcel/22/higher/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/maths/edexcel/past-papers/"),
        ("Maths Genie", "https://www.mathsgenie.co.uk/gcse.php"),
        ("Corbettmaths", "https://corbettmaths.com/contents/"),
        ("Pearson papers & mark schemes", "https://qualifications.pearson.com/en/qualifications/edexcel-gcses/mathematics-2015.coursematerials.html"),
    ]),
    // AQA GCSE Mathematics (8300) Higher. Each link opened and checked 30 September 2026
    ("maths_aqa", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/maths/aqa/22/higher/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/maths/aqa/22/higher/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/maths/aqa/past-papers/"),
        ("Maths Genie", "https://www.mathsgenie.co.uk/gcse.php"),
        ("Corbettmaths", "https://corbettmaths.com/contents/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/mathematics/gcse/mathematics-8300/assessment-resources"),
    ]),
    // OCR GCSE Mathematics (J560) Higher. Each link opened and checked 30 September 2026
    ("maths_ocr", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/maths/ocr/22/higher/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/maths/ocr/22/higher/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/maths/ocr/past-papers/"),
        ("Maths Genie", "https://www.mathsgenie.co.uk/gcse.php"),
        ("Corbettmaths", "https://corbettmaths.com/contents/"),
        ("OCR papers & mark schemes", "https://www.ocr.org.uk/qualifications/gcse/mathematics-j560-from-2015/assessment/"),
    ]),
    // AQA GCSE Biology (8461) Higher. Each link opened and checked 1 October 2026
    ("bio_aqa", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/biology/aqa/18/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/biology/aqa/18/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/biology/aqa/past-papers/"),
        ("PMT by topic", "https://www.physicsandmathstutor.com/biology-revision/gcse-aqa/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/biology/gcse/biology-8461/assessment-resources"),
    ]),
    // English Language - WJEC Eduqas GCSE C700QS. All links opened and checked.
    ("englang_edq", &[
        ("Eduqas spec & papers", "https://www.eduqas.co.uk/qualifications/english-language-gcse/"),
        ("BBC Bitesize (Eduqas)", "https://www.bbc.co.uk/bitesize/examspecs/zpxh82p"),
        ("Revision notes", "https://www.savemyexams.com/gcse/english-language/wjec-eduqas/"),
        ("Past papers", "https://www.savemyexams.com/gcse/english-language/wjec-eduqas/past-papers/"),
        ("PMT past papers", "https://www.physicsandmathstutor.com/past-papers/gcse-english-language/eduqas-component-1/"),
    ]),
    // AQA GCSE Chemistry (8462) Higher. Each link opened and checked 1 October 2026
    ("chem_aqa", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/chemistry/aqa/18/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/chemistry/aqa/18/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/chemistry/aqa/past-papers/"),
        ("PMT by topic", "https://www.physicsandmathstutor.com/chemistry-revision/gcse-aqa/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/chemistry/gcse/chemistry-8462/assessment-resources"),
    ]),
    // AQA GCSE Physics (8463) Higher. Each link opened and checked 1 October 2026
    ("phys_aqa", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/physics/aqa/18/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/physics/aqa/18/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/physics/aqa/past-papers/"),
        ("PMT by topic", "https://www.physicsandmathstutor.com/physics-revision/gcse-aqa/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/physics/gcse/physics-8463/assessment-resources"),
    ]),
    // AQA GCSE Economics 8136. Every link opened on 30 September 2026: the
    // tutor2u page is its AQA GCSE Economics student hub (study notes, quizzes,
    // topic videos); Save My Exams has 8136 past papers but no revision notes
    // for this course; BBC Bitesize has no GCSE Economics.
    ("econ_aqa", &[
        ("tutor2u notes & videos (AQA)", "https://www.tutor2u.net/students/gcse/aqa-gcse-economics"),
        ("Past papers", "https://www.savemyexams.com/gcse/economics/aqa/past-papers/"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/economics/gcse/economics-8136/assessment-resources"),
        ("AQA spec", "https://www.aqa.org.uk/subjects/economics/gcse/economics-8136/specification"),
    ]),
    // WJEC Eduqas GCSE English Literature C720QS. Every link opened and checked:
    // the Eduqas page (specification, examiners' reports, grade boundaries), its
    // past-papers tab, the 2027 anthology itself, Eduqas's free blended-learning
    // resources and knowledge organisers for all 15 new poems, and Bitesize's
    // Eduqas English Literature course (Macbeth, An Inspector Calls, A Christmas Carol).
    ("englit_edq", &[
        ("Eduqas spec & key documents", "https://www.eduqas.co.uk/qualifications/english-literature-gcse/"),
        ("Eduqas past papers & mark schemes", "https://www.eduqas.co.uk/qualifications/english-literature-gcse/#tab_pastpapers"),
        ("Poetry anthology (from 2027)", "https://www.eduqas.co.uk/media/zd1b4ii5/new-poetry-anthology-for-first-examination.pdf"),
        ("Anthology poems: Eduqas resources", "https://resources.eduqas.co.uk/Pages/ResourceSingle.aspx?rIid=2197"),
        ("BBC Bitesize (Eduqas)", "https://www.bbc.co.uk/bitesize/examspecs/zw9mycw"),
    ]),
    // Pearson Edexcel GCSE Business 1BS0 - opened and checked against content.
    ("bus_edx", &[
        ("BBC Bitesize (Edexcel)", "https://www.bbc.co.uk/bitesize/examspecs/z98snbk"),
        ("Revision notes", "https://www.savemyexams.com/gcse/business/edexcel/19/revision-notes/"),
        ("Past papers", "https://www.savemyexams.com/gcse/business/edexcel/past-papers/"),
        ("Pearson papers & mark schemes", "https://qualifications.pearson.com/en/qualifications/edexcel-gcses/business-2017.coursematerials.html#filterQuery=category:Pearson-UK:Category%2FExam-materials"),
    ]),
    // AQA GCSE Computer Science 8525. Every link opened and checked on 30
    // September 2026. The Save My Exams course is its AQA 8525 course: notes and
    // questions sorted by spec section, and the past papers are 8525/1A-1C and
    // 8525/2. Its notes still follow the 2020 spec, so a few pages (topologies,
    // Ethernet/Wi-Fi, UDP, FTP, optical storage) go beyond the 2027 exams. The
    // BBC Bitesize pages are its AQA computer science exam spec.
    ("cs_aqa", &[
        ("Revision notes", "https://www.savemyexams.com/gcse/computer-science/aqa/20/revision-notes/"),
        ("Topic questions", "https://www.savemyexams.com/gcse/computer-science/aqa/20/topic-questions/"),
        ("Past papers", "https://www.savemyexams.com/gcse/computer-science/aqa/past-papers/"),
        ("BBC Bitesize", "https://www.bbc.co.uk/bitesize/examspecs/zkwsjhv"),
        ("AQA papers & mark schemes", "https://www.aqa.org.uk/subjects/computer-science/gcse/computer-science-8525/assessment-resources"),
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

/// One built-in subject as an editable configuration.
fn subject_cfg(d: &crate::plan::SubjectDef) -> SubjectCfg {
    SubjectCfg {
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
    }
}

/// Every built-in subject, ready to add to a plan: the starter ten plus the
/// catalog of fully written subjects that start switched off.
pub fn catalog() -> Vec<SubjectCfg> {
    crate::plan::SUBJECTS.iter().map(subject_cfg).collect()
}

impl Default for PlanConfig {
    fn default() -> Self {
        PlanConfig {
            subjects: crate::plan::SUBJECTS
                .iter()
                .filter(|d| crate::plan::STARTER.contains(&d.id))
                .map(subject_cfg)
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

fn monday_of(d: NaiveDate) -> NaiveDate {
    d - Duration::days(d.weekday().num_days_from_monday() as i64)
}

fn monday_on_or_after(d: NaiveDate) -> NaiveDate {
    d + Duration::days((7 - d.weekday().num_days_from_monday() as i64) % 7)
}

fn ymd(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).expect("valid date")
}

fn last_monday_of(y: i32, m: u32) -> NaiveDate {
    let next = if m == 12 { ymd(y + 1, 1, 1) } else { ymd(y, m + 1, 1) };
    monday_of(next - Duration::days(1))
}

/// Easter Sunday in the Gregorian calendar (the anonymous algorithm).
fn easter(y: i32) -> NaiveDate {
    let a = y % 19;
    let b = y / 100;
    let c = y % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = (h + l - 7 * m + 114) % 31 + 1;
    ymd(y, month as u32, day as u32)
}

/// The summer someone starting a two-year GCSE course today sits their exams:
/// the summer after next, counting from the school year that `today` is in.
pub fn default_exam_year(today: NaiveDate) -> i32 {
    let school_year = if today.month() >= 8 { today.year() } else { today.year() - 1 };
    school_year + 2
}

/// A typical English school calendar from the week of `from` to the GCSE exam
/// season in the summer of `exam_year`: six half-terms a year with the usual
/// holidays, Easter from the real date, and exams from the second Monday in
/// May. Schools differ by a week here and there, so it is a starting point to
/// edit, not a promise. Empty if the exams are already over.
pub fn school_calendar(from: NaiveDate, exam_year: i32) -> Vec<BlockCfg> {
    let from = monday_of(from);
    let first = if from.month() >= 8 { from.year() } else { from.year() - 1 };
    let mut out: Vec<BlockCfg> = Vec::new();
    let mut push = |start: NaiveDate, end: NaiveDate, kind: &str, label: &str, year: u8, block: &str| {
        let weeks = (end - start).num_days() / 7;
        if weeks > 0 {
            out.push(BlockCfg { start: start.format("%Y-%m-%d").to_string(), weeks: weeks as u32, kind: kind.into(), label: label.into(), year, block: block.into() });
        }
    };
    for y in first..exam_year {
        let year = (11 - (exam_year - 1 - y)).clamp(7, 11) as u8;
        let a1 = monday_on_or_after(ymd(y, 9, 2));
        let oct = last_monday_of(y, 10);
        let xmas = monday_of(ymd(y, 12, 21));
        let s1 = xmas + Duration::weeks(2);
        let feb = monday_of(ymd(y + 1, 2, 15));
        let easter_hol = monday_of(easter(y + 1));
        let u1 = easter_hol + Duration::weeks(2);
        push(a1, oct, "term", "Autumn 1", year, "A1");
        push(oct, oct + Duration::weeks(1), "half", "October half-term", year, "H");
        push(oct + Duration::weeks(1), xmas, "term", "Autumn 2", year, "A2");
        push(xmas, s1, "holiday", "Christmas holiday", year, "H");
        push(s1, feb, "term", "Spring 1", year, "S1");
        push(feb, feb + Duration::weeks(1), "half", "February half-term", year, "H");
        push(feb + Duration::weeks(1), easter_hol, "term", "Spring 2", year, "S2");
        push(easter_hol, u1, "holiday", "Easter holiday", year, "H");
        if y + 1 == exam_year {
            let exams = monday_on_or_after(ymd(y + 1, 5, 1)) + Duration::weeks(1);
            push(u1, exams, "term", "Summer 1", year, "U1");
            push(exams, exams + Duration::weeks(7), "exam", "Exam season", year, "X");
        } else {
            let may = last_monday_of(y + 1, 5);
            let summer = monday_of(ymd(y + 1, 7, 20));
            let next = monday_on_or_after(ymd(y + 1, 9, 2));
            push(u1, may, "term", "Summer 1", year, "U1");
            push(may, may + Duration::weeks(1), "half", "May half-term", year, "H");
            push(may + Duration::weeks(1), summer, "term", "Summer 2", year, "U2");
            push(summer, next, "summer", "Summer holiday", year, "H");
        }
    }
    // Start at this week: drop what is over and trim the block we are in.
    out.retain_mut(|b| {
        let start = NaiveDate::parse_from_str(&b.start, "%Y-%m-%d").expect("formatted above");
        let end = start + Duration::weeks(b.weeks as i64);
        if end <= from {
            return false;
        }
        if start < from {
            b.weeks = ((end - from).num_days() / 7) as u32;
            b.start = from.format("%Y-%m-%d").to_string();
        }
        true
    });
    out
}

impl PlanConfig {
    /// Where a new person starts: no subjects yet and a standard calendar
    /// running to the exams two summers away, all of it theirs to set up.
    pub fn blank(today: NaiveDate) -> Self {
        PlanConfig { subjects: Vec::new(), blocks: school_calendar(today, default_exam_year(today)), review_gaps: vec![7, 21, 56] }
    }

    /// The same subjects back at their built-in topics, hours and links; a
    /// subject someone wrote themselves is left as it is, and so are the
    /// term dates.
    pub fn reset_subjects(&self) -> Self {
        let cat = catalog();
        PlanConfig {
            subjects: self.subjects.iter().map(|s| cat.iter().find(|c| c.id == s.id).cloned().unwrap_or_else(|| s.clone())).collect(),
            ..self.clone()
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn easter_is_right() {
        assert_eq!(easter(2027), d("2027-03-28"));
        assert_eq!(easter(2028), d("2028-04-16"));
        assert_eq!(easter(2029), d("2029-04-01"));
    }

    /// Generated from the start of Year 10, the calendar lands on the same
    /// weeks as the hand-entered one for 2026-2028, apart from Easter, which
    /// it takes from the real date.
    #[test]
    fn the_standard_calendar_matches_a_real_school_year() {
        let cal = school_calendar(d("2026-09-07"), 2028);
        let legacy = PlanConfig::default().blocks;
        let key = |b: &BlockCfg| (b.label.clone(), b.year);
        assert_eq!(cal.iter().map(key).collect::<Vec<_>>(), legacy.iter().map(key).collect::<Vec<_>>());
        for (a, b) in cal.iter().zip(&legacy) {
            if !a.label.starts_with("Spring 2") && !a.label.starts_with("Easter") && !a.label.starts_with("Summer 1") {
                assert_eq!((&a.start, a.weeks), (&b.start, b.weeks), "{} {}", a.label, a.year);
            }
        }
        assert_eq!(cal.last().unwrap().start, "2028-05-08");
        let weeks: u32 = cal.iter().map(|b| b.weeks).sum();
        let legacy_weeks: u32 = legacy.iter().map(|b| b.weeks).sum();
        assert_eq!(weeks, legacy_weeks, "same span, first Monday to the end of the exams");
    }

    #[test]
    fn the_calendar_starts_this_week_and_runs_on_without_gaps() {
        let cal = school_calendar(d("2026-09-30"), 2028);
        assert_eq!(cal[0].start, "2026-09-28");
        assert_eq!(cal[0].label, "Autumn 1");
        for w in cal.windows(2) {
            let end = d(&w[0].start) + Duration::weeks(w[0].weeks as i64);
            assert_eq!(end, d(&w[1].start), "gap or overlap after {}", w[0].label);
        }
        assert!(school_calendar(d("2026-09-30"), 2026).is_empty(), "exams already over");
        assert_eq!(school_calendar(d("2027-02-01"), 2027).last().unwrap().kind, "exam");
    }

    #[test]
    fn a_new_person_starts_blank_and_the_plan_still_builds() {
        let cfg = PlanConfig::blank(d("2026-09-30"));
        assert!(cfg.subjects.is_empty());
        assert_eq!(cfg.blocks.last().unwrap().start, "2028-05-08");
        let plan = crate::plan::build(&cfg);
        assert!(plan.unscheduled.is_empty());
    }

    #[test]
    fn reset_restores_built_in_subjects_and_keeps_the_rest() {
        let mut cfg = PlanConfig::blank(d("2026-09-30"));
        let mut music = catalog().into_iter().find(|s| s.id == "music").unwrap();
        music.topics.truncate(2);
        let mut mine = music.clone();
        mine.id = "mine".into();
        cfg.subjects = vec![music, mine];
        let reset = cfg.reset_subjects();
        assert!(reset.subjects[0].topics.len() > 2, "built-in subject restored");
        assert_eq!(reset.subjects[1].topics.len(), 2, "own subject untouched");
        assert!(reset.blocks == cfg.blocks, "term dates kept");
    }
}
