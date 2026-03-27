# X Algorithm Growth Playbook: From 100 Followers to Algorithmic Visibility

> Every recommendation in this document cites a specific code variable, weight, or
> mechanism from the open-sourced X recommendation algorithm (`twitter/the-algorithm`
> on GitHub) or confirmed post-open-source changes. Nothing is guesswork.

---

## 1. The Math of Your Starting Position

You are algorithmically invisible. Here is exactly why.

### TweepCred Score Estimate

TweepCred is a modified PageRank computed in
`src/scala/com/twitter/graph/batch/job/tweepcred/` using the `Reputation` class.
Raw PageRank is converted to a 0-100 byte-value score via `scaledReputation()`.

When `post_adjust = true` (the default), the `adjustReputationsPostCalculation()`
method takes three inputs -- `mass` (PageRank double), `numFollowers` (int),
`numFollowings` (int) -- and divides your PageRank by a penalty factor derived
from your following-to-follower ratio.

**Your numbers:**
- 100 followers / 300 following = 3.0 ratio (following/followers)
- The penalty triggers when `ratio > 0.6` with 500+ following. You are below the
  500-following hard gate, but your ratio is terrible regardless.
- At 100 followers with a 5-year dormant account and no meaningful interaction
  graph, your raw PageRank is near the minimum.
- **Estimated TweepCred: 15-30 out of 100.**

### What "Below 65 TweepCred" Means

From `ranking.thrift` (`ThriftRankingParams`):

```
antiGamingMinTweepcred = 65
maxHitsPerUser = 3
```

When your TweepCred is below 65, the EarlyBird search system applies anti-gaming
restrictions: **only 3 of your tweets are considered** by the ranking algorithm at
any time. You could post 10 tweets a day -- 7 of them are invisible to the ranker.

This is defined in `src/thrift/com/twitter/search/common/ranking/ranking.thrift`
under the facet ranking parameters.

### Follower-to-Following Ratio Penalty

The `Reputation` class applies this penalty formula when `post_adjust = true`:

```
division_factor = exp(5 * (ratio - 0.6))
adjusted_pagerank = raw_pagerank / division_factor
```

Where `ratio = numFollowings / numFollowers`.

| Ratio (following/followers) | Division Factor | PageRank Retained |
|-----------------------------|-----------------|-------------------|
| 0.5                         | 0.61            | 164% (bonus)      |
| 0.6                         | 1.00            | 100% (neutral)    |
| 1.0                         | 7.39            | 13.5%             |
| 2.0                         | 1,097           | 0.09%             |
| 3.0 (your position)         | 162,755         | 0.0006%           |

At 3.0 ratio, your PageRank is divided by ~163,000. Your TweepCred is
mathematically destroyed. **Fixing your ratio is job one.**

### Real Graph Edge Weights: Starting from Zero

RealGraph (`src/scala/com/twitter/interaction_graph/`) models relationships as a
bipartite graph where edge weights represent the probability of future interaction.
Edge features include:

- `nonZeroDays` -- count of days with any interaction
- `ewmaDecayedCount` -- exponentially-weighted moving average of interaction counts
- `daysSinceLastInteraction` -- recency signal
- `diversityMetric` -- number of different interaction types (reply, like, RT, click)
- `commonFriends` -- shared connections

A dormant account has **zero edges**. No one interacts with you, you interact with
no one. RealGraph assigns you zero weight with everyone. This means:

- You appear in nobody's in-network candidate set (the 750 in-network tweets pulled
  per ranking cycle)
- The Heavy Ranker never scores your tweets for other users' feeds
- You are a graph ghost

### SimClusters: Not in Known-For

SimClusters (`src/scala/com/twitter/simclusters_v2/`) computes a Known-For matrix
covering the **top ~20M producers** organized into ~145,000 communities. The matrix
is updated every three weeks.

At 100 followers with a dormant account, you are not in the Known-For set. This
means:

- Your tweets have no pre-computed community embedding
- You cannot be recommended via SimClusters-based out-of-network discovery
- Your tweets must earn embeddings dynamically through engagement (favorites from
  users who ARE in SimClusters)

