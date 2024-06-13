# Avail Prover Demo

## Environment variables
Add the following details to your `config.json` file. Use the aleo sdk to obtain your account private key. The secret is used to encrypt your private key.

```
{
    "private_key": "APrivateKey...",
    "query_url": "https://api.explorer.aleo.org/v1",
    "secret": "***"
}
```

## Instructions
To start the prover, use `cargo run --release`

## Api calls for the prover

### 1. Test prover 

```cmd
curl --location --request GET 'http://localhost:3030/api/test'
```

#### Expected output:

```
{
    "message": "The Avail prover is running!!",
    "data": null
}
```

### 2. Benchmark prover

```cmd
curl --location --request GET 'http://localhost:3030/api/benchmark'
```



#### Expected output:

```
{
    "message": "Proof generated, the proof generation time returned is in milliseconds",
    "data": "1861"
}
```

### 3. Generate proof for private market 

```cmd
curl --location --request POST 'http://localhost:3030/api/generateProof' \
--header 'Content-Type: application/json' \
--data-raw '{"ask": {
    "market_id": "1",
    "reward": "10",
    "expiry": "100",
    "time_taken_for_proof_generation": "1000000",
    "deadline": "100",
    "refund_address": "0x0469866e13cd7DF08f5482FBb127a72fF197365D",
    "prover_data": "0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000043275363400000000000000000000000000000000000000000000000000000000"
},
"private_input": [123, 34, 112, 114, 105, 118, 97, 116, 101, 34, 58, 34, 116, 114, 117, 101, 34, 44, 34, 97, 100, 100, 114, 101, 115, 115, 34, 58, 34, 97, 108, 101, 111, 49, 114, 110, 54, 51, 54, 103, 57, 52, 109, 120, 51, 113, 113, 104, 102, 55, 109, 55, 57, 110, 115, 110, 101, 51, 108, 108, 118, 52, 100, 113, 115, 50, 53, 55, 48, 55, 121, 104, 119, 99, 114, 107, 57, 50, 112, 48, 107, 119, 114, 99, 57, 113, 101, 51, 57, 50, 119, 103, 34, 44, 34, 97, 109, 111, 117, 110, 116, 34, 58, 34, 51, 117, 54, 52, 34, 125],
"ask_id": 10
}'
```

#### Expected output:

