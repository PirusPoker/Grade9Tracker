//! The study plan: subjects and topics from the Edexcel IGCSE specifications,
//! a two-year calendar, and the scheduler that pours topic-hours into weeks and
//! places the four layers of tests. Produced once at startup and sent to the UI.

use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::config::{PlanConfig, ResourceLink};
use std::collections::HashMap;

// ---------- Data ----------

pub struct SubjectDef {
    pub id: &'static str,
    pub name: &'static str,
    pub full: &'static str,
    pub color: &'static str,
    pub papers: &'static str,
    pub spec: &'static str,
    pub sections: &'static [&'static str],
    pub topics: &'static [(&'static str, &'static str, f64)],
}

/// The subjects a new profile starts with. Everything else in [`SUBJECTS`] is
/// the built-in catalog: fully written, but only in a plan once someone adds
/// it from the Plan tab, so adding a subject here never changes anyone's plan.
pub const STARTER: &[&str] = &["maths", "fpm", "bus", "econ", "cs", "englit", "englang", "bio", "chem", "phys"];

pub const SUBJECTS: &[SubjectDef] = &[
    SubjectDef {
        id: "maths", name: "Maths", full: "Pearson Edexcel International GCSE Mathematics A (4MA1) Higher", color: "var(--maths)",
        papers: "Two 2-hour papers, 100 marks each, calculator allowed",
        spec: "https://qualifications.pearson.com/en/qualifications/edexcel-international-gcses/international-gcse-mathematics-a-2016.html",
        sections: &["1 Number", "2 Algebra", "3 Sequences, functions, graphs & calculus", "4 Geometry & trigonometry", "5 Vectors & transformations", "6 Statistics & probability"],
        topics: &[
            ("1.1", "Integers, primes, factors, multiples, HCF & LCM", 0.5),
            ("1.2", "Fractions: the four rules, mixed numbers and ordering", 0.5),
            ("1.3", "Decimals, and converting recurring decimals to fractions", 1.0),
            ("1.4a", "Powers, roots and the index laws", 1.5),
            ("1.4b", "Surds and rationalising the denominator", 1.5),
            ("1.5", "Set language, notation and Venn diagrams", 1.0),
            ("1.6", "Percentages: reverse, compound and repeated change", 1.5),
            ("1.7", "Ratio and proportion", 1.0),
            ("1.8", "Accuracy, upper and lower bounds, estimation", 1.5),
            ("1.9", "Standard form", 1.0),
            ("1.10", "Applying number: units, time, money and currency", 0.5),
            ("1.11", "Getting everything out of your calculator", 0.5),
            ("2.1", "Symbols, index notation and the rules of algebra", 0.5),
            ("2.2a", "Expanding and factorising", 2.0),
            ("2.2b", "Algebraic fractions, completing the square and algebraic proof", 2.0),
            ("2.3", "Formulae: substituting and changing the subject", 1.0),
            ("2.4", "Linear equations", 0.5),
            ("2.5", "Direct and inverse proportion", 1.0),
            ("2.6", "Simultaneous linear equations", 1.0),
            ("2.7a", "Quadratic equations: factorising, formula, completing the square", 2.5),
            ("2.7b", "Quadratics from context, and one linear with one quadratic", 1.5),
            ("2.8", "Inequalities: linear, quadratic and graphical regions", 2.0),
            ("3.1a", "Sequences and the nth term", 1.0),
            ("3.1b", "Arithmetic series: first term, common difference and the sum of n terms", 1.5),
            ("3.2", "Functions: notation, domain, range, composite and inverse", 2.0),
            ("3.3a", "Straight lines: gradient, y = mx + c, parallel and perpendicular", 1.5),
            ("3.3b", "Curves: cubic, reciprocal and trigonometric graphs", 2.0),
            ("3.3c", "Transforming graphs, and solving equations from intersections", 2.0),
            ("3.4", "Calculus: differentiation, stationary points and kinematics", 3.0),
            ("4.1", "Angles, lines and triangles", 0.5),
            ("4.2", "Polygons, quadrilaterals and congruence", 1.0),
            ("4.3", "Symmetry", 0.5),
            ("4.4", "Measures, compound measures and bearings", 1.0),
            ("4.5", "Constructions, loci and scale drawings", 1.0),
            ("4.6", "Circle theorems, cyclic quadrilaterals and intersecting chords", 2.5),
            ("4.7", "Geometrical reasoning: giving the reason", 1.0),
            ("4.8a", "Pythagoras and right-angled trigonometry", 2.0),
            ("4.8b", "Sine rule, cosine rule and the area of a triangle", 2.0),
            ("4.8c", "Pythagoras and trigonometry in three dimensions", 1.5),
            ("4.9", "Mensuration: perimeter, area, arcs and sectors", 1.5),
            ("4.10", "3D shapes: surface area and volume", 1.5),
            ("4.11", "Similar shapes: length, area and volume scale factors", 1.5),
            ("5.1", "Vectors, magnitude, resultants and vector proof", 2.0),
            ("5.2", "Transformations: reflection, rotation, translation, enlargement", 1.0),
            ("6.1a", "Presenting data: charts, tables and two-way tables", 0.5),
            ("6.1b", "Histograms with unequal class widths", 1.5),
            ("6.1c", "Cumulative frequency diagrams", 1.5),
            ("6.2", "Averages, grouped data, quartiles and spread", 1.0),
            ("6.3a", "Probability: sample space, complement and the addition rule", 1.5),
            ("6.3b", "Tree diagrams, conditional probability and without replacement", 2.0),
        ],
    },
    SubjectDef {
        id: "fpm", name: "Further Maths", full: "Pearson Edexcel International GCSE Further Pure Mathematics (4PM1)", color: "var(--fpm)",
        papers: "Two 2-hour papers, 100 marks each, calculator + formula sheet",
        spec: "https://qualifications.pearson.com/en/qualifications/edexcel-international-gcses/international-gcse-further-pure-mathematics-2017.html",
        sections: &["1 Logs & indices", "2 The quadratic function", "3 Identities & inequalities", "4 Graphs", "5 Series", "6 The binomial series", "7 Vectors", "8 Coordinate geometry", "9 Calculus", "10 Trigonometry"],
        topics: &[
            ("1a", "Logarithms and the laws of logs", 2.0),
            ("1b", "Indices, surds and rationalising the denominator", 1.5),
            ("2", "The quadratic function: discriminant, and functions of the roots", 2.5),
            ("3a", "Algebraic division, the factor and remainder theorems, cubics", 2.0),
            ("3b", "Inequalities, linear and quadratic", 1.5),
            ("3c", "Graphical inequalities and linear programming", 1.0),
            ("4", "Graphs of polynomials and rational functions, asymptotes", 2.0),
            ("5", "Series: sigma notation, arithmetic and geometric, sum to infinity", 3.0),
            ("6", "The binomial series for positive integer and rational n", 2.0),
            ("7", "Vectors: components, magnitude, position vectors and geometric proof", 2.5),
            ("8", "Coordinate geometry: distance, ratio, gradient, parallel and perpendicular", 2.0),
            ("9a", "Differentiation: powers, trig, exponentials, product, quotient and chain", 3.0),
            ("9b", "Stationary points, maxima and minima, tangents and normals", 2.0),
            ("9c", "Integration, areas under curves and volumes of revolution", 2.5),
            ("9d", "Kinematics and connected rates of change", 1.5),
            ("10a", "Radians, arcs, sectors and exact values", 1.5),
            ("10b", "Sine and cosine rules, and problems in three dimensions", 2.0),
            ("10c", "Trigonometric identities, addition formulae and equations", 3.0),
        ],
    },
    SubjectDef {
        id: "bus", name: "Business", full: "AQA GCSE Business (8132)", color: "var(--bus)",
        papers: "Paper 1 (3.1-3.4) and Paper 2 (3.1, 3.2, 3.5, 3.6), each 1h45, 90 marks, 50%. Each paper opens with 20 marks of multiple choice and short answer. Formulae are NOT given",
        spec: "https://www.aqa.org.uk/subjects/business/gcse/business-8132",
        sections: &["3.1 Business in the real world", "3.2 Influences on business", "3.3 Business operations", "3.4 Human resources", "3.5 Marketing", "3.6 Finance"],
        topics: &[
            ("3.1.1", "The purpose and nature of business, enterprise and entrepreneurs", 1.0),
            ("3.1.2", "Business ownership, from sole trader to plc and not-for-profit", 1.5),
            ("3.1.3", "Setting aims and objectives, and why they differ and change", 1.0),
            ("3.1.4", "Stakeholders, their objectives and the conflicts between them", 1.0),
            ("3.1.5", "Business location", 0.5),
            ("3.1.6", "Business planning, and basic costs, revenue and profit", 1.0),
            ("3.1.7", "Expanding a business, economies and diseconomies of scale", 1.5),
            ("3.2.1", "Technology: e-commerce and digital communication", 0.75),
            ("3.2.2", "Ethical and environmental considerations", 1.0),
            ("3.2.3", "The economic climate: interest rates, employment and consumer spending", 1.25),
            ("3.2.4", "Globalisation, international competitiveness and exchange rates", 1.0),
            ("3.2.5", "Legislation: employment, health and safety, and consumer law", 1.0),
            ("3.2.6", "The competitive environment, risk and uncertainty", 1.0),
            ("3.3.1", "Production processes: job, flow, lean production and JIT", 1.25),
            ("3.3.2", "Procurement, stock, choosing suppliers and the supply chain", 1.5),
            ("3.3.3", "Quality, and the cost of getting it wrong", 1.0),
            ("3.3.4", "Customer service and the sales process", 1.0),
            ("3.4.1", "Organisational structures, centralisation and delegation", 1.5),
            ("3.4.2", "Recruitment, selection and contracts of employment", 1.5),
            ("3.4.3", "Motivating employees, financially and non-financially", 1.25),
            ("3.4.4", "Training", 1.0),
            ("3.5.1", "Identifying and understanding customers", 0.75),
            ("3.5.2", "Segmentation", 0.75),
            ("3.5.3", "The purpose and methods of market research", 1.5),
            ("3.5.4", "The marketing mix: the 4Ps, product life cycle and Boston matrix", 3.0),
            ("3.6.1", "Sources of finance, internal and external", 1.5),
            ("3.6.2", "Cash flow, and why it is not the same as profit", 1.75),
            ("3.6.3", "Financial terms, average rate of return and break-even", 2.0),
            ("3.6.4", "Analysing financial performance and the profit margins", 1.75),
        ],
    },
    SubjectDef {
        id: "econ", name: "Economics", full: "Cambridge IGCSE (9-1) Economics (0987)", color: "var(--econ)",
        papers: "Paper 1 multiple choice, 1h, 40 marks, 30%. Paper 2 structured, 2h, 80 marks, 70% - one compulsory six-part question, then three from four",
        spec: "https://www.cambridgeinternational.org/programmes-and-qualifications/cambridge-igcse-economics-9-1-0987/",
        sections: &["1 The basic economic problem", "2 The allocation of resources", "3 Microeconomic decision-makers", "4 Government and the macroeconomy", "5 Economic development", "6 International trade and globalisation"],
        topics: &[
            ("1.1", "The basic economic problem: scarcity, and the three questions", 1.0),
            ("1.2", "Factors of production and their rewards", 0.75),
            ("1.3", "Opportunity cost and how it shapes decisions", 0.75),
            ("1.4", "Production possibility curve diagrams", 1.25),
            ("2.1", "The role of markets in allocating resources", 0.5),
            ("2.2", "Demand: movements along and shifts of the curve", 1.25),
            ("2.3", "Supply: movements along and shifts of the curve", 1.25),
            ("2.4", "Price determination, equilibrium and disequilibrium", 1.5),
            ("2.5", "Causes and consequences of price changes", 1.0),
            ("2.6", "Price elasticity of demand, and its effect on revenue", 1.75),
            ("2.7", "Price elasticity of supply", 1.0),
            ("2.8", "The market economic system, for and against", 0.75),
            ("2.9", "Market failure: public, merit and demerit goods, externalities", 1.75),
            ("2.10", "The mixed economy and government intervention", 2.0),
            ("3.1", "Money and banking: central and commercial banks", 1.0),
            ("3.2", "Households: spending, saving and borrowing", 0.75),
            ("3.3", "Workers, wage determination and the labour market", 2.0),
            ("3.4", "Firms, mergers, and economies and diseconomies of scale", 1.75),
            ("3.5", "Firms and production: labour- and capital-intensive, productivity", 1.25),
            ("3.6", "Firms' costs, revenue and objectives", 1.75),
            ("3.7", "Competitive markets and monopoly", 1.0),
            ("4.1", "Macroeconomic aims and the conflicts between them", 1.25),
            ("4.2", "Fiscal policy: the budget, taxation and spending", 1.75),
            ("4.3", "Monetary policy", 1.25),
            ("4.4", "Supply-side policy", 1.0),
            ("4.5", "Economic growth and recession", 1.5),
            ("4.6", "Employment and unemployment", 1.5),
            ("4.7", "Inflation and deflation", 1.5),
            ("5.1", "Living standards: real GDP per head and the HDI", 1.0),
            ("5.2", "Poverty, absolute and relative, and policies against it", 1.0),
            ("5.3", "Population: birth rates, death rates and migration", 1.25),
            ("5.4", "Differences in economic development between countries", 1.0),
            ("6.1", "Specialisation by country and free trade", 1.0),
            ("6.2", "Globalisation, multinationals and trade restrictions", 2.0),
            ("6.3", "Foreign exchange rates", 1.5),
            ("6.4", "The current account of the balance of payments", 1.5),
        ],
    },
    SubjectDef {
        id: "cs", name: "Computer Science", full: "OCR GCSE Computer Science (J277)", color: "var(--cs)",
        papers: "J277/01 Computer systems and J277/02 Computational thinking, algorithms and programming: each 1h30, 80 marks, 50%. No calculator. Paper 2 Section B (30 marks) is answered in OCR Exam Reference Language or a high-level language",
        spec: "https://www.ocr.org.uk/qualifications/gcse/computer-science-j277-from-2020/",
        sections: &["1.1 Systems architecture", "1.2 Memory and storage", "1.3 Computer networks, connections and protocols", "1.4 Network security", "1.5 Systems software", "1.6 Ethical, legal, cultural and environmental impacts", "2.1 Algorithms", "2.2 Programming fundamentals", "2.3 Producing robust programs", "2.4 Boolean logic", "2.5 Programming languages and IDEs"],
        topics: &[
            ("1.1.1", "Architecture of the CPU: fetch-execute, components, von Neumann registers", 1.5),
            ("1.1.2", "CPU performance: clock speed, cache size, cores", 0.5),
            ("1.1.3", "Embedded systems", 0.5),
            ("1.2.1", "Primary storage: RAM, ROM, virtual memory, cache", 1.0),
            ("1.2.2", "Secondary storage: optical, magnetic, solid state, and choosing between them", 1.0),
            ("1.2.3", "Units of data and calculating capacity", 0.5),
            ("1.2.4", "Data storage: binary, hexadecimal, shifts, characters, images and sound", 3.0),
            ("1.2.5", "Compression: lossy and lossless", 0.5),
            ("1.3.1", "Networks and topologies: LAN, WAN, client-server, hardware, DNS, cloud, star and mesh", 2.0),
            ("1.3.2", "Wired and wireless, encryption, IP and MAC addresses, protocols and layers", 2.0),
            ("1.4.1", "Threats: malware, social engineering, brute force, DoS, interception, SQL injection", 1.0),
            ("1.4.2", "Preventing vulnerabilities: pen testing, anti-malware, firewalls, access levels, passwords, encryption, physical security", 1.0),
            ("1.5.1", "Operating systems: interface, memory, peripherals, users, files", 1.0),
            ("1.5.2", "Utility software: encryption, defragmentation, compression", 0.5),
            ("1.6.1", "Ethical, legal, cultural and environmental impacts, and the legislation", 1.5),
            ("2.1.1", "Computational thinking: abstraction, decomposition, algorithmic thinking", 0.5),
            ("2.1.2", "Designing, creating and refining algorithms: pseudocode, flowcharts, trace tables, errors", 2.5),
            ("2.1.3", "Searching and sorting: binary, linear, bubble, merge, insertion", 2.0),
            ("2.2.1", "Programming fundamentals: variables, constants, sequence, selection, iteration, operators", 2.0),
            ("2.2.2", "Data types and casting", 0.5),
            ("2.2.3", "Additional techniques: strings, files, records, SQL, arrays, sub programs, random numbers", 3.5),
            ("2.3.1", "Defensive design: input validation, authentication, maintainability", 1.0),
            ("2.3.2", "Testing: iterative and final, syntax and logic errors, test data, refining", 1.5),
            ("2.4.1", "Boolean logic: logic diagrams, truth tables, combining AND, OR and NOT", 1.5),
            ("2.5.1", "Languages: high- and low-level, translators, compilers and interpreters", 1.0),
            ("2.5.2", "The IDE and its tools", 0.5),
        ],
    },
    SubjectDef {
        id: "englit", name: "Eng Literature", full: "AQA GCSE English Literature (8702)", color: "var(--englit)",
        papers: "Paper 1 1h45 (40%), Paper 2 2h15 (60%). Closed book — no texts in the exam",
        spec: "https://www.aqa.org.uk/subjects/english/gcse/english-8702/specification",
        sections: &["3.1.1 Macbeth", "3.1.2 A Christmas Carol", "3.2.1 An Inspector Calls", "3.2.2 Power and Conflict poetry", "3.2.3 Unseen poetry", "3.3 Writing about texts"],
        topics: &[
            // 3.1.1 Shakespeare — Macbeth
            ("3.1.1a", "Macbeth: plot and structure", 1.5),
            ("3.1.1b", "Macbeth: his character and his downfall", 1.5),
            ("3.1.1c", "Lady Macbeth", 1.5),
            ("3.1.1d", "Macbeth: ambition, power and kingship", 1.5),
            ("3.1.1e", "Macbeth: guilt, fate and the supernatural", 1.5),
            ("3.1.1f", "Macbeth: Jacobean context, language and stagecraft", 1.5),
            // 3.1.2 The 19th-century novel — A Christmas Carol
            ("3.1.2a", "A Christmas Carol: the five staves and their structure", 1.5),
            ("3.1.2b", "Scrooge and his transformation", 1.5),
            ("3.1.2c", "The ghosts and what each one is for", 1.5),
            ("3.1.2d", "A Christmas Carol: poverty, social responsibility and redemption", 1.5),
            ("3.1.2e", "A Christmas Carol: Victorian context and Dickens's methods", 1.5),
            // 3.2.1 Modern texts — An Inspector Calls
            ("3.2.1a", "An Inspector Calls: the three acts and dramatic structure", 1.5),
            ("3.2.1b", "The Birlings and Gerald", 1.5),
            ("3.2.1c", "The Inspector: role, method and significance", 1.5),
            ("3.2.1d", "An Inspector Calls: responsibility, class, gender and generation", 1.5),
            ("3.2.1e", "An Inspector Calls: 1912 against 1945, and Priestley's stagecraft", 1.5),
            // 3.2.2 Poetry — Power and Conflict
            ("3.2.2a", "Ozymandias — Shelley", 0.75),
            ("3.2.2b", "London — Blake", 0.75),
            ("3.2.2c", "Extract from The Prelude — Wordsworth", 0.75),
            ("3.2.2d", "My Last Duchess — Browning", 0.75),
            ("3.2.2e", "The Charge of the Light Brigade — Tennyson", 0.75),
            ("3.2.2f", "Exposure — Owen", 0.75),
            ("3.2.2g", "Storm on the Island — Heaney", 0.75),
            ("3.2.2h", "Bayonet Charge — Hughes", 0.75),
            ("3.2.2i", "Remains — Armitage", 0.75),
            ("3.2.2j", "Poppies — Weir", 0.75),
            ("3.2.2k", "War Photographer — Duffy", 0.75),
            ("3.2.2l", "Tissue — Dharker", 0.75),
            ("3.2.2m", "The Émigrée — Rumens", 0.75),
            ("3.2.2n", "Checking Out Me History — Agard", 0.75),
            ("3.2.2o", "Kamikaze — Garland", 0.75),
            ("3.2.2p", "Grouping the poems by theme", 1.0),
            ("3.2.2q", "Writing a poetry comparison", 1.0),
            // 3.2.3 Unseen poetry
            ("3.2.3a", "Analysing an unseen poem", 1.0),
            ("3.2.3b", "Comparing two unseen poems", 1.0),
            // 3.3 Skills
            ("3.3a", "Writing about language, form and structure", 1.0),
            ("3.3b", "Using context without bolting it on", 1.0),
        ],
    },
    SubjectDef {
        id: "englang", name: "Eng Language", full: "AQA GCSE English Language (8700)", color: "var(--englang)",
        papers: "Paper 1 1h45 (50%), Paper 2 1h45 (50%), plus a Spoken Language endorsement",
        spec: "https://www.aqa.org.uk/subjects/english/gcse/english-8700/specification",
        sections: &["1.1 Paper 1A fiction reading", "1.2 Paper 1B creative writing", "2.1 Paper 2A non-fiction reading", "2.2 Paper 2B viewpoint writing", "3 Spoken Language"],
        topics: &[
            // 1.1 Paper 1 Section A — reading one literature fiction text
            ("1.1a", "Q1: finding and listing explicit information", 0.75),
            ("1.1b", "Q2: analysing the writer's language choices", 1.5),
            ("1.1c", "Q3: analysing structure across a whole extract", 1.5),
            ("1.1d", "Q4: evaluating a statement about the text", 2.0),
            // 1.2 Paper 1 Section B — descriptive or narrative writing
            ("1.2a", "Descriptive writing from a picture or prompt", 1.5),
            ("1.2b", "Narrative writing: shape, voice and restraint", 1.5),
            ("1.2c", "Openings, endings and technical accuracy", 1.5),
            // 2.1 Paper 2 Section A — two non-fiction texts
            ("2.1a", "Q1: true or false selection under time pressure", 0.75),
            ("2.1b", "Q2: summarising and inferring across two texts", 1.5),
            ("2.1c", "Q3: analysing language in a non-fiction text", 1.5),
            ("2.1d", "Q4: comparing writers' viewpoints and methods", 2.0),
            // 2.2 Paper 2 Section B — writing to present a viewpoint
            ("2.2a", "Writing to argue and to persuade", 1.5),
            ("2.2b", "Form, audience and purpose: letter, article, speech, essay", 1.5),
            ("2.2c", "Structuring an argument, and accuracy under time", 1.5),
            // 3 Spoken Language endorsement
            ("3a", "Preparing and delivering a presentation", 1.0),
            ("3b", "Responding to questions and feedback", 0.75),
        ],
    },
    SubjectDef {
        id: "bio", name: "Biology", full: "Pearson Edexcel International GCSE Biology (4BI1)", color: "var(--bio)",
        papers: "Paper 1 2h (61.1%), Paper 2 1h15 (38.9%). 13 required practicals",
        spec: "https://qualifications.pearson.com/en/qualifications/edexcel-international-gcses/international-gcse-biology-2017.html",
        sections: &["1 Nature and variety of living organisms", "2 Structures and functions", "3 Reproduction and inheritance", "4 Ecology and the environment", "5 Use of biological resources"],
        topics: &[
            ("1a", "Characteristics and variety of living organisms", 1.5),
            ("2a", "Levels of organisation, cell structure and differentiation", 1.5),
            ("2b", "Biological molecules and enzymes", 2.0),
            ("2c", "Diffusion, osmosis and active transport", 1.5),
            ("2d", "Photosynthesis and the leaf", 2.0),
            ("2e", "Nutrition and digestion", 2.5),
            ("2f", "Respiration, aerobic and anaerobic", 1.5),
            ("2g", "Gas exchange in plants", 1.5),
            ("2h", "Gas exchange in humans", 1.5),
            ("2i", "Transport in plants: xylem, phloem and transpiration", 2.0),
            ("2j", "Blood, immunity and vaccination", 2.0),
            ("2k", "The heart and circulation", 2.0),
            ("2l", "Excretion and the kidney", 2.0),
            ("2m", "Homeostasis and plant responses", 1.5),
            ("2n", "The nervous system, reflexes and the eye", 2.0),
            ("2o", "Hormones in humans", 1.0),
            ("3a", "Reproduction in plants", 2.0),
            ("3b", "Reproduction in humans", 2.0),
            ("3c", "DNA, the genome and protein synthesis", 2.0),
            ("3d", "Inheritance, genetic diagrams and pedigrees", 2.5),
            ("3e", "Mitosis and meiosis", 1.5),
            ("3f", "Variation, mutation and evolution", 2.0),
            ("4a", "Populations, communities and ecosystems", 1.5),
            ("4b", "Feeding relationships and energy transfer", 1.5),
            ("4c", "The carbon and nitrogen cycles", 1.5),
            ("4d", "Human influences: greenhouse gases, pollution, deforestation", 2.0),
            ("5a", "Crop production and pest control", 1.5),
            ("5b", "Microorganisms, fermentation and fish farming", 2.0),
            ("5c", "Selective breeding", 1.0),
            ("5d", "Genetic modification", 2.0),
            ("5e", "Cloning and micropropagation", 1.5),
        ],
    },
    SubjectDef {
        id: "chem", name: "Chemistry", full: "Pearson Edexcel International GCSE Chemistry (4CH1)", color: "var(--chem)",
        papers: "Paper 1 2h (61.1%), Paper 2 1h15 (38.9%). Calculator allowed",
        spec: "https://qualifications.pearson.com/en/qualifications/edexcel-international-gcses/international-gcse-chemistry-2017.html",
        sections: &["1 Principles of chemistry", "2 Inorganic chemistry", "3 Physical chemistry", "4 Organic chemistry"],
        topics: &[
            ("1a", "States of matter and diffusion", 1.0),
            ("1b", "Solubility and solubility curves", 1.0),
            ("1c", "Pure substances, mixtures and separation techniques", 1.5),
            ("1d", "Atomic structure and the periodic table", 2.0),
            ("1e", "Formulae, equations and relative formula mass", 1.5),
            ("1f", "Moles, reacting masses and percentage yield", 2.5),
            ("1g", "Empirical and molecular formulae", 1.5),
            ("1h", "Ionic bonding and the properties of ionic compounds", 2.0),
            ("1i", "Covalent bonding and simple molecular substances", 2.0),
            ("1j", "Giant covalent structures: diamond, graphite, silicon dioxide", 1.0),
            ("1k", "Metallic bonding", 1.0),
            ("1l", "Electrolysis and ionic half-equations", 2.5),
            ("2a", "Group 1: the alkali metals", 1.0),
            ("2b", "Group 7: the halogens and displacement", 1.5),
            ("2c", "Gases in the air, combustion and carbon dioxide", 1.5),
            ("2d", "The reactivity series and rusting", 1.5),
            ("2e", "Extraction of metals, uses and alloys", 2.0),
            ("2f", "Acids, alkalis, indicators and the pH scale", 1.5),
            ("2g", "Reactions of acids and preparing salts", 2.5),
            ("2h", "Chemical tests for gases, cations, anions and water", 2.0),
            ("3a", "Energetics, calorimetry and bond energies", 2.5),
            ("3b", "Rates of reaction and catalysts", 2.5),
            ("3c", "Reversible reactions and equilibrium", 2.0),
            ("4a", "Homologous series, formulae, isomers and naming", 2.0),
            ("4b", "Crude oil and fractional distillation", 1.5),
            ("4c", "Fuels, combustion and pollutants", 1.5),
            ("4d", "Cracking", 1.0),
            ("4e", "Alkanes", 1.5),
            ("4f", "Alkenes", 1.5),
            ("4g", "Alcohols", 1.5),
            ("4h", "Carboxylic acids and esters", 2.0),
            ("4i", "Addition and condensation polymers", 2.0),
        ],
    },
    SubjectDef {
        id: "phys", name: "Physics", full: "Pearson Edexcel International GCSE Physics (4PH1)", color: "var(--phys)",
        papers: "Paper 1 2h (61.1%), Paper 2 1h15 (38.9%). Formulae must be recalled",
        spec: "https://qualifications.pearson.com/en/qualifications/edexcel-international-gcses/international-gcse-physics-2017.html",
        sections: &["1 Forces and motion", "2 Electricity", "3 Waves", "4 Energy resources", "5 Solids, liquids and gases", "6 Magnetism and electromagnetism", "7 Radioactivity and particles", "8 Astrophysics"],
        topics: &[
            ("1a", "Units, distance-time and velocity-time graphs", 1.5),
            ("1b", "Speed, velocity and acceleration calculations", 2.0),
            ("1c", "Forces, vectors and resultant force", 1.5),
            ("1d", "Newton's laws, friction, weight and mass", 2.0),
            ("1e", "Stopping distance and falling objects", 1.5),
            ("1f", "Hooke's law and elastic behaviour", 1.5),
            ("1g", "Momentum, impulse and collisions", 2.0),
            ("1h", "Moments, centre of gravity and equilibrium", 2.0),
            ("2a", "Current, voltage, resistance and Ohm's law", 2.0),
            ("2b", "Series and parallel circuits", 2.0),
            ("2c", "Component characteristics and I–V graphs", 1.5),
            ("2d", "Electrical energy, power and mains electricity safety", 2.0),
            ("2e", "Charge, current and the relationship Q = It", 1.0),
            ("2f", "Static electricity and its uses and dangers", 1.5),
            ("3a", "Wave properties, wave equations and wave behaviour", 2.0),
            ("3b", "The electromagnetic spectrum, its uses and its dangers", 1.5),
            ("3c", "Reflection, refraction and ray diagrams", 2.0),
            ("3d", "Refractive index and total internal reflection", 2.0),
            ("3e", "Sound waves, pitch, loudness and the oscilloscope", 2.0),
            ("4a", "Energy stores, transfers and conservation", 1.5),
            ("4b", "Efficiency and Sankey diagrams", 1.5),
            ("4c", "Thermal energy transfer: conduction, convection, radiation", 1.5),
            ("4d", "Work, power, kinetic and gravitational potential energy", 2.5),
            ("4e", "Energy resources and electricity generation", 1.5),
            ("5a", "Density and pressure", 2.0),
            ("5b", "Specific heat capacity and changes of state", 2.0),
            ("5c", "The particle model, absolute zero and the Kelvin scale", 1.5),
            ("5d", "The gas laws: pressure, volume and temperature", 2.0),
            ("6a", "Magnets, magnetic fields and electromagnets", 1.5),
            ("6b", "The motor effect and the left-hand rule", 1.5),
            ("6c", "Electromagnetic induction and generators", 1.5),
            ("6d", "Transformers and the National Grid", 2.0),
            ("7a", "Atomic structure, isotopes and the three radiations", 2.0),
            ("7b", "Nuclear equations and radioactive decay", 2.0),
            ("7c", "Half-life, detection and background radiation", 2.0),
            ("7d", "Uses and dangers of radioactivity", 1.5),
            ("7e", "Nuclear fission and the reactor", 2.0),
            ("7f", "Nuclear fusion", 1.5),
            ("8a", "Gravity, orbits and the solar system", 2.0),
            ("8b", "Classifying stars and stellar evolution", 2.0),
            ("8c", "Hertzsprung–Russell diagram, red shift and the Big Bang", 2.0),
        ],
    },
    // AQA GCSE French 8652, the new specification: first taught September 2024,
    // first examined June 2026. AQA is the most-taken board for French: the June
    // 2026 entries in each board's own results statistics put AQA ahead of
    // Pearson Edexcel, with Eduqas far behind. Themes, grammar (3.2.1
    // Foundation, 3.2.2 Higher) and the papers read from the specification PDF.
    SubjectDef {
        id: "fre", name: "French", full: "AQA GCSE French (8652) Higher", color: "var(--fre)",
        papers: "Higher tier, four papers of 50 marks, 25% each, all sat in the same June: Listening 45 min (questions in English, then a dictation), Speaking 10-12 min (role-play, reading aloud, photo card), Reading 1 h (questions in English, then a translation into English), Writing 1 h 15 (a translation into French, a 90-word task and a 150-word task). Only the AQA vocabulary list and grammar are examined",
        spec: "https://www.aqa.org.uk/subjects/french/gcse/french-8652/specification",
        sections: &["Theme 1: People and lifestyle", "Theme 2: Popular culture", "Theme 3: Communication and the world around us", "Grammar", "Paper 1 Listening", "Paper 2 Speaking", "Paper 3 Reading", "Paper 4 Writing"],
        topics: &[
            ("3.1.1a", "Identity and relationships: family, friends and describing people", 1.5),
            ("3.1.1b", "Healthy living and lifestyle", 1.5),
            ("3.1.1c", "Education and work: school, jobs and future plans", 1.5),
            ("3.1.2a", "Free-time activities: sport, music, cinema, eating out", 1.5),
            ("3.1.2b", "Customs, festivals and celebrations", 1.5),
            ("3.1.2c", "Celebrity culture: role models, influencers and fame", 1.5),
            ("3.1.3a", "Travel and tourism, including places of interest", 1.5),
            ("3.1.3b", "Media and technology: phones, social media, TV and film", 1.5),
            ("3.1.3c", "The environment and where people live", 1.5),
            ("3.2.1a", "Nouns and articles: gender, plurals, the partitive and de after negatives", 1.0),
            ("3.2.1b", "Determiners and pronouns: ce, mon, quel, tout; object and reflexive pronouns; qui", 1.25),
            ("3.2.1c", "The present tense, negatives and questions", 1.5),
            ("3.2.1d", "The perfect tense with avoir and être", 1.5),
            ("3.2.1e", "The near future, the imperfect and the imperative", 1.25),
            ("3.2.1f", "Modal, reflexive and impersonal verbs: devoir, pouvoir, il faut, il y a", 1.0),
            ("3.2.1g", "Adjectives and adverbs: agreement, position and comparisons", 1.0),
            ("3.2.1h", "Prepositions, places and word families", 1.0),
            ("3.2.2a", "Higher pronouns: y, en, plural object pronouns, emphatic pronouns, où and que", 1.25),
            ("3.2.2b", "The future and conditional tenses, and the imperfect in full", 1.5),
            ("3.2.2c", "Time and verb structures: depuis, venir de, en train de, en + -ant, après avoir", 1.25),
            ("3.2.2d", "More negatives, the passive, impersonal phrases and superlatives", 1.0),
            ("4.4a", "Paper 1 Listening: questions in English and the distractors", 1.5),
            ("4.4b", "Paper 1 dictation: French sounds and how they are spelt", 1.0),
            ("4.5", "Paper 2 Speaking: role-play, reading aloud and the photo card", 2.0),
            ("4.6", "Paper 3 Reading, and translating into English", 1.5),
            ("4.7", "Paper 4 Writing: translation into French, the 90- and 150-word tasks", 2.0),
        ],
    },
    // AQA GCSE Spanish 8692, the new specification: first taught September 2024,
    // first examined June 2026. AQA is the most-taken board for Spanish: the June
    // 2026 entries in each board's own results statistics put AQA ahead of
    // Pearson Edexcel, with Eduqas far behind. Themes, grammar (3.2.1
    // Foundation, 3.2.2 Higher) and the papers read from the specification PDF.
    SubjectDef {
        id: "spa", name: "Spanish", full: "AQA GCSE Spanish (8692) Higher", color: "var(--spa)",
        papers: "Higher tier, four papers of 50 marks, 25% each, all sat in the same June: Listening 45 min (questions in English, then a dictation), Speaking 10-12 min (role-play, reading aloud, photo card), Reading 1 h (questions in English, then a translation into English), Writing 1 h 15 (a translation into Spanish, a 90-word task and a 150-word task). Only the AQA vocabulary list and grammar are examined",
        spec: "https://www.aqa.org.uk/subjects/spanish/gcse/spanish-8692/specification",
        sections: &["Theme 1: People and lifestyle", "Theme 2: Popular culture", "Theme 3: Communication and the world around us", "Grammar", "Paper 1 Listening", "Paper 2 Speaking", "Paper 3 Reading", "Paper 4 Writing"],
        topics: &[
            ("3.1.1a", "Identity and relationships: family, friends and describing people", 1.5),
            ("3.1.1b", "Healthy living and lifestyle", 1.5),
            ("3.1.1c", "Education and work: school, jobs and future plans", 1.5),
            ("3.1.2a", "Free-time activities: sport, music, cinema, eating out", 1.5),
            ("3.1.2b", "Customs, festivals and celebrations", 1.5),
            ("3.1.2c", "Celebrity culture: role models, influencers and fame", 1.5),
            ("3.1.3a", "Travel and tourism, including places of interest", 1.5),
            ("3.1.3b", "Media and technology: phones, social media, TV and film", 1.5),
            ("3.1.3c", "The environment and where people live", 1.5),
            ("3.2.1a", "Nouns and articles: gender, plurals, este, mi, cada, otro, todo", 1.0),
            ("3.2.1b", "Pronouns: dropping the subject, object and reflexive pronouns, que, esto and eso", 1.25),
            ("3.2.1c", "The present tense: regular verbs, the stem-changers, ser, estar, tener, ir and hacer", 1.5),
            ("3.2.1d", "The preterite tense", 1.5),
            ("3.2.1e", "The present continuous, the present perfect and the imperfect", 1.25),
            ("3.2.1f", "Talking about the future: ir a, the future and conditional, and tú commands", 1.25),
            ("3.2.1g", "Modals, gustar-type verbs, reflexives, hay que and se puede", 1.0),
            ("3.2.1h", "Adjectives and adverbs: agreement, position, ser or estar, and comparisons", 1.0),
            ("3.2.1i", "Prepositions, the personal a, and word endings: -ito, -ísimo, -mente, -idad", 1.0),
            ("3.2.2a", "Higher pronouns: nos and os, lo que, el que, possessive pronouns, conmigo, aquel", 1.25),
            ("3.2.2b", "The future, conditional and imperfect in full, and preterite stem changes", 1.25),
            ("3.2.2c", "The present subjunctive after cuando, que and para que", 1.25),
            ("3.2.2d", "Verb structures and negatives: acabar de, desde hace, seguir, the passive, lo bueno, superlatives", 1.25),
            ("3.2.3", "Sounds, spelling and where the stress falls", 1.0),
            ("4.4", "Paper 1 Listening: questions in English, distractors and dictation", 1.5),
            ("4.5", "Paper 2 Speaking: role-play, reading aloud and the photo card", 2.0),
            ("4.6", "Paper 3 Reading, and translating into English", 1.5),
            ("4.7", "Paper 4 Writing: translation into Spanish, the 90- and 150-word tasks", 2.0),
        ],
    },
    // AQA GCSE German 8662, the new specification: first taught September 2024,
    // first examined June 2026. AQA is the most-taken board for German: the June
    // 2026 entries in each board's own results statistics put AQA ahead of
    // Pearson Edexcel, with Eduqas far behind. Themes, grammar (3.2.1
    // Foundation, 3.2.2 Higher) and the papers read from the specification PDF.
    SubjectDef {
        id: "ger", name: "German", full: "AQA GCSE German (8662) Higher", color: "var(--ger)",
        papers: "Higher tier, four papers of 50 marks, 25% each, all sat in the same June: Listening 45 min (questions in English, then a dictation), Speaking 10-12 min (role-play, reading aloud, photo card), Reading 1 h (questions in English, then a translation into English), Writing 1 h 15 (a translation into German, a 90-word task and a 150-word task). Only the AQA vocabulary list and grammar are examined",
        spec: "https://www.aqa.org.uk/subjects/german/gcse/german-8662/specification",
        sections: &["Theme 1: People and lifestyle", "Theme 2: Popular culture", "Theme 3: Communication and the world around us", "Grammar", "Paper 1 Listening", "Paper 2 Speaking", "Paper 3 Reading", "Paper 4 Writing"],
        topics: &[
            ("3.1.1a", "Identity and relationships: family, friends and describing people", 1.5),
            ("3.1.1b", "Healthy living and lifestyle", 1.5),
            ("3.1.1c", "Education and work: school, jobs and future plans", 1.5),
            ("3.1.2a", "Free-time activities: sport, music, cinema, eating out", 1.5),
            ("3.1.2b", "Customs, festivals and celebrations", 1.5),
            ("3.1.2c", "Celebrity culture: role models, influencers and fame", 1.5),
            ("3.1.3a", "Travel and tourism, including places of interest", 1.5),
            ("3.1.3b", "Media and technology: phones, social media, TV and film", 1.5),
            ("3.1.3c", "The environment and where people live", 1.5),
            ("3.2.1a", "Nouns and cases: gender, plurals, compounds; der, ein and kein in three cases", 1.5),
            ("3.2.1b", "Determiners and pronouns: dieser, jeder, mein; mich and mir; relative der, die, das", 1.25),
            ("3.2.1c", "The present tense: weak and strong verbs, haben, sein, werden, wissen, and questions", 1.25),
            ("3.2.1d", "Word order: verb second, inversion, subordinate clauses, separable verbs and negation", 1.5),
            ("3.2.1e", "The perfect tense with haben and sein, plus war and hatte", 1.5),
            ("3.2.1f", "The future with werden, modal verbs and um ... zu", 1.25),
            ("3.2.1g", "Adjective endings, comparisons, and gern and lieber", 1.25),
            ("3.2.1h", "Prepositions and their cases, and word building: Lieblings-, un-, -ung", 1.0),
            ("3.2.2a", "Higher nouns and pronouns: weak nouns, das Gute, the genitive, dative pronouns, wo and was", 1.25),
            ("3.2.2b", "The simple past for stories, past modals, the imperative and seit", 1.25),
            ("3.2.2c", "Would and could: hätte, wäre, würde, sollte, and man instead of the passive", 1.25),
            ("3.2.2d", "Higher word order, two-way prepositions, da- and wo- compounds, and superlatives", 1.25),
            ("3.2.3", "Sounds, spelling and reading German aloud", 1.0),
            ("4.4", "Paper 1 Listening: questions in English, distractors and dictation", 1.5),
            ("4.5", "Paper 2 Speaking: role-play, reading aloud and the photo card", 2.0),
            ("4.6", "Paper 3 Reading, and translating into English", 1.5),
            ("4.7", "Paper 4 Writing: translation into German, the 90- and 150-word tasks", 2.0),
        ],
    },
    SubjectDef {
        id: "geog", name: "Geography", full: "AQA GCSE Geography (8035)", color: "var(--geog)",
        papers: "Paper 1 Living with the physical environment and Paper 2 Challenges in the human environment, each 1h30, 88 marks including 3 for SPaG, 35%. Paper 3 Geographical applications 1h30, 76 marks including 6 for SPaG, 30%, with a pre-release resource booklet 12 weeks before. Options here: hot deserts, coasts and rivers, food",
        spec: "https://www.aqa.org.uk/subjects/geography/gcse/geography-8035",
        sections: &["3.1.1 Natural hazards", "3.1.2 The living world", "3.1.3 Physical landscapes in the UK", "3.2.1 Urban issues and challenges", "3.2.2 The changing economic world", "3.2.3 Resource management", "3.3 Geographical applications", "3.4 Geographical skills"],
        topics: &[
            ("3.1.1.1", "Natural hazards and what makes the risk higher or lower", 0.5),
            ("3.1.1.2a", "Plate tectonics and what happens at each plate margin", 1.5),
            ("3.1.1.2b", "Earthquakes in contrasting countries: Chile 2010 and Nepal 2015", 1.5),
            ("3.1.1.2c", "Living with tectonic hazards: why people stay, and monitoring, prediction, protection and planning", 1.0),
            ("3.1.1.3a", "Global atmospheric circulation, and how tropical storms form", 1.5),
            ("3.1.1.3b", "Tropical storm effects and responses: Typhoon Haiyan 2013, and reducing the effects", 1.5),
            ("3.1.1.3c", "UK weather hazards and an extreme event: the Somerset Levels floods 2014", 1.0),
            ("3.1.1.4", "Climate change: evidence, causes, effects, mitigation and adaptation", 2.0),
            ("3.1.2.1", "Ecosystems: a small UK ecosystem, food webs, nutrient cycling and global biomes", 1.0),
            ("3.1.2.2a", "Tropical rainforests: characteristics, interdependence and adaptations", 1.0),
            ("3.1.2.2b", "Deforestation in Malaysia, and managing rainforests sustainably", 1.5),
            ("3.1.2.3a", "Hot deserts: characteristics, interdependence and adaptations", 1.0),
            ("3.1.2.3b", "The Sahara: opportunities and challenges, and desertification in the Sahel", 1.5),
            ("3.1.3.1", "The UK's physical landscapes: uplands, lowlands and river systems", 0.5),
            ("3.1.3.2a", "Coastal processes: waves, weathering, mass movement, erosion and longshore drift", 1.0),
            ("3.1.3.2b", "Coastal landforms of erosion and deposition: the Dorset coast at Swanage", 1.5),
            ("3.1.3.2c", "Coastal management: hard and soft engineering, managed retreat, and Lyme Regis", 1.5),
            ("3.1.3.3a", "River valleys: long and cross profiles, and fluvial processes", 1.0),
            ("3.1.3.3b", "River landforms from source to mouth: the River Tees", 1.5),
            ("3.1.3.3c", "Flood risk, hydrographs and flood management: the Banbury scheme", 1.5),
            ("3.2.1a", "Urbanisation: global patterns, push-pull migration and megacities", 1.0),
            ("3.2.1b", "Rio de Janeiro: opportunities and challenges of urban growth, and Favela Bairro", 2.0),
            ("3.2.1c", "Bristol: a major UK city and its opportunities and challenges", 2.0),
            ("3.2.1d", "Urban regeneration at Temple Quarter, sustainable urban living and urban transport", 1.5),
            ("3.2.2a", "Measuring development: classifications, indicators, their limits, and the DTM", 1.5),
            ("3.2.2b", "Uneven development and strategies to close the gap: tourism in Jamaica", 1.5),
            ("3.2.2c", "Nigeria: rapid economic development in an NEE", 2.0),
            ("3.2.2d", "The changing UK economy: post-industrial jobs, rural change, infrastructure and the north-south divide", 2.0),
            ("3.2.3.1", "Resource management: global inequality, and food, water and energy in the UK", 1.5),
            ("3.2.3.2a", "Food security and insecurity: patterns, causes and impacts", 1.0),
            ("3.2.3.2b", "Increasing food supply sustainably: Indus Basin Irrigation and Makueni, Kenya", 1.5),
            ("3.3.1", "Issue evaluation: using the pre-release booklet and making the 9-mark decision", 1.0),
            ("3.3.2", "Fieldwork: the six stages of a geographical enquiry", 1.5),
            ("3.4a", "Map skills: OS maps, grid references, scale, contours, cross-sections and photos", 1.5),
            ("3.4b", "Graphs, statistics and numbers in geography", 1.0),
        ],
    },
    SubjectDef {
        id: "hist", name: "History", full: "Pearson Edexcel GCSE History (1HI0)", color: "var(--hist)",
        papers: "Paper 1 Medicine in Britain and the Western Front, 1h20, 52 marks, 30%. Paper 2 the Cold War period study and Early Elizabethan England, 1h50, 64 marks, 40%. Paper 3 Weimar and Nazi Germany, 1h30, 52 marks, 30%. Plus 8 SPaG marks. Options here: 11, P4, B4 and 31",
        spec: "https://qualifications.pearson.com/en/qualifications/edexcel-gcses/history-2016.html",
        sections: &["11 Medicine in Britain, c1250-present", "11.5 The British sector of the Western Front, 1914-18", "B4 Early Elizabethan England, 1558-88", "P4 Superpower relations and the Cold War, 1941-91", "31 Weimar and Nazi Germany, 1918-39"],
        topics: &[
            ("11.1a", "Medieval medicine: supernatural, religious and rational ideas, the Four Humours, miasma and Galen", 1.0),
            ("11.1b", "Medieval prevention, treatment and care, and the Black Death 1348-49", 1.5),
            ("11.2a", "The Medical Renaissance: Vesalius, Sydenham, the printing press and the Royal Society", 1.5),
            ("11.2b", "Renaissance treatment and care, William Harvey, and the Great Plague of 1665", 1.5),
            ("11.3a", "Germ theory: Pasteur and Koch, and Jenner and vaccination", 1.5),
            ("11.3b", "Nightingale, anaesthetics, antiseptics, the 1875 Public Health Act, and Snow and cholera 1854", 2.0),
            ("11.4a", "Modern medicine: genetics, lifestyle, diagnosis, the NHS, magic bullets and penicillin", 2.0),
            ("11.4b", "High-tech treatment, prevention campaigns, and the fight against lung cancer", 1.5),
            ("11.5a", "The Western Front: the Ypres salient, the Somme, Arras and Cambrai, the trenches, and wounds", 1.5),
            ("11.5b", "The Western Front: the RAMC, the chain of evacuation, and new techniques in surgery", 1.5),
            ("11.5c", "Using sources on the Western Front: usefulness and planning a follow-up enquiry", 1.0),
            ("B4.1a", "Elizabeth's accession: England in 1558, legitimacy, gender and marriage, and the challenges she faced", 1.5),
            ("B4.1b", "The religious settlement of 1559 and the Church of England", 1.0),
            ("B4.1c", "The Puritan and Catholic challenges, and Mary, Queen of Scots 1568-69", 1.5),
            ("B4.2a", "Plots and revolts: the Northern Earls, Ridolfi, Throckmorton, Babington, Walsingham, and Mary's execution", 2.0),
            ("B4.2b", "Relations with Spain: rivalry, Drake and privateering, the Netherlands and Cadiz", 1.5),
            ("B4.2c", "The Spanish Armada, 1588", 1.0),
            ("B4.3a", "Elizabethan society: education, leisure, and the problem of the poor", 1.5),
            ("B4.3b", "Exploration, Drake's circumnavigation, and the attempted colonisation of Virginia", 1.5),
            ("P4.1a", "Origins of the Cold War: the Grand Alliance, the conferences, ideology, the bomb and the telegrams", 1.5),
            ("P4.1b", "The Cold War develops: Truman Doctrine, Marshall Plan, Cominform, Comecon, NATO and the Berlin Blockade", 1.5),
            ("P4.1c", "The Cold War intensifies: the arms race, the Warsaw Pact and the Hungarian Uprising 1956", 1.0),
            ("P4.2a", "Berlin 1958-63: refugees, Khrushchev's ultimatum, the summits, the Wall and Kennedy's visit", 1.5),
            ("P4.2b", "Cuba: the revolution, the Bay of Pigs, the Missile Crisis and its consequences", 1.5),
            ("P4.2c", "Czechoslovakia 1968: the Prague Spring and the Brezhnev Doctrine", 1.0),
            ("P4.3a", "Detente and its end: SALT 1, Helsinki, SALT 2, Afghanistan, the Carter Doctrine and the Olympic boycotts", 1.5),
            ("P4.3b", "The end of the Cold War: Reagan, SDI, Gorbachev, the fall of the Berlin Wall and the collapse of the USSR", 1.5),
            ("31.1a", "The origins of the Weimar Republic 1918-19 and the Weimar Constitution", 1.0),
            ("31.1b", "Early challenges 1919-23: Versailles, the stab in the back, Spartacists, Kapp, hyperinflation and the Ruhr", 1.5),
            ("31.1c", "The Golden Years: Stresemann, the Rentenmark, Dawes and Young, Locarno and the League", 1.5),
            ("31.1d", "Weimar society 1924-29: living standards, women and culture", 1.0),
            ("31.2a", "Hitler's early career, the Munich Putsch, and the Nazis' lean years 1924-28", 1.5),
            ("31.2b", "The Depression, the growth in Nazi support 1929-32, and how Hitler became Chancellor", 1.5),
            ("31.3a", "Creating a dictatorship 1933-34: the Reichstag Fire, the Enabling Act and the Night of the Long Knives", 1.5),
            ("31.3b", "The police state, propaganda, culture and the Churches", 1.5),
            ("31.3c", "Support, opposition and resistance: Niemoller, the Swing Youth and the Edelweiss Pirates", 1.0),
            ("31.4a", "Nazi policies towards women and the young", 1.5),
            ("31.4b", "Employment and living standards under the Nazis", 1.0),
            ("31.4c", "The persecution of minorities and of Jewish people", 1.0),
        ],
    },
    // Music - WJEC Eduqas GCSE (9-1) C660QS, the most-taken GCSE Music board
    // (June 2026 entries: Eduqas 14,367, Pearson 7,972, OCR 7,290, AQA 4,449).
    // Only Component 3 (Appraising) is a written exam. Topic codes are short
    // forms of the spec's own labels for Component 3 (section 2.3): ME musical
    // elements, MC musical contexts, ML musical language, AoS1-AoS4 the four
    // areas of study, with a letter suffix where a section is split.
    SubjectDef {
        id: "music", name: "Music", full: "Music (WJEC Eduqas C660QS)", color: "var(--music)",
        papers: "Component 3 Appraising: one listening exam, about 1h15, 96 marks, 40%. Eight 12-mark questions, two on each area of study; two are on the set works (Bach, Badinerie and Toto, Africa); one question contains a 10-mark extended answer and one a pitch or rhythm dictation. The real exam is played from a recording; the app cannot play it, so lessons teach the knowledge and vocabulary and point to videos for the listening. Component 1 Performing (30%) and Component 2 Composing (30%) are NEA and are not taught in the app",
        spec: "https://www.eduqas.co.uk/qualifications/music-gcse/",
        sections: &["ME Musical elements", "MC Musical contexts", "ML Musical language", "AoS1 Musical forms and devices", "AoS2 Music for ensemble", "AoS3 Film music", "AoS4 Popular music"],
        topics: &[
            ("MEa", "Melody: shape, intervals, scales and ornaments", 1.0),
            ("MEb", "Tonality and harmony: major, minor, modulation, dissonance and harmonic rhythm", 0.75),
            ("MEc", "Rhythm, metre and tempo", 0.75),
            ("MEd", "Sonority and dynamics: instruments, voices and how they are played", 1.0),
            ("MC", "Musical contexts: purpose, occasion, audience and venue", 0.5),
            ("MLa", "Reading and writing staff notation in treble and bass clef", 1.0),
            ("MLb", "Key signatures to four sharps and flats, major and relative minor", 0.75),
            ("MLc", "Chords: Roman numerals I to vi and chord symbols in a major key", 1.0),
            ("MLd", "Dictation: completing the pitch or rhythm of a short melody", 0.75),
            ("AoS1a", "The Western Classical Tradition 1650-1910: Baroque, Classical and Romantic", 0.75),
            ("AoS1b", "Forms: binary, ternary, minuet and trio, rondo, variation and strophic", 1.0),
            ("AoS1c", "Melodic and rhythmic devices: repetition, sequence, imitation, ostinato and more", 1.0),
            ("AoS1d", "Harmonic devices: cadences, chord progressions, pedal, drone, Alberti bass and modulation", 1.0),
            ("AoS1e", "Set work: Bach, Badinerie from Orchestral Suite No. 2", 1.5),
            ("AoS2a", "Texture: how composers combine musical lines", 1.0),
            ("AoS2b", "Chamber music: string quartet, basso continuo and the sonata", 0.75),
            ("AoS2c", "Musical theatre: solos, duets, trios, chorus and backing vocals", 0.75),
            ("AoS2d", "Jazz and blues: the jazz/blues trio, rhythm section and 12-bar blues", 0.75),
            ("AoS3a", "Film music: creating mood, responding to a commission, performers and audience", 1.0),
            ("AoS3b", "Leitmotif and thematic transformation", 0.75),
            ("AoS3c", "Timbre, dynamics, music technology and minimalism in film", 0.75),
            ("AoS4a", "Song structures: strophic, 32-bar, 12-bar blues, verse-chorus and their features", 1.0),
            ("AoS4b", "Voices, instruments and music technology in popular music", 1.0),
            ("AoS4c", "Styles: pop, rock and pop, bhangra and fusion", 0.75),
            ("AoS4d", "Set work: Toto, Africa", 1.5),
        ],
    },
    SubjectDef {
        id: "rs", name: "Religious Studies", full: "AQA GCSE Religious Studies A (8062)", color: "var(--rs)",
        papers: "Paper 1 Christianity and Islam, 1h45, 96 marks plus 6 SPaG, 50%. Paper 2A four themes, 1h45, 96 marks plus 3 SPaG, 50%. Every question is five parts worth 1, 2, 4, 5 and 12 marks. Themes here: A, B, D and E",
        spec: "https://www.aqa.org.uk/subjects/religious-studies/gcse/religious-studies-a-8062",
        sections: &["3.1.2 Christianity", "3.1.5 Islam", "3.2.1.1 Theme A: Relationships and families", "3.2.1.2 Theme B: Religion and life", "3.2.1.4 Theme D: Religion, peace and conflict", "3.2.1.5 Theme E: Religion, crime and punishment"],
        topics: &[
            ("3.1.2.1a", "Christian beliefs: the nature of God, the Trinity, the problem of evil, and creation", 1.5),
            ("3.1.2.1b", "Christian beliefs about the afterlife: resurrection, judgement, heaven and hell", 1.0),
            ("3.1.2.1c", "Jesus Christ and salvation: incarnation, crucifixion, resurrection, ascension, sin and atonement", 1.5),
            ("3.1.2.2a", "Christian worship and prayer, including the Lord's Prayer", 1.0),
            ("3.1.2.2b", "The sacraments: baptism and Holy Communion", 1.0),
            ("3.1.2.2c", "Pilgrimage to Lourdes and Iona, and the celebration of Christmas and Easter", 1.0),
            ("3.1.2.2d", "The Church in the local and worldwide community: food banks, mission, reconciliation, persecution and Christian Aid", 1.5),
            ("3.1.5.1a", "Islamic beliefs: the six articles and five roots, Tawhid, and the nature of God", 1.5),
            ("3.1.5.1b", "Angels, predestination and human freedom, and Akhirah", 1.0),
            ("3.1.5.1c", "Authority: Risalah, the holy books, and the imamate in Shi'a Islam", 1.0),
            ("3.1.5.2a", "The Five Pillars and the Ten Obligatory Acts; Shahadah and Salah", 1.5),
            ("3.1.5.2b", "Sawm, Zakah and Khums", 1.0),
            ("3.1.5.2c", "Hajj: the pilgrimage to Makkah", 1.0),
            ("3.1.5.2d", "Jihad, and the festivals of Id-ul-Adha, Id-ul-Fitr and Ashura", 1.0),
            ("3.2.1.1a", "Sex, marriage and divorce", 1.5),
            ("3.2.1.1b", "Families, the roles of men and women, and gender equality", 1.5),
            ("3.2.1.2a", "The origins and value of the universe, the environment, and the use of animals", 1.5),
            ("3.2.1.2b", "The origins and value of human life: sanctity, quality of life, abortion, euthanasia and the afterlife", 1.5),
            ("3.2.1.4a", "Peace, justice, violence, terrorism and war: just war, holy war and pacifism", 1.5),
            ("3.2.1.4b", "Religion and 21st-century conflict: nuclear weapons, WMD, peace-making and helping victims", 1.5),
            ("3.2.1.5a", "Crime and its causes: good and evil, reasons for crime, types of crime", 1.0),
            ("3.2.1.5b", "Punishment: its aims, prison, corporal punishment, community service, forgiveness and the death penalty", 1.5),
        ],
    },
    // AQA GCSE Drama 8261, specification version 1.8 (June 2026). AQA is the
    // most-taken board for Drama: 18,629 June 2026 entries against 12,840 for
    // Eduqas and 10,820 for Pearson. Only Component 1, the written paper, is
    // taught here. 3.1.2f-3.1.2n are the nine set plays on AQA's current list;
    // a student studies one and deletes the other eight in the Plan tab, which
    // leaves 16 h of study (24 h with all nine).
    SubjectDef {
        id: "drama", name: "Drama", full: "AQA GCSE Drama (8261)", color: "var(--drama)",
        papers: "Component 1 Understanding drama: one written exam, 1h45, open book with a clean copy of your set play, 80 marks, 40%. Section A multiple choice on theatre roles and terminology (4 marks), Section B four questions on an extract from your set play (44 marks), Section C one question from a choice on a live production you have seen (32 marks). The nine Set play topics cover every play on AQA's list: in the Plan tab, delete the eight you do not study. Component 2 Devising drama (devising log and devised performance, 40%) and Component 3 Texts in practice (two performed extracts, 20%) are practical, assessed in school and by an AQA examiner, and are not taught in the app",
        spec: "https://www.aqa.org.uk/subjects/drama/gcse/drama-8261/specification",
        sections: &["3.1.1 Knowledge and understanding", "3.1.2 Set play (Section B)", "3.1.3 Live theatre production (Section C)"],
        topics: &[
            // 3.1.1 Knowledge and understanding - Section A, and the toolkit for B and C
            ("3.1.1a", "Theatre makers: the twelve roles and what each is accountable for", 0.75),
            ("3.1.1b", "Stage positions and staging configurations", 0.75),
            ("3.1.1c", "Reading a play: genre, form, style, structure, language and stage directions", 0.75),
            ("3.1.1d", "Character, sub-text, motivation, mood, pace and climax", 0.75),
            ("3.1.1e", "Context, and the theatrical conventions of the period", 0.75),
            ("3.1.1f", "The performer: vocal and physical interpretation of character", 1.0),
            ("3.1.1g", "Performance conventions, use of space and the actor-audience relationship", 0.75),
            ("3.1.1h", "Design fundamentals, set and props", 1.0),
            ("3.1.1i", "Costume, hair, make-up and puppets", 0.75),
            ("3.1.1j", "Lighting and sound design", 1.0),
            // 3.1.2 Area of study 1: set play - the four Section B question types
            ("3.1.2a", "Section B: the design and context question (4 marks)", 0.5),
            ("3.1.2b", "Section B: performing a line (8 marks)", 0.75),
            ("3.1.2c", "Section B: space and interaction in the shaded section (12 marks)", 0.75),
            ("3.1.2d", "Section B: the 20-mark question as a performer", 1.0),
            ("3.1.2e", "Section B: the 20-mark question as a designer", 1.0),
            // 3.1.2 The nine set plays - keep only the one you study
            ("3.1.2f", "Set play: The Crucible (Arthur Miller)", 1.0),
            ("3.1.2g", "Set play: Blood Brothers (Willy Russell)", 1.0),
            ("3.1.2h", "Set play: Noughts and Crosses (Malorie Blackman, adapted by Dominic Cooke)", 1.0),
            ("3.1.2i", "Set play: Around the World in 80 Days (Jules Verne, adapted by Laura Eason)", 1.0),
            ("3.1.2j", "Set play: Things I Know to Be True (Andrew Bovell)", 1.0),
            ("3.1.2k", "Set play: Romeo and Juliet (William Shakespeare)", 1.0),
            ("3.1.2l", "Set play: A Taste of Honey (Shelagh Delaney)", 1.0),
            ("3.1.2m", "Set play: The Great Wave (Francis Turnly)", 1.0),
            ("3.1.2n", "Set play: The Empress (Tanika Gupta)", 1.0),
            // 3.1.3 Area of study 2: live theatre production - Section C
            ("3.1.3a", "Section C: seeing, noting and researching a live production", 0.75),
            ("3.1.3b", "Section C: analysing and evaluating performers (32 marks)", 1.0),
            ("3.1.3c", "Section C: analysing and evaluating design (32 marks)", 1.0),
        ],
    },
    SubjectDef {
        id: "pe", name: "PE", full: "Physical Education (AQA 8582)", color: "var(--pe)",
        papers: "Paper 1 The human body and movement in physical activity and sport (3.1: anatomy and physiology, movement analysis, physical training, use of data) and Paper 2 Socio-cultural influences and well-being in physical activity and sport (3.2: sports psychology, socio-cultural influences, health, fitness and well-being, use of data), each 1h15, 78 marks, 30%. Each paper mixes multiple choice, short answers and extended answers of 6 and 9 marks. The non-exam assessment (practical performance in three activities plus analysis and evaluation of a performance, 100 marks, 40%) is NOT taught in the app - it is done and assessed at school",
        spec: "https://www.aqa.org.uk/subjects/physical-education/gcse/physical-education-8582/specification",
        sections: &["3.1.1 Applied anatomy and physiology", "3.1.2 Movement analysis", "3.1.3 Physical training", "3.1.4 Use of data", "3.2.1 Sports psychology", "3.2.2 Socio-cultural influences", "3.2.3 Health, fitness and well-being"],
        topics: &[
            ("3.1.1.1a", "The skeleton: bones, functions, synovial joints and the movements they allow", 1.0),
            ("3.1.1.1b", "Muscles, tendons, antagonistic pairs and types of contraction", 0.75),
            ("3.1.1.2a", "Pathway of air, gaseous exchange and blood vessels", 0.75),
            ("3.1.1.2b", "The heart, cardiac cycle and cardiac output", 0.75),
            ("3.1.1.2c", "Mechanics of breathing and spirometer traces", 0.75),
            ("3.1.1.3", "Aerobic and anaerobic exercise, EPOC and recovery", 0.75),
            ("3.1.1.4", "The short- and long-term effects of exercise", 0.5),
            ("3.1.2.1", "Lever systems, mechanical advantage and analysing movement", 1.0),
            ("3.1.2.2", "Planes and axes of movement", 0.5),
            ("3.1.3.1", "Health, fitness and the role of exercise", 0.5),
            ("3.1.3.2a", "The components of fitness and the activities that need them", 0.75),
            ("3.1.3.2b", "Fitness testing: reasons, limitations, procedures and data", 1.0),
            ("3.1.3.3a", "The principles of training: SPORT and FITT", 0.75),
            ("3.1.3.3b", "Types of training and choosing a method", 1.0),
            ("3.1.3.4a", "Training intensities, one rep max and preventing injury", 0.75),
            ("3.1.3.4b", "High altitude training and the training seasons", 0.75),
            ("3.1.3.5", "Warming up and cooling down", 0.5),
            ("3.1.4.1", "Collecting data: quantitative and qualitative", 0.5),
            ("3.1.4.2", "Presenting data in tables and graphs", 0.5),
            ("3.1.4.3", "Analysing and evaluating data", 0.5),
            ("3.2.1.1", "Skill and ability, skill classification and types of goal", 0.75),
            ("3.2.1.2", "Goal setting and SMART targets", 0.5),
            ("3.2.1.3", "Basic information processing", 0.5),
            ("3.2.1.4", "Guidance and feedback on performance", 0.75),
            ("3.2.1.5a", "Arousal, the inverted-U theory and stress management", 0.75),
            ("3.2.1.5b", "Aggression, personality and motivation", 0.75),
            ("3.2.2.1", "Engagement patterns of different social groups", 0.75),
            ("3.2.2.2", "Commercialisation: sponsorship, the media and technology", 1.0),
            ("3.2.2.3a", "Conduct of performers and performance-enhancing drugs", 1.0),
            ("3.2.2.3b", "Spectator behaviour and hooliganism", 0.5),
            ("3.2.3.1", "Physical, emotional and social health, fitness and well-being", 0.5),
            ("3.2.3.2", "Sedentary lifestyles, obesity and somatotypes", 0.75),
            ("3.2.3.3", "Energy use, diet, nutrition and hydration", 0.75),
        ],
    },
    // WJEC Eduqas GCSE Media Studies (C680QS), specification version 10,
    // September 2025, read from the specification PDF. Eduqas is the
    // most-taken board for Media Studies: 19,791 June 2026 entries against
    // 5,484 for AQA and 3,588 for OCR. Set products are the list for exams
    // from 2028. Codes 2a-2e are the theoretical framework and contexts
    // (spec section 2), 2.1x Component 1 and 2.2x Component 2; the TV and
    // music topics marked "option" are the school's choice.
    SubjectDef {
        id: "media", name: "Media Studies", full: "Media Studies (WJEC Eduqas C680QS)", color: "var(--media)",
        papers: "Component 1 Exploring the Media, 1h30, 80 marks, 40%: Section A media language and representation in print (magazines, film posters, newspapers, print adverts), including a comparison with an unseen product; Section B industries and audiences (The Sun, Desert Island Discs, No Time to Die, Fortnite). Component 2 Understanding Media Forms and Products, 1h30, 60 marks, 30%: Section A television, starting from an extract of the set episode; Section B music videos and online media. Component 3 Creating Media Products is non-exam assessment (a production for a set brief) worth 30%, and is not taught in the app. Your school picks one TV option (Trigger Point with The Sweeney, or Man Like Mobeen or Modern Family with Friends) and one music video from each pair (Lizzo or Taylor Swift; Stormzy or Justin Bieber; Duran Duran or TLC): delete the topics for the options you don't study. Set products are those for exams from 2028",
        spec: "https://www.eduqas.co.uk/qualifications/media-studies-gcse/",
        sections: &["2 Theoretical framework and contexts", "2.1 Component 1: Exploring the Media", "2.2 Component 2: Understanding Media Forms and Products"],
        topics: &[
            // 2 The theoretical framework and the contexts of media
            ("2a", "Media language: semiotics, codes and conventions, genre and narrative", 1.25),
            ("2b", "Representation: selection, stereotypes, under-representation and feminist approaches", 1.25),
            ("2c", "Media industries: ownership, convergence, funding and regulation", 1.25),
            ("2d", "Audiences: targeting, categorising, uses and gratifications, active audiences", 1.0),
            ("2e", "Media contexts: historical, social, cultural and political", 0.75),
            // 2.1 Component 1, Section A: media language and representation (print)
            ("2.1a", "Magazine front covers: Vogue (July 2021) and GQ (August 2019)", 1.25),
            ("2.1b", "Film posters: The Man with the Golden Gun (1974) and No Time to Die (2021)", 1.25),
            ("2.1c", "Newspaper front pages: The Guardian (6 May 2025) and The Sun (22 March 2025)", 1.25),
            ("2.1d", "Print adverts: Quality Street (1956) and NHS 111 (2023)", 1.0),
            ("2.1e", "Section A skills: the context question and comparing with an unseen product", 0.75),
            // 2.1 Component 1, Section B: media industries and audiences
            ("2.1f", "The Sun: the newspaper industry and its audiences", 1.0),
            ("2.1g", "Desert Island Discs: the radio industry and its audiences", 0.75),
            ("2.1h", "No Time to Die: the film industry", 0.75),
            ("2.1i", "Fortnite: the video games industry and its audiences", 0.75),
            // 2.2 Component 2, Section A: television (one option)
            ("2.2a", "Television: crime drama and sitcom as genres, and the TV industry", 0.75),
            ("2.2b", "Crime drama option: Trigger Point (Series 2, Episode 1)", 1.0),
            ("2.2c", "Crime drama option: The Sweeney (1975 extract)", 0.5),
            ("2.2d", "Sitcom option: Man Like Mobeen (Series 1, Episode 2)", 1.0),
            ("2.2e", "Sitcom option: Modern Family (Season 8, Episode 2)", 1.0),
            ("2.2f", "Sitcom option: Friends (1994 extract)", 0.5),
            // 2.2 Component 2, Section B: music videos and online media (one from each pair)
            ("2.2g", "Music videos and the music industry", 0.75),
            ("2.2h", "Artist websites, and social and participatory media", 0.75),
            ("2.2i", "Music option: Lizzo, Good as Hell, and her website", 0.75),
            ("2.2j", "Music option: Taylor Swift, The Man, and her website", 0.75),
            ("2.2k", "Music option: Stormzy, Superheroes, and his website", 0.5),
            ("2.2l", "Music option: Justin Bieber, Intentions, and his website", 0.5),
            ("2.2m", "Music option: Duran Duran, Rio (1982)", 0.5),
            ("2.2n", "Music option: TLC, Waterfalls (1995)", 0.5),
        ],
    },
    // AQA GCSE Design and Technology 8552 (first exams June 2019). AQA is the
    // most-taken board: 51,236 June 2026 entries, against about 11k for Eduqas
    // and 9.6k for Pearson. One written paper plus the NEA; only the paper is
    // taught here. Section B questions let each student answer through the
    // material category they studied, so the 3.2 lessons cover all six.
    SubjectDef {
        id: "dt", name: "Design & Tech", full: "Design and Technology (AQA 8552)", color: "var(--dt)",
        papers: "One written paper, 2 h, 100 marks, 50%: Section A core technical principles (20 marks: 10 multiple choice, then short answers), Section B specialist technical principles (30 marks: short answers of 2-5 marks, several letting you pick your own material category, then one extended response), Section C designing and making principles (50 marks: short answers, calculations, drawing and extended responses). At least 15% of the marks are maths and 10% science; bring a calculator and a protractor. The NEA (a 30-35 hour design-and-make project against an AQA contextual challenge, a prototype plus a portfolio, 100 marks, 50%) is not taught in the app",
        spec: "https://www.aqa.org.uk/subjects/design-and-technology/gcse/design-and-technology-8552/specification",
        sections: &["3.1 Core technical principles", "3.2 Specialist technical principles", "3.3 Designing and making principles"],
        topics: &[
            ("3.1.1a", "New technologies in industry, enterprise and production: automation, CAD/CAM, FMS, JIT and lean", 0.75),
            ("3.1.1b", "Technology and society: sustainability, people, culture, environment and planned obsolescence", 0.75),
            ("3.1.2", "Energy generation and storage: fossil fuels, nuclear, renewables and batteries", 0.75),
            ("3.1.3", "Developments in new materials: modern, smart and composite materials, technical textiles", 0.75),
            ("3.1.4", "Systems approach to designing: inputs, processes and outputs", 0.5),
            ("3.1.5", "Mechanical devices: movement, levers, linkages, cams, gears and pulleys", 1.0),
            ("3.1.6.1a", "Material categories: papers and boards, and natural and manufactured timbers", 0.75),
            ("3.1.6.1b", "Material categories: metals and alloys, polymers and textiles", 1.0),
            ("3.1.6.2", "Material properties: physical and working properties", 0.5),
            ("3.2.1", "Selecting materials and components: the factors a designer weighs", 0.5),
            ("3.2.2", "Forces and stresses, and how materials are reinforced and stiffened", 0.5),
            ("3.2.3", "Ecological and social footprint, and the six Rs", 0.75),
            ("3.2.4", "Sources and origins: from raw material to workable form, and life cycle assessment", 1.0),
            ("3.2.5a", "Using materials: properties in commercial products, and modifying properties", 1.0),
            ("3.2.5b", "Shaping and forming materials by cutting, abrasion and addition", 0.75),
            ("3.2.6", "Stock forms, types and sizes, and calculating quantities", 0.75),
            ("3.2.7", "Scales of production: prototype, batch, mass and continuous", 0.75),
            ("3.2.8a", "Production aids, and processes of wastage, addition, deforming and reforming", 1.0),
            ("3.2.8b", "Tolerances, commercial processes and quality control", 1.0),
            ("3.2.9", "Surface treatments and finishes", 0.75),
            ("3.3.1", "Investigation: primary and secondary data, anthropometrics, the brief and specification", 1.0),
            ("3.3.2", "Environmental, social and economic challenges in design", 0.5),
            ("3.3.3", "The work of others: designers and companies", 0.75),
            ("3.3.4", "Design strategies: collaboration, user-centred, systems, iterative, avoiding fixation", 0.5),
            ("3.3.5", "Communicating design ideas: sketching, orthographic, exploded views and modelling", 1.0),
            ("3.3.6", "Prototype development and evaluation", 0.5),
            ("3.3.7", "Selecting materials and components for a prototype", 0.5),
            ("3.3.8", "Tolerances in making", 0.5),
            ("3.3.9", "Material management: nesting, waste, marking out and datums", 0.75),
            ("3.3.10", "Specialist tools and equipment, and working safely", 0.5),
            ("3.3.11", "Specialist techniques and processes, and applying finishes", 0.5),
        ],
    },
    // AQA GCSE Food Preparation and Nutrition 8585 (first taught 2016, first
    // examined 2018). AQA is the most-taken board for this subject: 31,938 June
    // 2026 entries against 18,117 for Eduqas and 4,562 for OCR. Topics are the
    // spec's own references for sections 3.2-3.6, read from the specification
    // PDF (version 1.1, 21 January 2019). The twelve skill groups in 3.1 are
    // folded into the topics they illustrate; 3.7 is assessed only by the NEA.
    SubjectDef {
        id: "food", name: "Food", full: "Food Preparation and Nutrition (AQA 8585)", color: "var(--food)",
        papers: "One written paper, 1h45, 100 marks, 50%: 20 marks of multiple choice, then five questions worth 80 marks between them, ending in 8- and 12-mark analyse-and-evaluate answers. The NEA is the other 50% and is not taught in the app: Task 1 food investigation (30 marks) and Task 2 food preparation assessment (70 marks, three dishes cooked in 3 hours plus a portfolio)",
        spec: "https://www.aqa.org.uk/subjects/food-preparation-and-nutrition/gcse/food-preparation-and-nutrition-8585/specification",
        sections: &["3.2 Food, nutrition and health", "3.3 Food science", "3.4 Food safety", "3.5 Food choice", "3.6 Food provenance"],
        topics: &[
            // 3.2 Food, nutrition and health
            ("3.2.1.1", "Protein: biological value, complementation, alternatives and needs", 0.75),
            ("3.2.1.2", "Fats: saturated and unsaturated, functions, sources and limits", 0.5),
            ("3.2.1.3", "Carbohydrates: starch, sugars and dietary fibre", 0.75),
            ("3.2.2.1a", "Fat-soluble vitamins A, D, E and K", 0.5),
            ("3.2.2.1b", "Water-soluble vitamins, cooking losses and antioxidants", 0.75),
            ("3.2.2.2", "Minerals: calcium, iron, sodium, fluoride, iodine and phosphorus", 0.5),
            ("3.2.2.3", "Water and hydration", 0.5),
            ("3.2.3.1a", "Healthy eating guidelines, the Eatwell Guide, portions and costing", 0.75),
            ("3.2.3.1b", "Planning diets for life stages and dietary groups", 0.5),
            ("3.2.3.2", "Energy needs: BMR, PAL and energy from nutrients", 0.5),
            ("3.2.3.3", "Nutritional analysis and modifying recipes", 0.5),
            ("3.2.3.4", "Diet, nutrition and health: the diet-related diseases", 0.75),
            // 3.3 Food science
            ("3.3.1.1", "Why food is cooked, and conduction, convection and radiation", 0.5),
            ("3.3.1.2", "Selecting cooking methods: water, dry heat and fat based", 0.5),
            ("3.3.2.1", "Proteins: denaturation, coagulation, gluten and foams", 1.0),
            ("3.3.2.2", "Carbohydrates: gelatinisation, dextrinisation and caramelisation", 0.75),
            ("3.3.2.3", "Fats and oils: shortening, aeration, plasticity and emulsification", 0.75),
            ("3.3.2.4", "Fruit and vegetables: enzymic browning and oxidation", 0.5),
            ("3.3.2.5", "Raising agents: chemical, mechanical, steam and yeast", 0.75),
            // 3.4 Food safety
            ("3.4.1.1", "Microorganisms, enzymes and controlling spoilage", 0.75),
            ("3.4.1.2", "The signs of food spoilage", 0.5),
            ("3.4.1.3", "Microorganisms in food production: bread, cheese and yoghurt", 0.5),
            ("3.4.1.4", "Bacterial contamination and the food poisoning bacteria", 0.75),
            ("3.4.2.1", "Buying and storing food: temperatures, date marks and fridges", 0.75),
            ("3.4.2.2", "Preparing, cooking and serving food safely", 0.5),
            // 3.5 Food choice
            ("3.5.1.1", "Factors which influence food choice, and costing recipes", 0.5),
            ("3.5.1.2a", "Food choice, religion and culture", 0.5),
            ("3.5.1.2b", "Ethical and moral choices, intolerances and allergies", 0.75),
            ("3.5.1.3", "Food labelling and marketing influences", 0.5),
            ("3.5.2", "British and international cuisines", 0.5),
            ("3.5.3", "Sensory evaluation and taste panels", 0.75),
            // 3.6 Food provenance
            ("3.6.1.1", "Food sources: grown, reared and caught; farming methods", 0.5),
            ("3.6.1.2", "Food and the environment: food miles, waste and packaging", 0.5),
            ("3.6.1.3", "Sustainability of food and food security", 0.5),
            ("3.6.2.1", "Primary and secondary processing, and its effects", 0.75),
            ("3.6.2.2", "Fortification, modified foods, additives and GM", 0.5),
        ],
    },
    SubjectDef {
        id: "maths_edx", name: "Maths", full: "Pearson Edexcel GCSE Mathematics (1MA1) Higher", color: "var(--maths)",
        papers: "Three papers, 1h30 and 80 marks each: Paper 1 non-calculator, Papers 2 and 3 calculator. Higher tier, grades 4 to 9",
        spec: "https://qualifications.pearson.com/en/qualifications/edexcel-gcses/mathematics-2015.html",
        sections: &["N Number", "A Algebra", "R Ratio, proportion and rates of change", "G Geometry and measures", "P Probability", "S Statistics"],
        topics: &[
            ("N1-3", "Ordering numbers, the four operations and inverse operations", 0.5),
            ("N4-5", "Primes, factors, multiples, HCF and LCM, and systematic listing", 1.0),
            ("N6-7", "Powers, roots, and integer and fractional indices", 1.5),
            ("N8", "Calculating exactly with fractions, surds and multiples of pi", 1.5),
            ("N9", "Standard form", 1.0),
            ("N10-12", "Fractions, decimals and percentages, including recurring decimals", 1.0),
            ("N13-16", "Units, estimation, rounding, and upper and lower bounds", 1.5),
            ("A1-3", "Algebraic notation, substitution, and expressions, equations and identities", 0.5),
            ("A4", "Expanding, factorising and algebraic fractions", 2.0),
            ("A5-6", "Rearranging formulae, identities and algebraic proof", 1.5),
            ("A7", "Functions, including composite and inverse functions", 1.0),
            ("A8-10", "Coordinates, straight-line graphs, gradients and intercepts", 1.5),
            ("A11-12", "Quadratic, cubic, reciprocal, exponential and trigonometric graphs", 2.0),
            ("A13", "Transforming graphs: translations and reflections", 1.0),
            ("A14-15", "Real-life graphs, gradients of curves and areas under graphs", 1.5),
            ("A16", "The equation of a circle and the tangent at a point", 1.0),
            ("A17", "Linear equations", 0.5),
            ("A18", "Quadratic equations: factorising, completing the square and the formula", 2.0),
            ("A19", "Simultaneous equations, linear and quadratic", 1.5),
            ("A20", "Iteration", 1.0),
            ("A21-22", "Forming equations, and linear and quadratic inequalities", 1.5),
            ("A23-25", "Sequences: term-to-term, special sequences and the nth term", 1.5),
            ("R1-2", "Converting units, scale factors, scale diagrams and maps", 0.5),
            ("R3-8", "Ratio and proportion", 1.5),
            ("R9", "Percentages, percentage change and reverse percentages", 1.5),
            ("R10", "Direct and inverse proportion problems", 1.0),
            ("R11", "Compound units: speed, density, pressure and rates", 1.0),
            ("R12", "Comparing lengths, areas and volumes using ratio", 0.5),
            ("R13", "Proportion as equations and graphs", 1.0),
            ("R14-15", "Rates of change, and gradients as rates", 1.0),
            ("R16", "Growth, decay and compound interest", 1.0),
            ("G1-2", "Constructions and loci", 1.0),
            ("G3-4", "Angles, parallel lines, polygons and quadrilaterals", 1.5),
            ("G5-6", "Congruent triangles and geometric proof", 1.0),
            ("G7-8", "Transformations: reflection, rotation, translation and enlargement", 1.5),
            ("G9-10", "Circle definitions and the circle theorems", 2.0),
            ("G11-15", "Coordinate geometry, 3D shapes, plans and elevations, measures and bearings", 1.5),
            ("G16-18", "Area, perimeter, volume and surface area, arcs and sectors", 2.0),
            ("G19", "Similarity: lengths, areas and volumes of similar shapes", 1.0),
            ("G20-21", "Pythagoras' theorem, trigonometry and exact trig values", 2.0),
            ("G22-23", "Sine rule, cosine rule, the area of a triangle and 3D problems", 2.0),
            ("G24-25", "Vectors and vector proof", 1.5),
            ("P1-5", "Probability, relative frequency and expected outcomes", 1.0),
            ("P6-8", "Sample spaces, Venn diagrams and tree diagrams", 1.5),
            ("P9", "Conditional probability", 1.0),
            ("S1", "Populations and samples", 0.5),
            ("S2-4", "Charts, histograms, cumulative frequency, box plots, averages and spread", 2.0),
            ("S5-6", "Describing a population, scatter graphs and correlation", 1.0),
        ],
    },
    SubjectDef {
        id: "maths_aqa", name: "Maths", full: "AQA GCSE Mathematics (8300) Higher", color: "var(--maths)",
        papers: "Three papers, 1h30 and 80 marks each: Paper 1 non-calculator, Papers 2 and 3 calculator. Higher tier, grades 4 to 9",
        spec: "https://www.aqa.org.uk/subjects/mathematics/gcse/mathematics-8300",
        sections: &["N Number", "A Algebra", "R Ratio, proportion and rates of change", "G Geometry and measures", "P Probability", "S Statistics"],
        topics: &[
            ("N1-3", "Ordering numbers, the four operations and inverse operations", 0.5),
            ("N4-5", "Primes, factors, multiples, HCF and LCM, and systematic listing", 1.0),
            ("N6-7", "Powers, roots, and integer and fractional indices", 1.5),
            ("N8", "Calculating exactly with fractions, surds and multiples of pi", 1.5),
            ("N9", "Standard form", 1.0),
            ("N10-12", "Fractions, decimals and percentages, including recurring decimals", 1.0),
            ("N13-16", "Units, estimation, rounding, and upper and lower bounds", 1.5),
            ("A1-3", "Algebraic notation, substitution, and expressions, equations and identities", 0.5),
            ("A4", "Expanding, factorising and algebraic fractions", 2.0),
            ("A5-6", "Rearranging formulae, identities and algebraic proof", 1.5),
            ("A7", "Functions, including composite and inverse functions", 1.0),
            ("A8-10", "Coordinates, straight-line graphs, gradients and intercepts", 1.5),
            ("A11-12", "Quadratic, cubic, reciprocal, exponential and trigonometric graphs", 2.0),
            ("A13", "Transforming graphs: translations and reflections", 1.0),
            ("A14-15", "Real-life graphs, gradients of curves and areas under graphs", 1.5),
            ("A16", "The equation of a circle and the tangent at a point", 1.0),
            ("A17", "Linear equations", 0.5),
            ("A18", "Quadratic equations: factorising, completing the square and the formula", 2.0),
            ("A19", "Simultaneous equations, linear and quadratic", 1.5),
            ("A20", "Iteration", 1.0),
            ("A21-22", "Forming equations, and linear and quadratic inequalities", 1.5),
            ("A23-25", "Sequences: term-to-term, special sequences and the nth term", 1.5),
            ("R1-2", "Converting units, scale factors, scale diagrams and maps", 0.5),
            ("R3-8", "Ratio and proportion", 1.5),
            ("R9", "Percentages, percentage change and reverse percentages", 1.5),
            ("R10", "Direct and inverse proportion problems", 1.0),
            ("R11", "Compound units: speed, density, pressure and rates", 1.0),
            ("R12", "Comparing lengths, areas and volumes using ratio", 0.5),
            ("R13", "Proportion as equations and graphs", 1.0),
            ("R14-15", "Rates of change, and gradients as rates", 1.0),
            ("R16", "Growth, decay and compound interest", 1.0),
            ("G1-2", "Constructions and loci", 1.0),
            ("G3-4", "Angles, parallel lines, polygons and quadrilaterals", 1.5),
            ("G5-6", "Congruent triangles and geometric proof", 1.0),
            ("G7-8", "Transformations: reflection, rotation, translation and enlargement", 1.5),
            ("G9-10", "Circle definitions and the circle theorems", 2.0),
            ("G11-15", "Coordinate geometry, 3D shapes, plans and elevations, measures and bearings", 1.5),
            ("G16-18", "Area, perimeter, volume and surface area, arcs and sectors", 2.0),
            ("G19", "Similarity: lengths, areas and volumes of similar shapes", 1.0),
            ("G20-21", "Pythagoras' theorem, trigonometry and exact trig values", 2.0),
            ("G22-23", "Sine rule, cosine rule, the area of a triangle and 3D problems", 2.0),
            ("G24-25", "Vectors and vector proof", 1.5),
            ("P1-5", "Probability, relative frequency and expected outcomes", 1.0),
            ("P6-8", "Sample spaces, Venn diagrams and tree diagrams", 1.5),
            ("P9", "Conditional probability", 1.0),
            ("S1", "Populations and samples", 0.5),
            ("S2-4", "Charts, histograms, cumulative frequency, box plots, averages and spread", 2.0),
            ("S5-6", "Describing a population, scatter graphs and correlation", 1.0),
        ],
    },
    SubjectDef {
        id: "maths_ocr", name: "Maths", full: "OCR GCSE Mathematics (J560) Higher", color: "var(--maths)",
        papers: "Three papers, 1h30 and 100 marks each: Papers 4 and 6 calculator, Paper 5 non-calculator. Higher tier, grades 4 to 9",
        spec: "https://www.ocr.org.uk/qualifications/gcse/mathematics-j560-from-2015/",
        sections: &["N Number", "A Algebra", "R Ratio, proportion and rates of change", "G Geometry and measures", "P Probability", "S Statistics"],
        topics: &[
            ("N1-3", "Ordering numbers, the four operations and inverse operations", 0.5),
            ("N4-5", "Primes, factors, multiples, HCF and LCM, and systematic listing", 1.0),
            ("N6-7", "Powers, roots, and integer and fractional indices", 1.5),
            ("N8", "Calculating exactly with fractions, surds and multiples of pi", 1.5),
            ("N9", "Standard form", 1.0),
            ("N10-12", "Fractions, decimals and percentages, including recurring decimals", 1.0),
            ("N13-16", "Units, estimation, rounding, and upper and lower bounds", 1.5),
            ("A1-3", "Algebraic notation, substitution, and expressions, equations and identities", 0.5),
            ("A4", "Expanding, factorising and algebraic fractions", 2.0),
            ("A5-6", "Rearranging formulae, identities and algebraic proof", 1.5),
            ("A7", "Functions, including composite and inverse functions", 1.0),
            ("A8-10", "Coordinates, straight-line graphs, gradients and intercepts", 1.5),
            ("A11-12", "Quadratic, cubic, reciprocal, exponential and trigonometric graphs", 2.0),
            ("A13", "Transforming graphs: translations and reflections", 1.0),
            ("A14-15", "Real-life graphs, gradients of curves and areas under graphs", 1.5),
            ("A16", "The equation of a circle and the tangent at a point", 1.0),
            ("A17", "Linear equations", 0.5),
            ("A18", "Quadratic equations: factorising, completing the square and the formula", 2.0),
            ("A19", "Simultaneous equations, linear and quadratic", 1.5),
            ("A20", "Iteration", 1.0),
            ("A21-22", "Forming equations, and linear and quadratic inequalities", 1.5),
            ("A23-25", "Sequences: term-to-term, special sequences and the nth term", 1.5),
            ("R1-2", "Converting units, scale factors, scale diagrams and maps", 0.5),
            ("R3-8", "Ratio and proportion", 1.5),
            ("R9", "Percentages, percentage change and reverse percentages", 1.5),
            ("R10", "Direct and inverse proportion problems", 1.0),
            ("R11", "Compound units: speed, density, pressure and rates", 1.0),
            ("R12", "Comparing lengths, areas and volumes using ratio", 0.5),
            ("R13", "Proportion as equations and graphs", 1.0),
            ("R14-15", "Rates of change, and gradients as rates", 1.0),
            ("R16", "Growth, decay and compound interest", 1.0),
            ("G1-2", "Constructions and loci", 1.0),
            ("G3-4", "Angles, parallel lines, polygons and quadrilaterals", 1.5),
            ("G5-6", "Congruent triangles and geometric proof", 1.0),
            ("G7-8", "Transformations: reflection, rotation, translation and enlargement", 1.5),
            ("G9-10", "Circle definitions and the circle theorems", 2.0),
            ("G11-15", "Coordinate geometry, 3D shapes, plans and elevations, measures and bearings", 1.5),
            ("G16-18", "Area, perimeter, volume and surface area, arcs and sectors", 2.0),
            ("G19", "Similarity: lengths, areas and volumes of similar shapes", 1.0),
            ("G20-21", "Pythagoras' theorem, trigonometry and exact trig values", 2.0),
            ("G22-23", "Sine rule, cosine rule, the area of a triangle and 3D problems", 2.0),
            ("G24-25", "Vectors and vector proof", 1.5),
            ("P1-5", "Probability, relative frequency and expected outcomes", 1.0),
            ("P6-8", "Sample spaces, Venn diagrams and tree diagrams", 1.5),
            ("P9", "Conditional probability", 1.0),
            ("S1", "Populations and samples", 0.5),
            ("S2-4", "Charts, histograms, cumulative frequency, box plots, averages and spread", 2.0),
            ("S5-6", "Describing a population, scatter graphs and correlation", 1.0),
        ],
    },
    SubjectDef {
        id: "bio_aqa", name: "Biology", full: "AQA GCSE Biology (8461) Higher", color: "var(--bio)",
        papers: "Paper 1 (topics 4.1-4.4) and Paper 2 (topics 4.5-4.7), each 1h45, 100 marks, 50%. Multiple choice, structured, closed short answer and open response, including the 10 required practicals",
        spec: "https://www.aqa.org.uk/subjects/biology/gcse/biology-8461",
        sections: &["4.1 Cell biology", "4.2 Organisation", "4.3 Infection and response", "4.4 Bioenergetics", "4.5 Homeostasis and response", "4.6 Inheritance, variation and evolution", "4.7 Ecology"],
        topics: &[
            ("4.1.1", "Cell structure: eukaryotes, prokaryotes, specialisation, microscopy and culturing microorganisms", 2.0),
            ("4.1.2", "Cell division: chromosomes, mitosis, the cell cycle and stem cells", 1.5),
            ("4.1.3", "Transport in cells: diffusion, osmosis and active transport", 1.5),
            ("4.2.1", "Principles of organisation", 0.5),
            ("4.2.2a", "The digestive system and enzymes", 1.5),
            ("4.2.2b", "The heart, blood vessels and blood, and the lungs", 1.5),
            ("4.2.2c", "Coronary heart disease, health issues, lifestyle and cancer", 1.5),
            ("4.2.3", "Plant tissues, organs and systems: transpiration and translocation", 1.5),
            ("4.3.1a", "Communicable diseases: pathogens, and viral, bacterial, fungal and protist diseases", 1.5),
            ("4.3.1b", "Human defences, vaccination, antibiotics, painkillers and drug development", 1.5),
            ("4.3.2", "Monoclonal antibodies", 1.0),
            ("4.3.3", "Plant disease and plant defences", 1.0),
            ("4.4.1", "Photosynthesis: the reaction, rate and limiting factors, and uses of glucose", 1.5),
            ("4.4.2", "Respiration: aerobic and anaerobic, exercise and metabolism", 1.5),
            ("4.5.1", "Homeostasis", 0.5),
            ("4.5.2", "The nervous system, reflexes, the brain and the eye", 2.0),
            ("4.5.3a", "Hormones: the endocrine system, blood glucose and diabetes, and water and nitrogen balance", 2.0),
            ("4.5.3b", "Hormones in reproduction, contraception, infertility, adrenaline, thyroxine and negative feedback", 1.5),
            ("4.5.4", "Plant hormones: auxins, gibberellins, ethene and their uses", 1.0),
            ("4.6.1a", "Sexual and asexual reproduction, meiosis, DNA and the genome, and protein synthesis", 2.0),
            ("4.6.1b", "Genetic inheritance, inherited disorders and sex determination", 1.5),
            ("4.6.2", "Variation, evolution, selective breeding, genetic engineering and cloning", 2.0),
            ("4.6.3", "The development of genetics and evolution: Darwin, Mendel, speciation and resistant bacteria", 1.5),
            ("4.6.4", "Classification of living organisms", 0.5),
            ("4.7.1", "Adaptations, interdependence and competition", 1.0),
            ("4.7.2", "Organisation of an ecosystem: food chains, sampling, and the carbon and water cycles", 1.5),
            ("4.7.3", "Biodiversity and the effect of human interaction on ecosystems", 1.5),
            ("4.7.4", "Trophic levels, pyramids of biomass and biomass transfer", 1.0),
            ("4.7.5", "Food production and food security", 1.0),
        ],
    },
    // WJEC Eduqas GCSE English Language C700QS, specification "Version 3
    // January 2019" (teaching from 2015, award from 2017), read 30 September
    // 2026. The spec has no numbered content statements: 2.1 is Component 1 and
    // 2.2 is Component 2, each split into its Section A questions and its
    // Section B writing. 2.3 Component 3 (Spoken Language) is a separately
    // reported endorsement and is not taught here.
    SubjectDef {
        id: "englang_edq", name: "Eng Language", full: "Eduqas GCSE English Language (C700QS)", color: "var(--englang)",
        papers: "Component 1 20th Century Literature Reading and Creative Prose Writing: 1h45, 80 marks, 40% (Section A five questions on one unseen 20th-century prose extract, 40 marks; Section B one story from a choice of four titles, 40 marks). Component 2 19th and 21st Century Non-Fiction Reading and Transactional/Persuasive Writing: 2h, 80 marks, 60% (Section A six questions on two unseen non-fiction texts, 40 marks; Section B two compulsory 20-mark writing tasks). Component 3 Spoken Language is a presentation assessed by your teacher and reported separately as Pass, Merit or Distinction; it is compulsory but does not count towards the 9-1 grade, and it is not taught in the app",
        spec: "https://www.eduqas.co.uk/qualifications/english-language-gcse/",
        sections: &["2.1 Component 1: 20th-century literature reading and creative prose writing", "2.2 Component 2: 19th- and 21st-century non-fiction reading and transactional writing"],
        topics: &[
            // 2.1 Component 1, Section A - reading one unseen 20th-century prose extract (40 marks)
            ("2.1a", "Reading an unseen 20th-century prose extract: approach and timing", 0.75),
            ("2.1b", "A1: listing five things, explicit and implicit (5 marks)", 0.5),
            ("2.1c", "A2: impressions from language in a short section (5 marks)", 1.0),
            ("2.1d", "A3: how language shows character, feeling or atmosphere (10 marks)", 1.5),
            ("2.1e", "A4: language and structure together (10 marks)", 1.5),
            ("2.1f", "A5: evaluating a view of the whole passage (10 marks)", 1.5),
            // 2.1 Component 1, Section B - creative prose writing (40 marks)
            ("2.1g", "Section B: choosing a title and planning a story (40 marks)", 1.0),
            ("2.1h", "Section B: narrative craft - voice, character, detail and pace", 1.5),
            ("2.1i", "Section B: vocabulary, sentences, punctuation and spelling", 1.0),
            // 2.2 Component 2, Section A - reading a 21st- and a 19th-century non-fiction text (40 marks)
            ("2.2a", "Reading 19th- and 21st-century non-fiction: forms, voice and older prose", 1.0),
            ("2.2b", "A1 and A3: short retrieval questions (1 mark each)", 0.5),
            ("2.2c", "A2: how the writer tries to achieve an aim - what, how, tone (10 marks)", 1.5),
            ("2.2d", "A4: evaluating a statement about the 19th-century text (10 marks)", 1.5),
            ("2.2e", "A5: synthesising information from both texts (4 marks)", 0.75),
            ("2.2f", "A6: comparing both texts - ideas and methods (10 marks)", 1.5),
            // 2.2 Component 2, Section B - two transactional/persuasive tasks (2 x 20 marks)
            ("2.2g", "Section B: letters, articles, reviews, talks, reports and guides", 1.5),
            ("2.2h", "Section B: persuading, arguing and advising with rhetoric", 1.0),
            ("2.2i", "Section B: two 20-mark tasks in an hour - planning, register and accuracy", 1.0),
        ],
    },
    SubjectDef {
        id: "chem_aqa", name: "Chemistry", full: "AQA GCSE Chemistry (8462) Higher", color: "var(--chem)",
        papers: "Paper 1 (topics 4.1-4.5) and Paper 2 (topics 4.6-4.10), each 1h45, 100 marks, 50%. Multiple choice, structured, closed short answer and open response, including the 8 required practicals",
        spec: "https://www.aqa.org.uk/subjects/chemistry/gcse/chemistry-8462",
        sections: &["4.1 Atomic structure and the periodic table", "4.2 Bonding, structure and the properties of matter", "4.3 Quantitative chemistry", "4.4 Chemical changes", "4.5 Energy changes", "4.6 The rate and extent of chemical change", "4.7 Organic chemistry", "4.8 Chemical analysis", "4.9 Chemistry of the atmosphere", "4.10 Using resources"],
        topics: &[
            ("4.1.1", "Atoms, elements, compounds, mixtures, the atomic model and electronic structure", 1.5),
            ("4.1.2", "The periodic table: groups 0, 1 and 7", 1.5),
            ("4.1.3", "Properties of transition metals", 0.5),
            ("4.2.1", "Ionic, covalent and metallic bonding", 1.5),
            ("4.2.2", "How bonding and structure relate to properties: states of matter and the three structure types", 1.5),
            ("4.2.3", "Structure and bonding of carbon: diamond, graphite, graphene and fullerenes", 1.0),
            ("4.2.4", "Nanoparticles: bulk and surface properties", 0.5),
            ("4.3.1", "Conservation of mass, relative formula mass and uncertainty", 1.0),
            ("4.3.2", "Moles, reacting masses, limiting reactants and balancing from masses", 2.0),
            ("4.3.3", "Yield and atom economy", 1.0),
            ("4.3.4", "Concentration of solutions and titrations", 1.5),
            ("4.3.5", "Volumes of gases", 0.5),
            ("4.4.1", "Reactivity of metals: oxidation, reduction, extraction and ionic equations", 1.5),
            ("4.4.2", "Reactions of acids: salts, neutralisation, pH, and strong and weak acids", 1.5),
            ("4.4.3", "Electrolysis", 2.0),
            ("4.5.1", "Exothermic and endothermic reactions, reaction profiles and bond energies", 1.5),
            ("4.5.2", "Chemical cells and fuel cells", 1.0),
            ("4.6.1", "Rate of reaction: measuring it, the factors that affect it, collision theory and catalysts", 2.0),
            ("4.6.2", "Reversible reactions and dynamic equilibrium", 1.5),
            ("4.7.1", "Crude oil, hydrocarbons, fractional distillation and cracking", 1.5),
            ("4.7.2", "Reactions of alkenes and alcohols, and carboxylic acids", 1.5),
            ("4.7.3", "Synthetic and natural polymers", 1.0),
            ("4.8.1", "Purity, formulations and chromatography", 1.0),
            ("4.8.2", "Identification of common gases", 0.5),
            ("4.8.3", "Identification of ions: flame tests, hydroxide precipitates, anion tests and instrumental methods", 1.5),
            ("4.9.1", "The composition and evolution of the Earth's atmosphere", 1.0),
            ("4.9.2", "Carbon dioxide and methane as greenhouse gases, and climate change", 1.0),
            ("4.9.3", "Common atmospheric pollutants and their sources", 0.5),
            ("4.10.1", "Using the Earth's resources, potable water, waste water and alternative metal extraction", 1.5),
            ("4.10.2", "Life cycle assessment and recycling", 0.5),
            ("4.10.3", "Using materials: corrosion, alloys, ceramics, polymers and composites", 1.0),
            ("4.10.4", "The Haber process and NPK fertilisers", 1.0),
        ],
    },
    SubjectDef {
        id: "phys_aqa", name: "Physics", full: "AQA GCSE Physics (8463) Higher", color: "var(--phys)",
        papers: "Paper 1 (topics 4.1-4.4) and Paper 2 (topics 4.5-4.8), each 1h45, 100 marks, 50%. Multiple choice, structured, closed short answer and open response, including the 10 required practicals; an equation sheet is provided",
        spec: "https://www.aqa.org.uk/subjects/physics/gcse/physics-8463",
        sections: &["4.1 Energy", "4.2 Electricity", "4.3 Particle model of matter", "4.4 Atomic structure", "4.5 Forces", "4.6 Waves", "4.7 Magnetism and electromagnetism", "4.8 Space physics"],
        topics: &[
            ("4.1.1", "Energy stores, systems, changes in energy, energy transferred and power", 2.0),
            ("4.1.2", "Conservation and dissipation of energy, and efficiency", 1.0),
            ("4.1.3", "National and global energy resources", 1.0),
            ("4.2.1", "Current, potential difference and resistance, and I-V characteristics", 2.0),
            ("4.2.2", "Series and parallel circuits", 1.5),
            ("4.2.3", "Domestic uses and safety: mains electricity and plugs", 0.5),
            ("4.2.4", "Energy transfers in circuits: power and the National Grid", 1.5),
            ("4.2.5", "Static electricity and electric fields", 1.0),
            ("4.3.1", "Changes of state and the particle model: density", 1.0),
            ("4.3.2", "Internal energy, specific heat capacity and specific latent heat", 1.5),
            ("4.3.3", "The particle model and gas pressure", 1.0),
            ("4.4.1", "Atoms, isotopes and the development of the atomic model", 1.0),
            ("4.4.2", "Radioactive decay, nuclear radiation, nuclear equations and half-life", 2.0),
            ("4.4.3", "Hazards and uses of radiation, contamination, irradiation and background radiation", 1.0),
            ("4.4.4", "Nuclear fission and fusion", 1.0),
            ("4.5.1", "Forces: scalars and vectors, contact and non-contact forces, gravity and resultant forces", 1.5),
            ("4.5.2", "Work done and energy transfer", 0.5),
            ("4.5.3", "Forces and elasticity: Hooke's law and elastic potential energy", 1.0),
            ("4.5.4", "Moments, levers and gears", 1.0),
            ("4.5.5", "Pressure in fluids, upthrust and atmospheric pressure", 1.0),
            ("4.5.6a", "Describing motion: distance, displacement, speed, velocity and acceleration graphs", 2.0),
            ("4.5.6b", "Newton's laws, terminal velocity and inertia", 1.5),
            ("4.5.6c", "Stopping distances and reaction time", 1.0),
            ("4.5.7", "Momentum and its conservation, and changes in momentum", 1.5),
            ("4.6.1", "Waves: transverse and longitudinal, wave properties, reflection, refraction and sound", 2.0),
            ("4.6.2", "Electromagnetic waves: the spectrum, properties, uses, lenses and visible light", 2.0),
            ("4.6.3", "Black body radiation", 1.0),
            ("4.7.1", "Permanent and induced magnetism, magnetic forces and fields", 1.0),
            ("4.7.2", "The motor effect, Fleming's left-hand rule and electric motors", 1.5),
            ("4.7.3", "Induced potential, generators, microphones, transformers and the National Grid", 1.5),
            ("4.8.1", "The solar system, the life cycle of a star, orbital motion and satellites", 1.5),
            ("4.8.2", "Red-shift and the expanding universe", 1.0),
        ],
    },
    SubjectDef {
        id: "econ_aqa", name: "Economics", full: "AQA GCSE Economics (8136)", color: "var(--econ)",
        papers: "Paper 1 How markets work (3.1: economic foundations, resource allocation, prices, costs and revenue, market structures and the labour market, market failure) and Paper 2 How the economy works (3.2: the national economy, government objectives and policies, international trade and globalisation, money and financial markets): each 1h45, 80 marks, 50%, calculator allowed. Each paper has Section A (ten multiple choice, then short, calculation and data questions, a 3-mark diagram and a 9-mark assess) and Section B (a case study ending in a 15-mark extended answer). There is no NEA - everything in the course is examined and taught in the app",
        spec: "https://www.aqa.org.uk/subjects/economics/gcse/economics-8136/specification",
        sections: &["3.1.1 Economic foundations", "3.1.2 Resource allocation", "3.1.3 How prices are determined", "3.1.4 Production, costs, revenue and profit", "3.1.5 Competitive and concentrated markets", "3.1.6 Market failure", "3.2.1 Introduction to the national economy", "3.2.2 Government objectives", "3.2.3 How the government manages the economy", "3.2.4 International trade and the global economy", "3.2.5 The role of money and financial markets"],
        topics: &[
            ("3.1.1.1", "Economic activity: needs, wants, the key decisions and the main economic groups", 0.5),
            ("3.1.1.2", "The factors of production and their rewards", 0.5),
            ("3.1.1.3", "Making choices: the basic economic problem and opportunity cost", 0.5),
            ("3.1.2.1", "Markets, the allocation of resources, and factor and product markets", 0.5),
            ("3.1.2.2", "Primary, secondary and tertiary sectors; goods and services", 0.5),
            ("3.1.2.3", "Specialisation, the division of labour and exchange", 0.5),
            ("3.1.3.1", "Demand: its determinants, the demand curve, shifts and movements", 0.75),
            ("3.1.3.2", "Supply: its determinants, the supply curve, shifts and movements", 0.5),
            ("3.1.3.3", "Equilibrium price, excess demand and supply, and revenue on the diagram", 1.0),
            ("3.1.3.4", "Intermarket relationships: complements and substitutes", 0.5),
            ("3.1.3.5", "Price elasticity of demand: calculation, factors and implications", 1.0),
            ("3.1.3.6", "Price elasticity of supply: calculation, factors and implications", 0.5),
            ("3.1.4.1", "Business objectives, costs, revenue and profit", 1.0),
            ("3.1.4.2", "Production and productivity", 0.5),
            ("3.1.4.3", "Economies and diseconomies of scale", 0.75),
            ("3.1.5.1", "Market structures and how to tell them apart", 0.5),
            ("3.1.5.2", "Competitive markets and their impact on consumers, producers and workers", 0.5),
            ("3.1.5.3", "Non-competitive markets: monopoly and oligopoly", 0.75),
            ("3.1.5.4", "The labour market: wage determination, differentials, gross and net pay", 0.75),
            ("3.1.6.1", "Market failure as a misallocation of resources, and government intervention", 0.75),
            ("3.1.6.2", "Externalities: private and social costs and benefits", 0.75),
            ("3.2.1.1", "Interest rates and decisions to save, borrow, spend and invest", 0.75),
            ("3.2.1.2", "Government income and spending: direct, indirect, progressive and regressive taxes", 0.5),
            ("3.2.2.1", "The government's economic objectives and the conflicts between them", 0.75),
            ("3.2.2.2", "Economic growth: GDP, real GDP and GDP per capita", 0.75),
            ("3.2.2.3", "Employment and unemployment: measurement, types and consequences", 0.75),
            ("3.2.2.4", "Inflation: CPI, cost-push and demand-pull, and its consequences", 0.75),
            ("3.2.2.5", "The balance of payments on current account", 0.5),
            ("3.2.2.6", "The distribution of income and wealth, and redistribution", 0.5),
            ("3.2.3.1", "Fiscal policy and the government budget", 0.75),
            ("3.2.3.2", "Monetary policy", 0.5),
            ("3.2.3.3", "Supply-side policies", 0.5),
            ("3.2.3.4", "Policies to correct positive and negative externalities", 0.5),
            ("3.2.4.1", "Why countries trade and the importance of trade to the UK", 0.5),
            ("3.2.4.2", "Exchange rates: determination and effects", 0.75),
            ("3.2.4.3", "Free trade and free-trade agreements, including the EU", 0.5),
            ("3.2.4.4", "Globalisation: causes, benefits and drawbacks", 0.5),
            ("3.2.5.1", "The role and functions of money", 0.5),
            ("3.2.5.2", "The financial sector: the Bank of England, banks and building societies", 0.5),
        ],
    },
    // WJEC Eduqas GCSE (9-1) English Literature C720QS, specification Version 4
    // (August 2024), read 30 September 2026. Second-board version of English
    // Literature (AQA 8702 is "englit"). The spec has no numbered content, so the
    // references are its own section labels: C1A Shakespeare, C1B poetry
    // anthology, C2A post-1914 prose/drama, C2B 19th-century prose, C2C unseen
    // poetry. Set texts are chosen by schools; the built topics are the
    // most-taught choice in each list (Macbeth, An Inspector Calls, A Christmas
    // Carol - Eduqas examiners' reports 2023-2026) plus the whole poetry
    // anthology for first assessment in 2027 (fixed, 15 poems). The method
    // topics (C1Ab, C1Ac, C2Ab, C2Bb) apply to any set text.
    SubjectDef {
        id: "englit_edq", name: "Eng Literature", full: "Eduqas GCSE English Literature (C720QS)", color: "var(--englit)",
        papers: "Component 1 Shakespeare and Poetry 2h, 80 marks, 40%: an extract question (15) and an essay (25, including 5 for spelling, punctuation and grammar) on your Shakespeare play, then one question on a named anthology poem (15) and a comparison with a second anthology poem you choose (25). Component 2 Post-1914 Prose/Drama, 19th Century Prose and Unseen Poetry 2h30, 120 marks, 60%: one 40-mark extract-and-whole-text question on your modern text (including 5 for accuracy), one 40-mark extract-and-whole-novel question on your 19th-century novel (with context), and two unseen poems (15 and 25). Closed book: no texts or anthology in either exam. The topics cover Macbeth, An Inspector Calls and A Christmas Carol, the texts most schools teach, plus all 15 poems of the anthology examined from 2027: if your school studies a different Shakespeare play, modern text or novel, delete that text's topic (Macbeth, An Inspector Calls or A Christmas Carol) in the Plan tab and keep the method topics, which work for any text. There is no NEA or coursework: the two exams are the whole GCSE",
        spec: "https://www.eduqas.co.uk/qualifications/english-literature-gcse/",
        sections: &["C1A Shakespeare", "C1B Poetry anthology (from 2027)", "C2A Post-1914 prose/drama", "C2B 19th-century prose", "C2C Unseen poetry"],
        topics: &[
            // Component 1 Section A - Shakespeare (Macbeth; the method topics fit any play)
            ("C1Aa", "Macbeth: the play, its characters, themes and context", 2.0),
            ("C1Ab", "The Shakespeare extract question: how an audience might respond (15 marks)", 1.5),
            ("C1Ac", "The Shakespeare essay question: the whole play, plus accuracy marks (25 marks)", 1.5),
            // Component 1 Section B - the Eduqas Poetry Anthology, first assessed 2027
            ("C1Ba", "The Schoolboy — Blake", 0.75),
            ("C1Bb", "I Wandered Lonely as a Cloud — Wordsworth", 0.75),
            ("C1Bc", "Cousin Kate — Rossetti", 0.75),
            ("C1Bd", "Sonnet 29 — Elizabeth Barrett Browning", 0.75),
            ("C1Be", "Drummer Hodge — Hardy", 0.75),
            ("C1Bf", "Disabled — Owen", 0.75),
            ("C1Bg", "I Shall Return — McKay", 0.75),
            ("C1Bh", "Decomposition — Ghose", 0.75),
            ("C1Bi", "Catrin — Clarke", 0.75),
            ("C1Bj", "Blackberry Picking — Heaney", 0.75),
            ("C1Bk", "Kamikaze — Garland", 0.75),
            ("C1Bl", "War Photographer — Duffy", 0.75),
            ("C1Bm", "Dusting the Phone — Kay", 0.75),
            ("C1Bn", "Remains — Armitage", 0.75),
            ("C1Bo", "Origin Story — Ewing", 0.75),
            ("C1Bp", "Linking the anthology: themes, contexts and pairings", 1.0),
            ("C1Bq", "The two anthology questions: one named poem (15) and the comparison (25)", 1.25),
            // Component 2 Section A - post-1914 prose/drama (An Inspector Calls)
            ("C2Aa", "An Inspector Calls: the play, its characters, themes and stagecraft", 2.0),
            ("C2Ab", "The post-1914 source-based question: extract and whole text, plus accuracy (40 marks)", 1.5),
            // Component 2 Section B - 19th-century prose (A Christmas Carol)
            ("C2Ba", "A Christmas Carol: the staves, characters, themes and context", 2.0),
            ("C2Bb", "The 19th-century prose question: extract, whole novel and context (40 marks)", 1.5),
            // Component 2 Section C - unseen poetry
            ("C2Ca", "Unseen poetry: one poem and its effect on you (15 marks)", 1.75),
            ("C2Cb", "Unseen poetry: comparing two poems (25 marks)", 1.75),
        ],
    },
    // Pearson Edexcel GCSE (9-1) Business 1BS0, specification Issue 2 (July
    // 2022), read on 30 September 2026. This is the Edexcel alternative to the
    // AQA 8132 subject `bus`, and a student takes one of the two. There is no
    // NEA: both papers are written exams, so the whole course is taught here.
    // 1.3.2 is split in two (a: revenue, costs, profit and interest; b: break
    // even). The other codes are the spec's own subsection numbers.
    SubjectDef {
        id: "bus_edx", name: "Business", full: "Pearson Edexcel GCSE Business (1BS0)", color: "var(--bus)",
        papers: "Paper 1 Investigating small business (Theme 1) and Paper 2 Building a business (Theme 2): each a written exam of 1h45, 90 marks, 50%. Section A (35 marks) is multiple choice, calculations, 3-mark explain and a 6-mark discuss. Sections B (30) and C (25) are on two business case studies in a source booklet: outline, calculate, 6-mark analyse, 9-mark justify and a 12-mark evaluate. Calculators allowed; formulae are NOT given. There is no coursework or NEA, so the whole course is covered here",
        spec: "https://qualifications.pearson.com/en/qualifications/edexcel-gcses/business-2017.html",
        sections: &["1.1 Enterprise and entrepreneurship", "1.2 Spotting a business opportunity", "1.3 Putting a business idea into practice", "1.4 Making the business effective", "1.5 Understanding external influences on business", "2.1 Growing the business", "2.2 Making marketing decisions", "2.3 Making operational decisions", "2.4 Making financial decisions", "2.5 Making human resource decisions"],
        topics: &[
            // Theme 1: Investigating small business (Paper 1)
            ("1.1.1", "The dynamic nature of business: why and how new ideas come about", 0.5),
            ("1.1.2", "Risk and reward in starting a business", 0.5),
            ("1.1.3", "The role of enterprise, adding value and the entrepreneur", 0.5),
            ("1.2.1", "Customer needs, and why understanding them matters", 0.5),
            ("1.2.2", "Market research: purpose, methods and data", 0.75),
            ("1.2.3", "Market segmentation and market mapping", 0.5),
            ("1.2.4", "The competitive environment", 0.5),
            ("1.3.1", "Business aims and objectives when starting up", 0.5),
            ("1.3.2a", "Revenue, costs, profit and loss, and interest", 0.75),
            ("1.3.2b", "Break even, margin of safety and break-even diagrams", 0.75),
            ("1.3.3", "Cash and cash-flow forecasts", 1.0),
            ("1.3.4", "Sources of finance for a start-up or small business", 0.5),
            ("1.4.1", "Limited liability, types of ownership and franchising", 0.75),
            ("1.4.2", "Business location", 0.5),
            ("1.4.3", "The marketing mix and how its elements work together", 0.5),
            ("1.4.4", "Business plans", 0.5),
            ("1.5.1", "Business stakeholders", 0.5),
            ("1.5.2", "Technology and business", 0.5),
            ("1.5.3", "Legislation: consumer law and employment law", 0.5),
            ("1.5.4", "The economy and business", 0.75),
            ("1.5.5", "Responding to external influences", 0.5),
            // Theme 2: Building a business (Paper 2)
            ("2.1.1", "Business growth: organic and external growth, plcs and finance for growth", 0.75),
            ("2.1.2", "Why and how aims and objectives change as a business evolves", 0.5),
            ("2.1.3", "Globalisation, barriers to trade and competing internationally", 0.75),
            ("2.1.4", "Ethics, the environment and business", 0.5),
            ("2.2.1", "Product: the design mix, product life cycle and differentiation", 0.75),
            ("2.2.2", "Price: pricing strategies and what influences them", 0.5),
            ("2.2.3", "Promotion", 0.5),
            ("2.2.4", "Place: retailers and e-tailers", 0.5),
            ("2.2.5", "Using the marketing mix to make business decisions", 0.5),
            ("2.3.1", "Business operations: production processes and technology", 0.5),
            ("2.3.2", "Working with suppliers: stock control, JIT, procurement and logistics", 0.75),
            ("2.3.3", "Managing quality", 0.5),
            ("2.3.4", "The sales process and customer service", 0.5),
            ("2.4.1", "Business calculations: gross and net profit, margins and ARR", 1.0),
            ("2.4.2", "Understanding business performance: using and questioning data", 0.5),
            ("2.5.1", "Organisational structures, communication and ways of working", 0.75),
            ("2.5.2", "Effective recruitment", 0.5),
            ("2.5.3", "Effective training and development", 0.5),
            ("2.5.4", "Motivation", 0.5),
        ],
    },
    // AQA GCSE Computer Science 8525, the second-board version of Computer
    // Science (the `cs` entry is OCR J277). Topics are the spec's own references
    // for sections 3.1-3.8, read from the updated specification for first
    // teaching in September 2025, first exams June 2027 (AQA spec pages and
    // PDF dated 16 June 2025). 3.1.1, 3.2.2, 3.2.11 and 3.4.5 are split a/b,
    // and 3.5, which has no sub-numbers, is split 3.5a/3.5b.
    SubjectDef {
        id: "cs_aqa", name: "Computer Science", full: "AQA GCSE Computer Science (8525)", color: "var(--cs)",
        papers: "Paper 1 Computational thinking and programming skills (8525/1A C#, 1B Python or 1C VB.NET, the language the school chose): written, 2h, 90 marks, 50%. Paper 2 Computing concepts (8525/2): written, 1h45, 90 marks, 50%, including SQL and two 9-mark discussions. No calculator. There is no NEA: the practical programming the school must provide is confirmed by a statement and carries no marks, so the two papers are the whole grade",
        spec: "https://www.aqa.org.uk/subjects/computer-science/gcse/computer-science-8525/specification",
        sections: &["3.1 Fundamentals of algorithms", "3.2 Programming", "3.3 Fundamentals of data representation", "3.4 Computer systems", "3.5 Fundamentals of computer networks", "3.6 Cyber security", "3.7 Relational databases and SQL", "3.8 Ethical, legal and environmental impacts"],
        topics: &[
            // 3.1 Fundamentals of algorithms
            ("3.1.1a", "Algorithms, decomposition, abstraction; pseudo-code and flowcharts", 0.5),
            ("3.1.1b", "Inputs, processing and outputs; trace tables and the purpose of an algorithm", 0.75),
            ("3.1.2", "Efficiency of algorithms", 0.5),
            ("3.1.3", "Searching algorithms: linear and binary search", 0.5),
            ("3.1.4", "Sorting algorithms: merge sort and bubble sort", 0.75),
            // 3.2 Programming
            ("3.2.1", "Data types: integer, real, Boolean, character and string", 0.5),
            ("3.2.2a", "Programming concepts: variables, constants, assignment, selection and identifiers", 0.5),
            ("3.2.2b", "Programming concepts: definite, indefinite and nested iteration", 0.75),
            ("3.2.3", "Arithmetic operations: real division, DIV and MOD", 0.5),
            ("3.2.4", "Relational operations", 0.5),
            ("3.2.5", "Boolean operations: NOT, AND and OR in conditions", 0.5),
            ("3.2.6", "Data structures: one- and two-dimensional arrays and records", 0.75),
            ("3.2.7", "Input and output", 0.5),
            ("3.2.8", "String handling: length, position, substring, concatenation and conversions", 0.75),
            ("3.2.9", "Random number generation", 0.5),
            ("3.2.10", "Subroutines, parameters, return values, local variables and structured programming", 1.0),
            ("3.2.11a", "Robust and secure programming: validation and authentication routines", 0.75),
            ("3.2.11b", "Testing: test data, syntax and logic errors, correcting errors", 0.5),
            // 3.3 Fundamentals of data representation
            ("3.3.1", "Number bases: decimal, binary and hexadecimal", 0.5),
            ("3.3.2", "Converting between binary, decimal and hexadecimal", 0.5),
            ("3.3.3", "Units of information: bits, bytes and decimal prefixes", 0.5),
            ("3.3.4", "Binary arithmetic: adding binary numbers and logical shifts", 0.5),
            ("3.3.5", "Character encoding: ASCII and Unicode", 0.5),
            ("3.3.6", "Representing images: pixels, colour depth and bitmap file size", 0.75),
            ("3.3.7", "Representing sound: sampling rate, sample resolution and file size", 0.5),
            ("3.3.8", "Data compression: Huffman coding and run length encoding", 0.75),
            // 3.4 Computer systems
            ("3.4.1", "Hardware and software", 0.5),
            ("3.4.2", "Boolean logic: truth tables, logic circuits and Boolean expressions", 0.75),
            ("3.4.3", "Software classification: system and application software, operating systems and utilities", 0.5),
            ("3.4.4", "Programming languages and translators: low and high level, compilers, interpreters, assemblers", 0.5),
            ("3.4.5a", "Systems architecture: CPU components, the fetch-execute cycle and CPU performance", 0.75),
            ("3.4.5b", "Memory and storage: RAM, ROM, cache, secondary storage, cloud storage and embedded systems", 0.75),
            // 3.5 Fundamentals of computer networks
            ("3.5a", "Computer networks: PAN, LAN and WAN, wired and wireless, and network security", 0.5),
            ("3.5b", "Network protocols and the four-layer TCP/IP model", 0.5),
            // 3.6 Cyber security
            ("3.6.1", "Fundamentals of cyber security", 0.5),
            ("3.6.2", "Cyber security threats and penetration testing", 0.5),
            ("3.6.2.1", "Social engineering: blagging, phishing and shouldering", 0.5),
            ("3.6.2.2", "Malicious code: viruses, trojans and spyware", 0.5),
            ("3.6.3", "Detecting and preventing cyber security threats", 0.5),
            // 3.7 Relational databases and SQL
            ("3.7.1", "Relational databases: tables, records, fields, keys", 0.5),
            ("3.7.2", "SQL: SELECT, INSERT, UPDATE and DELETE", 1.0),
            // 3.8 Ethical, legal and environmental impacts
            ("3.8", "Ethical, legal and environmental impacts of digital technology, including privacy", 0.75),
        ],
    },
    // AQA GCSE History 8145, specification version 1.3 (24 September 2019),
    // read 30 September 2026. Each of the four exam sections has options; the
    // topics cover the two most-taken options in each section (AQA's own
    // topic-popularity figures, 2019 insight report): Germany 55% and America
    // 1920-73 30% (Paper 1A); Conflict and tension 1918-39 42% and East and
    // West 1945-72 22% (1B); Health and the people 75% and Power and the
    // people 17% (2A); Elizabethan England 57% and Norman England 36% (2B).
    // A student deletes the four options their school does not take, leaving
    // about 15 h (30 h with all eight). Codes are paper + AQA option code +
    // Part number (1AB.2 = Paper 1, option AB, Part two).
    SubjectDef {
        id: "hist_aqa", name: "History", full: "AQA GCSE History (8145)", color: "var(--hist)",
        papers: "Paper 1 Understanding the modern world, 2h, 84 marks including 4 for SPaG, 50%: Section A period study (six questions, 40 marks: how interpretations differ 4, why they differ 4, which is more convincing 8, describe 4, in what ways 8, a two-bullet essay 12) and Section B wider world depth study (four questions, 44 marks: source analysis 4, how useful are two sources 12, write an account 8, a how-far-do-you-agree essay 16 plus 4 SPaG). Paper 2 Shaping the nation, 2h, 84 marks including 4 for SPaG, 50%: Section A thematic study (how useful is a source 8, significance 8, similarity or difference 8, a factor essay 16 plus 4 SPaG) and Section B British depth study with the historic environment (how convincing is an interpretation 8, explain 8, write an account 8, a 16-mark essay on the specified site). The whole course is examined: there is no NEA. Your school takes one option in each of the four sections. The topics here cover the two most-taken in each: Germany 1890-1945 or America 1920-1973 (1A), Conflict and tension 1918-1939 or East and West 1945-1972 (1B), Health and the people or Power and the people (2A), Elizabethan England or Norman England (2B). In the Plan tab, delete the four options your school does not take. America 1840-1895, Russia, the First World War, Asia, the Gulf and Afghanistan, Migration and empires, Edward I and Restoration England are not in the app. The historic environment site changes every year: the 2B lessons teach the sites for June 2028 (Kenilworth Castle, the White Tower)",
        spec: "https://www.aqa.org.uk/subjects/history/gcse/history-8145/specification",
        sections: &[
            "1AB Germany, 1890-1945: Democracy and dictatorship (Paper 1A)",
            "1AD America, 1920-1973: Opportunity and inequality (Paper 1A)",
            "1BB Conflict and tension: the inter-war years, 1918-1939 (Paper 1B)",
            "1BC Conflict and tension between East and West, 1945-1972 (Paper 1B)",
            "2AA Britain: Health and the people, c1000 to the present day (Paper 2A)",
            "2AB Britain: Power and the people, c1170 to the present day (Paper 2A)",
            "2BA Norman England, c1066-c1100 (Paper 2B)",
            "2BC Elizabethan England, c1568-1603 (Paper 2B)",
        ],
        topics: &[
            // 1AB Germany, 1890-1945 (Paper 1 Section A) - delete if your school does not take it
            ("1AB.1a", "Germany 1890-1914: Kaiser Wilhelm, parliament, Prussian militarism, industrialisation, socialism and the Navy Laws", 0.5),
            ("1AB.1b", "War and Weimar 1918-1929: defeat, reparations, the Ruhr, hyperinflation, the putsches, Stresemann and Weimar culture", 0.75),
            ("1AB.2", "Germany and the Depression: Nazi support 1928-32, Hitler becomes Chancellor, and the dictatorship to 1934", 0.75),
            ("1AB.3a", "The Nazi economy: jobs, public works, rearmament, self-sufficiency and the impact of war on the German people", 0.5),
            ("1AB.3b", "Nazi social policy: women, young people, education, the churches, racial policy and the Final Solution", 0.75),
            ("1AB.3c", "Nazi control: Goebbels and propaganda, the police state, and opposition and resistance", 0.5),
            // 1AD America, 1920-1973 (Paper 1 Section A) - delete if your school does not take it
            ("1AD.1a", "The Boom: consumer society, hire purchase, mass production, Republican policies, inequality, cinema, jazz and flappers", 0.5),
            ("1AD.1b", "A divided society: prohibition, organised crime, immigration, racial tension, the Ku Klux Klan and the Red Scare", 0.5),
            ("1AD.2a", "The Depression and the New Deal: Hoover, Roosevelt, and how far the New Deal worked", 0.75),
            ("1AD.2b", "The impact of the Second World War: recovery, Lend Lease, African Americans and women", 0.5),
            ("1AD.3a", "Post-war America: prosperity, the American Dream, McCarthyism, rock and roll and television", 0.5),
            ("1AD.3b", "Civil rights in the 1950s and 1960s: segregation, King, Malcolm X, Black Power and the Civil Rights Acts", 0.5),
            ("1AD.3c", "The Great Society and the women's movement: Kennedy, Johnson, NOW, equal pay, Roe v Wade and the ERA", 0.5),
            // 1BB Conflict and tension, 1918-1939 (Paper 1 Section B) - delete if your school does not take it
            ("1BB.1", "Peacemaking 1919: the Big Three's aims, the Treaty of Versailles, and reactions to the settlement", 1.0),
            ("1BB.2a", "The League of Nations in the 1920s: organisation, powers, agencies, successes and failures, Locarno and Kellogg-Briand", 0.75),
            ("1BB.2b", "The collapse of the League: the Depression, Manchuria and Abyssinia", 0.5),
            ("1BB.3a", "Rising tension 1933-38: Hitler's aims, rearmament, the Rhineland, the Anschluss, appeasement and Munich", 1.0),
            ("1BB.3b", "The outbreak of war in 1939: Czechoslovakia, the Nazi-Soviet Pact, Poland, and who was responsible", 0.5),
            // 1BC East and West, 1945-1972 (Paper 1 Section B) - delete if your school does not take it
            ("1BC.1a", "The end of the Second World War: Yalta, Potsdam, the division of Germany, rival ideologies and the atomic bomb", 0.75),
            ("1BC.1b", "The Iron Curtain: Soviet expansion, the Truman Doctrine, Marshall Plan, Cominform, Comecon, Yugoslavia and the Berlin Blockade", 0.75),
            ("1BC.2", "The Cold War develops: China, Korea and Vietnam, the arms and space races, NATO and the Warsaw Pact, Hungary and the U2", 1.0),
            ("1BC.3a", "Transformation: the Berlin Wall and the Cuban Missile Crisis", 0.75),
            ("1BC.3b", "The Prague Spring, the Brezhnev Doctrine, and the easing of tension: détente and SALT 1", 0.5),
            // 2AA Health and the people (Paper 2 Section A) - delete if your school does not take it
            ("2AA.1", "Medicine stands still: medieval ideas, Hippocrates and Galen, Christianity, Islamic medicine, surgery and the Black Death", 0.75),
            ("2AA.2", "The beginnings of change: the Renaissance, Vesalius, Paré and Harvey, treatments, John Hunter, and Jenner's vaccination", 0.75),
            ("2AA.3a", "A revolution in medicine: germ theory, Pasteur, Koch, Ehrlich, and anaesthetics, antiseptics and aseptic surgery", 0.75),
            ("2AA.3b", "Public health in industrial Britain: cholera, the reformers, and the 1848 and 1875 Public Health Acts", 0.5),
            ("2AA.4a", "Modern medicine: penicillin, the pharmaceutical industry, new diseases, and war and technology in surgery", 0.5),
            ("2AA.4b", "Modern public health: Booth, Rowntree, the Liberal reforms, the world wars, Beveridge, the NHS and healthcare today", 0.75),
            // 2AB Power and the people (Paper 2 Section A) - delete if your school does not take it
            ("2AB.1", "Challenging authority and feudalism: Magna Carta, Simon de Montfort and the Peasants' Revolt", 0.75),
            ("2AB.2", "Challenging royal authority: the Pilgrimage of Grace, the English Revolution and the American Revolution", 1.0),
            ("2AB.3", "Reform and reformers: the Great Reform Act, Chartism, campaigning groups and trade unionism", 1.0),
            ("2AB.4", "Equality and rights: women's suffrage, the General Strike, trade union reform and minority rights", 1.0),
            // 2BA Norman England (Paper 2 Section B) - delete if your school does not take it
            ("2BA.1a", "The Norman Conquest: the claimants of 1066, Stamford Bridge, Hastings, and cavalry and castles", 0.75),
            ("2BA.1b", "Establishing control: the revolts of 1067-75, the Harrying of the North, William I's government and William II", 0.5),
            ("2BA.2", "Life under the Normans: feudalism, government, law, the Domesday Book, towns, villages and forest law", 0.75),
            ("2BA.3", "The Norman Church and monasticism: Lanfranc's reforms, church building, Church and state, and monastic life", 0.75),
            ("2BA.4", "The historic environment of Norman England: the specified site (the White Tower for 2028)", 0.75),
            // 2BC Elizabethan England (Paper 2 Section B) - delete if your school does not take it
            ("2BC.1", "Elizabeth's court and Parliament: her character, patronage, ministers, marriage and succession, and Essex's rebellion", 0.75),
            ("2BC.2", "Life in Elizabethan times: the Golden Age, the theatre, the poor, and Hawkins, Drake and Raleigh", 1.0),
            ("2BC.3a", "Religious matters and Mary, Queen of Scots: Catholics, Puritans, plots and her execution", 0.75),
            ("2BC.3b", "Conflict with Spain and the defeat of the Armada", 0.5),
            ("2BC.4", "The historic environment of Elizabethan England: the specified site (Kenilworth Castle for 2028)", 0.75),
        ],
    },
];