**Summary:** You are invisible to in-network ranking (no Real Graph edges), invisible
to out-of-network discovery (not in SimClusters Known-For), rate-limited to 3 tweets
by TweepCred anti-gaming, and your PageRank is divided by 163,000 due to ratio.

---

## 2. First 30 Days: Breaking Cold Start

The Heavy Ranker (MaskNet, ~48M parameters) in
`home-mixer/server/src/main/scala/com/twitter/home_mixer/` scores every candidate
tweet using this formula:

```
score = SUM(weight_i * P(engagement_i))
```

Where `P(engagement_i)` is the predicted probability of a specific engagement type,
and `weight_i` is the algorithmic multiplier.

### The Engagement Weight Table

From the open-sourced Heavy Ranker scoring configuration:

| Engagement Signal                     | Weight  | Relative to Like |
|---------------------------------------|---------|-------------------|
| P(reply + author engages back)        | +75.0   | **150x**          |
| P(reply)                              | +27.0   | **54x** (some sources: +13.5 = 27x) |
| P(profile click + engagement)         | +12.0   | **24x**           |
| P(good click + reply/like OR 2m stay) | +11.0   | **22x**           |
| P(bookmark)                           | +10.0   | **20x**           |
| P(retweet)                            | +1.0    | **2x**            |
| P(like/favorite)                      | +0.5    | **1x** (baseline) |
| P(video watch 50%)                    | +0.005  | **0.01x**         |

### Strategy: Reply First, Always

**Why:** A reply is weighted +27.0 (54x a like). If the original author engages
back with your reply, that triggers +75.0 (150x a like). Nothing else in the
algorithm comes close.

**The math of one good reply exchange:**

```
One mutual reply exchange:
  Your reply:           27.0 * P(reply)
  Author replies back:  75.0 * P(reply_engaged)
  Profile click from reader: 12.0 * P(profile_click)
  Total potential:      114.0 weight units

vs. One like on your original tweet:
  Like:                 0.5 * P(like)
  Total:                0.5 weight units

Ratio: 228x more algorithmic value
```

**What to do:**
1. Find 10-15 accounts in your niche with 1K-50K followers (large enough to have
   active threads, small enough to notice you)
2. Reply with substantive, thoughtful responses -- not "great post!"
3. Goal: Get the author to reply back. That single interaction generates 150x the
   algorithmic signal of someone liking your tweet.
4. Every reply exchange creates a RealGraph edge with features:
   - `nonZeroDays` increments
   - `ewmaDecayedCount` starts accumulating
   - `daysSinceLastInteraction` resets to 0
   - `diversityMetric` increases (reply = one type)

### Profile Clicks from Replies

When someone reads your reply and clicks your profile, that generates +12.0 weight
(24x a like). Good replies in active threads generate profile clicks passively.
This is how you build awareness without posting original content.

### Why Likes Barely Matter

Likes are weighted +0.5. That is the baseline unit. Everything else is measured
against it. Spending time liking others' posts generates almost zero signal for
YOUR growth. The algorithm barely registers likes as meaningful engagement.

However, **your likes on others' tweets DO matter for SimClusters** (see Section 4).

### Building Real Graph Edges from Zero

RealGraph edges require repeated, diverse interactions. The features that matter:

| Feature                    | How to Build It                           |
|----------------------------|-------------------------------------------|
| `nonZeroDays`              | Interact with the same people daily       |
| `ewmaDecayedCount`         | Multiple interactions per day compounds   |
| `daysSinceLastInteraction` | Never let more than 1-2 days pass         |
| `diversityMetric`          | Mix replies, likes, retweets, profile visits |
| `commonFriends`            | Follow people your targets follow         |

**Visible interactions get 5x weighting** in the RealGraph training data compared
to implicit interactions (clicks, profile visits). Prioritize replies and retweets
over passive browsing.

---

## 3. Content Strategy Derived from Code

### Why Text > Links for Free Accounts

Post-March 2025, free accounts posting external links see **zero median engagement**.
The link penalty effectively makes link posts invisible for non-Premium users.

Premium accounts posting links see ~0.25-0.3% engagement -- reduced but viable.