```
{
    "message":"Proof generated",
    "data":"0x00000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000168000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000043275363400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000154870726f6f6631717972717171717171717171717171707171717171717171717171717a71717171717171717171717179717171717171717171717171677171717171717171717171717371717171717171717171717071717171717171717171713976787236337779396c683474336b3835723276776d736b77647575617232616a363835726563707832753466356e65396e656d773533366a3964363634766e79356a38356a367979323067716d67736336386d6e656a376e677175776877646163763676766b376d656b7076756773386c77377766306e6c367a32716c67307a763777347563747763726d776464386c35766864636c6573707a7336386b6d386833777637383571766a706a77787863737676706e32386a667a367970726a657a6b65677a36637a34797278726865736b33636a323870646a737463667371646876397a7172796b7a73717365727674646b6b3378776776613476336370796d3470377235377961306c72797a383872347167336135667768737a30366571346772356b7963306b726477666876676376717236646b706c663937796179647036346a7135653037636138616168707132753663386177757a6e63306438767a646830366d787139787171713532656c6876356b306b687832387832777a71716c3561776377357a787578686567657866393830687576336d753339726c7571303673646c6a3277736e357776686d6c70796d7275766a7439767165336466686b3664346a3479303866677071796d77666e30397039717532637663793573747278726b74303277326578673668636b73687861376c74776779366e7635617a6172786e3930396765343274703861323273616c796d3478357139386c376c7a327679326a30383375677167706a7a6c68756668706767303367347964397374327275686d686e367364367a3464326873777263797565666a377a7936753873397975756b7a71303863616a676e70676a3761613279336137616d7a616a327a6c707a67366530683971673375706c72796d75616a686b393461326c6d646e7a63757a6b75356a77637734303638396178357370727439343678766a337a616368387061787532767a717578773968647a666a67713263677a6a61776b6479617a72786a633071757232636d686432343239746c6674386167756a78636a7a6370656b76353833346737366b357a3374336a33783761667570703965356768767266653978706c67757673763064336d3266663061646a6838753675636a7335726e39636161347a7968677073717237356a396d77366430713737397830786464337161796b643973336b6373367863716b77357334756d37703663746e63326d777374646d793070766b7234323335716833353375707a7971797778656d3037617a7a78666c34706e7a6e727977396e746b77356433356c68766a6e7a6576756c6d35613974786a78736b357a6373363572723273336737716d68776435647272736766737170746e677972736b65397271757a6e66666b3377383938336d686367637167763875737134747378746667766d33716535716d387879306b35656b39306a366833386d7a7338343477653671716533613636373030396a393934363866777477637a636b666d67796d6d68736b686a32326130796b777664326d357665326c636b6d6537686e74366e35766c6830676c6c3261676b6d70687172377a646e6c36746b3664677432706833347865657965757834747033367a6e7374673467336b7a61706d71353335666537676468307a7464336c7067737a307373736572396464787264727138356a796b32376c376177367635383974766434376b30393671776872796a6e337a6467373977686873686561616b37666c3039736e6166746d36797438366c6b706464307a74747374736d713061667064307379686d377165733271786638657370706779653774746d3434737263393730723939377072667a38673268646b6c74613676656771377579306376647273703938763465637171346b786d613468396e39327734306b326b3565756d643266687078386d6c3478616e3375346b736670683070346c656c616d336e7a6d6b7970397666307878333465657061773477363471707163767333636a74736a646c65776d6e6a3270753264617075776d6538307373366864347a356a797232377a6a6136706d35686b7532633565616a383337343761366d736c3068356433747370387964727938366d6778636e363375653467706778786865706666326674637a703267727432376d7968336e33676b37753968793733763271743636663873726e6663633877633775397a713930787a37766a6a63386c676630376a6c6d6d74773570393333663877646176786c756b337573393939756477777979727067396c66356b6875616c6e337430613372396475366630657a7771796a6d7866706776306c6c35653063667a6d7532323335336b3236326e3879777179373866396e7876346a7a6c34746835336a716b6735346a357a736d646b6835636d7a6d6365307a717432716d35686168777a72343573337133356b6c3330376d713672383372743537786d6a7a6b306861336438647a6e77386779763270326174797667346a336177776b6d68377178356465736e6a76717565357a72336e376b74726c6c396635646679723565663333663771676c7238387337683263646139793276306773346c65787672716767703576377a3775723833776b726d763861636771713436653475336467766875656d303868757034646e747535656834726e71666b6563337178756a6c73323430657233797339667571797470647a377a7932636c39663037613838326b677173727934726432666e373263616163736138643967756a613467683366337936377a65373778673338707765677238336564683764786d38346b6574783733797670357068656d6164346a336671796b3070656d6e347137366364616730686a676d6a743037706568387976677a30667039337874757168663670686d7668306c39396d766565646d6c643374676a75776638376767787432717166646c6a61716b7868353968367479657079753978323365336734386d766a6b32636d6c376a6c7a76397136726e6739376472633432793676726571346165617a396161786e633933716873706a646d613634776e6e3065683467726764786338766e3763326e713865736c3761767165726c667332367933673336377a6370757a6773346c7570756839376d67367937707963743633656d32737830686b6c797a387a7437747579307479327279716e737a367279377165386e6d376165717836707874663237663979666d3461647471723536376a6e6d35736d6a387932613066707a7a323837613766616b74643667766868687235377a7a36646a76726b717333336c71326c367a326639783636366b36706a6370703961747230617a336d61666178647936713938733261743064386c74387a7a77783238376161326e397232376e6a77726735633064367535796a7a746c6876707234636d663937377476366d3461776c7233753470377379753767707665757a776133756367727461616776303070376734753532706c7a3432676d6a323876347875723277646b32656633657034673336336e72657771737a74777164716a7a3333376d387a687632336533796335307038396875383567336475396b746e3475356574306a30667933667a38756e61387077337a366a73736d73716e3066767a687573647a7137787368363370757473357a3768713036657a757572657365613472336868356b306b306a7a76677277636d39757a726539646579346e6e336e7264787632306766616775393637633471727a6573357034763035726b36706d3836677033686878656336377179687a74786a6e68333039666a3964386e3530666166677075376e396637346866736a6e343561347a32713470676370336835347a77796c64726e3868746a36357067616b7a776a7a377078743339386c78376b74766d716d6379797532636c7677613479673479373837356c6768753375686d757a32396d36707639717375396b6e6672617568356e396a787733656a7276796d74386668677939726d38677261756e356b303265713667687868737136646a6d666a66356c6534307735717368617038306b7a6c6c7632617478766b766639633033306a79656a686736347870637377386a3637373666787761766a30773867743873616877366c76366d7332776a736e366b346b396670796a66663032776c783572767830743076357564327564306761343630737a30676a747279346c68787361797564723667617a6e686a3064783476307972397339763064727a7661327875786a75796d646b7835766b6b713866646b7637306632633977376d3671673573783330796a7a6639367a356578636561637a7433756c636365676b746e726a726d64617338673674646e766134337a366135636e676a6e7371716e7276716664703030383870616a7a65643674327a636c356b6d713477387771306c66647334687877653273337a766c3467766e6a6d6664766c73336c6c6e766170647a327465726e786676736c3271376e73666d37327078387a6b6461703367737771633863616574303379727a68667771706a7368776b73677439367466723765783932637870657379786d64327a67726779366e677175646b743763376439793361376b786b75367a733338327776767a3032616e326b643479766b7a65376e6d6578656739636c716a767a326a36356e376d79796e35377273666e6130643533776b767673717173633335797232786b77706e67636a3032326471736671766a666b36683478767173773773676873326765646163676a63776a67326477383779666373666e327a61636c306e7133396b6a3833706c3934666c667a376d39657068376c676d72726c63776c64736b686673716c646537663832367079763837757865366a687576373870336575637966346639686a6e783274376a766d3937357276783430736139687674347772646c6d78757974386d653739743930677a6b6a3373737871777466737a37387a6a63306c6533656e3534747934796e686b6566346e3961777076663675347261387a797a7876306b6e61656836343437766867676c6e66733668736b67366172353961323576323839716b7376656136726d726665766d356e7a3575326366777777656a796d796e3736766a39707134366c74357566717434393577727773783973777a7179356370686437757a793863687233686b3771647166737366726879736573377a6e746735337a74767370377671347478397a3037656a747737613463353576707479346c343661663772646a746366713475787861646a6d6638756861306371707668646167686b7a766b307a30337178797475796a3864726578397075333777377a367a726d7236617a616a7679683261716373726e71776e79736573333767707861306375336871633764757730643768703768797234726a6e65336c34746b393267713378636d763574303075776d7133763035346c6b646e667730726a6b7265656579773467377867676a79306c68673476397a737a7268636537726865773633726832776a39613535776b6b3268377536757675663675756b6b64787264756c763575787539356677306c6c68387939676c666c6a34667436367a6875376c323879776c736c70677a716c373070306b376e386d3776683774777964796c78306a32363975756e717a6c346b68786d396a7734396a767633656e326b6d796e6730336a356d796b736d3778383071656b7439767137637373343870717134653576643939766d716d3274766a3835776d6a333472326c667a6d3463796434336173746874337370766c6a756e77393232396c65373878373937363039676c737972326c667271746a753474766e6d376b36786867773539736e6171676e6c726334723067706634676536786b6c63727a646d7738737075797267346d796e7275337266796c386366677a6b6c366d79746c6576616a3530796c706b797432307772647032767a76676b64656663747377657a7a38356b65746c37727673766b3677356d6a777a6a376532386468726775776c6a7373667035336e3366723366703535346b6b66337a72356c73307130677664676d38616c7a67357430377336666776327737646b737a7636636371396c73377a76386e726e6d716b367070336c73637979706e723532786c6d6c3871376b6672766c68713673746166686e7361673735736e337074326661746332346e616e7a7178376763676e32713632703767716d6376753670686d6d7337337972677178357a7278377976743276306565686a6439347435396c396d6b7768683077383461706a7476797767776577363666786e3638786768756e7173676e7930343963706379656664757239677a6136733768707132777776793866337573666d7771637436716d66366b65757966346d75346a6b667a6661666e396a71726a7230757a637730616a6577717637726b653970616d3872366e64353738366a38326d666d796b673468326c723676726773306473616d3664716b6d6a357077796633377830326d673478386e70673432636466686368636d6e323964786c737877717a736a336a373334333766367165323661377961356c337536666a38663834366c74616c73706a3878343572323767646566647a633839357a6464366a72327a7377767273717733747233777436723065673039746c6b6d616e34776e7065643467756c79347a7a7277363073703534656c64386b6e39336d67636a7779637735346d32336666656363377a656b79746e6b6d30676d38387434747164327036676b686e7273766b34706436716a77356134717a39703465353861707764746c65357038747536776734767878717165366838653079686b37757767737163746d6c3634686b7679783063376138637a33397179636e396639726a6d327477736c6338717671717171717171717171706d6d3566376e3864306a727a72717164736461786b3732676a78756432636e70776c767970633671797679326c3668703230343979786164746d666738786168336b707a3436757a6e33677379717030656877717676386a68397734706371307a34356d716178306a667933746e79706e63373277356a37766b6d6532636a793870656567396c6d783864746b736d76643963763064336a79767171787a677a75706c6766363570743934387477746c7132636d703272763277337270636b3777657633753875703333656a7a6e73397535677875763370726c756d30716730776738326a706d647070643065337478637532743279746e3239737732686c7775637978776d333674343076647a723338336e7a786c617678663973717171647263636171000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004199aaaafc1e3ab20c9ba5cae3f920f6844c07faa155064272de34915c5a5261e6003508e505c70a27e75faf26dfe80b51254d8e1799fdd7f8bab5a29d434223f31b00000000000000000000000000000000000000000000000000000000000000"
}
```

