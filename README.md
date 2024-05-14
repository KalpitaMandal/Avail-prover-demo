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

### 1. Fetch program 

```cmd
curl --location --request GET 'http://localhost:3030/welcome'
```

#### Expected output: 
Fetches the `credits.aleo` program from the aleo networks and prints out the program on the console.

```
{
    "total_time":"2008ms",
    "setup_time":"0ms",
    "execution_time":"2008ms"
}
```

### 2. Execute offline [Simple hello.aleo program]
```cmd
curl --location --request POST 'http://localhost:3030/executeOffline' \
--header 'Content-Type: application/json' \
--data-raw '{"function_name":"hello",
"input_r0":"5u32",
"input_r1":"3u32"
}'
```

#### Expected output: 
The execution id and the execution proof are received as response.

```
{
    "id":"7066704592553694670344556094960150780320939951583425784597049013447289679328field","proof":"proof1qyqsqqqqqqqqqqqpqqqqqqqqqqqrdwln87mpatsw60v8mnm9gvftxjflh0zh64r7qac9t865dw4cvmyrkug45ks6dsp4sm3z5t3advqqqyp5nwjz8w33s3p2anqf986jakpq202yjt4nzm7qacruyym8k6c32nj678zll79mldwe02xly6ggeqvy5gp6edfzfhjsglvgjtvly4gv8edl84jpsqf03f2l0waurjtl4e0pkns2gr7wz9d970va3znkxxq3djr0qh3k33qk664p906jgxf379e4pf56nkvyzxyrdvqaq3m8zvnlewtt57k6hfd5h3mkzflhk05q984ey437krs4vg32u2dp989dm2mw3rvxz4ncf6gu5e6gwawxtshwqzlccgmxyhhs357cjgs2wkwcpac7y37vzjpehc58g5xxhpz5ufeflxwspa6uva7kt5xhjquvadcrlgs0ya780l5g3vg9tfrt233vqrazr4a9qxffcux2kppt5n4mf7mj95d4fjwqrgs0fl3kfqj4mh9xu7l2ew6ape4xjsw803ce773fxqpxqtutymnf80tzaj8944sktv0ephrh2030tlmm7esru5dtndm2s3cz8fv54ypqpwm2amxf9kr4qxq8snu9pem9z056sfqnywurdaj0u9zv3m7q50znasjakwjrnyu0z0xff0judumhm65kvmh0a6c4vggqx5nwk80vhck25pkc36ecamys7cdwsat9fljvazwcaph4wndu9v9amzhxmss9wrdft2dj2t8a5tatgtn5hfywse929ju7q6769wmvjrl0le0jt6qfpn097r6c5x66tzzwcs54zgpk3fgrx50dvuyxlnuhp6jh8n2jsz638rwpww27zdyapwpgpf9l9rkd3f8vy6yex5dr0u8s484h8dluwcjwhzv0n3mmph6sv2d96pgk47yxmp42sgz0yplknygprzyddsmngx3m9jrsr9acm55s0utlanrw7k883xqz5w9nryankqxtexy3a4qlurcrq27v0n7xe4vjsarauee05ps2zx0wtwzssclrsr275n24y6qunmw6xzqn6y4vmsnmy9sur28asr0fpuw4uhejm5wmp4lca80lgk5pngaxcu2qpvfr6jdeshyecgxhqljzg6sudr9lrjvqal7yf3lf6jucpmv0axtygkv6kzzr6qes5463389xm7shx3j22cc8qvqqqqqqqqqqqkgcrxav3s0799nxm45dtze6tucemttwcxtkmvduk0wvyggrjmg4dvwe4tyndju44lky9t0uehulqqqg33usshfwt4r63tpxgrshthw8enjkh6sqew57j7qmdq8tglw3zaj6r0wcec4zl8hk7nexusstqe5pqxnwpuvy2hhedyngrut5rqv8xrt9lz99l9952xhddztxqvh8wmrsycflkx3eqvvwkj2kf525xvfmw373ncr9fe6khf095x99pps4mr9g46tj6gv356taqutnu085tld2qqqqdr7xp8",
    "total_time":"2864ms",
    "setup_time":"11ms",
    "execution_time":"2852ms"
}
```

### 3. Execute [transfer_public function, credits.aleo program]
```cmd
curl --location --request POST 'http://localhost:3030/execute' \
--header 'Content-Type: application/json' \
--data-raw '{"program_id":"credits.aleo",
"function":"transfer_public",
"input_add":"aleo1va0hzrcsj569gz0gd0mvue7xk54vku626nsmvl86s7j490x7yupq89l82z",
"input_amt":"3u64"
}'
```

