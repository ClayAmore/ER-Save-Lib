use std::{collections::HashMap, sync::LazyLock};

pub static PROTECTOR_NAME: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(200,""),
		(1000,"[ERROR]Type 1"),
		(1100,"[ERROR]Type 2"),
		(1200,"[ERROR]Type 3"),
		(1300,"[ERROR]Type 4"),
		(1400,"[ERROR]Type 5"),
		(1500,"[ERROR]Type 6"),
		(1600,"[ERROR]Type 7"),
		(1700,"[ERROR]Type 8"),
		(1800,"[ERROR]Type 9"),
		(1900,"[ERROR]Type 10"),
		(2000,"[ERROR]Type 11"),
		(2100,"[ERROR]Type 12"),
		(2200,"[ERROR]Type 13"),
		(2300,"[ERROR]Type 14"),
		(2400,"[ERROR]Type 15"),
		(2500,"[ERROR]Type 16"),
		(2600,"[ERROR]Type 17"),
		(2700,"[ERROR]Type 18"),
		(2800,"[ERROR]Type 19"),
		(2900,"[ERROR]Type 20"),
		(3000,"[ERROR]Type 1"),
		(3100,"[ERROR]Type 2"),
		(3200,"[ERROR]Type 3"),
		(3300,"[ERROR]Type 4"),
		(3400,"[ERROR]Type 5"),
		(3500,"[ERROR]Type 6"),
		(3600,"[ERROR]Type 7"),
		(3700,"[ERROR]Type 8"),
		(3800,"[ERROR]Type 9"),
		(3900,"[ERROR]Type 10"),
		(4000,"[ERROR]Type 11"),
		(4100,"[ERROR]Type 12"),
		(4200,"[ERROR]Type 13"),
		(4300,"[ERROR]Type 14"),
		(4400,"[ERROR]Type 15"),
		(4500,"[ERROR]Type 16"),
		(4600,"[ERROR]Type 17"),
		(4700,"[ERROR]Type 18"),
		(4800,"[ERROR]Type 19"),
		(4900,"[ERROR]Type 20"),
		(5000,"[ERROR]Travel Hairstyle"),
		(10000,"Head"),
		(10100,"Body"),
		(10200,"Arms"),
		(10300,"Legs"),
		(40000,"Iron Helmet"),
		(40100,"Scale Armor"),
		(40200,"Iron Gauntlets"),
		(40300,"Leather Trousers"),
		(50000,"Kaiden Helm"),
		(50100,"Kaiden Armor"),
		(50200,"Kaiden Gauntlets"),
		(50300,"Kaiden Trousers"),
		(60000,"Drake Knight Helm"),
		(60100,"Drake Knight Armor"),
		(60200,"Drake Knight Gauntlets"),
		(60300,"Drake Knight Greaves"),
		(61000,"Drake Knight Helm (Altered)"),
		(61100,"Drake Knight Armor (Altered)"),
		(80000,"Scaled Helm"),
		(80100,"Scaled Armor"),
		(80200,"Scaled Gauntlets"),
		(80300,"Scaled Greaves"),
		(81100,"Scaled Armor (Altered)"),
		(90000,"Perfumer Hood"),
		(90100,"Perfumer Robe"),
		(90200,"Perfumer Gloves"),
		(90300,"Perfumer Sarong"),
		(91100,"Perfumer Robe (Altered)"),
		(100000,"Traveler's Hat"),
		(100100,"Perfumer's Traveling Garb"),
		(100200,"Traveler's Gloves"),
		(100300,"Traveler's Slops"),
		(101100,"Perfumer's Traveling Garb (Altered)"),
		(120000,"Alberich's Pointed Hat"),
		(120100,"Alberich's Robe"),
		(120200,"Alberich's Bracers"),
		(120300,"Alberich's Trousers"),
		(121000,"Alberich's Pointed Hat (Altered)"),
		(121100,"Alberich's Robe (Altered)"),
		(130000,"Spellblade's Pointed Hat"),
		(130100,"Spellblade's Traveling Attire"),
		(130200,"Spellblade's Gloves"),
		(130300,"Spellblade's Trousers"),
		(131100,"Spellblade's Traveling Attire (Altered)"),
		(140000,"Bull-Goat Helm"),
		(140100,"Bull-Goat Armor"),
		(140200,"Bull-Goat Gauntlets"),
		(140300,"Bull-Goat Greaves"),
		(150000,"Iron Kasa"),
		(150100,"Ronin's Armor"),
		(150200,"Ronin's Gauntlets"),
		(150300,"Ronin's Greaves"),
		(151100,"Ronin's Armor (Altered)"),
		(160000,"Guilty Hood"),
		(160100,"Cloth Garb"),
		(160200,"Tarnished Wrap"),
		(160300,"Cloth Trousers"),
		(170000,"Black Wolf Mask"),
		(170100,"Blaidd's Armor"),
		(170200,"Blaidd's Gauntlets"),
		(170300,"Blaidd's Greaves"),
		(171100,"Blaidd's Armor (Altered)"),
		(180000,"Black Knife Hood"),
		(180100,"Black Knife Armor"),
		(180200,"Black Knife Gauntlets"),
		(180300,"Black Knife Greaves"),
		(181100,"Black Knife Armor (Altered)"),
		(190000,"Exile Hood"),
		(190100,"Exile Armor"),
		(190200,"Exile Gauntlets"),
		(190300,"Exile Greaves"),
		(200000,"Banished Knight Helm"),
		(200100,"Banished Knight Armor"),
		(200200,"Banished Knight Gauntlets"),
		(200300,"Banished Knight Greaves"),
		(201000,"Banished Knight Helm (Altered)"),
		(201100,"Banished Knight Armor (Altered)"),
		(202100,"[ERROR]"),
		(210000,"Briar Helm"),
		(210100,"Briar Armor"),
		(210200,"Briar Gauntlets"),
		(210300,"Briar Greaves"),
		(211100,"Briar Armor (Altered)"),
		(220000,"Page Hood"),
		(220100,"Page Garb"),
		(220300,"Page Trousers"),
		(221100,"Page Garb (Altered)"),
		(230000,"Night's Cavalry Helm"),
		(230100,"Night's Cavalry Armor"),
		(230200,"Night's Cavalry Gauntlets"),
		(230300,"Night's Cavalry Greaves"),
		(231000,"Night's Cavalry Helm (Altered)"),
		(231100,"Night's Cavalry Armor (Altered)"),
		(240000,"Blue Silver Mail Hood"),
		(240100,"Blue Silver Mail Armor"),
		(240200,"Blue Silver Bracelets"),
		(240300,"Blue Silver Mail Skirt"),
		(241100,"Blue Silver Mail Armor (Altered)"),
		(250000,"Nomadic Merchant's Chapeau"),
		(250100,"Nomadic Merchant's Finery"),
		(250300,"Nomadic Merchant's Trousers"),
		(251100,"Nomadic Merchant's Finery (Altered)"),
		(260000,"Malformed Dragon Helm"),
		(260100,"Malformed Dragon Armor"),
		(260200,"Malformed Dragon Gauntlets"),
		(260300,"Malformed Dragon Greaves"),
		(270000,"Tree Sentinel Helm"),
		(270100,"Tree Sentinel Armor"),
		(270200,"Tree Sentinel Gauntlets"),
		(270300,"Tree Sentinel Greaves"),
		(271100,"Tree Sentinel Armor (Altered)"),
		(280000,"Royal Knight Helm"),
		(280100,"Royal Knight Armor"),
		(280200,"Royal Knight Gauntlets"),
		(280300,"Royal Knight Greaves"),
		(281100,"Royal Knight Armor (Altered)"),
		(290000,"Nox Monk Hood"),
		(290100,"Nox Monk Armor"),
		(290200,"Nox Bracelets"),
		(290300,"Nox Greaves"),
		(291000,"Nox Monk Hood (Altered)"),
		(291100,"Nox Monk Armor (Altered)"),
		(292000,"Nox Swordstress Crown"),
		(292100,"Nox Swordstress Armor"),
		(293000,"Night Maiden Twin Crown"),
		(293100,"Night Maiden Armor"),
		(294000,"Nox Swordstress Crown (Altered)"),
		(294100,"Nox Swordstress Armor (Altered)"),
		(300000,"Great Horned Headband"),
		(300100,"Fur Raiment"),
		(300300,"Fur Leggings"),
		(301000,"Shining Horned Headband"),
		(301100,"Shaman Furs"),
		(301300,"Shaman Leggings"),
		(310000,"Duelist Helm"),
		(310100,"Gravekeeper Cloak"),
		(310300,"Duelist Greaves"),
		(311100,"Gravekeeper Cloak (Altered)"),
		(320000,"Sanguine Noble Hood"),
		(320100,"Sanguine Noble Robe"),
		(320300,"Sanguine Noble Waistcloth"),
		(330000,"Guardian Mask"),
		(330100,"Guardian Garb (Full Bloom)"),
		(330200,"Guardian Bracers"),
		(330300,"Guardian Greaves"),
		(331100,"Guardian Garb"),
		(340000,"Cleanrot Helm"),
		(340100,"Cleanrot Armor"),
		(340200,"Cleanrot Gauntlets"),
		(340300,"Cleanrot Greaves"),
		(341000,"Cleanrot Helm (Altered)"),
		(341100,"Cleanrot Armor (Altered)"),
		(350000,"Fire Monk Hood"),
		(350100,"Fire Monk Armor"),
		(350200,"Fire Monk Gauntlets"),
		(350300,"Fire Monk Greaves"),
		(351000,"Blackflame Monk Hood"),
		(351100,"Blackflame Monk Armor"),
		(351200,"Blackflame Monk Gauntlets"),
		(351300,"Blackflame Monk Greaves"),
		(360000,"Fire Prelate Helm"),
		(360100,"Fire Prelate Armor"),
		(360200,"Fire Prelate Gauntlets"),
		(360300,"Fire Prelate Greaves"),
		(361100,"Fire Prelate Armor (Altered)"),
		(370000,"Aristocrat Headband"),
		(370100,"Aristocrat Garb"),
		(370300,"Aristocrat Boots"),
		(371100,"Aristocrat Garb (Altered)"),
		(380000,"Aristocrat Hat"),
		(380100,"Aristocrat Coat"),
		(390000,"Old Aristocrat Cowl"),
		(390100,"Old Aristocrat Gown"),
		(390300,"Old Aristocrat Shoes"),
		(420000,"Vulgar Militia Helm"),
		(420100,"Vulgar Militia Armor"),
		(420200,"Vulgar Militia Gauntlets"),
		(420300,"Vulgar Militia Greaves"),
		(430000,"Sage Hood"),
		(430100,"Sage Robe"),
		(430300,"Sage Trousers"),
		(440000,"Pumpkin Helm"),
		(460000,"Elden Lord Crown"),
		(460100,"Elden Lord Armor"),
		(460200,"Elden Lord Bracers"),
		(460300,"Elden Lord Greaves"),
		(461100,"Elden Lord Armor (Altered)"),
		(470000,"Radahn's Redmane Helm"),
		(470100,"Radahn's Lion Armor"),
		(470200,"Radahn's Gauntlets"),
		(470300,"Radahn's Greaves"),
		(471100,"Radahn's Lion Armor (Altered)"),
		(480100,"Lord of Blood's Robe"),
		(481100,"Lord of Blood's Robe (Altered)"),
		(510000,"Queen's Crescent Crown"),
		(510100,"Queen's Robe"),
		(510200,"Queen's Bracelets"),
		(510300,"Queen's Leggings"),
		(520000,"Godskin Apostle Hood"),
		(520100,"Godskin Apostle Robe"),
		(520200,"Godskin Apostle Bracelets"),
		(520300,"Godskin Apostle Trousers"),
		(530000,"Godskin Noble Hood"),
		(530100,"Godskin Noble Robe"),
		(530200,"Godskin Noble Bracelets"),
		(530300,"Godskin Noble Trousers"),
		(540000,"Depraved Perfumer Headscarf"),
		(540100,"Depraved Perfumer Robe"),
		(540200,"Depraved Perfumer Gloves"),
		(540300,"Depraved Perfumer Trousers"),
		(541100,"Depraved Perfumer Robe (Altered)"),
		(570000,"Crucible Axe Helm"),
		(570100,"Crucible Axe Armor"),
		(570200,"Crucible Gauntlets"),
		(570300,"Crucible Greaves"),
		(571000,"Crucible Tree Helm"),
		(571100,"Crucible Tree Armor"),
		(572100,"Crucible Axe Armor (Altered)"),
		(573100,"Crucible Tree Armor (Altered)"),
		(580000,"Lusat's Glintstone Crown"),
		(580100,"Lusat's Robe"),
		(580200,"Lusat's Manchettes"),
		(580300,"Old Sorcerer's Legwraps"),
		(581000,"Azur's Glintstone Crown"),
		(581100,"Azur's Glintstone Robe"),
		(581200,"Azur's Manchettes"),
		(590000,"All-Knowing Helm"),
		(590100,"All-Knowing Armor"),
		(590200,"All-Knowing Gauntlets"),
		(590300,"All-Knowing Greaves"),
		(591100,"All-Knowing Armor (Altered)"),
		(600000,"Twinned Helm"),
		(600100,"Twinned Armor"),
		(600200,"Twinned Gauntlets"),
		(600300,"Twinned Greaves"),
		(601100,"Twinned Armor (Altered)"),
		(601300,"Twinned Greaves (Altered)"),
		(610000,"Ragged Hat"),
		(610100,"Ragged Armor"),
		(610200,"Ragged Gloves"),
		(610300,"Ragged Loincloth"),
		(611000,"Ragged Hat (Altered)"),
		(611100,"Ragged Armor (Altered)"),
		(620000,"Prophet Blindfold"),
		(620100,"Corhyn's Robe"),
		(620300,"Prophet Trousers"),
		(621100,"Prophet Robe (Altered)"),
		(622100,"Prophet Robe"),
		(630000,"Astrologer Hood"),
		(630100,"Astrologer Robe"),
		(630200,"Astrologer Gloves"),
		(630300,"Astrologer Trousers"),
		(631100,"Astrologer Robe (Altered)"),
		(640000,"Lionel's Helm"),
		(640100,"Lionel's Armor"),
		(640200,"Lionel's Gauntlets"),
		(640300,"Lionel's Greaves"),
		(641100,"Lionel's Armor (Altered)"),
		(650000,"Hoslow's Helm"),
		(650100,"Hoslow's Armor"),
		(650200,"Hoslow's Gauntlets"),
		(650300,"Hoslow's Greaves"),
		(651000,"Diallos's Mask"),
		(652100,"Hoslow's Armor (Altered)"),
		(660000,"Vagabond Knight Helm"),
		(660100,"Vagabond Knight Armor"),
		(660200,"Vagabond Knight Gauntlets"),
		(660300,"Vagabond Knight Greaves"),
		(661100,"Vagabond Knight Armor (Altered)"),
		(670000,"Blue Cloth Cowl"),
		(670100,"Blue Cloth Vest"),
		(670200,"Warrior Gauntlets"),
		(670300,"Warrior Greaves"),
		(680000,"White Mask"),
		(680100,"War Surgeon Gown"),
		(680200,"War Surgeon Gloves"),
		(680300,"War Surgeon Trousers"),
		(681100,"War Surgeon Gown (Altered)"),
		(690000,"Royal Remains Helm"),
		(690100,"Royal Remains Armor"),
		(690200,"Royal Remains Gauntlets"),
		(690300,"Royal Remains Greaves"),
		(700000,"[ERROR]Brave's Cord Circlet"),
		(700100,"[ERROR]Brave's Battlewear"),
		(700200,"[ERROR]Brave's Bracer"),
		(700300,"[ERROR]Brave's Legwraps"),
		(701000,"[ERROR]Brave's Leather Helm"),
		(702000,"[ERROR]Brave's Battlewear (Altered)"),
		(720000,"Beast Champion Helm"),
		(720100,"Beast Champion Armor"),
		(720200,"Beast Champion Gauntlets"),
		(720300,"Beast Champion Greaves"),
		(721100,"Beast Champion Armor (Altered)"),
		(730000,"Champion Headband"),
		(730100,"Champion Pauldron"),
		(730200,"Champion Bracers"),
		(730300,"Champion Gaiters"),
		(740000,"Crimson Hood"),
		(740100,"Noble's Traveling Garb"),
		(740200,"Noble's Gloves"),
		(740300,"Noble's Trousers"),
		(741000,"Navy Hood"),
		(742000,"[ERROR]"),
		(760000,"Maliketh's Helm"),
		(760100,"Maliketh's Armor"),
		(760200,"Maliketh's Gauntlets"),
		(760300,"Maliketh's Greaves"),
		(761100,"Maliketh's Armor (Altered)"),
		(770000,"Malenia's Winged Helm"),
		(770100,"Malenia's Armor"),
		(770200,"Malenia's Gauntlet"),
		(770300,"Malenia's Greaves"),
		(771100,"Malenia's Armor (Altered)"),
		(780000,"Veteran's Helm"),
		(780100,"Veteran's Armor"),
		(780200,"Veteran's Gauntlets"),
		(780300,"Veteran's Greaves"),
		(781100,"Veteran's Armor (Altered)"),
		(790000,"Bloodhound Knight Helm"),
		(790100,"Bloodhound Knight Armor"),
		(790200,"Bloodhound Knight Gauntlets"),
		(790300,"Bloodhound Knight Greaves"),
		(791100,"Bloodhound Knight Armor (Altered)"),
		(800000,"Festive Hood"),
		(800100,"Festive Garb"),
		(801000,"Festive Hood (Altered)"),
		(801100,"Festive Garb (Altered)"),
		(802000,"Blue Festive Hood"),
		(802100,"Blue Festive Garb"),
		(810000,"Commoner's Headband"),
		(810100,"Commoner's Garb"),
		(810300,"Commoner's Shoes"),
		(811000,"Commoner's Headband (Altered)"),
		(811100,"Commoner's Garb (Altered)"),
		(812000,"Commoner's Simple Garb"),
		(812100,"Commoner's Simple Garb (Altered)"),
		(820000,"Envoy Crown"),
		(830000,"Twinsage Glintstone Crown"),
		(830100,"Raya Lucarian Robe"),
		(830200,"Sorcerer Manchettes"),
		(830300,"Sorcerer Leggings"),
		(831000,"Olivinus Glintstone Crown"),
		(832000,"Lazuli Glintstone Crown"),
		(833000,"Karolos Glintstone Crown"),
		(834000,"Witch's Glintstone Crown"),
		(834100,"[ERROR]"),
		(835000,"[ERROR]"),
		(835100,"[ERROR]"),
		(835200,"[ERROR]"),
		(835300,"[ERROR]"),
		(840000,"Marionette Soldier Helm"),
		(840100,"Marionette Soldier Armor"),
		(850000,"Marionette Soldier Birdhelm"),
		(860000,"Raging Wolf Helm"),
		(860100,"Raging Wolf Armor"),
		(860200,"Raging Wolf Gauntlets"),
		(860300,"Raging Wolf Greaves"),
		(861100,"Raging Wolf Armor (Altered)"),
		(870000,"Land of Reeds Helm"),
		(870100,"Land of Reeds Armor"),
		(870200,"Land of Reeds Gauntlets"),
		(870300,"Land of Reeds Greaves"),
		(871100,"Land of Reeds Armor (Altered)"),
		(872000,"Okina Mask"),
		(872100,"White Reed Armor"),
		(872200,"White Reed Gauntlets"),
		(872300,"White Reed Greaves"),
		(873000,"[ERROR]"),
		(880000,"Confessor Hood"),
		(880100,"Confessor Armor"),
		(880200,"Confessor Gloves"),
		(880300,"Confessor Boots"),
		(881000,"Confessor Hood (Altered)"),
		(881100,"Confessor Armor (Altered)"),
		(890000,"Prisoner Iron Mask"),
		(890100,"Prisoner Clothing"),
		(890300,"Prisoner Trousers"),
		(891000,"Blackguard's Iron Mask"),
		(892100,"[ERROR]"),
		(900000,"Traveling Maiden Hood"),
		(900100,"Traveling Maiden Robe"),
		(900200,"Traveling Maiden Gloves"),
		(900300,"Traveling Maiden Boots"),
		(901100,"Traveling Maiden Robe (Altered)"),
		(901200,"[ERROR]"),
		(901600,"[ERROR]"),
		(902000,"Finger Maiden Fillet"),
		(902100,"Finger Maiden Robe"),
		(902300,"Finger Maiden Shoes"),
		(903100,"Finger Maiden Robe (Altered)"),
		(904100,"[ERROR]"),
		(910000,"Preceptor's Big Hat"),
		(910100,"Preceptor's Long Gown"),
		(910200,"Preceptor's Gloves"),
		(910300,"Preceptor's Trousers"),
		(911000,"Mask of Confidence"),
		(911100,"Preceptor's Long Gown (Altered)"),
		(912000,"[ERROR]"),
		(912100,"[ERROR]"),
		(912200,"[ERROR]"),
		(912300,"[ERROR]"),
		(913000,"[ERROR]"),
		(920000,"[ERROR]Grass Hair Ornament"),
		(930000,"Skeletal Mask"),
		(930100,"Raptor's Black Feathers"),
		(930200,"Bandit Manchettes"),
		(930300,"Bandit Boots"),
		(931100,"Bandit Garb"),
		(940000,"Eccentric's Hood"),
		(940100,"Eccentric's Armor"),
		(940200,"Eccentric's Manchettes"),
		(940300,"Eccentric's Breeches"),
		(941000,"Eccentric's Hood (Altered)"),
		(950000,"Fingerprint Helm"),
		(950100,"Fingerprint Armor"),
		(950200,"Fingerprint Gauntlets"),
		(950300,"Fingerprint Greaves"),
		(951100,"Fingerprint Armor (Altered)"),
		(960000,"Consort's Mask"),
		(960100,"Consort's Robe"),
		(960300,"Consort's Trousers"),
		(961000,"Ruler's Mask"),
		(961100,"Ruler's Robe"),
		(962100,"Upper-Class Robe"),
		(963000,"Marais Mask"),
		(963100,"Marais Robe"),
		(963200,"Bloodsoaked Manchettes"),
		(964000,"Bloodsoaked Mask"),
		(964100,"Official's Attire"),
		(965000,"[ERROR]"),
		(965100,"[ERROR]"),
		(965300,"[ERROR]"),
		(966000,"[ERROR]"),
		(967000,"[ERROR]"),
		(970000,"Omen Helm"),
		(970100,"Omen Armor"),
		(970200,"Omen Gauntlets"),
		(970300,"Omen Greaves"),
		(980000,"Carian Knight Helm"),
		(980100,"Carian Knight Armor"),
		(980200,"Carian Knight Gauntlets"),
		(980300,"Carian Knight Greaves"),
		(981100,"Carian Knight Armor (Altered)"),
		(990000,"Hierodas Glintstone Crown"),
		(990100,"Errant Sorcerer Robe"),
		(990200,"Errant Sorcerer Manchettes"),
		(990300,"Errant Sorcerer Boots"),
		(991100,"Errant Sorcerer Robe (Altered)"),
		(992000,"[ERROR]"),
		(1000000,"Haima Glintstone Crown"),
		(1000100,"Battlemage Robe"),
		(1000200,"Battlemage Manchettes"),
		(1000300,"Battlemage Legwraps"),
		(1010000,"Snow Witch Hat"),
		(1010100,"Snow Witch Robe"),
		(1010300,"Snow Witch Skirt"),
		(1011100,"Snow Witch Robe (Altered)"),
		(1020100,"Traveler's Clothes"),
		(1020200,"Traveler's Manchettes"),
		(1020300,"Traveler's Boots"),
		(1030000,"Juvenile Scholar Cap"),
		(1030100,"Juvenile Scholar Robe"),
		(1040000,"Radiant Gold Mask"),
		(1040100,"Goldmask's Rags"),
		(1040200,"Gold Bracelets"),
		(1040300,"Gold Waistwrap"),
		(1050100,"Fell Omen Cloak"),
		(1060000,"Albinauric Mask"),
		(1060100,"Dirty Chainmail"),
		(1070000,"Zamor Mask"),
		(1070100,"Zamor Armor"),
		(1070200,"Zamor Bracelets"),
		(1070300,"Zamor Legwraps"),
		(1080000,"Imp Head (Cat)"),
		(1081000,"Imp Head (Fanged)"),
		(1082000,"Imp Head (Long-Tongued)"),
		(1083000,"Imp Head (Corpse)"),
		(1084000,"Imp Head (Wolf)"),
		(1085000,"Imp Head (Elder)"),
		(1090000,"Silver Tear Mask"),
		(1100000,"Chain Coif"),
		(1100100,"Chain Armor"),
		(1100200,"Gauntlets"),
		(1100300,"Chain Leggings"),
		(1101000,"Greathelm"),
		(1101100,"Eye Surcoat"),
		(1102100,"Tree Surcoat"),
		(1110000,"Octopus Head"),
		(1120000,"Jar"),
		(1130000,"Mushroom Head"),
		(1130100,"Mushroom Body"),
		(1130200,"Mushroom Arms"),
		(1130300,"Mushroom Legs"),
		(1300000,"Nox Mirrorhelm"),
		(1301000,"Iji's Mirrorhelm"),
		(1400000,"Black Hood"),
		(1400100,"Leather Armor"),
		(1400200,"Leather Gloves"),
		(1400300,"Leather Boots"),
		(1401000,"Bandit Mask"),
		(1500000,"Knight Helm"),
		(1500100,"Knight Armor"),
		(1500200,"Knight Gauntlets"),
		(1500300,"Knight Greaves"),
		(1600000,"Greathood"),
		(1610000,"[ERROR]"),
		(1700000,"Godrick Soldier Helm"),
		(1700100,"Tree-and-Beast Surcoat"),
		(1700200,"Godrick Soldier Gauntlets"),
		(1700300,"Godrick Soldier Greaves"),
		(1710000,"Raya Lucarian Helm"),
		(1710100,"Cuckoo Surcoat"),
		(1710200,"Raya Lucarian Gauntlets"),
		(1710300,"Raya Lucarian Greaves"),
		(1720000,"Leyndell Soldier Helm"),
		(1720100,"Erdtree Surcoat"),
		(1720200,"Leyndell Soldier Gauntlets"),
		(1720300,"Leyndell Soldier Greaves"),
		(1730000,"Radahn Soldier Helm"),
		(1730100,"Redmane Surcoat"),
		(1730200,"Radahn Soldier Gauntlets"),
		(1730300,"Radahn Soldier Greaves"),
		(1740100,"Mausoleum Surcoat"),
		(1740200,"Mausoleum Gauntlets"),
		(1740300,"Mausoleum Greaves"),
		(1750000,"Haligtree Helm"),
		(1750100,"Haligtree Crest Surcoat"),
		(1750200,"Haligtree Gauntlets"),
		(1750300,"Haligtree Greaves"),
		(1760000,"Gelmir Knight Helm"),
		(1760100,"Gelmir Knight Armor"),
		(1760200,"Gelmir Knight Gauntlets"),
		(1760300,"Gelmir Knight Greaves"),
		(1761100,"Gelmir Knight Armor (Altered)"),
		(1770000,"Godrick Knight Helm"),
		(1770100,"Godrick Knight Armor"),
		(1770200,"Godrick Knight Gauntlets"),
		(1770300,"Godrick Knight Greaves"),
		(1771100,"Godrick Knight Armor (Altered)"),
		(1780000,"Cuckoo Knight Helm"),
		(1780100,"Cuckoo Knight Armor"),
		(1780200,"Cuckoo Knight Gauntlets"),
		(1780300,"Cuckoo Knight Greaves"),
		(1781100,"Cuckoo Knight Armor (Altered)"),
		(1790000,"Leyndell Knight Helm"),
		(1790100,"Leyndell Knight Armor"),
		(1790200,"Leyndell Knight Gauntlets"),
		(1790300,"Leyndell Knight Greaves"),
		(1791100,"Leyndell Knight Armor (Altered)"),
		(1800000,"Redmane Knight Helm"),
		(1800100,"Redmane Knight Armor"),
		(1800200,"Redmane Knight Gauntlets"),
		(1800300,"Redmane Knight Greaves"),
		(1801100,"Redmane Knight Armor (Altered)"),
		(1810100,"Mausoleum Knight Armor"),
		(1810200,"Mausoleum Knight Gauntlets"),
		(1810300,"Mausoleum Knight Greaves"),
		(1811100,"Mausoleum Knight Armor (Altered)"),
		(1820000,"Haligtree Knight Helm"),
		(1820100,"Haligtree Knight Armor"),
		(1820200,"Haligtree Knight Gauntlets"),
		(1820300,"Haligtree Knight Greaves"),
		(1821100,"Haligtree Knight Armor (Altered)"),
		(1830000,"Foot Soldier Cap"),
		(1830100,"Chain-Draped Tabard"),
		(1830200,"Foot Soldier Gauntlets"),
		(1830300,"Foot Soldier Greaves"),
		(1840000,"Foot Soldier Helmet"),
		(1840100,"Foot Soldier Tabard"),
		(1850000,"Gilded Foot Soldier Cap"),
		(1850100,"Leather-Draped Tabard"),
		(1860000,"Foot Soldier Helm"),
		(1860100,"Scarlet Tabard"),
		(1870100,"Bloodsoaked Tabard"),
		(1880000,"Sacred Crown Helm"),
		(1880100,"Ivory-Draped Tabard"),
		(1890000,"Omensmirk Mask"),
		(1890100,"Omenkiller Robe"),
		(1890200,"Omenkiller Long Gloves"),
		(1890300,"Omenkiller Boots"),
		(1900000,"Ash-of-War Scarab"),
		(1901000,"Incantation Scarab"),
		(1902000,"Glintstone Scarab"),
		(1910000,"Crimson Tear Scarab"),
		(1920000,"Cerulean Tear Scarab"),
		(1930100,"Deathbed Dress"),
		(1930300,"[ERROR]Deathbed Smalls"),
		(1932100,"[ERROR]"),
		(1940000,"Fia's Hood"),
		(1940100,"Fia's Robe"),
		(1941100,"Fia's Robe (Altered)"),
		(1950000,""),
		(1950100,"[ERROR]Millicent's Robe"),
		(1950200,"[ERROR]Millicent's Gloves"),
		(1950300,"[ERROR]Millicent's Boots"),
		(1960000,""),
		(1960100,"[ERROR]"),
		(1960200,"[ERROR]"),
		(1970100,"[ERROR]Millicent's Tunic"),
		(1970200,"[ERROR]Golden Prosthetic"),
		(1971100,"[ERROR]"),
		(1971200,"[ERROR]"),
		(1980000,"Highwayman Hood"),
		(1980100,"Highwayman Cloth Armor"),
		(1980200,"Highwayman Gauntlets"),
		(1990000,"High Page Hood"),
		(1990100,"High Page Clothes"),
		(1991100,"High Page Clothes (Altered)"),
		(2000000,"Rotten Duelist Helm"),
		(2000100,"Rotten Gravekeeper Cloak"),
		(2000300,"Rotten Duelist Greaves"),
		(2001100,"Rotten Gravekeeper Cloak (Altered)"),
		(2010000,"Mushroom Crown"),
		(2020000,"Black Dumpling"),
		(2030000,"Lazuli Robe"),
])});
