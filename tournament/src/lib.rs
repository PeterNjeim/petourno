#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! Tournament (competition) generation.
//!
//! Provides functions to generate various tournament formats:
//! - N-Tuple Elimination (Knockout)
//!   - Repechage
//!     - Full
//!     - Quarter-Final
//!     - Double-Elimination
//!     - Two Third Place Finishers
//!     - Consolation Bracket
//!   - Stepladder
//!   - Nth Place Game
//!   - Split Start
//!   - Consolation
//! - Combination
//! - Multilevel
//! - Round-Robin
//!   - Straight
//!   - N-Tuple Split
//!   - Semi-
//!   - Circle Method
//!   - Berger Tables
//!   - Lombard
//! - Group (League, Division (Franchise), Conference, Extended)
//!   - Ladder
//!   - Pyramid
//! - N-Game Guarantee
//! - Consolation
//! - Compass Draw
//! - Best N Out Of 2N-1 (Best-Of-N)
//!   - Twice-To-Beat (try to generalise)
//! - Swiss
//!   - Dutch
//!   - Monrad
//!   - Accelerated (Accelerated Pairings)
//!   - Danish System
//!   - Grand Prix System
//!   - McMahon System
//!   - Amalfi System
//! - Pot
//! - Group
//! - Pair Events? (Duplicate Bridge)
//! - Poker?
//! - Shootout? (Poker)
//! - GSL (Global StarCraft II League (4-Player Double Elimination))
//! - Hybrid Elimination
//! - Points (Aggregate)
//!   - XP
//!   - HP
//! - Sudden Death
//! - Scheveningen
//! - Tour (Tennis)
//! - Select, Melee, and Panache
//! - Snake
//!
//!
//!
//! and features:
//! - Venues
//! - Handicapping
//! - Pools (Pool Play, Pool Stage)
//! - Groups (Group Stage)
//! - Qualifiers
//! - Waves
//! - Phases
//! - Extended Standings
//! - Draws
//! - Multi-Stage
//! - Playoffs (Postseason)
//!   - Page System (Same as Page-McIntyre, same as McIntyre Final Four)
//!   - McIntyre Systems (try to generalise)
//!     - Page-McIntyre
//!     - Final Five (Top Five)
//!     - First Final Six (Top Six)
//!     - Second Final Six (Top Six)
//!     - Final Eight (Top Eight)
//!   - AFL Final Eight (Top Eight) (try to generalise)
//!   - NRL System
//!   - Super League System (AFL + Club Call) (not going to be implemented)
//!   - Argus Systems (Not going to be implemented)
//!     - First
//!     - First Ammended
//!     - Second Ammended
//!     - Round-Robin
//!   - Shaughnessy (Same as Argus?)
//!   - Wild Cards
//! - Play-In
//! - Ranking Systems
//!   - Seeds
//!   - ELO
//!   - Glicko
//!   - Glicko2
//!   - Buchholz
//!   - Neustadtl
//!   - Sonneborn-Verger
//!   - Koya
//!   - Net Run Rate
//!   - Winning Percentage
//!   - Lots more lol
//! - Seeding
//!   - Skill
//!   - Region
//!   - Jacobian Ladders
//! - Reseeding
//! - Berths
//!   - Magic Number
//! - Byes
//! - Rankings
//! - Seasons
//!   - Preseason
//!   - Regular Season
//!   - Postseason
//!   - Offseason
//! - Promotion & Relegation
//! - Two-Legged Tie (try to generalise)
//! - Strength Of Schedule
//! - Tie-Breaking Systems
//!   - Buchholz
//!     - Cut 1
//!     - Median (Harkness)
//!       - Modified Median
//!     - Solkoff
//!   - Sonneborn-Berger (Neustadtl)
//!   - Cumulative (Golombek)
//!   - Result Between Tied Players (Head-To-Head)
//!   - Scoring Average (Ratio of Points Scored to Points Conceded)
//!   - Scoring Differential (Difference between Points and Points Conceded)
//!   - Points Scored
//!   - Points Scored When Away
//!   - Disciplinary Record (Least Fouls, etc.)
//!   - Seeding (Either Higher or Lower)
//!   - Koya
//!   - Sum Of Defeated Opponents' Scores (SODOS)
//!   - Sum Of Opponents' Scores (SOS)
//!   -
//!   - Most Games Away (Black Pieces)
//!   - Most Wins (Baumbach)
//!   - Kashdan
//!   - Opponent's Performance
//!   - Average Rating Of Opposition
//!   - Time Of Loss
//!   - Tardiness
//!   - Speed Playoff Games (Time Control, Fast Chess)
//!     - Armageddon
//!   - Coin Flip
//!   - USCF Recommended Order
//!   - NBA Order
//!   - etc.
//!   - Overtime (Extra Time)
//!   - Penalty Shootout
//!   - Golden Goal
//!   - Extra Game
//!   - Super Over (One Over Per Side Eliminator)
//!
//! and non-Tournament competitions:
//! - Challenge
//!   - Lineal
//! - Ladder (Pyramid)
//! - Selection
//!   - Vote
//!
//! Match fixing prevention
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!

pub mod generation;
