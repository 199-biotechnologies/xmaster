# Deep Dive: How X's Algorithm Treats Small, Dormant-then-Reactivated Accounts

**Date**: 2026-03-27
**Sources**: [twitter/the-algorithm](https://github.com/twitter/the-algorithm) open-source release (March 2023), subsequent community analysis, and confirmed post-release parameter disclosures.

> **Target profile**: ~100 followers, following ~300, dormant for years, now becoming active again. Free account.

---

## Table of Contents

1. [TweepCred: The PageRank Reputation System](#1-tweepcred-the-pagerank-reputation-system)
2. [Concrete Scenario Calculation](#2-concrete-scenario-calculation)
3. [The Cold Start Problem](#3-the-cold-start-problem)
4. [Discovery Mechanisms That Actually Work](#4-discovery-mechanisms-that-actually-work)
5. [Negative Signals and Penalties](#5-negative-signals-and-penalties)
6. [Premium vs Free: The Structural Disadvantage](#6-premium-vs-free-the-structural-disadvantage)
7. [The Engagement Scoring System](#7-the-engagement-scoring-system)
8. [Strategic Implications](#8-strategic-implications)
9. [Source Code References](#9-source-code-references)

---

## 1. TweepCred: The PageRank Reputation System

**Source**: `src/scala/com/twitter/graph/batch/job/tweepcred/`

TweepCred is a batch-computed reputation score (0--100) assigned to every X account. It runs daily as a Hadoop MapReduce job, executing weighted PageRank over the social graph until convergence. The system has been running since October 2, 2015.

### 1.1 Architecture (7 files)

| File | Role |
|------|------|
| `PreparePageRankData.scala` | Constructs the user interaction graph from Flock (follow) edges and Real Graph (interaction) edges. Filters self-loops. Initialises user mass. |
| `UserMass.scala` | Computes initial "mass" (0--100) for each user based on account characteristics. |
| `WeightedPageRank.scala` | Iterative PageRank computation. Damping factor (ALPHA) = 0.1 (probability of random jump). Converges when totalDiff < 0.001 or max iterations reached. |
| `Reputation.scala` | Post-PageRank adjustment. Applies follower-ratio penalty. Maps raw PageRank to 0--100 via logarithmic scaling. |
| `ExtractTweepcred.scala` | Final pipeline stage. Filters deactivated accounts, applies `adjustReputationsPostCalculation`, normalises, and outputs final score. |
| `TweepcredBatchJob.scala` | Orchestrator. Runs daily (24h interval, 36h timeout). Outputs to `/user/cassowary/tweepcred/`. |
| `README` | Documents the system as "PageRank-based influence scoring." |

### 1.2 UserMass Calculation (Initial Seed)

From `UserMass.scala`, the initial mass before PageRank iteration:

```
mass = base_score(device, age, restriction_status) * scale_to_100
```

**Contributing factors**:

| Factor | Contribution | Details |
|--------|-------------|---------|
| Suspended | mass = 0 | Immediate zero |
| Verified | mass = 100 | Fixed maximum, bypasses all other calculation |
| Device validity | +0.5 additive | If messaging devices (mobile app) present |
| Account age | `min(1.0, log(1 + age/30))` | Logarithmic, normalised, caps at 1.0 after ~30 days |
| Restricted | 0.1x multiplier | If account has been restricted |

**Key insight**: Account age normalisation caps at 1.0 after roughly 30 days. A 5-year-old account and a 2-month-old account get the same age factor. Age alone is not an advantage beyond the first month.

### 1.3 The Follower-Ratio Penalty (Critical for Small Accounts)

From `Reputation.scala`, the `adjustReputationsPostCalculation` method:

```
ratio = (1.0 + numFollowings) / (1.0 + numFollowers)

if (numFollowings > threshold AND ratio > 0.6):
    divFactor = exp(k * (ratio - 0.6) * log(log(numFollowings)))
    divFactor = clamp(divFactor, min=1.0, max=50.0)
    adjusted_mass = mass / divFactor
```

There are **two versions** of this penalty in the codebase:

| Parameter | UserMass.scala (pre-PageRank) | Reputation.scala (post-PageRank) |
|-----------|------------------------------|----------------------------------|
| Following threshold | 500 | 2,500 |
| Ratio threshold | 0.6 | 0.6 |
| Exponential factor (k) | 5.0 | 3.0 |
| Max penalty cap | None specified | 50x divisor |
| Min divisor | None specified | 1.0 |

The penalty in `UserMass.scala` uses a simpler formula:

```
mass / exp(5.0 * (ratio - 0.6))   [when followings > 500 AND ratio > 0.6]
```

The penalty in `Reputation.scala` (post-PageRank) uses:

```
mass / exp(3.0 * (ratio - 0.6) * log(log(numFollowings)))
```

Both penalise accounts where following/followers exceeds 0.6.

### 1.4 Logarithmic Score Mapping

From `Reputation.scala`, the `scaledReputation` method converts raw PageRank to the 0--100 byte scale:

```
scaledScore = 130.0 + 5.21 * ln(rawPageRank)
```

This maps:
- Maximum PageRank -> approximately 95
- Minimum PageRank -> approximately 15
- The mapping is logarithmic, meaning small differences at the bottom of the raw PageRank distribution produce negligible score differences.

### 1.5 The Critical Threshold: Score 65

**This is the single most important number for small accounts.**

If your TweepCred score falls below 65 (on the 0--100 scale), the algorithm **only considers 3 of your tweets for distribution**. Above 65, all tweets are eligible.

This means:
- Below 65: Even if you post 10 excellent tweets per day, only 3 enter the ranking pipeline
- Above 65: Every tweet gets a chance
- Threads are excluded entirely if score is under 65

### 1.6 Weighted PageRank Convergence

From `WeightedPageRank.scala`:

```scala
// Damping factor (probability of random jump)
ALPHA = args.getOrElse("jumpprob", "0.1").toDouble  // default 0.1

// Convergence
if (CURITERATION < MAXITERATIONS - 1 && totalDiff > THRESHOLD)  // THRESHOLD = 0.001

// Rank propagation (weighted mode)
pagerankNext(N_i) = SUM_j[ inputPagerank(N_j) * w(N_j, N_i) / totalWeight(N_j) ]
```

Edge weights come from two sources:
1. **Flock edges** (follows): Default weight 1.0
2. **Real Graph edges** (interactions): Overwrite Flock weights when present; `max(weight)` used when both exist

**For dormant accounts**: With no recent interactions, Real Graph edges have decayed to near-zero or are absent entirely. The account's nodes connect to the graph only through stale follow edges at weight 1.0, meaning PageRank flows through but weakly.

---

## 2. Concrete Scenario Calculation

### Profile: 100 followers, following 300, 5 years old, free account, dormant

#### Step 1: UserMass (Initial Seed)

```
Suspended?  No
Verified?   No
Device?     Assuming mobile app present -> +0.5

Age factor: min(1.0, log(1 + 1825/30)) = min(1.0, log(61.83)) = min(1.0, 4.12) = 1.0
  (capped -- 5 years gives same as 2 months)

Base score (before ratio penalty): ~(0.5 + 1.0) * scaling
  Approximate base: ~30-40 out of 100 (before graph effects)

Ratio check (UserMass.scala penalty):
  followings = 300 (< 500 threshold)
  -> Pre-PageRank ratio penalty does NOT apply (threshold is 500)

Initial mass: ~30-40 (estimated)
```

#### Step 2: PageRank Iteration

With 100 followers and dormant status:
- Inbound edges: 100 follow edges at weight 1.0 (no Real Graph boost -- no recent interactions)
- Outbound edges: 300 follow edges at weight 1.0
- Your followers likely have modest PageRank themselves (small account begets small-account followers)
- PageRank distributes proportionally: each of your 100 followers sends you a fraction of their rank divided by their total outbound edges

**Estimated raw PageRank**: Very low. In a graph of hundreds of millions of nodes, 100 inbound edges with no interaction weight puts you deep in the long tail.

#### Step 3: Post-PageRank Adjustment (Reputation.scala)

```
ratio = (1 + 300) / (1 + 100) = 301/101 = 2.98

followings = 300 (< 2,500 threshold for post-PageRank penalty)
-> Post-PageRank ratio penalty does NOT apply
```

**Good news**: With only 300 followings, neither the pre-PageRank (threshold 500) nor post-PageRank (threshold 2,500) ratio penalty triggers. The 0.6 ratio is bad (2.98 >> 0.6), but the following count is below both thresholds.

#### Step 4: Logarithmic Mapping

```
scaledScore = 130.0 + 5.21 * ln(rawPageRank)

For very low rawPageRank (e.g., 1e-10):
  130.0 + 5.21 * ln(1e-10) = 130.0 + 5.21 * (-23.03) = 130.0 - 119.97 = ~10

For moderate-low rawPageRank (e.g., 1e-7):
  130.0 + 5.21 * ln(1e-7) = 130.0 + 5.21 * (-16.12) = 130.0 - 83.97 = ~46
```

#### Estimated Final TweepCred Score: **15--35**

This is **well below the critical threshold of 65**.

#### What This Means Concretely

| Metric | Value |
|--------|-------|
| Estimated TweepCred | 15--35 |
| Tweets considered for distribution | **3 maximum** |
| Thread eligibility | **No** |
| Out-of-network discovery | Extremely unlikely |
| In-network feed priority | Very low |
| Premium accounts with same content | Would score 4x higher |

#### What Would It Take to Reach 65?

Working backwards from the logarithmic formula:

```
65 = 130.0 + 5.21 * ln(rawPageRank)
ln(rawPageRank) = (65 - 130) / 5.21 = -12.48
rawPageRank = e^(-12.48) = 3.8e-6
```

vs.

```
35 = 130.0 + 5.21 * ln(rawPageRank)
ln(rawPageRank) = (35 - 130) / 5.21 = -18.23
rawPageRank = e^(-18.23) = 1.2e-8
```

You need approximately a **300x increase in raw PageRank** to go from a score of 35 to 65. This requires significantly more inbound edges with strong interaction weights -- meaning followers who actively engage with your content, who themselves have decent PageRank.

---

## 3. The Cold Start Problem

A dormant account reactivating faces compounding cold-start penalties across multiple systems simultaneously.

### 3.1 Real Graph (Interaction Graph)

**Source**: `src/scala/com/twitter/interaction_graph/`

The Real Graph predicts the likelihood of one user interacting with another using a gradient-boosted tree classifier. It tracks:

- Public engagements: favourites, retweets, follows
- Private engagements: profile views, tweet clicks
- Address book data

Edge weights are stored as **decayed sums** of interactions between user pairs.

**For a dormant account**:
- All interaction edges have decayed to near-zero or zero
- Edge weight formula uses exponential time decay: `weight * decay^(days_since_interaction)`
- After years of dormancy, every edge weight is effectively 0
- Result: Your tweets will **not surface** in followers' algorithmically-ranked feeds because the Real Graph predicts zero interaction probability

**Recovery path**: When followers start interacting with your new tweets, edge weights rebuild. But this is a chicken-and-egg problem -- they cannot interact with tweets they never see.

### 3.2 SimClusters (Community Embedding)

**Source**: `src/scala/com/twitter/simclusters_v2/`

SimClusters maintains 145,000 communities, updated every three weeks. The system has two critical components:

**Known-For Matrix**: Identifies which communities producers belong to. Covers only the **top 20 million producers**. A 100-follower dormant account is almost certainly **not** in the Known-For set.

**InterestedIn Matrix**: Derived by multiplying the follow graph by the Known-For matrix: `U = A * V`. If you are not in V (Known-For), you cannot anchor any community, and your content lacks a community embedding at creation time.

**Tweet embedding mechanism**: "Tweet embeddings are updated each time the tweet is favourited. Specifically, the InterestedIn vector of each user who fav-ed the tweet is added to the tweet vector."

This is the key escape hatch: **engagement-driven, not follower-driven**. Even if you are not in Known-For, if users from a specific community favourite your tweet, it gets embedded into that community and can surface for other community members.

### 3.3 The Social Proof Gate

Out-of-network tweets require **social proof** -- engagement from second-degree connections. For a small account:

- Your followers are few (100) and likely have small networks themselves
- Second-degree reach is limited: 100 followers * their avg followers
- Without social proof, your tweets cannot enter the out-of-network candidate pool
- Out-of-network tweets also receive a **0.75x scaling penalty** (`OONTweetScalingScorer.scala`: `ScaleFactor = 0.75`)

### 3.4 Candidate Sourcing Bottleneck

The For You timeline sources approximately **1,500 candidates** from **500 million daily tweets**. Split:
- ~50% in-network (from people you follow)
- ~50% out-of-network (from SimClusters, GraphJet, social proof)

For your tweets to be *someone else's* out-of-network candidate, they need to pass through SimClusters embedding or GraphJet recommendation -- both of which are cold for a dormant small account.

---

## 4. Discovery Mechanisms That Actually Work

Despite the structural disadvantages, the algorithm does have pathways for small accounts to gain traction.

### 4.1 SimClusters Tweet Embedding (Engagement-Driven)

**This is the primary escape route.**

When community members favourite your tweet, their InterestedIn vectors are added to your tweet's embedding. This means:

1. You post a tweet relevant to a topic community (e.g., "Rust programming")
2. Even 2--3 favourites from users who are known members of that community will embed your tweet
3. The tweet then surfaces as an out-of-network candidate for other community members
4. This is **engagement-driven, not follower-driven** -- your follower count is irrelevant

**Limitation**: Requires those initial 2--3 community favourites, which depends on in-network reach (back to the cold start).

### 4.2 GraphJet "Circle of Trust"

**Source**: `src/java/com/twitter/recos/graph_common/`

GraphJet is a real-time bipartite graph (users <-> tweets) that powers approximately **15% of For You tweets**. It uses:

- Dynamic, in-memory graph with rolling temporal window
- Personalized PageRank from a **seed set** called the "circle of trust"
- Specifically designed for cold-start users with no recent engagement history

**How it helps**: For users who *view* your content (even without engaging), GraphJet's real-time graph can propagate your tweets to similar users. The circle of trust mechanism uses a seed of known-good users to bootstrap recommendations.

### 4.3 EarlyBird In-Network Amplification

**Source**: `src/python/twitter/deepbird/projects/timelines/scripts/models/earlybird/`

The EarlyBird light ranker is a logistic regression model for initial tweet scoring. Two variants:

- **recap_earlybird**: In-network ranking (tweets from people you follow)
- **rectweet_earlybird**: Out-of-network ranking

Features include:
- `tweepcred`: Author credibility (your TweepCred score directly feeds into this)
- Engagement counts: likes, replies, retweets (real-time via Signal Ingester)
- Content quality: text entropy, offensiveness, readability
- URL presence, quote status, retweet status

**The cascade effect**: If your followers engage with your tweet, EarlyBird pushes it higher in *their* followers' feeds. Each level of amplification is a multiplier on reach. But the initial in-network engagement must come first.

### 4.4 The 30-Minute Window

The first 30 minutes after posting represent the critical algorithmic momentum window:

- Early engagement signals are weighted most heavily
- The algorithm uses early velocity to predict total engagement
- Tweets that get traction in the first 30 minutes receive amplified distribution
- Tweets that get zero engagement in 30 minutes are effectively dead

**For a small dormant account**: With ~100 followers, many of whom may also be dormant or have decayed Real Graph edges, the probability of engagement in the first 30 minutes is very low.

### 4.5 Tweet Relevancy Half-Life

```
relevancy_decay = 0.5^(age_minutes / 360)
decay_rate = 0.003
minimum_floor = 0.6
```

Tweets lose 50% relevancy every 6 hours (360 minutes). After 24 hours, a tweet is at roughly 6% of its initial relevancy. Combined with the low initial score of a small account, the effective reach window is extremely narrow.

---

## 5. Negative Signals and Penalties

### 5.1 Engagement Signal Weights

From the heavy ranker (MaskNet, ~48M parameters):

| Signal | Weight | Relative to Like |
|--------|--------|-----------------|
| Like (favourite) | 0.5 | 1x baseline |
| Retweet | 1.0 | 2x |
| Click + stay 2+ min | 11.0 | 22x |
| Profile visit + engagement | 12.0 | 24x |
| Reply | 27.0 | 54x |
| Reply + author responds | 75.0 | 150x |
| **Block/mute/"show less"** | **-74.0** | **-148x** |
| **Report** | **-369.0** | **-738x** |

**Implication**: A single report (-369) requires **738 likes** to offset. A single block/mute (-74) requires **148 likes**. For an account with 100 followers, even one negative signal can be catastrophic.

### 5.2 Mass Unfollow Penalty

If many users unfollow in a short period:
- **Duration**: 3-month reduced distribution (one of the longest standard shadowban periods)
- **Mechanism**: Algorithm interprets sudden follower loss as content quality decline
- **Recovery**: Requires sustained positive engagement over months

### 5.3 External Link Penalty

- Free accounts posting links: **Near-zero median engagement** (as of March 2025/2026)
- Premium accounts with links: ~0.25--0.3% engagement (reduced but viable)
- Platform rationale: X wants to retain users on-platform for ad revenue

### 5.4 Hashtag Penalty

- 3+ hashtags: ~40% reach reduction
- Optimal: 1--2 relevant hashtags maximum

### 5.5 Follower-Ratio Penalty (Detailed)

For the target profile (100 followers, 300 following):

```
ratio = 301/101 = 2.98

UserMass.scala: threshold = 500 followings -> NO PENALTY (300 < 500)
Reputation.scala: threshold = 2,500 followings -> NO PENALTY (300 < 2,500)
```

**However**, if the account were to grow following to >500 while keeping followers at 100:

```
ratio = 501/101 = 4.96
penalty = exp(5.0 * (4.96 - 0.6)) = exp(21.8) = 2.95 billion

Mass is divided by this enormous number -> effectively zero
```

The exponential penalty is extreme. At 500+ followings with only 100 followers, TweepCred is destroyed.

### 5.6 FeedbackFatigue Scorer

From `FeedbackFatigueScorer.scala`:

- "See Fewer" actions decay over a 140-day window
- Score multiplier range: 0.2x to 1.0x (can reduce a tweet to 20% of its score)
- Multiple negative signals compound multiplicatively:

```
Final Score = Original * Author_multiplier * Liker_multiplier * Follower_multiplier * Retweeter_multiplier
```

---

## 6. Premium vs Free: The Structural Disadvantage

### 6.1 Blue Verified Multipliers

From `HomeGlobalParams.scala`:

```scala
BlueVerifiedAuthorInNetworkMultiplierParam   = 4.0  // default
BlueVerifiedAuthorOutOfNetworkMultiplierParam = 2.0  // default
```

| Context | Free Account | Premium Account | Multiplier |
|---------|-------------|-----------------|------------|
| In-network | 1.0x | 4.0x | 4x advantage |
| Out-of-network | 0.75x (OON penalty) | 2.0x * 0.75x = 1.5x | 2x advantage (post-penalty) |
| Combined reach | ~1x | ~4--10x | Massive |

### 6.2 TweepCred Premium Bonus

Premium subscribers receive a +4 to +16 point boost to their TweepCred score. For a small account scoring 30, this could push it to 34--46 -- still below 65, but closing the gap.

### 6.3 External Links

| Account Type | Link Post Engagement | Status |
|-------------|---------------------|--------|
| Free | Near-zero median | Essentially suppressed |
| Premium | 0.25--0.3% | Reduced but functional |

### 6.4 Total Disadvantage Calculation

For the target profile (TweepCred ~30, free):

```
Free account effective reach:
  TweepCred 30 -> only 3 tweets considered
  No OON multiplier boost
  Link posts suppressed
  No TweepCred bonus

Premium account with same followers:
  TweepCred 30 + 16 bonus = 46 (still below 65, but closer)
  4x in-network boost
  2x out-of-network boost
  Links functional

Effective reach ratio: Premium is roughly 4-10x higher
```

Even with Premium, reaching 65 TweepCred requires organic PageRank growth through real engagement networks. Premium helps but does not solve the fundamental cold-start problem.

---

## 7. The Engagement Scoring System

### 7.1 Heavy Ranker Pipeline

The full ranking pipeline for the For You feed:

```
500M daily tweets
    |
    v
Candidate Sourcing (~1,500 selected)
    |-- 50% in-network (EarlyBird light ranker)
    |-- 50% out-of-network (SimClusters, GraphJet, Social Graph)
    |
    v
Light Ranker (logistic regression)
    |-- Features: tweepcred, engagement counts, content quality
    |-- Filters: blocked users, safety, policy violations
    |
    v
Heavy Ranker (MaskNet, ~48M params, neural network)
    |-- Predicts: P(favourite), P(reply), P(retweet), P(report)
    |-- Combined score: 0.5*P(fav) + 27*P(reply) + 1*P(RT) - 369*P(report)
    |
    v
Post-Ranking Adjustments
    |-- OON scaling: 0.75x
    |-- Blue boost: 4x in-network, 2x OON
    |-- Reply scaling: 0.75x
    |-- Author diversity decay: 0.5x
    |-- FeedbackFatigue: 0.2x--1.0x
    |
    v
Final Timeline (~50 tweets served)
```

### 7.2 Key Parameters from ScoredTweetsParam.scala

| Parameter | Default | Range |
|-----------|---------|-------|
| OutOfNetworkScaleFactor | 0.75 | 0.0--100.0 |
| ReplyScaleFactor | 0.75 | 0.0--100.0 |
| AuthorDiversityDecayFactor | 0.5 | 0.0--1.0 |
| CreatorInNetworkMultiplier | 1.0 | 0.0--100.0 |
| CreatorOutOfNetworkMultiplier | 1.0 | 0.0--100.0 |
| RequestRankDecayFactorParam | 0.95 | -- |

### 7.3 What "Author Diversity Decay" Means for Prolific Posters

The 0.5 decay factor means each successive tweet from the same author in a feed gets 50% of the previous one's score. For a small account, this compounds with TweepCred limitations:

```
Tweet 1: score * 1.0
Tweet 2: score * 0.5
Tweet 3: score * 0.25

If score is already low from TweepCred, tweet 3 is invisible.
AND if TweepCred < 65, only 3 tweets are considered anyway.
```

---

## 8. Strategic Implications

### 8.1 The Vicious Cycle for Small Dormant Accounts

```
Low TweepCred (15-35)
  -> Only 3 tweets enter pipeline
  -> Low Real Graph weights (no recent interactions)
  -> Tweets don't surface in followers' feeds
  -> No engagement
  -> No SimClusters embedding
  -> No out-of-network discovery
  -> TweepCred stays low
  -> Repeat
```

### 8.2 Breaking the Cycle: Concrete Tactics

**Phase 1: Rebuild Real Graph Edges (Week 1--4)**

1. **Reply to accounts you follow** -- replies carry 27x weight, replies-with-response carry 75x
2. **Target active followers** -- engage with their content to rebuild bilateral edge weights
3. **Avoid external links entirely** -- near-zero engagement for free accounts
4. **Post 2--3 times daily, 30--60 min apart** -- maximise the 3-tweet consideration window
5. **Use 1 hashtag maximum** -- avoid the 3+ hashtag penalty

**Phase 2: Earn SimClusters Embedding (Week 4--8)**

1. **Post niche content** -- you want favourites from users *within a specific SimClusters community*
2. **Quote tweet community members** -- creates a directed interaction edge
3. **Time posts for when your niche is active** -- maximise 30-minute window engagement
4. **Avoid mass-following** -- keep following count under 500 to avoid the UserMass penalty

**Phase 3: Cross the 65 Threshold (Month 2--6)**

1. PageRank grows logarithmically -- this takes time
2. Each engaged follower who has their own engaged followers amplifies your rank
3. Focus on attracting followers who are *themselves* well-connected
4. The 300x raw PageRank increase needed (score 35 -> 65) likely requires growing to 500--1000 engaged followers

**Phase 4: Consider Premium**

- The +4 to +16 TweepCred bonus helps but does not solve the fundamental problem
- The 4x in-network boost is significant once you have in-network reach to amplify
- Links become functional with Premium
- ROI is poor until you have ~200+ engaged followers to multiply

### 8.3 Account Hygiene: Ratio Management

Current state: 100 followers / 300 following (ratio = 2.98).

The ratio does not trigger algorithmic penalties yet (both thresholds require >500 followings), but it signals low authority. Recommendations:

1. **Do NOT mass-unfollow** -- triggers 3-month shadowban
2. Unfollow gradually: 50--100 per day maximum
3. Target: Get ratio under 1.0 (more followers than following)
4. Absolute ceiling: Never exceed 500 following until followers exceed 833 (to stay under 0.6 ratio)

### 8.4 The Old Account Advantage

Despite all disadvantages, a 5-year-old account has one structural advantage over creating a new one:

- New accounts face stricter anti-spam rules
- Account age factor caps at 1.0 after 30 days, so old = new in that regard
- BUT: Old accounts retain dormant follow edges that can be reactivated
- Existing followers, even if dormant, are *potential* Real Graph edges once they interact
- New accounts start with zero edges entirely

**Verdict**: Reactivating the old account is strictly better than starting fresh.

---

## 9. Source Code References

### Primary Repository
- [twitter/the-algorithm](https://github.com/twitter/the-algorithm) -- Open-sourced March 31, 2023

### TweepCred System
- [`UserMass.scala`](https://github.com/twitter/the-algorithm/blob/main/src/scala/com/twitter/graph/batch/job/tweepcred/UserMass.scala) -- Initial mass calculation with device, age, ratio factors
- [`WeightedPageRank.scala`](https://github.com/twitter/the-algorithm/blob/main/src/scala/com/twitter/graph/batch/job/tweepcred/WeightedPageRank.scala) -- Iterative PageRank with damping=0.1, convergence threshold=0.001
- [`Reputation.scala`](https://github.com/twitter/the-algorithm/blob/main/src/scala/com/twitter/graph/batch/job/tweepcred/Reputation.scala) -- Post-PageRank adjustment, logarithmic 0--100 mapping, ratio penalty
- [`ExtractTweepcred.scala`](https://github.com/twitter/the-algorithm/blob/main/src/scala/com/twitter/graph/batch/job/tweepcred/ExtractTweepcred.scala) -- Final score extraction pipeline
- [`PreparePageRankData.scala`](https://github.com/twitter/the-algorithm/blob/main/src/scala/com/twitter/graph/batch/job/tweepcred/PreparePageRankData.scala) -- Graph construction from Flock + Real Graph edges
- [`TweepcredBatchJob.scala`](https://github.com/twitter/the-algorithm/blob/main/src/scala/com/twitter/graph/batch/job/tweepcred/TweepcredBatchJob.scala) -- Daily batch orchestrator (running since Oct 2015)

### Ranking System
- [`OONTweetScalingScorer.scala`](https://github.com/twitter/the-algorithm/blob/main/home-mixer/server/src/main/scala/com/twitter/home_mixer/functional_component/scorer/OONTweetScalingScorer.scala) -- Out-of-network 0.75x scale factor
- [`FeedbackFatigueScorer.scala`](https://github.com/twitter/the-algorithm/blob/main/home-mixer/server/src/main/scala/com/twitter/home_mixer/functional_component/scorer/FeedbackFatigueScorer.scala) -- 140-day negative feedback decay, 0.2--1.0x multiplier
- [`ScoredTweetsParam.scala`](https://github.com/twitter/the-algorithm/blob/main/home-mixer/server/src/main/scala/com/twitter/home_mixer/product/scored_tweets/param/ScoredTweetsParam.scala) -- All scoring parameters and defaults
- [`HomeGlobalParams.scala`](https://github.com/twitter/the-algorithm/blob/main/home-mixer/server/src/main/scala/com/twitter/home_mixer/param/HomeGlobalParams.scala) -- Blue Verified multipliers (4x in-network, 2x OON)

### Community & Discovery
- [`SimClusters README`](https://github.com/twitter/the-algorithm/blob/main/src/scala/com/twitter/simclusters_v2/README.md) -- 145K communities, Known-For (top 20M producers), engagement-driven tweet embedding
- [`Interaction Graph README`](https://github.com/twitter/the-algorithm/blob/main/src/scala/com/twitter/interaction_graph/README.md) -- Real Graph: GBT classifier for interaction prediction, decayed sums
- [`EarlyBird README`](https://github.com/twitter/the-algorithm/blob/main/src/python/twitter/deepbird/projects/timelines/scripts/models/earlybird/README.md) -- Light ranker: logistic regression, in-network vs out-of-network variants

### External Analysis
- [Steven Tey: "How the Twitter Algorithm works in 2023"](https://steventey.com/blog/twitter-algorithm)
- [TweetHunter: "Cracking the Code"](https://tweethunter.io/blog/twitter-algorithm-full-analysis)
- [PostEverywhere: "How the X Algorithm Works in 2026"](https://posteverywhere.ai/blog/how-the-x-twitter-algorithm-works)
- [Tweet Archivist: "Complete Technical Breakdown"](https://www.tweetarchivist.com/how-twitter-algorithm-works-2025)
- [Sumit's Diary: "Twitter's For You Recommendation Algorithm"](https://blog.reachsumit.com/posts/2023/04/the-twitter-ml-algo/)

---

## Appendix: Key Numbers at a Glance

| Metric | Value | Source |
|--------|-------|--------|
| TweepCred scale | 0--100 | Reputation.scala |
| Critical threshold | 65 | Community analysis + code |
| Tweets below threshold | 3 max | Community analysis + code |
| PageRank damping | 0.1 | WeightedPageRank.scala |
| Convergence threshold | 0.001 | WeightedPageRank.scala |
| Log mapping formula | `130 + 5.21 * ln(raw)` | Reputation.scala |
| Ratio penalty threshold | 0.6 | UserMass.scala, Reputation.scala |
| Following threshold (pre-PR) | 500 | UserMass.scala |
| Following threshold (post-PR) | 2,500 | Reputation.scala |
| OON scale factor | 0.75x | OONTweetScalingScorer.scala |
| Reply scale factor | 0.75x | ScoredTweetsParam.scala |
| Author diversity decay | 0.5x | ScoredTweetsParam.scala |
| Blue in-network boost | 4.0x | HomeGlobalParams.scala |
| Blue OON boost | 2.0x | HomeGlobalParams.scala |
| Like weight | 0.5 | Heavy ranker |
| Reply weight | 27.0 | Heavy ranker |
| Retweet weight | 1.0 | Heavy ranker |
| Report weight | -369.0 | Heavy ranker |
| Block/mute weight | -74.0 | Heavy ranker |
| Tweet half-life | 360 min (6h) | Decay config |
| Decay rate | 0.003 | Decay config |
| Decay floor | 0.6 | Decay config |
| SimClusters communities | 145,000 | SimClusters README |
| Known-For producers | Top 20M | SimClusters README |
| Daily tweets ranked | 500M | X Engineering blog |
| Candidates per timeline | ~1,500 | X Engineering blog |
| GraphJet feed share | ~15% | X Engineering blog |
| Mass unfollow penalty | 3 months | Community analysis |
| Feedback fatigue window | 140 days | FeedbackFatigueScorer.scala |
| Batch job interval | 24 hours | TweepcredBatchJob.scala |

---

*Note: X's algorithm has evolved since the open-source release. Elon Musk announced in October 2025 that Grok would replace the legacy recommendation system, with changes rolling out January 2026. The TweepCred and SimClusters infrastructure likely still underpins much of the system, but specific weights and thresholds may have shifted. The structural dynamics -- ratio penalties, cold start problems, engagement-driven discovery -- remain architecturally sound observations.*
