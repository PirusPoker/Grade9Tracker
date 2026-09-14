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

pub const SUBJECTS: &[SubjectDef] = &[
    SubjectDef {
        id: "maths", name: "Maths", full: "Mathematics A (4MA1) Higher", color: "var(--maths)",
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
        id: "fpm", name: "Further Maths", full: "Further Pure Mathematics (4PM1)", color: "var(--fpm)",
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
        id: "bus", name: "Business", full: "Business (AQA 8132)", color: "var(--bus)",
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
        id: "econ", name: "Economics", full: "Economics (Cambridge 0987)", color: "var(--econ)",
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
        id: "cs", name: "Computer Science", full: "Computer Science (4CP0)", color: "var(--cs)",
        papers: "Paper 1 theory (2 h) + Paper 2 practical in Python (3 h), 80 marks each",
        spec: "https://qualifications.pearson.com/en/qualifications/edexcel-international-gcses/international-gcse-computer-science-2017.html",
        sections: &["1 Problem solving", "2 Programming", "3 Data", "4 Computers", "5 Communication & the internet", "6 The bigger picture"],
        topics: &[
            ("1.1a", "Algorithms: flowcharts, pseudocode and trace tables", 2.0),
            ("1.1b", "Decomposition and abstraction", 1.0),
            ("1.1c", "Standard algorithms: linear and binary search, bubble and merge sort", 2.0),
            ("2.1", "Developing code: error types, testing and debugging", 2.0),
            ("2.2", "Programming constructs: sequence, selection and iteration", 2.0),
            ("2.3", "Data types and structures: lists, 2D lists, strings and records", 2.0),
            ("2.4", "Input, output, validation and file handling", 2.0),
            ("2.5", "Operators: arithmetic, relational and logical", 1.0),
            ("2.6", "Subprograms: functions, procedures, parameters and scope", 2.0),
            ("3.1", "Binary and hexadecimal: conversion, addition, two's complement and shifts", 2.0),
            ("3.2", "Data representation: text, images and sound", 1.0),
            ("3.3", "Data storage, file size and compression", 1.0),
            ("3.4", "Encryption", 1.0),
            ("4.1", "Machines and computational models", 1.0),
            ("4.2", "Hardware: CPU, fetch–decode–execute, memory and storage", 2.0),
            ("4.3", "Logic gates and truth tables", 1.0),
            ("4.4", "Software: operating systems and utility software", 1.0),
            ("4.5", "Programming languages and translators", 1.0),
            ("5.1", "Networks: topologies, protocols, layers and transmission", 2.0),
            ("5.2", "Network security: threats and protection", 1.0),
            ("5.3", "The internet and the world wide web", 1.0),
            ("6.1", "Emerging trends, issues and impact", 1.0),
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
        id: "bio", name: "Biology", full: "Biology (4BI1)", color: "var(--bio)",
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
        id: "chem", name: "Chemistry", full: "Chemistry (4CH1)", color: "var(--chem)",
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
        id: "phys", name: "Physics", full: "Physics (4PH1)", color: "var(--phys)",
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

    /// A mistyped key (several contain en-dashes) would silently write
    /// objectives for a topic that does not exist, so check every one lands.
    #[test]
    fn every_written_topic_matches_a_real_one() {
        let plan = build(&PlanConfig::default());
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
        let plan = build(&PlanConfig::default());
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
        let plan = build(&PlanConfig::default());
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
        let plan = build(&PlanConfig::default());
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

    /// A statement filed against a topic that does not exist would silently
    /// vanish from the checklist.
    #[test]
    fn every_statement_lands_on_a_real_topic() {
        let plan = build(&PlanConfig::default());
        for id in ["maths", "bio", "chem", "phys"] {
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