#### Expected output: 
The transaction hash is returned as response.
```
{
    "txnHash":"\"at13fpemrvdlrttc807lagfrs0uapl8kph3dnqtma0mmf8y0xtd3vqq77pdre\"",
    "total_time":"9166ms",
    "setup_time":"0ms",
    "execution_time":"9165ms"
}
```

### 4. Execute + proof + verify

```cmd
curl --location --request GET 'http://localhost:3030/prove'
```

#### Expected output:
The time benchmarks for each process is returned as response
```
{
    "id":"8192708659475831108076709259453139065621886191373139353534557557121263654082field",
    "result":"[\"8u32\"]","proof":"proof1qyqsqqqqqqqqqqqpqqqqqqqqqqqyxwhldssexp4hr40v383yy5nemtqr97tqxy90pdu2x3kswjj52tvqgpyy4m854cp0cx88v77fgyuqq9d3xxesxjpyfwlu28c68a6eqa8c5gy2mv20pkplg5wr8dfvvc9ewj8ss8za92nr887zvaj9ghmp7qrfzl03scq77z89m9gm76x8mhv883800xjflsv2fd4wz27cnuusjh3n265c2n4kv96jd7yz9rgkdxq7tudnwfvh2zqg4ar6d5yd7ag9c4ehrtef86hw7xcfqfru080uclpgyd0tvs5rd5r4f7qjzkj6q3vqn0ytslh4l9fhmesmehe5x7zxwlfuj2hnq2qch8u70ft446yp5vwxaguhv95hmtnd0aqnx3n7pg4gr5szg23j4wwws82fntknwe08f7ppfpjycaz5hn3fapxqy3w846wk4d8de2rx7cks03p7mylygkfuqzfhrvdf707xkh9xkgw476m6uxqtw75kzlcmc093epk20pq9yqaujpx66cqeuz55gr43mlsxpxn3qqg9cv0rkzew0qkux3kywmx7ruv78603m45jwt6l3a42jr3rn37agx8dpneycdad2mnjjptd5lpjxwq3zvnynd4wluqqaygp2d6zt2kala6wvj8spelu9a7ky2ec2f8cy02jl0ltxr8lrec049htyah4sy5pfqjyfvf6s7pczkajlhu69wf5g3dukhj0w0qu6lcx9mfp350vuyx5vcukcwavf0sjxlqqdwcr8scpctl4zmk48uer4ge2cz5ydw86vz0vgtqz6gaugry50c9jf8p9dke52zjhpp4tffkz8wf3wqx0xa4xpjqqqjkqg5tzuhn87aj606w354paqrm3msp6raan99wfrptspmesskfxa4x02m7sk3udg69g67er8c79f4z7lgqtat94p5rsexxxd8sxtvxl4p8y3wykh53u76rp2kmx47zdutl2g7jz00hm47u850pdxvr5l5t99rv62nnzdqtc23l2h6v4jq8mc66fnlhyuu6sm60tafx8uqq0smcmklsjn364fjmvhmd903qrvd00zexqswuhg72ahqdrvq6kq9n4u3jjz4mymddn4c0tuvgu6lduhlsg69qgf7vrv4mfhs2uug7q8qrz48n420wkpv37ggldwkvf2vagqak5ztpc27rmhxy5qmguvgsxqvqqqqqqqqqqqdagf0l4wnpl6fx0hu238a5kwmclgv2nlf5tulrwta5ryee4z4gfu5n49puc3dja2dm5g7csm0uzqqqr6fe0nrk2lkrm9nf34wu0nt5f3ygu6e3ljv508aqwc2w92ndxjngtm0svhya4gfvv58krapyalkyqqy48a82t0j4re355z3m6r59mgpq7ksgs3xq2pznzhqp38c6g6g9qv6v8fyzn89h9rvlt08seye0tvx7dwfld5fxpludwcaap62se3df487pp4ptgqlq28q7m2zc9a0t9qyqqpa3n0x",
    "verified":true,
    "auth_time":"185ms",
    "proof_time":"758ms",
    "total_time":"1989ms",
    "setup_time":"37ms",
    "execution_time":"1008ms"
}
```

### 5. Multiple transitions

```cmd
curl --location --request GET 'http://localhost:3030/executeMulti'
```

