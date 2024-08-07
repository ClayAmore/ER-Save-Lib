use std::{collections::HashMap, sync::LazyLock};

pub static GEM_CAPTION: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(10,""),
		(11,""),
		(12,""),
		(13,""),
		(14,""),
		(15,""),
		(16,""),
		(17,""),
		(18,""),
		(19,""),
		(20,""),
		(21,""),
		(22,""),
		(23,""),
		(24,""),
		(25,""),
		(26,""),
		(27,""),
		(28,""),
		(29,""),
		(30,""),
		(80,""),
		(81,""),
		(82,""),
		(83,""),
		(84,""),
		(85,""),
		(86,""),
		(87,""),
		(88,""),
		(89,""),
		(90,""),
		(91,""),
		(92,""),
		(93,""),
		(94,""),
		(95,""),
		(96,""),
		(97,""),
		(98,""),
		(99,""),
		(100,""),
		(101,""),
		(102,""),
		(103,""),
		(104,""),
		(105,""),
		(106,""),
		(107,""),
		(108,""),
		(109,""),
		(110,""),
		(111,""),
		(112,""),
		(113,""),
		(114,""),
		(115,""),
		(116,""),
		(117,""),
		(118,""),
		(119,""),
		(120,""),
		(121,""),
		(122,""),
		(123,""),
		(124,""),
		(125,""),
		(126,""),
		(127,""),
		(128,""),
		(129,""),
		(130,""),
		(131,""),
		(132,""),
		(133,""),
		(134,""),
		(135,""),
		(136,""),
		(137,""),
		(138,""),
		(139,""),
		(140,""),
		(141,""),
		(142,""),
		(143,""),
		(144,""),
		(145,""),
		(146,""),
		(147,""),
		(148,""),
		(149,""),
		(150,""),
		(151,""),
		(152,""),
		(153,""),
		(154,""),
		(155,""),
		(156,""),
		(157,""),
		(158,""),
		(159,""),
		(160,""),
		(161,""),
		(162,""),
		(163,""),
		(164,""),
		(165,""),
		(166,""),
		(167,""),
		(168,""),
		(169,""),
		(170,""),
		(171,""),
		(172,""),
		(173,""),
		(174,""),
		(175,""),
		(176,""),
		(177,""),
		(178,""),
		(179,""),
		(180,""),
		(181,""),
		(182,""),
		(183,""),
		(184,""),
		(185,""),
		(186,""),
		(187,""),
		(188,""),
		(189,""),
		(190,""),
		(191,""),
		(192,""),
		(193,""),
		(194,""),
		(1000,""),
		(1001,""),
		(1002,""),
		(1003,""),
		(1004,""),
		(1005,""),
		(1006,""),
		(1007,""),
		(1008,""),
		(1009,""),
		(1010,""),
		(1011,""),
		(1012,""),
		(1013,""),
		(1014,""),
		(1015,""),
		(1016,""),
		(1017,""),
		(10000,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Lion's Claw: Skill of the Redmanes, who fought alongside General Radahn. Somersault forwards, striking foes with armament.'<<NEWLINE>><<NEWLINE>>Usable on swords, axes, and hammers<<NEWLINE>>(small armaments and thrusting swords excepted)."),
		(10100,"This Ash of War grants an armament the Keen affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Impaling Thrust: Skill that lets piercing armaments overcome enemy shields. Build power, then lunge forward for a strong thrust that pierces an enemy's guard.'<<NEWLINE>><<NEWLINE>>Usable on armaments capable of thrusting<<NEWLINE>>(colossal weapons excepted)."),
		(10200,"This Ash of War grants an armament the Keen affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Piercing Fang: Skill used by Yura, the Bloody Finger Hunter. Starting with the blade held horizontally, make a powerful thrust that cannot be blocked.'<<NEWLINE>><<NEWLINE>>Usable on medium and large armaments capable of thrusting."),
		(10300,"This Ash of War grants an armament the Keen affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Spinning Slash: Skill favored by dexterous warriors. Slash foes as your body spins. Additional input allows for a follow-up attack.'<<NEWLINE>><<NEWLINE>>Usable on swords, axes, and polearms (colossal weapons excepted)."),
		(10500,"This Ash of War grants an armament the Quality affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Charge Forth: Quickly charge forward with the armament at the hip, carrying the momentum into a thrust. Hold to cover a greater distance.'<<NEWLINE>><<NEWLINE>>Usable on polearms capable of thrusting, heavy thrusting swords, and twinblades."),
		(10600,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Stamp (Upward Cut): Brace armament and step into a low stance that prevents recoil from most enemy attacks. Follow up with a strong attack for an upward strike.'<<NEWLINE>><<NEWLINE>>Usable on swords, axes, and hammers (small and medium swords excepted)."),
		(10700,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Stamp (Sweep): Brace armament and step into a low stance that prevents recoil from most enemy attacks. Follow up with a strong attack for a sweeping strike.'<<NEWLINE>><<NEWLINE>>Usable on swords, axes, and hammers (small and medium swords excepted)."),
		(10800,"This Ash of War grants an armament the Blood affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Blood Tax: Blood Oath skill granted by the Lord of Blood. Twist to build power, then unleash a flurry of thrusts that rob the target of both their blood and their HP.'<<NEWLINE>><<NEWLINE>>Usable on armaments capable of thrusting<<NEWLINE>>(colossal weapons excepted)."),
		(10900,"This Ash of War grants an armament the Keen affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Repeating Thrust: Twist to build power, then unleash a flurry of thrusts.'<<NEWLINE>><<NEWLINE>>Usable on armaments capable of thrusting<<NEWLINE>>(colossal weapons excepted)."),
		(11000,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Wild Strikes: Swing armament with wild abandon. Hold to continue swinging. Can be followed up with a normal or strong attack.'<<NEWLINE>><<NEWLINE>>Usable on axes and hammers as well as curved swords and greatswords (colossal weapons excepted)."),
		(11100,"This Ash of War grants an armament the Quality affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Spinning Strikes: Polearm skill that performs continuous spinning attacks. Hold to continue the attack. Can be followed up with a normal or strong attack. Nullifies projectiles such as arrows while spinning.'<<NEWLINE>><<NEWLINE>>Usable on polearms (great spears excepted)."),
		(11200,"This Ash of War grants an armament the Keen affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Double Slash: Skill of superior swordsmen. Perform a crossing slash attack from a low stance. Repeated inputs allow for up to two follow-up attacks.'<<NEWLINE>><<NEWLINE>>Usable on swords and polearms capable of slashing (colossal weapons excepted)."),
		(11300,"This Ash of War grants an armament the Flame Art affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Prelate's Charge: Slam armament into the ground to create a surge of flames, then charge in. Hold to continue the charge.'<<NEWLINE>><<NEWLINE>>Usable on large and colossal axes and hammers."),
		(11400,"This Ash of War grants an armament the Keen affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Unsheathe: Skill of swordsmen from the Land of Reeds. Sheathe blade, holding it at the hip in a composed stance. Follow up with a normal or strong attack to perform a swift slash attack.'<<NEWLINE>><<NEWLINE>>Usable on katana."),
		(11500,"This Ash of War grants an armament the Quality affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Square Off: This skill starts with the sword held level. Follow up with a normal attack to slash upwards through enemy's guard, or a strong attack to perform a running thrust.'<<NEWLINE>><<NEWLINE>>Usable on straight swords."),
		(11600,"This Ash of War grants an armament the Quality affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Giant Hunt: Skill developed for confronting gigantic foes. Step forward from a low stance, carrying the momentum into a sudden upward thrust.'<<NEWLINE>><<NEWLINE>>Usable on large and colossal weapons capable of thrusting, spears, and twinblades."),
		(11700,"This Ash of War grants an armament<<NEWLINE>>the Skill: Torch Attack."),
		(11800,"This Ash of War grants an armament the Magic affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Loretta's Slash: Skill of Loretta the Royal Knight. Leap forward, imbuing the blade with glintstone, then descend, accelerating into a sweeping slash.'<<NEWLINE>><<NEWLINE>>Usable on polearms and twinblades."),
		(11900,"This Ash of War grants an armament the Poison affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Poison Moth Flight: Slash with a poison-infused blade. If the follow-up strike lands on a poisoned foe, it will deal significant damage.'<<NEWLINE>><<NEWLINE>>Usable on small and medium swords (twinblades excepted)."),
		(12000,"This Ash of War grants an armament the Magic affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Spinning Weapon: Defensive skill employed by Carian princesses. Lifts armament into mid-air, then makes it spin violently. Those it touches will suffer successive attacks.'<<NEWLINE>><<NEWLINE>>Usable on small and medium swords, axes, and hammers, as well as polearms and staves (great spears excepted)"),
		(12100,"Affix to weapon to enable use<<NEWLINE>>of the Skill: Wicked Stance"),
		(12200,"This Ash of War grants an armament the Quality affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Storm Assault: One of the skills that channel the tempests of Stormveil. Leap forward through surrounding storm winds and thrust armament downward. The attack will produce more storm winds at the point of impact.'<<NEWLINE>><<NEWLINE>>Usable on polearms capable of thrusting, heavy thrusting swords, and twinblades."),
		(12300,"This Ash of War grants an armament the Quality affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Stormcaller: One of the skills that channel the tempests of Stormveil. Spin armament to create surrounding storm winds. Repeated inputs allow for up to two follow-up attacks.'<<NEWLINE>><<NEWLINE>>Usable on swords capable of slashing, axes, hammers, and polearms (small and colossal weapons excepted)"),
		(12400,"This Ash of War grants an armament the Keen affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Sword Dance: Quickly close in to perform a series of spinning upward slashes. Follow up with an additional input to finish with a downward slash.'<<NEWLINE>><<NEWLINE>>Usable on swords, axes, and polearms capable of slashing (colossal weapons and great spears excepted)."),
		(12500,"This Ash of War grants an armament<<NEWLINE>>the Skill: Pulverize."),
		(20000,"This Ash of War grants an armament the Magic affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Glintblade Phalanx: Skill used by the enchanted knights who served the Carian royal family. Form an arch of magic glintblades overhead, which will attack foes automatically. Follow up with a strong attack to chain this skill into a lunging thrust.'<<NEWLINE>><<NEWLINE>>Usable on swords as well as polearms capable of thrusting (colossal weapons excepted)."),
		(20100,"This Ash of War grants an armament the Sacred affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Sacred Blade: Grants armament's attacks holy essence and fires off a golden blade projectile. The armament retains its holy essence for a while.'<<NEWLINE>><<NEWLINE>>Usable on melee armaments (whips, fists, and claws excepted)."),
		(20200,"This Ash of War grants an armament the Cold affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Ice Spear: Skill of the warriors who served Lunar Princess Ranni. Spin armament to release cold magic, then channel it into a piercing spear of ice.'<<NEWLINE>><<NEWLINE>>Usable on polearms capable of thrusting and twinblades."),
		(20300,"This Ash of War grants an armament the Magic affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Glintstone Pebble: Skill that employs the glintstone sorcery of the same name. Follow up with a strong attack to chain this skill into a lunging thrust, performed while the armament is still imbued with glintstone.'<<NEWLINE>><<NEWLINE>>Usable on swords as well as polearms capable of thrusting (colossal weapons excepted)."),
		(20400,"This Ash of War grants an armament the Blood affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Bloody Slash: Blood Oath skill granted by the Lord of Blood. From a low stance, coat the blade in your own blood to unleash a rending blood slash in a wide arc.'<<NEWLINE>><<NEWLINE>>Usable on swords (colossal weapons excepted)."),
		(20500,"This Ash of War grants an armament the Occult affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Lifesteal Fist: Skill that demonstrates mastery of the art of controlling vital energies.<<NEWLINE>>A slow, controlled punch with an energy-infused fist that renders foes unconscious and steals their HP. Only effective against foes of human build.'<<NEWLINE>><<NEWLINE>>Usable on fists and claws."),
		(20700,"This Ash of War grants an armament the Fire affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Eruption: Skill of the knights who serve at Volcano Manor. Slam armament into the ground, spawning roiling lava which spouts up upon release.'<<NEWLINE>><<NEWLINE>>Usable on large and colossal swords, axes, and hammers."),
		(20800,"This Ash of War grants an armament the Sacred affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Prayerful Strike: Raise armament aloft in prayer, then slam it into the ground. This inspired blow restores HP to the self and nearby allies if it successfully hits.'<<NEWLINE>><<NEWLINE>>Usable on axes and hammers."),
		(20900,"This Ash of War grants an armament the Magic affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Gravitas: Skill originating from the Alabaster Lords, who had skin of stone. Thrust the armament into the ground to create a gravity well. In addition to dealing damage, this attack pulls enemies in.'<<NEWLINE>><<NEWLINE>>Usable on melee armaments (small armaments and whips excepted)."),
		(21000,"This Ash of War grants an armament the Quality affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Storm Blade: Lost skill of Stormveil. Surround armament with shearing storm winds that can be fired forward. Can be fired in rapid succession.'<<NEWLINE>><<NEWLINE>>Usable on swords (colossal weapons and twinblades excepted)."),
		(21200,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Earthshaker: Thrust armament into the ground, then gather strength to unleash an earth-shaking shockwave. Follow up with a strong attack to swing the armament in a sweeping strike.'<<NEWLINE>><<NEWLINE>>Usable on greataxes, great hammers, great spears, and colossal weapons."),
		(21300,"This Ash of War grants an armament the Sacred affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Golden Land: Thrust armament into the ground, then gather strength to unleash a blast of sacred energy that coalesces into golden darts. Follow up with a strong attack to swing the armament in a sweeping strike.'<<NEWLINE>><<NEWLINE>>Usable on greataxes, great hammers, great spears, and colossal weapons."),
		(21400,"This Ash of War grants an armament the Fire affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Flaming Strike: Skill that emits flame in a wide frontward arc. Follow up with a strong attack to perform a lunging, sweeping strike. This will also coat the armament in fire.'<<NEWLINE>><<NEWLINE>>Usable on melee armaments (colossal weapons and whips excepted)."),
		(21600,"This Ash of War grants an armament the Lightning affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Thunderbolt: Skill used by the capital's ancient dragon cult. Raise armament aloft to call down a bolt of lightning. Can be fired in rapid succession.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(21700,"This Ash of War grants an armament the Lightning affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Lightning Slash: Call down a bolt of lightning into armament, then swing it down to create an explosive shock. The armament retains the lightning enchantment for a while.'<<NEWLINE>><<NEWLINE>>Usable on swords, axes, and hammers."),
		(21800,"This Ash of War grants an armament the Magic affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Carian Grandeur: Carian royal prestige embodied in a skill. Transform blade into a magical greatsword and swing it down. Can be charged to increase its power by up to two levels.'<<NEWLINE>><<NEWLINE>>Usable on swords (colossal weapons excepted)."),
		(21900,"This Ash of War grants an armament the Magic affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Carian Greatsword: Carian royal prestige embodied in a skill. Transform blade into a magical greatsword and swing it down. Can be charged to increase its power.'<<NEWLINE>><<NEWLINE>>Usable on swords (colossal weapons excepted)."),
		(22000,"This Ash of War grants an armament the Quality affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Vacuum Slice: Lost skill of ancient heroes. Hold the armament aloft to surround it with a shearing vacuum, then launch it forwards as a blade-like projectile.'<<NEWLINE>><<NEWLINE>>Usable on swords and axes (colossal axes excepted)."),
		(22100,"This Ash of War grants an armament the Flame Art affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Black Flame Tornado: Spin armament overhead and then plunge it into the ground to summon a raging vortex of black flames. Hold to create an initial flame tornado while spinning the armament.'<<NEWLINE>><<NEWLINE>>Usable on polearms and twinblades."),
		(22200,"This Ash of War grants an armament the Sacred affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Sacred Ring of Light: Skill used by the commanders of the Cleanrot Knights. Gather a sacred ring of light in the armament, then fire it forwards. Can be fired in rapid succession.'<<NEWLINE>><<NEWLINE>>Usable on polearms (great spears excepted)."),
		(22300,"This Ash of War grants an armament<<NEWLINE>>the Skill: Fire Breath."),
		(22400,"This Ash of War grants an armament the Blood affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Blood Blade: Wound self to coat the armament with blood, then unleash an airborne blood blade that causes hemorrhaging. Can be fired in rapid succession.'<<NEWLINE>><<NEWLINE>>Usable on small and medium swords."),
		(22500,"This Ash of War grants an armament the Quality affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Phantom Slash: Skill inspired by the fond remembrances of the Night's Cavalry. Creates an apparition of the knights' former instructor who guides a joint lunging upward swing. Additional input allows for a follow-up attack.'<<NEWLINE>><<NEWLINE>>Usable on polearms and twinblades (great spears excepted)."),
		(22600,"This Ash of War grants an armament the Occult affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Spectral Lance: Skill of the headless Mausoleum Knights. Hurl a phantasmic spear at foes.'<<NEWLINE>><<NEWLINE>>Usable on polearms (reapers excepted)."),
		(22700,"This Ash of War grants an armament the Cold affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Chilling Mist: Coat armament in frost, and then slash, spreading frigid mist forwards. The armament retains its frost for a while.'<<NEWLINE>><<NEWLINE>>Usable on melee armaments (whips, fists, and claws excepted)."),
		(22800,"This Ash of War grants an armament the Poison affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Poisonous Mist: Bathe armament in poison, and then slash, spreading toxic mist forwards. The armament retains its poison for a while.'<<NEWLINE>><<NEWLINE>>Usable on melee armaments (whips, fists, and claws excepted)."),
		(30000,"This Ash of War grants no affinity to an armament, but imparts the following skill:<<NEWLINE>><<NEWLINE>>'Shield Bash: Brace behind shield before using bodyweight to ram foes while maintaining guarding stance. Weaker enemies will be shoved backwards, and can even be staggered.'<<NEWLINE>><<NEWLINE>>Usable on all shields."),
		(30100,"This Ash of War grants no affinity to an armament, but imparts the following skill:<<NEWLINE>><<NEWLINE>>'Barricade Shield: Skill made famous by Sir Neidhardt. Focus your energy into the shield, temporarily hardening it to deflect greater blows.'<<NEWLINE>><<NEWLINE>>Usable on all shields."),
		(30200,"This Ash of War grants no affinity to an armament, but imparts the following skill:<<NEWLINE>><<NEWLINE>>'Parry: Use this skill in time with a foe's melee attack to deflect it and break that foe's stance. This provides an opening to perform a critical hit.'<<NEWLINE>><<NEWLINE>>Usable on daggers, curved swords, thrusting swords, fists, claws, and small and medium shields."),
		(30300,"This Ash of War grants an armament<<NEWLINE>>the Skill: Buckler Parry."),
		(30400,"This Ash of War grants an armament<<NEWLINE>>the Skill: Parry."),
		(30500,"This Ash of War grants an armament the Magic affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Carian Retaliation: Swing the shield to dispel incoming sorceries and incantations, transforming the magic into retaliatory glintblades. Can also be used in the same way as a regular parry.'<<NEWLINE>><<NEWLINE>>Usable on small and medium shields."),
		(30600,"This Ash of War grants no affinity to an armament, but imparts the following skill:<<NEWLINE>><<NEWLINE>>'Storm Wall: Swing the shield to create a wall of storm winds in front of you, deflecting arrows and other such physical projectiles. Can also be used in the same way as a regular parry.'<<NEWLINE>><<NEWLINE>>Usable on small and medium shields."),
		(30700,"This Ash of War grants an armament the Sacred affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Golden Parry: Perform an Erdtree incantation and swing the shield to deflect enemy attacks and break their stance. Effective even at a slight distance.'<<NEWLINE>><<NEWLINE>>Usable on small and medium shields."),
		(30800,"This Ash of War grants no affinity to an armament, but imparts the following skill:<<NEWLINE>><<NEWLINE>>'Shield Crash: Two-hand the shield and charge forwards while maintaining guard. Weaker enemies will be shoved backwards and can even be staggered. Hold to extend the duration of the charge forwards.'<<NEWLINE>><<NEWLINE>>Usable on all shields."),
		(30900,"This Ash of War grants no affinity to an armament and replaces any skill present with the following:<<NEWLINE>><<NEWLINE>>'No skill: This armament has no skill. If the armament in the other hand has a skill, that skill will be used instead.'<<NEWLINE>><<NEWLINE>>Usable on shields and torches."),
		(31000,"This Ash of War grants an armament the Magic affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Thops's Barrier: Erect a magical forcefield while swinging the shield to deflect sorceries and incantations. Can also be used in the same way as a regular parry.'<<NEWLINE>><<NEWLINE>>Usable on small and medium shields."),
		(40000,"This Ash of War grants no affinity to an armament, but imparts the following skill:<<NEWLINE>><<NEWLINE>>'Through and Through: Powerful archery skill using a greatbow held in an oblique stance. Ready the greatbow, then twist the bowstring to fire a mighty greatarrow that can penetrate through enemies.'<<NEWLINE>><<NEWLINE>>Usable on greatbows."),
		(40100,"This Ash of War grants no affinity to an armament, but imparts the following skill:<<NEWLINE>><<NEWLINE>>'Barrage: Archery skill using a bow held horizontally. Ready the bow, then fire off a rapid succession of shots faster than the eye can see.'<<NEWLINE>><<NEWLINE>>Usable on light bows."),
		(40200,"This Ash of War grants no affinity to an armament, but imparts the following skill:<<NEWLINE>><<NEWLINE>>'Mighty Shot: Archery skill performed from an oblique stance. Ready the bow, then pull the bowstring to its limit to enhance the power of the shot, penetrating the enemy's guard.'<<NEWLINE>><<NEWLINE>>Usable on light bows and longbows."),
		(40400,"This Ash of War grants no affinity to an armament, but imparts the following skill:<<NEWLINE>><<NEWLINE>>'Enchanted Shot: Archery skill that enlivens the arrow with spiritual essence. The resulting shot will fly faster than regular shots and change its trajectory to follow the target.'<<NEWLINE>><<NEWLINE>>Usable on light bows and longbows."),
		(40500,"This Ash of War grants no affinity to an armament, but imparts the following skill:<<NEWLINE>><<NEWLINE>>'Sky Shot: Archery skill performed from a low stance. Ready the bow, then fire an arrow high up into the air, arcing so as to strike the enemy from above when it comes down.'<<NEWLINE>><<NEWLINE>>Usable on light bows and longbows."),
		(40600,"This Ash of War grants no affinity to an armament, but imparts the following skill:<<NEWLINE>><<NEWLINE>>'Rain of Arrows: Archery skill performed from a low stance. Ready the bow, then fire a burst of arrows into the sky to shower the enemy with projectiles.'<<NEWLINE>><<NEWLINE>>Usable on all bows."),
		(40700,"Affix to weapon to enable use<<NEWLINE>>of the Skill: Invisible Arrow"),
		(50100,"This Ash of War grants an armament the Cold affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Hoarfrost Stomp: Stomp hard to spread a trail of freezing mist on the ground. The mist applies the frost status effect.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(50200,"This Ash of War grants an armament the Quality affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Storm Stomp: One of the skills that channel the tempests of Stormveil. Stomp hard on the ground to kick up a momentary storm.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(50300,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Kick: Push an enemy back with a high kick. Effective against enemies who are guarding, and can break a foe's stance. Sometimes a simple tool is the most effective.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(50400,"This Ash of War grants an armament the Lightning affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Lightning Ram: Skill inspired by tumbling rams. Let out a bleat, then tumble forwards, clad in lightning. Tumbles can be repeated in rapid succession.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(50500,"This Ash of War grants an armament the Fire affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Flame of the Redmanes: Skill of the Redmanes, who fought alongside General Radahn. Produce a powerful burst of flames in a wide frontward arc.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(50600,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Ground Slam: Jump up high into the air and crash down on the ground ahead. The resulting pratfall sends a powerful shockwave in all directions.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(50700,"This Ash of War grants an armament the Sacred affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Golden Slam: Skill of the avatars who protect Minor Erdtrees. Jump up high into the air and crash down on the ground ahead. The resulting pratfall sends golden shockwaves in all directions.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(50800,"This Ash of War grants an armament the Magic affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Waves of Darkness: Plunge armament into the ground to release three waves of darkness. Follow up with a strong attack to swing the armament in a sweeping strike.'<<NEWLINE>><<NEWLINE>>Usable on greataxes, great hammers, great spears, and colossal weapons."),
		(50900,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Hoarah Loux's Earthshaker: Slam both hands onto the ground to violently shake the earth and unleash a shockwave. Follow up with an additional input to slam the ground again.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(60000,"This Ash of War grants an armament the Quality affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Determination: A knightly skill. Hold the flat of the armament to your face and pledge your resolve, powering up your next attack.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(60100,"This Ash of War grants an armament the Quality affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Royal Knight's Resolve: Skill of the knights who once served the Elden Lord. Hold the flat of the armament to your face and pledge your resolve, greatly powering up your next attack.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(60200,"This Ash of War grants an armament the Occult affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Assassin's Gambit: Skill that masks the user's presence at the cost of a self-inflicted wound. Grants near-invisibility and silences footsteps.'<<NEWLINE>><<NEWLINE>>Usable on small and medium straight swords and thrusting swords."),
		(60300,"This Ash of War grants an armament the Sacred affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Golden Vow: Skill passed down from antiquity among the knights of the capital. Raise armament aloft and pledge to honor the Erdtree in battle, granting self and nearby allies increased attack power and defense.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(60400,"This Ash of War grants an armament the Sacred affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Sacred Order: Skill of the Golden Order fundamentalist knights. Perform a salute and grant the armament holy essence. Highly effective against Those Who Live in Death.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(60500,"This Ash of War grants an armament the Sacred affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Shared Order: Skill of the Golden Order fundamentalist knights. Grant the armament and those of nearby allies holy essence. Highly effective against Those Who Live in Death.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(60600,"This Ash of War grants an armament the Blood affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Seppuku: A forbidden technique used by swordsmen from the Land of Reeds. Plunge the blade into your stomach to stain it with blood. Increases attack power and improves ability to inflict blood loss.'<<NEWLINE>><<NEWLINE>>Usable on swords as well as polearms capable of thrusting (small and colossal weapons excepted)."),
		(60700,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Cragblade: Skill that manipulates gravity. Bury the armament in the ground, pulling rocks from the earth to reinforce it. Increases attack power and makes it easier to break enemy stance.'<<NEWLINE>><<NEWLINE>>Usable on melee armaments (whips excepted)."),
		(65000,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Barbaric Roar: Let loose a bestial roar to rally the spirit and increase attack power. While active, strong attacks change to savage combo attacks.'<<NEWLINE>><<NEWLINE>>Usable on melee armaments (daggers, thrusting swords, and whips excepted)."),
		(65100,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'War Cry: Give a war cry to rally the spirit and increase attack power. While active, strong attacks change to charging attacks.'<<NEWLINE>><<NEWLINE>>Usable on melee armaments (daggers, thrusting swords, and whips excepted)."),
		(65200,"This Ash of War grants an armament the Keen affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Beast's Roar: Unleash a beastly roar, rending the air as a forward-travelling projectile.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(65300,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Troll's Roar: Look into the distance and let out an intense roar, generating a powerful shockwave that blows back surrounding foes. Follow up with a strong attack to slam the armament down.'<<NEWLINE>><<NEWLINE>>Usable on large and colossal swords, axes, and hammers."),
		(65400,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Braggart's Roar: Declare your presence with a boastful roar. Raises attack power, defense, and stamina recovery speed.'<<NEWLINE>><<NEWLINE>>Usable on melee armaments (daggers, thrusting swords, and whips excepted)."),
		(70000,"This Ash of War grants an armament the Heavy affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Endure: Assume an anchored stance to brace for incoming attacks, briefly boosting poise. Damage taken while using this skill is reduced.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(70100,"This Ash of War grants an armament the Sacred affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Vow of the Indomitable: Skill of the ancient warriors of the Erdtree. Hold shield aloft to imbue yourself with golden power, granting momentary invincibility.'<<NEWLINE>><<NEWLINE>>Usable on all shields."),
		(70200,"This Ash of War grants an armament the Sacred affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Holy Ground: Raise shield to create an Erdtree-consecrated area that continuously restores HP and boosts defense for self and allies inside it.'<<NEWLINE>><<NEWLINE>>Usable on all shields."),
		(80000,"This Ash of War grants an armament the Keen affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Quickstep: Skill prized by the crafty and fleet of foot.<<NEWLINE>>Perform a quickstep maneuver that allows for circling around lock-on targets.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(80100,"This Ash of War grants an armament the Keen affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Bloodhound's Step: Skill that allows the user to become temporarily invisible while dodging at high speed. Moves faster and travels farther than a regular quickstep. This skill can be used to circle around lock-on targets.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(80200,"This Ash of War grants an armament the Keen affinity and the following skill:<<NEWLINE>><<NEWLINE>>'Raptor of the Mists: Duck into a low stance, momentarily vanishing. If an enemy attack connects, avian wings will allow for a quick escape into the air.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
		(85000,"This Ash of War grants an armament the Occult affinity and the following skill:<<NEWLINE>><<NEWLINE>>'White Shadow's Lure: Hold armament in a brief, silent prayer to create a white shadow. The apparition lures in foes of human build who are not in combat, drawing their aggression. Effective on demi-humans even if they are already in a combat state.'<<NEWLINE>><<NEWLINE>>Usable on all melee armaments."),
])});
