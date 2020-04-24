use std::collections::{ HashSet, HashMap };

/// # NFAのエラー
#[derive(Debug, PartialEq)]
enum NFAError {
    NonReservedState,
}

/// # NFA
/// ## members
/// - start: i32 => 開始状態
/// - finish: i32 =>  受理状態
/// - reserved_state: tuple(i32, i32) => 使用済み状態番号の範囲
struct NFA {
    pub start: i32,
    pub finish: i32,
    pub reserved_state: (i32, i32),
    move_table: HashMap<i32, HashMap<char, HashSet<i32>>>,
    epsilon_chain: HashMap<i32, (HashSet<i32>, HashSet<i32>)>  // (forward, back)
}

impl NFA {
    /// # NFAのコンストラクタ
    ///
    /// ## args
    /// - init_states: Vec<i32> => 状態
    pub fn new(state_f: i32, state_t: i32) -> NFA {
        let mut move_table: HashMap<i32, HashMap<char, HashSet<i32>>> = HashMap::new();
        let mut epsilon_chain: HashMap<i32, (HashSet<i32>, HashSet<i32>)> = HashMap::new();
        for state in state_f..=state_t {
            move_table.insert(state, HashMap::new());
            epsilon_chain.insert(state, (HashSet::new(), HashSet::new())); // (f, b)
        }
        NFA { start: -1, finish: -1, reserved_state: (state_f, state_t), move_table, epsilon_chain }
    }

    /// # 初期状態セット
    ///
    /// ## args
    /// - state: i32 => 初期状態にセットする状態
    ///
    /// ## returns
    /// Result<(), ()>
    pub fn set_start(&mut self, state: i32) -> Result<(), NFAError> {
        if Self::check_state(self, &state) {
            self.start = state;
            return Ok(());
        }
        Err(NFAError::NonReservedState)
    }

    /// # 受理状態セット
    ///
    /// ## args
    /// - state: i32 => 受理状態にセットする状態
    ///
    /// ## returns
    /// Result<(), ()>
    pub fn set_finish(&mut self, state: i32) -> Result<(), NFAError> {
        if Self::check_state(self, &state) {
            self.start = state;
            return Ok(());
        }
        Err(NFAError::NonReservedState)
    }

    /// # 状態S1と状態S2を文字Cで繋ぐ
    ///
    /// # note
    /// - ε = '@'
    ///
    /// ## args
    /// - state_a: i32 => 状態S1
    /// - state_b: i32 => 状態S2
    /// - c: 文字C
    ///
    /// ## returns
    /// Result<(), ()>
    pub fn set_chain(&mut self, state_a: i32, state_b: i32, c: char) -> Result<(), NFAError> {
        if !(Self::check_state(self, &state_a) && Self::check_state(self, &state_b)) {
            return Err(NFAError::NonReservedState)
        }
        // 遷移表更新
        if !self.move_table[&state_a].contains_key(&c) {
            self.move_table.get_mut(&state_a).unwrap()
                           .insert(c, HashSet::new());
        }
        self.move_table.get_mut(&state_a).unwrap()
                       .get_mut(&c).unwrap()
                       .insert(state_b);
        // ε-chain更新
        if c == '@' {
            self.epsilon_chain.get_mut(&state_a).unwrap().0.insert(state_b);
            self.epsilon_chain.get_mut(&state_b).unwrap().1.insert(state_a);
            let mut b_state_stack = vec![state_a];
            let mut f_states: HashSet<i32> = HashSet::new();
            f_states.extend(self.epsilon_chain[&state_a].0.iter());
            f_states.extend(self.epsilon_chain[&state_b].0.iter());
            loop {
                if b_state_stack.len() == 0 {
                    break;
                }
                let state = b_state_stack.pop().unwrap();
                let mut b_states: Vec<i32> = self.epsilon_chain[&state].1.iter().cloned().collect();
                self.epsilon_chain.get_mut(&state).unwrap().0.extend(&f_states);
                b_state_stack.append(&mut b_states);
            }
        }
        Ok(())
    }

