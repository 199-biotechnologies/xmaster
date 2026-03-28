# Small Account Analysis: X's 2026 Algorithm

**Subject:** @longevityboris -- ~96 followers, ~104 following, dormant account reactivating
**Date:** 2026-03-27
**Source:** X open-source algorithm release (2026), `source-2026/`
**Methodology:** Direct source code reading + empirical web research

---

## 1. What the 2026 Algorithm Actually Is

The 2026 system is a complete replacement of the 2023 architecture. From the README:

> "We have eliminated every single hand-engineered feature and most heuristics from the system. The Grok-based transformer does all the heavy lifting by understanding your engagement history."

### What Was Killed

| 2023 System | Status in 2026 |
|---|---|
| TweepCred (PageRank reputation) | **Gone.** No reputation scoring at all. |
| SimClusters (community embeddings) | **Gone.** No community/topic clusters. |
| Real Graph (pairwise follow-engagement scores) | **Gone.** Replaced by the engagement sequence transformer. |
| EarlyBird (Lucene-based light ranker) | **Gone.** No pre-scoring step. |
| Hand-engineered features (1,300+ signals) | **Gone.** Zero manual features. |

### What Replaced Them

A single unified system called **Phoenix**, built on the Grok transformer architecture:

1. **Phoenix Retrieval** -- A two-tower model that finds out-of-network posts from a global corpus
2. **Phoenix Ranking** -- A transformer that predicts engagement probabilities per candidate post
3. **Thunder** -- An in-memory store of recent posts from accounts you follow (in-network)

The pipeline runs: Thunder + Phoenix Retrieval --> Hydration --> Filters --> Phoenix Scoring --> Weighted Scoring --> Author Diversity --> OON Scoring --> Top-K Selection --> VF Filter.

---

## 2. How Phoenix Ranking Sees a Dormant Account

### The Input Structure

From `recsys_model.py` (`PhoenixModelConfig`):

```python
history_seq_len: int = 128   # your last 128 engagement events
candidate_seq_len: int = 32  # posts being ranked in each batch
```

The transformer input is a concatenation of three blocks:

```
[User Embedding (1 pos)] + [History Embeddings (128 pos)] + [Candidate Embeddings (32 pos)]
```

Each history position encodes four things:
- **Post embedding** (what content you engaged with)
- **Author embedding** (who wrote it)
- **Action embedding** (what you did -- like, reply, repost, etc.)
- **Product surface embedding** (where you saw it -- For You, search, etc.)

From `block_history_reduce()` in `recsys_model.py`:

```python
post_author_embedding = jnp.concatenate([
    history_post_embeddings_reshaped,    # the post you saw
    history_author_embeddings_reshaped,  # who wrote it
    history_actions_embeddings,          # what you did (like/reply/etc)
    history_product_surface_embeddings,  # where you saw it
], axis=-1)
```

### What a Dormant Account Looks Like to the Transformer

A dormant account has **zero engagement history**. The User Action Sequence Hydrator (`user_action_seq_query_hydrator.rs`) fetches your recent actions:

```rust
let thrift_user_actions = uas_thrift.user_actions.clone().unwrap_or_default();
if thrift_user_actions.is_empty() {
    return Err(format!("No user actions found for user {}", user_id));
}
```

If there are no actions, the hydrator **returns an error**. This propagates to Phoenix Scorer (`phoenix_scorer.rs`):

```rust
if let Some(sequence) = &query.user_action_sequence {
    // ... score candidates
}
// Return candidates unchanged if no scoring could be done
Ok(candidates.to_vec())
```

When there is no user action sequence, **candidates are returned unscored**. The Phoenix ML model is never called. The weighted scorer then sees `None` for every score:

```rust
fn apply(score: Option<f64>, weight: f64) -> f64 {
    score.unwrap_or(0.0) * weight  // all zeros
}
```

**Bottom line: A fully dormant account gets zero ML-driven ranking. Every post scores the same. Your feed is essentially random.**

