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
    Automatic,
    Beam,
    Burst,
    Charged,
    Semiautomatic,
    Type,
    Varies,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Essential {
    name: String,
    notes: Option<String>,
    tier: Option<Tier>,
    #[serde(rename(deserialize = "type"))]
    kind: Kind,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct V1 {
    name: String,
    notes: Option<String>,
    quote: Option<String>,
    tier: Option<Tier>,
    #[serde(rename = "type")]
    kind: Kind,
    dps: Option<String>,
    magazine_size: Option<String>,
    ammo_capacity: Option<String>,
    damage: Option<String>,
    fire_rate: Option<String>,
    reload_time: Option<String>,
    shot_speed: Option<String>,
    range: Option<String>,
    force: Option<String>,
    spread: Option<String>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
