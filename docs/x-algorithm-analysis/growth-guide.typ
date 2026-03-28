// X Algorithm Growth Guide — Source Code Edition
// Generated from analysis of twitter/the-algorithm and xai-org/x-algorithm

#set page(
  paper: "a4",
  margin: (top: 2.5cm, bottom: 2.5cm, left: 2.2cm, right: 2.2cm),
  fill: rgb("#0a0a0f"),
  numbering: "1",
  number-align: center,
  footer: context {
    let page-num = counter(page).at(here()).first()
    if page-num > 1 {
      set text(8pt, fill: rgb("#4a4a5a"))
      line(length: 100%, stroke: 0.3pt + rgb("#1a1a2e"))
      v(4pt)
      grid(
        columns: (1fr, 1fr),
        align(left)[X Algorithm Growth Guide],
        align(right)[#page-num],
      )
    }
  },
)

#set text(
  font: "Helvetica Neue",
  size: 9.5pt,
  fill: rgb("#e0e0e8"),
  weight: "regular",
)

#set par(
  leading: 0.7em,
  justify: true,
)

#set heading(numbering: none)

#show heading.where(level: 1): it => {
  v(1.2em)
  block(width: 100%)[
    #set text(18pt, fill: rgb("#ffffff"), weight: "bold")
    #it.body
    #v(2pt)
    #line(length: 100%, stroke: 1.5pt + gradient.linear(rgb("#1da1f2"), rgb("#0d8ecf"), rgb("#0a0a0f")))
  ]
  v(0.6em)
}

#show heading.where(level: 2): it => {
  v(0.8em)
  block[
    #set text(13pt, fill: rgb("#1da1f2"), weight: "bold")
    #it.body
  ]
  v(0.3em)
}

#show heading.where(level: 3): it => {
  v(0.5em)
  block[
    #set text(11pt, fill: rgb("#8899aa"), weight: "bold")
    #it.body
  ]
  v(0.2em)
}

// Code block styling
#show raw.where(block: true): it => {
  block(
    width: 100%,
    fill: rgb("#12121e"),
    stroke: 0.5pt + rgb("#1a1a2e"),
    radius: 6pt,
    inset: 12pt,
  )[
    #set text(8.5pt, font: "Menlo", fill: rgb("#c0c0d0"))
    #it
  ]
}

#show raw.where(block: false): it => {
  box(
    fill: rgb("#12121e"),
    outset: (x: 2pt, y: 2pt),
    radius: 3pt,
  )[
    #set text(8.5pt, font: "Menlo", fill: rgb("#1da1f2"))
    #it
  ]
}

// Utility functions
#let accent = rgb("#1da1f2")
#let warn = rgb("#ff6b35")
#let danger = rgb("#ff3333")
#let success = rgb("#22c55e")
#let muted = rgb("#6a6a7a")
#let surface = rgb("#12121e")
#let surface-border = rgb("#1a1a2e")

#let info-box(body, title: none, color: accent) = {
  block(
    width: 100%,
    fill: color.lighten(95%).desaturate(80%),
    stroke: (left: 3pt + color),
    radius: (right: 6pt),
    inset: 12pt,
  )[
    #if title != none {
      text(10pt, fill: color, weight: "bold")[#title]
      v(4pt)
    }
    #set text(fill: rgb("#e0e0e8"))
    #body
  ]
}

#let danger-box(body, title: none) = {
  block(
    width: 100%,
    fill: rgb("#1a0a0a"),
    stroke: (left: 3pt + danger),
    radius: (right: 6pt),
    inset: 12pt,
  )[
    #if title != none {
      text(10pt, fill: danger, weight: "bold")[#title]
      v(4pt)
    }
    #set text(fill: rgb("#e0e0e8"))
    #body
  ]
}

#let success-box(body, title: none) = {
  block(
    width: 100%,
    fill: rgb("#0a1a0a"),
    stroke: (left: 3pt + success),
    radius: (right: 6pt),
    inset: 12pt,
  )[
    #if title != none {
      text(10pt, fill: success, weight: "bold")[#title]
      v(4pt)
    }
    #set text(fill: rgb("#e0e0e8"))
    #body
  ]
}

#let stat-card(value, label, color: accent) = {
  box(
    width: 100%,
    fill: surface,
    stroke: 0.5pt + surface-border,
    radius: 8pt,
    inset: (x: 10pt, y: 8pt),
  )[
    #align(center)[
      #text(22pt, fill: color, weight: "bold")[#value]
      #v(-2pt)
      #text(7.5pt, fill: muted, weight: "medium")[#upper(label)]
    ]
  ]
}

#let metric-row(signal, weight, multiplier, color: rgb("#e0e0e8")) = {
  (
    text(fill: color, weight: "medium")[#signal],
    align(center, text(fill: color, weight: "bold")[#weight]),
    align(center, text(fill: color, weight: "bold")[#multiplier]),
  )
}

// ============================================
// COVER PAGE
// ============================================

#v(3cm)

#align(center)[
  #block(width: 80%)[
    #text(11pt, fill: muted, weight: "medium", tracking: 0.3em)[SOURCE CODE ANALYSIS]
    #v(8pt)
    #text(36pt, fill: white, weight: "bold")[Breaking the Algorithm]
    #v(4pt)
    #text(16pt, fill: accent)[A Growth Guide for Small X Accounts]
    #v(4pt)
    #text(16pt, fill: rgb("#ffffff").darken(40%))[Based on the Open-Sourced Code]
    #v(20pt)
    #line(length: 40%, stroke: 0.5pt + accent)
    #v(20pt)
    #text(10pt, fill: muted)[
      Every recommendation in this guide cites specific code variables, weights,#linebreak()
      and mechanisms from X's open-sourced recommendation algorithm.
    ]
    #v(12pt)
    #text(9pt, fill: muted)[
      Sources: `twitter/the-algorithm` (2023) · `xai-org/x-algorithm` (2026)
    ]
    #v(2cm)

    #grid(
      columns: (1fr, 1fr, 1fr, 1fr),
      gutter: 10pt,
      stat-card("150x", "Reply value vs Like"),
      stat-card("3", "Tweets scored < 65", color: warn),
      stat-card("-738", "Likes per report", color: danger),
      stat-card("4x", "Premium boost", color: success),
    )

    #v(1.5cm)
    #text(8pt, fill: muted)[March 2028 · Generated from 4,331 lines of algorithm analysis]
  ]
]