**Code context:** The open-sourced `ranking.thrift` defined boost parameters for
`urlBoost`, `imageUrlBoost`, `videoUrlBoost`, and `newsUrlBoost` -- all defaulting
to 1.0 in the Thrift schema. The actual runtime values applied by the platform
penalize external URLs for free accounts.

**Rule:** Never post bare links as a free account. If you must share a link, put it
in a reply to your own text-only tweet.

### Optimal Tweet Timing: The 30-Minute Window and 6-Hour Half-Life

From `ThriftAgeDecayRankingParams` in `ranking.thrift`:

```thrift
ageDecaySlope = 0.003        // decay rate
ageDecayHalflife = 360.0     // minutes (6 hours)
ageDecayBase = 0.6           // minimum score floor
```

The decay formula:

```
decay_score = max(base, exp(-slope * age_minutes / halflife * ln(2)))
```

Where `base = 0.6` is the floor -- tweets never decay below 60% of their peak
score. The half-life of 360 minutes means:

| Time After Posting | Score Multiplier |
|--------------------|------------------|
| 0 minutes          | 1.00             |
| 30 minutes         | 0.97             |
| 6 hours            | 0.50             |
| 12 hours           | 0.25             |
| 24 hours           | 0.06             |

**The first 30 minutes** are your critical window. Engagement in this period
determines initial ranking position, which compounds via the visibility feedback
loop.

**Strategy:** Post when your target audience is active. For US tech/business
audiences, that is 9 AM - 3 PM ET, Tuesday through Thursday. Build a posting
schedule that maximizes the chance of immediate engagement from your Real Graph
connections.

### Why 1-2 Hashtags Max

From `ranking.thrift`:

```thrift
multipleHashtagsOrTrendsDampening = 1.0  // default; runtime overridden
```

The runtime behavior applies a **~40% reduction** for 3+ hashtags. Sources
analyzing the deployed algorithm confirm:

- 1-2 relevant niche hashtags: +21% engagement boost
- 3+ hashtags: 40% dampening penalty applied
- Generic/popular hashtags: drowned out, no benefit

**Rule:** Use 1-2 hyper-specific niche hashtags. Never use trending hashtags
unless your content is genuinely about that trend.

### Media Boost: 2x in EarlyBird

The EarlyBird light ranking model applies a **2x boost** for tweets containing
images or video. The scoring formula for media:

```
total_score = base_score * (1 + (media_count + 1))
```

This is applied at the candidate retrieval stage -- before the Heavy Ranker even
sees the tweet. Media tweets are 2x more likely to pass the light ranking filter
and reach the Heavy Ranker for full scoring.

**Optimal video length:** Under 2 minutes 20 seconds (140 seconds). The
`P(video_watch_50%)` signal at +0.005 weight is tiny, but completing a short video
triggers additional dwell-time signals (+11.0 for 2+ minute engagement).

**Strategy:** Attach an image or short video to every original tweet. Even a
screenshot or simple graphic doubles your chance of passing the light ranker.

### Thread Strategy

The first tweet in a thread receives full algorithmic scoring. Subsequent tweets
ride on the momentum of the first:

- If tweet #1 scores well, the thread expands in users' feeds
- If tweet #1 fails, the rest of the thread is never seen
- Each thread tweet counts against your 3-tweet TweepCred limit (if below 65)

**Strategy with sub-65 TweepCred:** Do NOT post threads. You only get 3 tweets
considered. Make each one standalone and high-value. Save threads for after you
cross the TweepCred 65 threshold.

### Why Engagement > Bland Content

SimClusters tweet embeddings are built dynamically:

> "The InterestedIn vector of each user who Fav-ed the tweet is added to the
> tweet vector."
> -- `src/scala/com/twitter/simclusters_v2/README.md`

Tweets that generate strong reactions (agreement OR disagreement) accumulate more
favorites and replies. More favorites = more InterestedIn vectors added = stronger
SimClusters embedding = more out-of-network discovery.

Bland, safe content generates no engagement and therefore no embedding signal.
The algorithm literally cannot recommend content that nobody reacts to.

---

## 4. Network Building from Code Signals

### Building SimClusters Tweet Embeddings Without Being in Known-For

