use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum MyStage {
  BeforeRound,
  AfterRound,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum MyState {
  Loading,
  InGame,
}

fn load_system(mut state: ResMut<State<MyState>>) {
  // Prepare something (such as texture loading) and wait until done.
  // ...

  // if loading.is_done() {
    state.replace(MyState::InGame).unwrap();
  //}
}

fn print_before_system() {
  println!("Before Update");
}

fn print_after_system() {
  println!("After Update");
}
fn print_system() {
  println!("Update");
}

fn main() {
  let loading_system_set = SystemSet::on_update(MyState::Loading)
    .with_system(load_system.system());
  let before_system_set = SystemSet::on_update(MyState::InGame)
    .with_system(print_before_system.system());
  let after_system_set = SystemSet::on_update(MyState::InGame)
    .with_system(print_after_system.system());
  let update_system_set = SystemSet::on_update(MyState::InGame)
    .with_system(print_system.system());
  App::build()
    .add_plugins(DefaultPlugins)
    .add_stage_before(
      CoreStage::Update,
      MyStage::BeforeRound,
      SystemStage::parallel(),
    )
    .add_stage_after(
      CoreStage::Update,
      MyStage::AfterRound,
      SystemStage::parallel(),
    )
    .add_state_to_stage(MyStage::BeforeRound, MyState::Loading)
    .add_state_to_stage(CoreStage::Update, MyState::Loading)
    .add_state_to_stage(MyStage::AfterRound, MyState::Loading)
    .add_system_set_to_stage(CoreStage::Update, loading_system_set)
    .add_system_set_to_stage(MyStage::BeforeRound, before_system_set)
    .add_system_set_to_stage(CoreStage::Update, update_system_set)
    .add_system_set_to_stage(MyStage::AfterRound, after_system_set)
    .run();
}