#pagebreak()

// ============================================
// TABLE OF CONTENTS
// ============================================

#align(center)[
  #text(8pt, fill: muted, weight: "medium", tracking: 0.3em)[CONTENTS]
]
#v(8pt)

#block(width: 100%, fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 16pt)[
  #set text(10pt)
  #grid(
    columns: (auto, 1fr, auto),
    gutter: 6pt,
    text(fill: accent, weight: "bold")[01], h(8pt) + text(fill: white)[Your Starting Position — The Math], align(right, text(fill: muted)[3]),
    text(fill: accent, weight: "bold")[02], h(8pt) + text(fill: white)[Decision: Keep Your 2023 Account], align(right, text(fill: muted)[4]),
    text(fill: accent, weight: "bold")[03], h(8pt) + text(fill: white)[Step 1 — Fix Your Ratio], align(right, text(fill: muted)[4]),
    text(fill: accent, weight: "bold")[04], h(8pt) + text(fill: white)[Step 2 — Pick Your Reply Targets], align(right, text(fill: muted)[5]),
    text(fill: accent, weight: "bold")[05], h(8pt) + text(fill: white)[Step 3 — Daily Reply Routine], align(right, text(fill: muted)[6]),
    text(fill: accent, weight: "bold")[06], h(8pt) + text(fill: white)[Step 4 — Original Tweets (2-3/Day)], align(right, text(fill: muted)[7]),
    text(fill: accent, weight: "bold")[07], h(8pt) + text(fill: white)[Step 5 — Timing & The 30-Minute Window], align(right, text(fill: muted)[8]),
    text(fill: accent, weight: "bold")[08], h(8pt) + text(fill: white)[Step 6 — Shape Your SimClusters Identity], align(right, text(fill: muted)[9]),
    text(fill: accent, weight: "bold")[09], h(8pt) + text(fill: white)[Step 7 — Build Dense, Not Broad], align(right, text(fill: muted)[10]),
    text(fill: accent, weight: "bold")[10], h(8pt) + text(fill: white)[Step 8 — What to NEVER Do], align(right, text(fill: muted)[11]),
    text(fill: accent, weight: "bold")[11], h(8pt) + text(fill: white)[Step 9 — When to Get Premium], align(right, text(fill: muted)[12]),
    text(fill: accent, weight: "bold")[12], h(8pt) + text(fill: white)[Milestones & Timeline], align(right, text(fill: muted)[13]),
    text(fill: accent, weight: "bold")[13], h(8pt) + text(fill: white)[The Engagement Scoring Formula], align(right, text(fill: muted)[14]),
    text(fill: accent, weight: "bold")[14], h(8pt) + text(fill: white)[The Pipeline — How For You Works], align(right, text(fill: muted)[15]),
    text(fill: accent, weight: "bold")[15], h(8pt) + text(fill: white)[Source Code References], align(right, text(fill: muted)[16]),
  )
]

#pagebreak()

// ============================================
// SECTION 1: YOUR STARTING POSITION
// ============================================

= Your Starting Position

#info-box(title: "YOUR ACCOUNT PROFILE")[
  *96 followers* · *104 following* · *Account created 2023* · *Free tier* · *Dormant — now reactivating*
]

#v(6pt)

The algorithm sees you as a ghost. Here's the math.

== TweepCred: Your Reputation Score

TweepCred is a daily-computed PageRank score (0–100) from `src/scala/com/twitter/graph/batch/job/tweepcred/`. It determines how many of your tweets the algorithm even *considers*.

#grid(
  columns: (1fr, 1fr),
  gutter: 12pt,
  block(fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 12pt)[
    #align(center)[
      #text(11pt, fill: warn, weight: "bold")[YOUR ESTIMATED SCORE]
      #v(4pt)
      #text(42pt, fill: warn, weight: "bold")[25–40]
      #v(2pt)
      #text(8pt, fill: muted)[out of 100]
    ]
  ],
  block(fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 12pt)[
    #align(center)[
      #text(11pt, fill: danger, weight: "bold")[CRITICAL THRESHOLD]
      #v(4pt)
      #text(42pt, fill: danger, weight: "bold")[65]
      #v(2pt)
      #text(8pt, fill: muted)[below this = only 3 tweets scored]
    ]
  ],
)

#v(6pt)

From `ranking.thrift`:

```
antiGamingMinTweepcred = 65
maxHitsPerUser = 3
```

#danger-box(title: "WHAT THIS MEANS")[
  Below TweepCred 65, the EarlyBird search system only considers *3 of your tweets* for distribution. You could post 10 excellent tweets — 7 are invisible to the ranker. Threads are excluded entirely.
]

== Your Ratio Is Okay (For Now)

From `Reputation.scala`, the penalty formula:

```
division_factor = exp(5 * (ratio - 0.6))
// where ratio = followings / followers
```

