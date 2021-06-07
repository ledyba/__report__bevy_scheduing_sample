use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum MyStage {
  BeforeRound,
  AfterRound,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum MyState {
  UpdateState,
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
  let before_system_set = SystemSet::on_update(MyState::UpdateState)
    .with_system(print_before_system.system());
  let after_system_set = SystemSet::on_update(MyState::UpdateState)
    .with_system(print_after_system.system());
  let update_system_set = SystemSet::on_update(MyState::UpdateState)
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
    .add_state_to_stage(MyStage::BeforeRound, MyState::UpdateState)
    .add_state_to_stage(CoreStage::Update, MyState::UpdateState)
    .add_state_to_stage(MyStage::AfterRound, MyState::UpdateState)
    .add_system_set_to_stage(MyStage::BeforeRound, before_system_set)
    .add_system_set_to_stage(CoreStage::Update, update_system_set)
    .add_system_set_to_stage(MyStage::AfterRound, after_system_set)
    .run();
}
