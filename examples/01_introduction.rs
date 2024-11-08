use bevy::prelude::*;

/// ## Introduction 介绍
///
/// - 组件是高度通用而且可以是任何数据类型
fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        // 注册系统
        // 启动计划为程序的最开始运行
        .add_systems(Startup, spawn_spaceship)
        // 注册系统
        // 启动计划为每个呈现帧运行
        .add_systems(Update, update_position)
        // 注册系统
        // 启动计划为每个呈现帧运行
        .add_systems(Update, print_position)
        .run();
}

// Position 创建位置组件
#[derive(Component, Debug)]
pub struct Position {
    x: f32,
    y: f32,
}

// Velocity 创建速度组件
#[derive(Component)]
pub struct Velocity {
    x: f32,
    y: f32,
}

/// # spawn_spaceship 定义生成飞船系统
///
/// - 通过添加可变的 Commands ([`mut commands: Commands`]) 参数可以获得命令队列
/// - Commands 队列支持 World 的修改
///
/// ## 实体的定义
/// - 太空船实体由两个组件定义 (Position, Velocity)
pub fn spawn_spaceship(mut commands: Commands) {
    // 生成太空船实体
    commands.spawn((
        // 位置组件
        Position { x: 0.0, y: 0.0 },
        // 速度组件
        Velocity { x: 1.0, y: 1.0 },
    ));
}

/// # update_position 更新位置系统
/// 
/// 通过 Query 类型指定一个含有 Velocity 和 Position 的元组,
/// 该查询将会获取两个组件:
/// 
/// - 不可变的 Velocity 组件
/// - 可变的 Position 组件
pub fn update_position(mut query: Query<(&Velocity, &mut Position)>) {
    // 获取可变的迭代器
    // 简单的遍历世界中所有包含 (Velocity 和 Position) 的实体
    for (vec, mut pos) in query.iter_mut() {
        // 为每一个实体去更新组件里的字段值
        pos.x += vec.x;
        pos.y += vec.y;
    }
}

/// # print_position 打印位置系统
/// 
/// 通过 Query 类型指定查询所有含有 Entity 和 Position 的实体
/// 
/// 该查询会得到两个组件:
/// 
/// - 实体
/// - 不可变的 Position 组件
pub fn print_position(query: Query<(Entity, &Position)>) {
    // 获取不可变的迭代器
    // 遍历并打印
    for (ent, pos) in query.iter() {
        println!("Entity {:?} is at position {:?}", ent, pos);
    }
}