use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Kind {
    Active,
    Passive,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Essential {
    name: String,
    effect: String,
    tier: Option<Tier>,
    #[serde(rename(deserialize = "type"))]
    kind: Kind,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct V1 {
    name: String,
    #[serde(rename(deserialize = "type"))]
    kind: Kind,
    quote: String,
    tier: Option<Tier>,
    effect: String,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Tag {
    #[serde(rename = "Master Round I")]
    _MasterRoundI,
    #[serde(rename = "Master Round II")]
    _MasterRoundII,
    #[serde(rename = "Master Round III")]
    _MasterRoundIII,
    #[serde(rename = "Master Round IV")]
    _MasterRoundIV,
    #[serde(rename = "Master Round V")]
    _MasterRoundV,
    #[serde(rename = "Prime Primer")]
    _PrimePrimer,
    #[serde(rename = "Arcane Gunpowder")]
    _ArcaneGunpowder,
    #[serde(rename = "Planar Lead")]
    _PlanarLead,
    #[serde(rename = "Obsidian Shell Casing")]
    _ObsidianShellCasing,
    #[serde(rename = "Meatbun")]
    _Meatbun,
    #[serde(rename = "Medkit")]
    _Medkit,
    #[serde(rename = "Ration")]
    _Ration,
    #[serde(rename = "Orange")]
    _Orange,
    #[serde(rename = "Friendship Cookie")]
    _FriendshipCookie,
    #[serde(rename = "Bottle")]
    _Bottle,
    #[serde(rename = "Bomb")]
    _Bomb,
    #[serde(rename = "Ice Bomb")]
    _IceBomb,
    #[serde(rename = "Chaff Grenade")]
    _ChaffGrenade,
    #[serde(rename = "Proximity Mine")]
    _ProximityMine,
    #[serde(rename = "Cluster Mine")]
    _ClusterMine,
    #[serde(rename = "C4")]
    _C4,
    #[serde(rename = "Weird Egg")]
    _WeirdEgg,
    #[serde(rename = "Molotov")]
    _Molotov,
    #[serde(rename = "Air Strike")]
    _AirStrike,
    #[serde(rename = "Napalm Strike")]
    _NapalmStrike,
    #[serde(rename = "Big Boy")]
    _BigBoy,
    #[serde(rename = "Roll Bomb")]
    _RollBomb,
    #[serde(rename = "iBomb Companion App")]
    _IBombCompanionApp,
    #[serde(rename = "Supply Drop")]
    _SupplyDrop,
    #[serde(rename = "Ammo Synthesizer")]
    _AmmoSynthesizer,
    #[serde(rename = "Armor Synthesizer")]
    _ArmorSynthesizer,
    #[serde(rename = "Heart Synthesizer")]
    _HeartSynthesizer,
    #[serde(rename = "Master of Unlocking")]
    _MasterOfUnlocking,
    #[serde(rename = "Utility Belt")]
    _UtilityBelt,
    #[serde(rename = "Macho Brace")]
    _MachoBrace,
    #[serde(rename = "Hidden Compartment")]
    _HiddenCompartment,
    #[serde(rename = "Backpack")]
    _Backpack,
    #[serde(rename = "Loot Bag")]
    _LootBag,
    #[serde(rename = "Drill")]
    _Drill,
    #[serde(rename = "Clown Mask")]
    _ClownMask,
    #[serde(rename = "Scope")]
    _Scope,
    #[serde(rename = "Scouter")]
    _Scouter,
    #[serde(rename = "Laser Sight")]
    _LaserSight,
    #[serde(rename = "Ammo Belt")]
    _AmmoBelt,
    #[serde(rename = "Bullet Time")]
    _BulletTime,
    #[serde(rename = "Aged Bell")]
    _AgedBell,
    #[serde(rename = "Singularity")]
    _Singularity,
    #[serde(rename = "Decoy")]
    _Decoy,
    #[serde(rename = "Shadow Clone")]
    _ShadowClone,
    #[serde(rename = "Explosive Decoy")]
    _ExplosiveDecoy,
    #[serde(rename = "Melted Rock")]
    _MeltedRock,
    #[serde(rename = "Trusty Lockpicks")]
    _TrustyLockpicks,
    #[serde(rename = "Smoke Bomb")]
    _SmokeBomb,
    #[serde(rename = "Box")]
    _Box,
    #[serde(rename = "Fortune's Favor")]
    _FortunesFavor,
    #[serde(rename = "Jar of Bees")]
    _JarofBees,
    #[serde(rename = "Potion of Lead Skin")]
    _PotionofLeadSkin,
    #[serde(rename = "Double Vision")]
    _DoubleVision,
    #[serde(rename = "Chest Teleporter")]
    _ChestTeleporter,
    #[serde(rename = "Relodestone")]
    _Relodestone,
    #[serde(rename = "Poison Vial")]
    _PoisonVial,
    #[serde(rename = "Potion of Gun Friendship")]
    _PotionofGunFriendship,
    #[serde(rename = "Portable Turret")]
    _PortableTurret,
    #[serde(rename = "Knife Shield")]
    _KnifeShield,
    #[serde(rename = "Grappling Hook")]
    _GrapplingHook,
    #[serde(rename = "Stuffed Star")]
    _StuffedStar,
    #[serde(rename = "Boomerang")]
    _Boomerang,
    #[serde(rename = "Shield of the Maiden")]
    _ShieldoftheMaiden,
    #[serde(rename = "Crisis Stone")]
    _CrisisStone,
    #[serde(rename = "Portable Table Device")]
    _PortableTableDevice,
    #[serde(rename = "Daruma")]
    _Daruma,
    #[serde(rename = "Partially-Eaten Cheese")]
    _PartiallyEatenCheese,
    #[serde(rename = "+1 Bullets")]
    _Plus1Bullets,
    #[serde(rename = "Rocket-Powered Bullets")]
    _RocketPoweredBullets,
    #[serde(rename = "Shock Rounds")]
    _ShockRounds,
    #[serde(rename = "Devolver Rounds")]
    _DevolverRounds,
    #[serde(rename = "Vorpal Bullets")]
    _VorpalBullets,
    #[serde(rename = "Katana Bullets")]
    _KatanaBullets,
    #[serde(rename = "Hungry Bullets")]
    _HungryBullets,
    #[serde(rename = "Heavy Bullets")]
    _HeavyBullets,
    #[serde(rename = "Bouncy Bullets")]
    _BouncyBullets,
    #[serde(rename = "Explosive Rounds")]
    _ExplosiveRounds,
    #[serde(rename = "Ghost Bullets")]
    _GhostBullets,
    #[serde(rename = "Alpha Bullet")]
    _AlphaBullet,
    #[serde(rename = "Omega Bullets")]
    _OmegaBullets,
    #[serde(rename = "Scattershot")]
    _Scattershot,
    #[serde(rename = "Irradiated Lead")]
    _IrradiatedLead,
    #[serde(rename = "Hot Lead")]
    _HotLead,
    #[serde(rename = "Battery Bullets")]
    _BatteryBullets,
    #[serde(rename = "Frost Bullets")]
    _FrostBullets,
    #[serde(rename = "Charming Rounds")]
    _CharmingRounds,
    #[serde(rename = "Magic Bullets")]
    _MagicBullets,
    #[serde(rename = "Fat Bullets")]
    _FatBullets,
    #[serde(rename = "Angry Bullets")]
    _AngryBullets,
    #[serde(rename = "Blank Bullets")]
    _BlankBullets,
    #[serde(rename = "Orbital Bullets")]
    _OrbitalBullets,
    #[serde(rename = "Homing Bullets")]
    _HomingBullets,
    #[serde(rename = "Shadow Bullets")]
    _ShadowBullets,
    #[serde(rename = "Easy Reload Bullets")]
    _EasyReloadBullets,
    #[serde(rename = "Stout Bullets")]
    _StoutBullets,
    #[serde(rename = "Snowballets")]
    _Snowballets,
    #[serde(rename = "Remote Bullets")]
    _RemoteBullets,
    #[serde(rename = "Zombie Bullets")]
    _ZombieBullets,
    #[serde(rename = "Flak Bullets")]
    _FlakBullets,
    #[serde(rename = "Silver Bullets")]
    _SilverBullets,
    #[serde(rename = "Gilded Bullets")]
    _GildedBullets,
    #[serde(rename = "Platinum Bullets")]
    _PlatinumBullets,
    #[serde(rename = "Chaos Bullets")]
    _ChaosBullets,
    #[serde(rename = "Cursed Bullets")]
    _CursedBullets,
    #[serde(rename = "Chance Bullets")]
    _ChanceBullets,
    #[serde(rename = "Helix Bullets")]
    _HelixBullets,
    #[serde(rename = "Bumbullets")]
    _Bumbullets,
    #[serde(rename = "Bloody 9mm")]
    _Bloody9mm,
    #[serde(rename = "Bionic Leg")]
    _BionicLeg,
    #[serde(rename = "Shotgun Coffee")]
    _ShotgunCoffee,
    #[serde(rename = "Shotga Cola")]
    _ShotgaCola,
    #[serde(rename = "Ballistic Boots")]
    _BallisticBoots,
    #[serde(rename = "Magic Sweet")]
    _MagicSweet,
    #[serde(rename = "Disarming Personality")]
    _DisarmingPersonality,
    #[serde(rename = "Mustache")]
    _Mustache,
    #[serde(rename = "Lichy Trigger Finger")]
    _LichyTriggerFinger,
    #[serde(rename = "Lich's Eye Bullets")]
    _LichsEyeBullets,
    #[serde(rename = "Enraging Photo")]
    _EnragingPhoto,
    #[serde(rename = "Ballot")]
    _Ballot,
    #[serde(rename = "Live Ammo")]
    _LiveAmmo,
    #[serde(rename = "Eyepatch")]
    _Eyepatch,
    #[serde(rename = "Military Training")]
    _MilitaryTraining,
    #[serde(rename = "Ring of the Resourceful Rat")]
    _RingOftheResourcefulRat,
    #[serde(rename = "Cartographer's Ring")]
    _CartographersRing,
    #[serde(rename = "Amulet of the Pit Lord")]
    _AmuletOfthePitLord,
    #[serde(rename = "Ring of Fire Resistance")]
    _RingOfFireResistance,
    #[serde(rename = "Ring of Miserly Protection")]
    _RingOfMiserlyProtection,
    #[serde(rename = "Unity")]
    _Unity,
    #[serde(rename = "Ring of Chest Vampirism")]
    _RingOfChestVampirism,
    #[serde(rename = "Cloranthy Ring")]
    _CloranthyRing,
    #[serde(rename = "Ring of Chest Friendship")]
    _RingOfChestFriendship,
    #[serde(rename = "Ring of Mimic Friendship")]
    _RingOfMimicFriendship,
    #[serde(rename = "Ring of Triggers")]
    _RingOfTriggers,
    #[serde(rename = "Ring of Ethereal Form")]
    _RingOfEtherealForm,
    #[serde(rename = "Gundromeda Strain")]
    _GundromedaStrain,
    #[serde(rename = "Broccoli")]
    _Broccoli,
    #[serde(rename = "Crutch")]
    _Crutch,
    #[serde(rename = "Spice")]
    _Spice,
    #[serde(rename = "Liquid Valkyrie")]
    _LiquidValkyrie,
    #[serde(rename = "Bloody Eye")]
    _BloodyEye,
    #[serde(rename = "Gunknight Helmet")]
    _GunknightHelmet,
    #[serde(rename = "Gunknight Greaves")]
    _GunknightGreaves,
    #[serde(rename = "Gunknight Gauntlet")]
    _GunknightGauntlet,
    #[serde(rename = "Gunknight Armor")]
    _GunknightArmor,
    #[serde(rename = "Old Knight's Shield")]
    _OldKnightsShield,
    #[serde(rename = "Old Knight's Helm")]
    _OldKnightsHelm,
    #[serde(rename = "Old Knight's Flask")]
    _OldKnightsFlask,
    #[serde(rename = "Old Crest")]
    _OldCrest,
    #[serde(rename = "Armor of Thorns")]
    _ArmorOfThorns,
    #[serde(rename = "Full Metal Jacket")]
    _FullMetalJacket,
    #[serde(rename = "Ticket")]
    _Ticket,
    #[serde(rename = "Heavy Boots")]
    _HeavyBoots,
    #[serde(rename = "Bug Boots")]
    _BugBoots,
    #[serde(rename = "Gunboots")]
    _Gunboots,
    #[serde(rename = "Springheel Boots")]
    _SpringheelBoots,
    #[serde(rename = "Rat Boots")]
    _RatBoots,
    #[serde(rename = "Coin Crown")]
    _CoinCrown,
    #[serde(rename = "Oiled Cylinder")]
    _OiledCylinder,
    #[serde(rename = "Ice Cube")]
    _IceCube,
    #[serde(rename = "Rolling Eye")]
    _RollingEye,
    #[serde(rename = "Cigarettes")]
    _Cigarettes,
    #[serde(rename = "Magazine Rack")]
    _MagazineRack,
    #[serde(rename = "Charm Horn")]
    _CharmHorn,
    #[serde(rename = "Cog of Battle")]
    _CogOfBattle,
    #[serde(rename = "Metronome")]
    _Metronome,
    #[serde(rename = "Honeycomb")]
    _Honeycomb,
    #[serde(rename = "Map")]
    _Map,
    #[serde(rename = "Gungeon Blueprint")]
    _GungeonBlueprint,
    #[serde(rename = "Sense of Direction")]
    _SenseOfDirection,
    #[serde(rename = "Duct Tape")]
    _DuctTape,
    #[serde(rename = "Gungeon Pepper")]
    _GungeonPepper,
    #[serde(rename = "Antibody")]
    _Antibody,
    #[serde(rename = "Resourceful Sack")]
    _ResourcefulSack,
    #[serde(rename = "Pink Guon Stone")]
    _PinkGuonStone,
    #[serde(rename = "White Guon Stone")]
    _WhiteGuonStone,
    #[serde(rename = "Orange Guon Stone")]
    _OrangeGuonStone,
    #[serde(rename = "Clear Guon Stone")]
    _ClearGuonStone,
    #[serde(rename = "Green Guon Stone")]
    _GreenGuonStone,
    #[serde(rename = "Red Guon Stone")]
    _RedGuonStone,
    #[serde(rename = "Blue Guon Stone")]
    _BlueGuonStone,
    #[serde(rename = "Glass Guon Stone")]
    _GlassGuonStone,
    #[serde(rename = "Iron Coin")]
    _IronCoin,
    #[serde(rename = "Super Hot Watch")]
    _SuperHotWatch,
    #[serde(rename = "Drum Clip")]
    _DrumClip,
    #[serde(rename = "Holey Grail")]
    _HoleyGrail,
    #[serde(rename = "Blood Brooch")]
    _BloodBrooch,
    #[serde(rename = "Backup Gun")]
    _BackupGun,
    #[serde(rename = "Sunglasses")]
    _Sunglasses,
    #[serde(rename = "Mimic Tooth Necklace")]
    _MimicToothNecklace,
    #[serde(rename = "Escape Rope")]
    _EscapeRope,
    #[serde(rename = "Jetpack")]
    _Jetpack,
    #[serde(rename = "Wax Wings")]
    _WaxWings,
    #[serde(rename = "Cat Bullet King Throne")]
    _CatBulletKingThrone,
    #[serde(rename = "Blast Helmet")]
    _BlastHelmet,
    #[serde(rename = "Lament Configurum")]
    _LamentConfigurum,
    #[serde(rename = "Monster Blood")]
    _MonsterBlood,
    #[serde(rename = "Nanomachines")]
    _Nanomachines,
    #[serde(rename = "Seven-Leaf Clover")]
    _SevenLeafClover,
    #[serde(rename = "Number 2")]
    _Number2,
    #[serde(rename = "Gold Ammolet")]
    _GoldAmmolet,
    #[serde(rename = "Chaos Ammolet")]
    _ChaosAmmolet,
    #[serde(rename = "Lodestone Ammolet")]
    _LodestoneAmmolet,
    #[serde(rename = "Uranium Ammolet")]
    _UraniumAmmolet,
    #[serde(rename = "Copper Ammolet")]
    _CopperAmmolet,
    #[serde(rename = "Frost Ammolet")]
    _FrostAmmolet,
    #[serde(rename = "Table Tech Sight")]
    _TableTechSight,
    #[serde(rename = "Table Tech Money")]
    _TableTechMoney,
    #[serde(rename = "Table Tech Rocket")]
    _TableTechRocket,
    #[serde(rename = "Table Tech Shotgun")]
    _TableTechShotgun,
    #[serde(rename = "Table Tech Heat")]
    _TableTechHeat,
    #[serde(rename = "Table Tech Rage")]
    _TableTechRage,
    #[serde(rename = "Table Tech Blanks")]
    _TableTechBlanks,
    #[serde(rename = "Table Tech Stun")]
    _TableTechStun,
    #[serde(rename = "Heart Holster")]
    _HeartHolster,
    #[serde(rename = "Heart Lunchbox")]
    _HeartLunchbox,
    #[serde(rename = "Heart Locket")]
    _HeartLocket,
    #[serde(rename = "Heart Bottle")]
    _HeartBottle,
    #[serde(rename = "Heart Purse")]
    _HeartPurse,
    #[serde(rename = "Ruby Bracelet")]
    _RubyBracelet,
    #[serde(rename = "Sixth Chamber")]
    _SixthChamber,
    #[serde(rename = "Busted Television")]
    _BustedTelevision,
    #[serde(rename = "Coolant Leak")]
    _CoolantLeak,
    #[serde(rename = "Heart of Ice")]
    _HeartOfIce,
    #[serde(rename = "Ancient Hero's Bandana")]
    _AncientHerosBandana,
    #[serde(rename = "Bloodied Scarf")]
    _BloodiedScarf,
    #[serde(rename = "Muscle Relaxant")]
    _MuscleRelaxant,
    #[serde(rename = "Hip Holster")]
    _HipHolster,
    #[serde(rename = "Clone")]
    _Clone,
    #[serde(rename = "Sponge")]
    _Sponge,
    #[serde(rename = "Gas Mask")]
    _GasMask,
    #[serde(rename = "Hazmat Suit")]
    _HazmatSuit,
    #[serde(rename = "Book of Chest Anatomy")]
    _BookOfChestAnatomy,
    #[serde(rename = "Gun Soul")]
    _GunSoul,
    #[serde(rename = "Shelleton Key")]
    _ShelletonKey,
    #[serde(rename = "Brick of Cash")]
    _BrickOfCash,
    #[serde(rename = "Battle Standard")]
    _BattleStandard,
    #[serde(rename = "Wingman")]
    _Wingman,
    #[serde(rename = "Wolf")]
    _Wolf,
    #[serde(rename = "Dog")]
    _Dog,
    #[serde(rename = "Owl")]
    _Owl,
    #[serde(rename = "Super Space Turtle")]
    _SuperSpaceTurtle,
    #[serde(rename = "Junk")]
    _Junk,
    #[serde(rename = "Gold Junk")]
    _GoldJunk,
    #[serde(rename = "Lies")]
    _Lies,
    #[serde(rename = "Ser Junkan")]
    _SerJunkan,
    #[serde(rename = "R2G2")]
    _R2G2,
    #[serde(rename = "Baby Good Shelleton")]
    _BabyGoodShelleton,
    #[serde(rename = "Badge")]
    _Badge,
    #[serde(rename = "Chicken Flute")]
    _ChickenFlute,
    #[serde(rename = "Space Friend")]
    _SpaceFriend,
    #[serde(rename = "Pig")]
    _Pig,
    #[serde(rename = "Turkey")]
    _Turkey,
    #[serde(rename = "Turtle Problem")]
    _TurtleProblem,
    #[serde(rename = "Sprun")]
    _Sprun,
    #[serde(rename = "Baby Good Mimic")]
    _BabyGoodMimic,
    #[serde(rename = "Blank Companion's Ring")]
    _BlankCompanionsRing,
    #[serde(rename = "Briefcase of Cash")]
    _BriefcaseOfCash,
    #[serde(rename = "Galactic Medal of Valor")]
    _GalacticMedalOfValor,
    #[serde(rename = "Bullet Idol")]
    _BulletIdol,
    #[serde(rename = "Riddle of Lead")]
    _RiddleOfLead,
    #[serde(rename = "Bracket Key")]
    _BracketKey,
    #[serde(rename = "Elder Blank")]
    _ElderBlank,
    #[serde(rename = "Teleporter Prototype")]
    _TeleporterPrototype,
    #[serde(rename = "Yellow Chamber")]
    _YellowChamber,
    #[serde(rename = "Infuriating Note")]
    _InfuriatingNote,
}