#### Expected output:
The time benchmarks for each process is returned as response
```
{
    "id":"1100502659318215686866868489723070942391434777203420379677843819126341265790field","proof":"proof1qyzsqqqqqqqqqqqpqqqqqqqqqqqqzqqqqqqqqqqqqyqqqqqqqqqqqqgqqqqqqqqqqqqsqqqqqqqqqqylqtzda2rl6yz7jde73h3je29tq98fmhjqrjvdz0l5gaeqc7xym75ul2ac9ts94mhd640s43xnzkqqscvegg6d2cz9zznrwfeau57kg9xhcd34u4gs60sng48tn8z8yku0c42ts3lm4acfy7q7m8pwhyypfcqdag74m4avu8vrrytwrtl2ymal4w433gyw927km72d4rh9lcu3yntfehwl0c8dm67t6vnw67qsqtj0jhnnrt9put066y5l54fw6w0pwnemw67ss8xunuhgd3enqnhh07fa8ysh7lfgdqp2f2l0vlqkq9dvar2xvgjl9gzxrq2kcj99delte6tuymdj4xddyrg8zspjnhamh2aua66ua966yx2a3gcsh6l57qgpesavd9jf8nyvta6exxs728y2q4rfnj4ssdxf59rwg802qfzltcxwwqht5jjxlw32a48nzr568u4sqn8hjdgeevfd5pwag8zug39v7mc5jsnql3ut4ufg7c56lfm6x4ks2zl2c6gce0d76qex5ejuaxszqyw7xkvhc5g64yfqhpp3q3d5m4rff9wwg0770rdjj2crfaag3zkzw2nka26ajtjr5jl83eaj693ywqy5dv090363fu6z9smuh42j4gvyeegu5ksakxvrqk44h0jhkcwc4ey3tmh9cpadvqv3vh9rwhy0akq82lr92jgk23zeu50dg5mnzlc8uhf2mr2fz0gdjzm473c76ngp7unpg3qh094ekw6nsv5e9uwaeyvp4rsgvlzgjtun0m2vge59y0yld9cv74sezqney70e5g0tkskvt8szf25c2gk9lh75fgc8872wt9nsz6vm55cx6fynf250dah8cx5q7s58v4eruj0q6sa6k4tc0nwwezftdj7p0cekjcl9qaxv4yyqa7wdsr3v2lmpsgrcn7xdvk2slapgwqf8vca5aq0vrk6ysy676k77h6ejmt99jr6xhlet9lg8xtazx5a3uqxkcjxk2y2cvr9tshdgy26pqn53l8tlcnhshezyxd03lwsl039amtma8gaucemnsy2fca4alurg4gqt379eql5grrfk2ujaws2uzsamq8xw5phgelydu56kyct4jmasf8nex9nf47hatywdd2cx5dvsf7sq77n3e0c078cjv0nztp0p79vuk7eumf9m92fjt75amfa69afyeah8fjccrgpc4vrjd7tf9zm9gjfqrra2hg8ulzx3yravqkpt8nlph4l96gwl2z30krduvuwt69unrcw2kaj7c20c685fmgtyljhq5tqxsqqltkyvp4p3jn900w0g9r0a4u6222vqjjnlhrpjtcvlnf9vsk4lctk885qzhfkx3hwfumywgcaf6qfwknh0ymesyf2qrx5pdkmelfwuze8308r2pdj828ldqnqkysw3t6w9alhnvkhxc2tjaeyg7u74qxqy6cr3u0je2d3es30atxwq8uk30j0l09a6hek5pqgmxz44l2rttuxekhh3xs0ex2dp4d05sj6ajxcpqtm5ma2lvq4cpns6yp9dpamfln7u4kzzsezds9zcyzcg8da8uq3zfz8q0j3fa3hjsj22rd8dl3mczeemletcyd526hgs65jq7attljm52tu7k6n56rn3d4q06wkz4jyjvhne4xs2eylxggwdmqqgdry3s8306c2nnmrdcg8s28enknl7wyx76jrpn4vkn2mgdm76wll05cjxscgn4c3rn0xnyhndpxjzr0rc6qpja9h8pmva5gglmjrdph8akuvmjvyaj6ru03rrpt0cc82nczngzgj9slywsxwfkqcx5cxp77tu9kqxqwlf5had4psgmr6ltsh4fjxemz3xdfnc7wqcmf5d0wkezmv74ukpae26jjw0kz2e3p2mld08j2qqe8grrt6nhf07waalyehez4vuuxnpc2ah5q085dpepn4f4dgepqr2xrkvyvr2zc5nzs3s6lfvnkhh3upphd2zq83l0764mvygkf7euq2myqa9u3sv93jrr7ha2jfwu4sfgjn4xkahe3jxkssdkhj0vp6eqlkv4esd9svrqchkslf7npdmw0t7u7923r9zw5cwkma2wzrymvlq5t8r5an5r4v5r8nc2kh74zvnxm3q8qj0h4gazjrgw6yu2l47fvs8n9ssh065djqqttpg3t8as5gnngtwjh5358jf4qm7ua6y53zpqsr8ykzzxw3t9kh4f2sv6h3mpzq8gkgazej0707z980gw5vsyuvhyrr5jpj5xvvmvv072a40mg53g48zr7ne84jlaf0hzle4rjwqz0ysqykhm48dl3py5898vusshkesj0l0a6cxhr80n3kqg292fc9ps7ssmzqp0nkg6qfrvzsq7j4xxmz6u3mklev6gs4mftl9xv94j4j42tqrfyw79w0cvn0mhea9q39frrr6lhk3jzvd360lrm47upy4fa6ynug95aeu8m65auu8xerjth4qj4sv9q60cwgqkdtld9wheuf04sp3kqvvv2vdkwprcen3c98lmrzxnt7ptwftu5n8sy52uaa7shya5x8cqpg5d460p5f4u7qvzuexlq4mph7exjphcn709leky534sutrg92qntarcpzskkllvn8clnwlxt67gnlfpn6x5l8clfmltctq9wcutgqdtfmwljytsuyvcgn73q8qmm4w497yazmn0wxc2yl35kur9dqnp5yc3lwsaqn5elgxlcwhs8w6tauyja04a3476txafr2twrgkezmlgqawfx986yrs7r5gvm7dh4vjdqmj7eglrj9qm2w6j8p6zl3vurmzp095dsj3njrf58trkazeq7d438rr35pqahr6s8mqzmztw75p3cvsx6h3hun4g6v0zevve6r6c3t439u2gyzqeph2glk5erz8c3fp2asdasjr25apykdx4jh67mpwussddcyp6h5kryf9g37xk39h39ak2urkflmvl3xslsjrzf2j9nlvdpc6hqfv2sfmztkfmg684xrj2le27pzh3wwymg0nupkptngu9t3utf4hh69q9j2f89j2qf7gdzmgn8fzpzak00pv7cwvalvmc3v2334fsaa9x0rwn3d26uxtcq637j5867wseqktqv6t0z7lw6x3h9ywkkfe6606ujtuy0565rx0lrjlmqgwrtq25x8fn72hdvxgxkcnx563r3xsztwq8sh297aqngqmd8d9w72l5y8ltwe9w3l5vrgslr97galck2l5ejl28zxrk4257yxf2nn9xn0vuzux97tdltnk3cv8ftn4wzqgc07he3ydf84nl4kl5apjyldqxdn5zywepapeveyq2vfqqnmxwtfrj0h4cen9fksvxdlu9yhpgtf2yymppk9eqt2enfafu36zeppk62xvrd4jk80me9ytefu2ehkcdtz2pwcznqahz66j5h6p763juvkqzal5evekauqkc6xfr88uk573cx7l0qqheqv5jj6v9xhzw62nens67xrm2shg6lx6hernv9dh05fs4ltdcrddncpvdq7d6rcfcelngfzv34g24jhz60d8prs5702q0x9rhxl0qkuc9q99kqyxn6fxa4wh3j28erw285hx8ra2zvfxgxj0jdhn32sql8krm8zt2cn45fu24sml0p9f5sh8szg2kev0p0ftlw6f6gestxsj5f9ad46jcervy257nkwqunv232s3lrhaevgc0tpcgjj73nk8ds9jktg935n6csypx6gmtp2v84euzlgjhftaptzm2pur50x3eknmuzcrwkqv48ww83tuwe3khwhj2v0q8vr5aelcd3vxygaaatcgl6ggpp0yx7zejgt20y0kwrj72y3r6jmxj3vjjlnme3pw65zvtfpdkk33qfzetltcuezexyru0pfmqkatmtklm4g73hu0lgkjrg052lf5stqdxneqz0r8lj5kk2usp77nl2v505z92fjj30qp5v88yzek4x4pvgyfcazct7lke54g04sfdh6tg25jku7zw6xcesmy02vv7dya3nr7uza84ktpxmpktlfj65xlk523d5kpwwdsz3dnha0hzzzna4sxvwtlzyhy965g3jj634y9qkcmjvc3yd4dhns7tn5g3r4hpl8palkygtgqjmgsqjgk7fpw596v0fdjqqsf7ck7calezahg5r4e7v7xapjunccf8zw239tk8hplzaggnsr9ka2nyn3yryr36ch2kxxram6r8v37guzsxqqqqqqqqqqqw6h5e2hhctqj9j35aq0ngdy7x5saqmrs5atutymyfrtuhm7zp698tajcuxe8nmh2c7ldz6v64xmsqqp5uz0klqk4380zgh3wwlmp580grd7vq3vfmeks33yjf4reryyf35w52aq0uxu7ecfmc0ajnylvgxqsrwfsatthfqwnz0m2zp0sfl679swvjt5hdxef4z5r602jjcp68eqryw8skyx7hfkyrsrwpjuwmnuqxdkm3mhsn600a7xfz5sx22c8havv72me3p85pjey57h9msszt5xcqqqw6uwq5",
    "total_time":"14390ms",
    "setup_time":"2ms",
    "execution_time":"14388ms"
}
```
## AWS benchmarks 

