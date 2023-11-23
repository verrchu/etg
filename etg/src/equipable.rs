use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Equipable {
    Gun(Gun),
    Item(Item),
}

impl From<Gun> for Equipable {
    fn from(gun: Gun) -> Equipable {
        Equipable::Gun(gun)
    }
}

impl From<Item> for Equipable {
    fn from(item: Item) -> Equipable {
        Equipable::Item(item)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, etg_derive::Tags)]
pub enum Gun {
    #[serde(rename = "Casey")]
    _Casey,
    #[serde(rename = "Pea Shooter")]
    _PeaShooter,
    #[serde(rename = "38 Special")]
    _38Special,
    #[serde(rename = "Derringer")]
    _Derringer,
    #[serde(rename = "Trank Gun")]
    _TrankGun,
    #[serde(rename = "Unfinished Gun")]
    _UnfinishedGun,
    #[serde(rename = "Elimentaler")]
    _Elimentaler,
    #[serde(rename = "Bullet")]
    _Bullet,
    #[serde(rename = "Shell")]
    _Shell,
    #[serde(rename = "Hyper Light Blaster")]
    _HyperLightBlaster,
    #[serde(rename = "Boxing Glove")]
    _BoxingGlove,
    #[serde(rename = "Makarov")]
    _Makarov,
    #[serde(rename = "M1911")]
    _M1911,
    #[serde(rename = "Magnum")]
    _Magnum,
    #[serde(rename = "Colt 1851")]
    _Colt1851,
    #[serde(rename = "SAA")]
    _SAA,
    #[serde(rename = "Cold 45")]
    _Cold45,
    #[serde(rename = "Polaris")]
    _Polaris,
    #[serde(rename = "Jolter")]
    _Jolter,
    #[serde(rename = "Dungeon Eagle")]
    _DungeonEagle,
    #[serde(rename = "Grey Mauser")]
    _GreyMauser,
    #[serde(rename = "Vorpal Gun")]
    _VorpalGun,
    #[serde(rename = "High Kaliber")]
    _HighKaliber,
    #[serde(rename = "Shellegun")]
    _Shellegun,
    #[serde(rename = "Dueling Pistol")]
    _DuelingPistol,
    #[serde(rename = "AU Gun")]
    _AUGun,
    #[serde(rename = "Big Iron")]
    _BigIron,
    #[serde(rename = "Composite Gun")]
    _CompositeGun,
    #[serde(rename = "Flare Gun")]
    _FlareGun,
    #[serde(rename = "Smiley's Revolver")]
    _SmileysRevolver,
    #[serde(rename = "Shades's Revolver")]
    _ShadessRevolver,
    #[serde(rename = "Knight's Gun")]
    _KnightsGun,
    #[serde(rename = "GuNNER")]
    _GuNNER,
    #[serde(rename = "Regular Shotgun")]
    _RegularShotgun,
    #[serde(rename = "Big Shotgun")]
    _BigShotgun,
    #[serde(rename = "Old Goldie")]
    _OldGoldie,
    #[serde(rename = "Sawed-Off")]
    _SawedOff,
    #[serde(rename = "Winchester")]
    _Winchester,
    #[serde(rename = "Rattler")]
    _Rattler,
    #[serde(rename = "Bubble Blaster")]
    _BubbleBlaster,
    #[serde(rename = "Elephant Gun")]
    _ElephantGun,
    #[serde(rename = "Tangler")]
    _Tangler,
    #[serde(rename = "Void Shotgun")]
    _VoidShotgun,
    #[serde(rename = "Mass Shotgun")]
    _MassShotgun,
    #[serde(rename = "Shotgun Full of Hate")]
    _ShotgunFullOfHate,
    #[serde(rename = "Shotgun Full of Love")]
    _ShotgunFullOfLove,
    #[serde(rename = "Shotgrub")]
    _Shotgrub,
    #[serde(rename = "Gilded Hydra")]
    _GildedHydra,
    #[serde(rename = "Blunderbuss")]
    _Blunderbuss,
    #[serde(rename = "Pulse Cannon")]
    _PulseCannon,
    #[serde(rename = "Siren")]
    _Siren,
    #[serde(rename = "Zilla Shotgun")]
    _ZillaShotgun,
    #[serde(rename = "Ice Breaker")]
    _IceBreaker,
    #[serde(rename = "The Membrane")]
    _TheMembrane,
    #[serde(rename = "Huntsman")]
    _Huntsman,
    #[serde(rename = "Blooper")]
    _Blooper,
    #[serde(rename = "JK-47")]
    _JK47,
    #[serde(rename = "Bow")]
    _Bow,
    #[serde(rename = "Charmed Bow")]
    _CharmedBow,
    #[serde(rename = "Crossbow")]
    _Crossbow,
    #[serde(rename = "Sticky Crossbow")]
    _StickyCrossbow,
    #[serde(rename = "Shotbow")]
    _Shotbow,
    #[serde(rename = "Triple Crossbow")]
    _TripleCrossbow,
    #[serde(rename = "Crescent Crossbow")]
    _CrescentCrossbow,
    #[serde(rename = "Gunbow")]
    _Gunbow,
    #[serde(rename = "Klobbe")]
    _Klobbe,
    #[serde(rename = "Machine Pistol")]
    _MachinePistol,
    #[serde(rename = "Thompson Sub-Machinegun")]
    _ThompsonSubMachinegun,
    #[serde(rename = "AK-47")]
    _AK47,
    #[serde(rename = "Strafe Gun")]
    _StrafeGun,
    #[serde(rename = "AKEY-47")]
    _AKEY47,
    #[serde(rename = "Stone Dome")]
    _StoneDome,
    #[serde(rename = "Crown of Guns")]
    _CrownOfGuns,
    #[serde(rename = "M16")]
    _M16,
    #[serde(rename = "Combined Rifle")]
    _CombinedRifle,
    #[serde(rename = "Zorgun")]
    _Zorgun,
    #[serde(rename = "The Fat Line")]
    _TheFatLine,
    #[serde(rename = "Mr. Accretion Jr.")]
    _MrAccretionJr,
    #[serde(rename = "VertebraeK-47")]
    _VertebraeK47,
    #[serde(rename = "Balloon Gun")]
    _BalloonGun,
    #[serde(rename = "MAC10")]
    _MAC10,
    #[serde(rename = "Triple Gun")]
    _TripleGun,
    #[serde(rename = "Heck Blaster")]
    _HeckBlaster,
    #[serde(rename = "Patriot")]
    _Patriot,
    #[serde(rename = "AC-15")]
    _AC15,
    #[serde(rename = "Vulcan Cannon")]
    _VulcanCannon,
    #[serde(rename = "Plague Pistol")]
    _PlaguePistol,
    #[serde(rename = "Gungine")]
    _Gungine,
    #[serde(rename = "The Predator")]
    _ThePredator,
    #[serde(rename = "Dragunfire")]
    _Dragunfire,
    #[serde(rename = "Sniper Rifle")]
    _SniperRifle,
    #[serde(rename = "A.W.P.")]
    _AWP,
    #[serde(rename = "M1")]
    _M1,
    #[serde(rename = "Winchester Rifle")]
    _WinchesterRifle,
    #[serde(rename = "Corsair")]
    _Corsair,
    #[serde(rename = "Railgun")]
    _Railgun,
    #[serde(rename = "Prototype Railgun")]
    _PrototypeRailgun,
    #[serde(rename = "Void Marshal")]
    _VoidMarshal,
    #[serde(rename = "Deck4rd")]
    _Deck4rd,
    #[serde(rename = "The Judge")]
    _TheJudge,
    #[serde(rename = "Mourning Star")]
    _MourningStar,
    #[serde(rename = "Alien Sidearm")]
    _AlienSidearm,
    #[serde(rename = "RUBE-ADYNE Prototype")]
    _RUBEADYNEPrototype,
    #[serde(rename = "RUBE-ADYNE MK.II")]
    _RUBEADYNEMKII,
    #[serde(rename = "Rubenstein's Monster")]
    _RubensteinsMonster,
    #[serde(rename = "Chamber Gun")]
    _ChamberGun,
    #[serde(rename = "Gunderfury")]
    _Gunderfury,
    #[serde(rename = "Mine Cutter")]
    _MineCutter,
    #[serde(rename = "Void Core Assault Rifle")]
    _VoidCoreAssaultRifle,
    #[serde(rename = "Flash Ray")]
    _FlashRay,
    #[serde(rename = "Wind Up Gun")]
    _WindUpGun,
    #[serde(rename = "H4mmer")]
    _H4mmer,
    #[serde(rename = "Snakemaker")]
    _Snakemaker,
    #[serde(rename = "Turbo-Gun")]
    _TurboGun,
    #[serde(rename = "Hegemony Carbine")]
    _HegemonyCarbine,
    #[serde(rename = "Screecher")]
    _Screecher,
    #[serde(rename = "Laser Lotus")]
    _LaserLotus,
    #[serde(rename = "Hegemony Rifle")]
    _HegemonyRifle,
    #[serde(rename = "Fightsabre")]
    _Fightsabre,
    #[serde(rename = "Helix")]
    _Helix,
    #[serde(rename = "Laser Rifle")]
    _LaserRifle,
    #[serde(rename = "Crestfaller")]
    _Crestfaller,
    #[serde(rename = "Rad Gun")]
    _RadGun,
    #[serde(rename = "Thunderclap")]
    _Thunderclap,
    #[serde(rename = "Charge Shot")]
    _ChargeShot,
    #[serde(rename = "Dark Marker")]
    _DarkMarker,
    #[serde(rename = "Particulator")]
    _Particulator,
    #[serde(rename = "The Emperor")]
    _TheEmperor,
    #[serde(rename = "RPG")]
    _RPG,
    #[serde(rename = "Grenade Launcher")]
    _GrenadeLauncher,
    #[serde(rename = "Stinger")]
    _Stinger,
    #[serde(rename = "The Exotic")]
    _TheExotic,
    #[serde(rename = "Com4nd0")]
    _Com4nd0,
    #[serde(rename = "RC Rocket")]
    _RCRocket,
    #[serde(rename = "Yari Launcher")]
    _YariLauncher,
    #[serde(rename = "Lil' Bomber")]
    _LilBomber,
    #[serde(rename = "Grasschopper")]
    _Grasschopper,
    #[serde(rename = "Void Core Cannon")]
    _VoidCoreCannon,
    #[serde(rename = "Bundle of Wands")]
    _BundleOfWands,
    #[serde(rename = "Staff of Firepower")]
    _StaffOfFirepower,
    #[serde(rename = "Witch Pistol")]
    _WitchPistol,
    #[serde(rename = "Hexagun")]
    _Hexagun,
    #[serde(rename = "Phoenix")]
    _Phoenix,
    #[serde(rename = "Magic Lamp")]
    _MagicLamp,
    #[serde(rename = "Teapot")]
    _Teapot,
    #[serde(rename = "Gunslinger's Ashes")]
    _GunslingersAshes,
    #[serde(rename = "Luxin Cannon")]
    _LuxinCannon,
    #[serde(rename = "Gunther")]
    _Gunther,
    #[serde(rename = "Unicorn Horn")]
    _UnicornHorn,
    #[serde(rename = "Cobalt Hammer")]
    _CobaltHammer,
    #[serde(rename = "Frost Giant")]
    _FrostGiant,
    #[serde(rename = "Bullet Bore")]
    _BulletBore,
    #[serde(rename = "Cat Claw")]
    _CatClaw,
    #[serde(rename = "Megahand")]
    _Megahand,
    #[serde(rename = "Demon Head")]
    _DemonHead,
    #[serde(rename = "Heroine")]
    _Heroine,
    #[serde(rename = "Mutation")]
    _Mutation,
    #[serde(rename = "Flame Hand")]
    _FlameHand,
    #[serde(rename = "Sunlight Javelin")]
    _SunlightJavelin,
    #[serde(rename = "Machine Fist")]
    _MachineFist,
    #[serde(rename = "Robot's Left Hand")]
    _RobotsLeftHand,
    #[serde(rename = "Snowballer")]
    _Snowballer,
    #[serde(rename = "Super Meat Gun")]
    _SuperMeatGun,
    #[serde(rename = "Anvillain")]
    _Anvillain,
    #[serde(rename = "Fossilized Gun")]
    _FossilizedGun,
    #[serde(rename = "Gamma Ray")]
    _GammaRay,
    #[serde(rename = "Freeze Ray")]
    _FreezeRay,
    #[serde(rename = "Science Cannon")]
    _ScienceCannon,
    #[serde(rename = "Disintegrator")]
    _Disintegrator,
    #[serde(rename = "Proton Backpack")]
    _ProtonBackpack,
    #[serde(rename = "Mega Douser")]
    _MegaDouser,
    #[serde(rename = "Plunger")]
    _Plunger,
    #[serde(rename = "Raiden Coil")]
    _RaidenCoil,
    #[serde(rename = "Moonscraper")]
    _Moonscraper,
    #[serde(rename = "Barrel")]
    _Barrel,
    #[serde(rename = "Trick Gun")]
    _TrickGun,
    #[serde(rename = "Mailbox")]
    _Mailbox,
    #[serde(rename = "Nail Gun")]
    _NailGun,
    #[serde(rename = "Light Gun")]
    _LightGun,
    #[serde(rename = "Dueling Laser")]
    _DuelingLaser,
    #[serde(rename = "Wood Beam")]
    _WoodBeam,
    #[serde(rename = "Mahoguny")]
    _Mahoguny,
    #[serde(rename = "The Scrambler")]
    _TheScrambler,
    #[serde(rename = "Trashcannon")]
    _Trashcannon,
    #[serde(rename = "Glacier")]
    _Glacier,
    #[serde(rename = "Origuni")]
    _Origuni,
    #[serde(rename = "The Kiln")]
    _TheKiln,
    #[serde(rename = "Skull Spitter")]
    _SkullSpitter,
    #[serde(rename = "Buzzkill")]
    _Buzzkill,
    #[serde(rename = "Tear Jerker")]
    _TearJerker,
    #[serde(rename = "Starpew")]
    _Starpew,
    #[serde(rename = "Eye of the Beholster")]
    _EyeOftheBeholster,
    #[serde(rename = "Molotov Launcher")]
    _MolotovLauncher,
    #[serde(rename = "Glass Cannon")]
    _GlassCannon,
    #[serde(rename = "Shock Rifle")]
    _ShockRifle,
    #[serde(rename = "Bait Launcher")]
    _BaitLauncher,
    #[serde(rename = "Brick Breaker")]
    _BrickBreaker,
    #[serde(rename = "Betrayer's Shield")]
    _BetrayersShield,
    #[serde(rename = "Lower Case r")]
    _LowerCaser,
    #[serde(rename = "Excaliber")]
    _Excaliber,
    #[serde(rename = "Face Melter")]
    _FaceMelter,
    #[serde(rename = "Really Special Lute")]
    _ReallySpecialLute,
    #[serde(rename = "Trident")]
    _Trident,
    #[serde(rename = "Kruller Glaive")]
    _KrullerGlaive,
    #[serde(rename = "Abyssal Tentacle")]
    _AbyssalTentacle,
    #[serde(rename = "Quad Laser")]
    _QuadLaser,
    #[serde(rename = "Tetrominator")]
    _Tetrominator,
    #[serde(rename = "Pitchfork")]
    _Pitchfork,
    #[serde(rename = "Evolver")]
    _Evolver,
    #[serde(rename = "Gungeon Ant")]
    _GungeonAnt,
    #[serde(rename = "Life Orb")]
    _LifeOrb,
    #[serde(rename = "Alien Engine")]
    _AlienEngine,
    #[serde(rename = "Microtransaction Gun")]
    _MicrotransactionGun,
    #[serde(rename = "Poxcannon")]
    _Poxcannon,
    #[serde(rename = "T-Shirt Cannon")]
    _TShirtCannon,
    #[serde(rename = "Banana")]
    _Banana,
    #[serde(rename = "Bee Hive")]
    _BeeHive,
    #[serde(rename = "Silencer")]
    _Silencer,
    #[serde(rename = "Camera")]
    _Camera,
    #[serde(rename = "Directional Pad")]
    _DirectionalPad,
    #[serde(rename = "3rd Party Controller")]
    _3rdPartyController,
    #[serde(rename = "Gunzheng")]
    _Gunzheng,
    #[serde(rename = "Sling")]
    _Sling,
    #[serde(rename = "Cactus")]
    _Cactus,
    #[serde(rename = "Black Hole Gun")]
    _BlackHoleGun,
    #[serde(rename = "BSG")]
    _BSG,
    #[serde(rename = "Compressed Air Tank")]
    _CompressedAirTank,
    #[serde(rename = "Serious Cannon")]
    _SeriousCannon,
    #[serde(rename = "Makeshift Cannon")]
    _MakeshiftCannon,
    #[serde(rename = "Devolver")]
    _Devolver,
    #[serde(rename = "Finished Gun")]
    _FinishedGun,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, etg_derive::Tags)]
pub enum Item {
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