### What Happens as You Build History

Once you start engaging (liking, replying, etc.), the system begins filling those 128 positions. The padding mask in `block_history_reduce()` marks positions with hash 0 as invalid:

```python
history_padding_mask = (history_post_hashes[:, :, 0] != 0).reshape(B, S)
```

With 5 engagements, you have 5 valid positions out of 128. The transformer sees a user profile that is 96% padding and 4% signal. The model can learn from those 5 data points, but the engagement predictions will be noisy and uncertain.

The critical insight: **your first ~50 engagements are disproportionately valuable** because they go from zero signal to enough for the transformer to build a coherent user embedding.

---

## 3. How Phoenix Retrieval Sees a Dormant Account

### The Two-Tower Architecture

From `recsys_retrieval_model.py`, the retrieval model builds a user representation:

```python
def build_user_representation(self, batch, recsys_embeddings):
    # Encode user + history through transformer
    embeddings = jnp.concatenate([user_embeddings, history_embeddings], axis=1)
    model_output = self.model(embeddings, padding_mask, candidate_start_offset=None)

    # Average pool over valid positions
    user_embeddings_masked = user_outputs * mask_float
    user_embedding_sum = jnp.sum(user_embeddings_masked, axis=1)
    mask_sum = jnp.sum(mask_float, axis=1)
    user_representation = user_embedding_sum / jnp.maximum(mask_sum, 1.0)

    # L2 normalize
    user_representation = user_representation / user_norm
```

Then retrieval is a dot product:

```python
scores = jnp.matmul(user_representation, corpus_embeddings.T)
top_k_scores, top_k_indices = jax.lax.top_k(scores, top_k)
```

### The Cold Start Retrieval Problem

With zero engagement history, the user tower processes only the user embedding (from hash-based lookup) through the transformer. The resulting user representation vector is:
- Dominated by the user hash embedding (which encodes follower count, account age, etc.)
- Missing all engagement-derived directional signal
- Essentially a **generic/default vector** in the embedding space

When this generic vector is dot-producted against the full corpus, it produces **low-variance, undifferentiated similarity scores**. The top-K results are effectively random -- whatever corpus items happen to be nearest to the origin/default region of the embedding space.

**Translation: Out-of-network discovery is nearly useless for dormant accounts.** Phoenix Retrieval cannot find posts you would like because it does not know what you like.

---

## 4. In-Network vs Out-of-Network: The Small Account Trap

### Thunder (In-Network)

Thunder serves posts from accounts you follow. From `thunder_source.rs`:

```rust
let request = GetInNetworkPostsRequest {
    user_id: query.user_id,
    following_user_ids: following_list.iter().map(|&id| id as u64).collect(),
    max_results: p::THUNDER_MAX_RESULTS,
    // ...
};
```

With 104 following, Thunder pulls recent posts from those 104 accounts. The `PostStore` (`post_store.rs`) has a 2-day retention window:

```rust
impl Default for PostStore {
    fn default() -> Self {
        Self::new(2 * 24 * 60 * 60, 0)  // 2 days
    }
}
```

Thunder caps posts per author (`MAX_ORIGINAL_POSTS_PER_AUTHOR`, `MAX_REPLY_POSTS_PER_AUTHOR`). If your 104 follows are moderately active, you might get 200--500 in-network candidates. This is your reliable content source.

### Phoenix Retrieval (Out-of-Network)

From `phoenix_source.rs`:

```rust
fn enable(&self, query: &ScoredPostsQuery) -> bool {
    !query.in_network_only  // only runs if OON is enabled
}
```

Phoenix Retrieval requires `user_action_sequence`. Without it:

```rust
let sequence = query
    .user_action_sequence
    .as_ref()
    .ok_or_else(|| "PhoenixSource: missing user_action_sequence".to_string())?;
```

**If the action sequence is missing, Phoenix Retrieval returns an error and produces zero OON candidates.** For a newly reactivating dormant account, this means your first few sessions may be entirely in-network.