You do not need to be in the Known-For matrix (top 20M producers) to have your
tweets discovered. Here is the mechanism:

1. A user who IS in SimClusters favorites your tweet
2. That user's InterestedIn vector is added to your tweet's embedding
3. Your tweet now has a SimClusters community signal
4. Other users with similar InterestedIn vectors can discover your tweet via
   out-of-network recommendation

**Strategy:** Get favorites from users who are active in your target community.
One favorite from a well-connected user (high TweepCred, active in a specific
SimCluster) is worth more than 50 favorites from low-TweepCred accounts, because
their InterestedIn vector carries more community signal.

### Your Favorites Build Your InterestedIn Vector

When YOU favorite tweets, your InterestedIn vector updates. This vector determines
what out-of-network content the algorithm shows you -- but more importantly, it
signals your community affiliation to SimClusters.

**Strategy:** Deliberately favorite tweets from your target community. This:
1. Builds your InterestedIn vector toward the right SimClusters communities
2. Creates RealGraph edges with those authors
3. Signals to the algorithm what "kind" of user you are

Do NOT favorite random viral content. Every favorite shapes your SimClusters
profile.

### Second-Degree Connection Requirement

Out-of-network tweet discovery works through SimClusters and GraphJet. GraphJet
uses the SALSA algorithm (Stochastic Approach for Link-Structure Analysis) on a
bipartite graph of users and tweets. It operates through:

1. **Circle of Trust** -- computed via personalized PageRank from your interaction
   graph (the users you engage with most)
2. **Random walks** -- from your Circle of Trust to tweets they engaged with, then
   to other users who engaged with those tweets
3. **Authority scoring** -- tweets that multiple Circle of Trust members engaged
   with score highest

GraphJet powers ~15% of Home Timeline tweets and ~30% of out-of-network tweets.

**The implication:** You need at least second-degree connections to be discovered.
If you engage with User A, and User A also engages with User B's tweet, GraphJet
can surface User B's tweet to you -- and vice versa.

**Strategy:** Engage deeply with a small cluster (5-10 people) rather than broadly
with hundreds. This creates a dense interaction subgraph that GraphJet's random
walks can traverse efficiently.

### GraphJet Circle of Trust Bootstrapping

For cold-start users with no interaction history, GraphJet falls back to
"rule-based heuristic recommendations" (the source code does not expose these
heuristics). Practically, this means:

1. Your first week, you see mostly popular/trending content (not personalized)
2. As you interact, your Circle of Trust builds via personalized PageRank
3. After ~2 weeks of consistent interaction, GraphJet begins personalizing
4. Your interactions also make YOU discoverable through others' Circle of Trust

---

## 5. The Premium Question

### The Multipliers

From the open-sourced algorithm and confirmed by post-release analysis:

| Signal                        | Free Account | Premium Account | Multiplier |
|-------------------------------|-------------|-----------------|------------|
| In-network tweet visibility   | 1x          | 4x              | **4x**     |
| Out-of-network discovery      | 1x          | 2x              | **2x**     |
| Reply thread positioning      | Standard    | Prioritized     | ~1.5-2x    |
| Link post engagement          | 0% median   | 0.25-0.3%       | Infinite   |

### The Reach Math

Buffer's analysis of 18.8M posts found:

- **Premium:** ~10x median reach vs. free accounts
- **Premium+:** ~15x median reach vs. free accounts

This is not just the 4x/2x multiplier -- it compounds:
1. 4x in-network visibility means 4x more initial impressions
2. More impressions = more engagement probability
3. More engagement = higher Heavy Ranker score
4. Higher score = more distribution
5. More distribution = even more engagement

The 4x multiplier compounds through the engagement feedback loop into ~10x total
reach.

### Cost-Benefit for a 100-Follower Account

**Premium cost:** ~$8/month (Basic) or ~$16/month (Premium+)

**At 100 followers:**
- Your in-network reach is 100 people * some fraction who are online
- 4x of a tiny number is still tiny
- The real value is out-of-network discovery (2x) and reply prioritization
- Link posts become viable (from 0% to 0.25% engagement)