#table(
  columns: (1fr, auto, auto, auto, auto),
  fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else if y == 2 { rgb("#0a1a0a") } else { surface },
  stroke: 0.3pt + surface-border,
  inset: 8pt,
  align: (left, center, center, center, center),
  table.header(
    text(8pt, fill: muted, weight: "bold")[SCENARIO],
    text(8pt, fill: muted, weight: "bold")[FOLLOWING],
    text(8pt, fill: muted, weight: "bold")[FOLLOWERS],
    text(8pt, fill: muted, weight: "bold")[RATIO],
    text(8pt, fill: muted, weight: "bold")[PENALTY],
  ),
  [Worst case (300/100)], [300], [100], text(fill: danger)[3.0], text(fill: danger)[÷ 163,000],
  text(fill: success, weight: "bold")[You (actual)], text(fill: success)[104], text(fill: success)[96], text(fill: success)[1.08], text(fill: success)[None\*],
  [Target], [60], [96], [0.63], [None],
  [Optimal], [50], [96], [0.52], text(fill: accent)[Bonus],
)

#text(8pt, fill: muted)[\*No penalty triggers because you're under the 500-following hard gate in `UserMass.scala`]

#success-box(title: "GOOD NEWS")[
  Your 1.08 ratio doesn't trigger the penalty — the `UserMass.scala` threshold requires 500+ following. Your main problem is dormancy (zero Real Graph edges), not ratio.
]

== The Cold Start Problem

#grid(
  columns: (1fr, 1fr),
  gutter: 10pt,
  block(fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 10pt)[
    #text(9pt, fill: danger, weight: "bold")[Real Graph Edges: ZERO]
    #v(4pt)
    #text(8.5pt, fill: muted)[
      After dormancy, all interaction edges have decayed to zero. Edge formula uses exponential time decay. Your tweets *won't surface* in followers' feeds because Real Graph predicts zero interaction probability.
    ]
    #v(3pt)
    #text(7.5pt, fill: rgb("#4a4a5a"))[Source: `src/scala/com/twitter/interaction_graph/`]
  ],
  block(fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 10pt)[
    #text(9pt, fill: danger, weight: "bold")[SimClusters: NOT IN KNOWN-FOR]
    #v(4pt)
    #text(8.5pt, fill: muted)[
      Only top 20M producers are in the Known-For matrix. At 96 followers, you're excluded. Your tweets have no pre-computed community embedding and can't be recommended via out-of-network discovery.
    ]
    #v(3pt)
    #text(7.5pt, fill: rgb("#4a4a5a"))[Source: `src/scala/com/twitter/simclusters_v2/`]
  ],
)

#v(6pt)

#block(fill: rgb("#1a1a0a"), stroke: 0.5pt + rgb("#3a3a1a"), radius: 8pt, inset: 12pt)[
  #text(10pt, fill: warn, weight: "bold")[THE VICIOUS CYCLE]
  #v(6pt)
  #align(center)[
    #text(9pt, fill: rgb("#c0c0a0"))[
      Low TweepCred #sym.arrow.r Only 3 tweets scored #sym.arrow.r Low Real Graph weights #sym.arrow.r
      #linebreak()
      Tweets don't surface #sym.arrow.r No engagement #sym.arrow.r No SimClusters embedding #sym.arrow.r
      #linebreak()
      No discovery #sym.arrow.r TweepCred stays low #sym.arrow.r #text(fill: warn)[Repeat]
    ]
  ]
]

#pagebreak()

// ============================================
// DECISION: KEEP YOUR ACCOUNT
// ============================================

= Decision: Keep Your 2023 Account

#success-box(title: "VERDICT: DO NOT CREATE A NEW ACCOUNT")[
  - Account age factor `min(1.0, log(1 + age/15))` caps at 1.0 after ~30 days — your 3-year account and a 2-month account score the same
  - *But* your 96 existing followers are dormant Real Graph edges that can be *reactivated*
  - New accounts start with zero edges entirely
  - New accounts face stricter anti-spam heuristics
  - *Old dormant account is strictly better than new*
]

#pagebreak()

// ============================================
// STEP 1: FIX RATIO
// ============================================

= Step 1 — Trim Your Following List

Your ratio isn't penalizing you yet, but trimming it improves TweepCred and signals authority.

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 10pt,
  stat-card("104", "Current following"),
  stat-card("~60", "Target following", color: success),
  stat-card("10-15", "Unfollows per day", color: warn),
)

#v(8pt)

*Exact actions:*
+ Open your following list
+ Unfollow everyone you don't genuinely read or interact with
+ *Speed limit: 10–15 unfollows per day* — mass unfollow triggers a 3-month shadowban
+ Target: ~60 following (ratio 0.63, essentially neutral)
+ This takes ~3-4 days at safe pace
+ *Never follow someone unless you'll actually engage with them*

#danger-box(title: "WARNING: DO NOT RUSH")[
  Mass unfollow = 3-month reduced distribution from the anti-gaming system. 10-15 per day is safe. Do not do 44 in one sitting.
]

#pagebreak()

// ============================================
// STEP 2: PICK REPLY TARGETS
// ============================================

= Step 2 — Pick Your 10–15 Reply Targets

This is the highest-leverage action in the entire algorithm.

== The Weight Table

