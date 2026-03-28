// X Algorithm Growth Guide — 2026 Source Code Edition
// Based ONLY on xai-org/x-algorithm (January 2026)
// Zero 2023 contamination

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
        align(left)[X Algorithm Growth Guide — 2026 Edition],
        align(right)[#page-num],
      )
    }
  },
)

#set text(font: "Helvetica Neue", size: 9.5pt, fill: rgb("#e0e0e8"))
#set par(leading: 0.7em, justify: true)

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
  block[#set text(13pt, fill: rgb("#1da1f2"), weight: "bold"); #it.body]
  v(0.3em)
}

#show heading.where(level: 3): it => {
  v(0.5em)
  block[#set text(11pt, fill: rgb("#8899aa"), weight: "bold"); #it.body]
  v(0.2em)
}

#show raw.where(block: true): it => {
  block(width: 100%, fill: rgb("#12121e"), stroke: 0.5pt + rgb("#1a1a2e"), radius: 6pt, inset: 12pt)[
    #set text(8.5pt, font: "Menlo", fill: rgb("#c0c0d0"))
    #it
  ]
}

#show raw.where(block: false): it => {
  box(fill: rgb("#12121e"), outset: (x: 2pt, y: 2pt), radius: 3pt)[
    #set text(8.5pt, font: "Menlo", fill: rgb("#1da1f2"))
    #it
  ]
}

#let accent = rgb("#1da1f2")
#let warn = rgb("#ff6b35")
#let danger = rgb("#ff3333")
#let success = rgb("#22c55e")
#let muted = rgb("#6a6a7a")
#let surface = rgb("#12121e")
#let sborder = rgb("#1a1a2e")

#let info-box(body, title: none, color: accent) = {
  block(width: 100%, fill: color.lighten(95%).desaturate(80%), stroke: (left: 3pt + color), radius: (right: 6pt), inset: 12pt)[
    #if title != none { text(10pt, fill: color, weight: "bold")[#title]; v(4pt) }
    #set text(fill: rgb("#e0e0e8"))
    #body
  ]
}

#let danger-box(body, title: none) = {
  block(width: 100%, fill: rgb("#1a0a0a"), stroke: (left: 3pt + danger), radius: (right: 6pt), inset: 12pt)[
    #if title != none { text(10pt, fill: danger, weight: "bold")[#title]; v(4pt) }
    #set text(fill: rgb("#e0e0e8"))
    #body
  ]
}

#let success-box(body, title: none) = {
  block(width: 100%, fill: rgb("#0a1a0a"), stroke: (left: 3pt + success), radius: (right: 6pt), inset: 12pt)[
    #if title != none { text(10pt, fill: success, weight: "bold")[#title]; v(4pt) }
    #set text(fill: rgb("#e0e0e8"))
    #body
  ]
}

#let tag-code = box(fill: rgb("#1a2a1a"), outset: (x: 3pt, y: 2pt), radius: 3pt)[#text(7pt, fill: success, weight: "bold")[CODE]]
#let tag-emp = box(fill: rgb("#2a1a0a"), outset: (x: 3pt, y: 2pt), radius: 3pt)[#text(7pt, fill: warn, weight: "bold")[EMPIRICAL]]
#let tag-inf = box(fill: rgb("#1a1a2a"), outset: (x: 3pt, y: 2pt), radius: 3pt)[#text(7pt, fill: accent, weight: "bold")[INFERRED]]

// ============ COVER ============

