# X Algorithm Changes Timeline: January 2025 -- March 2026

Chronological record of algorithm changes, policy shifts, and platform updates
between the 2023 open-source release and the January 2026 open-source release.

Each entry: **date, event, source, impact assessment.**

---

## 2025 Q1: Negativity Crackdown and Link Suppression Escalation

### January 3--4, 2025 -- Musk Announces "Unregretted User-Seconds" Goal

Elon Musk posted: *"Algorithm tweak coming soon to promote more informational/entertaining content. We will publish the changes to @XEng. Our goal is to maximize unregretted user-seconds. Too much negativity is being pushed that technically grows user time, but not unregretted user time."*

He also publicly criticised X's own recommendation engine for surfacing "Nazi salute" content and excessive negativity, signalling a shift toward sentiment-aware ranking.

- **Source:** [Elon Musk on X (Jan 3, 2025)](https://x.com/elonmusk/status/1875355425601999255) | [Benzinga](https://www.benzinga.com/general/social-media/25/01/42805588/elon-musk-announces-changes-to-x-over-negativity-concerns-too-much-negativity-is-being-pushed) | [GV Wire](https://gvwire.com/2025/01/04/elon-musk-announces-algorithm-change-to-reduce-negativity-on-x/)
- **Impact:** HIGH. Marked the philosophical pivot from raw engagement maximisation to "quality time" optimisation. Grok sentiment scoring began affecting distribution -- positive/constructive posts started receiving wider reach, while combative tones were demoted even when engagement was high.

### January 2025 -- Premium vs. Non-Premium Engagement Divergence Begins

Internal data showed a structural split: Premium accounts began seeing rising engagement rates while non-Premium accounts saw declining rates. Premium subscribers received a 2--4x initial reach multiplier, with tweets shown to 40--80% of followers vs. 10--20% for free users.

- **Source:** [Tweet Archivist](https://www.tweetarchivist.com/how-twitter-algorithm-works-2025) | [Hashmeta](https://hashmeta.com/insights/twitter-algorithm-changes-2025) | [Buffer](https://buffer.com/resources/x-premium-review/)
- **Impact:** HIGH. Created a two-tier platform. Premium accounts averaged ~600 impressions/post vs. ~100 for free accounts -- a 10x reach multiplier.

### March 2025 -- Link Posts Hit Zero Median Engagement for Free Accounts

Buffer research confirmed that external link posts from free accounts reached zero median engagement. A/B testing showed a 1,700% reach increase when removing a link from an otherwise identical tweet. Links to competing platforms (Facebook, Instagram) faced even steeper penalties of -60%.

- **Source:** [Buffer](https://buffer.com/resources/links-on-x/) | [Hashmeta](https://hashmeta.com/insights/twitter-algorithm-changes-2025)
- **Impact:** CRITICAL. The harshest link suppression in X's history. Creators and publishers were forced to adopt "link in reply" strategies. Posts with multiple links saw -70% reduction.

### March 2025 -- Video Reactions Feature Launches

X introduced the ability to respond to any post (video, image, or text) with a video reaction.

- **Source:** [SocialBee](https://socialbee.com/blog/twitter-updates/) | [HeyOrca](https://www.heyorca.com/blog/x-twitter-social-news)
- **Impact:** LOW-MEDIUM. New engagement surface, but did not significantly alter algorithmic ranking.

---

## 2025 Q2: Video Prioritisation and Thread Algorithm Overhaul

### Q2 2025 -- Video Posts Receive 2--4x Algorithmic Boost

Video content began receiving 2--4x more reach than text or image posts. Vertical (9:16) format outperformed horizontal; optimal length settled at 15--60 seconds with captions.

- **Source:** [Hashmeta](https://hashmeta.com/insights/twitter-algorithm-changes-2025) | [Sprout Social](https://sproutsocial.com/insights/twitter-algorithm/)
- **Impact:** HIGH. Shifted content strategy industry-wide. Short-form vertical video became the highest-reach native format on X.

### Q2 2025 -- Threads Treated as Cohesive Units

Algorithm began treating threads as cohesive content units with completion rate as a ranking factor. 3--5 tweet threads achieved 40--60% higher total impressions than the same content split across individual standalone posts.

- **Source:** [Hashmeta](https://hashmeta.com/insights/twitter-algorithm-changes-2025) | [PostNext](https://postnext.io/blog/x-twitter-algorithm-explained/)
- **Impact:** MEDIUM. Rewarded serialised storytelling. Thread completion rate became a new optimisation target.

### Q2 2025 -- Community Notes Impact Formalised

Tweets tagged with Community Notes began receiving 60--80% reach reduction and removal from "For You" recommendations. Accounts accumulating multiple notes saw overall authority reduction affecting all future posts for weeks.

- **Source:** [Hashmeta](https://hashmeta.com/insights/twitter-algorithm-changes-2025) | [Sprout Social](https://sproutsocial.com/insights/twitter-algorithm/)
- **Impact:** HIGH. Community Notes became a de facto reach-killing mechanism, not just a fact-check overlay. Misinformation accounts suffered compound penalties.

---

## 2025 Q3: Grok AI Integration Announced

### Summer 2025 -- Explore Tab Integrated with Grok AI

The Explore tab began displaying "Grok Analysis" -- AI-written summaries explaining trending topics by synthesising context from thousands of posts, replacing the simple hashtag list.

- **Source:** [PostEverywhere](https://posteverywhere.ai/blog/how-the-x-twitter-algorithm-works) | [Sprout Social](https://sproutsocial.com/insights/twitter-algorithm/)
- **Impact:** MEDIUM. Changed how users discovered trending content but did not directly alter For You ranking.

### Summer 2025 -- Community Notes Reaches 1 Million Contributors

X announced Community Notes had reached 1 million contributors worldwide, significantly expanding the fact-checking network and the algorithm's ability to apply reach penalties.

- **Source:** [HeyOrca](https://www.heyorca.com/blog/x-twitter-social-news)
- **Impact:** MEDIUM. Wider contributor base meant more posts received notes, amplifying the reach-penalty effect.

### September 2025 -- Musk Announces Full Grok Takeover of Algorithm

Elon Musk posted: *"The algorithm will be purely AI by November, with significant progress along the way. We will open source the algorithm every two weeks or so. By November or certainly December, you will be able to adjust your feed dynamically just by asking Grok."*

This was the first announcement of "promptable feeds" -- the idea that users could give natural language instructions to Grok to customise their timeline.

- **Source:** [Elon Musk on X](https://x.com/elonmusk/status/1969081066578149547) | [Social Media Today](https://www.socialmediatoday.com/news/x-formerly-twitter-switching-to-fully-ai-powered-grok-algorithm/803174/) | [Medianama](https://www.medianama.com/2025/09/223-x-elon-musk-ai-driven-feed-user-prompts/)
- **Impact:** CRITICAL. Set the roadmap for the biggest architectural change in X's history -- replacing all heuristic-based ranking with a transformer model derived from Grok-1.

### Q3 2025 -- Thumbs-Up Engagement Icon A/B Test

X experimented with swapping the heart (like) icon with a thumbs-up icon across a subset of users.

- **Source:** [HeyOrca](https://www.heyorca.com/blog/x-twitter-social-news)
- **Impact:** LOW. UI experiment with no confirmed algorithmic change.

---

## 2025 Q4: The Grok Revolution

### October 14, 2025 -- External Link Penalty Officially Removed

X removed the algorithmic penalty on external links. Early data showed approximately 8x increase in link post reach and 13x increase in follower growth for previously penalised creators. Nikita Bier (X's head of product) announced a new "link experience" being tested on iOS with an in-app browser overlay.

- **Source:** [Medium (Karim2k)](https://medium.com/@karim2k/i-hated-most-of-elons-changes-but-removing-link-penalties-he-finally-got-one-right-873aa18d5025) | [Roboin](https://roboin.io/article/en/2025/10/20/x-product-chief-denies-rumor-that-link-posts-are-deboosted/) | [Tomorrow's Publisher](https://tomorrowspublisher.today/content-creation/x-softens-stance-on-external-links/)
- **Impact:** HIGH. Reversed nearly two years of link suppression. Publishers and newsletter creators saw immediate reach recovery. However, the effect was short-lived for free accounts -- by March 2026, non-Premium link posts were again seeing near-zero engagement.

### October 2025 -- Musk Announces Grok Transition: "Deletion of All Heuristics Within 4--6 Weeks"

Musk announced that Grok would replace the legacy recommendation system entirely. The plan: Grok would "read every post and watch every video" (100M+ per day) to match users with content. All hand-engineered features and heuristics would be deleted.

- **Source:** [ROIC News](https://www.roic.ai/news/elon-musk-apologizes-for-x-algorithm-issues-as-platform-shifts-to-grok-ai-10-24-2025) | [Social Media Today](https://www.socialmediatoday.com/news/x-formerly-twitter-switching-to-fully-ai-powered-grok-algorithm/803174/)
- **Impact:** CRITICAL. This was the death sentence for the legacy Java/Scala recommendation stack that had been in use since pre-Musk Twitter.

### October 24, 2025 -- Musk Publicly Apologises for Algorithm Problems

Elon Musk posted: *"My apologies for frustrations with the X algorithm"* and *"We are working hard to fix the problem."* This came amid widespread user complaints about feed quality during the transition period.

- **Source:** [ROIC News](https://www.roic.ai/news/elon-musk-apologizes-for-x-algorithm-issues-as-platform-shifts-to-grok-ai-10-24-2025) | [FinancialContent](https://markets.financialcontent.com/stocks/article/tokenring-2025-10-24-elon-musk-grapples-with-xs-algorithmic-quandaries-apologizes-to-users)
- **Impact:** MEDIUM. Rare public acknowledgement of algorithm problems. Signalled that the Grok transition was causing visible user-facing issues.

### November 27, 2025 -- Following Feed Becomes Grok-Sorted

X launched AI-powered ranking in the "Following" feed, using Grok to analyse and rank posts from accounts users follow based on "predicted engagement and relevance." This meant neither "For You" nor "Following" showed content chronologically by default. Users could toggle back to chronological order.

- **Source:** [Social Media Today](https://www.socialmediatoday.com/news/x-formerly-twitter-sorts-following-feed-algorithm-ai-grok/806617/) | [EditorialGE](https://editorialge.com/elon-musk-x-grok-smart-following-feed-launch/)
- **Impact:** CRITICAL. Major backlash. The "Following" tab had been the last bastion of chronological content on X. Users who relied on it for real-time news felt betrayed.

### December 5, 2025 -- EU Fines X EUR120 Million Under Digital Services Act

The European Commission imposed a EUR120 million fine -- the first DSA enforcement action ever -- citing three violations:

1. **Deceptive blue checkmark** -- anyone can buy "verified" status, making it difficult to judge account authenticity. Verified replies are algorithmically boosted, meaning users "pay for reach."
2. **Advertising repository** -- missing content/topic information and advertiser legal entities.
3. **Researcher data access** -- X made it "unduly difficult" for researchers to access public data.

Deadlines: 60 working days for verification fix; 90 working days for ad transparency and researcher access action plan.

- **Source:** [European Commission](https://digital-strategy.ec.europa.eu/en/news/commission-fines-x-eu120-million-under-digital-services-act) | [TechCrunch](https://techcrunch.com/2025/12/05/in-its-first-dsa-penalty-eu-fines-x-e120m-for-deceptive-blue-check-verification-system/) | [IAPP](https://iapp.org/news/a/european-commission-fines-x-120m-euros-for-dsa-violations/)
- **Impact:** CRITICAL. First-ever DSA fine. Directly linked Premium's algorithmic boost to regulatory violations. Created pressure that likely accelerated the January 2026 open-source release.

### 2025 Full Year -- Platform Engagement Metrics

Average engagement rate dropped to 0.12% (down 48% year-over-year) -- the steepest decline of any social platform. X's share of worldwide digital ad spending fell to 0.2% (Facebook: 14.6%, TikTok: 7.1%). X had lost approximately $5.9 billion in ad revenue since Musk's acquisition.

- **Source:** [Enrich Labs](https://www.enrichlabs.ai/blog/twitter-x-benchmarks-2025) | [Social Insider](https://www.socialinsider.io/social-media-benchmarks) | [Buffer](https://buffer.com/resources/average-engagement-rate/)
- **Impact:** CONTEXT. The algorithm changes occurred against a backdrop of declining platform health.

---

## 2026 January: The Open-Source Release

### January 10, 2026 -- Musk Announces Open-Source in 7 Days

Elon Musk posted: *"We will make the new X algorithm, including all code used to determine what organic and advertising posts are recommended to users, open source in 7 days. This will be repeated every 4 weeks, with comprehensive developer notes, to help you understand what changed."*

- **Source:** [Elon Musk on X](https://x.com/elonmusk/status/2010062264976736482) | [Bloomberg](https://www.bloomberg.com/news/articles/2026-01-10/elon-musk-says-x-to-make-its-algorithm-open-source-in-seven-days) | [PhoneArena](https://www.phonearena.com/news/elon-musk-says-the-new-x-algorithm-will-be-made-open-source-in-a-week_id177219) | [Gizmodo](https://gizmodo.com/elon-musk-says-in-one-week-he-will-fully-reveal-why-your-x-timeline-is-like-that-2000708652)
- **Impact:** HIGH. Set expectations for full transparency. Promise of monthly updates with developer notes was unprecedented for a major social platform.

### January 20, 2026 -- Algorithm Open-Sourced on GitHub

X Engineering posted: *"We have open-sourced our new X algorithm, powered by the same transformer architecture as xAI's Grok model."*

Repository: `xai-org/x-algorithm` on GitHub, Apache 2.0 licence. 1,600 stars within 6 hours. Codebase: 62.9% Rust, 37.1% Python.

**Four components released:**

| Component | Role |
|---|---|
| **Home Mixer** | Orchestration layer -- coordinates pipeline, assembles final feed |
| **Thunder** | In-memory post store with Kafka ingestion; sub-millisecond lookups for in-network content |
| **Phoenix** | Grok-based transformer model; dual-function retrieval (two-tower neural network) and ranking (engagement probability prediction) |
| **Candidate Pipeline** | Reusable framework defining pipeline stages; parallel execution with configurable error handling |

**Pipeline stages:** Candidate gathering (1,500 from 500M) -> Information enrichment -> Filtering -> Scoring (15 engagement types predicted) -> Selection.

**15 predicted engagement types:** Favourites, replies, reposts, quotes, clicks, profile clicks, video views, photo expansion, shares, dwell time, author follows, plus negative signals (not interested, blocks, mutes, reports).

**Author diversity scoring** limits repeated appearances from single accounts.

- **Source:** [X Engineering on X](https://x.com/XEng/status/2013471689087086804) | [GitHub](https://github.com/xai-org/x-algorithm) | [PPC Land](https://ppc.land/xs-algorithm-source-code-drops-what-it-reveals-about-the-platforms-feed-mechanics/) | [VentureBeat](https://venturebeat.com/data/x-open-sources-its-algorithm-5-ways-businesses-can-benefit/) | [TechCrunch](https://techcrunch.com/2026/01/20/x-open-sources-its-algorithm-while-facing-a-transparency-fine-and-grok-controversies/)
- **Impact:** CRITICAL. First time a Grok-powered recommendation system was open-sourced. However, critical omissions (see below) undermined the transparency claims.

### January 20, 2026 -- What Was NOT Released (Critical Omissions)

| Missing Element | Why It Matters |
|---|---|
| **Engagement weight coefficients** | The numerical weights applied to each of the 15 signals were redacted. No way to know if a reply is worth 2x or 10x a like. |
| **Trained model weights** | Architecture released, but not the transformer weights -- cannot reproduce or audit predictions. |
| **Training data** | No information about what data trained the model. |
| **Advertising integration** | Despite Musk's promise to include ad code, no documentation on sponsored content ranking, auction mechanics, or advertiser objectives. |
| **Premium boost mechanism** | No mention of Premium subscription integration in released code. |
| **Safety/policy pipelines** | Labelling, evaluation, and content moderation logic excluded. |
| **Infrastructure specs** | Kafka streaming requirements, compute resources, deployment instructions absent. |

X stated engagement weights were excluded *"for security reasons."*

Compared to the 2023 release, which disclosed weights like reply = 27 retweets and author-reply = 75 retweets, the 2026 release was more architecturally complete but less transparent about the actual ranking mechanics.

- **Source:** [PPC Land](https://ppc.land/xs-algorithm-source-code-drops-what-it-reveals-about-the-platforms-feed-mechanics/) | [Typefully](https://typefully.com/blog/x-algorithm-open-source) | [Engadget](https://www.engadget.com/social-media/xs-open-source-algorithm-isnt-a-win-for-transparency-researchers-say-181836233.html)
- **Impact:** HIGH. Undermined the transparency narrative. Researchers and developers could not audit, reproduce, or independently verify the algorithm's behaviour.

---

## 2026 February: Researcher Backlash and Nature Study

### February 5, 2026 -- Researchers Label Release a "Redacted Version"

John Thickstun (Cornell University): *"What troubles me about these releases is that they give you a pretense that they're being transparent for releasing code and the sense that someone might be able to use this release to do some kind of auditing work or oversight work. And the fact is that that's not really possible at all."*

Community code analysis found that verified accounts receive a significantly higher scoring ceiling (up to +100) compared to unverified accounts (max +55), but researchers deemed most circulating claims about the algorithm "unsubstantiated" since they stemmed from "partial code views, not comprehensive evidence."

- **Source:** [Dataconomy](https://dataconomy.com/2026/02/05/researchers-label-recent-x-algorithm-release-as-a-redacted-version/) | [Engadget](https://www.engadget.com/social-media/xs-open-source-algorithm-isnt-a-win-for-transparency-researchers-say-181836233.html)
- **Impact:** HIGH. Established the academic consensus that the open-source release was transparency theatre, not genuine accountability.

### February 5, 2026 -- Grok Engagement Model Details Confirmed

Technical analysis confirmed the new algorithm uses a "Grok-like model to predict user engagement," eliminating all hand-engineered heuristics in favour of a single transformer predicting interaction probabilities across 15 engagement types.

- **Source:** [TechBriefly](https://techbriefly.com/2026/02/05/new-x-algorithm-uses-grok-like-model-to-predict-user-engagement/)
- **Impact:** MEDIUM. Confirmed the technical architecture but without weights, the confirmation was of limited practical value.

### February 2026 -- Communities Go Public

X's Communities feature went public: community posts became visible to all users and began surfacing in the "For You" feed, no longer restricted to community members.

- **Source:** [SocialBee](https://socialbee.com/blog/twitter-updates/) | [PostEverywhere](https://posteverywhere.ai/blog/how-the-x-twitter-algorithm-works)
- **Impact:** MEDIUM. Expanded content pool for the recommendation algorithm. Community posts entered the same ranking pipeline as regular tweets.

### February 2026 -- Nature Publishes X Algorithm Political Effects Study

A major peer-reviewed study in Nature (based on a 2023 field experiment with 4,965 US users) found:

- Switching from chronological to algorithmic feed shifted political opinion toward more conservative positions
- Conservative posts were ~20% more likely to appear in algorithmic feeds
- Traditional news appeared ~58% fewer times in algorithmic feeds
- Posts by political activists appeared ~27.4% more often
- The algorithm nudged users to follow more right-leaning accounts, and these following patterns persisted even after switching back to chronological
- Switching from algorithmic to chronological had no comparable reverse effect

- **Source:** [Nature](https://www.nature.com/articles/s41586-026-10098-2) | [Phys.org](https://phys.org/news/2026-02-algorithmic-shifting-political-views-conservatism.html) | [Gizmodo](https://gizmodo.com/researchers-find-that-xs-algorithm-can-push-users-to-lean-more-conservative-2000723017) | [The Debrief](https://thedebrief.org/xs-for-you-algorithm-may-be-able-to-shift-political-views-permanently-new-study-finds/)
- **Impact:** CRITICAL. First peer-reviewed evidence of lasting political attitude shifts from X's algorithm. Generated major media coverage and regulatory scrutiny.

---

## 2026 March: Ongoing Evolution

### March 2026 -- Non-Premium Link Posts Return to Near-Zero Engagement

Despite the October 2025 link penalty removal, non-Premium accounts posting external links again showed near-zero median engagement. The suppression appeared to have returned for free accounts, while Premium accounts retained link reach.

- **Source:** [PostEverywhere](https://posteverywhere.ai/blog/how-the-x-twitter-algorithm-works) | [SocialWick](https://www.socialwick.com/decoding-the-new-x-algorithm-to-stay-visible-in-2026)
- **Impact:** HIGH. Suggests the link penalty was either re-introduced silently for non-Premium accounts, or the October 2025 removal only applied to Premium subscribers.

### March 2026 -- Region Conversation Control Launched

X introduced the ability for users to restrict replies to posts based on geographic regions or countries, aiming to reduce spam and unwanted international replies.

- **Source:** [SocialWick](https://www.socialwick.com/decoding-the-new-x-algorithm-to-stay-visible-in-2026)
- **Impact:** MEDIUM. New tool for controlling conversations, with indirect algorithmic effects on reply distribution.

### March 26, 2026 -- Nikita Bier Announces "Biggest Grok Algorithm Change Yet"

X hired Benji Taylor as new design lead. Nikita Bier (head of product) announced what he called X's biggest Grok algorithm change yet, expected the following week.

- **Source:** [PiunikaWeb](https://piunikaweb.com/2026/03/26/x-benji-taylor-design-lead-grok-algorithm-change/)
- **Impact:** TBD. Signals continued rapid iteration on the Grok-powered system.

### March 2026 -- In-App Browser for Links Being Tested

X is testing a new in-app browser that keeps engagement buttons and the original post visible when users click links, rather than opening in an external browser. This addresses the structural reason links were penalised -- they took users off-platform.

- **Source:** [Social Media Today](https://www.socialmediatoday.com/news/x-formerly-twitter-testing-links-in-app-link-post-penalties/803176/)
- **Impact:** MEDIUM. If deployed, could permanently resolve the link penalty by keeping users inside X while allowing link content.

---

## Elon Musk Statements About the Algorithm (Collected)

| Date | Statement | Source |
|---|---|---|
| Jan 3, 2025 | *"Algorithm tweak coming soon... Our goal is to maximize unregretted user-seconds."* | [X post](https://x.com/elonmusk/status/1875355425601999255) |
| Sep 2025 | *"The algorithm will be purely AI by November... you will be able to adjust your feed dynamically just by asking Grok."* | [X post](https://x.com/elonmusk/status/1969081066578149547) |
| Oct 2025 | Announced Grok replacing legacy system: *"deletion of all heuristics within 4-6 weeks"* | [Social Media Today](https://www.socialmediatoday.com/news/x-formerly-twitter-switching-to-fully-ai-powered-grok-algorithm/803174/) |
| Oct 24, 2025 | *"My apologies for frustrations with the X algorithm. We are working hard to fix the problem."* | [ROIC News](https://www.roic.ai/news/elon-musk-apologizes-for-x-algorithm-issues-as-platform-shifts-to-grok-ai-10-24-2025) |
| Jan 10, 2026 | *"We will make the new X algorithm... open source in 7 days. This will be repeated every 4 weeks."* | [X post](https://x.com/elonmusk/status/2010062264976736482) |

---

## Known Undocumented Features and Server-Side Unknowns

### Confirmed in Code but Not Documented

- **Verified account scoring ceiling:** +100 max for verified (Premium) vs. +55 max for unverified. This acts as a hard cap before engagement scoring even begins.
- **Author diversity limit:** Explicit throttling of how many posts from a single account appear in any user's feed.
- **Hard age cutoff:** Posts are completely filtered after a certain time threshold -- not a gradual decay.
- **Sentiment analysis via Grok:** Every post's tone is scored. Positive/constructive posts get wider distribution; negative/combative posts get reduced visibility even with high engagement.
- **Volume attenuation:** High-frequency posting triggers "systematic score attenuation designed to maintain variety."

### Missing from Open-Source Code (Server-Side Only)

- **Exact engagement weight coefficients** -- the numbers that determine how much each of the 15 signals matters relative to each other. Redacted "for security reasons."
- **Trained transformer weights** -- the actual model parameters. Architecture is public; the trained model is not.
- **TweepCred reputation scores** -- the 0--100 PageRank-based account authority score. Below 65, only 3 tweets are considered for distribution. The scoring system exists in the legacy codebase but its integration with the new Grok system is undocumented.
- **Premium tier boost multipliers** -- the exact 4x in-network / 2x out-of-network numbers are community-measured, not in the released code.
- **Advertising auction and sponsored content ranking** -- entirely absent despite promises.
- **Regional content weighting** -- location affects what users see, especially around local events, but parameters are server-side only.
- **Safety model pipelines** -- content moderation, labelling, and enforcement logic all excluded.
- **A/B test configurations** -- active experiments and their parameters are not in the public repository.

### Community-Measured Values (Not in Code)

These engagement weights circulate widely but are inferred from observation and the 2023 release, not confirmed in the 2026 code:

| Signal | Estimated Weight | Confidence |
|---|---|---|
| Likes | 1x (baseline) | High (from 2023 code) |
| Retweets | 20x | Medium (2023 code; may have changed) |
| Replies | 13.5x | Medium |
| Profile Clicks | 12x | Medium |
| Link Clicks | 11x | Medium |
| Bookmarks | 10x | Medium |
| Author reply to reply | +75 weight (~150x a like) | Low-Medium (2023 code) |

**Key caveat:** X redacted these values from the 2026 release. The numbers above are from the 2023 open-source release and community measurement. The Grok-based system likely uses entirely different weighting via learned transformer parameters rather than hand-set coefficients.

---

## Summary: Architecture Shift 2023 vs. 2026

| Aspect | 2023 Release | 2026 Release |
|---|---|---|
| **Language** | Java/Scala | Rust/Python |
| **Ranking engine** | Hand-engineered heuristics + ML | Grok transformer (all ML) |
| **Engagement weights** | Published (like=1, retweet=20, etc.) | Redacted |
| **Model weights** | Not published | Not published |
| **Training data** | Not published | Not published |
| **Ad integration** | Partially documented | Not documented |
| **Premium boost** | Partially visible | Not in code |
| **Licence** | MIT | Apache 2.0 |
| **Update cadence** | One-time release | Promised every 4 weeks |
| **Repository** | `twitter/the-algorithm` | `xai-org/x-algorithm` |

---

*Last updated: 2026-03-27. Sources verified against web archives as of research date.*