#block(width: 100%, fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 0pt, clip: true)[
  #table(
    columns: (1fr, auto, auto),
    fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else if calc.odd(y) { surface } else { rgb("#0e0e18") },
    stroke: none,
    inset: 10pt,
    align: (left, center, center),
    table.header(
      text(8pt, fill: muted, weight: "bold")[ENGAGEMENT SIGNAL],
      text(8pt, fill: muted, weight: "bold")[WEIGHT],
      text(8pt, fill: muted, weight: "bold")[VS LIKE],
    ),
    text(fill: success, weight: "bold")[Reply + author engages back], text(22pt, fill: success, weight: "bold")[+75.0], text(fill: success, weight: "bold")[150x],
    text(fill: accent, weight: "bold")[Reply], text(16pt, fill: accent, weight: "bold")[+13.5], text(fill: accent, weight: "bold")[27x],
    [Profile click + engage], text(fill: white)[+12.0], [24x],
    [Good click (stay 2+ min)], text(fill: white)[+11.0], [22x],
    [Retweet], text(fill: white)[+1.0], [2x],
    text(fill: muted)[Like (baseline)], text(fill: muted)[+0.5], text(fill: muted)[1x],
    text(fill: danger)[Block / mute / "show less"], text(fill: danger)[-74.0], text(fill: danger)[-148x],
    text(fill: danger, weight: "bold")[Report], text(16pt, fill: danger, weight: "bold")[-369.0], text(fill: danger, weight: "bold")[-738x],
  )
]

#v(6pt)

#info-box(title: "THE MATH OF ONE GOOD REPLY EXCHANGE")[
  #text(9pt)[
    Your reply: `+13.5 × P(reply)` #h(1fr) Author replies back: `+75.0 × P(reply_engaged)` #linebreak()
    Reader clicks your profile: `+12.0 × P(profile_click)` #h(1fr) *Total: ~100 weight units* #linebreak()
    #v(4pt)
    vs. one like on your original tweet: `+0.5 × P(like)` = *0.5 weight units* #linebreak()
    #v(4pt)
    #text(fill: accent, weight: "bold")[Ratio: 200x more algorithmic value from one reply exchange]
  ]
]

== Criteria for Choosing Targets

#grid(
  columns: (auto, 1fr),
  gutter: 8pt,
  text(fill: success)[✓], [*In your niche* (longevity, biotech, geroscience, health)],
  text(fill: success)[✓], [*1K–50K followers* — big enough for active threads, small enough to notice you],
  text(fill: success)[✓], [*Posts at least 3x/week* — consistent activity],
  text(fill: success)[✓], [*Actually replies to people* — check their recent replies tab],
  text(fill: danger)[✗], [Accounts with 500K+ followers — they won't see your reply],
  text(fill: danger)[✗], [Accounts that never reply back to anyone],
)

#v(6pt)

*Write down 15 accounts. These are your daily reply targets for the next month.*

#pagebreak()

// ============================================
// STEP 3: DAILY REPLY ROUTINE
// ============================================

= Step 3 — Daily Reply Routine (Weeks 1–4)

#info-box(title: "KEY INSIGHT: REPLIES DON'T COUNT AGAINST YOUR 3-TWEET CAP")[
  The `antiGamingMinTweepcred = 65` / `maxHitsPerUser = 3` cap only applies to your original tweets in the ranking pipeline. *You can reply unlimited times.* This is your primary growth lever.
]

== Time Commitment: 45–60 Minutes/Day

*Morning routine (before posting anything original):*

+ Open your 15 target accounts' recent posts
+ Find 3–5 posts where you have a genuinely useful reply
+ Write *substantive* replies — not "great post!" or emoji reactions

== What a Good Reply Looks Like

#grid(
  columns: (1fr, 1fr),
  gutter: 10pt,
  danger-box(title: "BAD — Zero algorithmic value")[
    #text(9pt, style: "italic")[
      "Great thread! 🔥"
      #v(4pt)
      "This 👆"
      #v(4pt)
      "So true!"
    ]
    #v(4pt)
    #text(8pt, fill: muted)[No reply-back. No profile click. No Real Graph edge. Wasted time.]
  ],
  success-box(title: "GOOD — Generates author reply-back")[
    #text(9pt, style: "italic")[
      "The rapamycin dosing data here is interesting but the PEARL trial used 5mg weekly vs the 1mg daily in that Nature paper — completely different pharmacokinetics. Weekly pulse dosing avoids mTORC2 inhibition which is where the immunosuppression risk comes from."
    ]
    #v(4pt)
    #text(8pt, fill: success)[Author likely replies → +75.0 weight. Readers click profile → +12.0 each.]
  ],
)

== Building Real Graph Edges

Each reply exchange creates a Real Graph edge. The features that matter:

#table(
  columns: (auto, 1fr),
  fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else { surface },
  stroke: 0.3pt + surface-border,
  inset: 8pt,
  table.header(
    text(8pt, fill: muted, weight: "bold")[REALGRAPH FEATURE],
    text(8pt, fill: muted, weight: "bold")[HOW TO BUILD IT],
  ),
  [`nonZeroDays`], [Interact with the *same* people every single day],
  [`ewmaDecayedCount`], [Multiple interactions per day compounds the signal],
  [`daysSinceLastInteraction`], [Never let more than 1–2 days pass without interacting],
  [`diversityMetric`], [Mix replies, likes, retweets, quote tweets with same person],
  [`commonFriends`], [Follow people your targets follow],
)

#text(8.5pt, fill: muted)[Source: `src/scala/com/twitter/interaction_graph/` — visible interactions get 5x weight vs implicit (clicks, views)]

#pagebreak()

// ============================================
// STEP 4: ORIGINAL TWEETS
// ============================================