### The OON Weight Penalty

Even when OON candidates exist, from `oon_scorer.rs`:

```rust
let updated_score = c.score.map(|base_score| match c.in_network {
    Some(false) => base_score * p::OON_WEIGHT_FACTOR,  // reduced
    _ => base_score,  // unchanged
});
```

`OON_WEIGHT_FACTOR` is less than 1.0 -- it penalizes out-of-network posts. For a small account, this creates a paradox:

- Your in-network is tiny (104 people) and may produce low-quality candidates
- Your out-of-network is your only path to discovering new content and new people
- But OON content is systematically downranked

### Why This Matters for YOUR Feed vs YOUR Visibility

The above analysis is about what **you see**. Your feed will be impoverished -- mostly from your 104 follows, with minimal OON discovery.

For **your visibility to others**: other users' Phoenix Retrieval might find your posts in the global corpus. Whether they do depends on whether your posts' candidate embeddings are similar to other users' user embeddings. A new post from a dormant account has no engagement signal attached to it -- it relies purely on the post/author embedding quality.

---

## 5. Author Diversity Scorer: Impact on Prolific Small Accounts

From `author_diversity_scorer.rs`:

```rust
fn multiplier(&self, position: usize) -> f64 {
    (1.0 - self.floor) * self.decay_factor.powf(position as f64) + self.floor
}
```

The system tracks how many times each author appears in the ranked candidates. For each appearance:

| Author Appearance | Multiplier |
|---|---|
| 1st post | `(1 - floor) * decay^0 + floor` = 1.0 |
| 2nd post | `(1 - floor) * decay^1 + floor` |
| 3rd post | `(1 - floor) * decay^2 + floor` |
| nth post | Converges to `floor` |

`AUTHOR_DIVERSITY_DECAY` and `AUTHOR_DIVERSITY_FLOOR` are config parameters (not published).

### Impact on a Prolific Small Account

If you post 10 times in a day:
- Your 1st post is scored at full weight
- Each subsequent post gets exponentially diminished
- By post 5--10, multiplier approaches `floor` (likely 0.1--0.3)

This means **posting more than 3--5 times per day has sharply diminishing returns** in terms of reaching any single user's feed. The diversity scorer ensures no single author dominates. This is neutral for visibility (it affects all authors equally) but means that quantity-spamming is explicitly punished.

For a small account, the optimal strategy is **fewer, higher-quality posts** rather than volume.

---

## 6. Filters That Affect Small Accounts

### AgeFilter

From `age_filter.rs`:

```rust
pub struct AgeFilter {
    pub max_age: Duration,
}
```

Configured via `params::MAX_POST_AGE` in the pipeline. Posts older than this threshold are removed. Combined with Thunder's 2-day retention, posts have a hard expiration window.

**Small account impact:** Neutral. Applies to all posts equally.

### PreviouslySeenPostsFilter / PreviouslyServedPostsFilter

These use bloom filters and client-reported seen IDs to remove posts the user has already seen. For a returning dormant user with a clean slate, these filters would be empty initially -- **you will not be filtered by prior impression data**.

### VFFilter (Visibility Filtering)

Post-selection filter for deleted/spam/violence/gore content:

```rust
fn should_drop(reason: &Option<FilteredReason>) -> bool {
    match reason {
        Some(FilteredReason::SafetyResult(safety_result)) => {
            matches!(safety_result.action, Action::Drop(_))
        }
        Some(_) => true,
        None => false,
    }
}
```

**Small account impact:** Potentially disproportionate. If the safety model is more aggressive toward low-follower accounts (a common industry pattern to combat spam accounts), legitimate small account content could be more likely to trigger false positives. The code does not reveal the safety model's internals.

### AuthorSocialgraphFilter

Removes posts from blocked/muted authors. Neutral for small accounts.

### No Anti-Small-Account Filters

