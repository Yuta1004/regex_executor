use std::collections::{ HashSet, HashMap };

/// # NFA
/// ## members
/// - start: i32 => 開始状態
/// - finish: i32 =>  受理状態
/// - reserved_state: tuple(i32, i32) => 使用済み状態番号の範囲
struct NFA {
    pub start: i32,
    pub finish: i32,
    pub reserved_state: (i32, i32),
    move_table: HashMap<i32, HashMap<char, HashSet<i32>>>
}

impl NFA {
    /// # NFAのコンストラクタ
    ///
    /// ## args
    /// - init_states: Vec<i32> => 状態
    pub fn new(state_f: i32, state_t: i32) -> NFA {
        let mut move_table: HashMap<i32, HashMap<char, HashSet<i32>>> = HashMap::new();
        for state in state_f..state_t {
            move_table.insert(state, HashMap::new());
        }
        NFA { start: -1, finish: -1, reserved_state: (state_f, state_t), move_table }
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

    /// 状態S1と状態S2を文字Cで繋ぐ
    ///
    /// ## args
    /// - state_a: i32 => 状態S1
    /// - state_b: i32 => 状態S2
    /// - c: 文字C
    ///
    /// ## returns
    /// Result<(), ()>
    pub fn set_chain(&mut self, state_a: i32, state_b: i32, c: char) -> Result<(), ()> {
        if !(Self::check_state(self, state_a) && Self::check_state(self, state_b)) {
            return Err(())
        }
        if !self.move_table[&state_a].contains_key(&c) {
            self.move_table.get_mut(&state_a).unwrap()
                           .insert(c, HashSet::new());
        }
        self.move_table.get_mut(&state_a).unwrap()
                       .get_mut(&c).unwrap()
                       .insert(state_b);
        Ok(())
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
        if Self::check_state(self, state) {
            if let Some(states) = self.move_table[&state].get(&c) {
                return states.iter().cloned().collect();
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
    #[allow(unused_must_use)]
    fn test_get_chain() {
        /*
        1 -----(a)----- 2 ----(b)----- 4
         \                           /
          -----(a)----- 3 ----(a)----
        */
        let mut nfa = NFA::new(1, 4);
        nfa.set_chain(1, 2, 'a');
        nfa.set_chain(1, 3, 'a');
        nfa.set_chain(2, 4, 'b');
        nfa.set_chain(3, 4, 'a');
        assert_eq!(nfa.get_chains(1, 'b'), vec![]);
        assert_eq!(nfa.get_chains(2, 'b'), vec![4]);
        assert_eq!(nfa.get_chains(3, 'a'), vec![4]);
        let mut tmp = nfa.get_chains(1, 'a'); tmp.sort();
        assert_eq!(tmp, vec![2, 3]);
    }
}