// 1. Strategy(戦略)
// 戦略のインタフェースを定義します。
// type BinaryFn<T> = Fn(T, T) -> T;
type BinaryFn<T> = dyn Fn(T, T) -> T;
struct Context<'a, T: 'a> {
    strategy: &'a BinaryFn<T>,
}

// 3. Context(状況判断)
// 状況により「ConcreteStrategy」を切替えその戦略を利用します。
// 利用するのは、あくまで「Strategy」で定義したメソッドです。
impl<'a, T> Context<'a, T> {
    fn new(f: &'a BinaryFn<T>) -> Context<'a, T>
    {
        Context {
            strategy: f,
        }
    }

    fn execute(&self, x: T, y: T) -> T
    {
        (*self.strategy)(x, y)
    }

    fn set_strategy(&mut self, f: &'a BinaryFn<T>)
    {
        self.strategy = f
    }
}


fn main() {
    // 2. ConcreteStrategyA・B(具体的戦略)の役
    // 「Strategy」が定義したインタフェースを実装します。
    // 具体的な戦略を作成します。
    let add = |x: usize, y: usize| x + y;
    let mul = |x: usize, y: usize| x * y;
    let div = |x: usize, y: usize| x / y;
    let and = |x: usize, y: usize| x & y;

    // 4. Client(利用者)
    // 「Strategy」パターンを適用したクラスを用い処理を行います。
    let mut c = Context::new(&add);

    println!("{:?}", c.execute(1, 2));

    c.set_strategy(&mul);
    println!("{:?}", c.execute(1, 2));

    c.set_strategy(&div);
    println!("{:?}", c.execute(2, 2));

    c.set_strategy(&and);
    println!("{:?}", c.execute(2, 2));
}