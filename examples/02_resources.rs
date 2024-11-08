use bevy::prelude::*;

/// # Resource 资源
///
/// - 每个资源在所有系统中只有一个实例, 即单例
/// - 该资源在 Rust 中类型可以是 struct 或 enum
fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        // 初始化一个资源, 还可以使用 insert_resource
        // 使用 init_resource 需要实现 Default 特征
        .init_resource::<GameState>()
        .add_systems(Update, update_position)
        .run();
}

/// # 定义资源
///
/// 需要为当前定义的资源实现 Resource Trait
///
/// 资源存储游戏的当前状态
#[derive(Resource)]
pub struct GameState {
    pub is_playing: bool,
}

// 为 GameState 资源实现默认值
// 或者使用 #[derive(Default)] 来自动实现
#[allow(warnings)]
impl Default for GameState {
    fn default() -> Self {
        Self { is_playing: false }
    }
}

/// # update_position 更新位置系统
///
/// 获取 Resource 的只读副本来方便的访问其字段,
/// 并作出有序的动作
fn update_position(r_game_state: Res<GameState>) {
    if r_game_state.is_playing {
        // 继续更新位置
        // -- snip --
    }
}