### 3. Generate proof for public market 

```cmd
curl --location --request POST 'http://localhost:3030/api/generateProof' \
--header 'Content-Type: application/json' \
--data-raw '{"ask": {
    "market_id": "1",
    "reward": "10",
    "expiry": "100",
    "time_taken_for_proof_generation": "100",
    "deadline": "100",
    "refund_address": "0x0469866e13cd7DF08f5482FBb127a72fF197365D",
    "prover_data": "0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000003f616c656f31726e3633366739346d783371716866376d37396e736e65336c6c7634647173323537303779687763726b393270306b7772633971653339327767000000000000000000000000000000000000000000000000000000000000000004337536340000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000043175363400000000000000000000000000000000000000000000000000000000"
},
"private_input": [123, 34, 112, 114, 105, 118, 97, 116, 101, 34, 58, 34, 102, 97, 108, 115, 101, 34, 44, 34, 97, 100, 100, 114, 101, 115, 115, 34, 58, 34, 110, 117, 108, 108, 34, 44, 34, 97, 109, 111, 117, 110, 116, 34, 58, 34, 110, 117, 108, 108, 34, 125],
"ask_id": 10
}'
```

#### Expected output:

```
{
    "message":"Proof generated",
    "data":"0x000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000001e0000000000000000000000000000000000000000000000000000000000000144000000000000000000000000000000000000000000000000000000000000001600000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000003f616c656f31726e3633366739346d783371716866376d37396e736e65336c6c7634647173323537303779687763726b393270306b7772633971653339327767000000000000000000000000000000000000000000000000000000000000000004337536340000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000043175363400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000123b70726f6f663171797a737171717171717171717171707171717171717171717171717a717171717171717171717171797171717171717171717171716771717171717171717171717173717171717171717171717a706536347463716c6163767577736a3268717766616d346a376135393775686666763634797079393277346677636e74637777346a336a666374673772306e68756761366b6b6a6b61746a71706c3736753866746a306e3837663861766e343338396467636a783465646178306c706177333561367574306e796b65366c646e38636330766466786d6a3067376471616c306c75366a3373703033717134307637766c633777326d68377036376c7a676e37376e677a6d7875706a7278647732307a396761766d7a65306e637a3977396d38676e356164746b343033386a6471766164647371653766346b727164773879616e6375796b66756a61666d6861613379377a3778706d6634336678706875376637776d3261376e67767667716e6e667865736c653373767361303536746b347371776b733337676c756535676a33653836776d32706366707a3875797370353674773632646e36376c3432306b61726c657367677a6b796a35647161796b7466656b767a6363716d35633432716770363232656a7173633065637175687177796e6839736170636c36717175323235766c79666e71366667737967366d737736367733777639383077726b7a706e6e6c6b67383368756573647a71717463393239326b306568336e3772386b636b7476773770386a76613674706b7a33616a6332717a63646d65636b743738666d6a6772776468636c30386b67636a6b383867636c3375707a3773396533766b6133777137647a373939636d7768357a6839766c6639707a326574663935737475373371786a30747565383065673371683474717765706372786e787863336a306d39377635617132347173336d3968657461713663723479396e37686a633361723079797078787465337930747774783674757a79716c7465366e6a35756479727161736c3471777961733736347768327071716765616e6865666a74323864356d657172377933747135796b6a7532646d713832326563396e37673476336c6771766463743036637873337a6530653836356d7564683975306c3863797679707639687773357761336b786d38376834386d657733346d7071766b36367672703077346535736c6339666e72676a6c6333383872326c747a333835773070737738647835376768617938747371776e68306b6e6c6d6d353773656d683874767039707076747874716e7878717137613568636b733971797935726b666d7075396c76666866346c6677647a7679346475386e6778646b376a717a66727064746e763072357672357430636b376e376e6d33656d367135796a6c6d6b7a6c79666c37616b336c3561326c6638326776646878373863396c6b333837686635356739716c79797671786a646876766a34667333387a786864776875676639667778726e3033336a66763361616334776d636b653066746676326e6661797178717664306c65737065667072373877793475687679717930396e74736e74683870657070376a67783033616c61756d7a663373736a7235376d6c666871686c366d6a7a7a6e6a7533646176363479333332356333676c3464793371717676663563797077346b387363337761617034327072396e70736a666d386e333561756e74356577396b6b6d30366c6436346c3364307a7168753832653570657470676b636c64387773787173387673373873703463666c6779743677667863366e6e7a7034776b6174776c7366747a6c35616e3367796377357679776b3034386d6830786c777a366b6d676a35777371396c3734736d33727661396770667372776473367730647237356772766d386c786c783370716e6c35306c7272337a6a327773796763666e6a6d6d6c716e646e786e70727571327a36703565743372376b6e636476747168663974717a616a61796b303577657438356363736774386b74356a68367a7234613273396e6877303732356a78367037687a68336e347035666d387838337838796b706c3476726d6d6474616b747036716c73396876636e74687a706a396871376635677a65357a7a78716b61307172756578337433377139327136387564367265716832716a39656b386e717338377165657132686c64397163773570377874797a71617075786d3764367632726a6a663432326d7a7a65713478786c7376367972326c646574383436616a786d72636e78307467307070686a6e61677275706366753039656567737161336b3474636a64683673656a75327a67307839747271363336397a39397732337535673973647a7a346c70616a37756e306a787932326c7a65656b7a6d3530337733703561766c73757271786b34376e773339376b653939767564333964733933796a70303538326a77386e637636736c667134736c6d307039716d30676a736d3332766e77646c6b6867796c786b64336e366a3539307172716d6b75786535796a676e72673674636e746b717361366474686567793834666e777938396c6132777477666e33773861336d6d6d7565756e65393679673933387a6a756172343770773771376a6530767a70786d783337396765746d6676736b3471716e6a3666766b32306b357a703671356c7278706372677874726663307433636e6e7678676e6163386a376668386377366d6467357132326d3367646b3774777763786c73746539336b7a74343665716b6c6a6b68376d706d37776e6e7874357430386b396b7235706b653532676473396a307676396d67716c73757377737673366730776d6c64763238637930787578733070706a3870686771726e65657177716678757933736d39376873333533327a677a6a38687361637034786d643364366b6b6863657564766370776b717a3038396632393876676e6637767a6a72303574796d6c663838336c7472797634677771643074703733666436383071786a707a6835677635376e7738667476797938666d3861323232636a61673878766a6a7a6466326168676c63797a68387a676c746b6779716e3371343871357670356377386a346e777970737372376e63727268346c6538346736636168613037647564616c70687967673673337a3973676d667430796c657066387776636e6d306b7a6e776a3538796b303672707a7a6c766b326b32653873707672767473736b637034756e65706b79637178666a636a616d3079356c757664737366703367366e706b786b373334716b757a75707661343279347573367875706a716b6e3379753861766777677478687a6670386b7036727476306d796d7674386665796b6b707173617638777671356d763034377a77657464327432663765756138387972726763747434356c333473336d773639767767733871736766676765666e6663667a7a63747070386168356d6372376d6561336b713676366c6e61736d35657773676672663675727268756a7836677578306a656474756c3735653464376d766d32727377676167337a3335663076376b35367a396b3035657a7a78653566356673666833663333666a337478777139323677797978686e3535716c7568726666716c76767871377966796e34713879766a71767a6c666c7378773261327964356a367163676a7078336567303732736e343071797268686d6c3678366d643371646635396d78773430743639363065616e7036706878736d617633746a32306b3661386c666172676c386565706d326d7a687370336e617332613265646d30746d6d6a773876617075706d37303978636d633036616136767a6572776e66763070336e7375767034676b3476637a34717176727630367773756e32716e30726c77646b6c71343367676b713935336b7970787a75346438356b7232366a666b637930667630356461713678737776786d61323434397a7261783271686a646638336e6c686174327177686a63703867336a773334767868746e636a7a6e66397636396574336c6c7173636b71386d706461716832756b6a6a6475797830737971356a653437687874676d637465337765656765667672376768663776703264306b32366a6e326d66617a706c777865677538736a30673363346439636e686b32396c38333275726676383936773778306a327a77756732747074713271306573346171617a35797a723332686b6b7a377a65397a67376e64716a366c72686b786138787861367064686c6a6379617564377061746577646e7a716173633239666b706576796d387264617171776364736371726b66747939786164726b63393535646c78667a6d686676396e717179366a7376346e717463366c7768716538776d72786b783274663868667575363366663277377939657a657978306166617338306d737065337773367a3564756a6c7277666164613063796a35683937796e33706e307438776d707a336c376632336677717667373474663463326c71353439726d647a777961743978346d336b766c746e6c66686530746871376c6e726a397371686e797164666d76766c30703672646567346465683232327568326e793633346366706838646b71726b336479767234746477637a797a7361397266333832646a71337a34683365676b687763743678673361676c7a3370636c303467757a7179306366386e753239706b30666c357a35616d6677717465773667767572666664346168303034326c6c7468646d67686c617536793465366c6b336c33707a6177347a7078666b3563307a6d676c65766e30647663766e3330387032757178707334776c3875766d666377766d303667646a7a353572767368377136327632643872706a6e34327239757632763337746d77666c756471737561646a786e7863346b76383773326d637a7865797777756e786377736c68766b326136617035616a3932656b7a653372396d687378777163646577766b70397275686c676474376a746571733879727372797234766b336c757463366a686d6568366e70677a6e74307936336e3978367078633330793561613638396d7a68336d657675796d3832716632706d63636a3332666b71376a30357535387a61376374706e33793839377071687374363335676b7768613730746b3264667a326c766b65616e643234726b7376716b33327363676d61786a676679307164743937663366786b66746b79646e376473776d6d3037683938333570676e6e30766534767a68717575366d6b73357a6b3566737037377072377479357a66646c667a686a776b793439386c666a7135776e75386e34756a37373630717670666a7a713376307a36377730306e767a337a6a307271616e6b7764776139347637357578767132376b7473636c6b616a7a66666c7a72713275616a796475666735396e373967357371706b643472326e683861356a3535796136386e3970376c356b32396a30636771733434347333347a75726e666e61676e356179767065367a6d656c6e6d70726a717a6d65347a753371346a73306c766a786d75737a34646b74786a326376713233667930717337707a63656a336c786c6e676c646d7935376e34716d6d736c677668327064757978633039763038617667717438336b707771347333343674366b6573376a66733079396b3837687366356a757a33717361617379706b7776706134307178727364777a6c30737078643777797471707663706d6767367063757676667964726e756379396b777a396c7368706b74666a326d6577707737386c36386a686d73366163616336673436757967646572366b34617a366a796b7876717230647867326b63647568327968376e34776635326b676b6868757a72717975336e323334756e777233376b6b6e63777963637a35676c76616d676a33726b6639663068776e7667786a77333638746378666d73746c6835706d3236306c6879787374646c637a7378717171717171717171717134786d30636d7865616d7767747276706364786e68376d65777576686b3770766b3335737436757471767a617a30656d727430727a336c74796476346c6876736766776377656b7a6b676b737a717235797663766e676a617a64726c36733370717a39637971633430756a787076617432713379636c663377726a706e647034797763386b6173376c3264376b6c3635746a657775636d65703771737a6d3735727766663977397568736b7467776d333370686b74643830687a3439636e303565616a7a756767776573397063766777713770793739383868676b357138776d673364706c73746e386773726433713276346e726d68706172736c6137713972766c687a7273307366396c6a3973336d34656a64726d767268673973717171787461747677000000000000000000000000000000000000000000000000000000000000000000000000415e96f3f3def3e1cc271323ae82fa17f11bede872e7a9b6d2026ae6c22421c5c25e27a34e52369e3890e929cdf4609dcb2a17ac5b2053a5d93beb8539ed333d301c00000000000000000000000000000000000000000000000000000000000000"
}
```