Critically, there is **no explicit filter on follower count, account age, or engagement rate**. The 2023 system had TweepCred which assigned reputation scores (effectively punishing new accounts). The 2026 system has no equivalent. The discrimination against small accounts is implicit through the engagement-history-driven ML models, not through explicit filtering.

---

## 7. The Candidate Isolation Masking System

A unique architectural detail from `grok.py`:

```python
def make_recsys_attn_mask(seq_len, candidate_start_offset, dtype):
    # Candidates cannot attend to each other
    # They can only attend to user + history and themselves
    attn_mask = causal_mask.at[:, :, candidate_start_offset:, candidate_start_offset:].set(0)
    attn_mask = attn_mask.at[:, :, candidate_indices, candidate_indices].set(1)
```

Each candidate post is scored **independently** -- it can see the user context and history but not the other candidates. This means:

1. **Scores are position-independent** -- your post's score does not depend on what other posts are in the batch
2. **Scores are cacheable** -- the same user + same post always produces the same score
3. **No competitive suppression within a batch** -- a viral post next to yours does not push your score down

For a small account, this is mildly positive: your post is evaluated purely against the user's profile, not relative to competing high-engagement posts.

---

## 8. The Weighted Scorer: What Actions Matter

From `weighted_scorer.rs` and `runners.py`, the 19 predicted actions are:

```
favorite_score, reply_score, repost_score, photo_expand_score,
click_score, profile_click_score, vqv_score, share_score,
share_via_dm_score, share_via_copy_link_score, dwell_score,
quote_score, quoted_click_score, follow_author_score,
not_interested_score, block_author_score, mute_author_score,
report_score, dwell_time
```

The weighted sum:

```rust
let combined_score = Self::apply(s.favorite_score, p::FAVORITE_WEIGHT)
    + Self::apply(s.reply_score, p::REPLY_WEIGHT)
    + Self::apply(s.retweet_score, p::RETWEET_WEIGHT)
    // ... (all 19 actions)
    + Self::apply(s.not_interested_score, p::NOT_INTERESTED_WEIGHT)  // negative
    + Self::apply(s.block_author_score, p::BLOCK_AUTHOR_WEIGHT)      // negative
    + Self::apply(s.mute_author_score, p::MUTE_AUTHOR_WEIGHT)        // negative
    + Self::apply(s.report_score, p::REPORT_WEIGHT);                  // negative
```

The exact weights are not published, but empirical research consistently reports:

| Action | Approximate Multiplier (vs. Like = 1x) |
|---|---|
| Like | 1x |
| Bookmark | 10x |
| Profile Click | 12x |
| Retweet | 20x |
| Quote Tweet | 25x |
| Reply | 27x |
| Reply + Author Reply Back | 150x |

Negative signals (not_interested, block, mute, report) subtract from the score, actively suppressing content.

**Key takeaway:** The algorithm overwhelmingly rewards **conversation** (replies, quote tweets) over passive engagement (likes). A post that generates a single reply thread is worth more than a post that gets 27 likes.

---

## 9. Empirical Data: What Actually Works in 2026

### X Premium Is Nearly Table-Stakes

Buffer's analysis of **18.8 million posts from 71,000 accounts** (Aug 2024 -- Aug 2025):

| Account Type | Median Impressions/Post | Median Engagement Rate |
|---|---|---|
| Free account | <100 | 0% (half get zero) |
| Premium | ~600 | 0.49% |
| Premium+ | >1,550 | 0.53% |

Free accounts saw their link posts "completely suppressed since March 2025 -- median engagement rate is 0%." The gap between free and Premium on X is the largest reach differential of any social platform.

For a small account reactivating: **X Premium ($8/month) is the single most impactful lever.** Without it, the algorithm gives you functionally zero organic reach on most post types.

### The First-Hour Engagement Window

The algorithm applies a steep time decay -- a post loses roughly half its potential visibility score every six hours. The first 30--60 minutes are the critical scoring window. A tweet that gets 10 replies in 15 minutes dramatically outperforms one that gets 10 replies over 24 hours.