    /// # 状態Sからある文字Cを通じて到達できる状態を返す
    fn get_closure(&self, state: &i32, c: &char) -> HashSet<i32> {
        if Self::check_state(self, &state) {
            if let Some(states) = self.move_table[&state].get(&c) {
                return states.clone();
            }
        }
        HashSet::new()
    }

    /// # 状態集合Sからε-遷移のみで到達可能時な状態一覧を返す
    fn get_epsilon_closure(&self, states: &HashSet<i32>) -> HashSet<i32> {
        let mut reachable_states: HashSet<i32> = HashSet::new();
        for state in states {
            if Self::check_state(self, state) {
                reachable_states.extend(&self.epsilon_chain[state].0);
            }
        }
        reachable_states
    }

    /// # 自分が管理する状態かどうかチェック
    fn check_state(&self, state: &i32) -> bool {
        let (t, f) = self.reserved_state;
        t <= *state && *state <= f
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use rand::seq::SliceRandom;
    use super::{ NFA, NFAError };

    #[test]
    fn test_init() {
        let mut nfa = NFA::new(0, 4);
        assert_eq!(nfa.set_start(0), Ok(()));
        assert_eq!(nfa.set_finish(4), Ok(()));
        assert_eq!(nfa.set_start(-1), Err(NFAError::NonReservedState));
        assert_eq!(nfa.set_finish(5), Err(NFAError::NonReservedState));
        assert_eq!(nfa.reserved_state, (0, 4));
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_get_closure() {
        let mut nfa = NFA::new(1, 4);
        nfa.set_chain(1, 2, 'a');
        nfa.set_chain(1, 3, 'a');
        nfa.set_chain(2, 4, 'b');
        nfa.set_chain(3, 4, 'a');
        nfa.set_chain(1, 4, '@');
        nfa.set_start(1);
        nfa.set_finish(4);
        assert_eq!(nfa.get_closure(&1, &'b').iter().cloned().collect::<Vec<i32>>(), vec![]);
        assert_eq!(nfa.get_closure(&2, &'b').iter().cloned().collect::<Vec<i32>>(), vec![4]);
        assert_eq!(nfa.get_closure(&3, &'a').iter().cloned().collect::<Vec<i32>>(), vec![4]);
        assert_eq!(nfa.get_closure(&1, &'@').iter().cloned().collect::<Vec<i32>>(), vec![4]);
        let mut tmp: Vec<i32> = nfa.get_closure(&1, &'a').iter().cloned().collect(); tmp.sort();
        assert_eq!(tmp, vec![2, 3]);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_get_epsilon_closure() {
        let mut states: HashSet<i32> = HashSet::new();
        states.insert(3);
        states.insert(5);
        states.insert(6);
        let mut nfa = NFA::new(1, 6);
        nfa.set_chain(1, 2, '@');
        nfa.set_chain(2, 3, '@');
        nfa.set_chain(3, 4, '@');
        nfa.set_chain(3, 5, '@');
        nfa.set_chain(3, 6, '@');
        nfa.set_chain(5, 6, '@');
        assert_eq!(nfa.get_epsilon_closure(&states).len(), 3);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_epsilon_chain() {
        let mut nfa = NFA::new(1, 6);
        // パス構成
        let mut rng = rand::thread_rng();
        let mut chains = vec![(1, 2), (2, 3), (3, 4), (3, 5), (3, 6), (5, 6)];
        chains.shuffle(&mut rng);
        for chain in chains {
            nfa.set_chain(chain.0, chain.1, '@');
        }
        // チェック
        let checklist = vec![(5, 0), (4, 1), (3, 1), (0, 1), (1, 1), (0, 2)];
        for state in 1..=6 {
            assert_eq!(nfa.epsilon_chain[&state].0.len(), checklist[(state-1) as usize].0);
            assert_eq!(nfa.epsilon_chain[&state].1.len(), checklist[(state-1) as usize].1);
        }
    }
}