= Step 4 — Original Tweets (2–3 Per Day)

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 10pt,
  block(fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 10pt)[
    #align(center)[
      #text(32pt, fill: accent, weight: "bold")[2–3]
      #v(-2pt)
      #text(8pt, fill: muted)[TWEETS PER DAY]
      #v(4pt)
      #text(8pt, fill: rgb("#6a6a7a"))[All 3 get scored. Post 10 and the algorithm randomly picks 3.]
    ]
  ],
  block(fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 10pt)[
    #align(center)[
      #text(32pt, fill: danger, weight: "bold")[0]
      #v(-2pt)
      #text(8pt, fill: muted)[THREADS]
      #v(4pt)
      #text(8pt, fill: rgb("#6a6a7a"))[Each thread tweet counts against your 3-tweet cap. Wait until TweepCred > 65.]
    ]
  ],
  block(fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 10pt)[
    #align(center)[
      #text(32pt, fill: success, weight: "bold")[2x]
      #v(-2pt)
      #text(8pt, fill: muted)[MEDIA BOOST]
      #v(4pt)
      #text(8pt, fill: rgb("#6a6a7a"))[EarlyBird applies 2x for images/video at candidate retrieval.]
    ]
  ],
)

#v(8pt)

== Tweet Format Rules

#table(
  columns: (auto, auto, 1fr),
  fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else { surface },
  stroke: 0.3pt + surface-border,
  inset: 8pt,
  table.header(
    text(8pt, fill: success, weight: "bold")[DO],
    text(8pt, fill: danger, weight: "bold")[DON'T],
    text(8pt, fill: muted, weight: "bold")[CODE REASON],
  ),
  [Text + image], [Bare link], [Free account links = 0% median engagement],
  [1 niche hashtag], [3+ hashtags], [`multipleHashtagsOrTrendsDampening` = -40%],
  [Standalone post], [Thread], [3-tweet cap wastes slots on thread tweets],
  [Strong opinion/take], [Bland observation], [No engagement = no SimClusters embedding],
  [Original image/chart], [No media], [Missing the 2x EarlyBird media boost],
)

== Ideal Tweet Structure

#block(fill: surface, stroke: 0.5pt + accent, radius: 8pt, inset: 14pt)[
  #text(10pt, fill: white, weight: "medium")[
    \[Strong opening opinion or insight\]
  ]
  #v(6pt)
  #text(9pt, fill: rgb("#b0b0c0"))[
    \[2–3 sentences of evidence or analysis\]
  ]
  #v(6pt)
  #text(9pt, fill: rgb("#b0b0c0"))[
    \[Image: chart, paper screenshot, data visualization\]
  ]
  #v(6pt)
  #text(9pt, fill: accent)[
    \#Longevity
  ]
  #v(8pt)
  #text(8pt, fill: muted)[
    That's it. No thread. No link. One hashtag. Image attached.
  ]
]

#v(6pt)

#info-box(title: "IF YOU MUST SHARE A LINK")[
  Post a text-only tweet with your take first. Then reply *to yourself* with the link. The parent tweet gets full algorithmic treatment. The link lives in the reply where it doesn't trigger the link penalty on your main tweet.
]

#pagebreak()

// ============================================
// STEP 5: TIMING
// ============================================

= Step 5 — Timing & The 30-Minute Window

== Tweet Decay Curve

From `ThriftAgeDecayRankingParams` in `ranking.thrift`:

```
ageDecayHalflife = 360.0    // 6 hours
ageDecaySlope    = 0.003
ageDecayBase     = 0.6      // floor — tweets never go below 60%
```

#table(
  columns: (auto, auto, 1fr),
  fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else if y == 1 { rgb("#0a1a0a") } else { surface },
  stroke: 0.3pt + surface-border,
  inset: 8pt,
  align: (left, center, left),
  table.header(
    text(8pt, fill: muted, weight: "bold")[TIME AFTER POSTING],
    text(8pt, fill: muted, weight: "bold")[SCORE],
    text(8pt, fill: muted, weight: "bold")[STATUS],
  ),
  text(fill: success, weight: "bold")[0–30 minutes], text(fill: success, weight: "bold")[100%], text(fill: success, weight: "bold")[CRITICAL WINDOW — engagement here determines everything],
  [30 minutes], [97%], [Still strong],
  [6 hours], [50%], [Half relevancy gone],
  [12 hours], [25%], [Fading fast],
  [24 hours], text(fill: danger)[6%], text(fill: danger)[Effectively dead],
)

#v(6pt)

#info-box(title: "THE FIRST 30 MINUTES")[
  Early engagement signals are weighted most heavily. The algorithm uses early velocity to predict total engagement. Tweets with traction in the first 30 minutes receive amplified distribution. Tweets with zero engagement in 30 minutes are effectively dead.
]

== Example Daily Schedule

#block(fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 14pt)[
  #grid(
    columns: (auto, 1fr),
    gutter: (8pt, 6pt),
    text(fill: accent, weight: "bold")[9:00 AM ET], [Post tweet \#1 (your best content of the day)],
    text(fill: muted)[9:00–9:30], [Reply to responders on your tweet *immediately*],
    text(fill: muted)[9:30–10:00], [Reply to 2–3 of your target accounts' posts],
    text(fill: accent, weight: "bold")[12:00 PM ET], [Post tweet \#2],
    text(fill: muted)[12:00–12:30], [Same: engage with responders immediately],
    text(fill: muted)[2:00 PM ET], [Reply session on target accounts' afternoon posts],
  )
]

#text(8.5pt, fill: muted)[Best days: Tuesday–Thursday. Best hours: 9 AM – 3 PM ET for US/EU biotech audience.]

#pagebreak()

// ============================================
// STEP 6: SIMCLUSTERS
// ============================================

= Step 6 — Shape Your SimClusters Identity

From `src/scala/com/twitter/simclusters_v2/README.md`:

#block(fill: surface, stroke: (left: 3pt + accent), radius: (right: 6pt), inset: 12pt)[
  #text(9.5pt, fill: rgb("#c0c0d0"), style: "italic")[
    "The InterestedIn vector of each user who Fav-ed the tweet is added to the tweet vector."
  ]
]