**For a 96-follower account:** Initial distribution is ~5--15% of your follower base (5--15 people). If none of them engage in the first hour, the post dies. Your strategy must ensure those first impressions convert to engagement.

### Engagement Ratio Matters, Not Absolute Numbers

The algorithm evaluates engagement **ratio**, not absolute counts. A post with 10 impressions and 3 replies (30% engagement) can outperform a post with 10,000 impressions and 100 replies (1% engagement) in terms of further distribution probability.

**This is the small account's structural advantage.** With only 96 followers, a single reply creates a higher engagement ratio than a large account would achieve. The challenge is getting that first engagement at all.

### Reply Strategy Dominates Everything

Empirical consensus across all 2026 growth research:

1. **Replies to larger accounts are your primary discovery channel.** When you reply to a 50K-follower account and get a reply back, the 150x weight makes that interaction visible to a vastly larger audience than your posts ever reach organically.

2. **A reply is worth 27x a like.** Time spent crafting one thoughtful reply to a relevant account is worth more than time spent writing 27 mediocre original posts.

3. **Conversation depth is the apex signal.** One genuine reply chain where the author engages back is worth more than hundreds of likes.

### Content Type Performance (2026)

For Premium accounts (non-Premium performance is too suppressed to measure meaningfully):

| Content Type | Median Engagement Rate |
|---|---|
| Text-only posts | ~0.9% |
| Video posts | ~0.7% |
| Image posts | 0.4--0.5% |
| Link posts | 0.25--0.3% |

Text-only posts significantly outperform all other formats. The algorithm's Grok transformer can semantically parse text; it cannot extract the same depth of signal from images or links.

---

## 10. The Reactivation Playbook: Derived From Source Code

### Phase 1: Bootstrap the Engagement History (Days 1--7)

**Goal:** Fill the 128-position history sequence with signal.

The Phoenix models cannot rank or retrieve for you until you have engagement data. Every like, reply, and repost you perform teaches the transformer what you care about.

1. **Like 30--50 posts per day** in your niche (longevity, biotech, science). Each like fills one history position with a post embedding + author embedding + action embedding.
2. **Reply to 10--20 posts per day.** Replies create stronger history signals than likes (the action embedding for "reply" is a distinct, higher-signal vector).
3. **Follow 5--10 relevant accounts per day.** This expands your Thunder in-network pool from 104 to 150+, increasing in-network candidate variety.

After 7 days: ~350+ engagement events. The history buffer holds the last 128. Your user embedding in Phoenix Retrieval is now directionally meaningful. OON discovery begins to function.

### Phase 2: Establish Posting Cadence (Days 7--30)

**Goal:** Create content that generates engagement within the first hour.

1. **Post 2--3 high-quality text posts per day.** Not more -- the Author Diversity Scorer penalizes prolific posting. Quality over quantity is architecturally enforced.
2. **Time posts for your audience's active hours.** With 96 followers, your initial distribution is ~10 people. If they are asleep, the post dies in the time decay window.
3. **Craft posts that invite replies.** The 27x reply weight means a post that generates 3 replies scores equivalently to a post that gets 81 likes.
4. **Reply to your own replies within minutes.** The 150x weight on "reply + author reply back" is the most powerful signal in the entire system. When someone replies to you, respond immediately.

### Phase 3: Leverage Reply Discovery (Days 1--90, Ongoing)

**Goal:** Use replies to larger accounts as your primary growth channel.

Since Phoenix Retrieval is weak for your account (low OON discovery), and your in-network is small (104 people), the primary path to being seen is through other people's threads:

1. **Identify 20--30 accounts in longevity/biotech with 5K--100K followers.**
2. **Turn on notifications for their posts.**
3. **Reply within 15 minutes of their posts** with substantive, original insight -- not agreement, not flattery, but genuine contribution.
4. **The algorithm will surface your reply** to other people reading that thread. If your reply gets engagement, it creates a cascade: profile clicks (12x weight), follows (tracked by `follow_author_score`), and your user embedding strengthens.