/// Seed calendar: (first Monday, number of weeks, type, label, year, block).
/// Copied into `PlanConfig` on first run, and editable from there.
pub const BLOCKS: &[(&str, u32, &str, &str, u8, &str)] = &[
    ("2026-09-07", 7, "term", "Autumn 1", 10, "A1"), ("2026-10-26", 1, "half", "October half-term", 10, "H"),
    ("2026-11-02", 7, "term", "Autumn 2", 10, "A2"), ("2026-12-21", 2, "holiday", "Christmas holiday", 10, "H"),
    ("2027-01-04", 6, "term", "Spring 1", 10, "S1"), ("2027-02-15", 1, "half", "February half-term", 10, "H"),
    ("2027-02-22", 5, "term", "Spring 2", 10, "S2"), ("2027-03-29", 2, "holiday", "Easter holiday", 10, "H"),
    ("2027-04-12", 7, "term", "Summer 1", 10, "U1"), ("2027-05-31", 1, "half", "May half-term", 10, "H"),
    ("2027-06-07", 6, "term", "Summer 2", 10, "U2"), ("2027-07-19", 7, "summer", "Summer holiday", 10, "H"),
    ("2027-09-06", 7, "term", "Autumn 1", 11, "A1"), ("2027-10-25", 1, "half", "October half-term", 11, "H"),
    ("2027-11-01", 7, "term", "Autumn 2", 11, "A2"), ("2027-12-20", 2, "holiday", "Christmas holiday", 11, "H"),
    ("2028-01-03", 6, "term", "Spring 1", 11, "S1"), ("2028-02-14", 1, "half", "February half-term", 11, "H"),
    ("2028-02-21", 6, "term", "Spring 2", 11, "S2"), ("2028-04-03", 2, "holiday", "Easter holiday", 11, "H"),
    ("2028-04-17", 3, "term", "Summer 1", 11, "U1"), ("2028-05-08", 7, "exam", "Exam season", 11, "X"),
];

