use std::collections::HashMap;

/// # NFA
/// ## members
/// - start: i32 => 開始状態
/// - finish: i32 =>  受理状態
/// - reserved_state: tuple(i32, i32) => 使用済み状態番号の範囲
struct NFA {
    pub start: i32,
    pub finish: i32,
    pub reserved_state: (i32, i32),
    move_table: HashMap<i32, HashMap<char, Vec<i32>>>
}

impl NFA {
    /// # NFAのコンストラクタ
    ///
    /// ## args
    /// - init_states: Vec<i32> => 状態
    pub fn new(state_f: i32, state_t: i32) -> NFA {
        NFA { start: -1, finish: -1, reserved_state: (state_f, state_t), move_table: HashMap::new() }
    }

    /// # 初期状態セット
    ///
    /// ## args
    /// - state: i32 => 初期状態にセットする状態
    ///
    /// ## returns
    /// Result<(), ()>
    pub fn set_start(&mut self, state: i32) -> Result<(), ()> {
        if Self::check_state(self, state) {
            self.start = state;
            return Ok(());
        }
        Err(())
    }

    /// # 受理状態セット
    ///
    /// ## args
    /// - state: i32 => 受理状態にセットする状態
    ///
    /// ## returns
    /// Result<(), ()>
    pub fn set_finish(&mut self, state: i32) -> Result<(), ()> {
        if Self::check_state(self, state) {
            self.start = state;
            return Ok(());
        }
        Err(())
    }

    /// # 状態Sからある文字Cを通じて到達できる状態を返す
    ///
    /// ## args
    /// - state: i32 => 状態S
    /// - c: char => 文字C
    ///
    /// ## returns
    /// Vec<i32>
    pub fn get_chains(&self, state: i32, c: char) -> Vec<i32> {
        if let Some(alphabet_chains) = self.move_table.get(&state) {
            if let Some(states) = alphabet_chains.get(&c) {
                return states[0..].to_owned();
            }
        }
        Vec::new()
    }

    /// # 自分が管理する状態かどうかチェック
    fn check_state(&self, state: i32) -> bool {
        let (t, f) = self.reserved_state;
        t <= state && state <= f
    }
}

#[cfg(test)]
mod tests {
    use super::NFA;

    #[test]
    fn test_init() {
        let mut nfa = NFA::new(0, 4);
        assert_eq!(nfa.set_start(-1), Err(()));
        assert_eq!(nfa.set_start(0), Ok(()));
        assert_eq!(nfa.set_finish(4), Ok(()));
        assert_eq!(nfa.set_finish(5), Err(()));
        assert_eq!(nfa.reserved_state, (0, 4));
    }

    #[test]
    fn test_get_chain() {
        let nfa = NFA::new(0, 4);
        assert_eq!(nfa.get_chains(0, 'a'), vec![]);
    }
}