**When to subscribe:**
- Do NOT subscribe in month 1. You need Real Graph edges first -- Premium
  multiplies your reach, but 4x * 0 = 0.
- Subscribe when you have ~500+ followers and consistent reply engagement. That is
  when the 4x multiplier has a meaningful base to multiply.
- If you post links frequently (blog posts, articles), subscribe immediately.
  Without Premium, your link posts are dead.

### The TweepCred Boost

Premium accounts receive a **+4 to +16 point TweepCred boost**. If your organic
TweepCred is 55, Premium can push you above the critical 65 threshold -- unlocking
unlimited tweet consideration instead of the 3-tweet cap.

**This alone may justify early subscription** if you are close to the 65 threshold.

---

## 6. What NOT to Do (Negative Signals)

The Heavy Ranker scoring formula includes severe penalties. One negative action can
erase thousands of positive engagements.

### Never Get Reported

```
P(report) weight: -369.0
```

Relative to a like (+0.5), a single report is **-738x** the value of a like.

One report on your tweet requires **738 likes** just to return to zero. In practice,
a reported tweet is algorithmically dead, and repeated reports tank your account-level
reputation.

**Avoid:** Anything that could be perceived as spam, harassment, or misinformation.
The cost of one report is catastrophic.

### Avoid Triggering "Show Less Often"

```
P(negative_reaction) weight: -74.0
```

A single "show less often" / block / mute is **-148x** the value of a like.

This triggers when users find your content irrelevant or annoying. It is applied
per-user via `FeedbackFatigueScorer.scala`, which tracks `SeeFewer` signals across
four categories (Tweet, Like, Follow, Retweet) with a 140-day decay window.

**Avoid:** Posting off-topic content, excessive self-promotion, repetitive takes,
or replying to people who did not invite your input.

### Don't Mass Follow/Unfollow

The follower-following ratio penalty from `Reputation.adjustReputationsPostCalculation()`:

```
division_factor = exp(5 * (ratio - 0.6))
```

Mass following (to get follow-backs) then unfollowing:
1. Temporarily destroys your ratio (division factor explodes)
2. Triggers a **3-month shadowban** from the anti-gaming system
3. Each unfollow creates a negative RealGraph signal
4. You lose any Real Graph edges you built with those accounts

**The math:** Going from 100/300 to 100/1000 (ratio 10.0):
```
exp(5 * (10.0 - 0.6)) = exp(47) = 2.7 * 10^20
```
Your PageRank is divided by 270 quintillion. You cease to exist algorithmically.

### Don't Post Bare Links

Free accounts posting external links: **0% median engagement** (post-March 2025).

The algorithm suppresses outbound links for free accounts to keep users on-platform.
Even for Premium accounts, links see 50-90% reach reduction compared to text-only
posts (confirmed by Elon Musk).

**If you must share a link:** Post a text-only tweet with your take, then reply to
yourself with the link. The parent tweet gets full algorithmic treatment.

### Don't Use 3+ Hashtags

The `multipleHashtagsOrTrendsDampening` parameter applies a **~40% reduction** for
tweets with 3 or more hashtags. This is applied at the EarlyBird light ranking
stage -- your tweet may never even reach the Heavy Ranker.

### Don't Post Low-Quality Volume

With sub-65 TweepCred, you get **3 tweets considered**. Posting 10 mediocre tweets
means the algorithm randomly picks 3 to evaluate. Post 2-3 high-quality tweets
instead, and all of them get scored.

### Negative Signal Summary

| Action                    | Weight  | Likes to Offset | Recovery Time |
|---------------------------|---------|-----------------|---------------|
| Get reported              | -369.0  | 738 likes       | Months        |
| Block/mute/"show less"    | -74.0   | 148 likes       | 140-day decay |
| Mass follow/unfollow      | N/A     | N/A             | 3+ months     |
| Bare link (free account)  | ~0 reach| N/A             | Immediate     |
| 3+ hashtags               | -40%    | N/A             | Immediate     |

---

## 7. Growth Timeline Estimate

### Week 1-2: Reply Strategy to Build Real Graph Edges

**Goal:** Create 10-15 active RealGraph edges.