### 4. Checking inputs for private market 

```cmd
curl --location --request POST 'http://localhost:3030/api/checkInput' \
--header 'Content-Type: application/json' \
--data-raw '{"ask": {
    "market_id": "1",
    "reward": "10",
    "expiry": "100",
    "time_taken_for_proof_generation": "1000000",
    "deadline": "100",
    "refund_address": "0x0469866e13cd7DF08f5482FBb127a72fF197365D",
    "prover_data": "0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000043275363400000000000000000000000000000000000000000000000000000000"
},
"private_input": [123, 34, 112, 114, 105, 118, 97, 116, 101, 34, 58, 34, 116, 114, 117, 101, 34, 44, 34, 97, 100, 100, 114, 101, 115, 115, 34, 58, 34, 97, 108, 101, 111, 49, 114, 110, 54, 51, 54, 103, 57, 52, 109, 120, 51, 113, 113, 104, 102, 55, 109, 55, 57, 110, 115, 110, 101, 51, 108, 108, 118, 52, 100, 113, 115, 50, 53, 55, 48, 55, 121, 104, 119, 99, 114, 107, 57, 50, 112, 48, 107, 119, 114, 99, 57, 113, 101, 51, 57, 50, 119, 103, 34, 44, 34, 97, 109, 111, 117, 110, 116, 34, 58, 34, 51, 117, 54, 52, 34, 125],
"ask_id": 10
}'
```