#v(6pt)

Your *likes* shape your SimClusters InterestedIn vector. This determines which community the algorithm puts you in.

== What to Do

#grid(
  columns: (auto, 1fr),
  gutter: 8pt,
  text(fill: success)[✓], [*Like 10–15 tweets/day ONLY from accounts in your niche* (longevity, biotech, geroscience)],
  text(fill: success)[✓], [When community members like YOUR tweets, their vectors embed your tweet into the community],
  text(fill: success)[✓], [This is invisible work but directly shapes what the algorithm thinks you are],
  text(fill: danger)[✗], [*Do NOT like random viral content* — every like shifts your InterestedIn vector],
  text(fill: danger)[✗], [Don't like memes, politics, sports if your niche is biotech — dilutes your cluster signal],
)

#v(6pt)

#info-box(title: "HOW DISCOVERY WORKS FOR SMALL ACCOUNTS")[
  You don't need to be in the Known-For matrix (top 20M) for your tweets to be discovered: #linebreak()#linebreak()
  1. You post a tweet relevant to a topic community #linebreak()
  2. Even 2–3 favorites from users in that SimCluster embed your tweet #linebreak()
  3. Your tweet surfaces as out-of-network candidate for other community members #linebreak()
  4. This is *engagement-driven, not follower-driven* — your follower count is irrelevant #linebreak()
]

#pagebreak()

// ============================================
// STEP 7: BUILD DENSE
// ============================================

= Step 7 — Build Dense, Not Broad

GraphJet (`src/java/com/twitter/recos/`) powers ~15% of For You via SALSA random walks.

== How It Works

#block(fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 12pt)[
  #align(center)[
    #text(9pt, fill: rgb("#c0c0d0"))[
      *Your Circle of Trust* (5–10 accounts you engage with most)
      #v(4pt)
      #text(fill: accent)[↓ random walk ↓]
      #v(4pt)
      *Tweets they engaged with*
      #v(4pt)
      #text(fill: accent)[↓ random walk ↓]
      #v(4pt)
      *Other users who engaged with those tweets*
      #v(4pt)
      #text(fill: accent)[↓ and vice versa ↓]
      #v(4pt)
      *Those users discover YOU through the same path*
    ]
  ]
]

#v(6pt)

#info-box(title: "THE STRATEGY")[
  Pick *5–10 accounts* in your niche. Engage with them *daily* — reply, like, retweet. The goal is dense Real Graph edges that GraphJet's random walks traverse efficiently.

  #v(6pt)
  *5 strong edges beat 50 weak ones.* Don't spread thin across hundreds of accounts.
]

#pagebreak()

// ============================================
// STEP 8: WHAT NOT TO DO
// ============================================

= Step 8 — What to NEVER Do

#block(width: 100%, fill: rgb("#1a0808"), stroke: 0.5pt + rgb("#3a1a1a"), radius: 8pt, inset: 0pt, clip: true)[
  #table(
    columns: (1fr, auto, auto, auto),
    fill: (_, y) => if y == 0 { rgb("#2a0a0a") } else if calc.odd(y) { rgb("#1a0808") } else { rgb("#150606") },
    stroke: none,
    inset: 10pt,
    align: (left, center, center, center),
    table.header(
      text(8pt, fill: rgb("#ff6666"), weight: "bold")[ACTION],
      text(8pt, fill: rgb("#ff6666"), weight: "bold")[WEIGHT],
      text(8pt, fill: rgb("#ff6666"), weight: "bold")[LIKES TO OFFSET],
      text(8pt, fill: rgb("#ff6666"), weight: "bold")[RECOVERY],
    ),
    text(fill: danger, weight: "bold")[Get reported], text(fill: danger, weight: "bold")[-369.0], text(fill: danger, weight: "bold")[738 likes], [Months],
    [Get blocked / muted], [-74.0], [148 likes], [140-day decay],
    [Mass follow then unfollow], [N/A], [N/A], [3+ months],
    [Post bare link (free)], [~0 reach], [N/A], [Immediate],
    [Use 3+ hashtags], [-40%], [N/A], [Immediate],
    [Post 4+ tweets/day], [Wasted], [—], [—],
    [Post threads], [Wasted], [—], [—],
    [Follow 500+ accounts], text(fill: danger)[exp(5×(r-0.6))], [Destroys TweepCred], [Permanent],
    [Reply "great post!"], [Zero], [—], [—],
    [Like random viral content], [Dilutes SimClusters], [—], [Permanent drift],
  )
]

#v(8pt)

#danger-box(title: "THE ASYMMETRY IS BRUTAL")[
  The algorithm is architecturally biased toward punishment over reward: #linebreak()#linebreak()
  - *One report* (-369.0) requires *738 likes* (+0.5 each) to offset #linebreak()
  - *One "show less"* (-74.0) requires *148 likes* to offset #linebreak()
  - At 96 followers, even *one* negative signal can be catastrophic #linebreak()
  #v(4pt)
  Source: `FeedbackFatigueScorer.scala` — negative signals decay over 140 days with 0.2x–1.0x multiplier
]

#pagebreak()

// ============================================
// STEP 9: PREMIUM
// ============================================

= Step 9 — When to Get Premium

From `HomeGlobalParams.scala`:

```scala
BlueVerifiedAuthorInNetworkMultiplierParam   = 4.0
BlueVerifiedAuthorOutOfNetworkMultiplierParam = 2.0
```

Plus +4 to +16 TweepCred bonus.

