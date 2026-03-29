# 11 -- xmaster Improvement Proposals

**Date**: 2026-03-27
**Based on**: Synthesis document (10-synthesis-and-corrections.md) + independent web research + existing docs 01-05.
**Scope**: Changes to the xmaster CLI codebase, skill file, and documentation.

Every proposal below is tagged with its evidence source and prioritised.

---

## Proposal 1: Update Preflight Sentiment Check

**Priority**: HIGH
**Files**: `src/intel/preflight.rs`
**Source**: [PostEverywhere](https://posteverywhere.ai/blog/how-the-x-twitter-algorithm-works), [SocialWick](https://www.socialwick.com/decoding-the-new-x-algorithm-to-stay-visible-in-2026), doc 05 section 7.3

### Problem

The Grok transformer reads post text and predicts P(block), P(mute), P(report). Combative, negative, or outrage-driven content gets reduced visibility even when engagement is high. Our preflight currently checks for engagement bait and link placement but has zero sentiment awareness.

### Proposed Change

Add a basic sentiment/tone check to `analyze()` in `preflight.rs`:

1. Detect combative language patterns: all-caps phrases (>3 consecutive words in caps), excessive exclamation marks (>2), insult patterns ("you're wrong", "idiots", "morons", "stupid"), accusatory framing ("people who X are Y").
2. Detect negative-only framing without constructive elements: complaints without solutions, pure criticism without alternatives.
3. Issue severity: WARNING for mild negativity, CRITICAL for aggressive/combative tone.
4. Score impact: -15 for WARNING, -25 for CRITICAL.

```
Issue code: "combative_tone"
Message: "Aggressive tone detected — X algorithm reduces reach for combative content even with high engagement"
Fix: "Frame critiques constructively — lead with solutions, not complaints"
```

### Why This Matters

The sentiment signal is NOT a separate weight in `weighted_scorer.rs` -- it works through the Grok embedding pipeline influencing P(block)/P(mute)/P(report) predictions. Content that reads as combative will score higher on negative signals regardless of how many likes it gets. This makes it one of the most impactful preflight checks we could add.

### Scope

Keyword-based detection is a reasonable first pass. It does not require ML or API calls. False positives are acceptable since this is a WARNING, not a post-blocker.

---

## Proposal 2: Add Engagement Velocity Timer to Post Output

**Priority**: HIGH
**Files**: `src/commands/post.rs`, `src/commands/thread.rs`
**Source**: [OpenTweet](https://opentweet.io/blog/how-twitter-x-algorithm-works-2026), [SocialWick](https://www.socialwick.com/decoding-the-new-x-algorithm-to-stay-visible-in-2026), [HackerNoon](https://hackernoon.com/i-read-xs-open-source-algorithm-heres-what-actually-matters-in-2026)

### Problem

The first 30-60 minutes after posting is the critical engagement velocity window. The algorithm tests your content on 5-15% of followers during this period and decides whether to expand or suppress distribution. Users do not know this, and xmaster currently provides no post-posting guidance.

### Proposed Change

After `xmaster post` or `xmaster thread` succeeds, add to the output:

```
Posted successfully. ID: 1234567890

Algorithm tip: The next 30-60 minutes determine this post's reach.
  - Engage with replies immediately (replies = 20x a like)
  - Check performance: xmaster metrics 1234567890
```

This is a lightweight addition to the post confirmation output. No new commands needed.

### Why This Matters

Users who understand the velocity window can take action (replying to comments, engaging with their niche) during the critical period. This is the single highest-leverage behaviour change for growth.

---

## Proposal 3: Update `agent-info` Algorithm Data

**Priority**: HIGH
**Files**: `src/commands/agent_info.rs`
**Source**: Synthesis doc sections 3.2, 3.3, 3.5, 3.6

### Problem

The `AlgorithmInfo` struct in `agent_info.rs` has several fields set to zero or missing data that we now have evidence for:

- `time_decay_halflife_minutes: 0` -- should be 360 (6 hours)
- No Premium boost data
- No engagement velocity window information
- No feed composition data

### Proposed Changes

```rust
AlgorithmInfo {
    source: "xai-org/x-algorithm (January 2026, Grok-based transformer). Exact weights unpublished — estimates below from code structure + empirical data.".into(),
    // ... existing weights unchanged ...
    time_decay_halflife_minutes: 360, // ~6 hours (EMPIRICAL: multiple independent sources)
    out_of_network_reply_penalty: 0.0, // Keep as-is -- OON_WEIGHT_FACTOR is multiplicative
    // ... existing fields ...
}
```

Add new fields to `AlgorithmInfo`:

```rust
struct AlgorithmInfo {
    // ... existing fields ...
    premium_boost_in_network: String,   // "4x (EMPIRICAL)"
    premium_boost_out_of_network: String, // "2x (EMPIRICAL)"
    engagement_velocity_window_minutes: String, // "30-60 (critical), 120-240 (expansion)"
    feed_composition: String, // "~50% in-network, ~50% out-of-network (For You)"
}
```

Add new usage hints:

```rust
"The first 30-60 minutes after posting determine algorithmic distribution — engage immediately".into(),
"Premium accounts get ~4x in-network and ~2x out-of-network visibility boost".into(),
"~50% of For You feed is from non-followed accounts — your OON score is your growth engine".into(),
```

### Why This Matters

AI agents (Claude, GPT, etc.) read `agent-info --json` to understand how to optimise. Providing accurate time decay, Premium boost, and velocity data makes agents significantly more effective at advising users.

---

## Proposal 4: Soften the `too_short` Preflight Warning

**Priority**: MEDIUM
**Files**: `src/intel/preflight.rs`
**Source**: [AutoTweet](https://www.autotweet.io/blog/x-algorithm-explained-2026) -- "Short posts of 40-80 chars get 66% higher engagement"

### Problem

Current code:
```rust
if features.char_count < 50 && !features.has_media && !features.has_question {
    // ...
    score -= 15;
}
```

This penalises posts under 50 characters, but empirical data shows 40-80 character posts have the HIGHEST engagement rate. A punchy 45-character statement should not be penalised.

### Proposed Change

Lower the threshold from 50 to 30 characters, and reduce the penalty from -15 to -10:

```rust
if features.char_count < 30 && !features.has_media && !features.has_question {
    issues.push(Issue {
        severity: Severity::Info, // Downgrade from Warning to Info
        code: "very_short".into(),
        message: "Very short tweet without media — may lack context".into(),
        fix: Some("Posts of 40-80 chars have highest engagement rate — this is fine if punchy".into()),
    });
    score -= 10;
}
```

### Why This Matters

The current penalty discourages a format that empirically performs well. Small accounts especially benefit from short, punchy statements that generate immediate reactions.

---

## Proposal 5: Add Dwell Time Estimation to Preflight

**Priority**: MEDIUM
**Files**: `src/intel/preflight.rs`
**Source**: Doc 02 (dwell_time is a continuous signal with its own weight), [OpenTweet](https://opentweet.io/blog/how-twitter-x-algorithm-works-2026)

### Problem

`dwell_time` is the only continuous signal in the scoring formula. It captures how long a user pauses on your post. Our preflight evaluates many factors but does not estimate dwell time potential.

### Proposed Change

Add a `estimated_dwell_seconds` field to `FeatureVector`:

```rust
pub struct FeatureVector {
    // ... existing fields ...
    pub estimated_dwell_seconds: u32,
}
```

Estimation heuristic:
- Base: word_count / 4 (average reading speed of ~250 WPM = ~4 words/sec)
- +2 seconds if has_media (image viewing time)
- +3 seconds if has_question (deliberation time)
- +1 second per line break (scanning time)
- Cap at 30 seconds (diminishing returns)

Add a positive bonus for high estimated dwell:
```rust
if features.estimated_dwell_seconds >= 8 {
    score += 5; // Bonus for posts that hold attention
}
```

And an info-level note for very low dwell:
```rust
if features.estimated_dwell_seconds < 3 && !features.has_media {
    // "Post may scroll past quickly — add depth or media to increase dwell time"
}
```

### Why This Matters

Dwell time is double-tracked in the 2026 algorithm (binary dwell + continuous dwell_time). Making users aware of it encourages content that holds attention, which is rewarded by two separate scoring signals.

---

## Proposal 6: Add `--format long` Flag for Single Long-Form Posts

**Priority**: MEDIUM
**Files**: `src/commands/post.rs`, `src/intel/preflight.rs`
**Source**: [OpenTweet](https://opentweet.io/blog/how-twitter-x-algorithm-works-2026) -- "Single long-form posts more favourably distributed than multi-tweet threads"

### Problem

When a user has content exceeding 280 characters, xmaster's skill file recommends converting to a thread. But new evidence suggests single long-form posts (Premium feature, up to 25,000 chars) may now get better initial distribution than threads.

### Proposed Change

1. When `xmaster analyze` encounters a post >280 characters, instead of recommending a thread, show both options:

```
INFO: Post is 850 characters (requires X Premium for long-form post)
Options:
  - Post as single long-form (better initial distribution, higher dwell time)
  - Convert to thread with 'xmaster thread' (more engagement touchpoints, 3x total engagement)
```

2. Update preflight to give a positive bonus for long-form posts between 500-2000 characters:
```rust
if features.char_count > 500 && features.char_count < 2000 {
    score += 5; // Long-form dwell time bonus
}
```

3. Update the skill file to reflect both strategies.

### Why This Matters

Users should not default to threads for all long content. The algorithm may now favour single long-form posts for distribution, while threads still win on total engagement. Presenting both options is more accurate.

---

## Proposal 7: Add Premium Status Awareness

**Priority**: MEDIUM
**Files**: `src/config.rs`, `src/intel/preflight.rs`, `src/commands/agent_info.rs`
**Source**: Synthesis doc section 3.5, [Buffer](https://buffer.com/resources/x-premium-review/)

### Problem

Premium accounts get 4x in-network and 2x out-of-network visibility boosts, link suppression mitigation, and long-form posting. Our preflight and agent-info do not know whether the user has Premium, leading to advice that may be wrong for their account type.

### Proposed Change

1. Add a `premium` boolean to the config:
```rust
pub struct XmasterConfig {
    // ... existing fields ...
    pub premium: bool, // X Premium subscriber
}
```

2. Adjust preflight scoring based on Premium status:
   - If `premium == true`, downgrade `link_in_body` from CRITICAL (-30) to WARNING (-15) with message: "Links still reduce reach, but Premium mitigates the worst suppression."
   - If `premium == false`, add an info note: "Consider X Premium for ~4x visibility boost."

3. Add to agent-info a `premium_status` field.

### Why This Matters

The gap between Premium and free accounts has widened significantly in Q1 2026. Giving different advice based on account type makes xmaster more accurate for all users.

---

## Proposal 8: Update Skill File with New Intelligence

**Priority**: HIGH
**Files**: `~/.claude/skills/xmaster/SKILL.md`
**Source**: All synthesis findings

### Proposed Changes to SKILL.md

#### 8a. Add velocity window to posting workflow

After step 5 ("Post"), add:

```
6. VELOCITY WINDOW: The next 30-60 minutes are critical.
   - Reply to every comment that comes in
   - Engage with 5-10 posts in your niche (builds engagement history)
   - Check early performance: xmaster metrics <id>
   - If no engagement by 30 min, the post likely won't expand
```

#### 8b. Update thread vs. long-form guidance

Replace the current "Threads are the #1 growth driver" with:

```
### LONG CONTENT: THREAD vs. LONG-FORM

Two options for content over 280 characters:

1. THREAD (via xmaster thread):
   - More total engagement (3x average vs. single tweets)
   - Multiple engagement touchpoints
   - Each tweet needs its own hook
   - Best for: step-by-step content, lists, narratives

2. SINGLE LONG-FORM POST (requires Premium):
   - Better initial distribution (algorithmic preference for single posts)
   - Higher dwell time signal (continuous reading)
   - Simpler for readers
   - Best for: essays, analysis, detailed takes

Recommend threads for <1000 chars, long-form for >1000 chars.
```

#### 8c. Add sentiment awareness to posting workflow

In the posting workflow, after analyze step:

```
   - If tone is combative or negative: suggest constructive reframing
   - The Grok transformer penalises negative tone via higher P(block)/P(mute) predictions
   - Frame critiques with solutions, not just complaints
```

#### 8d. Add Premium context

Add to hints section:

```
- Premium accounts get ~4x in-network and ~2x out-of-network visibility boost
- Premium accounts have reduced (not zero) link suppression
- Check if user has Premium before advising on links and long-form posts
```

---

## Proposal 9: New Command -- `xmaster velocity`

**Priority**: LOW
**Files**: New command file
**Source**: Synthesis section 3.4

### Problem

Users have no way to check the engagement velocity of a recent post during the critical 30-60 minute window.

### Proposed Command

```
xmaster velocity <post-id>
```

Output:
```
Post: 1234567890 (posted 23 minutes ago)
Velocity window: 7 minutes remaining (30 min critical window)

Current engagement:
  Likes: 4       Replies: 2       Reposts: 1       Views: 142
  Reply rate: 1.4%  (good — above 1% triggers expansion)

Suggestion: Reply to your 2 comments to boost the conversation signal
```

This would call `xmaster metrics <id>` internally, compute time-since-posting, and overlay velocity context.

### Why This Might NOT Be Worth Building

This is essentially `xmaster metrics` with a time overlay. The delta is small. The velocity window information could just as well be added to the post output (Proposal 2) and the skill file (Proposal 8a). Marking as LOW priority because the information can be delivered through existing mechanisms.

---

## Proposal 10: Add Author Diversity Warning to `xmaster post`

**Priority**: MEDIUM
**Files**: `src/commands/post.rs`
**Source**: Doc 04 (AuthorDiversityScorer analysis), [SocialBee](https://socialbee.com/blog/twitter-algorithm/)

### Problem

The `AuthorDiversityScorer` applies exponential decay to successive posts from the same author in a user's feed session. Posting too frequently cannibalizes your own reach. Our existing `xmaster suggest next-post` already checks for this, but users often skip that step and post directly.

### Proposed Change

In `xmaster post`, before posting, optionally check the user's recent post timestamps:

```
$ xmaster post "my new tweet"

WARNING: You posted 45 minutes ago (recommended spacing: 2+ hours)
  AuthorDiversityScorer applies exponential decay to successive posts.
  Post anyway? [y/N]
```

This can be implemented by checking the user's timeline for the most recent post timestamp and comparing against a 2-hour threshold. Use `--force` to skip the check.

### Why This Matters

This is one of the most common growth mistakes: posting 5 tweets in an hour and wondering why none performed. The AuthorDiversityScorer is a code-confirmed mechanism, not speculation.

---

## Proposal 11: Update Hashtag Messaging in Preflight

**Priority**: LOW
**Files**: `src/intel/preflight.rs`
**Source**: [PostOwl](https://postowl.io/blog/twitter-hashtags-x-algorithm-2025/), [ContentStudio](https://contentstudio.io/blog/twitter-hashtags)

### Problem

Current preflight warns at >2 hashtags but does not provide specific data in the message.

### Proposed Change

Update the warning message:

```rust
message: format!(
    "{} hashtags — posts with 1-2 hashtags see 21% higher engagement; 5+ hashtags see 40% penalty",
    features.hashtag_count
),
```

Minor change. Low priority because the existing warning is already directionally correct.

---

## Proposal 12: Add Feed Composition Data to Agent-Info

**Priority**: LOW
**Files**: `src/commands/agent_info.rs`
**Source**: [OpenTweet](https://opentweet.io/blog/how-twitter-x-algorithm-works-2026)

### Proposed Change

Add to `AlgorithmInfo`:

```rust
feed_composition: "~50% in-network (Thunder), ~50% out-of-network (Phoenix Retrieval)".into(),
```

And add a usage hint:

```rust
"Half of every For You feed is out-of-network — your content reaches non-followers 50% of the time if it scores well".into(),
```

This helps AI agents understand the growth opportunity from out-of-network distribution.

---

## Priority Summary

| # | Proposal | Priority | Complexity | Impact |
|---|----------|----------|------------|--------|
| 1 | Sentiment check in preflight | HIGH | Medium | High -- prevents invisible reach killing |
| 2 | Velocity timer in post output | HIGH | Low | High -- drives behaviour change |
| 3 | Update agent-info algorithm data | HIGH | Low | High -- makes AI agents smarter |
| 8 | Update skill file | HIGH | Low | High -- improves all AI-driven workflows |
| 4 | Soften too_short warning | MEDIUM | Low | Medium -- stops penalising effective format |
| 5 | Dwell time estimation | MEDIUM | Medium | Medium -- new scoring dimension |
| 6 | Long-form vs. thread guidance | MEDIUM | Low | Medium -- better content format advice |
| 7 | Premium status awareness | MEDIUM | Medium | Medium -- personalised advice |
| 10 | Author diversity warning | MEDIUM | Medium | Medium -- prevents common mistake |
| 9 | Velocity command | LOW | Medium | Low -- covered by other proposals |
| 11 | Hashtag message update | LOW | Low | Low -- marginal improvement |
| 12 | Feed composition in agent-info | LOW | Low | Low -- informational |

### Recommended Implementation Order

1. **Proposals 2, 3, 8** (LOW complexity, HIGH impact) -- can be done in a single PR
2. **Proposals 1, 4** (Preflight improvements) -- second PR
3. **Proposals 5, 6, 7, 10** (MEDIUM complexity) -- third PR
4. **Proposals 9, 11, 12** (LOW priority) -- backlog

---

## Sources

All sources are documented in `10-synthesis-and-corrections.md`. Key evidence backing each proposal:

- Proposals 1, 8c: PostEverywhere sentiment analysis + doc 05 negative signals
- Proposals 2, 8a, 9: OpenTweet/SocialWick/HackerNoon engagement velocity data
- Proposal 3: Multi-source convergence on time decay, Premium boost, feed composition
- Proposal 4: AutoTweet 40-80 char engagement data
- Proposal 5: Doc 02 dwell_time analysis + code confirmation
- Proposals 6, 8b: OpenTweet long-form vs. thread algorithmic preference
- Proposal 7: Buffer 18M+ post Premium analysis
- Proposal 10: Doc 04 AuthorDiversityScorer code analysis
- Proposal 11: PostOwl/ContentStudio hashtag empirical data
- Proposal 12: OpenTweet feed composition reporting
