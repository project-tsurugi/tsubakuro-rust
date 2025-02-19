# tsubakuro-rust-core ResponseBox

tsubakuro-rust-coreの `session::wire::ResponseBox` の実装についての覚書。

## ResponseBox

ResponseBoxは、Tsurugi DBサーバーへのリクエストとレスポンスを紐づける為のオブジェクト。

ResponseBoxインスタンスは1つのWireにつき1つ保持され、そのライフサイクル（生存期間）はWire（Session）と同じ。

リクエストに対するレスポンスは非同期で返ってくるので、返ってきたレスポンスがどのリクエストに対するものなのかを紐づける必要がある。

1. サーバーにリクエストを送信する際に、slot番号という番号を採番し、リクエストに含める。
2. サーバーから返ってくるレスポンスには、対応するリクエストのslot番号が含まれる。

ResponseBoxでは、slot番号毎にリクエストを保持しておくことにより、返ってきたレスポンスのslot番号から、対応するリクエストを探すことが出来る。
このため、slot番号はResponseBox内で一意である必要がある。

slot番号は、レスポンスが返ってきたら解放され、次のリクエストで再使用される。  
逆に言うと、レスポンスが返ってくるまで、そのslot番号は解放されない。

slot番号はSlotEntryという構造体で管理する。  
使用中のslot番号はSlotEntryHandleという構造体で管理する。

ResponseBoxは、未使用のSlotEntryを保持する為のキューを持つ。  
また、SlotEntryHandle（リクエストに使われてレスポンスを待っているslot番号）は、slot番号をキーとするマップ（実体はVec）で管理している。

なお、select文の実行結果であるQueryResultのデータは、ResponseBoxの処理対象外である。
（「リクエスト-レスポンス」でない方式で受信される）

## SlotEntry

SlotEntryは、slot番号を意味するオブジェクト。

（当初はslot番号以外の情報も保持する想定で構造体にしたが、現在はslot番号のみしか保持していないので、構造体である必要は無い）

slot番号はリクエストを送信する際に使用・束縛され、レスポンスが返ってきたら解放される。

slot番号は使い回されるので、未使用のslot番号はResponseBoxのキューで管理され、リクエストを送信する際にキューからslot番号（SlotEntry）が取得される。  
キューが空だった場合は、新しいslot番号が採番される（新しいSlotEntryインスタンスが生成される）。

SlotEntryインスタンスは、ResponseBoxが破棄されるまで存在し続ける。

## SlotEntryHandle

SlotEntryHandleは、リクエストに使用したslot番号を管理するオブジェクト。  
内部でSlotEntryを保持する。

1. SlotEntryHandleインスタンスは、リクエストを送信する際にSlotEntryを受け取って生成される。同時に、ResponseBox内の待機マップ（レスポンス待ちのslot番号を管理するマップ）に保持される。
2. SlotEntryHandleインスタンスは、ユーザーに（Job内のデータとして）渡されることがある。
3. SlotEntryHandleインスタンスが破棄されるのは、レスポンスを受信し、かつ、ユーザーが保持をやめたとき。

つまり、SlotEntryHandleインスタンスは、ResponseBoxとユーザーの2か所（ユーザーが複製すれば2か所以上）で保持される。
このため、SlotEntryHandleは `Arc<SlotEntryHandle>` で扱う。

### レスポンスデータの扱い

SlotEntryHandleは、レスポンスのデータそのものも保持する。

1. レスポンスが返ってきたら、ResponseBoxはSlotEntryHandleを待機マップから削除し、そのSlotEntryHandleにレスポンスデータを渡す。
2. レスポンスデータを取得したいオブジェクト（Job）はSlotEntryHandleを保持しているので、（レスポンスが返ってきていれば）SlotEntryHandleからレスポンスデータを取得することが出来る。

### SlotEntryHandleの破棄

レスポンスが返ってきたslot番号は解放されて再使用される。が、レスポンスが返ってきた時点（直後）ではslot番号は解放できない。  
もしユーザーがslot番号（SlotEntryHandle）を保持し続けていたら、本来のリクエストに対するもの以外の（再使用されたslot番号の）レスポンスも受け取ってしまう危険がある為。

したがって、slot番号を解放できるのは、レスポンスが返ってきてResponseBoxの待機マップからSlotEntryHandleが削除され、かつ、ユーザーがSlotEntryHandleの保持（Jobの保持）をやめたときである。

この2つのどちらが先になるかは決定できない。  
しかし `Arc` の仕様では、（順序は問わず）全ての参照が無くなればデストラクター（dropメソッド）が呼ばれる。  
これを利用し、SlotEntryHandleではdropメソッド内でslot番号を解放する（保持していたSlotEntryをResponseBoxのキューに戻す）ようにした。

（このような使い方をする為に、slot番号を扱う構造体はSlotEntryとSlotEntryHandleの2つに分離した。インスタンスのライフサイクル（生存期間）は、SlotEntryはResponseBoxが生きている間はずっと生きているのに対し、SlotEntryHandleは1回のリクエストが処理される間である）

ただしこの方式では、ユーザーがSlotEntryHandle（Job）を保持し続けているといつまでも破棄されず、slot番号が再使用されなくて新しいslot番号がどんどん採番され、上限を超えて枯渇する危険性はある。
（slot番号は2バイト整数なので、通常の使い方であれば枯渇することは無いと思われる）