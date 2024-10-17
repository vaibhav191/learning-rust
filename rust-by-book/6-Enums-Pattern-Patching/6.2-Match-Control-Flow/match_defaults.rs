let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat();
    7 => remove_fancy_hat();
    other => move_player(other),
    // or you can do the following, if we handle case but don't use that value
    _ => reroll(), // avoids rust warnings
                   // or
    _ => (), // nothing happens
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
