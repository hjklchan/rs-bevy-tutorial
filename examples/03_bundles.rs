use bevy::prelude::*;

/// # Bundle 是一种组织组件的方式
///
/// Bevy 也提供了大量的内置捆绑包
fn main() {
    App::new().add_systems(Start, spawn_entity).run();
}

// Player 玩家组件
// NOTE 该组件只作为标记作用
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Xp(u32);

/// # 自定义的捆绑包
///
/// 需要派生 Bundle 特征
///
/// 该捆绑包由三个组件组成
/// - 两个自定义组件
/// - 一个 Bevy 内置的组件
#[derive(Bundle)]
pub struct OhMyBundle {
    marker: Player,
    xp: Xp,
    transform: Transform,
}

// 生成实体
fn spawn_entity(mut commands: Commands) {
    commands.spawn(OhMyBundle {
        marker: Player,
        xp: Xp(30),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
    });
}