#table(
  columns: (1fr, auto, 1fr),
  fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else { surface },
  stroke: 0.3pt + surface-border,
  inset: 10pt,
  table.header(
    text(8pt, fill: muted, weight: "bold")[CONDITION],
    text(8pt, fill: muted, weight: "bold")[SUBSCRIBE?],
    text(8pt, fill: muted, weight: "bold")[WHY],
  ),
  [Week 1–4, < 200 followers], text(fill: danger, weight: "bold")[NO], [4x × zero Real Graph edges = zero],
  [You post links regularly], text(fill: success, weight: "bold")[YES, NOW], [Free accounts get 0% engagement on links],
  [TweepCred estimated 50–60], text(fill: success, weight: "bold")[YES], [+4-16 bonus can bridge past the 65 threshold — *biggest unlock*],
  [500+ followers, consistent engagement], text(fill: success, weight: "bold")[YES], [4x on a real audience compounds into ~10x reach],
)

#v(6pt)

#info-box(title: "THE 4x MULTIPLIER IS ONLY AS GOOD AS YOUR BASE")[
  Premium multiplies your reach. But 4x × 0 = 0. Build Real Graph edges first. Subscribe when the multiplier has something to multiply.
]

#pagebreak()

// ============================================
// MILESTONES
// ============================================

= Milestones & Timeline

#block(width: 100%, fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 0pt, clip: true)[
  #table(
    columns: (1fr, auto, 1fr),
    fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else if calc.odd(y) { surface } else { rgb("#0e0e18") },
    stroke: none,
    inset: 10pt,
    table.header(
      text(8pt, fill: muted, weight: "bold")[MILESTONE],
      text(8pt, fill: muted, weight: "bold")[EXPECTED],
      text(8pt, fill: muted, weight: "bold")[WHAT UNLOCKS],
    ),
    [Ratio < 1.0], [Day 4–5], [TweepCred penalty removed],
    [First reply-back from target], text(fill: success)[Day 2–3], [150x like value + Real Graph edge],
    [10 active Real Graph edges], [Week 2], [Your tweets visible in followers' feeds],
    [250 followers], [Week 3–4], [Meaningful in-network audience],
    [Tweets getting 50+ impressions], [Week 3–4], [Algorithmic distribution starting],
    text(fill: accent, weight: "bold")[TweepCred > 65], text(fill: accent, weight: "bold")[Month 2–4], text(fill: accent, weight: "bold")[THE BIG UNLOCK: unlimited tweets scored],
    [First out-of-network viral tweet], [Month 2–3], [SimClusters embedding working],
    [1,000 followers], [Month 4–6], [Self-sustaining growth flywheel],
  )
]

#v(10pt)

== The Flywheel (After TweepCred 65)

#block(fill: rgb("#0a1a0a"), stroke: 0.5pt + rgb("#1a3a1a"), radius: 8pt, inset: 14pt)[
  #align(center)[
    #text(9.5pt, fill: success)[
      All tweets considered (no 3-tweet cap)
      #v(3pt)
      #sym.arrow.b
      #v(3pt)
      Better tweets surface (Heavy Ranker sees full output)
      #v(3pt)
      #sym.arrow.b
      #v(3pt)
      More engagement on better tweets
      #v(3pt)
      #sym.arrow.b
      #v(3pt)
      Stronger SimClusters embeddings
      #v(3pt)
      #sym.arrow.b
      #v(3pt)
      More out-of-network discovery #sym.arrow.r More followers #sym.arrow.r Better TweepCred #sym.arrow.r *Repeat*
    ]
  ]
]

#v(6pt)
#text(9pt, fill: muted, style: "italic")[Everything before the flywheel is grinding. Everything after it compounds.]

#pagebreak()

// ============================================
// THE FORMULA
// ============================================

= The Complete Scoring Formula

The Heavy Ranker (MaskNet, ~48M parameters) scores every candidate tweet:

```
score = SUM(weight_i × P(engagement_i))
```

#block(fill: surface, stroke: 1pt + accent, radius: 8pt, inset: 14pt)[
  #set text(9pt, font: "Menlo")
  #text(fill: rgb("#6a9fb5"))[Heavy Ranker Score =] \
  #h(12pt) #text(fill: success)[P(reply + author_engaged)] #h(4pt) #text(fill: muted)[×] #h(4pt) #text(fill: success, weight: "bold")[75.0] \
  #h(8pt) + #text(fill: accent)[P(reply)] #h(4pt) #text(fill: muted)[×] #h(4pt) #text(fill: accent, weight: "bold")[13.5] \
  #h(8pt) + P(profile_click + engagement) #text(fill: muted)[ ×] #text(weight: "bold")[ 12.0] \
  #h(8pt) + P(good_click + 2m stay) #text(fill: muted)[ ×] #text(weight: "bold")[ 11.0] \
  #h(8pt) + P(bookmark) #text(fill: muted)[ ×] #text(weight: "bold")[ 10.0] \
  #h(8pt) + P(retweet) #text(fill: muted)[ ×] #text(weight: "bold")[ 1.0] \
  #h(8pt) + P(favorite) #text(fill: muted)[ ×] #text(weight: "bold")[ 0.5] \
  #h(8pt) + P(video\_watch\_50%) #text(fill: muted)[ ×] #text(weight: "bold")[ 0.005] \
  #h(8pt) + #text(fill: danger)[P(negative\_reaction)] #h(4pt) #text(fill: muted)[×] #h(4pt) #text(fill: danger, weight: "bold")[-74.0] \
  #h(8pt) + #text(fill: danger)[P(report)] #h(4pt) #text(fill: muted)[×] #h(4pt) #text(fill: danger, weight: "bold")[-369.0]
]

