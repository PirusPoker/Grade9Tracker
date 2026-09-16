//! Introduction videos for topics. Each is a YouTube video that plays in
//! YouTube's own embedded player - the app never copies the file, so the
//! creator keeps their views and adverts and embedding stays within YouTube's
//! terms. The first video on a topic is the one to watch before the lesson;
//! the rest cover the remaining spec points.
//!
//! The built-in list is Craig'n'Dave's OCR GCSE (J277) specification-order
//! playlist, one video per spec bullet, read off YouTube on 16 September 2026.

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

/// (topic id, [(video id, title)]) in the order they should be watched.
const VIDEOS: &[(&str, &[(&str, &str)])] = &[
    // ---------- Computer Science (OCR GCSE J277) ----------
    ("cs:1.1.1", &[
        ("7Up7DIPkTzo", "The purpose of the CPU - the fetch-execute cycle"),
        ("hk9LPXzYeT0", "CPU components and their function"),
        ("KBmoqwVt4Qg", "Von Neumann architecture"),
    ]),
    ("cs:1.1.2", &[("pZs_jfoxNLA", "Characteristics of CPUs")]),
    ("cs:1.1.3", &[("WR242RfnsIo", "Embedded systems")]),
    ("cs:1.2.1", &[
        ("dhQOkkZXu5w", "The need for primary storage"),
        ("Q2pzT6oYPWg", "RAM and ROM"),
        ("M31SS70Od08", "Virtual memory"),
    ]),
    ("cs:1.2.2", &[
        ("FNwA-h_tfPo", "The need for secondary storage"),
        ("qIy_wgo03Oo", "Common types of storage"),
        ("xfDwcdap5LA", "Suitable storage devices"),
    ]),
    ("cs:1.2.3", &[
        ("jBXWZbHPLWI", "Units of data storage"),
        ("T5gIiz0VeyI", "Processing binary data"),
        ("KzgbVfnJ7I4", "Data capacity calculations"),
    ]),
    ("cs:1.2.4", &[
        ("pCIUh20mNlA", "Converting between denary and 8-bit binary"),
        ("3K_Nw_pDzUQ", "Adding two 8-bit binary integers"),
        ("nmbr7GxN6TA", "Converting between denary and 2-digit hexadecimal"),
        ("B_3W5C7ppE4", "Binary shifts"),
        ("9oYV4JvSsok", "Representing characters"),
        ("6EfxuAOKZKc", "Representing images"),
        ("Ed7AFAzB8PM", "Representing sound"),
    ]),
    ("cs:1.2.5", &[("kOFA8FPL5kE", "Compression")]),
    ("cs:1.3.1", &[
        ("KeN3H8_Jhbc", "Types of networks"),
        ("E_9mlCpmnuk", "Performance of networks"),
        ("w_LyIDAh_bY", "Client-server, peer-to-peer"),
        ("0VZz19fBOUQ", "LAN hardware"),
        ("u0uPibV0JOw", "The internet"),
        ("PR51Iu3DC88", "Star and mesh networks"),
    ]),
    ("cs:1.3.2", &[
        ("MeKllP5f-R8", "Modes of connection"),
        ("pe6Wbfl9qt4", "Wireless encryption"),
        ("p9D0Ca3VNpM", "IP and MAC addressing"),
        ("_xwKBxDs7aY", "Standards"),
        ("ncGIs1Wnxn8", "Common protocols"),
        ("S6Kwx5ZJpxg", "The concept of layers"),
    ]),
    ("cs:1.4.1", &[
        ("4f05t8ppJfk", "Forms of attack"),
        ("jlvvek8n5g8", "Threats to networks"),
    ]),
    ("cs:1.4.2", &[("XJEjQN-CEDk", "Preventing vulnerabilities")]),
    ("cs:1.5.1", &[
        ("tArQQD4SZ7Q", "The purpose of operating systems"),
        ("dX9zhaBLJ7w", "Operating systems 1"),
        ("feECP9Q21Ow", "Operating systems 2"),
    ]),
    ("cs:1.5.2", &[("PW_T3UtaMgw", "Utility system software")]),
    ("cs:1.6.1", &[
        ("NSFjAaGeJfY", "Investigating technologies"),
        ("6zTOHgTT9qw", "Privacy issues"),
        ("fHOHOqIdhh8", "Cultural issues"),
        ("g91-xCNv8-E", "Environmental issues"),
        ("X_NKAJ9j2Qs", "Impacts of technology on society"),
        ("fT_jls9bsoI", "Legislation"),
        ("49IVvPiiGP4", "Open source vs proprietary"),
    ]),
    ("cs:2.1.1", &[
        ("wLJ1n47sGRI", "Abstraction"),
        ("zcLlXCzb4IQ", "Decomposition"),
        ("5EsSYVP_eMU", "Algorithmic thinking"),
    ]),
    ("cs:2.1.2", &[
        ("SIOleZLPMb4", "Inputs, processes and outputs"),
        ("F6f6W7S9Y6k", "Structure diagrams"),
        ("MFojJssyKLw", "Pseudocode and diagrams"),
        ("I0e74jfo1Es", "Identifying errors and suggesting fixes"),
        ("zjJTKUDCSVU", "Trace tables"),
    ]),
    ("cs:2.1.3", &[
        ("pKW-hwvD2-A", "Binary search"),
        ("Hr5cP7LOUkU", "Linear search"),
        ("aOZBMnTswL8", "Bubble sort"),
        ("Y8y7PnlE4Dg", "Merge sort"),
        ("jmdP4Y-x0Hc", "Insertion sort"),
    ]),
    ("cs:2.2.1", &[
        ("dpBe_TXFqZ8", "Variables, constants, inputs, outputs and assignments"),
        ("t0VphK9cWgE", "The three basic programming constructs"),
        ("qozjsKdyBzM", "Arithmetic and comparison operators"),
        ("IILJVSOg6Oo", "Boolean operators"),
    ]),
    ("cs:2.2.2", &[("oQAQNKomako", "Data types and casting")]),
    ("cs:2.2.3", &[
        ("u2DxYA3fwYA", "Basic string manipulation"),
        ("DFxeKaRuFcU", "Basic file handling"),
        ("es-zBs43VAs", "Records to store data"),
        ("X7aJuMJpQxM", "SQL to search for data"),
        ("izvYtCaD9EE", "Arrays"),
        ("rm19TvcXSvk", "How to use sub programs"),
        ("bni4XVtEJp4", "Random number generation"),
    ]),
    ("cs:2.3.1", &[
        ("2IIF4Infdf4", "Defensive design considerations 1"),
        ("8I0il6GQbTo", "Defensive design considerations 2"),
        ("YwW_fBw1eCY", "Maintainability"),
    ]),
    ("cs:2.3.2", &[
        ("IgOXjw76d0g", "The purpose and types of testing"),
        ("upt_QTi0id8", "How to identify syntax and logic errors"),
        ("FbnEBkN_Nko", "Suitable test data"),
        ("pBz6YoFID0Y", "Making algorithms more robust"),
    ]),
    ("cs:2.4.1", &[
        ("jN9WtjyjXf4", "Simple logic diagrams"),
        ("U7dbx9fllLc", "Truth tables"),
        ("M7h8XBjp0-s", "Combining Boolean operators"),
        ("vPG5c-RJtog", "Applying logical operators in truth tables"),
    ]),
    ("cs:2.5.1", &[
        ("4qNj8zIQzWw", "Characteristics and purpose of different languages"),
        ("BYFXtpIUPpY", "The purpose of translators"),
        ("-uaeVcs2XFs", "Compilers and interpreters"),
    ]),
    ("cs:2.5.2", &[("QPMrXhx784Y", "The IDE")]),
];

/// The built-in videos for a topic, first to watch first.
pub fn for_topic(topic_id: &str) -> Vec<Video> {
    VIDEOS.iter().find(|(id, _)| *id == topic_id)
        .map(|(_, vs)| vs.iter().map(|(id, title)| Video { id: id.to_string(), title: title.to_string(), by: CND.into() }).collect())
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
    fn ids_are_well_formed_and_unique() {
        let mut seen = std::collections::HashSet::new();
        for (topic, vs) in VIDEOS {
            for (id, title) in *vs {
                assert_eq!(video_id(id).as_deref(), Some(*id), "{topic}: bad id {id}");
                assert!(seen.insert(*id), "{topic}: {id} listed twice");
                assert!(!title.trim().is_empty(), "{topic}: {id} has no title");
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