// ---------- Output types (serialised to the UI) ----------

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Statement { pub code: String, pub label: String }

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
    pub id: String, pub code: String, pub title: String, pub hours: f64, pub subj: String, pub idx: usize,
    pub url: String, pub objectives: Vec<String>, pub watch: String,
    /// The individual specification statements this topic teaches.
    pub statements: Vec<Statement>,
    /// Whether a built-in lesson exists for it (see lessons.rs).
    pub lesson: bool,
    /// Introduction videos, first to watch first (see videos.rs).
    pub videos: Vec<crate::videos::Video>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub id: String, pub name: String, pub full: String, pub color: String, pub papers: String, pub spec: String,
    pub sections: Vec<String>, pub topics: Vec<Topic>, pub content_ends_week: u32, pub resources: Vec<ResourceLink>,
    /// `ahead` or `school` - see `SubjectCfg::pace`.
    pub pace: String,
    /// Official spec references, empty if this subject has not been checked
    /// against its specification yet.
    pub spec_refs: Vec<String>,
}

#[derive(Serialize, Clone)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Entry {
    #[serde(rename_all = "camelCase")]
    Topic { topic_id: String, hours: f64, part_no: u32, parts: u32, #[serde(rename = "final")] is_final: bool },
    Rev { section: String, hours: f64 },
    /// A fixed weekly session such as a timed writing piece.
    Fixed { label: String, hours: f64, note: String },
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Week {
    pub n: u32, pub start: String, #[serde(rename = "type")] pub kind: String, pub label: String, pub year: u8, pub block: String,
    pub last: bool, pub first: bool, pub term_week: u32, pub plan: HashMap<String, Vec<Entry>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Test { pub id: String, pub week: u32, pub kind: String, pub title: String, pub mins: u32, pub subjects: Vec<String>, pub desc: String }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub subjects: Vec<Subject>, pub weeks: Vec<Week>, pub tests: Vec<Test>,
    pub topic_due: HashMap<String, u32>,
    pub review_gaps: Vec<u32>, pub exam_start: String,
    /// Topics that no longer fit before the exams — only ever non-empty after a
    /// catch-up has pushed more work into fewer weeks than it needs.
    pub unscheduled: Vec<String>,
}

// ---------- Scheduler ----------

/// A snapshot taken when the user asks to catch up: from this week on, re-pour
/// everything they have not done yet, so lost weeks move the remaining work
/// forward instead of leaving them permanently marked "behind".
///
/// It is a snapshot on purpose — taken once, when the button is pressed. If it
/// read the live list of finished topics the schedule would shift under the
/// user every time they ticked something off.
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CatchUp {
    pub from_week: u32,
    pub done: Vec<String>,
}

/// Turn a configuration into a full two-year plan: a week-by-week calendar with
/// every topic poured into it, plus the four layers of tests.
pub fn build(cfg: &PlanConfig) -> Plan {
    build_with(cfg, None)
}

/// As `build`, but re-pouring unfinished work from a catch-up week onwards.
/// Weeks before that week keep the schedule they always had, so the record of
/// what was originally planned survives.
pub fn build_with(cfg: &PlanConfig, catch_up: Option<&CatchUp>) -> Plan {
    // Calendar
    let mut weeks: Vec<Week> = Vec::new();
    let mut term_count = 0;
    for b in &cfg.blocks {
        let Ok(first) = NaiveDate::parse_from_str(&b.start, "%Y-%m-%d") else { continue };
        for i in 0..b.weeks {
            let mut term_week = 0;
            if b.kind == "term" { term_count += 1; term_week = term_count; }
            weeks.push(Week {
                n: weeks.len() as u32 + 1,
                start: (first + Duration::days(7 * i as i64)).format("%Y-%m-%d").to_string(),
                kind: b.kind.clone(), label: b.label.clone(), year: b.year, block: b.block.clone(),
                last: i == b.weeks - 1, first: i == 0, term_week, plan: HashMap::new(),
            });
        }
    }

    // Pour each subject's topic-hours into the weekly budgets; once content is
    // finished, cycle past-paper practice through the spec sections.
    let mut subjects = Vec::new();
    let mut topic_due: HashMap<String, u32> = HashMap::new();
    for def in &cfg.subjects {
        let topics: Vec<Topic> = def.topics.iter().enumerate().map(|(i, t)| Topic {
            id: format!("{}:{}", def.id, t.code), code: t.code.clone(), title: t.title.clone(), hours: t.hours,
            subj: def.id.clone(), idx: i, url: t.url.clone(),
            objectives: t.objectives.clone(), watch: t.watch.clone(),
            statements: crate::statements::for_topic(&def.id, &t.code).into_iter()
                .map(|(code, label)| Statement { code, label }).collect(),
            lesson: crate::lessons::has_lesson(&format!("{}:{}", def.id, t.code)),
            videos: if t.videos.is_empty() { crate::videos::for_topic(&format!("{}:{}", def.id, t.code)) }
                else { t.videos.iter().map(|v| crate::videos::Video { id: v.url.clone(), title: v.title.clone(), by: String::new() }).collect() },
        }).collect();
        let mut ti = 0usize;
        let mut rem = topics.first().map(|t| t.hours).unwrap_or(0.0);
        let mut cycle = 0usize;
        let mut content_ends_week = 0;
        let is_done = |i: usize| catch_up.is_some_and(|cu| cu.done.iter().any(|d| *d == topics[i].id));
        for w in weeks.iter_mut() {
            // From the catch-up week on, drop back to the earliest topic that is
            // still unfinished — anything missed comes round again from here.
            if catch_up.is_some_and(|cu| cu.from_week == w.n) {
                // Forget where the unfinished topics were originally due - if the
                // re-pour cannot reach one, it must show as not fitting, not
                // as quietly due in the past.
                for (i, t) in topics.iter().enumerate() { if !is_done(i) { topic_due.remove(&t.id); } }
                ti = (0..topics.len()).find(|i| !is_done(*i)).unwrap_or(topics.len());
                rem = topics.get(ti).map(|t| t.hours).unwrap_or(0.0);
            }
            let mut budget = cfg.hours_for(def, w.n, &w.kind);
            let mut entries = Vec::new();
            // Fixed weekly sessions come first and are not charged to the budget.
            if w.kind == "term" {
                for wk in &def.weekly {
                    entries.push(Entry::Fixed { label: wk.label.clone(), hours: wk.hours, note: wk.note.clone() });
                }
            }
            // School sets the pace: no content is poured, the hours are recall.
            if def.pace == "school" {
                while budget > 0.01 {
                    let take = budget.min(1.0);
                    entries.push(Entry::Rev { section: def.sections[cycle % def.sections.len()].to_string(), hours: take });
                    cycle += 1; budget -= take;
                }
                w.plan.insert(def.id.clone(), entries);
                continue;
            }
            let catching_up = catch_up.is_some_and(|cu| w.n >= cu.from_week);
            while budget > 0.01 {
                // Skip past anything already finished rather than spending the
                // week's hours re-teaching it.
                if catching_up && ti < topics.len() && is_done(ti) {
                    ti += 1;
                    rem = topics.get(ti).map(|t| t.hours).unwrap_or(0.0);
                    continue;
                }
                if ti < topics.len() {
                    let t = &topics[ti];
                    let take = budget.min(rem);
                    let done_before = t.hours - rem;
                    rem -= take; budget -= take;
                    let is_final = rem <= 0.01;
                    entries.push(Entry::Topic { topic_id: t.id.clone(), hours: take, part_no: done_before.floor() as u32 + 1, parts: t.hours.ceil() as u32, is_final });
                    topic_due.insert(t.id.clone(), w.n);
                    if is_final {
                        if ti + 1 == topics.len() { content_ends_week = w.n; }
                        ti += 1;
                        rem = topics.get(ti).map(|t| t.hours).unwrap_or(0.0);
                    }
                } else {
                    let take = budget.min(1.0);
                    entries.push(Entry::Rev { section: def.sections[cycle % def.sections.len()].to_string(), hours: take });
                    cycle += 1; budget -= take;
                }
            }
            w.plan.insert(def.id.clone(), entries);
        }
        subjects.push(Subject {
            id: def.id.clone(), name: def.name.clone(), full: def.full.clone(), color: def.color.clone(),
            papers: def.papers.clone(), spec: def.spec.clone(),
            sections: def.sections.clone(), topics, content_ends_week, resources: def.resources.clone(),
            pace: if def.pace == "school" { "school".into() } else { "ahead".into() },
            spec_refs: crate::course::spec_refs_for(&def.id).iter().map(|s| s.to_string()).collect(),
        });
    }

    // Tests: fortnightly retrieval, half-term timed sections, termly mocks, Easter full papers.
    let ids: Vec<String> = cfg.subjects.iter().map(|s| s.id.clone()).collect();
    let mut r = 0usize;
    let mut tests = Vec::new();
    let last_year = weeks.last().map(|w| w.year).unwrap_or(11);
    for wi in 0..weeks.len() {
        // A subject counts as started once it has had at least one session.
        let started: Vec<String> = ids.iter()
            .filter(|s| weeks[..=wi].iter().any(|x| x.plan.get(*s).map_or(false, |e| !e.is_empty())))
            .cloned().collect();
        if started.is_empty() { continue; }
        let w = &weeks[wi];
        let term_end = w.kind == "term" && w.last;
        let is_term_end_block = matches!(w.block.as_str(), "A2" | "S2" | "U2");
        let final_year = w.year == last_year;

        if w.kind == "term" && w.term_week % 2 == 0 {
            tests.push(Test { id: format!("ret-{}", w.n), week: w.n, kind: "retrieval".into(), title: "Retrieval test".into(), mins: 30, subjects: started.clone(),
                desc: "Two questions per subject from anything you have covered so far. From memory, no notes. Give yourself a % per subject.".into() });
        }
        if term_end && !is_term_end_block {
            let subs: Vec<String> = if final_year { started.clone() } else {
                (0..2.min(started.len())).map(|k| started[(r + k) % started.len()].clone()).collect()
            };
            r += 2;
            tests.push(Test { id: format!("timed-{}", w.n), week: w.n, kind: "timed".into(), title: "Timed paper section".into(), mins: if final_year { 60 } else { 45 }, subjects: subs,
                desc: "A chunk of a past paper under exam timing, only on topics you have covered. Mark it with the official mark scheme.".into() });
        }
        if term_end && is_term_end_block {
            let full = final_year;
            tests.push(Test { id: format!("mock-{}", w.n), week: w.n, kind: "mock".into(), title: if full { "Full mock" } else { "End-of-term mock" }.into(), mins: if full { 240 } else { 90 }, subjects: started.clone(),
                desc: if full { "Full past papers in every subject, timed, spread over the week." } else { "One timed past paper per subject. Skip questions on topics you have not done yet. Score = marks / marks available." }.into() });
        }
        if w.kind == "holiday" && final_year && w.label.starts_with("Easter") && w.first {
            tests.push(Test { id: format!("mock-{}", w.n), week: w.n, kind: "mock".into(), title: "Easter full papers".into(), mins: 240, subjects: ids.clone(),
                desc: "One full paper per subject over the two weeks, timed. Fix every dropped mark.".into() });
        }
    }

    let exam_start = weeks.iter().find(|w| w.kind == "exam").map(|w| w.start.clone())
        .or_else(|| weeks.last().map(|w| w.start.clone())).unwrap_or_default();

    // Anything that could not be fitted in. Always empty for the plan as laid
    // out; only a catch-up can squeeze more work in than there are weeks for.
    let unscheduled: Vec<String> = subjects.iter()
        .filter(|s| s.pace != "school")
        .flat_map(|s| s.topics.iter())
        .filter(|t| !topic_due.contains_key(&t.id))
        .map(|t| t.id.clone())
        .collect();

    Plan { subjects, weeks, tests, topic_due, review_gaps: cfg.review_gaps.clone(), exam_start, unscheduled }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_topic_is_scheduled_and_content_ends_before_the_final_year() {
        let plan = build(&PlanConfig::default());
        assert_eq!(plan.weeks.len(), 94);
        for s in &plan.subjects {
            if s.pace == "school" {
                // School-paced: nothing is poured, and that is not a failure to fit.
                for t in &s.topics { assert!(!plan.topic_due.contains_key(&t.id), "{} scheduled despite school pace", t.id); }
                assert_eq!(s.content_ends_week, 0, "{} has content pouring", s.id);
                continue;
            }
            for t in &s.topics { assert!(plan.topic_due.contains_key(&t.id), "{} unscheduled", t.id); }
            assert!(s.content_ends_week > 0 && s.content_ends_week <= 52, "{} ends week {}", s.id, s.content_ends_week);
        }
        // The subjects that run ahead are the ones agreed with the user.
        let ahead: Vec<&str> = plan.subjects.iter().filter(|s| s.pace == "ahead").map(|s| s.id.as_str()).collect();
        assert_eq!(ahead, vec!["bus", "econ", "cs"]);
        // School-paced subjects still get weekly recall sessions in term.
        let first_term = plan.weeks.iter().find(|w| w.kind == "term").unwrap();
        assert!(!first_term.plan["maths"].is_empty(), "maths has no recall session");
        // And the fixed timed piece for English Language appears every term week.
        assert!(plan.weeks.iter().filter(|w| w.kind == "term").all(|w|
            w.plan["englang"].iter().any(|e| matches!(e, Entry::Fixed { .. }))), "timed piece missing from a term week");
        assert!(plan.tests.iter().filter(|t| t.kind == "retrieval").count() > 30);
        assert_eq!(plan.exam_start, "2028-05-08");
    }

    /// Every built-in subject at once, catalog included, so checks on written
    /// content reach the subjects a new profile does not start with.
    fn every_subject() -> PlanConfig {
        PlanConfig { subjects: crate::config::catalog(), ..PlanConfig::default() }
    }

    /// A mistyped key (several contain en-dashes) would silently write
    /// objectives for a topic that does not exist, so check every one lands.
    #[test]
    fn every_written_topic_matches_a_real_one() {
        let plan = build(&every_subject());
        let ids: Vec<&str> = plan.subjects.iter().flat_map(|s| s.topics.iter().map(|t| t.id.as_str())).collect();
        for (id, _, _) in crate::course::TOPIC_DETAIL {
            assert!(ids.contains(id), "course notes written for unknown topic {id}");
        }
    }

    use crate::course::{covers, spec_refs_for};

    /// The point of the whole exercise: nothing in a checked specification is
    /// silently absent from the plan.
    #[test]
    fn every_spec_reference_is_covered() {
        let plan = build(&every_subject());
        for s in plan.subjects.iter().filter(|s| !s.spec_refs.is_empty()) {
            for r in &s.spec_refs {
                assert!(
                    s.topics.iter().any(|t| covers(&t.code, r)),
                    "{}: spec reference {r} is not covered by any topic",
                    s.id
                );
            }
        }
    }

    /// And nothing is scheduled that is not in the specification.
    #[test]
    fn no_topic_invents_a_spec_reference() {
        let plan = build(&every_subject());
        for s in plan.subjects.iter().filter(|s| !s.spec_refs.is_empty()) {
            for t in &s.topics {
                assert!(
                    s.spec_refs.iter().any(|r| covers(&t.code, r)),
                    "{}: topic \"{}\" claims spec reference {} which is not in the specification",
                    s.id, t.title, t.code
                );
            }
        }
    }

    /// A subject is only marked as checked if we really do have its reference list.
    #[test]
    fn checked_subjects_are_the_ones_with_reference_lists() {
        let plan = build(&every_subject());
        for s in &plan.subjects {
            assert_eq!(
                s.spec_refs.is_empty(),
                spec_refs_for(&s.id).is_empty(),
                "{} disagrees with its reference list", s.id
            );
        }
        for id in ["maths", "fpm", "bus", "econ", "cs"] {
            assert!(
                !spec_refs_for(id).is_empty(),
                "{id} has been checked against its specification - do not drop its reference list"
            );
        }
    }

    #[test]
    fn the_reference_matcher_does_not_confuse_1_1_with_1_10() {
        assert!(covers("1.1", "1.1"));
        assert!(covers("4.8a", "4.8"));
        assert!(!covers("1.10", "1.1"), "1.10 is its own reference, not part of 1.1");
        assert!(!covers("1.11", "1.1"));
        assert!(covers("1.10", "1.10"));
        assert!(covers("N1-3", "N1") && covers("N1-3", "N3") && !covers("N1-3", "N4"));
        assert!(covers("A18-19b", "A19") && !covers("A18-19b", "N18") && !covers("N1-3", "N"));
    }

    /// Subjects checked against their specification must be written up in full.
    #[test]
    fn checked_subjects_are_fully_written_up() {
        let plan = build(&PlanConfig::default());
        for s in plan.subjects.iter().filter(|s| !s.spec_refs.is_empty()) {
            for t in &s.topics {
                assert!(t.objectives.len() >= 3, "{} has only {} objectives", t.id, t.objectives.len());
                assert!(!t.watch.is_empty(), "{} has no watch-out", t.id);
            }
        }
    }

    /// Falling behind should move the remaining work forward, not leave you
    /// permanently marked as behind.
    #[test]
    fn catching_up_repours_unfinished_work_from_the_chosen_week() {
        let cfg = PlanConfig::default();
        let plain = build(&cfg);
        // Economics runs ahead, so it is the one with content to re-pour.
        let maths: Vec<String> = plain.subjects.iter().find(|s| s.id == "econ").unwrap()
            .topics.iter().map(|t| t.id.clone()).collect();

        // Pretend it is week 20 and only the first three economics topics are done.
        let cu = CatchUp { from_week: 20, done: maths[..3].to_vec() };
        let caught = build_with(&cfg, Some(&cu));

        // The fourth topic — the earliest unfinished one — is now due again from
        // week 20, rather than being stranded in the past.
        assert!(plain.topic_due[&maths[3]] < 20, "topic 4 was originally due before week 20");
        assert!(caught.topic_due[&maths[3]] >= 20, "topic 4 should be re-poured from week 20");

        // Finished topics are not re-taught.
        for id in &maths[..3] {
            assert!(caught.topic_due[id] < 20, "{id} is done and should not come round again");
        }

        // History before the catch-up week is untouched.
        for n in 0..19 {
            assert_eq!(
                plain.weeks[n].plan["econ"].len(), caught.weeks[n].plan["econ"].len(),
                "week {} should be unchanged", n + 1
            );
        }
        // And everything still gets scheduled somewhere.
        for id in &maths { assert!(caught.topic_due.contains_key(id), "{id} unscheduled after catch-up"); }
    }

    /// The plan as laid out fits, so nothing is ever reported as not fitting.
    #[test]
    fn the_plan_as_laid_out_fits_entirely() {
        assert!(build(&PlanConfig::default()).unscheduled.is_empty());
    }

    /// A new profile gets the starter ten and nothing else; every other
    /// built-in subject waits in the catalog, and each one fits a plan when
    /// added alongside them.
    #[test]
    fn new_profiles_start_with_the_starter_subjects_and_the_catalog_adds_the_rest() {
        let ids: Vec<String> = PlanConfig::default().subjects.iter().map(|s| s.id.clone()).collect();
        assert_eq!(ids, STARTER.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        let catalog = crate::config::catalog();
        assert_eq!(catalog.len(), SUBJECTS.len());
        for c in catalog.iter().filter(|c| !STARTER.contains(&c.id.as_str())) {
            let mut cfg = PlanConfig::default();
            cfg.subjects.push(c.clone());
            let plan = build(&cfg);
            assert!(plan.unscheduled.is_empty(), "{} does not fit alongside the starter subjects", c.id);
        }
    }

    /// Catching up very late cannot invent time. Rather than silently dropping
    /// topics, the plan reports exactly which ones no longer fit.
    #[test]
    fn catching_up_too_late_reports_what_no_longer_fits() {
        let cfg = PlanConfig::default();
        let caught = build_with(&cfg, Some(&CatchUp { from_week: 70, done: vec![] }));
        assert!(
            !caught.unscheduled.is_empty(),
            "restarting every ahead subject from week 70 cannot fit — that must be reported"
        );
        // Every topic is either scheduled or named as not fitting; none vanish.
        for s in caught.subjects.iter().filter(|s| s.pace != "school") {
            for t in &s.topics {
                assert!(
                    caught.topic_due.contains_key(&t.id) || caught.unscheduled.contains(&t.id),
                    "{} was neither scheduled nor reported as not fitting", t.id
                );
            }
        }
    }

    /// A realistic catch-up — a term lost, most of the year still ahead — fits.
    #[test]
    fn catching_up_after_losing_a_term_still_fits() {
        let cfg = PlanConfig::default();
        let done: Vec<String> = build(&cfg).subjects.iter()
            .flat_map(|s| s.topics.iter().filter(|t| build(&cfg).topic_due.get(&t.id).is_some_and(|w| *w <= 8)).map(|t| t.id.clone()))
            .collect();
        let caught = build_with(&cfg, Some(&CatchUp { from_week: 16, done }));
        assert!(caught.unscheduled.is_empty(), "losing weeks 9-15 should still leave room: {:?}", caught.unscheduled);
    }

    /// And catching up when everything is done leaves only past-paper practice.
    #[test]
    fn catching_up_with_everything_done_falls_through_to_practice() {
        let cfg = PlanConfig::default();
        let all: Vec<String> = build(&cfg).subjects.iter()
            .flat_map(|s| s.topics.iter().map(|t| t.id.clone())).collect();
        let caught = build_with(&cfg, Some(&CatchUp { from_week: 20, done: all }));
        let week25 = &caught.weeks[24];
        assert!(
            week25.plan.values().flatten().all(|e| matches!(e, Entry::Rev { .. } | Entry::Fixed { .. })),
            "with every topic done, later weeks should be revision only"
        );
    }

    /// How many lettered statements each 4MA1 reference has, counted by machine
    /// from the specification PDF rather than by eye. The parser walks the
    /// lettering — which runs A, B, C... and restarts at each reference — so a
    /// dropped statement shows up as a break in the sequence. If the table in
    /// `statements.rs` ever disagrees with these counts, something was lost.
    const EXPECTED_STATEMENTS: &[(&str, usize)] = &[
        ("1.1", 8), ("1.2", 9), ("1.3", 6), ("1.4", 8), ("1.5", 9), ("1.6", 9),
        ("1.7", 5), ("1.8", 5), ("1.9", 2), ("1.10", 3), ("1.11", 1),
        ("2.1", 5), ("2.2", 11), ("2.3", 7), ("2.4", 2), ("2.5", 1), ("2.6", 3),
        ("2.7", 5), ("2.8", 7),
        ("3.1", 6), ("3.2", 4), ("3.3", 16), ("3.4", 5),
        ("4.1", 4), ("4.2", 7), ("4.3", 1), ("4.4", 7), ("4.5", 4), ("4.6", 5),
        ("4.7", 2), ("4.8", 9), ("4.9", 6), ("4.10", 7), ("4.11", 5),
        ("5.1", 7), ("5.2", 13),
        ("6.1", 6), ("6.2", 8), ("6.3", 14),
    ];

    /// The whole point: every statement the specification makes is on the
    /// checklist, in the right number, against a topic that teaches it.
    #[test]
    fn every_maths_statement_is_accounted_for() {
        let plan = build(&PlanConfig::default());
        let maths = plan.subjects.iter().find(|s| s.id == "maths").expect("maths");
        for (r, expected) in EXPECTED_STATEMENTS {
            let found: usize = maths.topics.iter()
                .filter(|t| covers(&t.code, r))
                .map(|t| t.statements.len())
                .sum();
            assert_eq!(
                found, *expected,
                "spec reference {r} has {expected} lettered statements but the app lists {found}"
            );
        }
        let total: usize = maths.topics.iter().map(|t| t.statements.len()).sum();
        assert_eq!(total, 242, "4MA1 has 242 statements across both tiers");
    }

    /// The three sciences, numbered section.n by Pearson with a B, C or P suffix
    /// on separate-science-only statements. Section totals are the highest
    /// statement number in each section of the PDF, checked to have no gaps -
    /// so they come from the document, not from the table they are testing.
    const EXPECTED_SCIENCE: &[(&str, usize, usize, &[(&str, usize)])] = &[
        ("bio", 176, 42, &[("1", 4), ("2", 95), ("3", 39), ("4", 18), ("5", 20)]),
        ("chem", 182, 52, &[("1", 60), ("2", 50), ("3", 22), ("4", 50)]),
        // 195, not 191: four statements sit on a different line from their
        // number in the PDF and a first-line count misses them.
        ("phys", 195, 48, &[("1", 33), ("2", 28), ("3", 29), ("4", 19), ("5", 22), ("6", 20), ("7", 26), ("8", 18)]),
    ];

    #[test]
    fn every_science_statement_is_accounted_for() {
        let plan = build(&PlanConfig::default());
        for (id, total, separate, sections) in EXPECTED_SCIENCE {
            let s = plan.subjects.iter().find(|s| s.id == *id).expect(id);
            let all: Vec<&Statement> = s.topics.iter().flat_map(|t| t.statements.iter()).collect();
            assert_eq!(all.len(), *total, "{id} should list {total} statements");
            let sep = all.iter().filter(|st| crate::statements::separate_only(&st.code)).count();
            assert_eq!(sep, *separate, "{id} should have {separate} separate-science-only statements");
            for (sec, n) in *sections {
                let found = all.iter().filter(|st| st.code.split('.').next() == Some(*sec)).count();
                assert_eq!(found, *n, "{id} section {sec} should have {n} statements");
            }
            // Numbering is dense: every number from 1 to the section maximum appears once.
            for (sec, n) in *sections {
                for k in 1..=*n {
                    let hits = all.iter().filter(|st| st.code.trim_end_matches(['B', 'C', 'P']) == format!("{sec}.{k}")).count();
                    assert_eq!(hits, 1, "{id} statement {sec}.{k} appears {hits} times");
                }
            }
        }
    }

    /// Cambridge numbers every Economics sub-point section.subsection.n. These
    /// counts are the highest n under each subsection of the 2027-2029
    /// syllabus PDF (113 in all), so the table is checked against the document.
    const EXPECTED_ECON: &[(&str, usize)] = &[
        ("1.1", 3), ("1.2", 2), ("1.3", 2), ("1.4", 4),
        ("2.1", 1), ("2.2", 3), ("2.3", 3), ("2.4", 3), ("2.5", 2), ("2.6", 5), ("2.7", 3), ("2.8", 2), ("2.9", 4), ("2.10", 3),
        ("3.1", 2), ("3.2", 1), ("3.3", 5), ("3.4", 3), ("3.5", 3), ("3.6", 5), ("3.7", 2),
        ("4.1", 1), ("4.2", 6), ("4.3", 3), ("4.4", 3), ("4.5", 5), ("4.6", 5), ("4.7", 5),
        ("5.1", 2), ("5.2", 3), ("5.3", 2), ("5.4", 1),
        ("6.1", 2), ("6.2", 6), ("6.3", 4), ("6.4", 4),
    ];

    #[test]
    fn every_economics_sub_point_is_accounted_for() {
        let plan = build(&PlanConfig::default());
        let econ = plan.subjects.iter().find(|s| s.id == "econ").expect("econ");
        let mut total = 0;
        for (sub, n) in EXPECTED_ECON {
            let t = econ.topics.iter().find(|t| t.code == *sub).unwrap_or_else(|| panic!("no topic {sub}"));
            assert_eq!(t.statements.len(), *n, "econ {sub} should carry {n} sub-points");
            // Dense: sub.1 .. sub.n each exactly once.
            for k in 1..=*n {
                let code = format!("{sub}.{k}");
                assert_eq!(t.statements.iter().filter(|s| s.code == code).count(), 1, "econ sub-point {code} missing or doubled");
            }
            total += n;
        }
        assert_eq!(total, 113);
        assert_eq!(econ.topics.iter().map(|t| t.statements.len()).sum::<usize>(), 113);
    }

    /// Computer Science - OCR J277. OCR numbers sub-topics but not the bullets
    /// under them, so the letters are ours; the bullet count per sub-topic was
    /// extracted from the specification PDF (version 3.1) by machine.
    const EXPECTED_CS: &[(&str, usize)] = &[
        ("1.1.1", 3), ("1.1.2", 1), ("1.1.3", 2),
        ("1.2.1", 6), ("1.2.2", 4), ("1.2.3", 3), ("1.2.4", 13), ("1.2.5", 2),
        ("1.3.1", 6), ("1.3.2", 6),
        ("1.4.1", 1), ("1.4.2", 1),
        ("1.5.1", 1), ("1.5.2", 2),
        ("1.6.1", 2),
        ("2.1.1", 1), ("2.1.2", 5), ("2.1.3", 2),
        ("2.2.1", 4), ("2.2.2", 1), ("2.2.3", 7),
        ("2.3.1", 3), ("2.3.2", 5),
        ("2.4.1", 4),
        ("2.5.1", 3), ("2.5.2", 1),
    ];

    #[test]
    fn every_computer_science_statement_is_accounted_for() {
        let plan = build(&PlanConfig::default());
        let cs = plan.subjects.iter().find(|s| s.id == "cs").expect("cs");
        assert_eq!(cs.topics.len(), EXPECTED_CS.len(), "one topic per OCR sub-topic");
        for (sub, n) in EXPECTED_CS {
            let t = cs.topics.iter().find(|t| t.code == *sub).unwrap_or_else(|| panic!("no topic {sub}"));
            assert_eq!(t.statements.len(), *n, "cs {sub} should carry {n} bullet points");
            for (i, s) in t.statements.iter().enumerate() {
                let want = format!("{}{}", sub, (b'a' + i as u8) as char);
                assert_eq!(s.code, want, "cs {sub} bullet {} out of sequence", s.code);
            }
        }
        assert_eq!(cs.topics.iter().map(|t| t.statements.len()).sum::<usize>(), 89);
    }

    /// Further Pure: ten sections lettered A, B, C... in the PDF, sequence
    /// checked - so the letter count is the count.
    const EXPECTED_FPM: &[(&str, usize)] = &[
        ("1", 4), ("2", 3), ("3", 5), ("4", 2), ("5", 2), ("6", 1), ("7", 6), ("8", 5), ("9", 7), ("10", 8),
    ];

    #[test]
    fn every_further_pure_statement_is_accounted_for() {
        let plan = build(&PlanConfig::default());
        let fpm = plan.subjects.iter().find(|s| s.id == "fpm").expect("fpm");
        let all: Vec<&Statement> = fpm.topics.iter().flat_map(|t| t.statements.iter()).collect();
        for (sec, n) in EXPECTED_FPM {
            for k in 0..*n {
                let code = format!("{sec}.{}", (b'A' + k as u8) as char);
                assert_eq!(all.iter().filter(|s| s.code == code).count(), 1, "fpm statement {code} missing or doubled");
            }
            assert_eq!(all.iter().filter(|s| s.code.starts_with(&format!("{sec}."))).count(), *n, "fpm section {sec} should have {n}");
        }
        assert_eq!(all.len(), 43);
    }

    /// AQA does not number Business outcomes, so this cannot be checked against
    /// the document the way the others are. What it can check: every topic has
    /// outcomes, the letters run a, b, c... with no holes, and there are no
    /// duplicates.
    #[test]
    fn business_outcomes_are_lettered_without_holes() {
        let plan = build(&PlanConfig::default());
        let bus = plan.subjects.iter().find(|s| s.id == "bus").expect("bus");
        for t in &bus.topics {
            assert!(!t.statements.is_empty(), "bus {} has no outcomes", t.code);
            for (i, s) in t.statements.iter().enumerate() {
                let want = format!("{}{}", t.code, (b'a' + i as u8) as char);
                assert_eq!(s.code, want, "bus {} outcome {} is out of sequence", t.code, s.code);
            }
        }
        assert_eq!(bus.topics.iter().map(|t| t.statements.len()).sum::<usize>(), 125);
    }

    /// A statement filed against a topic that does not exist would silently
    /// vanish from the checklist.
    #[test]
    fn every_statement_lands_on_a_real_topic() {
        let plan = build(&PlanConfig::default());
        for id in ["maths", "bio", "chem", "phys", "econ", "bus", "cs", "fpm"] {
            let s = plan.subjects.iter().find(|s| s.id == id).expect(id);
            for (topic, code, _) in crate::statements::table_for(id) {
                assert!(
                    s.topics.iter().any(|t| t.code == *topic),
                    "{id} statement {code} is filed against topic {topic}, which does not exist"
                );
            }
        }
    }

    /// Within a reference, each tier's letters must run A, B, C with no holes —
    /// the property that makes a missing statement detectable at all.
    #[test]
    fn statement_letters_have_no_holes() {
        use std::collections::BTreeMap;
        let mut by_ref: BTreeMap<(String, char), Vec<char>> = BTreeMap::new();
        for (topic, code, _) in crate::statements::MATHS {
            let r: String = topic.trim_end_matches(|c: char| c.is_ascii_alphabetic()).to_string();
            let mut ch = code.chars();
            let tier = ch.next().expect("tier letter");
            let letter = ch.next().expect("statement letter");
            by_ref.entry((r, tier)).or_default().push(letter);
        }
        for ((r, tier), mut letters) in by_ref {
            letters.sort_unstable();
            for (i, c) in letters.iter().enumerate() {
                let want = (b'A' + i as u8) as char;
                assert_eq!(
                    *c, want,
                    "{r} tier {tier}: expected {want} at position {i} but found {c} — a statement is missing"
                );
            }
        }
    }

    /// Every subject must give a session somewhere to go.
    #[test]
    fn every_subject_ships_with_working_links() {
        for s in &build(&PlanConfig::default()).subjects {
            assert!(!s.resources.is_empty(), "{} has no links", s.id);
            for r in &s.resources {
                assert!(r.url.starts_with("https://"), "{} link {} is not https", s.id, r.label);
            }
        }
    }

    /// Dumps the default plan and config as JSON so the interface can be driven
    /// outside the app. Run with `cargo test -- --ignored dump_json`.
    #[test]
    #[ignore]
    fn dump_json() {
        let cfg = PlanConfig::default();
        let dir = std::env::temp_dir();
        std::fs::write(dir.join("g9-plan.json"), serde_json::to_string(&build(&cfg)).unwrap()).unwrap();
        std::fs::write(dir.join("g9-config.json"), serde_json::to_string(&cfg).unwrap()).unwrap();
    }

    /// Someone who strips the plan down to one subject should still get a plan.
    #[test]
    fn a_minimal_edited_config_still_schedules() {
        let mut cfg = PlanConfig::default();
        cfg.subjects.retain(|s| s.id == "econ");
        cfg.subjects[0].topics.truncate(3);
        cfg.sanitise();
        let plan = build(&cfg);
        assert_eq!(plan.subjects.len(), 1);
        assert_eq!(plan.subjects[0].topics.len(), 3);
        for t in &plan.subjects[0].topics { assert!(plan.topic_due.contains_key(&t.id)); }
        assert!(!plan.tests.is_empty());
    }

    /// Nonsense typed into the editor must not produce an unusable plan.
    #[test]
    fn sanitise_repairs_a_broken_config() {
        let mut cfg = PlanConfig::default();
        cfg.blocks.clear();
        cfg.subjects[0].rates.clear();
        cfg.review_gaps.clear();
        cfg.subjects[0].topics[0].url = "javascript:alert(1)".into();
        cfg.subjects[0].resources.push(crate::config::ResourceLink { label: "Bad".into(), url: "file:///C:/".into() });
        cfg.sanitise();
        assert!(!cfg.blocks.is_empty());
        assert!(!cfg.subjects[0].rates.is_empty());
        assert!(!cfg.review_gaps.is_empty());
        assert_eq!(cfg.subjects[0].topics[0].url, "", "non-http topic links must be dropped");
        assert!(cfg.subjects[0].resources.iter().all(|r| r.url.starts_with("http")), "non-http resources must be dropped");
        assert!(!build(&cfg).weeks.is_empty());
    }
}
