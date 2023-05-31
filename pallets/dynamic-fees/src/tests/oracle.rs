use crate::tests::mock::{AssetId, AssetVolume, Balance, CustomOracle};

const DATA: [(Balance, Balance, Balance); 259] = [
	(0, 0, 20000000000000000000),
	(930804711430423, 2532358650832923, 20008843361986886570),
	(530422613966861, 679413296565044, 20003089124313432372),
	(162941797162572, 173018292914100, 20000845973228169967),
	(761567491170346, 2071929805226937, 20007241808047484015),
	(504548340114819, 646271184537481, 20002940133630833795),
	(160917675458689, 170868997349950, 20000835896732418036),
	(623100674593919, 1695215295185675, 20005931445733427379),
	(479936225962877, 614745760901506, 20002798410786410759),
	(158918698123799, 168746401109578, 20000825945410526377),
	(509809642849570, 1386994332424643, 20004859331112835585),
	(456524702745175, 584758162808750, 20002663601251471773),
	(156944552805491, 166650172524366, 20000816117707540205),
	(417116980513284, 1134813544711072, 20003982146423260482),
	(434255205050289, 556233374379055, 20002535367791407860),
	(154994931031509, 164579984045802, 20000806412087820942),
	(341277529510869, 928483809309058, 20003264449859062670),
	(413072024316128, 529100039043491, 20002413389622078772),
	(153069528161553, 162535512194302, 20000796827034806266),
	(1625339677297557, 759668571252866, 20002677243579264461),
	(754074332512300, 503290281041370, 20002297361607351103),
	(243138469952560, 160516437508658, 20000787361050773139),
	(1329823372334364, 3168535961693148, 20003542914685309135),
	(717290218731212, 1162078034096640, 20002548145658821742),
	(240118116288553, 332540322988978, 20000869983083216667),
	(1088037304637207, 2592438514112575, 20001704202095950338),
	(682300451963836, 1105391300726072, 20002103357843456037),
	(237135282545838, 328409387299674, 20000777560876515873),
	(1484403226564727, 2121086057001197, 20000199800886474960),
	(808434570172359, 1051469773861386, 20001680266994693537),
	(274786395498543, 324329767581666, 20000686286771761673),
	(1214511730825686, 1735434046637343, 19999563118056038480),
	(768998737481025, 1000178565380342, 20001437231791004260),
	(271372899902288, 320300826369471, 20000636743399678191),
	(3483197783366014, 1419900583612371, 19999042195740226815),
	(1399402946374511, 951389367069106, 20001206051963104704),
	(438092305118377, 316321934116434, 20000587815473210652),
	(5838262164596950, 1161736841137395, 20001105492939980452),
	(2132898016314104, 904980129651101, 20001654065542409882),
	(636824722199343, 312392469096354, 20000709585844212244),
	(4776759952852050, 950511960930595, 20005782018263440002),
	(2028854210640245, 860834757472998, 20002881983429072669),
	(628913856085066, 308511817306337, 20001034018097314887),
	(3908258143242586, 3960732174269645, 20009608266255361453),
	(1929885712560233, 1672829312439943, 20004050002882239710),
	(621101261599537, 522154193914897, 20001354420136093273),
	(5581079081907134, 3240599051675164, 20009555792224334390),
	(2475197302479129, 1591227882564824, 20004307059282359804),
	(776227622398271, 515667806412848, 20001453367203777575),
	(4566337430651291, 2651399224097861, 20011896272254566357),
	(2354455970650878, 1513607010244588, 20005191028702273923),
	(766585043238044, 509261995153061, 20001713927019762664),
	(3736094261441965, 2169326637898250, 20013811210461119785),
	(2239604459887421, 1439772521939974, 20006031877662680037),
	(757062247669869, 502935759188427, 20001971250067847317),
	(3056804395725244, 1774903612825840, 20015377978084663498),
	(2130355461844132, 1369539715991683, 20006831709600627315),
	(747657747698815, 496688110005962, 20002225376556328434),
	(2501021778320654, 1452193865039324, 20016659878867562900),
	(2026435683217589, 1302732900577454, 20007592525346479604),
	(738370073814358, 490518071372348, 20002476346194020965),
	(2046290545898716, 1188158616850356, 20017708706780844228),
	(1927585162085023, 1239184954207822, 20008316228129119587),
	(729197774760763, 484424679181387, 20002724198196462657),
	(1674237719371677, 972129777423018, 20018566838709892588),
	(1833556617593071, 1178736907661099, 20009004628336996643),
	(720139417310319, 478406981303357, 20002968971292041720),
	(1369830861304099, 795378908800651, 20019268946651841246),
	(1744114831369018, 1121237546311777, 20009659448046928477),
	(711193586039383, 472464037436235, 20003210703728048372),
	(1120770704703354, 650764561745987, 20019843398604344693),
	(1659036059107115, 1066543031857544, 20010282325331985587),
	(702358883107217, 466594918958766, 20003449433276651215),
	(916994212939107, 532443732337626, 20020313404747302059),
	(1578107470857987, 1014516542498639, 20010874818359235034),
	(693633928037562, 460798708785365, 20003685197240799364),
	(750267992404724, 435635781003512, 20020697955227903540),
	(1501126618621012, 965027930669437, 20011438409287594263),
	(685017357502934, 455074501222814, 20003918032460051263),
	(613855630149320, 356429275366510, 20021012587439304752),
	(1427900929907792, 917953397466050, 20011974507975545726),
	(676507825111593, 449421401828742, 20004147975316331088),
	(502245515576716, 291623952572599, 20021270013794087562),
	(1358247226009851, 873175182955511, 20012484455507987360),
	(668104001197163, 443838527271863, 20004375061739613649),
	(410928149108222, 4274963740338832, 20021480635357091679),
	(1291991263765468, 1913507748922895, 20012969527551041598),
	(659804572610863, 714101313085714, 20004599327213538662),
	(336213940179454, 3509464844981191, 20017616599765861069),
	(1228967299679347, 1823322971700891, 20012348011065884074),
	(651608242516318, 706034462188647, 20004545030473063527),
	(275084132874099, 2871380327711883, 20014443348861059332),
	(1169017675304745, 1734380387715481, 20011753655393862439),
	(643513730186923, 697263847751521, 20004490604253390918),
	(225068835987899, 2349311177218813, 20011847052666221548),
	(1111992422850855, 1649776466363506, 20011188292681451615),
	(635519770805719, 688602185046533, 20004436854135826044),
	(184147229444644, 1922163690451756, 20009722810324990634),
	(1057748890028862, 1569299565565286, 20010650508637938880),
	(627625115267760, 680048120636017, 20004383771721584957),
	(150665915000164, 4343161592054362, 20007984793863983522),
	(1006151383198186, 2236048472087277, 20010138957962402376),
	(619828529984930, 860887922233906, 20004331348716216429),
	(123272112272861, 3553495848044478, 20003792298186929324),
	(957070827920225, 2126972936863507, 20008909060873513210),
	(612128796693192, 850193662330379, 20004090289323967187),
	(100859000950523, 2907405693854573, 20000362074451157707),
	(910384446070458, 2023218159455531, 20007739158764569856),
	(604524712262221, 839632250375965, 20003852224458329737),
	(82521000777700, 2378786476790105, 19997555527758253657),
	(865975448701167, 1924524590701603, 20006626325051184715),
	(597015088507410, 829202036085580, 20003617116920215733),
	(278154151524255, 1946279844646449, 19995259262282241252),
	(880245101441781, 1830645342374695, 20005567775909184215),
	(603990097717054, 818901389674579, 20003384929972637307),
	(227580669428936, 1592410781983458, 19993591136589119058),
	(837306316005597, 1741345569575929, 20004617375668251239),
	(596487115136718, 808728701604087, 20003170018680679529),
	(186202365896402, 1302881548895556, 19992226306476564536),
	(796462105468738, 1656401883255152, 20003713336414680848),
	(589077337308933, 798682382329502, 20002957777094211910),
	(619225443005284, 1065993994550910, 19991109627293565381),
	(882870260811495, 1575601791389047, 20002853396636894378),
	(613658106907521, 788760862052117, 20002748172049191093),
	(506638998822505, 872176904632562, 19990662858742019756),
	(839803418820690, 1498743167418850, 20002160665106316773),
	(606035024834135, 778962590473830, 20002573069294046253),
	(414522817218413, 713599285608460, 19990297320836209698),
	(798837398390413, 1425633744617930, 20001501725357718563),
	(598506639432469, 769286036554900, 20002400141728406317),
	(339155032269611, 3317225213367411, 19989998244367819652),
	(759869720420149, 2089434141869870, 20000874929011490998),
	(591071774346352, 946481761421084, 20002229362331283648),
	(277490480947863, 4155143329502651, 19987020174186721851),
	(722802904789898, 2374133688710912, 19999545364590041231),
	(583729267832732, 1033181054473135, 20001873952344208680),
	(227037666230070, 3399662724138533, 19983142521338167063),
	(687544226507463, 2258322289261599, 19997894033806120173),
	(576477972580152, 1020346507212600, 20001424500557568045),
	(185758090551875, 2781542228840617, 19979969896280258600),
	(654005483751002, 2148160226370789, 19996323255743365996),
	(569316755529467, 1007671395321760, 20000980632022935367),
	(151983892269716, 2275807278142323, 19977374112141969858),
	(622102777226563, 2043371922645385, 19994829101000746170),
	(562244497696802, 995153738237017, 20000542277383142848),
	(2264132691531044, 1862024136661900, 19975250288756097251),
	(1165844216786585, 1943695243491951, 19993407831855327310),
	(701456395715563, 982791579998048, 20000109368142602409),
	(1852472202161763, 1523474293632464, 19975652397310966394),
	(1108973767187239, 1848880841370392, 19992629980828621909),
	(692742651669407, 970582988942172, 19999828032958319702),
	(1760751176715288, 1246478967517470, 19975981395219495694),
	(1120633903529625, 1758691532035251, 19991890073754438722),
	(700882576023900, 958526057402518, 19999550192621046719),
	(1440614599130690, 1019846427968839, 19976495667428693512),
	(1065968835064765, 1672901701204263, 19991252016125933063),
	(692175960172672, 946618901409940, 19999292549139667886),
	(1178684672016019, 3252493915947149, 19976916435599855362),
	(1013970355305508, 2240048330991838, 19990645083259793535),
	(683577501040092, 1100069692843972, 19999038106198430406),
	(964378368013106, 3139233966362525, 19974842626355924232),
	(964508386754020, 2259049104759637, 19989419005284107176),
	(675085855064439, 1119069624525807, 19998621614006626315),
	(789036846556178, 2568464154296611, 19972667770757574813),
	(917459197156263, 2148851587454288, 19988124464566101532),
	(666699695374197, 1105168138506853, 19998177630237164739),
	(645575601727782, 2101470671697227, 19970888343449834380),
	(872705089977909, 2044029558797982, 19986893072175803480),
	(658417711580729, 1091439341755215, 19997739161794031878),
	(528198219595457, 1719385095025004, 19969432448379864934),
	(830134109978986, 1944320799832226, 19985721747706983382),
	(650238609573515, 1077881089062604, 19997306140163857190),
	(432162179669011, 1406769623202276, 19968241261504435388),
	(789639763150743, 1849475882767240, 19984607561017130118),
	(642161111317943, 1064491261869279, 19996878497684367901),
	(353587237911009, 1150993328074589, 19967266654060902123),
	(751120750314121, 1759257547022496, 19983547724897513598),
	(634183954655608, 1051267767933015, 19996456167533816367),
	(289298649199916, 941721813879209, 19966469247970738543),
	(714480713713432, 1673440105704325, 19982539588100805202),
	(626305893107092, 1038208541002170, 19996039083720538765),
	(236698894799931, 2390135707781097, 19965816824806059250),
	(679627995971314, 2026345380076778, 19981580628708814288),
	(618525695677190, 1135969903602635, 19995627181072643494),
	(9669496909247867, 1955565579093625, 19963663387993078084),
	(3188772385156302, 1927499263975471, 19980233911324708805),
	(1258259388578884, 1121858476228689, 19995109736864717859),
	(7911406562111890, 3206202052215390, 19971377319323232325),
	(3033222512709653, 2264404967257806, 19981495184445889617),
	(1242628837167966, 1217662298651480, 19995246137777067866),
	(6472969005364273, 2623256224539864, 19976082523833128826),
	(2885260438918938, 2153946188367181, 19982264001991341446),
	(1227192454097557, 1202536058916679, 19995271104315584166),
	(5296065549843496, 2146300547350798, 19979932236613953235),
	(2744516027264356, 2048875642593172, 19982995316241893187),
	(1211947827338581, 1187597722781068, 19995295760710764861),
	(4333144540781042, 1756064084196107, 19983082001616445933),
	(2610637196666094, 1948930489295944, 19983690956626564354),
	(1196892574825058, 1172844956038446, 19995320110815322193),
	(6689483329839817, 1436779705251360, 19985659082073030867),
	(3326850400760641, 1853860709330288, 19984352663333934489),
	(1396844317753993, 1158275453478962, 19995344158434108626),
	(6851297702512272, 1175547031569294, 19990911785697619325),
	(3534294887530284, 1763428479606859, 19985825653025364827),
	(1473647026596034, 1143886938528913, 19995582727298383481),
	(5605607211146404, 961811207647604, 19996587536368562302),
	(3361890258870270, 1677407578162622, 19987596519433288238),
	(1455340852352605, 1129677162895013, 19995912487386450428),
	(4586405900028876, 786936442620767, 20001231332372061102),
	(3197895612096110, 1595582818252250, 19989281002113995873),
	(1437262084000399, 1115643906213087, 19996238151075907847),
	(3752513918205443, 643857089416991, 20005030801829469211),
	(3041900704188983, 1517749510044823, 19990883314907839720),
	(1419407896621512, 1101784975701124, 19996559769253694988),
	(6592818169328667, 526792164068447, 20008139458658257663),
	(3838597611271626, 1443712948579222, 19992407466101983868),
	(1642448634544017, 1088098205816638, 19996877392174615209),
	(5394123956723454, 431011770601457, 20014205484663517883),
	(3651348947307156, 1373287926697309, 19994802350764676260),
	(1622045545916141, 1074581457918294, 19997431742603342422),
	(4413374146410099, 352645994128464, 20019168596849639881),
	(3473234364511685, 1306298271736464, 19997080411785286097),
	(1601895911805381, 1061232619931732, 19997979206691340105),
	(3610942483426444, 288528540650562, 20023229325001921515),
	(3303808297950139, 1242576404822490, 19999247347878061308),
	(1581996583708420, 1048049606019536, 19998519869983213593),
	(3161242321816852, 236068805986823, 20026551738944697397),
	(3198139190468361, 1181962921660418, 20001308579771188947),
	(1576476024837181, 1035030356255318, 19999053816960902317),
	(2586470990577424, 193147204898310, 20029476912460527426),
	(3042132400689417, 1124306193774544, 20003324756039996882),
	(1556892471733613, 1022172836301836, 19999595262629484022),
	(2116203537745165, 158029531280435, 20031870236246206541),
	(2893735698216762, 1069461989200175, 20005242582246911746),
	(1537552192581643, 1009475037093117, 20000129982264915643),
	(1731439258155135, 565966937400720, 20033828410252671270),
	(2752577859279359, 1134448490455874, 20007066855955928324),
	(1518452165344604, 1026769574085029, 20000658059420404015),
	(3521792329425876, 463063857873316, 20034993882573425686),
	(3183104849077644, 1079109539701929, 20008684985324751801),
	(1643420227267580, 1014014672543600, 20001149742011663438),
	(2881466451348444, 9686246332660139, 20038052611044978246),
	(3027831441805564, 3523570902116510, 20010788980634127508),
	(1623005069164877, 1637325887408907, 20001779147566387268),
	(2357563460194181, 7925110635812840, 20031247831163666550),
	(2880132347083341, 3351689394696192, 20010293241173816555),
	(1602843515510655, 1616986435391405, 20001764826748143090),
	(4364282739321890, 6484181429301414, 20025680283988047891),
	(3393029281147402, 3188192351052476, 20009821684126203697),
	(1749323962465760, 1596899647374121, 20001750683828262194),
	(4536647741377378, 5305239351246611, 20023560385298068367),
	(3486651767463483, 3032670772952355, 20010026521056298616),
	(1793584413250373, 1577062384673821, 20001903108143353689),
	(3711802697490582, 6000904863368966, 20022791793688199134),
	(3316571193440874, 3330169743438560, 20010480502050809738),
	(1771303861532977, 1670905083844576, 20002119630171930097),
	(3036929479765021, 4909831251847336, 20020502691522320750),
	(3154787232785221, 3167722438880581, 20010466903500812046),
	(1749300086855548, 1650148498952096, 20002220028949618357),
];

pub struct Oracle;

impl CustomOracle for Oracle {
	fn volume(&self, _asset_id: AssetId, block: usize) -> AssetVolume {
		DATA[block].into()
	}

	fn liquidity(&self, _asset_id: AssetId, block: usize) -> Balance {
		DATA[block].2
	}
}

impl Oracle {
	pub fn new() -> Oracle {
		Self {}
	}
}

pub struct SingleValueOracle {
	data: (Balance, Balance, Balance),
}
impl SingleValueOracle {
	pub fn new(volume_in: Balance, volume_out: Balance, liquidity: Balance) -> Self {
		Self {
			data: (volume_in, volume_out, liquidity),
		}
	}
}

impl CustomOracle for SingleValueOracle {
	fn volume(&self, _asset_id: AssetId, _block: usize) -> AssetVolume {
		self.data.into()
	}

	fn liquidity(&self, _asset_id: AssetId, _block: usize) -> Balance {
		self.data.2
	}
}