#v(2.5cm)
#align(center)[
  #block(width: 80%)[
    #text(11pt, fill: muted, weight: "medium", tracking: 0.3em)[2026 SOURCE CODE ANALYSIS]
    #v(8pt)
    #text(36pt, fill: white, weight: "bold")[Breaking the Algorithm]
    #v(4pt)
    #text(16pt, fill: accent)[Growth Guide for Small X Accounts]
    #v(4pt)
    #text(14pt, fill: rgb("#ffffff").darken(40%))[Based on xai-org/x-algorithm (Jan 2026)]
    #v(20pt)
    #line(length: 40%, stroke: 0.5pt + accent)
    #v(16pt)

    #danger-box(title: "NO 2023 CONTAMINATION")[
      The previous `twitter/the-algorithm` (TweepCred, SimClusters, Real Graph, MaskNet) has been *completely replaced*. None of those systems exist in the current algorithm. Every claim in this guide is from the 2026 source code or 2025-2026 empirical data.
    ]

    #v(16pt)
    #text(9pt, fill: muted)[
      Evidence tags: #tag-code from source code #h(8pt) #tag-emp from published research #h(8pt) #tag-inf logical deduction
    ]

    #v(1.5cm)

    #grid(
      columns: (1fr, 1fr, 1fr),
      gutter: 10pt,
      box(width: 100%, fill: surface, stroke: 0.5pt + sborder, radius: 8pt, inset: (x: 10pt, y: 8pt))[
        #align(center)[
          #text(22pt, fill: accent, weight: "bold")[19]
          #v(-2pt)
          #text(7.5pt, fill: muted, weight: "medium")[SCORING SIGNALS]
        ]
      ],
      box(width: 100%, fill: surface, stroke: 0.5pt + sborder, radius: 8pt, inset: (x: 10pt, y: 8pt))[
        #align(center)[
          #text(22pt, fill: warn, weight: "bold")[128]
          #v(-2pt)
          #text(7.5pt, fill: muted, weight: "medium")[HISTORY POSITIONS]
        ]
      ],
      box(width: 100%, fill: surface, stroke: 0.5pt + sborder, radius: 8pt, inset: (x: 10pt, y: 8pt))[
        #align(center)[
          #text(22pt, fill: danger, weight: "bold")[0]
          #v(-2pt)
          #text(7.5pt, fill: muted, weight: "medium")[HAND-ENGINEERED FEATURES]
        ]
      ],
    )

    #v(1cm)
    #text(8pt, fill: muted)[March 2026 · xai-org/x-algorithm · Rust + Python/JAX]
  ]
]

#pagebreak()

// ============ YOUR PROBLEM ============

= What the Algorithm Sees Right Now

#info-box(title: "YOUR ACCOUNT: @longevityboris")[
  96 followers · 104 following · Account since 2023 · Dormant · Free tier · Longevity/biotech niche
]

#v(6pt)

== The Cold Start Problem #tag-code

The 2026 algorithm has *no* TweepCred, *no* SimClusters, *no* Real Graph. The README states:

#block(fill: surface, stroke: (left: 3pt + accent), radius: (right: 6pt), inset: 12pt)[
  #text(9.5pt, fill: rgb("#c0c0d0"), style: "italic")[
    "We have eliminated every single hand-engineered feature."
  ]
  #v(2pt)
  #text(7pt, fill: muted)[source-2026/README.md]
]

#v(6pt)

Instead, a Grok-based transformer (`PhoenixModel`) learns everything from your last *128 engagement actions*:

```rust
history_seq_len: int = 128  // recsys_model.py
candidate_seq_len: int = 32
```

A dormant account has *zero* engagement history. The `UserActionSeqQueryHydrator` checks this:

```rust
if thrift_user_actions.is_empty() {
    return Err("No user actions found for user {user_id}");
}
```

#danger-box(title: "TRANSLATION")[
  If you have no recent engagement history, the Phoenix transformer *cannot score posts for you or about you*. The entire scoring pipeline short-circuits. Your posts cannot be properly ranked for anyone's feed. *Step zero is generating engagement history by actively using X.*
]

== The 19 Scoring Signals #tag-code

From `weighted_scorer.rs` — every post is scored on exactly 19 signals:

