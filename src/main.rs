fn main() {
    let x = 12; // デフォルトでは i32
    let a = 12u8;
    let b = 4.3; // デフォルトでは f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let y = 13;
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );

    let a = make_nothing();
    let b = make_nothing2();

    match x {
        0 => {
            println!("found zero");
        }
        // 複数の値にマッチ
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // 範囲にマッチ
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // マッチした数字を変数に束縛
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // どのパターンにもマッチしない場合のデフォルトマッチが必須
        _ => {
            println!("found something else!");
        }
    }

    // 空を表示するのは難しいので、
    // a と b のデバッグ文字列を表示
    println!("a の値: {:?}", a);
    println!("b の値: {:?}", b);
    println!("{}", add(x, y));

    example();
    init_sea_creature();
    option_enum();
    test_results_for_somethings();
}

fn add(x: i32, y: i32) -> i32 {
    let answer: i32 = x + y;
    answer
}

fn make_nothing() -> () {
    return ();
}

// 戻り値は () と推論
fn make_nothing2() {
    // この関数は戻り値が指定されないため () を返す
}

fn example() -> i32 {
    let x = 42;
    // Rust の三項式
    let v = if x < 42 { -1 } else { 1 };
    println!("if より: {}", v);

    let food = "ハンバーガー";
    let result = match food {
        "ホットドッグ" => "ホットドッグです",
        // 単一の式で値を返す場合、中括弧は省略可能
        _ => "ホットドッグではありません",
    };
    println!("食品の識別: {}", result);

    let v = {
        // ブロックのスコープは関数のスコープから分離されている
        let a = 1;
        let b = 2;
        a + b
    };
    println!("ブロックより: {}", v);

    // Rust で関数の最後から値を返す慣用的な方法
    v + 4
}

struct SeaCreature {
    // String は構造体である。
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn init_sea_creature() {
        // SeaCreatureのデータはスタックに入ります。
        let ferris = SeaCreature {
            // String構造体もスタックに入りますが、
            // ヒープに入るデータの参照アドレスが一つ入ります。
            animal_type: String::from("crab"),
            name: String::from("Ferris"),
            arms: 2,
            legs: 4,
            weapon: String::from("claw"),
        };
    
        let sarah = SeaCreature {
            animal_type: String::from("octopus"),
            name: String::from("Sarah"),
            arms: 8,
            legs: 0,
            weapon: String::from("none"),
        };
        
        println!(
            "{} is a {}. They have {} arms, {} legs, and a {} weapon",
            ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
        );
        println!(
            "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
            sarah.name, sarah.animal_type, sarah.arms, sarah.legs
        );
}

struct BagOfHolding<T> {
    // パラメータ T を渡すことが可能
    item: Option<T>,
}


fn option_enum() {
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_none() {
        println!("バッグには何もない！")
    } else {
        println!("バッグには何かある！")
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42) };

    if i32_bag.item.is_some() {
        println!("バッグには何かある！")
    } else {
        println!("バッグには何もない！")
    }

    // match は Option をエレガントに分解して、
    // すべてのケースが処理されることを保証できます！
    match i32_bag.item {
        Some(v) => println!("バッグに {} を発見！", v),
        None => println!("何も見付からなかった"),
    }
}

fn do_something_that_might_fail(i:i32) -> Result<f32,String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

fn test_results_for_somethings() {
    let result = do_something_that_might_fail(12);

    // match は Result をエレガントに分解して、
    // すべてのケースが処理されることを保証できます！
    match result {
        Ok(v) => println!("発見 {}", v),
        Err(e) => println!("Error: {}",e),
    }
}