#### Expected output:
```
{
    "message":"Payload is valid",
    "data":null
}
```

```cmd
curl --location --request POST 'http://localhost:3030/api/checkInputWithSignature' \
--header 'Content-Type: application/json' \
--data-raw '{"ask": {
    "market_id": "1",
    "reward": "10",
    "expiry": "100",
    "time_taken_for_proof_generation": "1000000",
    "deadline": "100",
    "refund_address": "0x0469866e13cd7DF08f5482FBb127a72fF197365D",
    "prover_data": "0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000043275363400000000000000000000000000000000000000000000000000000000"
},
"private_input": [123, 34, 112, 114, 105, 118, 97, 116, 101, 34, 58, 34, 116, 114, 117, 101, 34, 44, 34, 97, 100, 100, 114, 101, 115, 115, 34, 58, 34, 97, 108, 101, 111, 49, 114, 110, 54, 51, 54, 103, 57, 52, 109, 120, 51, 113, 113, 104, 102, 55, 109, 55, 57, 110, 115, 110, 101, 51, 108, 108, 118, 52, 100, 113, 115, 50, 53, 55, 48, 55, 121, 104, 119, 99, 114, 107, 57, 50, 112, 48, 107, 119, 114, 99, 57, 113, 101, 51, 57, 50, 119, 103, 34, 44, 34, 97, 109, 111, 117, 110, 116, 34, 58, 34, 51, 117, 54, 52, 34, 125],
"ask_id": 10
}'
```