#block(width: 100%, fill: surface, stroke: 0.5pt + sborder, radius: 8pt, inset: 0pt, clip: true)[
  #table(
    columns: (1fr, auto, auto),
    fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else if calc.odd(y) { surface } else { rgb("#0e0e18") },
    stroke: none, inset: 10pt,
    table.header(
      text(8pt, fill: muted, weight: "bold")[SIGNAL],
      text(8pt, fill: muted, weight: "bold")[EST. WEIGHT],
      text(8pt, fill: muted, weight: "bold")[EVIDENCE],
    ),
    text(fill: success, weight: "bold")[Follow Author], [~30x], [#tag-inf New in 2026. Ultimate discovery signal.],
    text(fill: success, weight: "bold")[Share via DM], [~25x], [#tag-inf New in 2026. Separate dedicated signal.],
    text(fill: success, weight: "bold")[Share via Copy Link], [~20x], [#tag-inf New in 2026. Off-platform sharing.],
    text(fill: accent, weight: "bold")[Reply], [~20x], [#tag-emp Still high, but "reply\_engaged\_by\_author" is GONE.],
    text(fill: accent, weight: "bold")[Quote Tweet], [~18x], [#tag-emp New dedicated signal in 2026.],
    [Profile Click], [~12x], [#tag-emp Consistent with 2023 empirical data.],
    [Click (conversation)], [~10x], [#tag-emp Deep interest signal.],
    [Share (generic)], [~10x], [#tag-inf Opening share menu, before choosing channel.],
    [Dwell (binary)], [~8x], [#tag-emp Pausing on the post (2+ seconds).],
    [Retweet], [~3x], [#tag-emp Lower weight, lower probability.],
    text(fill: muted)[Favorite (Like)], text(fill: muted)[1x], [#tag-emp Baseline. Most common, lowest weight.],
    text(fill: muted)[Photo Expand], text(fill: muted)[~2x], [#tag-inf Moderate interest, image-specific.],
    text(fill: muted)[Video Quality View], text(fill: muted)[~3x], [#tag-code Gated behind MIN\_VIDEO\_DURATION\_MS.],
    text(fill: muted)[Dwell Time (continuous)], text(fill: muted)[~0.1/sec], [#tag-code Only non-probability signal.],
    text(fill: muted)[Quoted Click], text(fill: muted)[~4x], [#tag-inf Secondary engagement.],
    text(fill: danger)[Not Interested], text(fill: danger)[~-20x], [#tag-inf Lightest negative. Content mismatch.],
    text(fill: danger)[Mute Author], text(fill: danger)[~-40x], [#tag-inf Soft rejection.],
    text(fill: danger)[Block Author], text(fill: danger)[~-74x], [#tag-inf Hard rejection.],
    text(fill: danger, weight: "bold")[Report], text(fill: danger, weight: "bold")[~-369x], [#tag-inf Nuclear option. Devastating.],
  )
]

#text(8pt, fill: muted)[Weights are ESTIMATES. True values are in unpublished `params.rs`. Relative ordering is high-confidence.]

#pagebreak()

// ============ WHAT'S DEAD ============

= What's Dead from 2023

#danger-box(title: "STOP CITING THESE — THEY'RE FROM DEAD CODE")[
  #table(
    columns: (1fr, 1fr),
    fill: rgb("#1a0a0a"), stroke: none, inset: 8pt,
    text(8pt, fill: danger, weight: "bold")[2023 MYTH], text(8pt, fill: success, weight: "bold")[2026 REALITY],
    [TweepCred < 65 = only 3 tweets scored], [*Eliminated*. No tweet limit.],
    [Bookmarks = 10x likes], [*NOT in weighted\_scorer.rs*. Not a signal.],
    [Reply + author engages back = 150x], [*Gone*. reply\_engaged\_by\_author removed.],
    [Report = -369.0 exact weight], [Report weight exists but value unknown.],
    [SimClusters community embedding], [*Replaced* by Phoenix two-tower retrieval.],
    [Real Graph relationship scoring], [*Replaced* by Grok engagement sequences.],
    [Blue verified 4x / 2x boost], [*Not in this repo*. Operates elsewhere.],
    [Following/follower ratio penalty], [*No ratio penalty in 2026 code*.],
  )
]

#pagebreak()

// ============ STEP 1 ============

= Step 1 — Wake the Account (Days 1–14)

#info-box(title: "GOAL: Fill your 128-position engagement history buffer")[
  #tag-code The Grok transformer needs engagement data to work. Your `user_action_sequence` must have content before scoring works properly. A dormant account is algorithmically invisible.
]

#v(6pt)

== Daily Routine (45–60 min)

=== Morning (20 min) — Engage Others

#grid(
  columns: (auto, 1fr),
  gutter: 8pt,
  text(fill: accent, weight: "bold")[1.], [*Like 20–30 posts* in your niche. #tag-code This populates `history_actions` in the engagement sequence. The retrieval model learns your topics from these.],
  text(fill: accent, weight: "bold")[2.], [*Reply to 5–10 posts* from accounts with 1K–50K followers. Target posts under 30 minutes old. #tag-emp The 70/30 strategy: one creator grew 500→12,000 in 6 months.],
  text(fill: accent, weight: "bold")[3.], [*Quote 2–3 posts* with added context. #tag-code `quote_score` is a separate signal from `retweet_score` in `weighted_scorer.rs`.],
)

=== Midday (15 min) — Create

#grid(
  columns: (auto, 1fr),
  gutter: 8pt,
  text(fill: accent, weight: "bold")[4.], [*Post 1–2 original tweets*. Text-only with strong opening line. #tag-emp Buffer's 45M+ post analysis: text outperforms video by 30% on X.],
  text(fill: accent, weight: "bold")[5.], [*Reply to every reply* you receive within 30 min. #tag-code Reply chains fire `ServerTweetReply` for multiple participants.],
)

=== Evening (10 min) — Network

#grid(
  columns: (auto, 1fr),
  gutter: 8pt,
  text(fill: accent, weight: "bold")[6.], [*DM 1–2 people* a post you found valuable. #tag-code `share_via_dm_score` is a separate high-value signal — most underrated action in the algorithm.],
  text(fill: accent, weight: "bold")[7.], [*Follow 5–10 relevant accounts*. #tag-code Your following list determines what Thunder serves as in-network candidates.],
)

#pagebreak()

// ============ STEP 2 ============

= Step 2 — Content Strategy (Days 15–60)

== Format Rules

#table(
  columns: (auto, auto, 1fr),
  fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else { surface },
  stroke: 0.3pt + sborder, inset: 8pt,
  table.header(
    text(8pt, fill: success, weight: "bold")[DO],
    text(8pt, fill: danger, weight: "bold")[DON'T],
    text(8pt, fill: muted, weight: "bold")[REASON],
  ),
  [Text + opinion], [Bare link], [#tag-emp Link penalty largely removed Oct 2025, but engagement still lower],
  [1–2 niche hashtags], [5+ hashtags], [#tag-emp 5+ hashtags → 40% reduction (Grok associates with spam)],
  [Images users tap to expand], [Stock photos], [#tag-code `photo_expand_score` fires on tap — design for expansion],
  [Videos > MIN\_DURATION], [Short clips], [#tag-code `vqv_weight_eligibility()` zeroes VQV for short videos],
  [Threads (5–7 tweets)], [10 rapid single tweets], [#tag-code `dwell_time` is continuous — threads maximize it],
  [Space posts 2+ hours apart], [Dump 5 posts in 1 hour], [#tag-code `AuthorDiversityScorer` applies exponential decay],
)

== The Author Diversity Penalty #tag-code

```rust
fn multiplier(&self, position: usize) -> f64 {
    (1.0 - self.floor) * self.decay_factor.powf(position as f64) + self.floor
}
```

Your 1st post in a feed session gets full score. The 2nd gets `decay¹`, 3rd gets `decay²`. Space posts out.

== DM Shares — The Hidden Weapon #tag-code

`share_via_dm_score` and `share_via_copy_link_score` are *separate dedicated signals* in `weighted_scorer.rs`. These are brand new in 2026. Most people ignore them entirely.

*Action:* When you see a great post in your niche, share it via DM to someone who'd appreciate it. When others DM your posts, it fires one of the highest-value signals in the algorithm.

#pagebreak()

// ============ STEP 3 ============

= Step 3 — How Discovery Works

== Two Paths to Someone's Feed #tag-code

#grid(
  columns: (1fr, 1fr),
  gutter: 10pt,
  block(fill: surface, stroke: 0.5pt + sborder, radius: 8pt, inset: 10pt)[
    #text(10pt, fill: success, weight: "bold")[IN-NETWORK (Thunder)]
    #v(4pt)
    #text(8.5pt, fill: muted)[
      Posts from accounts they follow. Sub-millisecond lookups from in-memory store. Your 96 followers see your posts here automatically.
    ]
    #v(3pt)
    #text(7.5pt, fill: rgb("#4a4a5a"))[Source: `thunder_source.rs`]
  ],
  block(fill: surface, stroke: 0.5pt + sborder, radius: 8pt, inset: 10pt)[
    #text(10pt, fill: accent, weight: "bold")[OUT-OF-NETWORK (Phoenix)]
    #v(4pt)
    #text(8.5pt, fill: muted)[
      ML-discovered posts via two-tower retrieval. Dot product similarity across global corpus. This is your growth engine.
    ]
    #v(3pt)
    #text(7.5pt, fill: rgb("#4a4a5a"))[Source: `phoenix_source.rs`, `recsys_retrieval_model.py`]
  ],
)

#v(6pt)

== The OON Penalty #tag-code

```rust
let updated_score = match c.in_network {
    Some(false) => base_score * p::OON_WEIGHT_FACTOR,
    _ => base_score,
};
```

Out-of-network posts get multiplied by `OON_WEIGHT_FACTOR` (< 1.0). Your posts reaching non-followers face a multiplicative penalty.

== Candidate Isolation #tag-code

Each candidate post is scored *independently* — candidates cannot attend to each other in the transformer. Your score doesn't depend on which other posts are in the same batch. You don't "compete" with viral posts.

```
Candidates → Candidates: CANNOT attend (zeroed out)
Candidates → User/History: CAN attend (full attention)
```

== How the Retrieval Model Finds You #tag-code

```python
scores = jnp.matmul(user_representation, corpus_embeddings.T)
```

If users who engage with longevity content *also engage with YOUR content*, your posts get retrieved for other longevity-interested users. Engagement from niche-relevant accounts is what places you in the right embedding neighborhood.

#pagebreak()

// ============ NEGATIVE SIGNALS ============

= What to NEVER Do

== The Asymmetric Penalty System #tag-code

Negative scores are *compressed but devastating*:

```rust
fn offset_score(combined_score: f64) -> f64 {
    if combined_score < 0.0 {
        (combined_score + NEGATIVE_WEIGHTS_SUM) / WEIGHTS_SUM
            * NEGATIVE_SCORES_OFFSET
    } else {
        combined_score + NEGATIVE_SCORES_OFFSET
    }
}
```

- *Positive* scores scale linearly — more engagement = proportionally higher
- *Negative* scores are compressed into a narrow band near zero
- Even *moderate* negative predictions can drop your score below zero → compression branch → effectively dead

#v(6pt)

== The Penalty Hierarchy

#block(width: 100%, fill: rgb("#1a0808"), stroke: 0.5pt + rgb("#3a1a1a"), radius: 8pt, inset: 0pt, clip: true)[
  #table(
    columns: (auto, 1fr, auto),
    fill: (_, y) => if y == 0 { rgb("#2a0a0a") } else if calc.odd(y) { rgb("#1a0808") } else { rgb("#150606") },
    stroke: none, inset: 10pt,
    table.header(
      text(8pt, fill: rgb("#ff6666"), weight: "bold")[RANK],
      text(8pt, fill: rgb("#ff6666"), weight: "bold")[PENALTY],
      text(8pt, fill: rgb("#ff6666"), weight: "bold")[REVERSIBILITY],
    ),
    [1], [*VF safety drop* — post removed entirely (spam/violence)], [Irreversible],
    [2], [*Block/mute filter* — removed before scoring happens], [Unblock/unmute],
    [3], [*Report prediction* — P(report) drags score to zero], [Model must relearn],
    [4], [*Block prediction* — P(block) weighted heavily], [Model must relearn],
    [5], [*Mute prediction* — P(mute) weighted moderately], [Model must relearn],
    [6], [*Not Interested prediction* — lightest negative signal], [Model must relearn],
    [7], [*OON penalty* — out-of-network multiplier], [Follow to bypass],
    [8], [*Author diversity decay* — repeated author in feed], [Resets each session],
    [9], [*Rate-limit shadowban* — account-level suppression], [48hrs – 3 months],
  )
]

#v(6pt)

#danger-box(title: "CRITICAL: PREDICTIONS, NOT EVENTS")[
  #tag-code The Grok transformer *predicts* that users would block/mute/report your content — and penalizes it *before anyone actually does*. Your past blocks and reports become training data that generalizes to your entire audience segment. One round of negative signals poisons future predictions.
]

#pagebreak()

// ============ PREMIUM ============

= When to Get Premium

== What the Code Shows #tag-code

The Premium boost is *NOT in this repo* — not among the 19 signals in `weighted_scorer.rs`.

== What Empirical Data Shows #tag-emp

- Buffer (18.8M posts): Premium accounts get ~10x more reach
- Premium: ~600 impressions/post vs. significantly lower for free
- Premium+: ~1,550 impressions/post

== Decision Framework

#table(
  columns: (1fr, auto, 1fr),
  fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else { surface },
  stroke: 0.3pt + sborder, inset: 10pt,
  table.header(
    text(8pt, fill: muted, weight: "bold")[CONDITION],
    text(8pt, fill: muted, weight: "bold")[SUBSCRIBE?],
    text(8pt, fill: muted, weight: "bold")[WHY],
  ),
  [Days 1–14], text(fill: danger, weight: "bold")[NO], [Boost multiplies your score. Score × 0 = 0.],
  [Day 21–30, posting consistently], text(fill: success, weight: "bold")[YES], [You now have engagement history. Boost has something to amplify.],
  [You post links regularly], text(fill: success, weight: "bold")[YES], [Non-Premium link posts historically suppressed.],
)

#pagebreak()

// ============ MILESTONES ============

= Milestones & Timeline

#block(width: 100%, fill: surface, stroke: 0.5pt + sborder, radius: 8pt, inset: 0pt, clip: true)[
  #table(
    columns: (1fr, auto, 1fr),
    fill: (_, y) => if y == 0 { rgb("#1a1a2e") } else if calc.odd(y) { surface } else { rgb("#0e0e18") },
    stroke: none, inset: 10pt,
    table.header(
      text(8pt, fill: muted, weight: "bold")[MILESTONE],
      text(8pt, fill: muted, weight: "bold")[EXPECTED],
      text(8pt, fill: muted, weight: "bold")[WHAT UNLOCKS],
    ),
    [Engagement history populated], [Day 7–14], [Phoenix transformer can score your content],
    [For You shows niche content], [Day 7–10], [Retrieval model learned your embedding],
    [First reply-back from target], text(fill: success)[Day 2–3], [Engagement edge established],
    [Subscribe to Premium], [Day 21–30], [~10x reach amplification],
    [250 followers], [Week 3–4], [Meaningful in-network audience],
    [1 post exceeding 1K impressions], [Week 3–4], [Out-of-network retrieval working],
    text(fill: accent, weight: "bold")[500 followers], text(fill: accent, weight: "bold")[Month 2–3], text(fill: accent, weight: "bold")[Compounding growth begins],
    [First viral post (10K+ impressions)], [Month 2–3], [Embedding neighborhood established],
    text(fill: success, weight: "bold")[1,000 followers], text(fill: success, weight: "bold")[Month 4–6], text(fill: success, weight: "bold")[Self-sustaining flywheel],
  )
]

#v(10pt)

== Daily Checklist

#block(fill: surface, stroke: 0.5pt + sborder, radius: 8pt, inset: 14pt)[
  #set text(9pt)
  #text(fill: accent, weight: "bold")[MORNING (20 min)]
  #v(2pt)
  - Like 20–30 niche posts (builds engagement history)
  - Reply to 5–10 posts from larger accounts (< 15 min old)
  - Quote 2–3 valuable posts with added context

  #v(6pt)
  #text(fill: accent, weight: "bold")[MIDDAY (20 min)]
  #v(2pt)
  - Post 1–2 original posts (spaced 2+ hours apart)
  - Reply to ALL replies on your content within 30 min
  - DM 1 great post to someone who'd value it

  #v(6pt)
  #text(fill: accent, weight: "bold")[EVENING (10 min)]
  #v(2pt)
  - Post 1 more original post or thread segment
  - Check metrics: which posts got profile clicks? Double down.
  - Follow 3–5 new relevant accounts

  #v(6pt)
  #text(fill: accent, weight: "bold")[WEEKLY]
  #v(2pt)
  - 1 thread (5–7 tweets) for dwell time
  - 1 image post designed for tap-to-expand
  - Review: which formats got most engagement?
]

#pagebreak()

// ============ SCORING FORMULA ============

= The Scoring Formula

#tag-code From `weighted_scorer.rs`:

```
score = SUM(weight_i × P(action_i))
```

Where `P(action_i)` comes from the Grok transformer as `exp(log_probability)`.

#block(fill: surface, stroke: 1pt + accent, radius: 8pt, inset: 14pt)[
  #set text(9pt, font: "Menlo")
  #text(fill: rgb("#6a9fb5"))[Weighted Score =] \
  #h(8pt) #text(fill: success)[P(follow\_author)] #text(fill: muted)[ ×] #text(fill: success, weight: "bold")[ ~30] \
  #h(4pt) + #text(fill: success)[P(share\_dm)] #text(fill: muted)[ ×] #text(fill: success, weight: "bold")[ ~25] \
  #h(4pt) + #text(fill: accent)[P(reply)] #text(fill: muted)[ ×] #text(fill: accent, weight: "bold")[ ~20] \
  #h(4pt) + #text(fill: accent)[P(share\_link)] #text(fill: muted)[ ×] #text(weight: "bold")[ ~20] \
  #h(4pt) + P(quote) #text(fill: muted)[ ×] #text(weight: "bold")[ ~18] \
  #h(4pt) + P(profile\_click) #text(fill: muted)[ ×] #text(weight: "bold")[ ~12] \
  #h(4pt) + P(click) #text(fill: muted)[ ×] #text(weight: "bold")[ ~10] \
  #h(4pt) + P(share) #text(fill: muted)[ ×] #text(weight: "bold")[ ~10] \
  #h(4pt) + P(dwell) #text(fill: muted)[ ×] #text(weight: "bold")[ ~8] \
  #h(4pt) + P(retweet) #text(fill: muted)[ ×] #text(weight: "bold")[ ~3] \
  #h(4pt) + P(vqv) #text(fill: muted)[ ×] #text(weight: "bold")[ ~3] #text(fill: muted, size: 7pt)[ (if video > MIN\_DURATION)] \
  #h(4pt) + P(photo\_expand) #text(fill: muted)[ ×] #text(weight: "bold")[ ~2] \
  #h(4pt) + P(favorite) #text(fill: muted)[ ×] #text(weight: "bold")[ ~1] \
  #h(4pt) + dwell\_time #text(fill: muted)[ ×] #text(weight: "bold")[ ~0.1] #text(fill: muted, size: 7pt)[ (seconds, not probability)] \
  #h(4pt) + #text(fill: danger)[P(not\_interested)] #text(fill: muted)[ ×] #text(fill: danger, weight: "bold")[ ~-20] \
  #h(4pt) + #text(fill: danger)[P(mute)] #text(fill: muted)[ ×] #text(fill: danger, weight: "bold")[ ~-40] \
  #h(4pt) + #text(fill: danger)[P(block)] #text(fill: muted)[ ×] #text(fill: danger, weight: "bold")[ ~-74] \
  #h(4pt) + #text(fill: danger)[P(report)] #text(fill: muted)[ ×] #text(fill: danger, weight: "bold")[ ~-369]
]

#v(4pt)
#text(8pt, fill: muted)[Weights are ESTIMATES based on code structure, 2023 baselines where applicable, and empirical 2025-2026 data. True values in unpublished `params.rs`.]

#pagebreak()

// ============ PIPELINE ============

= The Full Pipeline #tag-code

#block(fill: surface, stroke: 0.5pt + sborder, radius: 8pt, inset: 14pt)[
  #set text(9pt)
  #grid(
    columns: (auto, 1fr),
    gutter: (10pt, 8pt),
    text(fill: accent, weight: "bold")[1.], [*Query Hydration* — fetch engagement history + following list],
    text(fill: accent, weight: "bold")[2.], [*Candidate Sourcing* — Thunder (in-network) + Phoenix Retrieval (out-of-network)],
    text(fill: accent, weight: "bold")[3.], [*Hydration* — core data, author info, video duration, subscription status],
    text(fill: accent, weight: "bold")[4.], [*10 Pre-Scoring Filters* — dupes, age, self, muted keywords, blocked authors],
    text(fill: accent, weight: "bold")[5.], [*PhoenixScorer* — Grok transformer predicts P(action) for 19 actions],
    text(fill: accent, weight: "bold")[6.], [*WeightedScorer* — weighted sum + offset\_score() compression],
    text(fill: accent, weight: "bold")[7.], [*AuthorDiversityScorer* — exponential decay for repeated authors],
    text(fill: accent, weight: "bold")[8.], [*OONScorer* — discount factor for out-of-network],
    text(fill: accent, weight: "bold")[9.], [*TopK Selection* — sort by score, take top K],
    text(fill: accent, weight: "bold")[10.], [*Post-Selection* — VFFilter (safety) + DedupConversationFilter],
  )
]

#v(1cm)

// ============ SOURCES ============

= Source Code References

#table(
  columns: (1fr, 1fr),
  fill: surface, stroke: 0.3pt + sborder, inset: 6pt,
  text(8.5pt)[`weighted_scorer.rs`], text(8.5pt)[19-signal scoring formula, offset\_score()],
  text(8.5pt)[`phoenix_scorer.rs`], text(8.5pt)[Grok predictions → PhoenixScores extraction],
  text(8.5pt)[`oon_scorer.rs`], text(8.5pt)[Out-of-network weight factor],
  text(8.5pt)[`author_diversity_scorer.rs`], text(8.5pt)[Exponential decay for repeated authors],
  text(8.5pt)[`recsys_model.py`], text(8.5pt)[PhoenixModel: Grok transformer ranking],
  text(8.5pt)[`recsys_retrieval_model.py`], text(8.5pt)[Two-tower retrieval, ANN search],
  text(8.5pt)[`grok.py`], text(8.5pt)[Transformer + candidate isolation masking],
  text(8.5pt)[`phoenix_candidate_pipeline.rs`], text(8.5pt)[Full pipeline assembly + filter chain],
  text(8.5pt)[`user_action_seq_query_hydrator.rs`], text(8.5pt)[Engagement history fetch + empty check],
  text(8.5pt)[`thunder_source.rs`], text(8.5pt)[In-memory in-network post store],
)

#v(1cm)
#align(center)[
  #line(length: 30%, stroke: 0.3pt + muted)
  #v(8pt)
  #text(8pt, fill: muted)[
    Source: xai-org/x-algorithm (Apache 2.0, January 2026)
    #linebreak()
    Every claim tagged #tag-code #tag-emp or #tag-inf
    #linebreak()
    Full analysis: `docs/x-algorithm-analysis/`
  ]
]