### 1. Fetch program 

```cmd
curl --location --request GET 'http://65.1.148.110:3030/welcome'
```

#### Expected output: 
Fetches the `credits.aleo` program from the aleo networks and prints out the program on the console.

```
{
    "total_time":"1042ms",
    "setup_time":"0ms",
    "execution_time":"1042ms"
}
```

### 2. Execute offline [Simple hello.aleo program]
```cmd
curl --location --request POST 'http://65.1.148.110:3030/executeOffline' \
--header 'Content-Type: application/json' \
--data-raw '{"function_name":"hello",
"input_r0":"5u32",
"input_r1":"3u32"
}'
```

#### Expected output: 
The execution id and the execution proof are received as response.

```
{
    "id":"1221237096892849335876649717997127501297669822767237882404786722779612693128field","proof":"proof1qyqsqqqqqqqqqqqpqqqqqqqqqqqyp935sgvnkf242jcfqttv0pd2y4n9juatms3x8yzakyaps9m00a708gc252za9mhfjz0d42045xvpqypvx25skr85dn4acdnuvx66kvxz6qk9d8465yfyanxas9suvpgpr5wf5ha2s3v8tw67atpeexakjqpkgxr0a50dhka7fdr5q6atld8y3mqkrdvd7ce3pm6m0engme5vdequ6kzx4apwms7qvzs5hmrdw5qxjtfew7pz56dvyfzkgacfump8m8ty4z0lypr540uhcnpvmu8wk63dsu954k88yvtpcjj4ltja5zqqzr7yqjzwtuj4qlcdv4z79z63p7sc0d0n523awma6vepserlwvyagfkluk8t5386rrvg2srz5zwjqzace8wuwr36u7ph79x7yafk3s4fqysup5gx644yxw230rtmd247qd0k7we7l87xzwq5ycm7qnw02szs0zpxtjzjjcuan5uc4dnl2j5xsfxm4gzjwcj5ftw65cr6zel0866sl9gpyrmgzt7peel5628fkqqtctzpswsfa9tk3ccqhhcjkag2v9lrrswk6rhdvqsv2ujscwyxxaerfd3tgcpnxj626n9eg2gt3ssqp7m5p5kfcrjnpw7l49fpzuz9x5ngtm0fq8d9uaqjfvq7yndaaqkzswj80z478a8uhh2adlyey5ugpwacpnpelhqzfmsdqunuuat7nkuydkcpz53334xq3ncu9pf3cwvgla36szzj6yshhqntgkxjalj59a8emr8zu5kekggryh9uhjhwe2yd7p6sz6uymvx4x3ejylwe20umtj4h57dcmnlfafwevx5c2hsuuqvezpxvzsshg4va96en8vc2gnvvn89vkz72ruw6xgp3ze7yqczv3yetsa5gkdlnfd6839dlf4smwxyfm95ksjtqmlegmgmau5fsw55cdzwrra04s5y5dcajzdg6sx6x4ls83hg9hkz0fp8zkkesgmxa70y8myxfq6ghz5aqevj740znje7xsk3em7qjchh63y9gs82xwrqfc2phj9p3mweaw7vjta5jjexe3elku0u6llsuu6l3uldvlt9vplqk7p4k67yyuenlavpvzp4kjy3yp0hp4cgtfymdg9lvt5yc4rdydnatsq0wfefxrr56lgpherxcdze6rahsx9afpnp5kq8cjd2ty3rayvns0qvqqqqqqqqqqprtfepy7rkm0s574pr26nwcrjtjrsezyshhqqqd2yfdknddnwj3hzpxjqgfmuaqkmduavwgtdpchsqqgskrfrff4r5jxu6ht7ndag4mvs07d29d6gfvksxxuppyuqyh7a0n9ztkxe7mr348f9jajr0dglagqqykef3n5n29jazhjr4pxa5466w0shg94uzy75rt7ssdxxyk9czzqvdpf0n6h6dpyaz0swzw92fwyulsjdx2nsqpqqndg0wfa6hdv0fuuy8lnrfnw4fqwhcht8gswlrtaqqqqd20zk9",
    "total_time":"10901ms",
    "setup_time":"6ms",
    "execution_time":"10894ms"
}
```

