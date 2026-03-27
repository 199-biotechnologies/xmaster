# 06 -- Penalties & Negative Signals

> Deep dive into X/Twitter's penalty systems, negative signal weights, and content
> filtering mechanisms. Sourced from `twitter/the-algorithm` (2023),
> `xai-org/x-algorithm` (January 2026), and empirical platform analysis.

---

## 1. Negative Engagement Weights (2023 Heavy Ranker)

The 2023 open-source release (`twitter/the-algorithm-ml`) exposed the Heavy Ranker's
weighted scoring formula with explicit negative terms.

### Source: `projects/home/recap/README.md`

| Parameter | Weight | Trigger |
|-----------|--------|---------|
| `scored_tweets_model_weight_negative_feedback_v2` | **-74.0** | Block, mute, "show less often" |
| `scored_tweets_model_weight_report` | **-369.0** | Report Tweet |

### The Asymmetry Problem

Positive weights from the same config for context:

| Signal | Weight |
|--------|--------|
| `scored_tweets_model_weight_fav` | +0.5 |
| `scored_tweets_model_weight_retweet` | +1.0 |
| `scored_tweets_model_weight_reply` | +13.5 |
| `scored_tweets_model_weight_good_profile_click` | +12.0 |
| `scored_tweets_model_weight_bookmark` | +10.0 |

The math is devastating:

- **1 report = -369.0 score impact**
- **1 like = +0.5 score impact**
- Therefore: **1 report destroys the equivalent of 738 likes**
- **1 block/mute = -74.0**, equivalent to **148 likes destroyed**
- A single "show less often" click wipes out the value of **148 likes**

Reports are weighted **5x heavier** than blocks/mutes, making coordinated reporting
the most effective suppression vector on the platform.

### How Negative Weights Interact with Positive Scores

The Heavy Ranker computes a **weighted linear combination** across all predicted
engagement probabilities:

```
Final Score = SUM(weight_i * P(action_i))
```

Negative weights are not separate penalties -- they are subtracted directly from
the same sum as positive signals. A tweet with high like probability but even moderate
report probability can score net-negative and be buried entirely.

---

## 2. Negative Engagement Weights (2026 Grok-Powered)

The January 2026 `xai-org/x-algorithm` release replaced the Heavy Ranker with a
Grok-based transformer. The scoring architecture changed but the penalty mechanics
evolved rather than disappeared.

### Source: `home-mixer/scorers/weighted_scorer.rs`