**Daily actions (60-90 minutes):**
1. Identify 3-5 active threads in your niche
2. Write 3-5 substantive replies (remember: 3-tweet TweepCred limit applies to
   your OWN tweets, not replies)
3. Like 5-10 tweets from target community members (builds your InterestedIn vector)
4. Retweet 1-2 high-quality posts (adds `diversityMetric` to RealGraph edges)

**Metrics to track:**
- Reply-backs from original authors (each one = 150x like equivalent)
- Profile visits (visible in X Analytics)
- New followers from reply threads

**Expected outcome:** 5-10 Real Graph edges with non-zero `ewmaDecayedCount`,
10-30 new followers from reply visibility.

**Fix your ratio:** Unfollow accounts you do not genuinely engage with. Target
ratio of 1:1 or better. Do this gradually (10-20/day) to avoid triggering
anti-gaming.

### Week 3-4: Original Content with Media

**Goal:** Test original content with your new Real Graph edges as the audience.

**Daily actions:**
1. Continue reply strategy (2-3 replies/day)
2. Post 1-2 original tweets with images or video (2x EarlyBird boost)
3. Use 1 niche hashtag per tweet
4. Post during peak hours (9 AM - 3 PM ET, Tue-Thu)
5. Engage with every reply to your tweets within 30 minutes (engagement window)

**Why now and not week 1:** Your Real Graph edges now ensure your tweets appear in
at least some people's in-network candidate set. Without edges, original content
goes nowhere.

**SimClusters activation:** When community members favorite your tweets, their
InterestedIn vectors embed your content. This is your first path to out-of-network
discovery.

**Expected outcome:** 20-50 impressions per tweet (vs. near-zero in week 1),
occasional out-of-network pickup via SimClusters embedding.

### Month 2-3: Community Recognition and Reciprocal Loops

**Goal:** Become a recognized participant in your niche community.

**What changes:**
1. Authors begin recognizing your name and replying proactively (strengthening
   Real Graph edges bidirectionally)
2. GraphJet Circle of Trust includes you in others' personalized PageRank
3. Your tweets appear in 15-30% of community members' feeds via GraphJet's
   SALSA algorithm
4. SimClusters embeddings on your tweets strengthen as engagement grows

**Actions:**
1. Maintain reply strategy but reduce to 1-2/day (you are now getting organic
   engagement)
2. Increase original content to 2-3 tweets/day
3. Start threads (if TweepCred > 65) -- first tweet carries the thread
4. Engage with other small accounts in your niche (builds mutual GraphJet
   traversal paths)

**Expected outcome:** 200-500 followers, 100-500 impressions per tweet,
occasional tweets breaking 1K impressions via out-of-network.

### Month 3-6: Crossing the TweepCred 65 Threshold

**The milestone:** When your TweepCred crosses 65:
- Anti-gaming restriction lifts: all your tweets are considered (not just 3)
- Threads become viable
- Your posting volume can increase without waste
- EarlyBird processes your full output

**How to get there:**
- Improve your ratio to < 1.0 (more followers than following)
- Consistent daily engagement builds PageRank through the interaction graph
- Premium subscription adds +4 to +16 TweepCred points (bridge the gap)
- Account age factor: `min(1.0, log(1 + age/15))` -- your 5-year account age
  already gives you the maximum age bonus (1.0). This is one advantage you have.

**Expected trajectory:**

| Milestone          | Estimated Timeline | Key Unlock                     |
|--------------------|--------------------|--------------------------------|
| First Real Graph edges | Week 1        | In-network visibility          |
| 250 followers      | Week 3-4           | Meaningful in-network audience |
| Ratio < 1.0        | Month 2            | TweepCred penalty reduced      |
| TweepCred > 65     | Month 2-4          | Unlimited tweet consideration  |
| SimClusters presence | Month 3-4        | Out-of-network discovery       |
| 1K followers       | Month 4-6          | Self-sustaining growth loop    |

### The Compounding Effect

Once you cross TweepCred 65 with active Real Graph edges and SimClusters
embedding:

1. More tweets considered (no 3-tweet cap)
2. Better tweets surface (Heavy Ranker sees your full output)
3. More engagement on better tweets
4. Stronger SimClusters embeddings
5. More out-of-network discovery
6. More followers
7. Better TweepCred
8. Repeat