### Phase 4: X Premium (Immediately)

Subscribe to X Premium ($8/month). The empirical data is unambiguous:
- 10x reach multiplier vs. free
- Reply priority in conversation threads
- Replies from Premium accounts appear higher
- Free accounts have "effectively zero organic reach" on many post types

Without Premium, you are performing the source code equivalent of scoring every post at 0 and hoping for a miracle.

---

## 11. What the Algorithm Cannot See

The following things are **not signals** in the 2026 algorithm (based on the published source code):

- **Post text content** is not directly used in scoring. The Grok transformer sees post/author embeddings derived from hash lookups, not raw text. The semantic understanding comes from how the embeddings were trained, not from real-time text analysis during inference.
- **Follower count** is not an explicit feature. There is `author_followers_count` in the `PostCandidate` struct, but it appears in the hydration layer for display, not in the scoring pipeline.
- **Account age** is not a feature. There is no account creation date in `UserFeatures` or `ScoredPostsQuery`.
- **Blue checkmark / verification status** is not visible in the scoring code. The Premium boost (if it exists at the algorithm level) is not in the published source. It may operate at a layer above or below this code.

The discrimination against small accounts is **purely behavioral** -- you have no engagement history, so the ML model has no data to work with. It is not a penalty; it is an absence of signal.

---

## 12. Summary: The Small Account Equation in 2026

```
Your feed quality     = f(your engagement history depth)
Your post visibility  = f(first-hour engagement rate, Premium status)
Your discovery by     = f(replies to larger accounts' threads)
  others
```

**The 2026 algorithm is simultaneously more fair and more brutal than 2023:**

- **More fair:** No TweepCred reputation gatekeeping, no SimClusters community lock-in, no explicit small-account penalties. If you generate engagement, the algorithm will amplify you.

- **More brutal:** The cold start is absolute. Zero history means zero ML signal. The system literally cannot score your feed or retrieve content for you. You start from mathematical zero, and every engagement event is critical infrastructure.

The path out is narrow but well-defined: subscribe to Premium, engage aggressively to build your history, post 2--3 times daily in text format, reply strategically to larger accounts, and respond to every reply you receive within minutes. The source code says that is all there is. There are no shortcuts and no hidden levers -- but also no hidden penalties.

---

## Sources

### Primary Source
- [X For You Feed Algorithm 2026 source code](source-2026/) -- open-source release, Apache 2.0

### Empirical Research
- [Buffer: Does X Premium Really Boost Your Reach? (18.8M posts analyzed)](https://buffer.com/resources/x-premium-review/)
- [OpenTweet: How the Twitter/X Algorithm Works in 2026](https://opentweet.io/blog/how-twitter-x-algorithm-works-2026)
- [SocialBee: Understanding the X Algorithm in 2026](https://socialbee.com/blog/twitter-algorithm/)
- [Sprout Social: How the Twitter Algorithm Works in 2026](https://sproutsocial.com/insights/twitter-algorithm/)
- [SocialWick: Decoding the New X Algorithm to Stay Visible in 2026](https://www.socialwick.com/decoding-the-new-x-algorithm-to-stay-visible-in-2026)
- [Postel: How to Grow Your X Account from 0 to 500 Followers](https://www.postel.app/blog/How-to-Grow-Your-X-Account-To-500-Followers-in-2025-A-Step-by-Step-Guide)
- [SocialRails: How to Grow on Twitter/X Complete Guide 2026](https://socialrails.com/blog/how-to-grow-on-twitter-x-complete-guide)
- [IPFoxy: 2026 X Account Warm-Up Guide](https://www.ipfoxy.com/blog/ideas-inspiration/5622)
- [Medium: I Solved the New X Algorithm (Alain Yunes)](https://medium.com/write-a-catalyst/i-solved-the-new-x-algorithm-heres-how-to-grow-in-2026-8d54624adeb0)
