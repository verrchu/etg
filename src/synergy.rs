use super::gun::Gun;
use super::item::Item;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
enum Part {
    Gun(Gun),
    Item(Item),
}

#[derive(Debug, Serialize, Deserialize)]
struct Repr {
    name: String,
    effect: String,
    parts: Parts,
}

#[derive(Debug, Serialize, Deserialize)]
enum Parts {
    Single(Part),
    OneOf(Vec<Part>),
    TwoOf(Vec<Part>),
    AllOf(Vec<Part>),
    Combined(Box<Parts>, Box<Parts>),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
enum Synergy {
    #[serde(rename = "Absent Minded")]
    AbsentMinded,
    #[serde(rename = "Added Effect - Fire")]
    AddedEffectFire,
    #[serde(rename = "Added Effect - Ice")]
    AddedEffectIce,
    #[serde(rename = "Added Effect - Poison")]
    AddedEffectPoison,
    #[serde(rename = "Akey Breaky")]
    _AkeyBreaky,
    #[serde(rename = "AI Assistant")]
    _AIAssistant,
    #[serde(rename = "Air Support")]
    _AirSupport,
    #[serde(rename = "AK-47 (Island Forme)")]
    _AK47IslandForme,
    #[serde(rename = "Alas, Sniperion")]
    _AlasSniperion,
    #[serde(rename = "Alistair's Ladder")]
    _AlistairsLadder,
    #[serde(rename = "Ancient Aliens")]
    _AncientAliens,
    #[serde(rename = "Anti-Aircraft")]
    _AntiAircraft,
    #[serde(rename = "Antichamber")]
    _Antichamber,
    #[serde(rename = "Apiary")]
    _Apiary,
    #[serde(rename = "Assault and Battery")]
    _AssaultAndBattery,
    #[serde(rename = "Armored Corps")]
    _ArmoredCorps,
    #[serde(rename = "Bacon and Eggs")]
    _BaconAndEggs,
    #[serde(rename = "Banshee Cry")]
    _BansheeCry,
    #[serde(rename = "Barrage Shot")]
    _BarrageShot,
    #[serde(rename = "Bee Plus")]
    _BeePlus,
    #[serde(rename = "Behold!")]
    _Behold,
    #[serde(rename = "Beta Ray")]
    _BetaRay,
    #[serde(rename = "Betrayer's Lies")]
    _BetrayersLies,
    #[serde(rename = "Big Shotgun Gun")]
    _BigShotgunGun,
    #[serde(rename = "Birthright")]
    _Birthright,
    #[serde(rename = "Blade Runner")]
    _BladeRunner,
    #[serde(rename = "Blammo!")]
    _Blammo,
    #[serde(rename = "Blank Generation")]
    _BlankGeneration,
    #[serde(rename = "Blast Crown")]
    _BlastCrown,
    #[serde(rename = "Blessing and a Curse")]
    _BlessingandaCurse,
    #[serde(rename = "Block Party")]
    _BlockParty,
    #[serde(rename = "Blunderbrace")]
    _Blunderbrace,
    #[serde(rename = "Bomberpal")]
    _Bomberpal,
    #[serde(rename = "Breakfast Club")]
    _BreakfastClub,
    #[serde(rename = "Buggin' Out")]
    _BugginOut,
    #[serde(rename = "Bullet or the Egg")]
    _BulletortheEgg,
    #[serde(rename = "Bubble Trouble")]
    _BubbleTrouble,
    #[serde(rename = "Buckle Up")]
    _BuckleUp,
    #[serde(rename = "Buster Gun")]
    _BusterGun,
    #[serde(rename = "Camera Shy")]
    _CameraShy,
    #[serde(rename = "Captain Plant It")]
    _CaptainPlantIt,
    #[serde(rename = "Cash Rules Everygun Around Me")]
    _CashRulesEverygunAroundMe,
    #[serde(rename = "Cat Friend")]
    _CatFriend,
    #[serde(rename = "Chain Lightning")]
    _ChainLightning,
    #[serde(rename = "Chance On Hit")]
    _ChanceOnHit,
    #[serde(rename = "Chest High is the Best High")]
    _ChestHighIstheBestHigh,
    #[serde(rename = "Circle of Bricks")]
    _CircleOfBricks,
    #[serde(rename = "Clawntlet")]
    _Clawntlet,
    #[serde(rename = "Cold 1896")]
    _Cold1896,
    #[serde(rename = "Cold Ones")]
    _ColdOnes,
    #[serde(rename = "Comammo Belt")]
    _ComammoBelt,
    #[serde(rename = "Contractual Obligation")]
    _ContractualObligation,
    #[serde(rename = "Cool Kids Club")]
    _CoolKidsClub,
    #[serde(rename = "Cormorant")]
    _Cormorant,
    #[serde(rename = "Cosmic Horror")]
    _CosmicHorror,
    #[serde(rename = "Crave the Glaive")]
    _CraveTheGlaive,
    #[serde(rename = "Crestfallen Soul")]
    _CrestfallenSoul,
    #[serde(rename = "Criticaliber")]
    _Criticaliber,
    #[serde(rename = "Cryptic Cryptids")]
    _CrypticCryptids,
    #[serde(rename = "Crystal Horn")]
    _CrystalHorn,
    #[serde(rename = "Dead Cell")]
    _DeadCell,
    #[serde(rename = "Dead Place")]
    _DeadPlace,
    #[serde(rename = "Dead Prez")]
    _DeadPrez,
    #[serde(rename = "Deadly Distraction")]
    _DeadlyDistraction,
    #[serde(rename = "Deadlier Distraction")]
    _DeadlierDistraction,
    #[serde(rename = "Death Square")]
    _DeathSquare,
    #[serde(rename = "Decorated")]
    _Decorated,
    #[serde(rename = "Detective Magnum")]
    _DetectiveMagnum,
    #[serde(rename = "Detective Mode")]
    _DetectiveMode,
    #[serde(rename = "Devil's Plaything")]
    _DevilsPlaything,
    #[serde(rename = "Diamond Weapon")]
    _DiamondWeapon,
    #[serde(rename = "Don't Hoot the Messenger")]
    _DontHootTheMessenger,
    #[serde(rename = "Double Down")]
    _DoubleDown,
    #[serde(rename = "Double Double Helix")]
    _DoubleDoubleHelix,
    #[serde(rename = "Double Rainbow")]
    _DoubleRainbow,
    #[serde(rename = "Dr Worm")]
    _DrWorm,
    #[serde(rename = "Dragunice")]
    _Dragunice,
    #[serde(rename = "Dualing Pistol")]
    _DualingPistol,
    #[serde(rename = "Dumb Smart Bullets")]
    _DumbSmartBullets,
    #[serde(rename = "Dwarven Stout")]
    _DwarvenStout,
    #[serde(rename = "EGG MURDERER")]
    _EGGMURDERER,
    #[serde(rename = "Elder Blank Bullets")]
    _ElderBlankBullets,
    #[serde(rename = "Emerald Weapon")]
    _EmeraldWeapon,
    #[serde(rename = "Emergency Help")]
    _EmergencyHelp,
    #[serde(rename = "Empty Vessels")]
    _EmptyVessels,
    #[serde(rename = "Enter the Fruitgeon")]
    _EntertheFruitgeon,
    #[serde(rename = "Extreme Operation")]
    _ExtremeOperation,
    #[serde(rename = "Fairy Bow")]
    _FairyBow,
    #[serde(rename = "Fightknives")]
    _Fightknives,
    #[serde(rename = "Fightsabre Training")]
    _FightsabreTraining,
    #[serde(rename = "Flat Stanley")]
    _FlatStanley,
    #[serde(rename = "Friend to Gun and Bullet")]
    _FriendToGunAndBullet,
    #[serde(rename = "Frosty Distraction")]
    _FrostyDistraction,
    #[serde(rename = "Frostier Distraction")]
    _FrostierDistraction,
    #[serde(rename = "Frosty the Bullet Tyrant")]
    _FrostyTheBulletTyrant,
    #[serde(rename = "Full Circle")]
    _FullCircle,
    #[serde(rename = "Garbage Collecting")]
    _GarbageCollecting,
    #[serde(rename = "Get Equipped With -A-")]
    _GetEquippedWithA,
    #[serde(rename = "Get Equipped With -C-")]
    _GetEquippedWithC,
    #[serde(rename = "Get Equipped With -H-")]
    _GetEquippedWithH,
    #[serde(rename = "Get Equipped With -M-")]
    _GetEquippedWithM,
    #[serde(rename = "Get Equipped With -Q-")]
    _GetEquippedWithQ,
    #[serde(rename = "Get Equipped With -W-")]
    _GetEquippedWithW,
    #[serde(rename = "Gilded Tables")]
    _GildedTables,
    #[serde(rename = "Green Thumb")]
    _GreenThumb,
    #[serde(rename = "Grouch")]
    _Grouch,
    #[serde(rename = "Gruber's Bane")]
    _GrubersBane,
    #[serde(rename = "Gunderlord")]
    _Gunderlord,
    #[serde(rename = "Gungeonite")]
    _Gungeonite,
    #[serde(rename = "Gunnerang")]
    _Gunnerang,
    #[serde(rename = "Guns and Deviltry")]
    _GunsAndDeviltry,
    #[serde(rename = "Hacker")]
    _Hacker,
    #[serde(rename = "Hail, Satan!")]
    _HailSatan,
    #[serde(rename = "Hail To The King")]
    _HailToTheKing,
    #[serde(rename = "Hammer and Anvil")]
    _HammerAndAnvil,
    #[serde(rename = "Hammer and Nail")]
    _HammerAndNail,
    #[serde(rename = "Hard Light")]
    _HardLight,
    #[serde(rename = "Heads Up")]
    _HeadsUp,
    #[serde(rename = "Heart-Shaped Box")]
    _HeartShapedBox,
    #[serde(rename = "Heavy Artillery")]
    _HeavyArtillery,
    #[serde(rename = "Heavy Jolt")]
    _HeavyJolt,
    #[serde(rename = "Hidden Tech Big Shotgun")]
    _HiddenTechBigShotgun,
    #[serde(rename = "Hidden Tech Flare")]
    _HiddenTechFlare,
    #[serde(rename = "Holy Bell")]
    _HolyBell,
    #[serde(rename = "Hornucopia")]
    _Hornucopia,
    #[serde(rename = "Hot Head")]
    _HotHead,
    #[serde(rename = "Hot Ones")]
    _HotOnes,
    #[serde(rename = "Hot Rolls")]
    _HotRolls,
    #[serde(rename = "Hotter Than Heck")]
    _HotterThanHeck,
    #[serde(rename = "http://merch.devolverdigital.com")]
    _HttpMerchDevolverDigitalCom,
    #[serde(rename = "Human Shield")]
    _HumanShield,
    #[serde(rename = "Hunter-Seeker Tables")]
    _HunterSeekerTables,
    #[serde(rename = "Hydro Pump")]
    _HydroPump,
    #[serde(rename = "Hyper Beam")]
    _HyperBeam,
    #[serde(rename = "I Studied The Blade")]
    _IStudiedTheBlade,
    #[serde(rename = "Ice Beam")]
    _IceBeam,
    #[serde(rename = "Ice Water")]
    _IceWater,
    #[serde(rename = "In The Mood!")]
    _InTheMood,
    #[serde(rename = "Inconsistent Scale")]
    _InconsistentScale,
    #[serde(rename = "Insight")]
    _Insight,
    #[serde(rename = "It's Your Destiny")]
    _ItsYourDestiny,
    #[serde(rename = "J am")]
    _Jam,
    #[serde(rename = "Jolly Roger")]
    _JollyRoger,
    #[serde(rename = "Jotun Time")]
    _JotunTime,
    #[serde(rename = "Jumping... is useless")]
    _JumpingIsUseless,
    #[serde(rename = "Junk Mail")]
    _JunkMail,
    #[serde(rename = "Just In Case")]
    _JustInCase,
    #[serde(rename = "Just Shoot Me")]
    _JustShootMe,
    #[serde(rename = "Kage Bunshin")]
    _KageBunshin,
    #[serde(rename = "Keep The Change")]
    _KeepTheChange,
    #[serde(rename = "Keeping The Beat")]
    _KeepingTheBeat,
    #[serde(rename = "Key Witness")]
    _KeyWitness,
    #[serde(rename = "The Killing Joke")]
    _TheKillingJoke,
    #[serde(rename = "Kinjutsu")]
    _Kinjutsu,
    #[serde(rename = "Knight Time")]
    _KnightTime,
    #[serde(rename = "Kung Fu Hippie, Rappin' Surfer")]
    _KungFuHippieRappinSurfer,
    #[serde(rename = "Land Among The Stars")]
    _LandAmongTheStars,
    #[serde(rename = "Laser Light Show")]
    _LaserLightShow,
    #[serde(rename = "Liches Get Stitches")]
    _LichesGetStitches,
    #[serde(rename = "Like Shooting Fish")]
    _LikeShootingFish,
    #[serde(rename = "Love/Hate")]
    _LoveHate,
    #[serde(rename = "Lumberjacked")]
    _Lumberjacked,
    #[serde(rename = "Mad Cats")]
    _MadCats,
    #[serde(rename = "Magazine Clips")]
    _MagazineClips,
    #[serde(rename = "Mass Cutter")]
    _MassCutter,
    #[serde(rename = "Meltdown")]
    _Meltdown,
    #[serde(rename = "Mine Craft")]
    _MineCraft,
    #[serde(rename = "Mirror Shield")]
    _MirrorShield,
    #[serde(rename = "MM6 Mini Rocket")]
    _MM6MiniRocket,
    #[serde(rename = "Mmmmmmmmm MMMMmm!")]
    _MmmmmmmmmMMMMmm,
    #[serde(rename = "Monsters and Monocles")]
    _MonstersandMonocles,
    #[serde(rename = "Moonplanet")]
    _Moonplanet,
    #[serde(rename = "Mr. Shadow")]
    _MrShadow,
    #[serde(rename = "Monster Grub")]
    _MonsterGrub,
    #[serde(rename = "Napalm B")]
    _NapalmB,
    #[serde(rename = "No Odd Job")]
    _NoOddJob,
    #[serde(rename = "One Fish Two Fish")]
    _OneFishTwoFish,
    #[serde(rename = "Orrery")]
    _Orrery,
    #[serde(rename = "Outer Limits")]
    _OuterLimits,
    #[serde(rename = "Over Dose")]
    _OverDose,
    #[serde(rename = "Paper Lanterns")]
    _PaperLanterns,
    #[serde(rename = "Parchmental")]
    _Parchmental,
    #[serde(rename = "Particle Accelerator")]
    _ParticleAccelerator,
    #[serde(rename = "Pilot Wingsman")]
    _PilotWingsman,
    #[serde(rename = "Plant Power")]
    _PlantPower,
    #[serde(rename = "Plasma Beam")]
    _PlasmaBeam,
    #[serde(rename = "Porkulent")]
    _Porkulent,
    #[serde(rename = "Poseigun")]
    _Poseigun,
    #[serde(rename = "Powerhouse of the Cell")]
    _PowerhouseOftheCell,
    #[serde(rename = "Praise the Gun")]
    _PraisetheGun,
    #[serde(rename = "Primitive Shapes")]
    _PrimitiveShapes,
    #[serde(rename = "Pulp Gungeon")]
    _PulpGungeon,
    #[serde(rename = "Queen of Hearts")]
    _QueenOfHearts,
    #[serde(rename = "Quick and the Gundead")]
    _QuickAndTheGundead,
    #[serde(rename = "RAIDEN")]
    _RAIDEN,
    #[serde(rename = "Raizo's Stuff")]
    _RaizosStuff,
    #[serde(rename = "Recycling Bin")]
    _RecyclingBin,
    #[serde(rename = "The Red Hood")]
    _TheRedHood,
    #[serde(rename = "Regular Old Guns")]
    _RegularOldGuns,
    #[serde(rename = "Resourceful Indeed,")]
    _ResourcefulIndeed,
    #[serde(rename = "Revolution")]
    _Revolution,
    #[serde(rename = "Rubenstein's Monster")]
    _RubensteinsMonster,
    #[serde(rename = "Ruby Weapon")]
    _RubyWeapon,
    #[serde(rename = "Running Blades")]
    _RunningBlades,
    #[serde(rename = "Rabid")]
    _Rabid,
    #[serde(rename = "Radiant")]
    _Radiant,
    #[serde(rename = "Ravishing")]
    _Ravishing,
    #[serde(rename = "Reactive")]
    _Reactive,
    #[serde(rename = "Reanimate")]
    _Reanimate,
    #[serde(rename = "Reinforced")]
    _Reinforced,
    #[serde(rename = "Remnant")]
    _Remnant,
    #[serde(rename = "Replete")]
    _Replete,
    #[serde(rename = "Research")]
    _Research,
    #[serde(rename = "Resplendent")]
    _Resplendent,
    #[serde(rename = "Root")]
    _Root,
    #[serde(rename = "Runic")]
    _Runic,
    #[serde(rename = "Sausage and Peppers")]
    _SausageAndPeppers,
    #[serde(rename = "Saved the Best for Last")]
    _SavedTheBestForLast,
    #[serde(rename = "Savior of the Universe")]
    _SaviorOfTheUniverse,
    #[serde(rename = "Secret Twin")]
    _SecretTwin,
    #[serde(rename = "Serious Sam Publishing Contract")]
    _SeriousSamPublishingContract,
    #[serde(rename = "Shadow Warrior")]
    _ShadowWarrior,
    #[serde(rename = "Shell-A-Ton")]
    _ShellATon,
    #[serde(rename = "Shield Night")]
    _ShieldNight,
    #[serde(rename = "Shotgun Affinity")]
    _ShotgunAffinity,
    #[serde(rename = "Siren Sidearm")]
    _SirenSidearm,
    #[serde(rename = "Slings and Arrows")]
    _SlingsAndArrows,
    #[serde(rename = "Sleuth Out")]
    _SleuthOut,
    #[serde(rename = "Smile, you son of a...")]
    _SmileYouSonOfA,
    #[serde(rename = "Snicker-snack")]
    _SnickerSnack,
    #[serde(rename = "Snowball Shotgun")]
    _SnowballShotgun,
    #[serde(rename = "Soft Air")]
    _SoftAir,
    #[serde(rename = "Softshell")]
    _Softshell,
    #[serde(rename = "Solar Flare")]
    _SolarFlare,
    #[serde(rename = "some even larger number")]
    _SomeEvenLargerNumber,
    #[serde(rename = "Space Best Friend")]
    _SpaceBestFriend,
    #[serde(rename = "Spartan Loadout")]
    _SpartanLoadout,
    #[serde(rename = "Spelunker")]
    _Spelunker,
    #[serde(rename = "Spicy D-Boys")]
    _SpicyDBoys,
    #[serde(rename = "Spengbab")]
    _Spengbab,
    #[serde(rename = "Star Friends")]
    _StarFriends,
    #[serde(rename = "Starburst Fire")]
    _StarburstFire,
    #[serde(rename = "Steel Skin")]
    _SteelSkin,
    #[serde(rename = "The Submachinist")]
    _TheSubmachinist,
    #[serde(rename = "Succulent DOOM")]
    _SucculentDOOM,
    #[serde(rename = "Super Bomberpal")]
    _SuperBomberpal,
    #[serde(rename = "Super Space Turtle DX")]
    _SuperSpaceTurtleDX,
    #[serde(rename = "Sweetness and Light")]
    _SweetnessAndLight,
    #[serde(rename = "Swift Sloop")]
    _SwiftSloop,
    #[serde(rename = "Synthetic Shield")]
    _SyntheticShield,
    #[serde(rename = "Tears of Blood")]
    _TearsOfBlood,
    #[serde(rename = "Telefrag")]
    _Telefrag,
    #[serde(rename = "Terminated, Too")]
    _TerminatedToo,
    #[serde(rename = "that &^#$# 'pizza'")]
    _ThatPizza,
    #[serde(rename = "There Is Only War")]
    _ThereIsOnlyWar,
    #[serde(rename = "Thermal Imaging")]
    _ThermalImaging,
    #[serde(rename = "Three Part Harmony")]
    _ThreePartHarmony,
    #[serde(rename = "Three Sheets")]
    _ThreeSheets,
    #[serde(rename = "Throw All Daggers")]
    _ThrowAllDaggers,
    #[serde(rename = "Thrown For A Bloop")]
    _ThrownForABloop,
    #[serde(rename = "Trigger Twins")]
    _TriggerTwins,
    #[serde(rename = "Triple Jump")]
    _TripleJump,
    #[serde(rename = "Triple Sticky")]
    _TripleSticky,
    #[serde(rename = "Turret Link")]
    _TurretLink,
    #[serde(rename = "Turtle Solutions")]
    _TurtleSolutions,
    #[serde(rename = "Tutorialized")]
    _Tutorialized,
    #[serde(rename = "Twin Snakes")]
    _TwinSnakes,
    #[serde(rename = "Two Eggs Over Easy")]
    _TwoEggsOverEasy,
    #[serde(rename = "Two Heavy")]
    _TwoHeavy,
    #[serde(rename = "Two Kinds of People")]
    _TwoKindsofPeople,
    #[serde(rename = "Unsinkable")]
    _Unsinkable,
    #[serde(rename = "Usurper")]
    _Usurper,
    #[serde(rename = "The Void Corps")]
    _TheVoidCorps,
    #[serde(rename = "Vulcan Raving")]
    _VulcanRaving,
    #[serde(rename = "Wave Beam")]
    _WaveBeam,
    #[serde(rename = "Weird Science")]
    _WeirdScience,
    #[serde(rename = "Well Dressed")]
    _WellDressed,
    #[serde(rename = "Whale of a Time")]
    _WhaleOfATime,
    #[serde(rename = "what engine do you use")]
    _WhatEngineDoYouUse,
    #[serde(rename = "Why's Bullet Crying?")]
    _WhysBulletCrying,
    #[serde(rename = "Worlds of Guncraft")]
    _WorldsOfGuncraft,
    #[serde(rename = "Wound Up")]
    _WoundUp,
    #[serde(rename = "Wrath Of The Blam")]
    _WrathOfTheBlam,
    #[serde(rename = "y cant u crawl")]
    _ycantucrawl,
    #[serde(rename = "HIDDEN")]
    HIDDEN,
    #[serde(rename = "360 Yes Scope")]
    _360YesScope,
    #[serde(rename = "All Out Of Law")]
    _AllOutOfLaw,
    #[serde(rename = "Alternative Rock")]
    _AlternativeRock,
    #[serde(rename = "Arctic Warfare")]
    _ArcticWarfare,
    #[serde(rename = "Backdraft")]
    _Backdraft,
    #[serde(rename = "Battery Powered")]
    _BatteryPowered,
    #[serde(rename = "Battle Mode")]
    _BattleMode,
    #[serde(rename = "BEES")]
    _BEES,
    #[serde(rename = "Black Flag")]
    _BlackFlag,
    #[serde(rename = "blade")]
    _blade,
    #[serde(rename = "Bluer Guon Stone")]
    _BluerGuonStone,
    #[serde(rename = "Brave New World")]
    _BraveNewWorld,
    #[serde(rename = "Bullet Kiln")]
    _BulletKiln,
    #[serde(rename = "Cactus Flower")]
    _CactusFlower,
    #[serde(rename = "Careful Iteration")]
    _CarefulIteration,
    #[serde(rename = "Cast Iron")]
    _CastIron,
    #[serde(rename = "Cerebral Bros")]
    _CerebralBros,
    #[serde(rename = "Chicken Arise")]
    _ChickenArise,
    #[serde(rename = "Chief Master")]
    _ChiefMaster,
    #[serde(rename = "Clearer Guon Stone")]
    _ClearerGuonStone,
    #[serde(rename = "Contrail")]
    _Contrail,
    #[serde(rename = "Dawn of the Gundead")]
    _DawnOfTheGundead,
    #[serde(rename = "Decoy Octorok")]
    _DecoyOctorok,
    #[serde(rename = "Diazepam")]
    _Diazepam,
    #[serde(rename = "Don't Worry About The Vase")]
    _DontWorryAboutTheVase,
    #[serde(rename = "The Ecstasy Of Gold")]
    _TheEcstasyOfGold,
    #[serde(rename = "Electron Pack")]
    _ElectronPack,
    #[serde(rename = "Finger on the Pulse")]
    _FingerOnThePulse,
    #[serde(rename = "Fear the Old Blood")]
    _FearTheOldBlood,
    #[serde(rename = "Firing With Flair")]
    _FiringWithFlair,
    #[serde(rename = "Five O'Clock Somewhere")]
    _FiveOClockSomewhere,
    #[serde(rename = "Future Gangster")]
    _FutureGangster,
    #[serde(rename = "Get Equipped With -B-")]
    _GetEquippedWithB,
    #[serde(rename = "Get Equipped with -F-")]
    _GetEquippedwithF,
    #[serde(rename = "Ghost With The Most")]
    _GhostWithTheMost,
    #[serde(rename = "Great Queen Ant")]
    _GreatQueenAnt,
    #[serde(rename = "Greener Guon Stone")]
    _GreenerGuonStone,
    #[serde(rename = "Hardwood")]
    _Hardwood,
    #[serde(rename = "Heavy Metal")]
    _HeavyMetal,
    #[serde(rename = "Hegemony Special Forces")]
    _HegemonySpecialForces,
    #[serde(rename = "Hellhole")]
    _Hellhole,
    #[serde(rename = "Hell Singing")]
    _HellSinging,
    #[serde(rename = "Hidden Tech Bees")]
    _HiddenTechBees,
    #[serde(rename = "Hidden Tech Time")]
    _HiddenTechTime,
    #[serde(rename = "I need scissors! 61!")]
    _INeedScissors61,
    #[serde(rename = "Ice Accumulation")]
    _IceAccumulation,
    #[serde(rename = "Ice Cap")]
    _IceCap,
    #[serde(rename = "Ice To Meet You")]
    _IceToMeetYou,
    #[serde(rename = "Iron Slug")]
    _IronSlug,
    #[serde(rename = "Iron Stance")]
    _IronStance,
    #[serde(rename = "Iroquois")]
    _Iroquois,
    #[serde(rename = "Just Like The Real Thing")]
    _JustLikeTheRealThing,
    #[serde(rename = "Kaliber k'pow uboom k'bhang")]
    _KaliberKpowUboomKbhang,
    #[serde(rename = "Kaliber's Grip")]
    _KalibersGrip,
    #[serde(rename = "Kalibreath")]
    _Kalibreath,
    #[serde(rename = "King Bomber")]
    _KingBomber,
    #[serde(rename = "Klobbering Time")]
    _KlobberingTime,
    #[serde(rename = "The Line Of Fire")]
    _TheLineOfFire,
    #[serde(rename = "Lotus Bloom")]
    _LotusBloom,
    #[serde(rename = "M1 Multi-Tool")]
    _M1MultiTool,
    #[serde(rename = "Mak Pak")]
    _MakPak,
    #[serde(rename = "Massive Effect")]
    _MassiveEffect,
    #[serde(rename = "Master's Chambers")]
    _MastersChambers,
    #[serde(rename = "Maximize Spell")]
    _MaximizeSpell,
    #[serde(rename = "Max Pane")]
    _MaxPane,
    #[serde(rename = "Nailed It!")]
    _NailedIt,
    #[serde(rename = "Natural Selection")]
    _NaturalSelection,
    #[serde(rename = "Needless Acrimony")]
    _NeedlessAcrimony,
    #[serde(rename = "Neo Tech, Yo")]
    _NeoTechYo,
    #[serde(rename = "Not Quite As Mini")]
    _NotQuiteAsMini,
    #[serde(rename = "Not So Sawed-Off")]
    _NotSoSawedOff,
    #[serde(rename = "Noxin Cannon")]
    _NoxinCannon,
    #[serde(rename = "Oranger Guon Stone")]
    _OrangerGuonStone,
    #[serde(rename = "Paperwork")]
    _Paperwork,
    #[serde(rename = "Pandemic Pistol")]
    _PandemicPistol,
    #[serde(rename = "Particle Flow")]
    _ParticleFlow,
    #[serde(rename = "Pea Cannon")]
    _PeaCannon,
    #[serde(rename = "Peripheral Vision")]
    _PeripheralVision,
    #[serde(rename = "Phoenix Up")]
    _PhoenixUp,
    #[serde(rename = "Pinker Guon Stone")]
    _PinkerGuonStone,
    #[serde(rename = "Pistol Machine")]
    _PistolMachine,
    #[serde(rename = "Pitch Perfect")]
    _PitchPerfect,
    #[serde(rename = "Pretty Good")]
    _PrettyGood,
    #[serde(rename = "Production Model")]
    _ProductionModel,
    #[serde(rename = "Redder Guon Stone")]
    _RedderGuonStone,
    #[serde(rename = "Reload Roll")]
    _ReloadRoll,
    #[serde(rename = "Relodestar")]
    _Relodestar,
    #[serde(rename = "Ruby Carbine")]
    _RubyCarbine,
    #[serde(rename = "Second Accident")]
    _SecondAccident,
    #[serde(rename = "Shot Across The Bow")]
    _ShotAcrossTheBow,
    #[serde(rename = "Shredder")]
    _Shredder,
    #[serde(rename = "Smart Bombs")]
    _SmartBombs,
    #[serde(rename = "Sniper Woof")]
    _SniperWoof,
    #[serde(rename = "Special Delivery")]
    _SpecialDelivery,
    #[serde(rename = "Special Reserve")]
    _SpecialReserve,
    #[serde(rename = "Square Brace")]
    _SquareBrace,
    #[serde(rename = "Super Serum")]
    _SuperSerum,
    #[serde(rename = "Tea for Two")]
    _TeaForTwo,
    #[serde(rename = "The Elephant in the Room")]
    _TheElephantintheRoom,
    #[serde(rename = "The Return")]
    _TheReturn,
    #[serde(rename = "Thorn Bath, ooh!")]
    _ThornBathOoh,
    #[serde(rename = "Tile Match")]
    _TileMatch,
    #[serde(rename = "To Serve Android")]
    _ToServeAndroid,
    #[serde(rename = "Unbelievably Charming")]
    _UnbelievablyCharming,
    #[serde(rename = "Vegetables")]
    _Vegetables,
    #[serde(rename = "Venom Veins")]
    _VenomVeins,
    #[serde(rename = "What A Thrill")]
    _WhatAThrill,
    #[serde(rename = "Whiter Guon Stone")]
    _WhiterGuonStone,
    #[serde(rename = "Wicked Sister")]
    _WickedSister,
    #[serde(rename = "Willing To Sacrifice")]
    _WillingToSacrifice,
    #[serde(rename = r#"\o/"#)]
    _HandsUp,
}