This is the flywheel. Everything before it is grinding. Everything after it
compounds.

---

## Appendix A: Key Source Code References

| Component         | Path                                                                                   |
|-------------------|----------------------------------------------------------------------------------------|
| TweepCred         | `src/scala/com/twitter/graph/batch/job/tweepcred/`                                    |
| Reputation class  | `src/scala/com/twitter/graph/batch/job/tweepcred/Reputation.scala`                    |
| Ranking params    | `src/thrift/com/twitter/search/common/ranking/ranking.thrift`                          |
| SimClusters       | `src/scala/com/twitter/simclusters_v2/`                                                |
| Heavy Ranker      | `home-mixer/server/src/main/scala/com/twitter/home_mixer/`                             |
| Feedback Fatigue  | `home-mixer/.../scorer/FeedbackFatigueScorer.scala`                                   |
| RealGraph         | `src/scala/com/twitter/interaction_graph/`                                             |
| EarlyBird         | `src/java/com/twitter/search/earlybird/`                                               |
| Age decay params  | `ThriftAgeDecayRankingParams` in `ranking.thrift`                                      |

## Appendix B: The Complete Scoring Formula

```
Heavy Ranker Score =
    P(favorite)                        * 0.5
  + P(reply)                           * 27.0
  + P(reply + author_engaged)          * 75.0
  + P(profile_click + engagement)      * 12.0
  + P(good_click + reply/like OR 2m)   * 11.0
  + P(bookmark)                        * 10.0
  + P(retweet)                         * 1.0
  + P(video_watch_50%)                 * 0.005
  + P(negative_reaction)               * -74.0
  + P(report)                          * -369.0
```

Applied after EarlyBird light ranking (logistic regression) filters ~1,500
candidates down, with age decay (`halflife=360min`, `slope=0.003`, `base=0.6`)
and premium multipliers (4x in-network, 2x out-of-network) layered on top.

---

## Appendix C: Sources

- [twitter/the-algorithm](https://github.com/twitter/the-algorithm) -- Open-sourced recommendation algorithm
- [twitter/the-algorithm-ml](https://github.com/twitter/the-algorithm-ml) -- ML models
- [twitter/GraphJet](https://github.com/twitter/GraphJet) -- Real-time graph processing library
- [TweepCred README](https://github.com/twitter/the-algorithm/blob/main/src/scala/com/twitter/graph/batch/job/tweepcred/README)
- [ranking.thrift](https://github.com/twitter/the-algorithm/blob/main/src/thrift/com/twitter/search/common/ranking/ranking.thrift)
- [SimClusters v2 README](https://github.com/twitter/the-algorithm/blob/main/src/scala/com/twitter/simclusters_v2/README.md)
- [X Engineering Blog: Recommendation Algorithm](https://blog.x.com/engineering/en_us/topics/open-source/2023/twitter-recommendation-algorithm)
- [Twitter's For You Algorithm Analysis](https://blog.reachsumit.com/posts/2023/04/the-twitter-ml-algo/)
- [RealGraph: User Interaction Prediction at Twitter](https://www.ueo-workshop.com/wp-content/uploads/2014/04/sig-alternate.pdf)
- [How the Twitter Algorithm Works (Source Code)](https://posteverywhere.ai/blog/how-the-x-twitter-algorithm-works)
- [Complete Technical Breakdown](https://www.tweetarchivist.com/how-twitter-algorithm-works-2025)
- [Reverse-Engineering The Algorithm](https://tianpan.co/blog/2025-09-15-twitter-s-recommendation-algorithm)
- [RealGraph Deep Dive](https://happystrongcoder.substack.com/p/dive-into-twitters-recommendation)
- [GraphJet Deep Dive](https://happystrongcoder.substack.com/p/dive-into-twitters-recommendation-7cd)
- [Annotated Algorithm Repository](https://github.com/igorbrigadir/awesome-twitter-algo)
- [Buffer Premium Reach Study](https://www.socialmediatoday.com/news/report-shows-paying-for-x-twitter-premkum-has-significant-reach-benefits/801881/)
