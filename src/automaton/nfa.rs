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
    /// NFAのコンストラクタ
    pub fn new(init_states: Vec<i32>) -> NFA {
        let mut init_states = init_states;
        init_states.sort_unstable();
        let reserved_state = (init_states[0], init_states[init_states.len()-1]);
        NFA { start: -1, finish: -1, reserved_state, move_table: HashMap::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::NFA;

    #[test]
    fn init_test() {
        let _ = NFA::new(vec![0, 1, 2, 3, 4]);
    }
}