use rand::distr::{weighted::WeightedIndex, Distribution};
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, Write},
};

// **📌 Word構造体：単語と遷移を管理**
#[derive(Default, Clone)]
struct Word {
    word: String,
    transitions: Vec<String>,
}

impl Word {
    fn new(word: String) -> Self {
        Self {
            word,
            transitions: Vec::new(),
        }
    }

    fn add_transition(&mut self, new_transition: String) {
        self.transitions.push(new_transition);
    }
}

// **📌 MarkovChain構造体：単語と遷移を学習・生成**
#[derive(Default)]
struct MarkovChain {
    words: HashMap<String, Word>,
}

impl MarkovChain {
    // **🔍 学習**
    fn learn(&mut self, text: &str) {
        let whitespace_separated: Vec<&str> = text.split_whitespace().collect();
        let mut japanese_separated: Vec<String> = vec![];
        let mut separated: Vec<String> = vec![];

        // **📌 日本語を「漢字」「ひらがな」「カタカナ」「記号」単位で分割**
        for word in whitespace_separated {
            let tokens = separate_tokens(word);
            japanese_separated.extend(tokens);
        }

        for word in japanese_separated {
            separated.extend(chunk_string(&word, 5));
        }

        // **📌 マルコフ連鎖に単語を追加**
        for i in 0..separated.len() {
            let word_str = separated[i].clone();

            // **📌 HashMap に単語がなければ新規追加**
            self.words
                .entry(word_str.clone())
                .or_insert_with(|| Word::new(word_str.clone()));

            // **📌 遷移を追加**
            if let Some(next_word) = separated.get(i + 1) {
                self.words
                    .get_mut(&word_str)
                    .unwrap()
                    .add_transition(next_word.clone());
            }
        }
    }

    // **📝 文章を生成**
    fn generate(&self, start_word: &str, length: usize) -> String {
        let mut rng = rand::rng();
        let mut result = start_word.to_string();
        let mut current_word = start_word.to_string();

        for _ in 0..length {
            if let Some(word) = self.words.get(&current_word) {
                if !word.transitions.is_empty() {
                    // **📌 遷移の重みを計算（出現頻度に基づいて重み付け）**
                    let weights: Vec<_> = word
                        .transitions
                        .iter()
                        .map(|w| word.transitions.iter().filter(|&x| *x == *w).count())
                        .collect();

                    // **📌 WeightedIndexで重み付けしたランダム選択**
                    let dist = WeightedIndex::new(&weights).unwrap();
                    let next_word = &word.transitions[dist.sample(&mut rng)];

                    result.push_str(" ");
                    result.push_str(next_word);
                    current_word = next_word.clone().to_string();
                } else {
                    result = self.generate("。", 20);
                }
            } else {
                // **📌 現在の単語が辞書にない場合、ランダムな単語を選択**
                result = self.generate("。", 20);
            }
        }
        result
    }
}

fn chunk_string(input: &str, chunk_size: usize) -> Vec<String> {
    input
        .chars() // 文字単位で処理
        .collect::<Vec<char>>() // Vec<char>に変換
        .chunks(chunk_size) // chunk_sizeごとに区切る
        .map(|chunk| chunk.iter().collect()) // チャンクを文字列に変換
        .collect()
}

/// Separate tokens
fn separate_tokens(text: &str) -> Vec<String> {
    let re = Regex::new(r"([一-龯]+|[ぁ-ん]+|[ァ-ヴー]+|[。、a-zA-Z]+)").unwrap();
    let text: Vec<String> = re.find_iter(text).map(|m| m.as_str().to_string()).collect();
    // Then split by whitespace
    text.join(" ").split_whitespace().map(|s| s.to_string()).collect()
}

