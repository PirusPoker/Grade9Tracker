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
    // Computer Science (4CP0), Issue 1.
    ("cs", &[
        "1.1", "1.2",
        "2.1", "2.2", "2.3", "2.4", "2.5", "2.6",
        "3.1", "3.2", "3.3", "3.4",
        "4.1", "4.2", "4.3", "4.4", "4.5",
        "5.1", "5.2", "5.3",
        "6.1",
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
pub fn covers(code: &str, spec_ref: &str) -> bool {
    code == spec_ref
        || (code.starts_with(spec_ref)
            && !code[spec_ref.len()..].is_empty()
            && code[spec_ref.len()..].chars().all(|c| c.is_ascii_alphabetic()))
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

    // ---------- Computer Science (4CP0) ----------
    ("cs:1.1a", &[
        "Say what an algorithm is and what it is for",
        "Read and write algorithms as flowcharts and as pseudocode",
        "Complete a trace table to work out what an algorithm does with given inputs",
        "Find and fix a logic error in someone else's algorithm",
    ], "Fill in a trace table row by row, one line of code at a time. Jumping to what you think the answer is loses every mark when it is wrong."),

    ("cs:1.2", &[
        "Break a problem down into smaller sub-problems",
        "Explain abstraction as removing detail that does not matter",
        "Identify what to keep and what to leave out when modelling a problem",
        "Explain why decomposition and abstraction make a program easier to write and to maintain",
    ], "Abstraction is leaving detail out on purpose. Answers that describe it as \"making things simpler\" without saying what is removed score little."),

    ("cs:1.1c", &[
        "Carry out and explain a linear search and a binary search",
        "Carry out and explain a bubble sort and a merge sort",
        "Say why binary search needs sorted data, and compare it with linear search",
        "Compare algorithms by how much work they do as the data grows",
    ], "Binary search only works on sorted data. Applying it to an unsorted list is the standard trap in these questions."),

    ("cs:2.1", &[
        "Tell syntax, runtime and logic errors apart from what the program does",
        "Write and use test data: normal, boundary and erroneous",
        "Build a test plan with expected and actual results",
        "Debug a program methodically instead of by guessing",
    ], "Boundary test data means the values right at the edge of what is allowed — and just past it. Picking ordinary values instead misses the point of the test."),

    ("cs:2.2", &[
        "Write programs using sequence",
        "Use selection: if, elif and else, including nested conditions",
        "Use iteration: for and while loops, and choose the right one",
        "Combine the three constructs to solve a problem",
    ], "Use a for loop when you know how many repeats, a while loop when you do not. Choosing wrong usually means an infinite loop or one that never runs."),

    ("cs:2.3", &[
        "Use the data types integer, real, Boolean, character and string, and say why each is needed",
        "Use lists and two-dimensional lists, and index them correctly",
        "Use string handling: length, slicing, concatenation, case conversion",
        "Use records to hold related fields together",
    ], "Indexes start at 0, so the last item of a list of n is at n − 1. Off-by-one on the last element is the most common bug in these questions."),

    ("cs:2.4", &[
        "Take input from the user and produce clear output",
        "Convert between data types when reading input",
        "Validate input with range, type, length and presence checks",
        "Read from and write to a text file",
    ], "input() always returns a string. Doing arithmetic on it without int() or float() is the error that breaks most exam programs."),

    ("cs:2.5", &[
        "Use the arithmetic operators, including integer division and modulus",
        "Use the relational operators to build conditions",
        "Use the logical operators AND, OR and NOT",
        "Work out the order in which operators are applied in an expression",
    ], "Use == to compare and = to assign. Writing = inside an if is the classic slip."),

    ("cs:2.6", &[
        "Write and call a subprogram, and say why programs are broken into them",
        "Tell functions and procedures apart by whether they return a value",
        "Pass parameters and use returned values",
        "Explain local and global scope, and why local variables are usually better",
    ], "A variable made inside a subprogram disappears when it ends. Trying to use it outside is a scope error, not a typo."),

    ("cs:3.1", &[
        "Convert between denary, binary and hexadecimal",
        "Add binary numbers and identify overflow",
        "Represent negative numbers using two's complement",
        "Perform left and right binary shifts and say what each does to the value",
    ], "A left shift of one multiplies by two, a right shift divides by two. Getting the direction backwards is the usual error."),

    ("cs:3.2", &[
        "Explain how text is represented using character sets such as ASCII and Unicode",
        "Explain how images are represented, and the effect of resolution and colour depth",
        "Explain how sound is represented, and the effect of sample rate and bit depth",
        "Calculate the file size of an image or a sound file",
    ], "Image file size is width × height × colour depth, and the answer is in bits. Convert to bytes before you compare it with anything."),

    ("cs:3.3", &[
        "Convert between bits, bytes, kilobytes, megabytes, gigabytes and terabytes",
        "Calculate the storage needed for a set of files",
        "Explain lossy and lossless compression and when each is appropriate",
        "Explain why compression is used and what it costs",
    ], "Lossy compression throws data away permanently. Recommending it for a program file or a document is wrong — that data cannot come back."),

    ("cs:3.4", &[
        "Explain why data is encrypted",
        "Apply a Caesar cipher to encrypt and decrypt a message",
        "Explain the idea of a key, and why key length matters",
        "Explain the weakness of simple substitution ciphers",
    ], "Encryption scrambles data so only a key can read it. It does not stop the data being stolen, only being understood."),

    ("cs:4.1", &[
        "Explain the input–process–output model",
        "Compare sequential, parallel and other computational models",
        "Explain what computational modelling is used for and why",
        "Explain the limits of a model",
    ], "A model simplifies reality. Any evaluation should say what the model leaves out and why that matters."),

    ("cs:4.2", &[
        "Describe the parts of the CPU and what each does",
        "Explain the fetch–decode–execute cycle",
        "Explain what affects CPU performance: clock speed, cores and cache",
        "Compare RAM, ROM, virtual memory and secondary storage",
    ], "RAM is volatile and ROM is not. Describing RAM as where files are stored permanently is the standard error."),

    ("cs:4.3", &[
        "Recognise the AND, OR and NOT gates and their symbols",
        "Complete a truth table for a single gate",
        "Complete a truth table for a combination of gates",
        "Build a logic circuit from a written description",
    ], "Work a combined circuit out one gate at a time, writing each intermediate column in the table. Trying to do it in your head is where it goes wrong."),

    ("cs:4.4", &[
        "Explain what an operating system does: managing files, processes, hardware and users",
        "Explain the purpose of utility software",
        "Explain the difference between system software and application software",
        "Explain how the operating system sits between the user and the hardware",
    ], "Name the specific management job — files, memory, processes, peripherals — rather than saying the OS \"runs the computer\"."),

    ("cs:4.5", &[
        "Compare high-level and low-level languages",
        "Explain why programs must be translated into machine code",
        "Compare compilers and interpreters and their advantages",
        "Explain the role of an assembler",
    ], "A compiler translates the whole program before it runs; an interpreter translates line by line as it runs. That difference is what every question here turns on."),

    ("cs:5.1", &[
        "Explain why computers are connected in networks, and compare LAN and WAN",
        "Compare network topologies: star, mesh and bus",
        "Explain protocols and the idea of layers",
        "Compare wired and wireless transmission, and explain bandwidth and latency",
    ], "Protocols are a set of agreed rules, not hardware. Describing a protocol as a device is a common mix-up."),

    ("cs:5.2", &[
        "Explain why network security matters",
        "Describe threats: malware, phishing, brute force, denial of service, social engineering",
        "Explain protections: firewalls, encryption, authentication, access levels, backups",
        "Recommend the right protection for a stated threat",
    ], "Match the protection to the threat. A firewall does not stop a user handing over their password to a phishing email — training does."),

    ("cs:5.3", &[
        "Explain what the internet is and how it differs from the world wide web",
        "Explain IP addresses, DNS and how a page request is routed",
        "Explain the role of servers, clients and hosting",
        "Explain how web pages are built and delivered",
    ], "The internet is the network; the web is one service running on it. Treating the two as the same thing loses the mark."),

    ("cs:6.1", &[
        "Explain the environmental impact of computing, including energy use and e-waste",
        "Explain ethical issues: privacy, surveillance and the digital divide",
        "Explain the relevant legal issues, including data protection and computer misuse",
        "Discuss emerging trends and weigh their benefits against their risks",
    ], "These questions want a judgement, not a list. Give both sides, then say which weighs more and why."),

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
];

/// Objectives written for a topic, or an empty slice if it has none yet.
pub fn objectives_for(topic_id: &str) -> &'static [&'static str] {
    TOPIC_DETAIL.iter().find(|(id, _, _)| *id == topic_id).map(|(_, o, _)| *o).unwrap_or(&[])
}

/// The mark people drop on a topic, or an empty string.
pub fn watch_for(topic_id: &str) -> &'static str {
    TOPIC_DETAIL.iter().find(|(id, _, _)| *id == topic_id).map(|(_, _, w)| *w).unwrap_or("")
}