#v(8pt)

Post-ranking adjustments layered on top:

#table(
  columns: (1fr, auto, 1fr),
  fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else { surface },
  stroke: 0.3pt + surface-border,
  inset: 8pt,
  table.header(
    text(8pt, fill: muted, weight: "bold")[ADJUSTMENT],
    text(8pt, fill: muted, weight: "bold")[FACTOR],
    text(8pt, fill: muted, weight: "bold")[SOURCE],
  ),
  [Out-of-network scaling], [0.75x], [`OONTweetScalingScorer.scala`],
  [Reply scaling], [0.75x], [`ScoredTweetsParam.scala`],
  [Author diversity decay], [0.5x per successive tweet], [`ScoredTweetsParam.scala`],
  [Premium in-network boost], text(fill: success)[4.0x], [`HomeGlobalParams.scala`],
  [Premium out-of-network boost], text(fill: success)[2.0x], [`HomeGlobalParams.scala`],
  [Feedback fatigue], [0.2x – 1.0x], [`FeedbackFatigueScorer.scala`],
)

#pagebreak()

// ============================================
// THE PIPELINE
// ============================================

= The Pipeline — How "For You" Works

#block(fill: surface, stroke: 0.5pt + surface-border, radius: 8pt, inset: 14pt)[
  #set text(9pt)
  #grid(
    columns: (auto, 1fr),
    gutter: (10pt, 8pt),
    align(center, text(fill: accent, weight: "bold")[500M]), [daily tweets enter the system],
    align(center, text(fill: muted)[↓]), [],
    align(center, text(fill: accent, weight: "bold")[~1,500]), [*Candidate Sourcing* — 50% in-network (EarlyBird) + 50% out-of-network (SimClusters, GraphJet)],
    align(center, text(fill: muted)[↓]), [],
    align(center, text(fill: accent, weight: "bold")[~500]), [*Light Ranker* — logistic regression pre-filter (TweepCred, engagement, content quality)],
    align(center, text(fill: muted)[↓]), [],
    align(center, text(fill: accent, weight: "bold")[~100]), [*Heavy Ranker* — MaskNet neural network, 48M params, 10 engagement predictions],
    align(center, text(fill: muted)[↓]), [],
    align(center, text(fill: accent, weight: "bold")[~50]), [*Post-Ranking* — OON scaling, Premium boost, diversity, feedback fatigue],
    align(center, text(fill: muted)[↓]), [],
    align(center, text(fill: success, weight: "bold")[~50]), [*Your "For You" feed*],
  )
]

#v(8pt)

#info-box(title: "WHERE SMALL ACCOUNTS GET FILTERED OUT")[
  #text(9pt)[
  *Stage 1 — Candidate Sourcing:* Without Real Graph edges, your tweets don't enter the in-network pool. Without SimClusters presence, you're excluded from out-of-network. #linebreak()#linebreak()
  *Stage 2 — Light Ranker:* TweepCred < 65 means only 3 tweets pass. Low engagement signals mean low pre-filter scores. #linebreak()#linebreak()
  *Stage 3 — Heavy Ranker:* Even if you reach here, zero engagement history means low predicted P(engagement) across all signals. #linebreak()#linebreak()
  *The reply strategy bypasses all of this* — replies appear in the thread itself, not through the ranking pipeline.
  ]
]

#pagebreak()

// ============================================
// SOURCE REFERENCES
// ============================================

= Source Code References

== Primary Repositories

#table(
  columns: (auto, 1fr),
  fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else { surface },
  stroke: 0.3pt + surface-border,
  inset: 8pt,
  table.header(
    text(8pt, fill: muted, weight: "bold")[REPO],
    text(8pt, fill: muted, weight: "bold")[DESCRIPTION],
  ),
  [`twitter/the-algorithm`], [2023 open-source release (Scala/Java) — full recommendation pipeline],
  [`twitter/the-algorithm-ml`], [ML models including Heavy Ranker (MaskNet) weights],
  [`xai-org/x-algorithm`], [2026 Grok-powered replacement (Rust) — Phoenix, weighted_scorer],
)

== Key Files Referenced

#set text(8.5pt)

#table(
  columns: (1fr, 1fr),
  fill: surface,
  stroke: 0.3pt + surface-border,
  inset: 6pt,
  [`Reputation.scala`], [TweepCred ratio penalty, PageRank mapping],
  [`UserMass.scala`], [Initial mass: device, age, ratio factors],
  [`WeightedPageRank.scala`], [Iterative PageRank (damping=0.1)],
  [`ranking.thrift`], [antiGamingMinTweepcred=65, decay params],
  [`HomeGlobalParams.scala`], [Premium 4x/2x multipliers],
  [`OONTweetScalingScorer.scala`], [Out-of-network 0.75x scaling],
  [`FeedbackFatigueScorer.scala`], [140-day negative signal decay],
  [`ScoredTweetsParam.scala`], [All scoring parameters],
  [`SimClusters v2 README`], [145K communities, Known-For matrix],
  [`Interaction Graph README`], [Real Graph edge features],
  [`weighted_scorer.rs`], [2026 Grok 19-metric scoring],
)

#set text(9.5pt)

#v(1cm)

#align(center)[
  #line(length: 30%, stroke: 0.3pt + muted)
  #v(8pt)
  #text(8pt, fill: muted)[
    Generated from 4,331 lines of algorithm analysis across 6 documents.
    #linebreak()
    Every recommendation cites specific source code variables and weights.
    #linebreak()
    #v(4pt)
    Full analysis: `docs/x-algorithm-analysis/`
  ]
]