#### Expected output:
```
{
    "message":"Payload is valid",
    "data":"b8f04d42c508c6a1ac6b85c8696d565944afd6a3e0a84defa8facc027c8e6c84777e08c10b861541a06834fec497864d94ec5ef00591c3161a5e11bb590532b21b"
}
```


### 5. Checking inputs for public market 

```cmd
curl --location --request POST 'http://localhost:3030/api/checkInput' \
--header 'Content-Type: application/json' \
--data-raw '{"ask": {
    "market_id": "1",
    "reward": "10",
    "expiry": "100",
    "time_taken_for_proof_generation": "100",
    "deadline": "100",
    "refund_address": "0x0469866e13cd7DF08f5482FBb127a72fF197365D",
    "prover_data": "0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000003f616c656f31726e3633366739346d783371716866376d37396e736e65336c6c7634647173323537303779687763726b393270306b7772633971653339327767000000000000000000000000000000000000000000000000000000000000000004337536340000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000043175363400000000000000000000000000000000000000000000000000000000"
},
"private_input": [123, 34, 112, 114, 105, 118, 97, 116, 101, 34, 58, 34, 102, 97, 108, 115, 101, 34, 44, 34, 97, 100, 100, 114, 101, 115, 115, 34, 58, 34, 110, 117, 108, 108, 34, 44, 34, 97, 109, 111, 117, 110, 116, 34, 58, 34, 110, 117, 108, 108, 34, 125],
"ask_id": 10
}'
```

