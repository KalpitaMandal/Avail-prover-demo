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

```

### 4. Execute + proof + verify

```cmd
curl --location --request GET 'http://65.1.148.110:3030/prove'
```

#### Expected output:
The time benchmarks for each process is returned as response
```

```

### 5. Multiple transitions

```cmd
curl --location --request GET 'http://65.1.148.110:3030/executeMulti'
```

#### Expected output:
The time benchmarks for each process is returned as response
```

```
