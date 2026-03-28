# Engagement Weight Hierarchy: Deduced from X's 2026 Algorithm

**Status**: Reverse-engineered analysis -- weight constants are NOT published.
**Source**: `home-mixer/scorers/weighted_scorer.rs` from [xai-org/x-algorithm](https://github.com/xai-org/x-algorithm) (January 20, 2026 release).
**Date**: March 2026.

---

## What We Know vs. What We Deduce

The `params` module is explicitly excluded from the open-source release:

```rust
pub mod params; // Excluded from open source release for security reasons
```

The scoring code IS published. The weight constants are NOT. Everything below triangulates from three evidence sources:

| Label | Meaning |
|-------|---------|
| **CODE** | Directly observable in published source code |
| **EMPIRICAL** | Derived from community testing, Buffer studies, or third-party analysis |
| **INFERRED** | First-principles reasoning from platform economics and code structure |

---

## The Scoring Pipeline

**CODE**: The pipeline executes four scorers in sequence (from `phoenix_candidate_pipeline.rs` line 127):

```
PhoenixScorer -> WeightedScorer -> AuthorDiversityScorer -> OONScorer
```

1. **PhoenixScorer** -- Grok-based transformer predicts P(action) for 18 discrete actions + 1 continuous action (dwell_time) per candidate tweet. These are probabilities (0.0--1.0), converted from log-probs via `.exp()`.
2. **WeightedScorer** -- Multiplies each P(action) by its hidden weight constant, sums them, then applies `offset_score()` to handle negative contributions.
3. **AuthorDiversityScorer** -- Applies exponential decay to repeated appearances by the same author within a single feed.
4. **OONScorer** -- Multiplies out-of-network candidates by `OON_WEIGHT_FACTOR` (value hidden, but likely < 1.0 given the comment "Prioritize in-network candidates").

---

## The 19 Signals

### Positive Signals (15)

**CODE**: All 15 are summed in `compute_weighted_score()` at lines 49--63.

| # | Signal | Param Constant | Phoenix Action | Type |
|---|--------|---------------|----------------|------|
| 1 | Favorite (Like) | `FAVORITE_WEIGHT` | `ServerTweetFav` | Discrete |
| 2 | Reply | `REPLY_WEIGHT` | `ServerTweetReply` | Discrete |
| 3 | Retweet (Repost) | `RETWEET_WEIGHT` | `ServerTweetRetweet` | Discrete |
| 4 | Photo Expand | `PHOTO_EXPAND_WEIGHT` | `ClientTweetPhotoExpand` | Discrete |
| 5 | Click | `CLICK_WEIGHT` | `ClientTweetClick` | Discrete |
| 6 | Profile Click | `PROFILE_CLICK_WEIGHT` | `ClientTweetClickProfile` | Discrete |
| 7 | Video Quality View | `VQV_WEIGHT` | `ClientTweetVideoQualityView` | Discrete |
| 8 | Share | `SHARE_WEIGHT` | `ClientTweetShare` | Discrete |
| 9 | Share via DM | `SHARE_VIA_DM_WEIGHT` | `ClientTweetClickSendViaDirectMessage` | Discrete |
| 10 | Share via Copy Link | `SHARE_VIA_COPY_LINK_WEIGHT` | `ClientTweetShareViaCopyLink` | Discrete |
| 11 | Dwell (binary) | `DWELL_WEIGHT` | `ClientTweetRecapDwelled` | Discrete |
| 12 | Quote Tweet | `QUOTE_WEIGHT` | `ServerTweetQuote` | Discrete |
| 13 | Quoted Click | `QUOTED_CLICK_WEIGHT` | `ClientQuotedTweetClick` | Discrete |
| 14 | Continuous Dwell Time | `CONT_DWELL_TIME_WEIGHT` | `DwellTime` | Continuous |
| 15 | Follow Author | `FOLLOW_AUTHOR_WEIGHT` | `ClientTweetFollowAuthor` | Discrete |

### Negative Signals (4)

| # | Signal | Param Constant | Phoenix Action | Type |
|---|--------|---------------|----------------|------|
| 16 | Not Interested | `NOT_INTERESTED_WEIGHT` | `ClientTweetNotInterestedIn` | Discrete |
| 17 | Block Author | `BLOCK_AUTHOR_WEIGHT` | `ClientTweetBlockAuthor` | Discrete |
| 18 | Mute Author | `MUTE_AUTHOR_WEIGHT` | `ClientTweetMuteAuthor` | Discrete |
| 19 | Report | `REPORT_WEIGHT` | `ClientTweetReport` | Discrete |

---

## Deduced Weight Hierarchy

### Tier 1 -- VERY HIGH (estimated 10x--50x vs. favorite baseline)

| Signal | Est. Relative Weight | Confidence | Reasoning |
|--------|---------------------|------------|-----------|
| **Reply** | ~15x--27x | HIGH | **EMPIRICAL**: 2023 code had `reply: 13.5` vs `fav: 0.5` = 27x ratio. 2026 code still treats reply as `ServerTweetReply` (server-validated, high-intent action). Community consensus places replies at the top of positive signals. **INFERRED**: X's business model rewards conversation -- replies increase session time for BOTH participants. |
| **Share via DM** | ~15x--30x | MEDIUM | **CODE**: Elevated to its own dedicated signal (was NOT a separate signal in 2023). **INFERRED**: Sending a post via DM is the highest-conviction sharing action -- the user is personally vouching for the content to someone they know. Instagram's algorithm explicitly rates DM sends as the #1 signal for new audience reach (per Mosseri, Jan 2025). X building a separate `ClientTweetClickSendViaDirectMessage` action reflects equivalent intent. **EMPIRICAL**: No direct testing available for X, but the code separation strongly implies premium weighting. |
| **Share via Copy Link** | ~12x--25x | MEDIUM | **CODE**: Also elevated to its own signal (absent in 2023). **INFERRED**: Copying a link means the user is taking the content off-platform entirely -- sharing to WhatsApp, Slack, email, etc. This drives external traffic back to X and new user acquisition. The dedicated tracking implies X values it nearly as highly as DM sharing. Likely slightly lower than DM shares because DMs keep the interaction within X's ecosystem. |
| **Quote Tweet** | ~12x--25x | MEDIUM-HIGH | **EMPIRICAL**: Community consensus estimates ~25x a like. **INFERRED**: Quote tweets are public endorsements that create new content nodes in the graph. They generate their own engagement cascades. The 2026 code maps to `ServerTweetQuote` (server-validated, like reply/retweet). |
| **Follow Author** | ~15x--40x | MEDIUM | **CODE**: Present as a signal (was NOT in the 2023 scorer). **INFERRED**: A follow is the strongest possible signal of content resonance -- the user is subscribing to all future content from this author based on a single post. This is the ultimate "discovery" action X wants to encourage. Likely weighted extremely high but has very low predicted probability (follows are rare), so its contribution to average scores is moderate. |

### Tier 2 -- HIGH (estimated 5x--15x vs. favorite baseline)

| Signal | Est. Relative Weight | Confidence | Reasoning |
|--------|---------------------|------------|-----------|
| **Share (generic)** | ~8x--15x | MEDIUM | **CODE**: Separate from DM share and copy link share. Maps to `ClientTweetShare` -- the act of opening the share sheet. **INFERRED**: This is the "intent to share" signal, before the user picks a specific channel. Probably weighted lower than DM/copy-link since those are completed shares, while this is just opening the share menu. |
| **Profile Click** | ~10x--12x | HIGH | **EMPIRICAL**: 2023 weight was `good_profile_click: 12.0` vs `fav: 0.5` = 24x. **INFERRED**: Profile clicks indicate the user wants to learn more about the author -- a strong discovery signal. X's business depends on users following more accounts. Weight likely preserved near 2023 levels relative to favorite. |
| **Click (conversation)** | ~8x--11x | HIGH | **EMPIRICAL**: 2023 had `good_click: 11.0` and `good_click_v2: 10.0` vs `fav: 0.5`. **INFERRED**: Clicking into a conversation indicates deep interest. The 2026 code collapses this to a single `ClientTweetClick` action. |
| **Dwell (binary)** | ~6x--10x | MEDIUM-HIGH | **EMPIRICAL**: 2023 analysis suggests dwell carries +10 weighting out of 100. The signal `ClientTweetRecapDwelled` fires when the user pauses on the tweet (likely 2+ seconds). **INFERRED**: Passive engagement that captures readers who consume but don't interact. X needs this signal to rank long-form text and images that people read without tapping anything. |
| **Retweet** | ~2x--5x | HIGH | **EMPIRICAL**: 2023 weight was `retweet: 1.0` vs `fav: 0.5` = 2x. Community formulas cite "Retweets x 20" but this conflates the weighting with average probability differences. **INFERRED**: Retweets have become less valued over time as X pivoted toward conversation. The raw weight is likely low (2x--5x of favorite) but the predicted probability of retweeting is also lower than liking, so the contributions balance. |
| **Continuous Dwell Time** | ~3x--8x | MEDIUM | **CODE**: This is the only continuous (non-probability) signal -- it's a raw duration value from `ContinuousActionName::DwellTime`, not a probability. **INFERRED**: Because it's continuous (not 0--1), the weight must be scaled differently. A user dwelling for 30 seconds produces a different input than a probability of 0.7. The weight is likely small in absolute terms but the signal itself can be large. This captures "how long" rather than "did they dwell at all." |

### Tier 3 -- MODERATE (estimated 1x--5x vs. favorite baseline)

| Signal | Est. Relative Weight | Confidence | Reasoning |
|--------|---------------------|------------|-----------|
| **Favorite (Like)** | 1x (baseline) | HIGH | **EMPIRICAL**: 2023 weight was 0.5 -- the lowest positive weight. **INFERRED**: Likes are the most common engagement action with the highest predicted probability. The weight is set low precisely because likes are ubiquitous -- a 0.5 weight on a P(like) of 0.3 produces similar contribution to a 13.5 weight on a P(reply) of 0.01. This is the design principle stated in the 2023 docs: "weights were originally set so that each weighted engagement probability contributes approximately equal amounts." |
| **Photo Expand** | ~1x--3x | MEDIUM | **EMPIRICAL**: Not weighted separately in the 2023 public code. **INFERRED**: Opening a photo indicates interest but is lower-intent than clicking a profile or replying. Useful for image-heavy content ranking. |
| **Quoted Click** | ~2x--5x | LOW-MEDIUM | **CODE**: `ClientQuotedTweetClick` -- clicking into a quote tweet's original content. **INFERRED**: This is a secondary engagement signal. The user saw a quote tweet and was interested enough to visit the original. Moderate weight. |
| **Video Quality View** | ~1x--4x | MEDIUM | **CODE**: Gated behind `vqv_weight_eligibility()` -- only applies to videos longer than `MIN_VIDEO_DURATION_MS`. **EMPIRICAL**: 2023 had `video_playback50: 0.005` -- the lowest weight of any signal. **INFERRED**: The 2026 signal (`ClientTweetVideoQualityView`) is more specific than 50% playback -- it suggests the user watched enough to experience the video's quality. The gating function suggests X raised the bar (no weight for short clips), which means the weight itself may be higher than the 2023 0.005 to compensate. Still, video completion is common for engaging videos, so the weight is likely moderate. |

### Tier 4 -- NEGATIVE (strong asymmetric penalty)

| Signal | Est. Relative Weight | Confidence | Reasoning |
|--------|---------------------|------------|-----------|
| **Report** | ~ -200x to -370x | HIGH | **EMPIRICAL**: 2023 weight was -369.0 vs fav 0.5 = -738x ratio. Reports are the nuclear option. **INFERRED**: X must heavily penalize reported content to maintain trust. Even a small P(report) produces massive negative contribution. |
| **Block Author** | ~ -50x to -100x | HIGH | **EMPIRICAL**: 2023 combined blocks/mutes/"show less" into `negative_feedback_v2: -74.0`. The 2026 code SEPARATES block, mute, and not-interested into three distinct signals. **INFERRED**: Blocking is stronger than muting (permanent relationship severance vs. temporary suppression). Splitting them allows finer-grained punishment. Block weight likely carries most of the old -74.0 penalty. |
| **Mute Author** | ~ -30x to -60x | MEDIUM | **INFERRED**: Muting is softer than blocking -- the user doesn't want to see the content but isn't declaring the author harmful. Lighter penalty than block. |
| **Not Interested** | ~ -15x to -40x | MEDIUM | **INFERRED**: "Not interested" is the lightest negative signal -- a content relevance issue, not a safety issue. The user is providing topical feedback, not condemning the author. This signal helps calibrate the recommendation model without punishing the content creator as harshly. |

---

## The `offset_score()` Function: Negative Signal Asymmetry

**CODE** (lines 83--91):

```rust
fn offset_score(combined_score: f64) -> f64 {
    if p::WEIGHTS_SUM == 0.0 {
        combined_score.max(0.0)
    } else if combined_score < 0.0 {
        (combined_score + p::NEGATIVE_WEIGHTS_SUM) / p::WEIGHTS_SUM * p::NEGATIVE_SCORES_OFFSET
    } else {
        combined_score + p::NEGATIVE_SCORES_OFFSET
    }
}
```

### Analysis

This function does three things:

**1. Safety fallback** (line 84--85): If `WEIGHTS_SUM` is zero (configuration error), clamp to non-negative. This prevents division by zero.

**2. Negative score compression** (lines 86--87): When the combined weighted score is negative (negative signals dominated), the formula is:

```
output = (combined_score + NEGATIVE_WEIGHTS_SUM) / WEIGHTS_SUM * NEGATIVE_SCORES_OFFSET
```

Breaking this down:
- `combined_score + NEGATIVE_WEIGHTS_SUM`: Shifts the score upward. If `NEGATIVE_WEIGHTS_SUM` is the absolute sum of all negative weights, this maps the worst-possible score to approximately zero.
- `/ WEIGHTS_SUM`: Normalizes into a 0--1 range (assuming `WEIGHTS_SUM` is the total span).
- `* NEGATIVE_SCORES_OFFSET`: Scales the compressed negative range to a small region.

**INFERRED**: This means negative scores are compressed into a narrow band near zero. The asymmetry is deliberate -- X does NOT want a single "not interested" click to crater a post's score the same way a report does. Instead, all negative outcomes are mapped into a small range, while positive outcomes can scale without bound (line 89: `combined_score + NEGATIVE_SCORES_OFFSET`).

**3. Positive score offset** (lines 88--89): Positive scores get a flat offset added. This `NEGATIVE_SCORES_OFFSET` value is likely a small positive constant that shifts the entire positive range upward, ensuring that even posts with minimal positive engagement score higher than posts with negative signals.

### What This Means in Practice

- A post that gets 1 report and 100 likes: the negative contribution is compressed, the positive contribution dominates. The post survives.
- A post that gets 10 reports and 0 likes: the negative score is compressed into the narrow negative band. The post is buried but not infinitely penalized.
- The compression prevents adversarial mass-reporting from having unbounded impact -- there's a floor.
- Conversely, positive signals stack linearly without compression -- 100 genuine replies are worth 100x one reply.

**INFERRED**: `NEGATIVE_SCORES_OFFSET` is likely a small value (perhaps 1.0--5.0) that creates a "gap" between the worst possible compressed negative score and the baseline positive score. This gap is the algorithm's built-in margin of safety.

---

## Comparison with 2023 Weights

The 2023 algorithm (twitter/the-algorithm, April 2023) published these HeavyRanker weights:

| 2023 Signal | 2023 Weight | 2026 Equivalent | Change |
|------------|-------------|-----------------|--------|
| `fav` | 0.5 | `FAVORITE_WEIGHT` | Preserved as baseline |
| `retweet` | 1.0 | `RETWEET_WEIGHT` | Likely similar ratio |
| `reply` | 13.5 | `REPLY_WEIGHT` | Preserved but `reply_engaged_by_author` is GONE |
| `reply_engaged_by_author` | 75.0 | **REMOVED** | Eliminated to kill reply-farming |
| `good_profile_click` | 12.0 | `PROFILE_CLICK_WEIGHT` | Likely similar |
| `good_click` | 11.0 | `CLICK_WEIGHT` | Merged into single click signal |
| `good_click_v2` | 10.0 | (merged above) | Merged |
| `video_playback50` | 0.005 | `VQV_WEIGHT` | Upgraded to quality-gated signal |
| `negative_feedback_v2` | -74.0 | Split into 3 signals | Decomposed for precision |
| `report` | -369.0 | `REPORT_WEIGHT` | Likely similar magnitude |
| -- | -- | `SHARE_WEIGHT` | **NEW** |
| -- | -- | `SHARE_VIA_DM_WEIGHT` | **NEW** |
| -- | -- | `SHARE_VIA_COPY_LINK_WEIGHT` | **NEW** |
| -- | -- | `DWELL_WEIGHT` | **NEW** (explicit) |
| -- | -- | `CONT_DWELL_TIME_WEIGHT` | **NEW** |
| -- | -- | `QUOTE_WEIGHT` | **NEW** (was not separate) |
| -- | -- | `QUOTED_CLICK_WEIGHT` | **NEW** |
| -- | -- | `FOLLOW_AUTHOR_WEIGHT` | **NEW** |
| -- | -- | `PHOTO_EXPAND_WEIGHT` | **NEW** |

### Key Structural Changes (2023 to 2026)

**1. `reply_engaged_by_author` was the single most powerful signal in 2023 at 75.0. It is GONE.**
- **EMPIRICAL**: X's Head of Product Nikita Bier stated "Replies don't count anymore" for revenue sharing, targeting reply ring spam farms.
- **INFERRED**: The 75x bonus for author engagement was being exploited at scale. Removing it means plain replies still matter, but the author-reply-back "cheat code" is dead.

**2. Negative feedback decomposed from 1 to 3 signals.**
- 2023 combined block + mute + "show less" into one `-74.0` weight.
- 2026 separates `NOT_INTERESTED_WEIGHT`, `BLOCK_AUTHOR_WEIGHT`, `MUTE_AUTHOR_WEIGHT` -- allowing different penalty magnitudes for "not relevant to me" vs. "this person is harmful."

**3. Share signals went from 0 to 3 dedicated signals.**
- 2023 had no share tracking in the weighted scorer.
- 2026 tracks generic share, DM share, and copy-link share separately. This is the most significant addition and strongly suggests sharing is now a top-tier signal.

**4. Dwell time made explicit.**
- 2023 dwell was implicit in click metrics.
- 2026 has both binary dwell (did they pause) and continuous dwell time (how long). This dual measurement captures both "stopped scrolling" and "read the whole thread."

**5. Quote tweets separated.**
- 2023 did not have a dedicated quote weight.
- 2026 has `QUOTE_WEIGHT` and `QUOTED_CLICK_WEIGHT` (user clicks into the original from a quote tweet).

**6. Follow author is now a scoring signal.**
- 2023 did not include follows in the scorer.
- 2026's `FOLLOW_AUTHOR_WEIGHT` means content that causes people to follow the author gets a direct boost.

---

## Estimated Scoring Formula (Illustrative)

Using favorite = 1.0 as baseline, and normalizing the 2023 ratios where available, a plausible weight vector:

```
score = P(favorite)  *   1.0     // baseline
      + P(reply)     *  20.0     // high-intent conversation
      + P(retweet)   *   3.0     // public amplification
      + P(photo_exp) *   1.5     // passive media interest
      + P(click)     *  10.0     // conversation depth
      + P(prof_click)*  12.0     // discovery intent
      + P(vqv)       *   3.0     // video quality (if eligible)
      + P(share)     *  10.0     // intent to share
      + P(share_dm)  *  25.0     // private recommendation
      + P(share_link)*  20.0     // off-platform sharing
      + P(dwell)     *   8.0     // stopped scrolling
      + P(quote)     *  18.0     // public endorsement + new content
      + P(qt_click)  *   4.0     // secondary engagement
      + dwell_time   *   0.1     // continuous (seconds, not probability)
      + P(follow)    *  30.0     // ultimate discovery signal
      + P(not_int)   * -20.0     // content mismatch
      + P(block)     * -74.0     // author rejection
      + P(mute)      * -40.0     // soft rejection
      + P(report)    * -369.0    // safety violation
```

**These numbers are ESTIMATES.** The true values are in the unpublished `params` module. However, the relative ordering is high-confidence based on the convergence of code structure, 2023 baselines, community testing, and platform economics.

---

## Practical Implications for Content Strategy

Based on the deduced hierarchy:

1. **DM shares and link copies are probably the most underrated signals.** They are brand-new dedicated signals, and the code architecture treats them as first-class citizens alongside replies and retweets.

2. **Replies still matter, but the author-reply-back bonus is dead.** You cannot game the algorithm by replying to your own replies anymore. Plain reply quality matters.

3. **Follow-from-post is the ultimate win.** If your content causes someone to follow you, that post is getting a massive boost. This rewards genuinely novel or valuable content from accounts people haven't seen before.

4. **Dwell time is now double-tracked.** Both "did they stop" and "how long did they stop" feed the scorer. Long-form content that holds attention is explicitly rewarded.

5. **A single report is devastating.** The estimated -369x weight means even a P(report) of 0.001 contributes -0.369 to the score -- equivalent to wiping out the contribution of P(favorite) of 0.37 weighted at 1.0.

6. **Negative signal compression protects against brigading.** The `offset_score()` function compresses all negative scores into a narrow band, preventing mass-reporting from having unbounded effect.

7. **Video content must clear a duration threshold.** The `vqv_weight_eligibility()` function zeroes out the VQV weight for videos shorter than `MIN_VIDEO_DURATION_MS`. Short clips get no video quality boost -- only substantive videos do.

---

## Methodology Notes

- Weight estimates use favorite = 1.0 as the baseline (following 2023 convention where fav = 0.5 was the lowest positive weight).
- Where 2023 weights exist, the 2026 estimate preserves the approximate ratio unless structural changes (like removal of `reply_engaged_by_author`) suggest recalibration.
- New signals (shares, follows, dwell, quote) are estimated from their position in the code (order matters in engineering culture), their action semantics (server-validated vs. client-only), and cross-platform precedent (Instagram's DM-as-top-signal finding).
- Continuous dwell time weight is estimated low because the input is a raw duration, not a 0--1 probability. A weight of 0.1 on 30 seconds = 3.0 contribution, comparable to a reply's contribution.
- All community-cited formulas (e.g., "Likes x 1 + Retweets x 20 + Replies x 13.5") conflate weight with probability x weight. The true formula is `sum(weight_i * P(action_i))`, where P values differ by orders of magnitude across action types.

---

## Sources

### Code (primary)
- `home-mixer/scorers/weighted_scorer.rs` -- the complete scoring function
- `home-mixer/scorers/phoenix_scorer.rs` -- how Phoenix predictions map to PhoenixScores
- `home-mixer/candidate_pipeline/candidate.rs` -- the PhoenixScores struct (all 19 fields)
- `home-mixer/scorers/oon_scorer.rs` -- out-of-network penalty
- `home-mixer/scorers/author_diversity_scorer.rs` -- author repetition decay
- `home-mixer/lib.rs` -- confirms params module excluded

### 2023 Baseline
- [twitter/the-algorithm-ml HeavyRanker README](https://github.com/twitter/the-algorithm-ml/blob/main/projects/home/recap/README.md) -- exact 2023 weights

### Empirical Research
- [Buffer: Does X Premium Really Boost Your Reach? (18M+ posts analyzed)](https://buffer.com/resources/x-premium-review/) -- Premium 10x reach advantage
- [Social Media Today: X Reveals Key Signals for Post Reach](https://www.socialmediatoday.com/news/x-formerly-twitter-open-source-algorithm-ranking-factors/759702/) -- signal hierarchy from X's own disclosure
- [HackerNoon: I Read X's Open-Source Algorithm](https://hackernoon.com/i-read-xs-open-source-algorithm-heres-what-actually-matters-in-2026) -- 2026 code analysis
- [Typefully: X Algorithm Open Source Jan 2026](https://typefully.com/blog/x-algorithm-open-source) -- confirmation weights are redacted
- [PostEverywhere: How the X Algorithm Works](https://posteverywhere.ai/blog/how-the-x-twitter-algorithm-works) -- engagement weight table
- [Gajus: X For You Algorithm Dissected](https://gajus.com/blog/x-for-you-algorithm-disected) -- signal tier categorization
- [Tweet Archivist: Complete Technical Breakdown](https://www.tweetarchivist.com/how-twitter-algorithm-works-2025) -- dwell time and negative signal analysis
- [Radaar: TweepCred, Shadow Hierarchy, and Dwell Time](https://www.radaar.io/resources-121/blog-388/are-you-ready-to-discover-the-hidden-x-algorithm-secrets-behind-tweepcred-shadow-hierarchy-and-dwell-time-in-2025-15361/) -- hidden dwell time mechanics