#### Expected output:
```
{
    "message":"Payload is valid",
    "data":null
}
```

```cmd
curl --location --request POST 'http://localhost:3030/api/checkInputWithSignature' \
--header 'Content-Type: application/json' \
--data-raw '{"ask": {
    "market_id": "1",
    "reward": "10",
    "expiry": "100",
    "time_taken_for_proof_generation": "100",
    "deadline": "100",
    "refund_address": "0x0469866e13cd7DF08f5482FBb127a72fF197365D",
    "prover_data": "0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000003f616c656f31726e3633366739346d783371716866376d37396e736e65336c6c7634647173323537303779687763726b393270306b7772633971653339327767000000000000000000000000000000000000000000000000000000000000000004337536340000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000043175363400000000000000000000000000000000000000000000000000000000"
},
"private_input": [123, 34, 112, 114, 105, 118, 97, 116, 101, 34, 58, 34, 102, 97, 108, 115, 101, 34, 44, 34, 97, 100, 100, 114, 101, 115, 115, 34, 58, 34, 110, 117, 108, 108, 34, 44, 34, 97, 109, 111, 117, 110, 116, 34, 58, 34, 110, 117, 108, 108, 34, 125],
"ask_id": 10
}'
```

#### Expected output:
```
{
    "message":"Payload is valid",
    "data":"224b4cd908c661776c84e07491bf53263fe33994e368495ac785c69fbe0e60f7752ea92fb7caf5abdadd886bcaa37d54ea457927aa60e7f2a5144e6e7ee464211c"
}
```