### 3. Execute [transfer_public function, credits.aleo program]
```cmd
curl --location --request POST 'http://65.1.148.110:3030/execute' \
--header 'Content-Type: application/json' \
--data-raw '{"program_id":"credits.aleo",
"function":"transfer_public",
"input_add":"aleo1va0hzrcsj569gz0gd0mvue7xk54vku626nsmvl86s7j490x7yupq89l82z",
"input_amt":"3u64"
}'
```

#### Expected output: 
The transaction hash is returned as response.
```
{
    "txnHash":"\"at1dqgnjjpvgddf0tjr3lgkqdzrlg7r356ksrln6d88xwt4nemydy8ssuv6l7\"",
    "total_time":"22088ms",
    "setup_time":"0ms",
    "execution_time":"22088ms"
}
```

### 4. Execute + proof + verify (simple hello.aleo program)

```cmd
curl --location --request GET 'http://65.1.148.110:3030/prove'
```

#### Expected output:
The time benchmarks for each process is returned as response
```
{
    "id":"3645275379314345593902065154351501178850869022708450730809597064811590163080field",
    "result":"[\"8u32\"]","proof":"proof1qyqsqqqqqqqqqqqpqqqqqqqqqqqfsata7mj3lep5nfv2y40h6ey7a0wysycxks5jej4adqydergd97dfm9lttav2v0fntp77wedrkzvpq9n6nfy4lu7kfestdym2vvsvk8sazhcms2crel3pfq9ze72n28gfegal8gza2370yczsu4fjsjmyzqxt22unax7qelex4fcv7dxw9f0mr8whe99djtrxwgcdjttp6tday806zew8au7acvhtyrrw3ap8aqqxfww6tx0vg7m45grs2qz7de5kel85pqmzqu57lpjaa4qng072zrl8z5k2v2d2234644tmnznytsyq6krtcpg8njssccl34tqxxuh025hjz3a5znpur5fckjf49qateruh7x9a34ks5kvkuhyq5pls68dsqxqxmwfc9wr3kchvh3tllaeekt3hgjelzenrne00fyymyky2rz9jw7hv5y4g0s2y36ecz3w8g5skq8ajmkwftyztrl55gv26y6wtr2elan04enzlfha2wvk88lzess9t4x7pha2q5jyu9azsm4gj2f9j3qd83rr38e3wjdx2m65yj5uh23afc9wjxwtu25atgflxd87spelnsdkelw3ms4jja8s9u5xgu9rf87qc555r4pl5s6gmn354e2qzmz9stmtc2t0t5z7z3k7204lj3wdkwxv2s0uh48qcx69wqheqeytejq5p0wn20lu84ydv6le2gqvfq77gl4ce0d4eyr5p0z33v2rlskhr4yft2q0hwfgverc0p93e5ly5d5ky7d7c9hcdz9zzgdq09u2x4ychqrgdh0yd4r94ce2m00qc0n54j58ewmhrj0af4nvnysrteraav00upuyvf9q95hn760tdxz6qzwahrnlpsupakzgkwv9d9lzf29325lxsepmwd59cyq2ftmv3pdqgx356w9a0jkd5e0cehdz8994kggxd5wgw4nwykp4xw9svkxc52yhe54dpgyf6gwa8qeqdwl86htam87mjps9vj97ttj96kcwuwclt7p5jnmhcgz6y7jw6n6l8vl8e6du7m6ha2pt22kegkyqsnhlrjjr4m4f7rg637l5h23l6vy8eq8pf3g3er485psx95qfuavnz2ya59zp9mac628u4l7pd3tq6pv2m996n5zdcdvfsvg4a542902p4rvkkhlmr2s347ryre3xym48dhc0vnqdkdsm3pxsqqvqqqqqqqqqqqv8m64qgs7gcwr93eq6jarrxllyvg7lk77clyvscnqdh4qz6zpk8r6cl8c7qm9cc3un0psen7pjxqyqqy33txazmflex7qntr7mf8l9nv4ld473jv7kwmnr5ex3t4zgxtawka3znfrwlfl9ucm0skszc5ryqq9h4ld026kvcz8twwd9a8lxvven96g5naxmlhmvv4cyzyc42ufhsplh8xvqj0k5f2hkk7q4megujkz9dqu5ms4ppc5rpq9xazl2h5zvtsye4zprka7pnrf933swq8wnesyqqvkfxqv",
    "verified":true,
    "auth_time":"123ms",
    "proof_time":"4201ms",
    "total_time":"9773ms",
    "setup_time":"19ms",
    "execution_time":"5428ms"
}
```