The `WeightedScorer` applies weights to **19 distinct engagement signals** predicted
by Phoenix (Grok's ranking model):

**14 positive signals:**
- `favorite_score`, `reply_score`, `retweet_score`, `photo_expand_score`
- `click_score`, `profile_click_score`, `vqv_score` (video quality views)
- `share_score`, `share_via_dm_score`, `share_via_copy_link_score`
- `dwell_score`, `quote_score`, `quoted_click_score`, `follow_author_score`

**4 negative signals:**
- `not_interested_score` -- P(user marks "not interested")
- `block_author_score` -- P(user blocks the author)
- `mute_author_score` -- P(user mutes the author)
- `report_score` -- P(user reports the post)

**1 conditional signal:**
- `vqv_score` -- only weighted when video duration exceeds `MIN_VIDEO_DURATION_MS`

### The `offset_score` Function

The critical penalty mechanics live in the offset function:

```rust
fn offset_score(combined_score: f64) -> f64 {
    if p::WEIGHTS_SUM == 0.0 {
        combined_score.max(0.0)
    } else if combined_score < 0.0 {
        (combined_score + p::NEGATIVE_WEIGHTS_SUM)
            / p::WEIGHTS_SUM
            * p::NEGATIVE_SCORES_OFFSET
    } else {
        combined_score + p::NEGATIVE_SCORES_OFFSET
    }
}
```

Three cases:

1. **Degenerate** (`WEIGHTS_SUM == 0.0`): Score floors at 0.0
2. **Net-negative score**: Rescaled using `NEGATIVE_WEIGHTS_SUM / WEIGHTS_SUM * NEGATIVE_SCORES_OFFSET`
   -- this compresses negative scores into a small range rather than letting them
   plummet to arbitrary depths
3. **Net-positive score**: `NEGATIVE_SCORES_OFFSET` is added as a baseline shift

The exact values of `NEGATIVE_WEIGHTS_SUM`, `WEIGHTS_SUM`, and `NEGATIVE_SCORES_OFFSET`
are defined in the `params` module but **redacted from the open-source release** for
security reasons.

### Key Architectural Change from 2023

In 2023, negative weights were hand-tuned constants (-74.0, -369.0). In 2026, the
weights multiply *predicted probabilities* from a transformer model. The system no
longer needs a user to actually report -- it predicts the *likelihood* of reporting
and penalizes preemptively.

---

## 3. Negative Interaction Graph

### Source: `InteractionGraphNegativeJob.scala`

Twitter maintains a dedicated negative interaction graph tracking per-user-pair
negative signals:

| Signal | Feature Name | Severity |
|--------|-------------|----------|
| Blocks | `FeatureName.NumBlocks` | Highest |
| Mutes | `FeatureName.NumMutes` | High |
| Abuse reports | `FeatureName.NumReportAsAbuses` | High |
| Spam reports | `FeatureName.NumReportAsSpams` | High |
| Unfollows | `FeatureName.NumUnfollows` | Lower |

**Key details:**

- The system retains the **top 500 negative interactions per user** (p99 threshold),
  ordered by `getFeatureCounts` -- edges with more negative signals rank higher
- **Unfollows decay after 90 days**: the code explicitly notes unfollows are "treated
  as less critical than above 4 negative signals, since it deals more with interest
  than health"
- Blocks, mutes, and reports have **no documented decay** -- they persist indefinitely
  in the interaction graph

---

## 4. Visibility Filtering (VF) System

### Source: `visibilitylib/` in `twitter/the-algorithm`

The Visibility Filtering system is X's content policy enforcement layer. It operates
independently from the ranking algorithm and can override any score.

### Architecture

```
Content -> Rule Engine -> Action -> Visibility Result
              |
         SafetyLevel -> PolicyProvider -> VisibilityPolicy -> Rules
```

**Two rule engines** run in parallel (A/B tested):
- `candidateVisibilityRuleEngine` (includes policy provider integration)
- `fallbackVisibilityRuleEngine` (simpler path)

**Four visibility libraries** handle different surfaces:
- **TVL** -- Tweet Visibility Library (timeline)
- **BVL** -- Blender Visibility Library (feed blending)
- **SVL** -- Search Visibility Library (search results)
- **TCVL** -- Timeline Conversations Visibility Library (threads)

Each library has independent short-circuiting, verdict logging, and verdict scribing.

### Visibility Actions (by severity, descending)

| Action | Severity | Effect |
|--------|----------|--------|
| **Appealable** | 17 | Content removed with appeal option |
| **Drop** | 16 | Content removed entirely from view |
| **Tombstone** | 15 | Placeholder shown ("This tweet is unavailable") |
| **TweetInterstitial** | 12 | Composite: interstitial + soft intervention + limited engagement + downrank |
| **Interstitial** | 10 | Warning screen before content display |
| **Downrank (with reason)** | 9 | Reduced visibility in feeds with logged reason |
| **SoftIntervention** | 7 | Warning label, engagement nudge, autoplay suppression |
| **LimitedEngagements** | 6 | Restricted replies, retweets, or likes |
| **TweetVisibilityNudge** | 3 | Context nudge with CTA |
| **ComplianceTweetNotice** | 2 | Legal/compliance notice |
| **Avoid** | 1 | Minor downranking |
| **Downrank (base)** | 0 | Default reduced visibility |

### Filtered vs. Removed Content

- **Drop**: Content ceases to exist for the viewer. No trace, no placeholder.
- **Tombstone**: Viewer sees a placeholder ("This Tweet was deleted by the Tweet author").
  The content slot is consumed but content is hidden.
- **Interstitial**: Content exists behind a click-through warning. Engagement is suppressed
  but not eliminated.
- **Downrank**: Content remains fully visible but receives reduced distribution. The user
  never knows this happened.
- **Avoid**: Lightest touch -- minor deprioritization that's nearly undetectable.

### Search vs. Timeline Visibility

Search and timeline use **different visibility libraries** (SVL vs TVL/BVL) with
independent rule sets:

- Content can be **visible on timeline but invisible in search** (common for
  sensitive-labeled content)
- Content can be **visible in search but deprioritized on timeline** (e.g., low-engagement
  content from followed accounts)
- `ViewerOptInFilteringOnSearchRule` and `ViewerOptInBlockingOnSearchRule` apply only to
  search, allowing users to opt into seeing filtered content in search results
- The `LikelyIvsLabelNonFollowerDropRule` drops content from likely "Invalid/Violating
  State" accounts specifically for non-followers

### Rule Types

| Rule Class | Trigger |
|------------|---------|
| `UserHasLabelRule` | Author has a safety label |
| `AuthorLabelAndNonFollowerViewerRule` | Labeled author + non-follower viewer |
| `ConditionWithNotInnerCircleOfFriendsRule` | Viewer not in author's inner circle |
| `OnlyWhenNotAuthorViewerRule` | Author is not viewing own content |
| `AlwaysActRule` | Unconditional application |
| `ExperimentalRule` | A/B test candidate rules |

---

## 5. Trust & Safety Signals

### Content Moderation Models

From `trust_and_safety_models/README.md`:

| Model | Purpose |
|-------|---------|
| **pNSFWMedia** | Detect tweets with NSFW images (adult/porn) |
| **pNSFWText** | Detect tweets with NSFW text/sexual topics |
| **pToxicity** | Detect toxic tweets |
| **pAbuse** | Detect ToS violations: hate speech, harassment, abusive behavior |

Twitter notes they maintain **additional unpublished models** "because of the adversarial
nature of this area."

### Sensitive Content Labels and Distribution Impact

Content flagged as sensitive faces cascading penalties:

1. **Hidden from search by default** -- users must manually enable sensitive content viewing
2. **Removed from For You feed** -- NSFW content is filtered in the post-selection stage
3. **Warning interstitials** -- engagement suppressed behind click-through gates
4. **Account-level contagion** -- repeated sensitive posts trigger permanent account-level
   sensitive labeling, affecting ALL future content distribution

The 2026 algorithm applies NSFW filtering as a **post-selection filter** (after ML ranking),
meaning even high-scoring sensitive content gets removed at the final stage.

### Bot Detection Signals

The algorithm monitors for automation patterns:

| Signal | Detection Method |
|--------|-----------------|
| Mass following | >50 follows/hour triggers review |
| Mass unfollowing | Sudden drop pattern analysis |
| Rapid engagement | >100 likes/hour, repetitive timing patterns |
| Template posting | NLP similarity detection across recent posts |
| API patterns | Posting source metadata analysis |
| Login anomalies | Geographic impossibility, proxy detection |
| Engagement timing | Unnaturally consistent intervals between actions |

### Grok Sentiment Analysis (2026)

The January 2026 system introduced Grok-powered tone evaluation:

- Grok monitors the **tone of every post** in the scoring pipeline
- **Positive/constructive content** receives wider distribution
- **Negative/combative content** gets reduced visibility **even when engagement is high**
- This represents a fundamental shift: engagement alone no longer guarantees distribution
- The system uses reward signals that favor **civil discourse** and demote content flagged
  as ad hominem, trolling, or bad-faith argumentation

---

## 6. TweepCred: The Reputation Penalty System

### Source: `graph/batch/job/tweepcred/`

TweepCred is X's PageRank-derived user reputation score.

### Score Range and Thresholds

| TweepCred Level | Impact |
|-----------------|--------|
| **-128** (minimum) | New account starting point |
| **< +17** | Content not eligible for feed distribution |
| **< 0.65 (65)** | Maximum 3 tweets considered for distribution |
| **>= 0.65 (65)** | All tweets eligible for distribution |
| **50+** | 20-50x distribution boost vs. low-scoring accounts |
| **100** | Maximum score (verified/Premium accounts start here) |

### The Follow Ratio Penalty

From `Reputation.scala` and `UserMass.scala`, the penalty activates in two stages:

**Stage 1: UserMass penalty** (affects PageRank input)
- Triggers when: followings > 500 AND (followings+1)/(followers+1) > 0.6
- Formula: `mass = mass / exp(5.0 * (ratio - 0.6))`
- Effect: Exponential mass reduction for accounts following far more than follow them

**Stage 2: Reputation post-adjustment** (affects final TweepCred)
- Triggers when: followings > 2,500
- Formula: `reputation = reputation / exp(3.0 * (ratio - 0.6) * log(log(numFollowings)))`
- Maximum divisor: **50x reduction** (capped)
- Minimum divisor: 1.0x (no boost)

**Worked example:**
- Account with 10,000 followings and 1,000 followers
- Ratio = 10,000/1,000 = 10.0 (vastly exceeds 0.6 threshold)
- Stage 2 divisor = exp(3.0 * (10.0 - 0.6) * log(log(10000)))
- This account's TweepCred would be divided by the **maximum 50x cap**

### UserMass Components

The `UserMass` calculation factors in:

| Factor | Weight/Impact |
|--------|--------------|
| Suspended account | Mass = 0 (complete exclusion) |
| Verified account | Mass = 100 (maximum) |
| Has messaging device | +0.5 additive |
| Account age < 30 days | Logarithmic ramp: `min(1.0, log(1 + age/15))` |
| Account age > 30 days | Full weight (1.0 multiplier) |
| Restricted account | 0.1x multiplicative penalty |
| Base minimum | 0.01 (never zero for active accounts) |

---

## 7. Content Penalties

### External Link Penalty

**The most severe content-type penalty on the platform.**

Data from [Buffer's analysis](https://buffer.com/resources/links-on-x/) of 18.8 million
posts across 71,000 accounts (August 2025):

| Content Type | Free Account Engagement | Premium Engagement |
|-------------|------------------------|-------------------|
| Text-only | 0.40% | 0.90% |
| Video | 0.25% | 0.85% |
| Image | ~0.30% | ~0.80% |
| **Link post** | **0.00%** | **0.28%** |

- Free accounts: **zero median engagement** on link posts since March 2025
- Premium accounts: ~0.28% (still 69% lower than text-only posts)
- Link posts became "effectively invisible" for non-Premium users

**Update (October 2025):** X announced removal of algorithmic link penalties. The actual
impact of this change remains debated -- early data suggests partial restoration of link
reach, but nowhere near pre-penalty levels.

### Hashtag Overuse Penalty

| Hashtag Count | Effect |
|--------------|--------|
| 0 | Neutral |
| 1-2 (niche-relevant) | +21% engagement boost |
| 3+ | **~40% reach reduction** |
| 5+ | Flagged as spam signal |

Generic hashtags (#follow4follow, #blessed) receive harsher treatment than niche-specific
tags. The algorithm applies NLP to assess hashtag relevance to post content.

### Duplicate Content Detection

The 2026 algorithm detects:
- AI-generated or template-based content
- Recycled posts across engagement groups
- Structurally similar posts within spam clusters

**Penalties:**
- Affects the original poster AND everyone in identified clusters
- Creates **future similarity penalties** -- once flagged, the model learns to suppress
  similar patterns from the same account
- Premium accounts receive a **30% penalty reduction** on duplicate content violations

### All-Caps Text

Receives major penalization (specific weight not disclosed). The content moderation
models treat all-caps as a toxicity/aggression signal.

### Low Dwell Time Signal

Posts that users view for **less than 3 seconds** before scrolling trigger negative signals:

- Quality multiplier drops **15-20%**
- Algorithmic distribution walls strengthen
- Trust classifications lower
- The effect compounds: low dwell -> fewer impressions -> lower engagement rate -> lower TweepCred

---

## 8. Network Penalties

### Mass Follow/Unfollow

| Behavior | Penalty |
|----------|---------|
| Mass unfollowing (sudden large drop) | **3-month shadowban** with dramatically reduced visibility |
| Rapid following (>50/hour) | Flagged for bot review, immediate distribution reduction |
| Follow/unfollow cycling | Pattern detection triggers extended suppression |

The algorithm interprets sudden follower loss as evidence that content quality has
declined or the account is spammy. The 3-month duration is specifically documented as
the recovery period for mass-unfollow events.

### Follow/Follower Ratio Penalty

The exponential penalty from TweepCred (Section 6) is the primary mechanism:

| Ratio (following/followers) | Effect |
|-----------------------------|--------|
| < 0.6 | No penalty |
| 0.6 - 1.0 | Moderate exponential penalty |
| 1.0 - 5.0 | Severe exponential penalty |
| > 5.0 | Near-maximum 50x TweepCred reduction |

This penalty is **permanent and real-time** -- it recalculates on every TweepCred update.
The only way to recover is to improve the ratio (gain followers or unfollow accounts).

### Block/Mute Cascade Effects

When user A blocks/mutes user B:

1. **Interaction graph updated**: NumBlocks/NumMutes incremented in the negative graph
2. **Bidirectional signal**: B's content is suppressed for A, but A's *action* also
   contributes to B's negative signal profile
3. **Network propagation**: If multiple users in the same cluster block B, the algorithm
   infers broader content quality issues
4. **Top-500 persistence**: The system retains the top 500 negative interactions per user
   with no documented expiry for blocks/mutes
5. **Score impact**: The 2026 `block_author_score` prediction learns from historical
   block patterns -- accounts that have been blocked frequently have higher predicted
   block probability for all future content, creating a **permanent distribution drag**

### Mutual Follow Patterns

- Mutual follows generate **positive** interaction graph signals
- However, large clusters of mutual follows among low-engagement accounts trigger
  **engagement farm detection**
- Authentic mutual follows between actively interacting accounts strengthen
  distribution; reciprocal follows without interaction are neutral-to-negative

---

## 9. Cold Start Suppression

New accounts face compound penalties:

### The Engagement Debt Mechanism

If an account's first ~100 tweets generate:
- Less than **0.5% engagement rate**
- Excessive scroll-passes (low dwell time)
- Minimal profile clicks

The system applies **Cold Start Suppression**:
- Initial distribution reduced to **~10% of normal levels**
- Example: 100 impressions instead of 1,000 in the first 10 minutes
- Recovery requires "extraordinary engagement" from the suppressed sample size
- This creates a **nearly impossible catch-up cycle** for accounts that start poorly

### TweepCred Cold Start

- New accounts start at **-128** TweepCred
- The minimum threshold for any distribution is **+17**
- A new account must climb **145 points** before content appears in feeds
- Premium subscribers get +100 instant boost (starting at -28 instead of -128),
  reaching distribution eligibility almost immediately

---

## 10. Shadowban Tiers and Duration

| Trigger | Duration | Severity |
|---------|----------|----------|
| First-time minor violation | 48-72 hours | Distribution reduced |
| Repeat minor violations | 7-14 days | Significant suppression |
| Mass unfollow event | **3 months** | Dramatic visibility reduction |
| Continued violations while shadowbanned | Extended indefinitely | Escalation risk |
| Severe/persistent violations | Permanent | Algorithmic restriction |

### Types of Shadowban

1. **Search ban**: Posts don't appear in search results (SVL filtering)
2. **Ghost ban**: Posts invisible to non-followers (TVL/BVL filtering)
3. **Reply deboosting**: Replies hidden behind "Show more replies" (TCVL filtering)
4. **Thread ban**: Entire conversation threads suppressed
5. **Full suppression**: All of the above simultaneously

---

## 11. The 2026 Pre-Scoring Filters

### Source: `xai-org/x-algorithm` README

Before any ML scoring occurs, **9 pre-scoring filters** eliminate content:

1. **Duplicate filter**: Removes duplicate posts in the candidate set
2. **Age filter**: Removes posts exceeding maximum age threshold
3. **Self-post filter**: Removes user's own posts from their feed
4. **Block filter**: Removes content from blocked authors
5. **Mute filter**: Removes content from muted authors
6. **Muted keyword filter**: Removes posts containing user's muted keywords
7. **Previously seen filter**: Removes already-viewed posts
8. **Recently served filter**: Removes recently shown posts
9. **Subscription content filter**: Removes paywalled content user can't access

**2 post-selection filters** (after scoring):

1. **Visibility filter**: Removes deleted/spam/violence/gore content
2. **Conversation dedup filter**: Deduplicates conversation threads

The pre-scoring filters execute **before** the Grok transformer sees the content,
meaning blocked/muted authors never even get scored. The post-selection filters act as
a final safety net after ranking.

### Author Diversity Scorer

From `home-mixer/scorers/author_diversity_scorer.rs`:

When the same author appears multiple times in a feed, subsequent posts receive
**exponentially decaying scores**:

```
multiplier = (1.0 - floor) * decay_factor^position + floor
```

- `position`: 0-indexed count of how many times this author has appeared
- `decay_factor`: Controls penalty steepness (param: `AUTHOR_DIVERSITY_DECAY`)
- `floor`: Minimum multiplier to prevent total suppression (param: `AUTHOR_DIVERSITY_FLOOR`)

First post: full score. Second post: reduced. Third post: further reduced. The floor
prevents any single author's content from being zeroed out entirely.

### Out-of-Network Scorer

From `home-mixer/scorers/oon_scorer.rs`:

Out-of-network content (discovered via Phoenix retrieval, not from followed accounts)
receives a **multiplicative penalty**:

```
score = base_score * OON_WEIGHT_FACTOR (if not in_network)
```

The `OON_WEIGHT_FACTOR` value is redacted but is definitionally < 1.0, meaning
out-of-network content must score proportionally higher to compete with in-network posts.

---

## 12. Recovery Mechanisms

### Penalty Duration Summary

| Penalty Type | Duration | Recovery Method |
|-------------|----------|----------------|
| Report-based score reduction | Per-tweet (instant) | No recovery; tweet is permanently deprioritized |
| Block/mute interaction graph entry | Indefinite | Only removed if user unblocks/unmutes |
| Unfollow signal | **90-day decay** | Automatic after 90 days |
| TweepCred ratio penalty | Real-time | Improve follower/following ratio |
| Mass-unfollow shadowban | **3 months** | Wait + consistent positive behavior |
| Minor shadowban | 48 hours - 14 days | Pause activity, remove violating content |
| Cold start suppression | Until engagement debt is cleared | Produce high-engagement content from suppressed baseline |
| Sensitive content label | Permanent (account-level) | No documented recovery path |
| External link penalty | Per-post | Post links in replies instead |

### Rebuilding After Negative Signals

1. **Immediate actions** (first 48-72 hours):
   - Stop ALL activity that triggered the penalty
   - Delete any content that received reports or was flagged
   - Do not taper off -- stop completely

2. **Short-term recovery** (1-2 weeks):
   - Limit follows/likes to under 5 per hour
   - Post text-only, original content
   - Focus on replies to larger accounts (builds interaction graph)
   - Avoid hashtags, links, and call-to-action patterns

3. **Medium-term rebuilding** (1-3 months):
   - Consistent daily posting (1-3 posts/day, never >10)
   - Maximize dwell time: hooks, storytelling, visual content
   - Build genuine mutual interactions (replies, quote tweets)
   - Slowly reintroduce multimedia content

4. **TweepCred recovery**:
   - Clean and complete bio
   - Authentic follow patterns (follow accounts you engage with)
   - Coherent language and topic consistency
   - Avoid engagement pods and follow-for-follow schemes

### Account Age: Help or Hurt After Dormancy?

Account age contributes positively to UserMass (the `age > 30 days` factor gives full
weight), but **dormancy itself is neutral** -- the algorithm doesn't penalize inactivity.

However, returning from dormancy with aggressive behavior (mass posting, rapid following)
triggers the same bot-detection signals as a new account. The optimal dormancy recovery
is **gradual re-engagement**: 1-2 posts/day for the first week, slowly increasing.

### Positive Signal Accumulation vs. Negative Signal Decay

The asymmetry is stark:

| Signal Type | Accumulation/Decay |
|-------------|-------------------|
| Likes (positive) | +0.5 per event, no compounding |
| Reports (negative) | -369.0 per event, no documented decay |
| Blocks (negative) | Permanent in interaction graph |
| Unfollows (negative) | 90-day decay |
| TweepCred (aggregate) | Slow upward climb, fast downward crash |

**Negative signals accumulate faster and persist longer than positive signals.** A single
bad day (multiple reports) can take months of consistent positive behavior to overcome.
The system is architecturally biased toward punishment over reward -- by design, to
protect platform health at the cost of individual account resilience.

---

## 13. The Complete Penalty Stack

A single post passes through this gauntlet:

```
1. Pre-scoring filters (9 filters -- hard removal)
   |
2. Phoenix/Grok transformer (predicts 19 engagement probabilities)
   |
3. WeightedScorer (applies positive + negative weights to predictions)
   |-- Negative predictions: P(block), P(mute), P(report), P(not_interested)
   |-- offset_score() handles net-negative scores
   |
4. Author Diversity Scorer (penalizes repeated authors)
   |
5. Out-of-Network Scorer (penalizes non-followed content)
   |
6. Post-selection filters (spam/violence/gore removal)
   |
7. Visibility Filtering (Drop/Tombstone/Interstitial/Downrank)
   |-- Separate rule engines per surface (timeline, search, conversations)
   |-- Safety labels checked
   |-- Content policy enforcement
   |
8. Final feed assembly
```

At each stage, content can be killed, suppressed, or penalized. The compounding effect
means a post with even moderate negative signals faces multiplicative suppression across
multiple independent systems.

---

## Sources

### Primary Source Code
- [`twitter/the-algorithm`](https://github.com/twitter/the-algorithm) -- 2023 open-source release
- [`twitter/the-algorithm-ml`](https://github.com/twitter/the-algorithm-ml) -- ML models and weights
- [`xai-org/x-algorithm`](https://github.com/xai-org/x-algorithm) -- January 2026 Grok-powered release

### Key Source Files
- `projects/home/recap/README.md` -- Heavy Ranker weights (-74.0, -369.0)
- `home-mixer/scorers/weighted_scorer.rs` -- 2026 weighted scoring with offset function
- `home-mixer/scorers/phoenix_scorer.rs` -- 19 engagement probability outputs
- `home-mixer/scorers/oon_scorer.rs` -- Out-of-network penalty
- `home-mixer/scorers/author_diversity_scorer.rs` -- Author diversity decay
- `visibilitylib/.../rules/Rule.scala` -- Visibility rule types
- `visibilitylib/.../rules/Action.scala` -- Visibility actions (Drop through Avoid)
- `visibilitylib/.../configapi/configs/VisibilityDeciderGates.scala` -- VF feature gates
- `interaction_graph/scio/agg_negative/InteractionGraphNegativeJob.scala` -- Negative signals
- `graph/batch/job/tweepcred/Reputation.scala` -- TweepCred post-adjustment penalties
- `graph/batch/job/tweepcred/UserMass.scala` -- UserMass calculation with ratio penalty
- `trust_and_safety_models/README.md` -- Content moderation classifiers

### Analysis and Data
- [Buffer -- Links on X](https://buffer.com/resources/links-on-x/) -- 18.8M post engagement analysis
- [PostEverywhere -- X Algorithm Source Code Analysis](https://posteverywhere.ai/blog/how-the-x-twitter-algorithm-works)
- [Tweet Archivist -- Complete Technical Breakdown](https://www.tweetarchivist.com/how-twitter-algorithm-works-2025)
- [PPC Land -- X Algorithm Source Code Analysis](https://ppc.land/xs-algorithm-source-code-drops-what-it-reveals-about-the-platforms-feed-mechanics/)
- [Circleboom -- Hidden X Algorithm](https://blog-content.circleboom.com/the-hidden-x-algorithm-tweepcred-shadow-hierarchy-dwell-time-and-the-real-rules-of-visibility/)
- [Social Media Today -- Grok Algorithm Shift](https://www.socialmediatoday.com/news/x-formerly-twitter-switching-to-fully-ai-powered-grok-algorithm/803174/)
