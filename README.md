<a href="https://www.jetbrains.com/?from=whisperer"><img src="icon_CLion.svg" height="40px"/></a> 使用 CLion
制作。感谢 [JetBrains](https://www.jetbrains.com/?from=whisperer) 对开源的支持！

![注意](https://img.shields.io/static/v1?style=for-the-badge&label=&message=%E6%B3%A8%E6%84%8F&color=yellow)
当前版本号为 `0.0.4` 低于 `0.1.0` , 故而在开发过程中可能出现大规模的代码变动以及结构调整, 请及时关注最新的 `release` 说明

`rust`萌新练手项目, 因为最近在学习压缩,加密相关知识. 也因为目前网络上的`曰`类加密工具基本都没有开源无法保证长久有效的提供服务, 就想写一个开源方便的项目来避免相关的风险.

声明: 对本项目涉及的非法使用概不负责！

# _低语者_ / _Whisperer_

_古神的低语_

将文本编码为简短的中文字符, 防和谐, 并非加密. 如需加密请使用其他实用工具

## 安装

### 1. Windows

前往 [releases](https://github.com/Borber/Whisperer/releases) 下载最新二进制包

### 2. Linux / Mac / 其他`rust`支持的平台

下载源码, 使用 `cargo build --release` 编译最新二进制文件, 后续会添加多平台的二进制支持. 请耐心等待

## 使用方法

```shell
// 加密
./whisperer.exe 123
低语:诵想帝
// 解密
./whisperer.exe -d 低语:诵想帝
123
// 字典校验及排序
./whisperer.exe --check-dict
验证通过!
```

字典校验及排序, 检验的是 `config.toml` 中的 `dict` 数组

## TODO

- [ ] 服务器部署, 提供样例`api`接口
  - [x] 服务端开始提供
  - [ ] 更新为post方法, 提供 `json` 支持
  - [ ] 部署云端
- [ ] 尝试是否可以完全前端化 -> `wasm` 
- [ ] `GUI`界面
  - [ ] 历史记录
- [ ] 部署`Demo`网站
- [ ] 提供自部署方案

## 已完成

- [x] 添加随机字节码, 密文增长一位, 增强随机性
- [x] 去除字典顺序, 每次启动时自动对字典进行排序 仅需保证字典长度以及中文字符唯一
- [x] 字典校验工具
- [x] 命令行封装
- [x] 对输入字符串进行判断
- [x] 常量移入配置文件

  - [x] 准备工作

  - [x] 求余密钥

  - [x] 字典

  - [x] 标识

  - [x] 派生密钥

  - [x] 优化关键词
- [x] 使用简体字字典, 避免部分平台对繁体的操作
- [x] zstd 压缩数据
- [x] 自定义求余加密算法
- [x] 基本功能实现
- [x] 对常见链接进行优化
  - [x] 磁力链接
  - [x] 百度分享链接
  - [x] 阿里云盘分享链接

## 常见奇葩<sup>褒义</sup>加密对比

PS1: 长度不包括`:` 及之前的标识

PS2: 因为`Whisperer`尚处于开发阶段, 算法更替较大, 除非导致长度变化否则不会频繁更新下方密文

### 中文

| 版本 | 长度 | 比率 | 文本 | | :--------------------------------------------------------: | :--: | :--: | :
----------------------------------------------------------: | | 源文本 | 12 | 1 | 起来！不愿做奴隶的人们！ |
| [原版佛曰](https://www.keyfc.net/bbs/tools/tudoucode.aspx) | 48 | 4 | 佛曰：不呐佛故梵除梵蒙皤知奢數皤盧俱若諳度豆蒙娑故知孕他俱孕有呐孕奢礙冥逝姪罰那蒙顛佛冥夜依耶冥夢缽阿
| |        [新版佛曰](http://hi.pcmoe.net/buddha.html)         | 83 | 6.91 |
新佛曰：諸隸羯僧羯降羯吽諸羯陀摩羯隸僧缽薩嚤劫咒空叻蜜哆斯慧陀嘇哆嚴哆夷修心降吽陀慧菩伏菩寂劫吽喃伏羯陀羯諦慧闍羯囉陀羅羯陀羯劫嘚僧羯諸羯羅修祗囉吽羯愍缽羯波喼是眾如如羯囑囑 |
|           [熊曰](http://hi.pcmoe.net/index.html)           | 53 | 4.41 |
熊曰：呋既既現訴類常人囑蜂唬麼類嗥冬告告囑誒象取噗達唬哮類樣嗡萌嗄森物嗥出家蜜達嗒呆既吃溫唬哞咯襲拙告你嘿物人 | |           [兽音](http://hi.pcmoe.net/roar.html)            |
75 | 6.25 | `~呜嗷啊嗷~啊啊呜啊啊啊啊~嗷~嗷~嗷呜~啊嗷啊啊~嗷嗷嗷啊~~啊~嗷呜啊嗷啊呜呜嗷啊啊呜呜呜啊~嗷呜~呜嗷啊嗷呜~~呜呜嗷~啊呜呜呜呜~呜~啊啊呜~啊嗷嗷~嗷呜嗷呜~嗷呜~啊嗷呜~啊~~嗷~啊嗷嗷啊` | |
Whisperer | 37 | 3.08 | 低语:夷行敬瑟尸百昼文积游特寡利护族穆教山伊拔多界求沙孝谛哈妖下死谛文信开施怜戒 |

### 种子

| 版本 | 长度 | 比率 | 文本 | | :--------------------------------------------------------: | :--: | :--: | :
----------------------------------------------------------: | | 源文本 | 60 | 1 | “magnet:?xt=urn:btih:
857a45dce37a2a1330ba2414e5a734fa10050f3f” (无效种子, 不必尝试) | | [原版佛曰](https://www.keyfc.net/bbs/tools/tudoucode.aspx) | 191
| 3.18 |
佛曰：謹呐死侄故侄不集冥滅侄盧俱智哆謹故能老爍罰夢等奢離伊穆呐勝迦道夷漫冥他諳殿利遠怯勝俱伊娑冥是梵孕罰一特參梵迦是輸盧缽離阿怯槃老呐遮室知瑟罰死神奢盡集諳地哆謹提怯無侄跋伊集缽遠豆朋耨哆南奢耶智怯知冥以曰侄他怯參罰室哆竟呐恐奢涅能逝漫梵除多罰舍冥能夢是奢吉朋哆勝怯殿藝怯寫皤瑟缽心殿缽利勝僧世心依諳竟冥醯跋那罰等死皤舍哆漫俱帝竟梵智涅奢一朋侄漫沙世摩侄等姪奢想罰喝朋諸呐姪奢伊都上麼俱沙
| |        [新版佛曰](http://hi.pcmoe.net/buddha.html)         | 106 | 1,76 |
新佛曰：諸隸羯僧羯降吽諸陀摩隸僧缽羯薩願心蜜所愍眾吶囉所願僧吶莊劫眾嘇降我心愍羯迦僧摩摩願嚤愍迦亦降羯念嚤咤蜜念咤修羯諦陀即嘇聞隸兜迦寂夷念嘇寂薩薩迦缽夷阿所寂如摩羯囉諦諦兜嚩寂薩摩慧缽夷念亦羯願聞色阿願咤喼聞念如羯囑羯 |
|           [熊曰](http://hi.pcmoe.net/index.html)           | 76 | 1,26 |
熊曰：呋會現食哈雜物取和喜覺嗅和會哈噤魚咬山哞既盜喜誒氏噔我寶怎喜樣沒怎噗訴捕盜發你誘嚄物出樣森盜爾我和啽告很我肉蜂人象堅噔告噤怎嗥嗷沒森食唬呦呱意既噤森很圖 |
|           [兽音](http://hi.pcmoe.net/roar.html)            | 410 | 6,83
| `~呜嗷嗷嗷嗷呜啊嗷嗷嗷呜嗷呜呜~嗷啊嗷啊嗷啊呜嗷嗷嗷啊~嗷~呜呜嗷~呜嗷嗷嗷呜啊嗷啊嗷呜嗷呜呜~呜啊~啊嗷啊呜~呜呜呜~嗷~呜嗷呜~啊嗷嗷嗷呜啊呜啊~呜嗷呜呜~呜啊~啊嗷啊呜~呜啊嗷~嗷~呜呜呜呜嗷嗷嗷嗷呜啊呜呜呜呜嗷呜呜~嗷呜呜啊嗷啊呜~呜呜呜~嗷~呜呜嗷嗷呜嗷嗷嗷呜啊呜呜~呜嗷呜呜~嗷嗷嗷啊嗷啊呜嗷嗷嗷~~嗷~呜嗷呜啊呜嗷嗷嗷呜呜呜啊~呜嗷呜呜啊呜~嗷啊嗷啊呜~呜嗷啊~嗷~呜呜嗷嗷嗷嗷嗷嗷呜呜呜呜~呜嗷呜呜啊呜~嗷啊嗷啊呜嗷嗷~~~嗷~呜呜嗷嗷啊嗷嗷嗷呜啊嗷啊嗷呜嗷呜呜啊呜啊啊啊嗷啊呜~呜嗷啊~嗷~呜呜嗷嗷嗷嗷嗷嗷呜呜呜呜呜呜嗷呜呜~嗷啊嗷啊嗷啊呜~呜~嗷~嗷~呜嗷呜嗷啊嗷嗷嗷呜呜呜呜啊呜嗷呜呜啊呜呜~啊嗷啊呜嗷嗷~呜~嗷~呜呜嗷嗷嗷嗷嗷嗷呜呜呜呜呜呜嗷呜呜啊呜啊~啊嗷啊呜~呜~嗷~嗷~呜嗷呜嗷~嗷嗷嗷呜啊嗷啊嗷呜嗷呜呜啊呜~嗷啊嗷啊呜嗷嗷~嗷~嗷~呜嗷呜呜啊嗷嗷嗷呜呜呜呜啊呜嗷呜呜啊呜啊~啊嗷啊呜嗷嗷嗷呜~嗷~呜呜嗷嗷嗷嗷嗷嗷呜呜呜呜嗷呜嗷呜呜啊呜呜~啊嗷啊呜~呜啊~~嗷~呜嗷呜呜嗷嗷嗷嗷呜呜呜嗷~呜嗷呜呜~嗷~呜啊嗷啊呜~呜~啊~嗷~呜呜嗷呜呜啊`
| | Whisperer | 45 | 0.75 | 低语:卢令舍弟妖中稳礼开萨住藐凉智千麽孙姪慈诃智舍孙刚庙远遮印难七萨亲璃恤蒙哈参亦虚五矜桥牟夷忧 |

### 百度云分享

| 版本 | 长度 | 比率 | 文本 | | :--------------------------------------------------------: | :--: | :---: | :
----------------------------------------------------------: | | 源文本 | 96 | 1 |
“链接: https://pan.baidu.com/s/1yLLxGMGn1G4cf795gIZ6xw?pwd=wue5 提取码: wue5 复制这段内容后打开百度网盘手机App，操作更方便哦” (同样无效) |
| [原版佛曰](https://www.keyfc.net/bbs/tools/tudoucode.aspx) | 330 | 3.43 |
佛曰：道醯他薩怯逝侄寫參哆夢真俱除姪奢遮呐謹奢竟侄諦室喝哆道智缽。喝奢想侄夜呐跋諳朋顛梵輸奢曰皤等死爍俱世苦呐羯上實呐隸諸冥勝羅諳參耨悉俱。舍不罰麼伽梵竟侄大冥伽缽度冥耨侄吉倒罰蘇切俱怛呐能侄波俱輸梵苦怯輸亦皤度哆不諳老豆梵諸罰跋梵亦奢真諳怖闍俱菩盧冥菩侄地皤亦諳實侄地皤喝罰切爍皤道夢怯阿若罰那皤訶罰隸俱無梵夜寫怯實侄涅陀三隸梵婆罰夜缽曰涅特伊特哆遮老呐舍罰即輸奢豆皤呼皤亦菩缽恐梵咒缽漫麼罰彌奢度那一者呐夢呐竟冥夜以罰想呐般缽心盧諸梵道密曰怯夢奢死冥逝罰顛想伊梵羯奢怖一等蘇阿梵明梵知利耨罰倒奢他缽舍咒呐知俱夢逝都奢訶切涅特穆勝侄恐梵世梵佛諳姪醯冥爍怯遠栗呐等呐怛呐特罰蘇罰藐那缽悉梵遠他知神究皤礙帝奢倒侄麼以怯度亦怯波尼諳藐缽陀輸勝侄輸豆冥度俱遠奢明耶哆楞缽吉呐伽
| |        [新版佛曰](http://hi.pcmoe.net/buddha.html)         | 238 | 2.479 |
新佛曰：如波阿如咒菩哆訶訶莊諦訶摩喼皤陀訶訶莊諦皤訶摩喼皤陀嚴摩叻叻斯心僧色皤喃阿彌尊咤缽夷是隸宣耨嚤須蜜彌塞伏慧皤斯修尊波空兜皤囉慧斯寂須羅眾皤嘚隸慧皤隸塞吽諦婆所喼兜咤囉夷羅心塞夷皤寂彌寂斯寂隸摩囉若咤訶波寂陀尊嚩愍嚩是嚩若聞囉嚩缽空色囉阿宣波皤愍摩宣空愍訶伏諸薩薩諦莊薩缽嚩即訶寂眾聞訶嚴阿嚩訶耨慧羅皤莊須吽陀斯祗祗咒訶尊皤莊咒訶祗祗陀訶摩阿叻斯耨若愍訶須阿聞陀斯羅叻訶伏僧皤羅陀須若夷陀斯吽心斯耨般須斯陀慧皤嚴是蜜夷聞須須阿隸斯摩喼嚩摩須若隸斯斯叻諦斯訶祗陀摩須祗哆訶摩喃般囑
| |           [熊曰](http://hi.pcmoe.net/index.html)           | 192 | 2 |
熊曰：呋偶我性常誘麼非呱常沒捕蜂現麼嘶歡動達嗄眠誘樣嚁盜蜂麼呦捕圖既麼歡洞氏肉住訴呱誒達拙家嗅吃圖冬訴現盜我類歡囑咯眠啽呦和萌果嚁噗嘿發呱寶歡會更嗚嗷啽笨森山嗒呦人咬嘿家註嘶囑破噤蜜嘍唬嚄歡更啽我囑堅非樣嗷襲唬更嗥拙吖襲肉啽嗥蜂嗷爾常噤擊你麼達性咬冬取噔啽非肉象眠嗅盜圖氏嘿常取現嘍喜啽麼誘呦喜吃堅住嘍盜意象襲呆非唬蜜啽嘍呱嗷呦噗森咯噗雜誘呱樣蜜破有動爾果哮物嘍哈盜偶萌咯咬襲性拙歡襲噤蜜嚄
| |           [兽音](http://hi.pcmoe.net/roar.html)            | 623 | 6.48
| `~呜嗷啊呜呜呜嗷呜嗷呜啊啊啊嗷嗷嗷~嗷啊嗷啊呜~呜呜呜~嗷~呜嗷嗷~~嗷嗷嗷呜啊嗷啊~呜嗷呜呜~呜啊~啊嗷啊呜嗷呜~~~嗷~呜呜呜~~嗷嗷嗷呜啊呜呜啊呜嗷呜呜啊呜嗷呜啊嗷啊呜~嗷啊啊~嗷~呜嗷嗷~啊嗷嗷嗷呜啊呜嗷~呜嗷呜呜~嗷啊嗷啊嗷啊呜嗷嗷啊呜~嗷~呜嗷嗷~呜嗷嗷嗷呜啊嗷呜呜呜嗷呜呜~嗷啊嗷啊嗷啊呜嗷嗷呜嗷~嗷~呜呜嗷嗷~嗷嗷嗷呜啊呜啊嗷呜嗷呜呜啊嗷呜呜啊嗷啊呜嗷嗷~啊~嗷~呜呜嗷~啊嗷嗷嗷呜啊嗷嗷嗷呜嗷呜呜啊嗷呜啊啊嗷啊呜嗷呜~啊~嗷~呜嗷嗷~啊嗷嗷嗷呜呜呜呜嗷呜嗷呜呜~呜嗷嗷啊嗷啊呜~啊呜~~嗷~呜嗷啊啊~嗷嗷嗷呜啊呜啊~呜嗷呜呜啊啊~啊啊嗷啊呜~啊啊嗷~嗷~呜嗷啊呜啊嗷嗷嗷呜啊嗷嗷呜呜嗷呜呜啊呜啊嗷啊嗷啊呜~啊嗷啊~嗷~呜嗷呜嗷~嗷嗷嗷呜啊嗷呜啊呜嗷呜呜~嗷~呜啊嗷啊呜~呜嗷啊~嗷~呜嗷呜啊嗷嗷嗷嗷呜呜呜啊嗷呜嗷呜呜~嗷~啊啊嗷啊呜~啊呜嗷~嗷~呜嗷~啊呜嗷嗷嗷呜呜呜啊呜呜嗷呜呜~呜~~啊嗷啊呜嗷呜嗷啊~嗷~呜嗷呜~啊嗷嗷嗷呜啊呜嗷~呜嗷呜呜~呜~啊啊嗷啊呜嗷嗷~~~嗷~呜嗷呜~嗷嗷嗷嗷呜啊呜啊啊呜嗷呜呜~呜~嗷啊嗷啊呜嗷嗷嗷嗷~嗷~呜嗷呜呜嗷嗷嗷嗷呜呜嗷嗷~啊啊啊嗷嗷~呜~~呜~嗷呜~嗷呜嗷~呜呜~啊嗷嗷嗷嗷嗷呜呜呜~呜呜嗷呜呜啊嗷呜~啊嗷啊呜嗷呜嗷啊~嗷~呜呜呜呜嗷嗷嗷嗷呜啊嗷啊嗷呜嗷呜呜啊呜~嗷啊嗷啊呜~嗷啊~嗷呜呜啊~啊~嗷呜呜嗷~呜呜啊呜~嗷呜嗷嗷~嗷嗷~啊呜嗷呜呜嗷嗷嗷呜~啊呜啊呜嗷呜呜~嗷~呜~嗷啊呜啊呜呜啊呜呜~啊啊~~~~啊嗷呜~嗷~啊~~呜~呜~啊呜嗷呜啊呜嗷~嗷嗷~呜~~啊嗷~~~嗷嗷~嗷~啊~呜~呜啊嗷~呜啊~啊啊啊~嗷啊呜嗷呜啊嗷啊呜~啊~嗷~嗷~呜呜呜~~嗷嗷嗷呜啊呜嗷~嗷~呜嗷呜啊嗷~~啊~呜呜啊啊嗷嗷嗷~嗷嗷~啊~呜啊呜~嗷呜呜~啊啊啊啊嗷呜嗷嗷~嗷啊嗷呜呜啊啊嗷呜嗷呜~嗷呜呜啊`
| | Whisperer | 63 | 0.65 | 低语:曰生遮心戏药念涅闍孤求秘三卢殊虚广根闍捐害尼纷阿老妙创奉安福尸吼焰迦通须遮心惊金根灭来令祖诵焰药秘除沙及刚刚藐定琉能百即千卢先 |

### 阿里云分享

| 版本 | 长度 | 比率 | 文本 | | :--------------------------------------------------------: | :--: | :
----------------------------------------------------------: |
------------------------------------------------------------ | | 源文本 | 100 | 1 |
“「xxx.txt」https://www.aliyundrive.com/s/5TDHtNd8YTd 点击链接保存，或者复制本段内容，打开「阿里云盘」APP ，无需下载极速在线查看，视频原画倍速播放。” (同样无效) |
| [原版佛曰](https://www.keyfc.net/bbs/tools/tudoucode.aspx) | 319 | 3.19 |
佛曰：缽夢俱地逝奢藝梵特俱參度皤孕以呼梵究除怯咒奢蘇皤醯諳無哆僧罰特波梵是麼逝苦罰能得遠舍諳瑟諳大。缽耶舍罰參缽跋摩罰阿呐摩梵多冥迦俱羅哆醯參罰離闍怯死罰智冥地謹怯逝哆謹侄切呐尼上耶摩皤那哆等利爍缽豆彌罰倒得俱密缽咒藐梵礙密皤摩沙冥寫皤伽缽曳奢耶道舍爍呐薩罰倒尼侄豆特諳遮羅奢諦諳大得冥地陀顛有除呐槃罰陀呼盡亦奢波盡有三參訶謹梵婆醯參即梵故諳諦至皤爍豆一豆佛明缽曳缽婆梵僧侄摩冥麼孕俱除皤槃怖俱地罰瑟輸奢亦實皤曰怯逝曰奢薩怯藐怯波遮冥究哆勝是礙羯醯俱滅有梵盧呐竟瑟吉哆耨梵夜冥跋皤彌冥穆漫侄數冥滅哆礙想奢無三奢呼地恐穆罰僧皤伊怯伽帝老都哆者呐羅冥藐彌缽究朋諳菩故楞俱遮怖曰梵瑟罰闍怯。帝藐以缽羅若奢帝真竟羅藝俱遮缽娑俱諦奢度缽他哆心怯一輸怯盧舍俱耶
| |        [新版佛曰](http://hi.pcmoe.net/buddha.html)         | 290 | 2.90 |
新佛曰：夷梵般嘇嘚梵聞劫塞念彌婆是梵菩所兜宣咤摩咤兜劫婆莊梵寂修聞是波心僧阿諦婆梵咒叻耨咤降摩須嚤兜耨伏尊念尊宣訶阿莊念是彌囉塞塞夷喃羅嘚叻伏嘇缽須阿梵迦闍陀宣空蜜吽願我嚤如吶莊念哆陀眾亦慧梵婆摩眾尊闍諦摩般我空夷色迦色空吽陀訶兜塞慧訶咤阿缽吶劫訶喼兜僧梵須嚴夷訶願伏色耨咒吶宣聞闍空薩願所般即念迦耨摩婆波寂伏婆須劫咒梵色寂所陀須咒咒劫塞若吶嚩隸訶願訶薩叻伏婆伏叻是念般嚩阿尊吶阿嚴蜜祗嘚梵慧嚴亦願咒嘚僧僧願阿諦吶隸宣斯嘇波亦須伏喼嚤修咒空祗吶願若修阿是夷吽喃劫空婆喃缽伏嚤心叻哆色耨梵喃所羅聞是伏尊夷吶祗我修劫波菩喃闍如吶嚤念喼嘚降修隸如嘇喃願亦梵我祗愍薩是梵斯若叻修梵叻宣嚩薩嘚尊蜜囑
| |           [熊曰](http://hi.pcmoe.net/index.html)           | 246 | 2.46 |
熊曰：呋唬堅啽動嘿非唬誒住呱常嗄既溫家捕噔偶有寶常破嗚樣既非怎氏肉常喜嗄和家咯破嗡嚁誘歡有非告樣非笨噔森嚄住森洞我噔嘍沒呆嗅吃咯唬麼擊誘襲偶誘性發嚄山和蜜喜偶咯眠樣捕堅嘶溫出哮拙現雜和寶歡肉寶圖類喜溫囑嗒啽更會嚁哮動食性雜嗚咯眠嚄森物沒歡嘶爾蜜噔註家捕取怎呆告怎啽呦偶類取嗷現發取咬非更你蜜嘿哈和擊果呦破嗡常註達唬吖怎樣眠啽人森擊魚性哞呦噗盜象嗥既會噤象人唬註覺誘我嗚蜂類哮嘿象我氏發擊呆森麼嘍盜喜洞咯哮會呱告發啽達唬常性覺啽歡咬達性沒常破嘍森唬偶嗅人氏捕噔眠意果常嗚寶森氏圖嗒人象溫食拙嘶襲非溫魚
| |           [兽音](http://hi.pcmoe.net/roar.html)            | 630 | 6.30
| `~呜嗷嗷~嗷呜嗷啊~~呜嗷呜呜~呜~~啊嗷啊呜嗷呜嗷~~嗷~呜呜呜呜~嗷嗷嗷呜呜嗷嗷呜呜嗷呜呜~呜啊~啊嗷啊呜嗷呜嗷~~嗷~呜呜呜嗷~嗷~嗷呜嗷啊嗷嗷呜嗷呜呜~嗷~~啊嗷啊呜嗷呜~~~嗷~呜呜呜嗷~嗷嗷嗷呜啊呜嗷~呜嗷呜呜~呜啊啊啊嗷啊呜~呜呜呜~嗷~呜嗷嗷~啊嗷嗷嗷呜呜嗷嗷啊呜嗷呜呜~呜~啊啊嗷啊呜嗷呜嗷啊~嗷~呜呜呜呜啊嗷嗷嗷呜呜嗷嗷呜呜嗷呜呜~嗷啊嗷啊嗷啊呜嗷嗷呜~~嗷~呜呜嗷啊嗷嗷嗷嗷呜啊呜~嗷呜嗷呜呜~呜~嗷啊嗷啊呜嗷嗷啊呜~嗷~呜呜嗷嗷~嗷嗷嗷呜啊呜呜呜呜嗷呜呜~嗷嗷嗷啊嗷啊呜嗷呜嗷呜~嗷~呜呜嗷呜嗷嗷嗷嗷呜呜嗷嗷呜呜嗷呜呜~嗷啊啊啊嗷啊呜嗷嗷啊啊~嗷~呜呜嗷~嗷嗷嗷嗷呜呜嗷嗷啊呜嗷呜呜~呜啊啊啊嗷啊呜~嗷啊啊~嗷~呜嗷呜呜嗷嗷嗷嗷呜呜~呜~呜嗷呜呜啊啊啊~啊嗷啊呜~啊嗷~~嗷~呜呜呜嗷~嗷嗷嗷呜呜啊嗷呜呜嗷呜呜~嗷啊~啊嗷啊呜~呜嗷~~嗷~呜嗷~啊嗷嗷嗷嗷呜呜~呜~呜嗷呜呜~嗷啊~啊嗷啊呜~嗷啊~嗷~~呜啊呜啊嗷呜呜嗷啊嗷呜~啊~呜啊呜呜呜呜呜~啊~嗷呜嗷嗷嗷嗷嗷~嗷啊~~嗷呜呜~嗷呜~啊~嗷~呜嗷呜啊嗷~~啊啊~啊~嗷呜呜嗷~呜~啊呜嗷呜呜啊啊嗷啊嗷嗷啊呜呜~啊呜~呜~啊嗷嗷~嗷呜~嗷啊啊嗷啊呜呜嗷呜呜嗷啊啊啊啊嗷啊呜嗷嗷嗷呜嗷嗷呜~啊嗷啊啊呜~嗷啊~~嗷~嗷啊呜呜嗷嗷嗷啊嗷~呜~呜呜呜啊嗷~嗷呜~~~呜啊啊呜呜~啊啊啊啊~呜嗷~~啊~呜嗷啊~啊~嗷~~~啊~啊呜啊啊啊嗷~嗷~呜嗷啊嗷嗷嗷嗷嗷呜呜~嗷~呜嗷呜呜啊~呜~啊嗷啊呜~嗷啊~啊~~嗷~啊啊~呜啊呜啊嗷嗷嗷~~呜~嗷呜啊呜~~嗷呜~啊啊呜啊呜嗷~嗷呜呜~嗷呜啊啊嗷啊啊呜嗷~呜呜呜呜~呜啊~呜嗷嗷~嗷嗷~嗷~啊~啊呜~啊呜啊啊嗷嗷嗷啊嗷啊~~嗷呜啊嗷啊呜~啊嗷啊啊呜~呜嗷呜啊啊啊呜呜啊呜啊呜啊~呜嗷啊呜啊嗷~~呜啊~~~啊~呜呜啊嗷呜~呜~啊~嗷啊呜嗷呜嗷~嗷啊啊啊啊呜嗷嗷呜嗷~啊~啊~呜啊呜~~~呜~啊嗷呜啊`
| | Whisperer | 36 | 0.36 | 低语:凉众定消勒艺老礼游安怜实茶竟界于涅积栗幽惊僧粟亿写德惊闍央界逝稳藐迦殿即 |

参考项目:

1. 感谢 [TudouCode](https://github.com/lersh/TudouCode) 
   1. 参考了他的字典
   2. 保留他的加密KEY, 向他致敬