### 5. Multiple transitions

```cmd
curl --location --request GET 'http://65.1.148.110:3030/executeMulti'
```

#### Expected output:
The time benchmarks for each process is returned as response
```
{
    "id":"3501160642396527933271394328288804745844886526047521376920633768995089694956field","proof":"proof1qyzsqqqqqqqqqqqpqqqqqqqqqqqqzqqqqqqqqqqqqyqqqqqqqqqqqqgqqqqqqqqqqqqsqqqqqqqqqq9kmqs3gfy6c66y7xmfu5heet69gqzysejpquzn2xjtvt2cdgv0ylgzvwynz88k98qylx0xvf4enqql47f3jqtfk8dvafdm70pnqyxyccjvjscuxgrltmdjdp49an6ke90j36aspvzq7rd2y73wk8ejvtypwggp4cyhzehm2zwv2g4kulw3kpfe6ujtj472x9e3z5ls4l7ta2pxkl3pcrp2vwtz5s7e7r88h9kgrh7p4cc0twywe675vpywmacz7dl0rdz50dw978w4cjgse7t46rarszfa29dy55jd2t7hj0uc2nmes8nk6zctrujz50zkmgfr0thmgkk0hrkrqf2pnqldcg3dnvw84q5h7r6arrew877pv4aaqv6eq3jzeqqpktwyg5t35c73uvfknw23eykx7k8p2ta0ewj8s6vyxdyllul5wwk3t8cws22gtwejg694cnsa7tpgp92xfq5mwd87ya3u5av58g9grx5mw5nwrfmhl7d2ygmfdmywgdlm2x7yqxmkttaas2xqhdchuwens9y0qvkvrjqcjytw5h4n0jkew2fvfq59wp5la6htazhuwdmhwjgsfjhhdykq5f02u539uxhp46njxqtw8qdlurrmd6w63wv4q8yfltcx8r30earpg6x0gz63ld5eypa2uztt228zj5s068rtqucdw5lh2uqwkj3peg06egnszwnhaz8yks578ne4lrlu6yv94d4fh73rrv0dfzey4l78xfpn7k8hanly7e4jargqtkyuzns7mypreffx72706q5hcleakshnpfzcd9djudy99fsk2dnrtqzym5yyjutdjqapus4caq4qpqdazaa7dke73djqh0w062cg9l83uyzhdmlh5wa43cq053fpa55ja5r4a0qpk5rlwfnfvsmg2pk8szl377kzvn3rqag77nj6hwvxcr3qvt8k059g2y60uk3rqajlr9llk6jxflqmyel86swdp2m7j3df7qgj3ujlurvyp6kpc7xm2k77ldmwjsjapdal30c2clvy6pd7xkdgf5xdrtf9qufupx8vpa7j356kjwqmxj3jc20hsxyzkyvhjg9ave4f0v8f23qnkvhtkvj830xswance3a5r0rsclugukvt772fenewn6cqc69x5u2evdzeplqdj8adfle7znw4qa0cdufz9pu582ulsr4zczq6ja43kppkfavm0zxp6evvjkdgp7p260750tyfeen7fgqfyz38ad7mctjf3a2v7z5angswaeph2jd30y9ncvlvfyscz7q0w7536dd4szn2d0wf4hhsk87h7gg5c66tusqk98zawz74jqny4e7dhjmxm5klsj4t6s9aj90jarffmkaw9edfnqwv3avxa2chc8pgt5q4zg8n7tlfqveyhjryc8knw5xx7hdc4x6slp74svt7ernn4ypzzv02emtvm7qpfxe5q9xeula5rp0m69a03dxrqydc6mhrt33x9yfm2nh3d2fq0x99skc0wmp9378dyq90w6j5whuqjnaynw0hd8p8pxa6xuq7hkkrqs76rwg402vg7pxjc9zxz9mhpmflkpvcxcccpdgrppayhwgez22qz6jd65qnx9uy0297wgp0402kl75mdct3dpdf9u78htjp0xqtxtzl7clr3gnhyahlraypn0uttancqx82mg7s4f6gtx02qaj4z3uxqz4sfs7mfzkq5n09hcy8ukm6tpj4jptd3h2gzksffg8c7r3vhz663qxda56l0knpg4yeyta3sn7u8djw48ccljcjy5lv7uqyntgewxv00vhftg854j9l8cpx8fqw8z7teuqztgx8fhpy0kwmuvmkrsn4pc20ucgmgfpg08qqpt794mewxrz6h2f6ja6pxursl62ejgjv9lh3ssuqqar6jfc3w9damqe8auxtsf7pa7eckz3g2u7msajl3uv0kvudegglz3wdpv0gdp7ca3ds534888ctjtk3g9uf0pnltrghr4mp07fzxrfa49tlw3jnjfl2gkpe6qjdtn0269ln49jq8206e7ege4te3g92ztqd7g5xmpturxv2trdzc3qh3auyf995q7r5ksl5kgtcatlm39f3zra0424edmecnszcgsdnypxf63ntcaevy0zk7gdfms3l3vxzpgss5063yrn2sc3zley5xwl58tl5qnjcs2kf32j8zmujx36qm9ca9yzxz7hzwe6nvtrrta9enklvmlzhnw34kkxx799actcgflsy3fhmspgl33afsxjqv0r696e6vn50mzpwehnfp7dygkw340qjdsl69ly4pgfa3mae4vjs9ny9tv3my2jdj647y3p4epmfgp0klsu3mvm3m5qq3dhsztlk7yltx7p84hvvfqmfl20am2ds7krxwvgtlmvthldwsecju87cc0d6kyjfkp6kq44llzvw9zm66xpzds3kp7tnxlpjrql3dupue0lzfvh9k9juzc7jgl7tc2agep4e8055gz4w4r55e8vkknaukyfa9lplucse0vfmfsjlt8q0zf8pejjel5netd7kgevpgmsx582lq7nvn4mafgpd5hektfgjjfpzudm50avwt5ggzstplc27r37x3tvsng2g7rzah039wj3m349gakqhrdveg7lmlyl4n8q2j22jjt0ltkqyakxfvg4y70lr3ye4jk5ttjj7hs83ds6fy5yda5758shzwkywnqz6r43t0ypfg5mwjkhfzlrqcjepsjs5egv7w6r4ldhmxh4v32h67z3r5wxd8tvrjxyqxqcswte9350zhs4cv7lqjrp2k5qkhl48ez6gzxhndeauj469qeed6ehuuejc2ape5cup3y8z3gd7wkkctu6q0dfseqye44x5aja0pt8rgp26tgpjyt4lxl2mlek3yympcu5vavpswksjpaggt75eryplun7z2q2uvc3g6vqrxfuez0wk6l9j8dse2rygkcgqlka7nqj8deahwa3qm0cqtsg7mcr9ffdlfrcfx44cxn5vt0ga7rse3400j68hq3uqzwycm96erj6exa9ncwvcuxqulcsq3w5vpa8sqfdsgfs922ue3mrz7l3sk697jd7sk9jtvn8qj3gn6c98v6lxjvuppdvu96jpemlwyvz3g07uxy6yct2pec7d46fkk9cqqgv6hs7ddjsfku3kuxssqek0qepwecdqcdvgvx3zz6dk7dy08e2u4tehtks07ugr8dc4zueurupazp5d2ggtd48jem9x603p578cvm9qk4ehfs7x5rulmgeae9gm4tk2vz0kj0exmlgamckvf7kmn07pkue5nlm2fn64q059yh4dedglhqrsmyhsafjne5y9807sl6c3prdjq6t9tgcg2clp9ynn8864lcfjzpny28f4j5lq0xsu36wm6svw4er3ll945e0ft5sd9ge7q6lpgxpkht65aw5uu6zxfugj5g9llzcxw02zxud7trf2xsyxep44uehaq5ymcm28e9ejakae245pluvhrt2nu053j3uqtuwqxp0q89m9pkdmrwzeg4a63r2ck864k2tvgvt8eh2ml2mg6l8t5n4upyxngjcwnu3ylraehjwq4hy969dqxwmg8n24xkfu8e90rytu25gq86t300lyuuncn5llnu7xt2paz4qkw7q5vhz7tc95usrqy7axfkc9pmzrzfvah47wv4vqls4wgsadllg3kx3q4f6mazyx6dk4s8msd59y8fk98sjq8stcr82suk54ym75xhed8m2ceszct6vfxnklc5fwyq8fhdj4sqgq6gcau2zx5kvcz5zm6g8xh3hhk5zp4j22a7wze5kjpac6g2mw5g58ezh8w550w0endelkd9m38gmurt3jkvkvzuq8qpcqmkzmv2l7esys9esv6t5al8kh57m9hycts7jt8xgqg6c5semmn4c9v7mz63dl7c3trktx8ftjm9nhdzd2x7qcstxcmje7uzc0mje66585d3th79p6zg26m5vnhaugyjsfs4dtx07s97257zxudz0dzredzq77rc5ng0shru72ycwhm3ffulyczw2kngr7vs50unffp6gv8kelp88mtcx57lwgvp430xj0ch79uty8v535dxe6pgdarnkrgm5hdgwsafq973gj9x6crmdudgwyvvla54wqqc5va4476hqsmf6tm7kvh7cw2f4krmryxxdakw772vvaygu0r2rpuanqtrahrzaysmul2vw575xqxqqqqqqqqqqqrjtzfmss3p2ahwts3l62u3s3hcw08dppxstdtpt3rdpyszlt5msx0axtwewmxx5tzy9m2aqr7gscqqz0c02adhr79dhz067hr20zrk6gpwcvhy7m3fqumv079z8vndrzgz0drkh0u87f02jlh7sqm7jajuqqrppkzhdm4ajpvsz7c8j9h4duss74e5sdpgzyth39fr8zpfuy5tcx2falp90uhh0j326llc24zq7rnpuhnjzs30qtklg5erf23cfj8glsaxjjvgzs6n98kuykwh3er74qqqq7svq07",
    "total_time":"66626ms",
    "setup_time":"1ms",
    "execution_time":"66624ms"
}
```