/// Main function
fn main() {
    let mut chain = MarkovChain::default();

    let text = "今日は天気がいいですね。天気が悪い日もあります。明日はどうなるでしょうか？今日はいい天気ですね。気温も温かくて過ごしやすいです。午後は少し風が強くなるかもしれません。明日はもっと晴れるといいなと思っています。あなたはどうですか？最近は忙しいですか？私も少し忙しくて、いろいろなことを考えてしまいます。でも、少し休憩を取るとリフレッシュできるので、午後はゆっくりしたいです。お昼ご飯は何を食べましたか？私はサンドイッチを食べました。簡単だけど美味しかったです。来週の予定はどうですか？私は友達と会う予定があります。楽しみです。今日は本当に暑いですね。外に出るのが少し嫌になってしまいます。でも、夏は好きだからまあいいか。
そういえば、最近見た映画がすごく面白かったんです。君も映画はよく観る方ですか？
あ、でも、天気が悪いときは、家で読書やNetflixを見たりすることが多いかな。
君は最近、何か面白いことありましたか？
最近、友達と旅行に行ったんですよ。小旅行だったけど、すごく楽しかったです。
そういえば、明日の予定は決まってるんですか？
明日は、午後から少し買い物に行こうかなと思っています。何か欲しいものがあるんです。
そうそう、先週行ったレストラン、めちゃくちゃ美味しかったんですよ！何を頼んでも絶品でした。
あ、あなたは外食ってよくしますか？
最近、運動不足で、少し体がなまってきたかも。運動する時間を作らないとですね。
昨日の夜は、久しぶりに自分で料理をしました。炒飯を作ったんですけど、美味しくできました！
それは良いですね！自分で作る料理ってなんだか心がこもってる気がしますよね。
最近、よく考えているのが、将来のこと。やりたいことがたくさんあるけど、どれを選ぶか迷ってるんです。
あ、それは分かります。私はやりたいことがいっぱいあって、時間が足りないって感じです。
ところで、週末はどう過ごす予定ですか？私は友達とアウトドアに出かけるつもりです。
あ、それいいですね！私も最近、ハイキングがしたいなと思ってたんです。自然の中でリフレッシュしたいです。
実は来月、海外旅行に行く予定なんです。ずっと行きたかった場所なので、すごく楽しみです。
海外旅行いいですね！どこに行くんですか？私はまだ行ったことがないところが多いので、羨ましいです。
そういえば、最近何かハマっている趣味がありますか？私は最近、絵を描くことにはまっているんです。
絵を描くのは素晴らしい趣味ですね！私は最近、音楽に少し興味を持ち始めて、楽器を少し練習してます。
それにしても、今年の夏は本当に暑いですね。どうしても冷たいものばかり飲んじゃいます。
暑い日はやっぱりアイスが恋しくなりますよね。特に抹茶アイスが好きで、よく食べてます。
あ、アイスといえば、最近食べたフルーツタルトがめちゃくちゃ美味しかったんです。おすすめです！
最近の天気、どうだった？
昨日は少し寒かったけど、今日は晴れて気持ちがいいね。
うん、晴れてる日はなんだか元気が出るよね。

この前、映画見に行ったんだけど、すごく面白かったよ！
どんな映画だったの？私は最近映画館に行ってないなぁ。
『インセプション』みたいな頭を使う映画だよ。君は映画館行くとき、ポップコーン派？

ポップコーン派かな。塩味のが一番だね。でも、最近は映画館の料金が高くてあまり行かなくなったな。
そうだよね、映画料金が高いから家で見ることが多くなっちゃう。家でも十分楽しめるしね。

それに、最近はNetflixとかHuluで見れるから便利だよね。おすすめの映画ある？
最近見た映画だと『アバター』が良かったよ。視覚的にもすごく迫力があって、感動した！

それは面白そうだね！今度見てみようかな。最近は忙しくて、あまり映画を見る時間が取れてないんだよね。
わかる。忙しいと、リラックスする時間がなかなか取れないよね。でも、ちょっとした時間に映画やドラマを見るとリフレッシュできるよ。

それ、いいアイディアだね！リラックスしたい時におすすめのドラマとかある？
『フレンズ』は何度見ても面白いし、気軽に楽しめるからおすすめだよ。明るい雰囲気で癒されるし。

それは確かに癒されそうだね！私も久しぶりに見たくなった。君は最近、どんな趣味を楽しんでるの？
最近は写真を撮るのが好きで、週末にはよく散歩がてらカメラを持って外に出るよ。風景とか、街の雰囲気を撮るのが楽しいんだ。

素敵だね！私もカメラを持って外に出るのが好きだけど、最近は忙しくてなかなかできてないなぁ。
そうなんだ。忙しいと趣味の時間も取りづらいよね。でも、少しでも時間を見つけてやると気分がすごく変わるよ。

そうだよね、やっぱり自分の時間も大切にしないと。最近、体調はどう？
元気だよ！ちょっとした運動をしてるから、体調も良くなったし、心も落ち着いてるよ。

運動かぁ、良いね！私は最近、ウォーキングを始めたんだ。長時間歩くとすっきりするし、気分転換になるよ。
ウォーキングは体にも優しいし、リラックスできるからいいね。私ももっと歩くようにしようかな。

最近、健康に気を使ってるのかな？
そうだね、少し前から健康に気をつけるようになって。食事や運動も大事だけど、睡眠も大切だから、よく寝るようにしてるよ。

それ、すごく大事だよね。睡眠不足だと一日がダルく感じるもんね。君は何か健康的な食事を作ったりするの？
最近はサラダをよく作るよ。アボカドやトマトをたくさん入れて、オリーブオイルと塩で味付けするだけでおいしいんだ。

それ、ヘルシーでおいしそう！私は最近、野菜をたくさん取るようにしてるんだ。スムージーにして毎朝飲んでるよ。
それ、良い習慣だね！私もスムージーを試してみようかな。野菜と果物を一緒に取れるから栄養満点だしね。

じゃあ、今度一緒に作ってみる？
いいね！一緒に作ったら楽しそうだし、新しいレシピも試せそうだね。

そういえば、最近読んだ本とかはある？
最近読んだ本は『ノルウェイの森』だよ。村上春樹の作品は深くて考えさせられるんだよね。
村上春樹の本って、独特な雰囲気があって面白いよね。私は『海辺のカフカ』が好きだなぁ。

『海辺のカフカ』もいいよね！夢と現実が交錯する感じが好き。物語の進行がどうなるか気になっちゃう。
わかる！あの不思議な感じがたまらないんだよね。村上春樹の本を読むと、少し現実を忘れて違う世界に浸れる気がする。

そういえば、君は普段音楽は何を聴いてるの？
最近はジャズにハマってるんだ。リラックスしたいときにジャズを聴くと、気分が落ち着くんだよね。

ジャズか、いいね！私は最近、クラシック音楽をよく聴いてるよ。ベートーヴェンとかモーツァルトの曲が心に響くんだよね。
クラシックは本当に心に染みるよね。特に、ゆっくりとした曲を聴くと、リラックスできて疲れが取れる気がする。

音楽を聴きながらリラックスする時間って、本当に大切だよね。君は他にどんな趣味があるの？
私は絵を描くのが好きだよ。風景画を描いたり、ポートレートを描いたりしてるんだ。色を使って表現するのが楽しいんだよね。

素敵だね！私も昔、絵を描くのが好きだったけど、最近はあまり描けてないなぁ。君はどんな道具で絵を描くの？
私は主にアクリル絵の具を使ってるよ。乾くのが早いし、色の発色が鮮やかだから好きなんだ。

アクリル絵の具は扱いやすいよね！私は水彩が好きだったけど、少し難しいなと思って。やっぱり練習が必要だよね。
水彩画は難しいけど、その透明感がすごくきれいなんだよね。君もまた挑戦してみたらどう？

そうだね、たまには再挑戦してみようかな。絵を描くとき、集中できて気持ちも落ち着くから、時間を見つけてやってみるよ。
うん、それがいいと思う！絵を描く時間って、すごく自分と向き合う時間になって心がリフレッシュされるよね。

それにしても、最近忙しくてあまり自分の時間が取れてないんだよね。
忙しいと本当に時間が足りなく感じるよね。でも、少しの時間でも自分の好きなことをすることが大事だよ。

確かに、少しでもリフレッシュできる時間があればいいよね。君は最近、リフレッシュできた瞬間ってあった？
この前、友達と公園に行ったんだけど、外でのんびり過ごす時間がすごく気持ちよかったんだ。

それは素敵だね！自然の中で過ごすと、心が落ち着くよね。私もたまには外で散歩したりして、自然を感じたくなるよ。
外に出ると気分も変わるし、リラックスできるもんね。自然の中にいると、どんな小さなことにも感謝できる気がするよ。

感謝したこととかある？
最近、友達から手作りのクッキーをもらったんだけど、その気持ちがすごく嬉しくて感謝したなぁ。こんな些細なことでも、思いやりって大事だなって感じたよ。

それは素敵だね！そういう小さな優しさが心に残るよね。私は最近、家族が手伝ってくれて、すごく感謝してる。忙しい時に助けてもらうと、ありがたさがしみるよね。
本当にそうだよね。家族のサポートって本当に大きいよね。何気ない言葉や行動が、一番ありがたく感じる瞬間だな。

あ、それで思い出したけど、最近何か面白い映画を観た？
最近は『ジョーカー』を観たよ。すごく考えさせられる映画だった。キャラクターの心理描写が深くて、観た後に余韻が残ったなぁ。

『ジョーカー』は確かに印象に残る映画だよね。暗いけど、その中に人間の複雑さが表現されていて、観ていてドキドキする。君は映画を観るとき、どんなジャンルが好き？
私はサスペンスやミステリーが好きだな。謎が解ける瞬間にスッキリするんだよね。でも、たまには感動的な映画も観たくなる。

サスペンスやミステリーは本当にハラハラするよね。解決編に向けて、どんどん引き込まれていく感じがたまらない。感動的な映画も心が温かくなるよね。
映画って、その時の気分によって観るジャンルが変わるから面白いよね。君はどんな映画で感動したことがある？

『グリーンブック』がすごく感動したよ。人種差別や偏見について考えさせられる内容だったけど、最終的には希望を感じさせてくれる。あの映画を観た後、心が温かくなったんだ。
『グリーンブック』、いい映画だよね。あのストーリーは感動的で、人との絆の大切さを教えてくれる。観て良かったと思える作品だよね。

私もそんな映画を観て、自分がどう生きていくか、少し考えることがある。映画って本当に色々なことを教えてくれるよね。
本当にそうだよね。映画って、ただ楽しむだけじゃなくて、人生や人間についての深いメッセージを感じることができるから、毎回観た後に自分に何か残る気がする。

それにしても、最近天気が良い日が続いてるね。君は晴れの日は外で何かするのが好き？
晴れた日は、散歩に出かけるのが好きだな。陽射しが気持ちよくて、何だか心も軽くなる気がする。あと、友達とピクニックにも行くことが多いよ。

ピクニックって、いいよね！自然の中でのんびり過ごすのが楽しいし、美味しいものを食べながらの会話も最高だよね。最近はどうだった？
この前、友達と近くの公園でピクニックしたんだけど、天気も良くて、本当に楽しかった！手作りのサンドイッチを持って行ったんだけど、みんなで食べるとすごく美味しく感じるんだよね。

それは楽しそうだね！手作りのものって、何か特別感があって、さらに美味しく感じるよね。私もそんな時間を過ごしたくなったなぁ。
本当にそうだよね。自然の中で、みんなで食べたり話したりする時間って、すごくリフレッシュできるんだよね。

最近、自然の中で過ごすことが少なくなっているから、また少し時間を作って外に出てみようかな。
外に出ると、普段見逃している小さな美しさにも気づけるし、リフレッシュできるよね。君も少しでも時間を作って、自然を感じる時間を持つといいかも。

Hello! How are you doing today? 😊 I hope you're having a great day! 🌟
最近どうですか？元気ですか？😄 素敵な一日を過ごしていますように！🌸
I just finished reading a fantastic book. 📚 Have you read anything interesting lately?
最近、面白い本を読み終えました。📖 あなたは最近何か面白い本を読みましたか？
The weather has been so nice lately! ☀️ Perfect for a walk in the park. 🚶‍♂️
最近、天気がとても良いですね！☀️ 公園を散歩するのにぴったりです。🌳
I love listening to music while working. 🎧 It helps me focus. How about you?
仕事中に音楽を聴くのが好きです。🎶 集中するのに役立ちます。あなたはどうですか？
最近、友達とカフェに行きました。☕ 美味しいケーキを食べて、とても楽しかったです！🍰
I went to a new restaurant last weekend. 🍴 The food was amazing! 😋
先週末、新しいレストランに行きました。🍽️ 料理がとても美味しかったです！😋
Do you enjoy traveling? ✈️ I recently visited a beautiful place and took lots of photos. 📸
旅行は好きですか？✈️ 最近、美しい場所を訪れて、たくさん写真を撮りました。📷
Let's plan something fun for the weekend! 🎉 Maybe a picnic or a movie night? 🎥
週末に何か楽しいことを計画しましょう！🎊 ピクニックや映画鑑賞はどうですか？🎬
";

    chain.learn(text);

    println!("🔹 Wordora Markov ChatBot 🔹");

    loop {
        print!("あなた: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let mut tokens = separate_tokens(input);
        tokens = chunk_string(tokens.join("").as_str(), 3);
        let start_word = tokens
            .first()
            .cloned()
            .unwrap_or_else(|| "".to_string());

        let response = chain.generate(&start_word, 20);
        println!("Bot: {}", response);
    }
}
