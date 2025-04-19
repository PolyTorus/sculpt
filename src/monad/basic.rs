pub trait Monad: Sized {
    /// 値の型
    type Item;

    /// モナドの文脈に値を持ち上げる (return/pure)
    fn pure(value: Self::Item) -> Self;

    /// 値をモナドを返す関数で変換する (bind)
    fn bind<B, F>(self, f: F) -> B
    where
        F: FnOnce(Self::Item) -> B,
        B: Monad;

    /// 値を関数で変換する (fmap/map)
    fn map<B, F>(self, f: F) -> Self
    where
        F: FnOnce(Self::Item) -> B,
        Self: Monad<Item = B>;

    /// 前の操作を無視して値を置き換える
    fn replace<B>(self, new_value: B) -> Self
    where 
        Self: Monad<Item = B>;
}

/// 失敗可能なモナド型の操作を定義するトレイト
pub trait MonadError: Monad {
    /// エラーの型
    type Error;

    /// エラーからモナドを作成する
    fn fail(error: Self::Error) -> Self;

    /// エラーをキャッチする
    fn catch<F>(self, f: F) -> Self
    where
        F: FnOnce(Self::Error) -> Self;

    /// 値を検証し、条件を満たさない場合はエラーを返す
    fn ensure<F, P>(self, predicate: P, error: F) -> Self
    where
        P: FnOnce(&Self::Item) -> bool,
        F: FnOnce() -> Self::Error;
}
